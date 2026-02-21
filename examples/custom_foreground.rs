use bevy::color::palettes::basic::*;
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_health_bar3d::prelude::*;

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
        .register_type::<Mana>()
        .add_plugins((
            DefaultPlugins,
            EguiPlugin::default(),
            WorldInspectorPlugin::new(),
            HealthBarPlugin::<Mana>::default(),
            HealthBarPlugin::<Health>::default(),
        ))
        .insert_resource(
            ColorScheme::<Mana>::new().foreground_color(ForegroundColor::Static(BLUE.into())),
        )
        .insert_resource(ColorScheme::<Health>::new().foreground_color(
            ForegroundColor::TriSpectrum {
                high: LIMEGREEN.into(),
                moderate: ORANGE_RED.into(),
                low: PURPLE.into(),
            },
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.15;
    let values = [2.0f32, 5., 9.];

    values.into_iter().enumerate().for_each(|(i, value)| {
        commands.spawn((
            Mesh3d(meshes.add(Sphere { radius })),
            MeshMaterial3d(materials.add(Color::srgba(1., 0.2, 0.2, 1.))),
            Transform::from_xyz(-2. * radius, 0.4 + i as f32 / 2., 0.0),
            Mana {
                max: 10.,
                current: value,
            },
            BarSettings::<Mana> {
                offset: radius * 1.5,
                width: radius * 2.,
                ..default()
            },
        ));

        commands.spawn((
            Mesh3d(meshes.add(Sphere { radius })),
            MeshMaterial3d(materials.add(Color::srgba(1., 0.2, 0.2, 1.))),
            Transform::from_xyz(2. * radius, 0.4 + i as f32 / 2., 0.0),
            Health {
                max: 10.,
                current: value,
            },
            BarSettings::<Health> {
                offset: radius * 1.5,
                width: radius * 2.,
                ..default()
            },
        ));
    });

    // Light
    commands.spawn((
        PointLight {
            intensity: 1500.0,
            shadows_enabled: false,
            ..Default::default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Msaa::Sample4,
        Transform::from_xyz(0., 1.5, 4.0).looking_at(Vec3::Y, Vec3::Y),
    ));
}
