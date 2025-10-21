use std::marker::PhantomData;

use bevy::asset::load_internal_asset;
use bevy::prelude::*;

use crate::configuration::{ForegroundColor, Percentage};
use crate::mesh::MeshHandles;
use crate::prelude::{BarOrientation, BarSettings, ColorScheme};

// 3D-specific imports and type aliases
#[cfg(feature = "3d")]
use bevy::pbr::{MaterialPlugin, NotShadowCaster, NotShadowReceiver};
#[cfg(feature = "3d")]
use crate::constants::BAR_SHADER_HANDLE;
#[cfg(feature = "3d")]
use crate::material::BarMaterial;
#[cfg(feature = "3d")]
type Material = BarMaterial;
#[cfg(feature = "3d")]
type MeshComponent = Mesh3d;
#[cfg(feature = "3d")]
type MaterialComponent = MeshMaterial3d<BarMaterial>;

// 2D-specific imports and type aliases
#[cfg(feature = "2d")]
use bevy::sprite::Material2dPlugin;
#[cfg(feature = "2d")]
use crate::constants::BAR_SHADER_2D_HANDLE;
#[cfg(feature = "2d")]
use crate::material2d::BarMaterial2d;
#[cfg(feature = "2d")]
type Material = BarMaterial2d;
#[cfg(feature = "2d")]
type MeshComponent = Mesh2d;
#[cfg(feature = "2d")]
type MaterialComponent = MeshMaterial2d<BarMaterial2d>;

pub struct HealthBarPlugin<T: Percentage + Component + TypePath> {
    phantom: PhantomData<T>,
}

impl<T: Percentage + Component + TypePath> Default for HealthBarPlugin<T> {
    fn default() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<T: Percentage + Component + TypePath> Plugin for HealthBarPlugin<T> {
    fn build(&self, app: &mut App) {
        #[cfg(feature = "3d")]
        {
            if !app.is_plugin_added::<MaterialPlugin<BarMaterial>>() {
                app.add_plugins(MaterialPlugin::<BarMaterial>::default());
                load_internal_asset!(app, BAR_SHADER_HANDLE, "../assets/shaders/bar.wgsl", Shader::from_wgsl);
            }
        }

        #[cfg(feature = "2d")]
        {
            if !app.is_plugin_added::<Material2dPlugin<BarMaterial2d>>() {
                app.add_plugins(Material2dPlugin::<BarMaterial2d>::default());
                load_internal_asset!(app, BAR_SHADER_2D_HANDLE, "../assets/shaders/bar2d.wgsl", Shader::from_wgsl);
            }
        }

        app.init_resource::<MeshHandles>()
            .init_resource::<ColorScheme<T>>()
            .register_type::<BarSettings<T>>()
            .add_systems(PostUpdate, reset_rotation::<T>)
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
    mut materials: ResMut<Assets<Material>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_handles: ResMut<MeshHandles>,
    color_scheme: Res<ColorScheme<T>>,
    query: Query<(Entity, &T, &BarSettings<T>), Added<T>>,
) {
    query.iter().for_each(|(entity, percentage, settings)| {
        let width = settings.normalized_width();
        let height = settings.normalized_height();

        let mesh = mesh_handles.get(width, height).unwrap_or_else(|| {
            mesh_handles.insert(
                width,
                height,
                meshes.add(Mesh::from(Rectangle::new(width, height))),
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

        let material = materials.add(Material {
            value_and_dimensions: (percentage.value(), width, height, settings.border.width).into(),
            background_color: color_scheme.background_color.into(),
            high_color: high.into(),
            moderate_color: moderate.into(),
            low_color: low.into(),
            vertical: settings.orientation == BarOrientation::Vertical,
            offset: settings.normalized_offset().extend(0.),
            border_color: settings.border.color.into(),
        });

        #[cfg(feature = "3d")]
        let health_bar = commands
            .spawn((
                Name::new(format!("{}Bar", T::type_path())),
                Mesh3d(mesh.0),
                MeshMaterial3d(material),
                NotShadowCaster,
                NotShadowReceiver,
            ))
            .id();

        #[cfg(feature = "2d")]
        let health_bar = commands
            .spawn((
                Name::new(format!("{}Bar", T::type_path())),
                Mesh2d(mesh.0),
                MeshMaterial2d(material),
            ))
            .id();

        commands
            .entity(entity)
            .insert(WithBar(health_bar, PhantomData::<T>))
            .add_child(health_bar);
    });
}

fn update<T: Percentage + Component + TypePath>(
    mut materials: ResMut<Assets<Material>>,
    parent_query: Query<(&WithBar<T>, &T), Changed<T>>,
    bar_query: Query<&MaterialComponent>,
) {
    parent_query.iter().for_each(|(bar, percentage)| {
        let Ok(material_handle) = bar_query.get(bar.get()) else {
            return;
        };
        let material = materials.get_mut(&material_handle.0).unwrap();
        material.value_and_dimensions.x = percentage.value();
    });
}

#[allow(clippy::type_complexity)]
fn update_settings<T: Percentage + Component + TypePath>(
    mut commands: Commands,
    mut materials: ResMut<Assets<Material>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_handles: ResMut<MeshHandles>,
    parent_query: Query<(&WithBar<T>, &BarSettings<T>), Changed<BarSettings<T>>>,
    bar_query: Query<(Entity, &MaterialComponent, &MeshComponent)>,
) {
    parent_query.iter().for_each(|(bar, settings)| {
        let Ok((entity, material_handle, mesh_handle)) = bar_query.get(bar.get()) else {
            return;
        };

        let material = materials.get_mut(&material_handle.0).unwrap();
        let offset = settings.normalized_offset().extend(0.);
        let width = settings.normalized_width();
        let height = settings.normalized_height();

        let mesh_for_settings_dimensions = mesh_handles.get(width, height);
        let mesh_changed = mesh_for_settings_dimensions.as_ref().map(|m| &m.0) != Some(&mesh_handle.0);

        if mesh_changed {
            let new_mesh = mesh_for_settings_dimensions.unwrap_or_else(|| mesh_handles.insert(
                width,
                height,
                meshes.add(Mesh::from(Rectangle::new(width, height))),
            ));

            #[cfg(feature = "3d")]
            commands.entity(entity).insert(Mesh3d(new_mesh.0));

            #[cfg(feature = "2d")]
            commands.entity(entity).insert(Mesh2d(new_mesh.0));

            material.value_and_dimensions.y = width;
            material.value_and_dimensions.z = height;
        }

        material.offset = offset;
        material.border_color = settings.border.color.into();
        material.value_and_dimensions.w = settings.border.width;
        material.vertical = settings.orientation == BarOrientation::Vertical;
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

        if commands.get_entity(bar_entity).is_err() {
            return;
        }

        commands.entity(bar_entity).despawn()
    });
}

fn reset_rotation<T: Percentage + Component>(
    mut bar_query: Query<(&ChildOf, &mut Transform), With<MaterialComponent>>,
    q_transform: Query<&Transform, Without<MaterialComponent>>,
) {
    for (child_of, mut transform) in bar_query.iter_mut() {
        if let Ok(parent_transform) = q_transform.get(child_of.parent()) {
            transform.rotation = parent_transform.rotation.inverse();
        }
    }
}

