use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::define::*;

use super::EnemySchedule;

pub struct EnemyPattern001;

impl Plugin for EnemyPattern001 {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            enemy_spawn_pattern_001,
            enemy_move_pattern_001,
        ));
    }
}

#[derive(Component)]
pub struct EnemyMovePattern001;

pub fn enemy_spawn_pattern_001(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
) {
    if EnemySchedule::is_ready(&mut enemy_schedule.enemy_pattern_001, game_timer.seconds) {
        let x = 0.0;
        let y = window_size_limit.top - 30.0;

        // enemy
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(ENEMY_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(x, y, 9.0),
                ..default()
            },
            Enemy {
                shoot_interval: Timer::from_seconds(1.0, TimerMode::Repeating),
            },
            AutoDespawn,
            Velocity {x: 0.0, y: -1.0},
            EnemyMovePattern001,
        ));

        enemy_schedule.enemy_pattern_001.enable = false;
    }
}

pub fn enemy_move_pattern_001(
    mut query: Query<&mut Velocity, With<EnemyMovePattern001>>,
    time: Res<Time>,
) {
    let x = PI * (time.elapsed_seconds() % 60.0).sin();

    for mut velocity in query.iter_mut() {
        velocity.x = x;
    }
}
