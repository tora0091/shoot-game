use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::define::*;

use super::EnemySchedule;

pub struct EnemyPattern006;

impl Plugin for EnemyPattern006 {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            enemy_spawn_pattern_006,
            enemy_move_pattern_006,
        ));
    }
}

#[derive(Component)]
pub struct EnemyMovePattern006;

pub fn enemy_spawn_pattern_006(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
) {
    if EnemySchedule::is_ready(&mut enemy_schedule.enemy_pattern_006, game_timer.seconds) {
        let mut rng = rand::thread_rng();

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
            let shot_duration = rng.gen_range(1.0..3.0);
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
                },
                AutoDespawn,
                Velocity {x: 0.0, y: -2.0},
                EnemyMovePattern006,
            ));
        }
    }
}

pub fn enemy_move_pattern_006(
    mut query: Query<(&mut Velocity, &Transform), With<EnemyMovePattern006>>,
) {
    for (mut velocity, transform) in query.iter_mut() {
        if transform.translation.y <= -100.0 {
            if transform.translation.x >= 0.0 {
                (velocity.x, velocity.y) = (3.0, 3.0)
            } else {
                (velocity.x, velocity.y) = (-3.0, 3.0)
            }
        }
    }
}
