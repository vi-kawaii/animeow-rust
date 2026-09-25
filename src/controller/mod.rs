pub mod bot;
pub mod player;

use bevy::prelude::*;

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(player::PlayerControllerPlugin);
        // bot::BotControllerPlugin добавим позже
    }
}
