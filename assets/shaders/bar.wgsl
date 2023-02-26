#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

@group(1) @binding(0)
var<uniform> value: f32;
@group(1) @binding(1)
var<uniform> background_color: vec4<f32>;
@group(1) @binding(2)
var<uniform> high_color: vec4<f32>;
@group(1) @binding(3)
var<uniform> moderate_color: vec4<f32>;
@group(1) @binding(4)
var<uniform> low_color: vec4<f32>;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) x: f32,
};

struct FragmentInput {
     @location(0) x: f32,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(0., 0., 0., 1.0));
    out.clip_position += vec4<f32>(vertex.position, 0.);

    out.x = vertex.uv.x;
    return out;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    if in.x > value {
        return background_color;
    }

    if value < 0.4 {
        return low_color;
    }

    if value < 0.8 {
        return moderate_color;
    }

    return high_color;
}