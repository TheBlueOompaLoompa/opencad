#import bevy_pbr::mesh_view_bindings

struct FragmentInput {
  @builtin(front_facing) is_front: bool,
  @builtin(position) frag_coord: vec4<f32>,
  #import bevy_pbr::mesh_vertex_output
};

struct CadShader {
  camera_look_vec: vec4<f32>
}

const EDGE_THICKNESS: f32 = 0.005;
const EDGE_CHANGE: f32 = 0.7;


@group(1) @binding(0)
var<uniform> cad: CadShader;

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
  var closeness = dot(in.world_normal, cad.camera_look_vec.xyz);

  var out = vec4(closeness, closeness, closeness, 1.0) / 1.5;

  if(in.uv.x <= EDGE_THICKNESS || in.uv.x >= 1.0 - EDGE_THICKNESS ||in.uv.y <= EDGE_THICKNESS || in.uv.y >= 1.0 - EDGE_THICKNESS) {
    if(out.x > 0.2){
      out.x += -EDGE_CHANGE;
    } else {
      out.x += EDGE_CHANGE;
    }
    if(out.y > 0.2){
      out.y += -EDGE_CHANGE;
    } else {
      out.y += EDGE_CHANGE;
    }
    if(out.z > 0.2){
      out.z += -EDGE_CHANGE;
    } else {
      out.z += EDGE_CHANGE;
    }
  }

  return out;
}