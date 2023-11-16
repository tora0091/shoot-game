use bevy::{prelude::*, window::WindowResolution};
use define::*;
use player::PlayerPlugin;
use enemy::EnemyPlugin;
use collision::CollisionPlugin;
use common::CommonPlugin;

mod player;
mod enemy;
mod define;
mod collision;
mod common;

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
        .add_plugins((
            PlayerPlugin,
            EnemyPlugin,
            CollisionPlugin,
            CommonPlugin,
        ))
        .add_systems(Startup, setup_system)
        .add_systems(Update, (
            game_timer_system,
            // show_point,
            bevy::window::close_on_esc
        ))
        .run();
}

fn setup_system(
    mut commands: Commands,
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

    // player spawn
    commands.insert_resource(PlayerStatus {
        is_spawn: true,
        timer: Timer::from_seconds(1.0, TimerMode::Once),
        score: 0.0,
    });

    // set game timer
    commands.insert_resource(GameTimer {
        timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        seconds: 0,
    });
}

fn game_timer_system(
    mut game_timer: ResMut<GameTimer>,
    time: Res<Time>,
) {
    if game_timer.timer.tick(time.delta()).just_finished() {
        game_timer.seconds += 1;
    }
}

fn show_point(
    player_status: Res<PlayerStatus>,
) {
    println!("{}", player_status.score)
}