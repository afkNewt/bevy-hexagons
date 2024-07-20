use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use hexx::Hex;

use super::{
    components::{Border, HexTile, Team, TileVariant},
    resources::HexColors,
    BACKGROUND_HEX_LAYER, BACKGROUND_HEX_SIZE, BORDER_LAYER, HEX_GAP, HEX_LAYER, HEX_LAYOUT,
    HEX_RADIUS, HEX_SIZE,
};

pub fn load_colors(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(HexColors {
        backround_hex: materials.add(ColorMaterial::from(Color::srgb_u8(25, 25, 25))),

        neutral: materials.add(ColorMaterial::from(Color::srgb_u8(40, 40, 40))),
        neutral_weak_highlight: materials.add(ColorMaterial::from(Color::srgb_u8(60, 60, 60))),
        neutral_strong_highlight: materials.add(ColorMaterial::from(Color::srgb_u8(90, 90, 90))),

        ally_sprite: Color::srgb_u8(70, 130, 250),
        ally_unused_action_color: Color::srgb_u8(100, 150, 250),
        ally_used_action_color: Color::srgba_u8(100, 150, 250, 50),
        ally_border_color: materials.add(ColorMaterial::from(Color::srgba_u8(70, 130, 250, 100))),
        ally_capital: materials.add(ColorMaterial::from(Color::srgb_u8(70, 70, 200))),
        ally_capital_weak_highlight: materials
            .add(ColorMaterial::from(Color::srgb_u8(100, 100, 240))),
        ally_capital_strong_highlight: materials
            .add(ColorMaterial::from(Color::srgb_u8(150, 150, 255))),

        enemy_sprite: Color::srgb_u8(250, 130, 70),
        enemy_unused_action_color: Color::srgb_u8(250, 150, 100),
        enemy_used_action_color: Color::srgba_u8(250, 150, 100, 50),
        enemy_border_color: materials.add(ColorMaterial::from(Color::srgba_u8(200, 70, 70, 100))),
        enemy_capital: materials.add(ColorMaterial::from(Color::srgb_u8(200, 70, 70))),
        enemy_capital_weak_highlight: materials
            .add(ColorMaterial::from(Color::srgb_u8(240, 100, 100))),
        enemy_capital_strong_highlight: materials
            .add(ColorMaterial::from(Color::srgb_u8(255, 150, 150))),
    });
}

pub fn build_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<HexColors>,
) {
    let mut pointy_top_hex_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(RegularPolygon::new(HEX_SIZE, 6)).into(),
        material: colors.neutral.clone(),
        ..default()
    };

    let hex_coords = Hex::ZERO.range(HEX_RADIUS as u32);
    for coord in hex_coords {
        pointy_top_hex_mesh.transform.translation = Vec3::from_array(
            HEX_LAYOUT
                .hex_to_world_pos(coord)
                .extend(HEX_LAYER)
                .to_array(),
        );

        commands.spawn(pointy_top_hex_mesh.clone()).insert(HexTile {
            coordinate: coord,
            variant: TileVariant::Land,
            capture_progress: 0,
            team: Team::Neutral,
        });
    }

    let scale = 3_f32.sqrt() / 2.
        * (2. * HEX_RADIUS as f32 * HEX_GAP
            + HEX_SIZE * (2. * HEX_RADIUS as f32 + BACKGROUND_HEX_SIZE));

    let flat_top_hex_mesh = MaterialMesh2dBundle {
        mesh: meshes.add(RegularPolygon::new(scale, 6)).into(),
        material: colors.backround_hex.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., BACKGROUND_HEX_LAYER),
            rotation: Quat::from_rotation_z(30_f32.to_radians()),
            scale: Vec3::splat(1.),
        },
        ..Default::default()
    };

    commands.spawn(flat_top_hex_mesh);
}

pub fn draw_borders(
    mut commands: Commands,
    hexes: Query<&HexTile>,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<HexColors>,
    borders: Query<Entity, With<Border>>,
) {
    for border in &borders {
        commands.entity(border).despawn();
    }

    for team in [Team::Ally, Team::Enemy] {
        let Some(point_group) = tile_border(&hexes, team) else {
            continue;
        };

        let first = point_group[0][0];
        let second = point_group[0][1];

        let mut border = MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::new(first.distance(second), HEX_GAP * 2.))
                .into(),
            material: match team {
                Team::Neutral => colors.neutral.clone(),
                Team::Ally => colors.ally_border_color.clone(),
                Team::Enemy => colors.enemy_border_color.clone(),
            },
            ..default()
        };

        for points in point_group {
            for positions in points.windows(2) {
                border.transform = Transform {
                    translation: Vec3::new(
                        (positions[0].x + positions[1].x) / 2.,
                        (positions[0].y + positions[1].y) / 2.,
                        BORDER_LAYER,
                    ),
                    rotation: Quat::from_axis_angle(
                        Vec3::Z,
                        Vec2::new(
                            positions[0].x - positions[1].x,
                            positions[0].y - positions[1].y,
                        )
                        .angle_between(Vec2::X)
                            * -1.,
                    ),
                    ..Default::default()
                };

                commands.spawn(border.clone()).insert(Border);
            }
        }
    }
}

fn tile_border(hexes: &Query<&HexTile>, team: Team) -> Option<Vec<Vec<Vec2>>> {
    let valid_tiles = Hex::ZERO.range(HEX_RADIUS as u32).collect::<Vec<Hex>>();

    let mut unsorted_points = hexes
        .iter()
        .filter(|h| h.team == team)
        .flat_map(|h| {
            let neighbor_coords = h.coordinate.all_neighbors();
            let hex_pixel_pos = HEX_LAYOUT.hex_to_world_pos(h.coordinate);
            [
                neighbor_coords
                    .iter()
                    .filter(|c| !valid_tiles.contains(c))
                    .map(|c| {
                        let pixel_pos = HEX_LAYOUT.hex_to_world_pos(*c);
                        Vec2::new(
                            (pixel_pos.x + hex_pixel_pos.x) / 2.,
                            (pixel_pos.y + hex_pixel_pos.y) / 2.,
                        )
                    })
                    .collect::<Vec<_>>(),
                hexes
                    .iter()
                    .filter(|h| neighbor_coords.contains(&h.coordinate))
                    .filter_map(|n| {
                        if n.team != team {
                            let neighbor_pixel_pos = HEX_LAYOUT.hex_to_world_pos(n.coordinate);
                            return Some(Vec2::new(
                                (neighbor_pixel_pos.x + hex_pixel_pos.x) / 2.,
                                (neighbor_pixel_pos.y + hex_pixel_pos.y) / 2.,
                            ));
                        }

                        None
                    })
                    .collect::<Vec<_>>(),
            ]
            .concat()
        })
        .collect::<Vec<_>>();
    unsorted_points.dedup();

    if unsorted_points.is_empty() {
        return None;
    }

    let first = unsorted_points.remove(0);
    let mut point_groups = vec![vec![first]];
    let mut current_point = first;

    while !unsorted_points.is_empty() {
        let mut points = Vec::new();

        loop {
            let mut distance = i32::MAX;
            let mut index = 0;

            for (i, point) in unsorted_points.iter().enumerate() {
                let dist = current_point.distance(*point) as i32;
                if dist < distance {
                    index = i;
                    distance = dist;
                }
            }

            points.push(current_point);
            current_point = unsorted_points.remove(index);

            if unsorted_points.is_empty() {
                points.push(current_point);
            }

            if distance >= HEX_SIZE as i32 || unsorted_points.is_empty() {
                break;
            }
        }

        points.push(*points.first()?);
        point_groups.push(points);
    }

    point_groups.remove(0);
    Some(point_groups)
}
