use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::define::*;

use super::{EnemySchedule, get_shoot_duration};

pub struct EnemyPattern006;

impl Plugin for EnemyPattern006 {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            enemy_spawn_pattern_006,
            enemy_move_pattern_006,
            enemy_wait_and_go,
        ));
    }
}

#[derive(Component)]
pub struct EnemyMovePattern006;

#[derive(Component)]
pub struct WaitTimer {
    timer: Timer,
}

fn enemy_spawn_pattern_006(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
) {
    if EnemySchedule::is_ready(&mut enemy_schedule.enemy_pattern_006, game_timer.seconds) {
        let y = window_size_limit.top + 30.0;

        struct Position {
            x: f32,
            y: f32,
        }

        let positions = [
            Position {x: 80.0, y},
            Position {x: -80.0, y},
        ];

        for position in positions {
            let shot_duration = get_shoot_duration();
            // enemy
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(ENEMY_RADIUS).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    transform: Transform::from_xyz(position.x, position.y, 9.0),
                    ..default()
                },
                Enemy {
                    shoot_interval: Timer::from_seconds(shot_duration, TimerMode::Repeating),
                    point: 1.0,
                },
                AutoDespawn,
                Velocity {x: 0.0, y: -2.0},
                EnemyMovePattern006,
            ));
        }
    }
}

fn enemy_move_pattern_006(
    mut commands: Commands,
    mut query: Query<(&mut Velocity, &mut Transform, Entity), With<EnemyMovePattern006>>,
) {
    for (mut velocity, mut transform, entity) in query.iter_mut() {
        if transform.translation.y == -100.0 {
            (velocity.x, velocity.y) = (0.0, 0.0);

            commands.entity(entity).insert(WaitTimer{
                timer: Timer::from_seconds(3.0, TimerMode::Once),
            });
            transform.translation.y = -100.1;
        }
    }
}

fn enemy_wait_and_go(
    mut query: Query<(&mut Velocity, &Transform, &mut WaitTimer), With<EnemyMovePattern006>>,
    time: Res<Time<Virtual>>,
) {
    for (mut velocity, transform, mut wait_timer) in query.iter_mut() {
        if wait_timer.timer.tick(time.delta()).finished() {
            if transform.translation.x >= 0.0 {
                (velocity.x, velocity.y) = (3.0, 3.0)
            } else {
                (velocity.x, velocity.y) = (-3.0, 3.0)
            }
        }
    }
}