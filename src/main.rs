mod actor;
mod controller;
mod core;
mod modes;
mod states;
mod types;

use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};

use states::{GameState, InGameState};

fn main() {
    println!("Animeow starting...");
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
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .init_state::<GameState>()
        .add_sub_state::<InGameState>()
        .add_plugins((
            core::CorePlugin,
            modes::ModesPlugin,
            actor::ActorPlugin,
            controller::ControllerPlugin,
        ))
        .run();
}
