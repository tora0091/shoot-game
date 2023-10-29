use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::define::{WindowSizeLimit, EnemySchedule, ENEMY_RADIUS, Enemy, AutoDespawn, Velocity};

#[derive(Component)]
pub struct EnemyMovePattern001;

#[derive(Component)]
pub struct EnemyMovePattern002;



pub fn enemy_spawn_pattern001(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    time: Res<Time>,
) {
    if enemy_schedule.enemy_pattern001.tick(time.elapsed()).just_finished() {
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
    }
}

pub fn enemy_move_pattern001(
    mut query: Query<&mut Velocity, With<EnemyMovePattern001>>,
    time: Res<Time>,
) {
    let x = PI * (time.elapsed_seconds() % 60.0).sin();

    for mut velocity in query.iter_mut() {
        velocity.x = x;
    }
}


pub fn enemy_spawn_pattern002(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    time: Res<Time>,
) {
    if enemy_schedule.enemy_pattern002.tick(time.elapsed()).just_finished() {
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
            EnemyMovePattern002,
        ));
    }
}

pub fn enemy_move_pattern002(
    mut query: Query<&mut Velocity, With<EnemyMovePattern002>>,
) {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-10.0..10.0);

    for mut velocity in query.iter_mut() {
        velocity.x = x;
    }
}
