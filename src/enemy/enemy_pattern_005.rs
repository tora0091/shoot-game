use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::define::*;

use super::{EnemySchedule, get_shoot_duration};

pub struct EnemyPattern005;

impl Plugin for EnemyPattern005 {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PostStartup, setup)
            .add_systems(Update, (
                enemy_spawn_pattern_005,
                enemy_move_pattern_005,
        ));
    }
}

#[derive(Component)]
pub struct EnemyMovePattern005 {
    base_y: f32,
}

struct Position {
    x: f32,
    y: f32,
    velocity_x: f32,
}

#[derive(Resource)]
pub struct EnemyPositions {
    positions: Vec<Position>,
}

fn setup(
    mut commands: Commands,
    window_size_limit: Res<WindowSizeLimit>,
) {
    let enemy_positions = vec![
        Position {x: window_size_limit.left - 50.0, y: -200.0, velocity_x: 0.5,},
        Position {x: window_size_limit.right + 50.0, y: -100.0, velocity_x: -0.5},
        Position {x: window_size_limit.left - 50.0, y: 0.0, velocity_x: 0.5},
        Position {x: window_size_limit.right + 50.0, y: 100.0, velocity_x: -0.5},
        Position {x: window_size_limit.left - 50.0, y: 200.0, velocity_x: 0.5},
    ];

    commands.insert_resource(EnemyPositions {
        positions: enemy_positions,
    });
}

fn enemy_spawn_pattern_005(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
    enemy_positions: Res<EnemyPositions>,
) {
    if EnemySchedule::is_ready(&mut enemy_schedule.enemy_pattern_005, game_timer.seconds) {

        for position in enemy_positions.positions.iter() {
            let x = position.x;
            let y = position.y;
            let velocity_x = position.velocity_x;
            let shot_duration = get_shoot_duration();

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
                Velocity {x: velocity_x, y: 0.0},
                EnemyMovePattern005 {
                    base_y: y,
                },
            ));
        }
    }
}

fn enemy_move_pattern_005(
    mut query: Query<(&mut Transform, &mut EnemyMovePattern005), With<EnemyMovePattern005>>,
    time: Res<Time<Virtual>>,
) {
    let sin = time.elapsed_seconds().sin() * 50.0;

    for (mut transform, enemy_move) in query.iter_mut() {
        transform.translation.y = enemy_move.base_y + sin.abs();
    }
}
