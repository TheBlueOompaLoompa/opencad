#import bevy_pbr::mesh_view_bindings

struct FragmentInput {
  @builtin(front_facing) is_front: bool,
  @builtin(position) frag_coord: vec4<f32>,
  #import bevy_pbr::mesh_vertex_output
};

const EDGE_THICKNESS: f32 = 0.01;

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
  var out = vec4(0.0, 0.0, 0.0, 0.0);

  if(in.uv.x <= EDGE_THICKNESS || in.uv.x >= 1.0 - EDGE_THICKNESS ||in.uv.y <= EDGE_THICKNESS || in.uv.y >= 1.0 - EDGE_THICKNESS) {
    out = -vec4(0.3, 0.3, 0.3, 1.0);    
  }

  return out + vec4(
    (in.world_normal.x + in.world_normal.y + in.world_normal.z) / 3.0 + 0.2,
    (in.world_normal.x + in.world_normal.y + in.world_normal.z) / 3.0 + 0.2,
    (in.world_normal.x + in.world_normal.y + in.world_normal.z) / 3.0 + 0.2
  , 1.0);
}