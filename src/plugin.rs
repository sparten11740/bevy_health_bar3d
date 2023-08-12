use std::marker::PhantomData;

use bevy::asset::load_internal_asset;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;

use crate::configuration::{BarHeight, BarOffset, BarWidth, ForegroundColor, Percentage};
use crate::constants::{BAR_SHADER_HANDLE, DEFAULT_RELATIVE_HEIGHT, DEFAULT_WIDTH};
use crate::material::BarMaterial;
use crate::mesh::MeshHandles;
use crate::prelude::{BarBorder, BarOrientation, ColorScheme};

pub struct HealthBarPlugin<T: Percentage + Component> {
    phantom: PhantomData<T>,
}

impl<T: Percentage + Component> Default for HealthBarPlugin<T> {
    fn default() -> Self {
        Self {
            phantom: Default::default(),
        }
    }
}

impl<T: Percentage + Component> Plugin for HealthBarPlugin<T> {
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
            .add_systems(Update, (spawn::<T>, remove::<T>, update::<T>));
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
fn spawn<T: Percentage + Component>(
    mut commands: Commands,
    mut materials: ResMut<Assets<BarMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut mesh_handles: ResMut<MeshHandles>,
    color_scheme: Res<ColorScheme<T>>,
    query: Query<
        (
            Entity,
            &T,
            Option<&BarOffset<T>>,
            Option<&BarWidth<T>>,
            Option<&BarHeight<T>>,
            Option<&BarOrientation<T>>,
            Option<&BarBorder<T>>,
        ),
        Added<T>,
    >,
) {
    query.iter().for_each(
        |(entity, percentage, offset, width, height, orientation, border)| {
            let width = width.map(|it| it.get()).unwrap_or(DEFAULT_WIDTH);
            let orientation = orientation.unwrap_or(&BarOrientation::Horizontal);

            let height = height
                .map(|it| match it {
                    BarHeight::Relative(pct) => pct * width,
                    BarHeight::Static(height) => *height,
                })
                .unwrap_or(width * DEFAULT_RELATIVE_HEIGHT);

            let (width, height, vertical, offset_axis) = match orientation {
                BarOrientation::Horizontal => (width, height, false, Vec3::Y),
                BarOrientation::Vertical => (height, width, true, Vec3::X),
            };

            let mesh = mesh_handles.get(width, height).cloned().unwrap_or_else(|| {
                mesh_handles.insert(
                    width,
                    height,
                    meshes.add(Mesh::from(shape::Quad::new(Vec2::new(width, height)))),
                )
            });

            let offset = offset.map(|it| it.get()).unwrap_or(0.) * offset_axis;

            let (high, moderate, low) = match color_scheme.foreground_color {
                ForegroundColor::Static(color) => (color, color, color),
                ForegroundColor::TriSpectrum {
                    high,
                    moderate,
                    low,
                } => (high, moderate, low),
            };

            let default_border = BarBorder::new(0.);
            let border = border.unwrap_or(&default_border);

            let material = materials.add(BarMaterial {
                value_and_dimensions: (percentage.value(), width, height, border.width).into(),
                background_color: color_scheme.background_color,
                high_color: high,
                moderate_color: moderate,
                low_color: low,
                vertical,
                offset: offset.extend(0.),
                border_color: border.color,
            });

            let health_bar = commands
                .spawn((
                    Name::new(format!(
                        "{}Bar",
                        std::any::type_name::<T>()
                            .split("::")
                            .last()
                            .unwrap_or("Unknown")
                    )),
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
        },
    );
}

fn update<T: Percentage + Component>(
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

fn remove<T: Percentage + Component>(
    mut commands: Commands,
    mut removals: RemovedComponents<T>,
    parent_query: Query<&WithBar<T>>,
) {
    removals.iter().for_each(|entity| {
        let Ok(&WithBar(bar_entity, _)) = parent_query.get(entity) else {
            return;
        };

        commands.entity(bar_entity).despawn_recursive()
    });
}
