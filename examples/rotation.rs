//! Example with multiple rotating meshes and a moving camera.
//! Bars always face the camera.

use std::f32::consts::PI;

use bevy::app::App;
use bevy::asset::Assets;
use bevy::pbr::*;
use bevy::prelude::*;
use bevy::utils::default;
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_health_bar3d::prelude::{
    BarBundle, BarHeight, BarOffset, BarWidth, HealthBarPlugin, Percentage,
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

#[derive(Component)]
struct Jim;

#[derive(Component)]
struct Tom;

fn main() {
    App::new()
        .register_type::<Health>()
        .add_plugins((
            DefaultPlugins,
            WorldInspectorPlugin::new(),
            HealthBarPlugin::<Health>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_camera, rotate_jim, rotate_tom))
        .run();
}

const TREX_GLTF: &str = "../examples/assets/models/trex.gltf";

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let gltf_path = |id: &str| format!("{TREX_GLTF}#{id}");

    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 0,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let scene = asset_server.load(gltf_path("Scene0"));

    // T-Rex Jim
    commands.spawn((
        SceneBundle {
            scene: scene.clone(),
            transform: Transform {
                translation: Vec3::new(-1., 0., 0.),
                scale: Vec3::splat(0.04),
                ..default()
            },
            ..default()
        },
        Health {
            max: 10.,
            current: 8.,
        },
        BarBundle::<Health> {
            offset: BarOffset::new(18.),
            height: BarHeight::Static(1.),
            width: BarWidth::new(10.),
            ..default()
        },
        Jim,
        Name::new("Jim"),
    ));

    // T-Rex Tom
    commands.spawn((
        SceneBundle {
            scene: scene.clone(),
            transform: Transform {
                translation: Vec3::new(1., 0.75, 0.),
                scale: Vec3::splat(0.04),
                ..default()
            },
            ..default()
        },
        Health {
            max: 10.,
            current: 3.,
        },
        BarBundle::<Health> {
            // Have to locate it higher than Jim's so there is no clipping during rotation
            offset: BarOffset::new(21.),
            height: BarHeight::Static(1.),
            width: BarWidth::new(10.),
            ..default()
        },
        Tom,
        Name::new("Tom"),
    ));

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 3.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn rotate_camera(
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    mut angle: Local<f32>,
    time: Res<Time>,
) {
    let mut transform = camera_query.single_mut();
    let mut target_angle = *angle + 10. * time.delta_seconds();

    if target_angle > 360. {
        target_angle = 0.;
    }

    transform.translation.x = 5. * target_angle.to_radians().cos();
    transform.translation.z = 5. * target_angle.to_radians().sin();

    *angle = target_angle;
    *transform = transform.looking_at(Vec3::ZERO, Vec3::Y);
}

fn rotate_jim(mut jim_query: Query<&mut Transform, With<Jim>>, timer: Res<Time>) {
    let mut transform = jim_query.single_mut();
    transform.rotate_y(PI * timer.delta_seconds());
}

fn rotate_tom(mut tom_query: Query<&mut Transform, With<Tom>>, timer: Res<Time>) {
    let mut transform = tom_query.single_mut();
    transform.rotate_x(PI * timer.delta_seconds());
}
