struct VertexIn {
    @location(0) vertex_pos: vec3<f32>,
    // @location(1) texture_id: u32,
    @builtin(vertex_index) v_index: u32
}

struct VertexOut {
    @builtin(position) vertex_pos: vec4<f32>,
    // @location(0) tex_coords: vec2<f32>
}



@vertex
fn vs_main(in: VertexIn) -> VertexOut{
    var out: VertexOut;
    out.vertex_pos = vec4<f32>(in.vertex_pos, 1.0);
    // out.tex_coords = calculate_texture_coordinates(in.v_index, in.texture_id);
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    // return textureSample(texture, texture_sampler, in.tex_coords);
    return vec4<f32>(0.6, 0.4, 0.3, 1.0);
}