use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

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
pub struct EnemyMovePattern003 {
    counter: u32,
}

pub fn enemy_spawn_pattern_003(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
) {
    if EnemySchedule::is_ready(&mut enemy_schedule.enemy_pattern_003, game_timer.seconds) {
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
            EnemyMovePattern003 {counter: 0},
        ));

        enemy_schedule.enemy_pattern_003.enable = false;
    }
}

pub fn enemy_move_pattern_003(
    mut query: Query<(&mut Velocity, &mut EnemyMovePattern003)>,
) {

    let mut m = -1.0;
    for (mut velocity, mut enemy_pattern) in query.iter_mut() {

        let a = enemy_pattern.counter % 200;

        if a == 0 {
            // velocity.x = 150.0;
            m = 1.0;
        } else if a == 100 {
            // velocity.x = -150.0;
            m = -1.0;
        } else {
            velocity.x += 1.0 * m;
        }

        enemy_pattern.counter += 1;
    }
}
