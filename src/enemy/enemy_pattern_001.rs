use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::define::*;

use super::EnemySchedule;

pub struct EnemyPattern001;

impl Plugin for EnemyPattern001 {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            enemy_spawn_pattern_001,
        ));
    }
}

#[derive(Component)]
pub struct EnemyMovePattern001;

fn enemy_spawn_pattern_001(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
) {
    if EnemySchedule::is_ready(&mut enemy_schedule.enemy_pattern_001, game_timer.seconds) {
        let y = window_size_limit.top + 30.0;
        let x = window_size_limit.right / 3.0;

        let enemy_positions = [
            [x, y],
            [x * 2.0, y],
            [0.0, y],
            [-x, y],
            [-x * 2.0, y],
        ];

        let mut rng = rand::thread_rng();

        for position in enemy_positions {
            let shot_duration = rng.gen_range(1.0..3.0);

            // enemy
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(ENEMY_RADIUS).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::GREEN)),
                    transform: Transform::from_xyz(position[0], position[1], 9.0),
                    ..default()
                },
                Enemy {
                    shoot_interval: Timer::from_seconds(shot_duration, TimerMode::Repeating),
                },
                AutoDespawn,
                Velocity {x: 0.0, y: -0.5},
                EnemyMovePattern001,
            ));
        }
    }
}
