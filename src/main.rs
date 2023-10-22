use bevy::{prelude::*, window::WindowResolution, sprite::MaterialMesh2dBundle};

const WINDOW_SIZE_WIDTH: f32 = 500.0;
const WINDOW_SIZE_HEIGHT: f32 = 610.0;

const PLAYER_SIZE: f32 = 20.0;
const PLAYER_VELOCITY: f32 = 3.0;

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
        .add_systems(Update, (
            plyaer_move_system,
            bevy::window::close_on_esc,
        ))
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.spawn(Camera2dBundle::default());

    // player
    commands.spawn(
        (MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(PLAYER_SIZE).into()).into(),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..default()
        },
        Player
    ));
}

#[derive(Component)]
struct Player;

fn plyaer_move_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = query.single_mut();

    if input.pressed(KeyCode::Up) {
        transform.translation.y += PLAYER_VELOCITY;
    }

    if input.pressed(KeyCode::Down) {
        transform.translation.y -= PLAYER_VELOCITY;

    }

    if input.pressed(KeyCode::Right) {
        transform.translation.x += PLAYER_VELOCITY;

    }

    if input.pressed(KeyCode::Left) {
        transform.translation.x -= PLAYER_VELOCITY;
    }
}