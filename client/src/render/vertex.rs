use bytemuck::{Pod, Zeroable};
use vek::Vec3;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct TerrainVertex {
    pos: [f32; 3],
    texture_id: u32,
}

impl TerrainVertex {
    pub fn new(v: Vec3<f32>, texture_id: u32) -> Self {
        Self {
            pos: v.into_array(),
            texture_id,
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRS: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Uint32];
        wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRS,
            array_stride: std::mem::size_of::<TerrainVertex>() as wgpu::BufferAddress,
        }
    }
}
