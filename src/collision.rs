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
    enemies: Query<(Entity, &Transform, &Enemy), With<Enemy>>,
    mut player_status: ResMut<PlayerStatus>,
) {
    for (enemy_entity, enemy_transform, enemy) in enemies.iter() {
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

                let x = enemy_transform.translation.x;
                let y = enemy_transform.translation.y;

                player_status.score += enemy.point;

                commands.spawn(ShowBangPoint {x, y});
            }
        }
    }
}

fn enemy_shoot_collision_system(
    mut commands: Commands,
    enemy_shoots: Query<(Entity, &Transform), With<FromEnemyShoot>>,
    player: Query<(Entity, &Transform, &Player), With<Player>>,
    mut player_status: ResMut<PlayerStatus>,
) {
    if let Ok((player_entity, player_transform, player)) = player.get_single() {
        for (enemy_shoot_entity, enemy_shoot_transform) in enemy_shoots.iter() {
            if player.is_enable {
                let is_collide = collide(
                    player_transform.translation,
                    Vec2::new(PLAYER_RADIUS, PLAYER_RADIUS),
                    enemy_shoot_transform.translation,
                    Vec2::new(SHOOT_RADIUS, SHOOT_RADIUS));

                if is_collide != None {
                    commands.entity(player_entity).despawn();

                    player_status.is_spawn = true;
                    player_status.spawn_timer = Timer::from_seconds(3.0, TimerMode::Once);

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
}

fn player_enemy_collision_system(
    mut commands: Commands,
    player: Query<(Entity, &Transform, &Player), With<Player>>,
    enemy: Query<(Entity, &Transform), With<Enemy>>,
    mut player_status: ResMut<PlayerStatus>,
) {
    if let Ok((player_entity, player_transform, player)) = player.get_single() {
        for (enemy_entity, enemy_transform) in enemy.iter() {
            if player.is_enable {
                let is_collide = collide(
                    player_transform.translation,
                    Vec2::new(PLAYER_RADIUS, PLAYER_RADIUS),
                    enemy_transform.translation,
                    Vec2::new(ENEMY_RADIUS, ENEMY_RADIUS));

                if is_collide != None {
                    commands.entity(player_entity).despawn();
                    player_status.is_spawn = true;
                    player_status.spawn_timer = Timer::from_seconds(3.0, TimerMode::Once);
                    commands.spawn(ShowBangPoint {
                        x: player_transform.translation.x,
                        y: player_transform.translation.y,
                    });

                    commands.entity(enemy_entity).despawn();
                    commands.spawn(ShowBangPoint {
                        x: enemy_transform.translation.x,
                        y: enemy_transform.translation.y,
                    });
                }
            }
        }
    }
}