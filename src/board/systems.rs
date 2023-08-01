use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::hexagon::{hex_to_pixel, hexes_in_range, Cube};

use super::{
    components::{Border, HexTile, TileVariant, Team},
    resources::HexColors,
    BACKGROUND_HEX_SIZE, HEX_GAP, HEX_RADIUS, HEX_SIZE,
};

pub fn load_colors(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(HexColors {
        backround_hex: materials.add(ColorMaterial::from(Color::rgb_u8(25, 25, 25))),

        neutral: materials.add(ColorMaterial::from(Color::rgb_u8(40, 40, 40))),
        neutral_weak_highlight: materials.add(ColorMaterial::from(Color::rgb_u8(60, 60, 60))),
        neutral_strong_highlight: materials.add(ColorMaterial::from(Color::rgb_u8(90, 90, 90))),

        ally_sprite: Color::rgb_u8(70, 130, 250),
        ally_unused_action_color: Color::rgb_u8(100, 150, 250),
        ally_used_action_color: Color::rgba_u8(100, 150, 250, 50),
        ally_border_color: materials.add(ColorMaterial::from(Color::rgba_u8(70, 130, 250, 100))),
        ally_capital: materials.add(ColorMaterial::from(Color::rgb_u8(70, 70, 200))),
        ally_capital_weak_highlight: materials
            .add(ColorMaterial::from(Color::rgb_u8(100, 100, 240))),
        ally_capital_strong_highlight: materials
            .add(ColorMaterial::from(Color::rgb_u8(150, 150, 255))),

        enemy_sprite: Color::rgb_u8(250, 130, 70),
        enemy_unused_action_color: Color::rgb_u8(250, 150, 100),
        enemy_used_action_color: Color::rgba_u8(250, 150, 100, 50),
        enemy_border_color: materials.add(ColorMaterial::from(Color::rgba_u8(200, 70, 70, 100))),
        enemy_capital: materials.add(ColorMaterial::from(Color::rgb_u8(200, 70, 70))),
        enemy_capital_weak_highlight: materials
            .add(ColorMaterial::from(Color::rgb_u8(240, 100, 100))),
        enemy_capital_strong_highlight: materials
            .add(ColorMaterial::from(Color::rgb_u8(255, 150, 150))),
    });
}

pub fn build_board(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    colors: Res<HexColors>,
) {
    let mut pointy_top_hex_mesh = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::RegularPolygon::new(HEX_SIZE, 6).into())
            .into(),
        material: colors.neutral.clone(),
        ..default()
    };

    let hex_coords = hexes_in_range(HEX_RADIUS, Cube::axial_new(0, 0));
    for coord in hex_coords {
        // https://www.redblobgames.com/grids/hexagons/#hex-to-pixel
        pointy_top_hex_mesh.transform.translation = hex_to_pixel(coord).extend(1.);

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
        mesh: meshes
            .add(shape::RegularPolygon::new(scale, 6).into())
            .into(),
        material: colors.backround_hex.clone(),
        transform: Transform::from_rotation(Quat::from_rotation_z(30_f32.to_radians())),
        ..default()
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
    let ally_point_group = tile_border(
        &hexes,
        Team::Ally,
    );
    let enemy_point_group = tile_border(
        &hexes,
        Team::Enemy,
    );

    let Some(ally_point_group) = ally_point_group else {
        return;
    };

    let Some(enemy_point_group) = enemy_point_group else {
        return;
    };

    for border in &borders {
        commands.entity(border).despawn();
    }

    let first = ally_point_group[0][0];
    let second = ally_point_group[0][1];

    let mut border = MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(first.distance(second), HEX_GAP * 2.)).into())
            .into(),
        material: colors.ally_border_color.clone(),
        ..default()
    };

    for points in ally_point_group {
        for positions in points.windows(2) {
            border.transform = Transform {
                translation: Vec3::new(
                    (positions[0].x + positions[1].x) / 2.,
                    (positions[0].y + positions[1].y) / 2.,
                    2.,
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

    border.material = colors.enemy_border_color.clone();
    for points in enemy_point_group {
        for positions in points.windows(2) {
            border.transform = Transform {
                translation: Vec3::new(
                    (positions[0].x + positions[1].x) / 2.,
                    (positions[0].y + positions[1].y) / 2.,
                    2.,
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

fn tile_border(hexes: &Query<&HexTile>, team: Team) -> Option<Vec<Vec<Vec2>>> {
    let valid_tiles = hexes_in_range(HEX_RADIUS, Cube::axial_new(0, 0));

    let mut unsorted_points = hexes
        .iter()
        .filter(|h| h.team == team)
        .flat_map(|h| {
            let neighbor_coords = h.coordinate.cube_neighbors();
            let hex_pixel_pos = hex_to_pixel(h.coordinate);

            [
                neighbor_coords
                    .iter()
                    .filter(|c| !valid_tiles.contains(c))
                    .map(|c| {
                        let pixel_pos = hex_to_pixel(*c);
                        Vec2::new((pixel_pos.x + hex_pixel_pos.x) / 2., (pixel_pos.y + hex_pixel_pos.y) / 2.)
                    })
                    .collect::<Vec<_>>(),
                hexes
                    .iter()
                    .filter(|h| neighbor_coords.contains(&h.coordinate))
                    .filter_map(|n| {
                        if n.team != team {
                            let neighbor_pixel_pos = hex_to_pixel(n.coordinate);
                            return Some(Vec2::new((neighbor_pixel_pos.x + hex_pixel_pos.x) / 2., (neighbor_pixel_pos.y + hex_pixel_pos.y) / 2.));
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
