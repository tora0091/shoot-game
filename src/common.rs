use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::define::*;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(FixedUpdate, player_in_window_system)
            .add_systems(Update, (
                auto_move_system,
                auto_despawn_system,
                shoot_bang_system,
                show_bang_system,
                speed_control_system,
            ));
    }
}

fn player_in_window_system(
    mut query: Query<(&mut Transform, &mut Player)>,
    window_size_limit: Res<WindowSizeLimit>,
) {
    if let Ok((mut player_transform, mut player_position)) = query.get_single_mut() {
        let top_limit = window_size_limit.top - PLAYER_RADIUS;
        if player_position.y > top_limit {
            player_position.y = top_limit;
        }

        let bottom_limit = window_size_limit.bottom + PLAYER_RADIUS;
        if player_position.y < bottom_limit {
            player_position.y = bottom_limit;
        }

        let right_limit = window_size_limit.right - PLAYER_RADIUS;
        if player_position.x > right_limit {
            player_position.x = right_limit;
        }

        let light_limit = window_size_limit.left + PLAYER_RADIUS;
        if player_position.x < light_limit {
            player_position.x = light_limit;
        }

        player_transform.translation = player_position.get_position();
    }
}

fn auto_move_system(
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        let x = x + velocity.x;
        let y = y + velocity.y;

        transform.translation = Vec3::new(x, y, transform.translation.z);
    }
}

fn auto_despawn_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<AutoDespawn>>,
    window_size_limit: Res<WindowSizeLimit>,
) {
    let margin = WINDOW_SIZE_MARGIN;
    for (entity, transform) in query.iter() {
        if transform.translation.x > window_size_limit.right + margin
            || transform.translation.x < window_size_limit.left - margin
            || transform.translation.y > window_size_limit.top + margin
            || transform.translation.y < window_size_limit.bottom - margin
        {
            commands.entity(entity).despawn();
        }
    }
}

fn shoot_bang_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ShootBang)>,
    time: Res<Time<Virtual>>,
) {
    for (entity, mut shoot_bang) in query.iter_mut() {
        if shoot_bang.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn show_bang_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &ShowBangPoint)>,
) {
    for (entity, show_bang_point) in query.iter() {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(20.0, 50.0)).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform {
                    translation: Vec3::new(show_bang_point.x, show_bang_point.y, 1.0),
                    rotation: Quat::from_rotation_z(400.0),
                    ..default()
                },
                ..default()
            },
            ShootBang {
                timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once),
            }
        )).with_children(|p| {
            p.spawn(MaterialMesh2dBundle {
                mesh: meshes.add(shape::Quad::new(Vec2::new(20.0, 50.0)).into()).into(),
                material: materials.add(ColorMaterial::from(Color::RED)),
                transform: Transform {
                    rotation: Quat::from_rotation_z(20.0),
                    ..default()
                },
                ..default()
            });
        });

        commands.entity(entity).despawn();
    }
}

fn speed_control_system(
    input: Res<Input<KeyCode>>,
    mut speed_control: ResMut<SpeedControl>,
) {
    // speed up
    if input.just_pressed(KeyCode::A) {
        speed_control.value *= 1.2;
        println!("speed: {}", speed_control.value);
    }

    // speed down
    if input.just_pressed(KeyCode::Z) {
        speed_control.value *= 0.8;
        println!("speed: {}", speed_control.value);
    }
}
