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

pub fn enemy_spawn_pattern_002(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
) {
    if enemy_schedule.enemy_pattern_002.seconds == game_timer.seconds && enemy_schedule.enemy_pattern_002.enable {
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

        enemy_schedule.enemy_pattern_002.enable = false;
    }
}

pub fn enemy_move_pattern_002(
    mut query: Query<&mut Velocity, With<EnemyMovePattern002>>,
) {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(-10.0..10.0);

    for mut velocity in query.iter_mut() {
        velocity.x = x;
    }
}
