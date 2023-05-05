use bevy::app::App;
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial};
use bevy::prelude::{
    shape, Camera3dBundle, Color, Commands, Component, Mesh, Msaa, Reflect, ResMut, Transform, Vec3,
};
use bevy::utils::default;
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_health_bar3d::prelude::{
    BarBundle, BarOffset, BarWidth, ColorScheme, HealthBarPlugin, Percentage,
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
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(HealthBarPlugin::<Health>::default())
        .insert_resource(ColorScheme::<Health>::new().background_color(Color::RED))
        .add_startup_system(setup)
        .insert_resource(Msaa::Sample4)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane {
            size: 5.0,
            subdivisions: 0,
        })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });

    let radius = 0.2;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(
                TryInto::<Mesh>::try_into(shape::Icosphere {
                    radius,
                    ..default()
                })
                .unwrap(),
            ),
            material: materials.add(Color::rgb(1., 0.2, 0.2).into()),
            transform: Transform::from_xyz(0.0, 1., 0.0),
            ..Default::default()
        },
        Health {
            max: 10.,
            current: 8.,
        },
        BarBundle::<Health> {
            offset: BarOffset::new(radius * 1.5),
            width: BarWidth::new(radius * 2.),
            ..default()
        },
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

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
