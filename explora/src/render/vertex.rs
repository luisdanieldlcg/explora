use bytemuck::{Pod, Zeroable};
use vek::Vec3;

#[derive(Clone, Copy, Pod, Zeroable)]
#[repr(C)]
pub struct TerrainVertex {
    data: u32,
}

impl TerrainVertex {
    pub fn new(pos: Vec3<f32>, texture_id: u32) -> Self {
        Self {
            data: ((pos.x as u32)
                | ((pos.y as u32) << 5)
                | ((pos.z as u32) << 14)
                | (texture_id << 19)),
        }
    }

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        const ATTRS: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![0 => Uint32];
        wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &ATTRS,
            array_stride: std::mem::size_of::<TerrainVertex>() as wgpu::BufferAddress,
        }
    }
}

#[cfg(test)]
mod tests {

    use vek::Vec3;

    use super::TerrainVertex;

    #[test]
    fn test_vertex_data_compression() {
        let vertex = TerrainVertex::new(Vec3::new(16.0, 256.0, 16.0), 999);

        let expected_x = vertex.data & 0x1f;
        let expected_y = (vertex.data >> 5) & 0x1ff;
        let expected_z = (vertex.data >> 14) & 0x1f;
        let expected_texture_id = (vertex.data >> 19) & 0x1fff;

        assert_eq!(expected_x, 16);
        assert_eq!(expected_y, 256);
        assert_eq!(expected_z, 16);
        assert_eq!(expected_texture_id, 999);
    }
}
