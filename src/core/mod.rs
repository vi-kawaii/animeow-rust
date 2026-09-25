pub mod camera;
pub mod input;

use bevy::prelude::{App, Plugin};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, input::InputPlugin));
    }
}
