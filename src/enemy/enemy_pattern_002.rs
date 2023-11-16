use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

use crate::define::*;

use super::EnemySchedule;

pub struct EnemyPattern002;

impl Plugin for EnemyPattern002 {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            enemy_spawn_pattern_002,
            enemy_move_pattern_002,
        ));
    }
}

#[derive(Component)]
pub struct EnemyMovePattern002;

fn enemy_spawn_pattern_002(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
) {
    let mut rng = rand::thread_rng();

    if EnemySchedule::is_ready(&mut enemy_schedule.enemy_pattern_002, game_timer.seconds) {
        let y = window_size_limit.top + 30.0;

        let shot_duration = rng.gen_range(1.0..3.0);

        // enemy
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(ENEMY_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(0.0, y, 9.0),
                ..default()
            },
            Enemy {
                shoot_interval: Timer::from_seconds(shot_duration, TimerMode::Repeating),
                point: 1.0,
            },
            AutoDespawn,
            Velocity {x: 0.0, y: -0.5},
            EnemyMovePattern002,
        ));
    }
}

fn enemy_move_pattern_002(
    mut query: Query<(&mut Velocity, &Transform), With<EnemyMovePattern002>>,
    time: Res<Time>,
) {
    let sin = time.elapsed_seconds().sin();

    let x = sin * 100.0;
    for (mut velocity, transform) in query.iter_mut() {
        velocity.x = x - transform.translation.x;
    }
}
