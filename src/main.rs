use std::time::Duration;

use bevy::{prelude::*, window::WindowResolution, sprite::{MaterialMesh2dBundle, collide_aabb::collide}};
use rand::Rng;

const WINDOW_SIZE_WIDTH: f32 = 500.0;
const WINDOW_SIZE_HEIGHT: f32 = 610.0;
const WINDOW_SIZE_MARGIN: f32 = 100.0;

const PLAYER_RADIUS: f32 = 20.0;
const PLAYER_VELOCITY: f32 = 3.0;

const ENEMY_RADIUS: f32 = 25.0;
const ENEMY_SPAWN_MAX_COUNTER: u32 = 3;
const ENEMY_SPAWN_DURATION_SECONDS: f32 = 1.0;

const SHOOT_VELOCITY: f32 = 3.0;
const SHOOT_RADIUS: f32 = 5.0;

#[derive(Resource)]
struct WindowSizeLimit {
    top: f32,
    bottom: f32,
    right: f32,
    left: f32,
}

impl WindowSizeLimit {
    fn new(top: f32, bottom: f32, right: f32, left: f32) -> Self {
        Self { top, bottom, right, left }
    }
}

#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "shooting game!!!".to_string(),
                resolution: WindowResolution::new(WINDOW_SIZE_WIDTH, WINDOW_SIZE_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_system)
        .add_systems(FixedUpdate, player_in_window_system)
        .add_systems(Update, (
            player_move_system,
            player_shoot_system,
            auto_move_system,
            enemy_spawn_system,
            enemy_shoot_system,
            auto_despawn_system,
            player_shoot_collision_system,
            shoot_bang_system,
            speed_control_system,
            bevy::window::close_on_esc,
        ))
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Window>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // get window limit
    let window = query.single();
    let width = window.resolution.width() / 2.0;
    let height = window.resolution.height() / 2.0;

    commands.insert_resource(WindowSizeLimit::new(height, -height, width, -width));

    // enemy count
    commands.insert_resource(EnemySpawn {
        counter: 0,
        timer: Timer::from_seconds(ENEMY_SPAWN_DURATION_SECONDS, TimerMode::Repeating),
    });

    // speed control
    commands.insert_resource(SpeedControl { value: 1.0 });

    // set player init position
    let player = Player {
        x: 0.0,
        y: -height + PLAYER_RADIUS,
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
}

#[derive(Component)]
struct Player {
    x: f32,
    y: f32,
    z: f32,
}

impl Player {
    fn get_position(&self) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z: self.z }
    }
    fn set_z_position(&self, z: f32) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z }
    }
}

fn player_move_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player)>,
    speed_control: Res<SpeedControl>,
) {
    let (mut player_transform, mut player_position) = query.single_mut();

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

fn player_in_window_system(
    mut query: Query<(&mut Transform, &mut Player)>,
    window_size_limit: Res<WindowSizeLimit>,
) {
    let (mut player_transform, mut player_position) = query.single_mut();

    let top_limit = window_size_limit.top - PLAYER_RADIUS;
    if player_position.y > top_limit {
        player_position.y = top_limit;
    }

    let bottom_limit = window_size_limit.bottom + PLAYER_RADIUS;
    if player_position.y < bottom_limit {
        player_position.y = bottom_limit;
    }

    let right_limit = window_size_limit.right - PLAYER_RADIUS;
    if player_position.x > right_limit {
        player_position.x = right_limit;
    }

    let light_limit = window_size_limit.left + PLAYER_RADIUS;
    if player_position.x < light_limit {
        player_position.x = light_limit;
    }

    player_transform.translation = player_position.get_position();
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
        let player_position = query.single();

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

fn auto_move_system(
    mut query: Query<(&mut Transform, &mut Velocity)>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        let x = transform.translation.x;
        let y = transform.translation.y;

        let x = x + velocity.x;
        let y = y + velocity.y;

        transform.translation = Vec3::new(x, y, transform.translation.z);
    }
}

fn enemy_spawn_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window_size_limit: Res<WindowSizeLimit>,
    mut enemy_spawn: ResMut<EnemySpawn>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    let x = rng.gen_range(window_size_limit.left + ENEMY_RADIUS ..window_size_limit.right - ENEMY_RADIUS);
    let y = rng.gen_range(window_size_limit.bottom + ENEMY_RADIUS ..window_size_limit.top - ENEMY_RADIUS);
    let shot_interval = rng.gen_range(1.0..5.0);

    if enemy_spawn.counter < ENEMY_SPAWN_MAX_COUNTER && enemy_spawn.timer.tick(time.delta()).just_finished() {
        // enemy
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(ENEMY_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::GREEN)),
                transform: Transform::from_xyz(x, y, 9.0),
                ..default()
            },
            Enemy {
                x, y, shoot_interval: Timer::from_seconds(shot_interval as f32, TimerMode::Repeating),
            },
            AutoDespawn,
        ));
        enemy_spawn.counter += 1;
    }
}

fn enemy_shoot_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut query: Query<&mut Enemy>,
    time: Res<Time>,
    speed_control: Res<SpeedControl>,
) {
    for mut enemy in query.iter_mut() {
        if enemy.shoot_interval.tick(time.delta()).just_finished() {
            // fire shooting from enemy
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Circle::new(SHOOT_RADIUS).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_xyz(enemy.x, enemy.y, 0.0),
                    ..default()
                },
                Velocity {x: 0.0, y: -SHOOT_VELOCITY * speed_control.value },
                AutoDespawn,
            ));
        }
    }
}

#[derive(Component)]
struct Enemy {
    x: f32,
    y: f32,
    shoot_interval: Timer,
}

#[derive(Resource)]
struct EnemySpawn {
    counter: u32,
    timer: Timer,
}

#[derive(Component)]
struct AutoDespawn;

fn auto_despawn_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<AutoDespawn>>,
    window_size_limit: Res<WindowSizeLimit>,
) {
    let margin = WINDOW_SIZE_MARGIN;
    for (entity, transform) in query.iter() {
        if transform.translation.x > window_size_limit.right + margin
            || transform.translation.x < window_size_limit.left - margin
            || transform.translation.y > window_size_limit.top + margin
            || transform.translation.y < window_size_limit.bottom - margin
        {
            commands.entity(entity).despawn();
        }
    }
}

#[derive(Component)]
struct FromPlayerShoot;

fn player_shoot_collision_system(
    mut commands: Commands,
    player_shoots: Query<(Entity, &Transform), With<FromPlayerShoot>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut enemy_spawn: ResMut<EnemySpawn>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

                enemy_spawn.counter -= 1;

                let x = enemy_transform.translation.x;
                let y = enemy_transform.translation.y;

                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Quad::new(Vec2::new(20.0, 50.0)).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::RED)),
                        transform: Transform {
                            translation: Vec3::new(x, y, 1.0),
                            rotation: Quat::from_rotation_z(400.0),
                            ..default()
                        },
                        ..default()
                    },
                    ShootBang {
                        timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Once),
                    }
                )).with_children(|p| {
                    p.spawn(MaterialMesh2dBundle {
                        mesh: meshes.add(shape::Quad::new(Vec2::new(20.0, 50.0)).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::RED)),
                        transform: Transform {
                            rotation: Quat::from_rotation_z(20.0),
                            ..default()
                        },
                        ..default()
                    });
                });
            }
        }
    }
}

#[derive(Component)]
struct ShootBang {
    timer: Timer,
}

fn shoot_bang_system(
    mut commands: Commands,
    mut query: Query<(Entity, &mut ShootBang)>,
    time: Res<Time>,
) {
    for (entity, mut shoot_bang) in query.iter_mut() {
        if shoot_bang.timer.tick(time.delta()).just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Resource)]
struct SpeedControl {
    value: f32,
}

fn speed_control_system(
    input: Res<Input<KeyCode>>,
    mut speed_control: ResMut<SpeedControl>,
) {
    // speed up
    if input.just_pressed(KeyCode::A) {
        speed_control.value *= 1.2;
        println!("speed: {}", speed_control.value);
    }

    // speed down
    if input.just_pressed(KeyCode::Z) {
        speed_control.value *= 0.8;
        println!("speed: {}", speed_control.value);
    }
}