use bevy::prelude::*;
use bevy::asset::RenderAssetUsages;
use crate::states::{GameState, InGameState};
use crate::types::*;

pub struct VehiclePlugin;

impl Plugin for VehiclePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_vehicle)
           .add_systems(
                Update,
                rotate_vehicle.run_if(in_state(InGameState::Playing)),
            )
           .add_systems(OnExit(GameState::InGame), cleanup_vehicle);
    }
}

fn spawn_vehicle(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let r = 50.0_f32;
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
        Vehicle,
        Mesh2d(meshes.add(mesh)),
        MeshMaterial2d(materials.add(ColorMaterial::default())),
        Transform::from_xyz(0.0, 200.0, 0.0),
    ));
}

fn rotate_vehicle(time: Res<Time>, mut query: Query<&mut Transform, With<Vehicle>>) {
    for mut transform in &mut query {
        transform.rotate_z(time.delta_secs() * 1.5);
    }
}

fn cleanup_vehicle(
    mut commands: Commands,
    query: Query<Entity, With<Vehicle>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
