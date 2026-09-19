use bevy::prelude::*;
use crate::states::{GameState, InGameState};
use crate::types::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player)
           .add_systems(
                Update,
                move_player.run_if(in_state(InGameState::Playing)),
            )
           .add_systems(OnExit(GameState::InGame), cleanup_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player,
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.5, 0.0))),
        Transform::from_xyz(0.0, -200.0, 1.0),
    ));
}

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    settings: Res<GameSettings>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    for mut transform in &mut query {
        let mut direction = Vec2::ZERO;
        if keys.pressed(KeyCode::ArrowLeft) || keys.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keys.pressed(KeyCode::ArrowUp) || keys.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::ArrowDown) || keys.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if direction != Vec2::ZERO {
            let direction = direction.normalize();
            transform.translation.x += direction.x * settings.player_speed * time.delta_secs();
            transform.translation.y += direction.y * settings.player_speed * time.delta_secs();
        }
    }
}

fn cleanup_player(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
