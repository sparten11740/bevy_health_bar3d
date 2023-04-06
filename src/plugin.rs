use std::marker::PhantomData;

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
};
use bevy::prelude::{Added, App, Assets, BuildChildren, Changed, Commands, Component, default, Entity, Handle, MaterialMeshBundle, MaterialPlugin, Mesh, Name, Plugin, Query, Reflect, Res, ResMut, shape, Transform, Vec2, Vec3};

use crate::configuration::{BarHeight, BarOffset, BarWidth, ForegroundColor, Percentage};
use crate::constants::{DEFAULT_RELATIVE_HEIGHT, DEFAULT_WIDTH};
use crate::material::HealthBarMaterial;
use crate::mesh::MeshHandles;
use crate::prelude::{BarOrientation, ColorScheme};

pub struct HealthBarPlugin<T: Percentage + Component> {
    phantom: PhantomData<T>,
}

impl<T: Percentage + Component> Default for HealthBarPlugin<T> {
    fn default() -> Self {
        Self { phantom: Default::default() }
    }
}

impl<T: Percentage + Component> Plugin for HealthBarPlugin<T> {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<MaterialPlugin::<HealthBarMaterial>>() {
            app
                .add_plugin(MaterialPlugin::<HealthBarMaterial>::default())
                .register_type::<WithHealthBar>();
        }

        app
            .init_resource::<MeshHandles>()
            .init_resource::<ColorScheme<T>>()
            .register_type::<BarWidth<T>>()
            .register_type::<BarOffset<T>>()
            .add_system(spawn::<T>)
            .add_system(update::<T>);
    }
}

#[derive(Component, Reflect)]
struct WithHealthBar(Entity);

impl WithHealthBar {
    fn get(&self) -> Entity {
        self.0
    }
}

#[allow(clippy::type_complexity)]
fn spawn<T: Percentage + Component>(
    mut commands: Commands,
    mut materials: ResMut<Assets<HealthBarMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_handles: ResMut<MeshHandles>,
    color_scheme: Res<ColorScheme<T>>,
    query: Query<(Entity, &T, Option<&BarOffset<T>>, Option<&BarWidth<T>>, Option<&BarHeight<T>>, Option<&BarOrientation<T>>), Added<T>>,
) {
    query.iter().for_each(|(entity, percentage, offset, width, height, orientation)| {
        let width = width.map(|it| it.get()).unwrap_or(DEFAULT_WIDTH);
        let orientation = orientation.unwrap_or(&BarOrientation::Horizontal);

        let height = height.map(|it| match it {
            BarHeight::Relative(pct) => pct * width,
            BarHeight::Static(height) => *height
        }).unwrap_or(width * DEFAULT_RELATIVE_HEIGHT);

        let (width, height, vertical, offset_axis) = match orientation {
            BarOrientation::Horizontal => (width, height, false, Vec3::Y),
            BarOrientation::Vertical => (height, width, true, Vec3::X),
        };

        let mesh = mesh_handles.get(width, height).cloned().unwrap_or_else(|| {
            mesh_handles.insert(width, height, meshes.add(Mesh::from(shape::Quad::new(Vec2::new(width, height)))))
        });

        let offset = offset.map(|it| it.get()).unwrap_or(0.);
        let transform = Transform::from_translation(offset * offset_axis);


        let (high, moderate, low) = match color_scheme.foreground_color {
            ForegroundColor::Static(color) => (color, color, color),
            ForegroundColor::TriSpectrum { high, moderate, low } => (high, moderate, low)
        };

        let material = materials.add(HealthBarMaterial {
            value: percentage.value(),
            background_color: color_scheme.background_color,
            high_color: high,
            moderate_color: moderate,
            low_color: low,
            vertical
        });

        let health_bar = commands
            .spawn((
                Name::new(format!("{}Bar", std::any::type_name::<T>().split("::").last().unwrap_or("Unknown"))),
                MaterialMeshBundle {
                    mesh,
                    material,
                    transform,
                    ..default()
                },
                NotShadowCaster,
                NotShadowReceiver,
            ))
            .id();

        commands
            .entity(entity)
            .insert(WithHealthBar(health_bar))
            .add_child(health_bar);
    });
}

fn update<T: Percentage + Component>(
    mut materials: ResMut<Assets<HealthBarMaterial>>,
    parent_query: Query<(&WithHealthBar, &T), Changed<T>>,
    health_bar_query: Query<&Handle<HealthBarMaterial>>,
) {
    parent_query
        .iter()
        .for_each(|(health_bar_child, hitpoints)| {
            let Ok(material_handle) = health_bar_query.get(health_bar_child.get()) else { return; };
            let material = materials.get_mut(material_handle).unwrap();
            material.value = hitpoints.value();
        });
}