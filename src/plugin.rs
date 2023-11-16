use std::marker::PhantomData;

use bevy::asset::load_internal_asset;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;

use crate::configuration::{ForegroundColor, Percentage};
use crate::constants::BAR_SHADER_HANDLE;
use crate::material::BarMaterial;
use crate::mesh::MeshHandles;
use crate::prelude::{BarOrientation, BarSettings, ColorScheme};

pub struct HealthBarPlugin<T: Percentage + Component + TypePath> {
    phantom: PhantomData<T>,
}

impl<T: Percentage + Component + TypePath> Default for HealthBarPlugin<T> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

impl<T: Percentage + Component + TypePath> Plugin for HealthBarPlugin<T> {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<MaterialPlugin<BarMaterial>>() {
            app.add_plugins(MaterialPlugin::<BarMaterial>::default());

            load_internal_asset!(
                app,
                BAR_SHADER_HANDLE,
                "../assets/shaders/bar.wgsl",
                Shader::from_wgsl
            );
        }

        app.init_resource::<MeshHandles>()
            .init_resource::<ColorScheme<T>>()
            .register_type::<BarSettings<T>>()
            .add_systems(PostUpdate, reset_rotation)
            .add_systems(
                Update,
                (spawn::<T>, remove::<T>, update::<T>, update_settings::<T>),
            );
    }
}

#[derive(Component, Reflect)]
struct WithBar<T: Percentage + Component>(Entity, #[reflect(ignore)] PhantomData<T>);

impl<T: Percentage + Component> WithBar<T> {
    fn get(&self) -> Entity {
        self.0
    }
}

#[allow(clippy::type_complexity)]
fn spawn<T: Percentage + Component + TypePath>(
    mut commands: Commands,
    mut materials: ResMut<Assets<BarMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_handles: ResMut<MeshHandles>,
    color_scheme: Res<ColorScheme<T>>,
    query: Query<(Entity, &T, &BarSettings<T>), Added<T>>,
) {
    query.iter().for_each(|(entity, percentage, settings)| {
        let width = settings.normalized_width();
        let height = settings.normalized_height();

        let mesh = mesh_handles.get(width, height).cloned().unwrap_or_else(|| {
            mesh_handles.insert(
                width,
                height,
                meshes.add(Mesh::from(shape::Quad::new(Vec2::new(width, height)))),
            )
        });

        let (high, moderate, low) = match color_scheme.foreground_color {
            ForegroundColor::Static(color) => (color, color, color),
            ForegroundColor::TriSpectrum {
                high,
                moderate,
                low,
            } => (high, moderate, low),
        };

        let material = materials.add(BarMaterial {
            value_and_dimensions: (percentage.value(), width, height, settings.border.width).into(),
            background_color: color_scheme.background_color,
            high_color: high,
            moderate_color: moderate,
            low_color: low,
            vertical: settings.orientation == BarOrientation::Vertical,
            offset: settings.normalized_offset().extend(0.),
            border_color: settings.border.color,
        });

        let health_bar = commands
            .spawn((
                Name::new(format!("{}Bar", T::type_path())),
                MaterialMeshBundle {
                    mesh,
                    material,
                    ..default()
                },
                NotShadowCaster,
                NotShadowReceiver,
            ))
            .id();

        commands
            .entity(entity)
            .insert(WithBar(health_bar, PhantomData::<T>))
            .add_child(health_bar);
    });
}

fn update<T: Percentage + Component + TypePath>(
    mut materials: ResMut<Assets<BarMaterial>>,
    parent_query: Query<(&WithBar<T>, &T), Changed<T>>,
    bar_query: Query<&Handle<BarMaterial>>,
) {
    parent_query.iter().for_each(|(bar, percentage)| {
        let Ok(material_handle) = bar_query.get(bar.get()) else {
            return;
        };
        let material = materials.get_mut(material_handle).unwrap();
        material.value_and_dimensions.x = percentage.value();
    });
}

#[allow(clippy::type_complexity)]
fn update_settings<T: Percentage + Component + TypePath>(
    mut materials: ResMut<Assets<BarMaterial>>,
    parent_query: Query<(&WithBar<T>, &BarSettings<T>), Changed<BarSettings<T>>>,
    bar_query: Query<&Handle<BarMaterial>>,
) {
    parent_query.iter().for_each(|(bar, settings)| {
        let Ok(material_handle) = bar_query.get(bar.get()) else {
            return;
        };

        let material = materials.get_mut(material_handle).unwrap();
        let offset = settings.normalized_offset().extend(0.);

        if material.offset != offset {
            material.offset = offset
        }

        if material.border_color != settings.border.color {
            material.border_color = settings.border.color
        }

        if material.value_and_dimensions.w != settings.border.width {
            material.value_and_dimensions.w = settings.border.width
        }
    });
}

fn remove<T: Percentage + Component>(
    mut commands: Commands,
    mut removals: RemovedComponents<T>,
    parent_query: Query<&WithBar<T>>,
) {
    removals.read().for_each(|entity| {
        let Ok(&WithBar(bar_entity, _)) = parent_query.get(entity) else {
            return;
        };

        commands.entity(bar_entity).despawn_recursive()
    });
}

fn reset_rotation(
    mut bar_query: Query<(&Parent, &mut Transform), With<Handle<BarMaterial>>>,
    q_transform: Query<&Transform, Without<Handle<BarMaterial>>>,
) {
    for (parent, mut transform) in bar_query.iter_mut() {
        if let Ok(parent_transform) = q_transform.get(parent.get()) {
            transform.rotation = parent_transform.rotation.inverse();
        }
    }
}
