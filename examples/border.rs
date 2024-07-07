use bevy::color::palettes::css::PURPLE;
use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_health_bar3d::prelude::{BarBorder, BarHeight, BarSettings, HealthBarPlugin, Percentage};

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
        .add_plugins((
            HealthBarPlugin::<Health>::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_camera)
        .insert_resource(Msaa::Sample4)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(5.0, 5.0)),
        material: materials.add(Color::srgba(0.3, 0.5, 0.3, 1.)),
        ..Default::default()
    });

    let radius = 0.15;
    let values = [2.0f32, 5., 9.];

    let bar_width = radius * 2.;
    let bar_height = bar_width / 6.;
    let offset = radius * 1.5;

    // Spawn one mesh and bar for each value defined above
    values.into_iter().enumerate().for_each(|(i, value)| {
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Sphere { radius }),
                material: materials.add(Color::srgba(1., 0.2, 0.2, 1.)),
                transform: Transform::from_xyz(2. * radius, 0.4 + i as f32 / 2., 0.0),
                ..Default::default()
            },
            Health {
                max: 10.,
                current: value,
            },
            BarSettings::<Health> {
                offset,
                width: bar_width,
                height: BarHeight::Static(bar_height),
                // here is where the border is defined
                border: BarBorder::new(bar_height / 4.).color(PURPLE.into()),
                ..default()
            },
        ));
    });

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
        transform: Transform::from_xyz(0., 1.5, 4.0).looking_at(Vec3::Y, Vec3::Y),
        ..Default::default()
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
