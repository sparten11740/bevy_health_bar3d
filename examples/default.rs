use bevy::app::App;
use bevy::asset::Assets;
use bevy::DefaultPlugins;
use bevy::pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial};
use bevy::prelude::{Camera3d, Camera3dBundle, Color, Commands, Component, Local, Mesh, Msaa, Query, Reflect, Res, ResMut, shape, Time, Transform, Vec3, With};
use bevy::utils::default;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_health_bar3d::configuration::HealthBarWidth;
use bevy_health_bar3d::plugin::HealthBarPlugin;
use bevy_health_bar3d::prelude::{HealthBarOffset, Percentage};


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
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(HealthBarPlugin::<Health>::default())
        .add_startup_system(setup)
        .add_system(rotate_camera)
        .insert_resource(Msaa { samples: 4 })
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        }
    );


    let radius = 0.2;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius, ..default() })),
            material: materials.add(Color::rgb(1., 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 1., 0.0),
            ..Default::default()
        },
        Health {
            max: 10.,
            current: 8.,
        },
        HealthBarOffset(radius * 1.5),
        HealthBarWidth(radius * 2.)
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius, ..default() })),
            material: materials.add(Color::rgb(1., 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0 + 3. * radius, 0.5, 0.0),
            ..Default::default()
        },
        Health {
            max: 10.,
            current: 2.,
        },
        HealthBarOffset(radius * 1.5),
        HealthBarWidth(radius * 2.)
    ));

    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
    );
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