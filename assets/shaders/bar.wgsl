#import bevy_pbr::mesh_view_bindings view
#import bevy_pbr::mesh_bindings mesh

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
@group(1) @binding(5)
var<uniform> offset: vec3<f32>;
#ifdef HAS_BORDER
@group(1) @binding(6)
var<uniform> resolution: vec2<f32>;
@group(1) @binding(7)
var<uniform> border_width: f32;
@group(1) @binding(8)
var<uniform> border_color: vec4<f32>;
#endif

struct Vertex {
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

    let view_proj = view.view_proj;
    let camera_right = normalize(vec3<f32>(view_proj.x.x, view_proj.y.x, view_proj.z.x));
    let camera_up = normalize(vec3<f32>(view_proj.x.y, view_proj.y.y, view_proj.z.y));

    let world_space = camera_right * (vertex.position.x + offset.x) + camera_up * (vertex.position.y + offset.y);
    let position = view.view_proj * mesh.model * vec4<f32>(world_space, 1.);

    out.uv = vertex.uv;
    out.clip_position = position;

    return out;
}

struct FragmentInput {
     @location(0) uv: vec2<f32>
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    #ifdef HAS_BORDER
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
