use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::{WindowSizeLimit, EnemySpawn, ENEMY_RADIUS, ENEMY_SPAWN_MAX_COUNTER, Enemy, AutoDespawn, SpeedControl, SHOOT_RADIUS, SHOOT_VELOCITY, Velocity, FromEnemyShoot};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            enemy_spawn_system,
            enemy_shoot_system,
        ));
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_spawn: ResMut<EnemySpawn>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(window_size_limit.left + ENEMY_RADIUS ..window_size_limit.right - ENEMY_RADIUS);
    let y = rng.gen_range((window_size_limit.bottom + ENEMY_RADIUS) / 2.0 ..window_size_limit.top - ENEMY_RADIUS);
    let shot_interval = rng.gen_range(1.0..5.0);

    if enemy_spawn.counter < ENEMY_SPAWN_MAX_COUNTER && enemy_spawn.timer.tick(time.delta()).just_finished() {
        // enemy
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(ENEMY_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(x, y, 9.0),
                ..default()
            },
            Enemy {
                x, y, shoot_interval: Timer::from_seconds(shot_interval as f32, TimerMode::Repeating),
            },
            AutoDespawn,
        ));
        enemy_spawn.counter += 1;
    }
}

fn enemy_shoot_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Enemy>,
    time: Res<Time>,
    speed_control: Res<SpeedControl>,
) {
    for mut enemy in query.iter_mut() {
        if enemy.shoot_interval.tick(time.delta()).just_finished() {
            // fire shooting from enemy
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(SHOOT_RADIUS).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_xyz(enemy.x, enemy.y, 0.0),
                    ..default()
                },
                Velocity {x: 0.0, y: -SHOOT_VELOCITY * speed_control.value },
                AutoDespawn,
                FromEnemyShoot,
            ));
        }
    }
}
