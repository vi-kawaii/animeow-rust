pub mod character;
pub mod shoot;

use bevy::prelude::*;

pub struct ActorPlugin;

impl Plugin for ActorPlugin {
    fn build(&self, _app: &mut App) {}
}

/// Общая сущность: игрок, бот, NPC.
/// Пока пустой маркер — наполниим позже.
#[derive(Component)]
pub struct Actor;
