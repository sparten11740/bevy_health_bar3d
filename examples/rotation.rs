//! Example with multiple rotating meshes and a moving camera.
//! Bars always face the camera.

use std::f32::consts::PI;

use bevy::pbr::*;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_health_bar3d::prelude::{BarHeight, BarSettings, HealthBarPlugin, Percentage};

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

#[derive(Component)]
struct Jim;

#[derive(Component)]
struct Tom;

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
        .add_systems(Update, (rotate_camera, rotate_jim, rotate_tom))
        .run();
}

const TREX_GLTF: &str = "models/trex.gltf";

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let gltf_path = |id: &str| format!("{TREX_GLTF}#{id}");

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgba(0.3, 0.5, 0.3, 1.))),
    ));

    let scene = asset_server.load(gltf_path("Scene0"));

    // T-Rex Jim
    commands.spawn((
        SceneRoot(scene.clone()),
        Transform {
            translation: Vec3::new(-1., 0., 0.),
            scale: Vec3::splat(0.04),
            ..default()
        },
        Health {
            max: 10.,
            current: 8.,
        },
        BarSettings::<Health> {
            offset: 18.,
            height: BarHeight::Static(1.),
            width: 10.,
            ..default()
        },
        Jim,
        Name::new("Jim"),
    ));

    // T-Rex Tom
    commands.spawn((
        SceneRoot(scene.clone()),
        Transform {
            translation: Vec3::new(1., 0.75, 0.),
            scale: Vec3::splat(0.04),
            ..default()
        },
        Health {
            max: 10.,
            current: 3.,
        },
        BarSettings::<Health> {
            // Have to locate it higher than Jim's so there is no clipping during rotation
            offset: 21.,
            height: BarHeight::Static(1.),
            width: 10.,
            ..default()
        },
        Tom,
        Name::new("Tom"),
    ));

    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        Transform::from_xyz(0., 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn rotate_camera(
    mut transform: Single<&mut Transform, With<Camera3d>>,
    mut angle: Local<f32>,
    time: Res<Time>,
) {
    let mut target_angle = *angle + 10. * time.delta_secs();

    if target_angle > 360. {
        target_angle = 0.;
    }

    transform.translation.x = 5. * target_angle.to_radians().cos();
    transform.translation.z = 5. * target_angle.to_radians().sin();

    *angle = target_angle;
    transform.look_at(Vec3::ZERO, Vec3::Y);
}

fn rotate_jim(mut transform: Single<&mut Transform, With<Jim>>, timer: Res<Time>) {
    transform.rotate_y(PI * timer.delta_secs());
}

fn rotate_tom(mut transform: Single<&mut Transform, With<Tom>>, timer: Res<Time>) {
    transform.rotate_x(PI * timer.delta_secs());
}
