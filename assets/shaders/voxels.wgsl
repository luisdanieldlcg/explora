struct Uniforms {
    proj: mat4x4<f32>,
    view: mat4x4<f32>,
    atlas_size: u32,
    tile_size: u32,
}

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(1) @binding(0)
var<uniform> chunk_offset: vec2<i32>;

struct VertexIn {
    @location(0) data: u32,
    @builtin(vertex_index) v_index: u32
}

struct VertexOut {
    @builtin(position) vertex_pos: vec4<f32>,
    @location(0) tex_coords: vec2<f32>
}


fn calculate_texture_coordinates(v_index: u32, texture_id: u32) -> vec2<f32> {
    let tile_width = uniforms.tile_size;
    let tile_height = uniforms.tile_size;
    let tiles_per_row = uniforms.atlas_size / tile_width;
    let pixel_x = f32((texture_id % tiles_per_row) * tile_width);
    let pixel_y = f32((texture_id / tiles_per_row) * tile_height);
    switch (v_index % 4u) {
          case 0u: {
            // top left
            return vec2<f32>(pixel_x / f32(uniforms.atlas_size), pixel_y / f32(uniforms.atlas_size));
          }
          case 1u: {
            // bottom left
            return vec2<f32>(pixel_x / f32(uniforms.atlas_size), (pixel_y + f32(tile_height)) / f32(uniforms.atlas_size));
          }
          case 2u: {
            // bottom right
            return vec2<f32>((pixel_x + f32(tile_width)) / f32(uniforms.atlas_size), (pixel_y + f32(tile_height)) / f32(uniforms.atlas_size));
          }
          case 3u: {
            // top right
            return vec2<f32>((pixel_x + f32(tile_width)) / f32(uniforms.atlas_size), pixel_y / f32(uniforms.atlas_size));
          }
          default: {
              return vec2<f32>(0.0, 0.0);
          }
      }
}

fn calculate_vertex_coordinates(data: u32) -> vec3<f32> {
    let x = data & 0x1f;
    let y = (data >> 5u) & 0x1ff;
    let z = (data >> 14u) & 0x1f;
    return vec3<f32>(f32(x), f32(y), f32(z));
}

@vertex
fn vs_main(in: VertexIn) -> VertexOut{
    var out: VertexOut;
    var pos = calculate_vertex_coordinates(in.data);
    pos.x += f32(chunk_offset.x * 16);
    pos.z += f32(chunk_offset.y * 16);

    out.vertex_pos = uniforms.proj * uniforms.view * vec4<f32>(pos, 1.0);

    let texture_id = (in.data >> 19u) & 0x1fff;
    out.tex_coords = calculate_texture_coordinates(in.v_index, texture_id);
    return out;
}

@group(0) @binding(1)
var texture: texture_2d<f32>;
@group(0) @binding(2)
var texture_sampler: sampler;

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in.tex_coords);
}
