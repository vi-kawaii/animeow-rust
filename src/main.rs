use bevy::asset::RenderAssetUsages;
use bevy::dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin};
use bevy::prelude::*;
use bevy::window::{MonitorSelection, WindowMode};

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
                    // Используем FontSize::Px для размера в пикселях
                    font_size: FontSize::Px(24.0),
                    ..default()
                },
                text_color: Color::srgb(0.0, 1.0, 0.0),
                ..default()
            },
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_triangle)
        .run();
}

#[derive(Component)]
struct Triangle;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 2D-камера без сглаживания
    commands.spawn((Camera2d, Msaa::Off));

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
