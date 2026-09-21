mod states;
mod types;
mod modes;
mod actor;
mod controller;

use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};

use states::{GameState, InGameState};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Animeow".into(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Current),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                ..default()
            },
        })
        .insert_resource(ClearColor(Color::BLACK))
        .init_state::<GameState>()
        .add_sub_state::<InGameState>()
        .add_systems(Startup, setup_camera)
        .add_plugins((
            modes::ModesPlugin,
            actor::ActorPlugin,
            controller::ControllerPlugin,
        ))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Msaa::Off));
}
