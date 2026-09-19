use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Vehicle;

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource)]
pub struct GameSettings {
    pub player_speed: f32,
    pub vehicle_speed: f32,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            player_speed: 200.0,
            vehicle_speed: 400.0,
        }
    }
}
