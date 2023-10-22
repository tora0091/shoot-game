use bevy::{prelude::*, window::WindowResolution, sprite::MaterialMesh2dBundle};

const WINDOW_SIZE_WIDTH: f32 = 500.0;
const WINDOW_SIZE_HEIGHT: f32 = 610.0;

const PLAYER_RADIUS: f32 = 20.0;
const PLAYER_VELOCITY: f32 = 3.0;

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
}

fn player_move_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
) {
    let (mut player_transform, mut player_position) = query.single_mut();

    if input.pressed(KeyCode::Up) {
        player_position.y += PLAYER_VELOCITY;
    }

    if input.pressed(KeyCode::Down) {
        player_position.y -= PLAYER_VELOCITY;
    }

    if input.pressed(KeyCode::Right) {
        player_position.x += PLAYER_VELOCITY;
    }

    if input.pressed(KeyCode::Left) {
        player_position.x -= PLAYER_VELOCITY;
    }

    player_transform.translation = player_position.get_position();
}

fn player_in_window_system(
    mut query: Query<(&mut Transform, &mut Player), With<Player>>,
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