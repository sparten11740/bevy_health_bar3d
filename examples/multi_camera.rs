use bevy::pbr::*;
use bevy::prelude::*;
use bevy_health_bar3d::prelude::{BarSettings, HealthBarPlugin, Percentage};
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

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
            EguiPlugin {
                enable_multipass_for_primary_context: true,
            },
            WorldInspectorPlugin::new(),
            HealthBarPlugin::<Health>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgba(0.3, 0.5, 0.3, 1.))),
    ));

    let radius = 0.2;

    commands.spawn((
        Mesh3d(meshes.add(Sphere { radius })),
        MeshMaterial3d(materials.add(Color::srgba(1., 0.2, 0.2, 1.))),
        Transform::from_xyz(0.0, 1., 0.0),
        Health {
            max: 10.,
            current: 8.,
        },
        BarSettings::<Health> {
            offset: radius * 1.5,
            width: radius * 2.,
            ..default()
        },
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere { radius })),
        MeshMaterial3d(materials.add(Color::srgba(1., 0.2, 0.2, 1.))),
        Transform::from_xyz(0.0 + 3. * radius, 0.5, 0.0),
        Health {
            max: 10.,
            current: 2.,
        },
        BarSettings::<Health> {
            offset: radius * 1.5,
            width: radius * 2.,
            ..default()
        },
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

    // Rotating Camera
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        Transform::from_xyz(0., 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        Rotate,
    ));

    // Static Camera
    commands.spawn((
        Camera3d::default(),
        Camera {
            clear_color: ClearColorConfig::None,
            order: 1,
            ..default()
        },
        Msaa::Sample4,
        Transform::from_xyz(1., 2., 5.),
    ));
}

#[derive(Component)]
struct Rotate;

fn rotate_camera(
    mut transform: Single<&mut Transform, With<Rotate>>,
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
