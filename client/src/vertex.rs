use bytemuck::{Pod, Zeroable};
use vek::Vec3;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct TerrainVertex {
    pos: [f32; 3],
}

impl TerrainVertex {
    pub fn new(v: Vec3<f32>) -> Self {
        Self {
            pos: v.into_array(),
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Float32x3];
        wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRS,
            array_stride: std::mem::size_of::<TerrainVertex>() as wgpu::BufferAddress,
        }
    }
}
