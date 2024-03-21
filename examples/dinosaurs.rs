//! Example with multiple tracked components, moving meshes, changing component values, and a moving camera.

use std::f32::consts::PI;
use std::time::Duration;

use bevy::pbr::*;
use bevy::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformRotationLens};
use bevy_tweening::{Animator, EaseFunction, Tracks, Tween, TweeningPlugin};

use bevy_health_bar3d::prelude::{
    BarHeight, BarSettings, ColorScheme, ForegroundColor, HealthBarPlugin, Percentage,
};

#[derive(Component, Reflect)]
struct Health {
    pub max: f32,
    pub current: f32,
}

#[derive(Component, Reflect)]
struct Distance {
    pub initial: f32,
    pub current: f32,
}

impl Distance {
    fn new(initial: f32) -> Self {
        Self {
            initial,
            current: initial,
        }
    }
}

impl Percentage for Health {
    fn value(&self) -> f32 {
        self.current / self.max
    }
}

impl Percentage for Distance {
    fn value(&self) -> f32 {
        self.current / self.initial
    }
}

fn main() {
    App::new()
        .register_type::<Health>()
        .register_type::<Distance>()
        .add_plugins((
            DefaultPlugins,
            HealthBarPlugin::<Distance>::default(),
            HealthBarPlugin::<Health>::default(),
            TweeningPlugin,
        ))
        .insert_resource(
            ColorScheme::<Distance>::new().foreground_color(ForegroundColor::Static(Color::BISQUE)),
        )
        .insert_resource(
            ColorScheme::<Health>::new()
                .foreground_color(ForegroundColor::Static(Color::GREEN))
                .background_color(Color::RED),
        )
        .insert_resource(Msaa::Sample4)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                move_camera,
                link_animations,
                setup_idle_animation,
                setup_walking_animation,
                move_trex,
                kill_trex,
            ),
        )
        .run();
}

#[derive(Resource)]
struct Animations {
    walk: Handle<AnimationClip>,
    idle: Handle<AnimationClip>,
    die: Handle<AnimationClip>,
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
        mesh: meshes.add(Plane3d::default().mesh().size(1000.0, 1000.0)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        ..Default::default()
    });

    // Animations
    commands.insert_resource(Animations {
        walk: asset_server.load(gltf_path("Animation5")),
        idle: asset_server.load(gltf_path("Animation2")),
        die: asset_server.load(gltf_path("Animation1")),
    });

    let scene = asset_server.load(gltf_path("Scene0"));

    // T-Rex 1
    commands.spawn((
        SceneBundle {
            scene: scene.clone(),
            transform: Transform::from_xyz(25., 0., -50.),
            ..default()
        },
        Distance::new(80.),
        Health {
            max: 10.,
            current: 8.,
        },
        BarSettings::<Distance> {
            offset: 15.,
            height: BarHeight::Static(1.),
            width: 10.,
            ..default()
        },
        BarSettings::<Health> {
            offset: 17.,
            height: BarHeight::Static(1.),
            width: 10.,
            ..default()
        },
    ));

    // T-Rex 2
    commands.spawn((
        SceneBundle { scene, ..default() },
        Health {
            max: 10.,
            current: 10.,
        },
        BarSettings::<Health> {
            offset: 17.,
            height: BarHeight::Static(1.),
            width: 10.,
            ..default()
        },
    ));

    // Sunlight
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 50.0, 20.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(100., 90., 100.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        FogSettings {
            color: Color::rgba(1., 1., 1., 1.),
            falloff: FogFalloff::Linear {
                start: 200.,
                end: 400.,
            },
            ..default()
        },
    ));
}

fn setup_walking_animation(
    animations: Res<Animations>,
    dinos: Query<&WithAnimationPlayer, (With<Distance>, Added<WithAnimationPlayer>)>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for &WithAnimationPlayer(entity) in dinos.iter() {
        players
            .get_mut(entity)
            .unwrap()
            .play(animations.walk.clone())
            .repeat();
    }
}

fn setup_idle_animation(
    animations: Res<Animations>,
    dinos: Query<&WithAnimationPlayer, (Without<Distance>, Added<WithAnimationPlayer>)>,
    mut players: Query<&mut AnimationPlayer>,
) {
    for &WithAnimationPlayer(entity) in dinos.iter() {
        players
            .get_mut(entity)
            .unwrap()
            .play(animations.idle.clone())
            .repeat();
    }
}

fn kill_trex(
    animations: Res<Animations>,
    mut commands: Commands,
    mut query: Query<(&mut Health, &WithAnimationPlayer, Entity), Without<Distance>>,
    mut players: Query<&mut AnimationPlayer>,
    time: Res<Time>,
) {
    query
        .iter_mut()
        .filter(|(health, _, _)| health.current > 0.)
        .for_each(|(mut health, with_animation_player, entity)| {
            let delta_z = time.delta_seconds();
            health.current -= delta_z;

            if health.current <= 0.01 {
                commands.entity(entity).remove::<Health>();
                players
                    .get_mut(with_animation_player.0)
                    .unwrap()
                    .play_with_transition(animations.die.clone(), Duration::from_millis(120));
            }
        })
}

fn move_trex(
    animations: Res<Animations>,
    mut players: Query<&mut AnimationPlayer>,
    mut query: Query<(&mut Transform, &mut Distance, &WithAnimationPlayer)>,
    time: Res<Time>,
) {
    query
        .iter_mut()
        .filter(|(_, distance, _)| distance.current > 0.)
        .for_each(|(mut transform, mut distance, with_animation_player)| {
            let delta_z = time.delta_seconds() * 10.;
            transform.translation.z += delta_z;
            distance.current -= delta_z;

            if distance.current <= 1. {
                players
                    .get_mut(with_animation_player.0)
                    .unwrap()
                    .play_with_transition(animations.idle.clone(), Duration::from_millis(120))
                    .repeat();
            }
        })
}

#[derive(Component)]
struct Moving;

#[allow(clippy::type_complexity)]
fn move_camera(
    mut commands: Commands,
    mut camera_query: Query<(Entity, &Transform), (With<Camera3d>, Without<Moving>)>,
) {
    camera_query.iter_mut().for_each(|(entity, transform)| {
        commands.entity(entity).insert(Moving);

        let Vec3 { x, y, z } = transform.translation;

        let target = Transform::from_xyz(x + 20., y - 30., z - 50.).looking_at(Vec3::ZERO, Vec3::Y);

        let translation_tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_secs(5),
            TransformPositionLens {
                start: transform.translation,
                end: target.translation,
            },
        );

        let rotation_tween = Tween::new(
            EaseFunction::QuadraticIn,
            Duration::from_secs(5),
            TransformRotationLens {
                start: transform.rotation,
                end: target.rotation,
            },
        );

        commands.entity(entity).insert(Animator::new(Tracks::new([
            translation_tween,
            rotation_tween,
        ])));
    });
}

#[derive(Component)]
pub struct WithAnimationPlayer(pub Entity);

fn get_root(mut entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    while let Ok(parent) = parent_query.get(entity) {
        entity = parent.get()
    }
    entity
}

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&WithAnimationPlayer>,
    mut commands: Commands,
) {
    for entity in player_query.iter() {
        let root = get_root(entity, &parent_query);
        if animations_entity_link_query.get(root).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
            return;
        }

        commands.entity(root).insert(WithAnimationPlayer(entity));
    }
}
