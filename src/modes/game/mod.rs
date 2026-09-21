pub mod dialog;
pub mod fight;
pub mod hud;
pub mod move_;

use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;
use crate::states::{GameState, InGameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<crate::types::GameSettings>()
           .init_resource::<crate::types::Score>()
           .add_systems(OnEnter(GameState::InGame), setup_game)
           .add_systems(
                Update,
                rotate_triangle.run_if(in_state(InGameState::Playing)),
            )
           .add_systems(OnExit(GameState::InGame), cleanup_game);
    }
}

#[derive(Component)]
struct Triangle;

fn setup_game(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let r = 100.0_f32;
    let half = r * 3.0_f32.sqrt() / 2.0;

    let mut mesh = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[0.0, r, 0.0], [-half, -r / 2.0, 0.0], [half, -r / 2.0, 0.0]],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_COLOR,
        vec![
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
        ],
    );

    commands.spawn((
        Triangle,
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::default())),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn rotate_triangle(time: Res<Time>, mut query: Query<&mut Transform, With<Triangle>>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_secs() * 1.5);
    }
}

fn cleanup_game(mut commands: Commands, query: Query<Entity, With<Triangle>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
