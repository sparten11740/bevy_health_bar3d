use bevy::prelude::*;
use bevy::render::mesh::*;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};
use bevy::sprite::{Material2d, Material2dKey};

use crate::constants::BAR_SHADER_2D_HANDLE;

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
#[bind_group_data(BarMaterial2dKey)]
pub(crate) struct BarMaterial2d {
    #[uniform(0)]
    pub value_and_dimensions: Vec4,
    // (value, width, height, border_width) vec4 to be 16byte aligned
    #[uniform(1)]
    pub background_color: LinearRgba,
    #[uniform(2)]
    pub high_color: LinearRgba,
    #[uniform(3)]
    pub moderate_color: LinearRgba,
    #[uniform(4)]
    pub low_color: LinearRgba,
    #[uniform(5)]
    pub offset: Vec4,
    #[uniform(6)]
    pub border_color: LinearRgba,
    pub vertical: bool,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub(crate) struct BarMaterial2dKey {
    vertical: bool,
    border: bool,
}

impl From<&BarMaterial2d> for BarMaterial2dKey {
    fn from(material: &BarMaterial2d) -> Self {
        Self {
            vertical: material.vertical,
            border: material.value_and_dimensions.w > 0.,
        }
    }
}

impl Material2d for BarMaterial2d {
    fn vertex_shader() -> ShaderRef {
        BAR_SHADER_2D_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        BAR_SHADER_2D_HANDLE.into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
        ])?;

        let fragment = descriptor.fragment.as_mut().unwrap();
        if key.bind_group_data.vertical {
            fragment.shader_defs.push("IS_VERTICAL".into());
        }

        if key.bind_group_data.border {
            fragment.shader_defs.push("HAS_BORDER".into());
        }

        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}
