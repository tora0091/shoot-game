use bevy::{prelude::*, sprite::MaterialMesh2dBundle, math::{cubic_splines::CubicCurve, vec3}};

use crate::define::*;

use super::{EnemySchedule, get_shoot_duration};
pub struct EnemyPattern007;

impl Plugin for EnemyPattern007 {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            enemy_spawn_pattern_007,
            enemy_move_pattern_007,
        ));
    }
}

#[derive(Component)]
pub struct EnemyMovePattern007;

#[derive(Component)]
pub struct Curve(CubicCurve<Vec3>);

fn enemy_spawn_pattern_007(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_schedule: ResMut<EnemySchedule>,
    game_timer: Res<GameTimer>,
) {
    if EnemySchedule::is_ready(&mut enemy_schedule.enemy_pattern_007, game_timer.seconds) {
        let shot_duration = get_shoot_duration();

        let x = window_size_limit.right + (ENEMY_RADIUS * 2.0);
        let points = [[
            vec3(-x,0.0, 0.0),
            vec3(x * 2.0, 240.0, 0.0),
            vec3(-x * 2.0, 240.0, 0.0),
            vec3(x, 0.0, 0.0),
        ]];
    
        let bezier = CubicBezier::new(points).to_curve();
    
        // enemy
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(ENEMY_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_translation(points[0][0]),
                ..default()
            },
            Enemy {
                shoot_interval: Timer::from_seconds(shot_duration, TimerMode::Repeating),
                point: 1.0,
            },
            AutoDespawn,
            Velocity {x: 0.0, y: 0.0},
            EnemyMovePattern007,
            Curve(bezier),
        ));
    }
}

fn enemy_move_pattern_007(
    mut query: Query<(&mut Transform, &Curve), With<EnemyMovePattern007>>,
    time: Res<Time>,
    // mut gizmos: Gizmos,
) {
    let t = (time.elapsed_seconds().sin() + 1.0) / 2.0;

    for (mut transform, curve) in &mut query {
        // gizmos.linestrip(curve.0.iter_positions(50), Color::WHITE);
        transform.translation = curve.0.position(t);
    }
}
