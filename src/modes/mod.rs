pub mod game;
pub mod loading;
pub mod pause;
pub mod vehicle;

use bevy::prelude::*;

pub struct ModesPlugin;

impl Plugin for ModesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((loading::LoadingPlugin, pause::PausePlugin, game::GamePlugin));
        // vehicle::VehiclePlugin — временно отключён (2D-реализация),
        // вернём на Этапе 5 (DESIGN.md §7).
    }
}
