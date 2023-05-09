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
    BarBorder, BarBundle, BarHeight, BarOffset, BarWidth, HealthBarPlugin, Percentage,
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
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(HealthBarPlugin::<Health>::default())
        .add_startup_system(setup)
        .insert_resource(Msaa::Sample4)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.15;
    let values = [2.0f32, 5., 9.];

    let bar_width = radius * 2.;
    let bar_height = bar_width / 6.;
    let offset = radius * 1.5;

    values.into_iter().enumerate().for_each(|(i, value)| {
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
                transform: Transform::from_xyz(2. * radius, 0.4 + i as f32 / 2., 0.0),
                ..Default::default()
            },
            Health {
                max: 10.,
                current: value,
            },
            BarBundle::<Health> {
                offset: BarOffset::new(offset),
                width: BarWidth::new(bar_width),
                height: BarHeight::Static(bar_height),
                border: BarBorder::new(bar_height / 4.).color(Color::PURPLE),
                ..default()
            },
        ));
    });

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
