use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::define::*;

use super::EnemySchedule;

pub struct EnemyPattern005;

impl Plugin for EnemyPattern005 {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            enemy_spawn_pattern_005,
            enemy_move_pattern_005,
        ));
    }
}

#[derive(Component)]
pub struct EnemyMovePattern005 {
    base_y: f32,
}

pub fn enemy_spawn_pattern_005(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
) {
    let mut rng = rand::thread_rng();

    if EnemySchedule::is_ready(&mut enemy_schedule.enemy_pattern_005, game_timer.seconds) {
        let shot_duration = rng.gen_range(1.0..3.0);

        let x = window_size_limit.left - 50.0;
        let y = 10.0;

        // enemy
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(ENEMY_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(x, y, 9.0),
                ..default()
            },
            Enemy {
                shoot_interval: Timer::from_seconds(shot_duration, TimerMode::Repeating),
            },
            AutoDespawn,
            Velocity {x: 0.5, y: 0.0},
            EnemyMovePattern005 {
                base_y: y,
            },
        ));
    }
}

pub fn enemy_move_pattern_005(
    mut query: Query<(&mut Transform, &mut EnemyMovePattern005), With<EnemyMovePattern005>>,
    time: Res<Time>,
) {
    let sin = time.elapsed_seconds().sin() * 100.0;

    for (mut transform, enemy_move) in query.iter_mut() {
        transform.translation.y = enemy_move.base_y + sin.abs();
    }
}
