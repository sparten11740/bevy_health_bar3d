use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::mesh::*;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};

use crate::constants::BAR_SHADER_HANDLE;

#[derive(Asset, AsBindGroup, Debug, Clone, Reflect)]
#[bind_group_data(BarMaterialKey)]
pub(crate) struct BarMaterial {
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
pub(crate) struct BarMaterialKey {
    vertical: bool,
    border: bool,
}

impl From<&BarMaterial> for BarMaterialKey {
    fn from(material: &BarMaterial) -> Self {
        Self {
            vertical: material.vertical,
            border: material.value_and_dimensions.w > 0.,
        }
    }
}

impl Material for BarMaterial {
    fn vertex_shader() -> ShaderRef {
        BAR_SHADER_HANDLE.into()
    }

    fn fragment_shader() -> ShaderRef {
        BAR_SHADER_HANDLE.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        key: MaterialPipelineKey<Self>,
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
