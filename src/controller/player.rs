use crate::core::input::InputState;
use crate::states::{GameState, InGameState};
use crate::types::Player;
use bevy::prelude::*;

pub struct PlayerControllerPlugin;

impl Plugin for PlayerControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player)
            .add_systems(Update, move_player.run_if(in_state(InGameState::Playing)))
            .add_systems(OnExit(GameState::InGame), cleanup_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Player,
        Mesh3d(meshes.add(Cuboid::new(1.0, 2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
}

/// Временное движение: WASD через InputState, без физики, без поворота.
/// Кинематический контроллер (ускорение, гравитация, поворот к движению)
/// — следующий заход.
fn move_player(
    input: Res<InputState>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = 5.0;
    for mut transform in &mut query {
        if input.move_dir == Vec2::ZERO {
            continue;
        }
        // input.move_dir: x — вправо, y — вперёд (в локальных осях мира).
        // Пока без учёта yaw камеры — это следующий заход.
        let dir = Vec3::new(input.move_dir.x, 0.0, -input.move_dir.y);
        transform.translation += dir * speed * time.delta_secs();
    }
}

fn cleanup_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
