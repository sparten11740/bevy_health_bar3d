use bevy::pbr::{AlphaMode, Material, MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::{Color, Mesh, Vec4};
use bevy::reflect::{TypeUuid, TypePath};
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "94B33B1F-CDA6-468C-9F72-176557EFD304"]
#[bind_group_data(BarMaterialKey)]
pub(crate) struct BarMaterial {
    #[uniform(0)]
    pub settings: Vec4,
    #[uniform(1)]
    pub background_color: Color,
    #[uniform(2)]
    pub high_color: Color,
    #[uniform(3)]
    pub moderate_color: Color,
    #[uniform(4)]
    pub low_color: Color,
    #[uniform(5)]
    pub offset: Vec4,
    #[uniform(6)]
    pub border_color: Color,
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
            border: material.settings.y > 0.,
        }
    }
}

impl Material for BarMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/bar.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/bar.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Blend
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
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

impl TypePath for BarMaterial {
    fn type_path() -> &'static str {
        "bevy_health_bar3d::BarMaterial"
    }

    fn short_type_path() -> &'static str {
        "bevy_health_bar3d::BarMaterial"
    }
}
