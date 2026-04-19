//! Demonstrates the difference between [`BarOffsetMode::CameraSpace`] and
//! [`BarOffsetMode::WorldSpace`].
//!
//! The camera sweeps from a side view to a top-down view.
//!
//! - **Left** (`CameraSpace`): the offset follows the camera's up vector, so
//!   as the camera moves overhead the bar drifts away from the object.
//! - **Right** (`WorldSpace`): the offset is applied along the world Y axis,
//!   so the bar stays anchored above the object regardless of camera angle.

use bevy::pbr::*;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_health_bar3d::prelude::{
    BarHeight, BarOffsetMode, BarSettings, HealthBarPlugin, Percentage,
};

#[derive(Component, Reflect)]
struct Health {
    max: f32,
    current: f32,
}

impl Percentage for Health {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}

fn main() {
    App::new()
        .register_type::<Health>()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            HealthBarPlugin::<Health>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, orbit_camera)
        .run();
}

// Half the cuboid height plus a small gap.
const OFFSET: f32 = 1.7;
const BAR_WIDTH: f32 = 1.2;
// Tall cuboid: 1 × 3 × 1. Center placed at y = 1.5 so the base sits on the ground.
const CUBOID_HALF_HEIGHT: f32 = 1.5;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    let mesh = meshes.add(Cuboid::new(1.0, CUBOID_HALF_HEIGHT * 2.0, 1.0));
    let mat = materials.add(Color::srgb(0.6, 0.35, 0.25));

    // Left — camera-space offset (default / legacy behavior).
    // The bar drifts forward when the camera looks down from above.
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_xyz(-2.0, CUBOID_HALF_HEIGHT, 0.0),
        Health {
            max: 10.,
            current: 7.,
        },
        BarSettings::<Health> {
            offset: OFFSET,
            width: BAR_WIDTH,
            height: BarHeight::Static(0.2),
            offset_mode: BarOffsetMode::CameraSpace,
            ..default()
        },
        Name::new("CameraSpace offset"),
    ));

    // Right — world-space offset.
    // The bar stays directly above the object regardless of camera angle.
    commands.spawn((
        Mesh3d(mesh.clone()),
        MeshMaterial3d(mat.clone()),
        Transform::from_xyz(2.0, CUBOID_HALF_HEIGHT, 0.0),
        Health {
            max: 10.,
            current: 7.,
        },
        BarSettings::<Health> {
            offset: OFFSET,
            width: BAR_WIDTH,
            height: BarHeight::Static(0.2),
            offset_mode: BarOffsetMode::WorldSpace,
            ..default()
        },
        Name::new("WorldSpace offset"),
    ));

    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        // Start from the side so both bars look identical initially.
        Transform::from_xyz(0.0, 3.0, 8.0)
            .looking_at(Vec3::new(0.0, CUBOID_HALF_HEIGHT, 0.0), Vec3::Y),
    ));
}

/// Sweeps the camera elevation from a low side-on angle to nearly overhead and back.
fn orbit_camera(
    mut transform: Single<&mut Transform, With<Camera3d>>,
    mut t: Local<f32>,
    time: Res<Time>,
) {
    *t += time.delta_secs() * 0.4;

    // Oscillate elevation between ~5° and ~85°.
    let elevation = (t.sin() * 0.5 + 0.5) * 80_f32.to_radians() + 5_f32.to_radians();
    let radius = 9.0;
    let target = Vec3::new(0.0, CUBOID_HALF_HEIGHT, 0.0);

    transform.translation =
        target + Vec3::new(0.0, radius * elevation.sin(), radius * elevation.cos());
    transform.look_at(target, Vec3::Y);
}
