use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{Enemy, AutoDespawn, SpeedControl, SHOOT_RADIUS, SHOOT_VELOCITY, Velocity, FromEnemyShoot};

use self::{
    enemy_pattern_001::EnemyPattern001,
    enemy_pattern_002::EnemyPattern002,
    enemy_pattern_003::EnemyPattern003,
    enemy_pattern_004::EnemyPattern004,
    enemy_pattern_005::EnemyPattern005,
    enemy_pattern_006::EnemyPattern006,
};

mod enemy_pattern_001;
mod enemy_pattern_002;
mod enemy_pattern_003;
mod enemy_pattern_004;
mod enemy_pattern_005;
mod enemy_pattern_006;

pub struct EnemyPlugin;

#[derive(Resource)]
pub struct EnemySchedule {
    pub enemy_pattern_001: EnemyScheduleValue,
    pub enemy_pattern_002: EnemyScheduleValue,
    pub enemy_pattern_003: EnemyScheduleValue,
    pub enemy_pattern_004: EnemyScheduleValue,
    pub enemy_pattern_005: EnemyScheduleValue,
    pub enemy_pattern_006: EnemyScheduleValue,
}

impl EnemySchedule {
    pub fn is_ready(enemy_schedule_value: &mut EnemyScheduleValue, game_time_seconds: u64) -> bool {
        if enemy_schedule_value.enable && enemy_schedule_value.seconds <= game_time_seconds {
            enemy_schedule_value.enable = false;
            return true;
        }
        return false;
    }
}

pub struct EnemyScheduleValue {
    seconds: u64,
    enable: bool,
}

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(
                EnemySchedule {
                    enemy_pattern_001: EnemyScheduleValue { seconds: 5, enable: true },
                    enemy_pattern_002: EnemyScheduleValue { seconds: 15, enable: true },
                    enemy_pattern_003: EnemyScheduleValue { seconds: 30, enable: true },
                    enemy_pattern_004: EnemyScheduleValue { seconds: 45, enable: true },
                    enemy_pattern_005: EnemyScheduleValue { seconds: 60, enable: true },
                    enemy_pattern_006: EnemyScheduleValue { seconds: 75, enable: true },
                }
            )
            .add_plugins((
                EnemyPattern001,
                EnemyPattern002,
                EnemyPattern003,
                EnemyPattern004,
                EnemyPattern005,
                EnemyPattern006,
            ))
            .add_systems(Update, enemy_shoot_system);
    }
}

fn enemy_shoot_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<(&Transform, &mut Enemy)>,
    time: Res<Time>,
    speed_control: Res<SpeedControl>,
) {
    for (enemy_transform, mut enemy) in query.iter_mut() {
        if enemy.shoot_interval.tick(time.delta()).just_finished() {
            // fire shooting from enemy
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(SHOOT_RADIUS).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_xyz(enemy_transform.translation.x, enemy_transform.translation.y, 0.0),
                    ..default()
                },
                Velocity {x: 0.0, y: -SHOOT_VELOCITY * speed_control.value },
                AutoDespawn,
                FromEnemyShoot,
            ));
        }
    }
}
