use bevy::{prelude::{Resource, Component, Vec3}, time::Timer};

pub const WINDOW_SIZE_WIDTH: f32 = 400.0;
pub const WINDOW_SIZE_HEIGHT: f32 = 720.0;
pub const WINDOW_SIZE_MARGIN: f32 = 100.0;

pub const PLAYER_RADIUS: f32 = 14.0;
pub const PLAYER_VELOCITY: f32 = 3.0;

pub const ENEMY_RADIUS: f32 = 15.0;
pub const ENEMY_SPAWN_DURATION_SECONDS: f32 = 1.0;

pub const SHOOT_VELOCITY: f32 = 3.0;
pub const SHOOT_RADIUS: f32 = 5.0;

////////// Resource

#[derive(Resource)]
pub struct WindowSizeLimit {
    pub top: f32,
    pub bottom: f32,
    pub right: f32,
    pub left: f32,
}

impl WindowSizeLimit {
    pub fn new(top: f32, bottom: f32, right: f32, left: f32) -> Self {
        Self { top, bottom, right, left }
    }
}

#[derive(Resource)]
pub struct PlayerStatus {
    pub is_spawn: bool,
    pub timer: Timer,
    pub score: f32,
}

#[derive(Resource)]
pub struct EnemySpawn {
    pub counter: u32,
    pub timer: Timer,
}

#[derive(Resource)]
pub struct SpeedControl {
    pub value: f32,
}

#[derive(Resource)]
pub struct GameTimer {
    pub timer: Timer,
    pub seconds: u64,
}

////////// Component

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

pub enum ShootType {
    Normal,
    Double,
    Triple,
}


#[derive(Component)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub is_enable: bool,
    pub shoot_type: ShootType,
}

impl Player {
    pub fn get_position(&self) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z: self.z }
    }
    pub fn set_z_position(&self, z: f32) -> Vec3 {
        Vec3 { x: self.x, y: self.y, z }
    }
}

#[derive(Component)]
pub struct PlayerStartPosition;

#[derive(Component)]
pub struct Enemy {
    pub shoot_interval: Timer,
    pub point: f32,
}

#[derive(Component)]
pub struct AutoDespawn;

#[derive(Component)]
pub struct FromPlayerShoot;

#[derive(Component)]
pub struct FromEnemyShoot;

#[derive(Component)]
pub struct ShootBang {
    pub timer: Timer,
}

#[derive(Component)]
pub struct ShowBangPoint {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct ScoreBoard;