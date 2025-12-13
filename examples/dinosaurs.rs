//! Example with multiple tracked components, moving meshes, changing component values, and a moving camera.

use bevy::animation::RepeatAnimation;
use bevy::color::palettes::css::*;
use bevy::pbr::*;
use bevy::prelude::*;
use bevy_tweening::lens::{TransformPositionLens, TransformRotationLens};
use bevy_tweening::Sequence;
use bevy_tweening::TweenAnim;
use bevy_tweening::{Tween, TweeningPlugin};
use std::f32::consts::PI;
use std::time::Duration;

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
            ColorScheme::<Distance>::new().foreground_color(ForegroundColor::Static(BISQUE.into())),
        )
        .insert_resource(
            ColorScheme::<Health>::new()
                .foreground_color(ForegroundColor::Static(GREEN.into()))
                .background_color(RED.into()),
        )
        .add_systems(Startup, (setup, setup_animations))
        .add_systems(
            Update,
            (start_animations, move_camera, move_trex, kill_trex),
        )
        .run();
}

#[derive(Resource)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

const TREX_GLTF: &str = "models/trex.gltf";

fn gltf_path(id: &str) -> String {
    format!("{TREX_GLTF}#{id}")
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(1000.0, 1000.0))),
        MeshMaterial3d(materials.add(Color::srgba(0.3, 0.5, 0.3, 1.))),
    ));

    let scene = asset_server.load(gltf_path("Scene0"));

    // T-Rex 1
    commands.spawn((
        SceneRoot(scene.clone()),
        Transform::from_xyz(25., 0., -50.),
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
        SceneRoot(scene.clone()),
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
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0.0, 50.0, 20.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
    ));

    // Camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(100., 90., 100.0).looking_at(Vec3::ZERO, Vec3::Y),
        Msaa::Sample4,
        DistanceFog {
            color: Color::srgba(1., 1., 1., 1.),
            falloff: FogFalloff::Linear {
                start: 200.,
                end: 400.,
            },
            ..default()
        },
    ));
}

fn setup_animations(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut graphs: ResMut<Assets<AnimationGraph>>,
) {
    let walk_animation = asset_server.load(gltf_path("Animation5"));
    let idle_animation = asset_server.load(gltf_path("Animation2"));
    let die_animation = asset_server.load(gltf_path("Animation1"));

    let mut graph = AnimationGraph::new();
    let animations = graph
        .add_clips(
            [
                idle_animation.clone(),
                walk_animation.clone(),
                die_animation.clone(),
            ],
            1.0,
            graph.root,
        )
        .collect();

    let graph = graphs.add(graph);
    commands.insert_resource(Animations {
        animations,
        graph: graph.clone(),
    });
}

fn kill_trex(
    animations: Res<Animations>,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut Health, &WithAnimationPlayer, Entity), Without<Distance>>,
    mut players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
) {
    query
        .iter_mut()
        .filter(|(health, _, _)| health.current > 0.)
        .for_each(|(mut health, with_animation_player, entity)| {
            let delta_z = time.delta_secs();
            health.current -= delta_z;

            if health.current <= 0.01 {
                commands.entity(entity).remove::<Health>();

                let (mut player, mut transitions) = players
                    .get_mut(with_animation_player.0)
                    .expect("Animation player not found");

                transitions
                    .play(
                        &mut player,
                        animations.animations[2],
                        Duration::from_millis(120),
                    )
                    .set_repeat(RepeatAnimation::Never);
            }
        })
}

fn move_trex(
    animations: Res<Animations>,
    time: Res<Time>,
    mut players: Query<(&mut AnimationPlayer, &mut AnimationTransitions)>,
    mut query: Query<(&mut Transform, &mut Distance, &WithAnimationPlayer)>,
) {
    query
        .iter_mut()
        .filter(|(_, distance, _)| distance.current > 0.)
        .for_each(|(mut transform, mut distance, with_animation_player)| {
            let delta_z = time.delta_secs() * 10.;
            transform.translation.z += delta_z;
            distance.current -= delta_z;

            if distance.current <= 0. {
                let (mut player, mut transitions) = players
                    .get_mut(with_animation_player.0)
                    .expect("Animation player not found");

                player.stop(animations.animations[1]);

                transitions
                    .play(
                        &mut player,
                        animations.animations[0],
                        Duration::from_millis(120),
                    )
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

        commands
            .entity(entity)
            .insert(TweenAnim::new(Sequence::new([
                translation_tween,
                rotation_tween,
            ])));
    });
}

#[derive(Component)]
pub struct WithAnimationPlayer(pub Entity);

fn get_root(mut entity: Entity, parent_query: &Query<&ChildOf>) -> Entity {
    while let Ok(child_of) = parent_query.get(entity) {
        entity = child_of.parent()
    }
    entity
}

#[allow(clippy::type_complexity)]
fn start_animations(
    parent_query: Query<&ChildOf>,
    distance_query: Query<&Distance>,
    animations_entity_link_query: Query<&WithAnimationPlayer>,
    animations: Res<Animations>,
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut AnimationPlayer), Added<AnimationPlayer>>,
) {
    for (entity, mut player) in player_query.iter_mut() {
        let root = get_root(entity, &parent_query);
        if animations_entity_link_query.get(root).is_ok() {
            warn!("Problem with multiple animationsplayers for the same top parent");
            return;
        }

        commands.entity(root).insert(WithAnimationPlayer(entity));

        let index = if distance_query.contains(root) { 1 } else { 0 };
        let mut transitions = AnimationTransitions::new();

        transitions
            .play(&mut player, animations.animations[index], Duration::ZERO)
            .repeat();

        commands
            .entity(entity)
            .insert(AnimationGraphHandle(animations.graph.clone()))
            .insert(transitions);
    }
}
