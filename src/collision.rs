use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::define::*;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            player_shoot_collision_system,
            enemy_shoot_collision_system,
            player_enemy_collision_system,
        ));
    }
}

fn player_shoot_collision_system(
    mut commands: Commands,
    player_shoots: Query<(Entity, &Transform), With<FromPlayerShoot>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    // mut enemy_spawn: ResMut<EnemySpawn>,
) {
    for (enemy_entity, enemy_transform) in enemies.iter() {
        for (player_shoot_entity, player_shoot_transform) in player_shoots.iter() {
            let is_collide = collide(
                enemy_transform.translation,
                Vec2::new(ENEMY_RADIUS, ENEMY_RADIUS),
                player_shoot_transform.translation,
                Vec2::new(PLAYER_RADIUS, PLAYER_RADIUS));

            // hit a player shoot to enemy
            if is_collide != None {
                commands.entity(enemy_entity).despawn();
                commands.entity(player_shoot_entity).despawn();

                // enemy_spawn.counter -= 1;

                let x = enemy_transform.translation.x;
                let y = enemy_transform.translation.y;

                commands.spawn(ShowBangPoint {x, y});
            }
        }
    }
}

fn enemy_shoot_collision_system(
    mut commands: Commands,
    enemy_shoots: Query<(Entity, &Transform), With<FromEnemyShoot>>,
    player: Query<(Entity, &Transform), With<Player>>,
    mut player_spawn: ResMut<PlayerSpawn>,
) {
    if let Ok((player_entity, player_transform)) = player.get_single() {
        for (enemy_shoot_entity, enemy_shoot_transform) in enemy_shoots.iter() {
            let is_collide = collide(
                player_transform.translation,
                Vec2::new(PLAYER_RADIUS, PLAYER_RADIUS),
                enemy_shoot_transform.translation,
                Vec2::new(SHOOT_RADIUS, SHOOT_RADIUS));

            if is_collide != None {
                commands.entity(player_entity).despawn();

                player_spawn.is_spawn = true;
                player_spawn.timer = Timer::from_seconds(3.0, TimerMode::Once);

                // player bang
                commands.spawn(ShowBangPoint {
                    x: player_transform.translation.x,
                    y: player_transform.translation.y,
                });

                commands.entity(enemy_shoot_entity).despawn();
            }
        }
    }
}

fn player_enemy_collision_system(
    mut commands: Commands,
    player: Query<(Entity, &Transform), With<Player>>,
    enemy: Query<(Entity, &Transform), With<Enemy>>,
    mut player_spawn: ResMut<PlayerSpawn>,
    mut enemy_spawn: ResMut<EnemySpawn>,
) {
    if let Ok((player_entity, player_transform)) = player.get_single() {
        for (enemy_entity, enemy_transform) in enemy.iter() {
            let is_collide = collide(
                player_transform.translation,
                Vec2::new(PLAYER_RADIUS, PLAYER_RADIUS),
                enemy_transform.translation,
                Vec2::new(ENEMY_RADIUS, ENEMY_RADIUS));

            if is_collide != None {
                commands.entity(player_entity).despawn();
                player_spawn.is_spawn = true;
                player_spawn.timer = Timer::from_seconds(3.0, TimerMode::Once);
                commands.spawn(ShowBangPoint {
                    x: player_transform.translation.x,
                    y: player_transform.translation.y,
                });

                commands.entity(enemy_entity).despawn();
                enemy_spawn.counter -= 1;
                commands.spawn(ShowBangPoint {
                    x: enemy_transform.translation.x,
                    y: enemy_transform.translation.y,
                });
            }
        }
    }
}