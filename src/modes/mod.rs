pub mod loading;
pub mod pause;
pub mod vehicle;
pub mod game;

use bevy::prelude::*;

pub struct ModesPlugin;

impl Plugin for ModesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            loading::LoadingPlugin,
            pause::PausePlugin,
            vehicle::VehiclePlugin,
            game::GamePlugin,
        ));
    }
}
