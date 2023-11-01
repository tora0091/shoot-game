use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{Enemy, AutoDespawn, SpeedControl, SHOOT_RADIUS, SHOOT_VELOCITY, Velocity, FromEnemyShoot};

use self::formations::*;

mod formations;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(
                EnemySchedule {
                    enemy_pattern_001: 5,
                    enemy_pattern_002: 10,
                    enemy_pattern_003: 15,
                }
            )
            .add_systems(Update, (
                enemy_shoot_system,

                enemy_spawn_pattern_001,
                enemy_move_pattern_001,

                enemy_spawn_pattern_002,
                enemy_move_pattern_002,

                enemy_spawn_pattern_003,
                enemy_move_pattern_003,
        ));
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
