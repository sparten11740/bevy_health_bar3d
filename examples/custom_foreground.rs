use bevy::app::App;
use bevy::asset::Assets;
use bevy::DefaultPlugins;
use bevy::pbr::{PbrBundle, PointLight, PointLightBundle, StandardMaterial};
use bevy::prelude::{Camera3dBundle, Color, Commands, Component, Mesh, Msaa, Reflect, ResMut, shape, Transform, Vec3};
use bevy::utils::default;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_health_bar3d::configuration::ForegroundColor;
use bevy_health_bar3d::prelude::{BarBundle, BarOffset, BarWidth, ColorScheme, HealthBarPlugin, Percentage};

#[derive(Component, Reflect)]
struct Mana {
    max: f32,
    current: f32,
}

impl Percentage for Mana {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}

fn main() {
    App::new()
        .register_type::<Mana>()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin)
        .add_plugin(HealthBarPlugin::<Mana>::default())
        .insert_resource(ColorScheme::<Mana>::new().foreground_color(ForegroundColor::Static(Color::BLUE)))
        .add_startup_system(setup)
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
        Mana {
            max: 10.,
            current: 8.,
        },
        BarBundle::<Mana> {
            offset: BarOffset::new(radius * 1.5),
            width: BarWidth::new(radius * 2.),
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

    commands.spawn(
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
    );
}