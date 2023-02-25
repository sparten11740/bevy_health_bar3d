#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

let BACKGROUND_COLOR = vec4<f32>(0., 0., 0., 0.75);
let GOOD_HEALTH = vec4<f32>(0., 1., 0., 0.95);
let OK_HEALTH = vec4<f32>(1., 1., 0., 0.95);
let BAD_HEALTH = vec4<f32>(1., 0., 0., 0.95);

@group(1) @binding(0)
var<uniform> value: f32;

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
        return BACKGROUND_COLOR;
    }

    if value < 0.4 {
        return BAD_HEALTH;
    }

    if value < 0.8 {
        return OK_HEALTH;
    }

    return GOOD_HEALTH;
}
