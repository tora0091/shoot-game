use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::define::*;

use super::EnemySchedule;

pub struct EnemyPattern003;

impl Plugin for EnemyPattern003 {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            enemy_spawn_pattern_003,
            enemy_move_pattern_003,
        ));
    }
}

#[derive(Component)]
pub struct EnemyMovePattern003;

fn enemy_spawn_pattern_003(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
) {
    let mut rng = rand::thread_rng();

    if EnemySchedule::is_ready(&mut enemy_schedule.enemy_pattern_003, game_timer.seconds) {
        let shot_duration = rng.gen_range(1.0..3.0);

        struct Position {
            x: f32,
            y: f32,
            velocity_x: f32,
        }

        let enemy_positions = [
            Position {x: 40.0, y: window_size_limit.top + 30.0, velocity_x: -0.2},
            Position {x: -40.0, y: window_size_limit.top + 50.0, velocity_x: 0.2},
        ];

        for position in enemy_positions {
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
                Velocity {x: position.velocity_x, y: 0.0},
                EnemyMovePattern003,
            ));
        }
    }
}

fn enemy_move_pattern_003(
    mut query: Query<(&mut Velocity, &mut Transform), With<EnemyMovePattern003>>,
) {
    // y = 0.1x^2 + 10x -200
    for (mut velocity, transform) in query.iter_mut() {
        let org_y = transform.translation.y;
        let org_x = transform.translation.x;
        let y = 0.1 * (org_x * org_x) + (10.0 * org_x) - 200.0;

        velocity.y = y - org_y;
    }
}
