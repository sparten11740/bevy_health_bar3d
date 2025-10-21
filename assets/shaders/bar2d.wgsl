#import bevy_sprite::{
    mesh2d_view_bindings::view,
    mesh2d_functions::get_world_from_local
}

@group(2) @binding(0)
var<uniform> value_and_dimensions: vec4<f32>;
@group(2) @binding(1)
var<uniform> background_color: vec4<f32>;
@group(2) @binding(2)
var<uniform> high_color: vec4<f32>;
@group(2) @binding(3)
var<uniform> moderate_color: vec4<f32>;
@group(2) @binding(4)
var<uniform> low_color: vec4<f32>;
@group(2) @binding(5)
var<uniform> offset: vec4<f32>;
#ifdef HAS_BORDER
@group(2) @binding(6)
var<uniform> border_color: vec4<f32>;
#endif

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let world_from_local = get_world_from_local(vertex.instance_index);
    let world_position = world_from_local * vec4<f32>(vertex.position.x + offset.x, vertex.position.y + offset.y, vertex.position.z, 1.);
    out.clip_position = view.clip_from_world * world_position;
    out.uv = vertex.uv;

    return out;
}

struct FragmentInput {
     @location(0) uv: vec2<f32>
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let value = value_and_dimensions.x;
    #ifdef HAS_BORDER
      let resolution = value_and_dimensions.yz;
      let border_width = value_and_dimensions.w;
      let border_y = border_width / resolution.y;
      let border_x = border_width / resolution.x;

      if in.uv.y < border_y || in.uv.y > 1. - border_y || in.uv.x < border_x || in.uv.x > 1. - border_x {
          return border_color;
      }
    #endif

    #ifdef IS_VERTICAL
      let val = 1.0 - value;
      if in.uv.y < val {
          return background_color;
      }

      if val > 0.6 {
          return low_color;
      }

      if val > 0.2 {
          return moderate_color;
      }

      return high_color;
    #else
      if in.uv.x > value {
          return background_color;
      }

      if value < 0.4 {
          return low_color;
      }

      if value < 0.8 {
          return moderate_color;
      }

      return high_color;
    #endif
}

