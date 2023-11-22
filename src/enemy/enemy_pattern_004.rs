use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::define::*;

use super::EnemySchedule;

pub struct EnemyPattern004;

impl Plugin for EnemyPattern004 {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            enemy_spawn_pattern_004,
            enemy_move_pattern_004,
        ));
    }
}

#[derive(Component)]
pub struct EnemyMovePattern004 {
    base_y: f32,
}

fn enemy_spawn_pattern_004(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
) {
    let mut rng = rand::thread_rng();

    if EnemySchedule::is_ready(&mut enemy_schedule.enemy_pattern_004, game_timer.seconds) {
        let shot_duration = rng.gen_range(1.0..3.0);

        let x = 0.0;
        let y = window_size_limit.top + 30.0;

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
                point: 1.0,
            },
            AutoDespawn,
            Velocity {x: 0.0, y: -0.5},
            EnemyMovePattern004 {
                base_y: y,
            },
        ));
    }
}

fn enemy_move_pattern_004(
    mut query: Query<(&Velocity, &mut Transform, &mut EnemyMovePattern004), With<EnemyMovePattern004>>,
    time: Res<Time<Virtual>>,
) {
    let theta = (time.elapsed_seconds() * 100.0) % 360.0;

    let radian = 2.0 * PI * (theta / 360.0);

    let x = radian.cos() * 100.0;
    let y = radian.sin() * 100.0;

    for (velocity, mut transform, mut enemy_move) in query.iter_mut() {
        transform.translation.x = x;
        transform.translation.y = enemy_move.base_y + y;
        enemy_move.base_y += velocity.y;
    }
}
