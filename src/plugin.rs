use std::marker::PhantomData;

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
};
use bevy::prelude::{Added, App, Assets, BuildChildren, Changed, Commands, Component, default, Entity, Handle, MaterialMeshBundle, MaterialPlugin, Mesh, Name, Plugin, Query, Reflect, Res, ResMut, shape, Transform, Vec2, Vec3};

use crate::configuration::{ForegroundColor, BarHeight, BarOffset, BarWidth, Percentage};
use crate::constants::{DEFAULT_RELATIVE_HEIGHT, DEFAULT_WIDTH};
use crate::material::HealthBarMaterial;
use crate::mesh::MeshHandles;
use crate::prelude::ColorScheme;

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
    query: Query<(Entity, &T, Option<&BarOffset<T>>, Option<&BarWidth<T>>, Option<&BarHeight<T>>), Added<T>>,
) {
    query.iter().for_each(|(entity, percentage, offset, width, height)| {
        let width = width.map(|it| it.get()).unwrap_or(DEFAULT_WIDTH);
        let height = height.map(|it| match it {
            BarHeight::Relative(pct) => pct * width,
            BarHeight::Static(height, _) => *height
        }).unwrap_or(width * DEFAULT_RELATIVE_HEIGHT);

        let mesh = mesh_handles.get(width, height).cloned().unwrap_or_else(|| {
            mesh_handles.insert(width, height, meshes.add(Mesh::from(shape::Quad::new(Vec2::new(width, height)))))
        });

        let offset = offset.map(|it| it.get()).unwrap_or(0.);
        let transform = Transform::from_translation(offset * Vec3::Y);

        let (high, moderate, low) = match color_scheme.foreground_color {
            ForegroundColor::Static(color) => (color, color, color),
            ForegroundColor::TriSpectrum { high, moderate, low } => (high, moderate, low)
        };

        let material = materials.add(HealthBarMaterial {
            value: percentage.value(),
            background_color: color_scheme.background_color,
            high_color: high,
            moderate_color: moderate,
            low_color: low
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