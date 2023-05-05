use bevy::app::App;
use bevy::asset::Assets;
use bevy::pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial};
use bevy::prelude::{
    shape, Camera3dBundle, Color, Commands, Component, Mesh, Msaa, Query, Reflect, Res, ResMut,
    Transform, Vec3,
};
use bevy::time::Time;
use bevy::utils::default;
use bevy::DefaultPlugins;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_health_bar3d::prelude::{BarBundle, BarOffset, BarWidth, HealthBarPlugin, Percentage};

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
        .add_system(update_health)
        .insert_resource(Msaa { samples: 4 })
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.15;

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius,
                ..default()
            })),
            material: materials.add(Color::rgb(1., 0.2, 0.2).into()),
            transform: Transform::from_xyz(0., 1., 0.0),
            ..Default::default()
        },
        Health {
            max: 10.,
            current: 10.,
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
            shadows_enabled: false,
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 1.5, 4.0).looking_at(Vec3::Y, Vec3::Y),
        ..Default::default()
    });
}

fn update_health(time: Res<Time>, mut query: Query<&mut Health>) {
    query.iter_mut().for_each(|mut health| {
        health.current -= 5. * time.delta_seconds();

        if health.current < 0. {
            health.current = health.max
        }
    })
}
