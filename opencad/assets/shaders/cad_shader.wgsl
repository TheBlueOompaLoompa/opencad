#import bevy_pbr::mesh_view_bindings

struct FragmentInput {
  @builtin(front_facing) is_front: bool,
  @builtin(position) frag_coord: vec4<f32>,
  #import bevy_pbr::mesh_vertex_output
};

struct CadShader {
  camera_look_vec: vec4<f32>,
  scale: f32
}

const EDGE_THICKNESS: f32 = 0.005;
const EDGE_CHANGE: f32 = 0.7;


@group(0) @binding(0)
var<uniform> cad: CadShader;

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
  var closeness = max(1.0 - abs(dot(normalize(in.world_normal), normalize(cad.camera_look_vec.xyz))), .2);

  var out = vec4(closeness, closeness, closeness, 1.0);

  if(in.uv.x <= EDGE_THICKNESS || in.uv.x >= 1.0 - EDGE_THICKNESS ||in.uv.y <= EDGE_THICKNESS || in.uv.y >= 1.0 - EDGE_THICKNESS) {
    out = vec4(0.0, 0.0, 0.0, 1.0);
  }

  return out;
}