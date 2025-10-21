//! 2D Example: Spawns sprites with health bars that decrease over time.

use bevy::color::palettes::css::*;
use bevy::prelude::*;
//use bevy::sprite::SpriteBundle;
use bevy_health_bar3d::prelude::{
    BarHeight, BarSettings, ColorScheme, ForegroundColor, HealthBarPlugin, Percentage,
};

#[derive(Component, Reflect)]
struct Health {
    pub max: f32,
    pub current: f32,
}

impl Percentage for Health {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}

fn main() {
    App::new()
        .register_type::<Health>()
        .add_plugins((DefaultPlugins, HealthBarPlugin::<Health>::default()))
        .insert_resource(
            ColorScheme::<Health>::new()
                .foreground_color(ForegroundColor::Static(GREEN.into()))
                .background_color(RED.into()),
        )
        .add_systems(Startup, setup)
        .add_systems(Update, decrease_health)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 1000.0)));

    // Spawn 3 sprites with health bars
    for i in 0..3 {
        commands.spawn((
            Sprite {
                custom_size: Some(Vec2::splat(20.)),
                color: Color::BLACK,
                ..default()
            },
            Transform::from_xyz(i as f32 * 150.0 - 150.0, 0.0, 0.0),
            Health { max: 10.0, current: 10.0 - i as f32 * 3.0 },
            BarSettings::<Health> {
                //offset: -1.,
                height: BarHeight::Static(4.0),
                width: 40.0,
                ..default()
            },
        ));
    }
}

fn decrease_health(mut query: Query<&mut Health>, time: Res<Time>) {
    for mut health in &mut query {
        if health.current > 0.0 {
            health.current -= time.delta_secs();
            if health.current < 0.0 {
                health.current = 0.0;
            }
        }
    }
}
