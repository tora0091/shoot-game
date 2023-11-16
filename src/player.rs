use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{WindowSizeLimit, PlayerStatus, SpeedControl, SHOOT_RADIUS, Velocity, SHOOT_VELOCITY, AutoDespawn, PLAYER_RADIUS, PLAYER_VELOCITY, FromPlayerShoot, Player};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            player_status_system,
            player_move_system,
            player_shoot_system,
        ));
    }
}

fn player_status_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut player_status: ResMut<PlayerStatus>,
    time: Res<Time>,
) {
    if player_status.is_spawn == true && player_status.timer.tick(time.delta()).just_finished() {
        // set player init position
        let player = Player {
            x: 0.0,
            y: window_size_limit.bottom + PLAYER_RADIUS + 50.0,
            z: 10.0,
        };

        // player
        commands.spawn(
            (MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(PLAYER_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform {
                    translation: Vec3::new(player.x, player.y, player.z),
                    ..default()
                },
                ..default()
            },
            player,
        ));

        player_status.is_spawn = false;
    }
}

fn player_move_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    speed_control: Res<SpeedControl>,
) {
    if let Ok((mut player_transform, mut player_position)) = query.get_single_mut() {
        let speed = PLAYER_VELOCITY * speed_control.value;

        if input.pressed(KeyCode::Up) {
            player_position.y += speed;
        }

        if input.pressed(KeyCode::Down) {
            player_position.y -= speed;
        }

        if input.pressed(KeyCode::Right) {
            player_position.x += speed;
        }

        if input.pressed(KeyCode::Left) {
            player_position.x -= speed;
        }

        player_transform.translation = player_position.get_position();
    }
}

fn player_shoot_system(
    mut commands: Commands,
    query: Query<&Player>,
    input: Res<Input<KeyCode>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    speed_control: Res<SpeedControl>,
) {
    if input.just_pressed(KeyCode::Space) {
        if let Ok(player_position) = query.get_single() {
            // player shoot
            commands.spawn(
                (MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(SHOOT_RADIUS).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(player_position.set_z_position(0.0)),
                    ..default()
                },
                Velocity { x: 0.0, y: SHOOT_VELOCITY * speed_control.value },
                AutoDespawn,
                FromPlayerShoot,
            ));
        }
    }
}
