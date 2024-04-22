use std::collections::HashMap;

use common::chunk::Chunk;
use vek::Vec2;

use crate::block::BlockMap;

use super::{
    atlas::BlockAtlas, buffer::Buffer, mesh::create_chunk_mesh, texture::Texture,
    vertex::TerrainVertex,
};

pub struct TerrainGeometry {
    vertex_buffer: Buffer<TerrainVertex>,
    chunk_pos_buffer: Buffer<[i32; 2]>,
    bind_group: wgpu::BindGroup,
}

impl TerrainGeometry {
    pub fn new(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        vertices: &[TerrainVertex],
        pos: Vec2<i32>,
    ) -> Self {
        let vertex_buffer = Buffer::new(
            device,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            &vertices,
        );
        let offset_buffer = Buffer::new(device, wgpu::BufferUsages::UNIFORM, &[pos.into_array()]);
        let chunk_pos_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Chunk Pos Bind Group"),
            layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: offset_buffer.as_entire_binding(),
            }],
        });

        Self {
            vertex_buffer,
            chunk_pos_buffer: offset_buffer,
            bind_group: chunk_pos_bind_group,
        }
    }
}

pub struct Voxels {
    index_buffer: Buffer<u32>,
    terrain_pipeline: wgpu::RenderPipeline,
    geometry: HashMap<Vec2<i32>, TerrainGeometry>,
}

impl Voxels {
    pub fn new(
        device: &wgpu::Device,
        common_bg_layout: &wgpu::BindGroupLayout,
        config: &wgpu::SurfaceConfiguration,
        block_atlas: &BlockAtlas,
        block_map: &BlockMap,
    ) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(
                include_str!("../../../assets/shaders/voxels.wgsl").into(),
            ),
        });

        let chunk_pos_bg_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Chunk Pos Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&common_bg_layout, &chunk_pos_bg_layout],
            push_constant_ranges: &[],
        });

        let terrain_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[TerrainVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::all(),
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });
        let mut geometry = HashMap::new();
        let mut vertex_count = 0;
        for z in 0..3 {
            for x in 0..3 {
                let chunk = Chunk::flat();
                let offset = Vec2::new(z, x);
                let mesh = create_chunk_mesh(&chunk, block_atlas, block_map);

                let terrain = TerrainGeometry::new(device, &chunk_pos_bg_layout, &mesh, offset);

                vertex_count += terrain.vertex_buffer.len();

                geometry.insert(offset, terrain);
            }
        }

        let indices = compute_voxel_indices(vertex_count as usize);
        let index_buffer = Buffer::new(device, wgpu::BufferUsages::INDEX, &indices);

        Self {
            terrain_pipeline,
            geometry,
            index_buffer,
        }
    }

    pub fn draw<'pass>(
        &'pass mut self,
        mut frame: wgpu::RenderPass<'pass>,
        common_bg: &'pass wgpu::BindGroup,
    ) {
        frame.set_pipeline(&self.terrain_pipeline);
        frame.set_bind_group(0, common_bg, &[]);
        frame.set_index_buffer(self.index_buffer.slice(), wgpu::IndexFormat::Uint32);

        for (pos, geometry) in &self.geometry {
            frame.set_bind_group(1, &geometry.bind_group, &[]);
            frame.set_vertex_buffer(0, geometry.vertex_buffer.slice());
            frame.draw_indexed(0..geometry.vertex_buffer.len() / 4 * 6, 0, 0..1);
        }
    }
}

fn compute_voxel_indices(number_of_vertices: usize) -> Vec<u32> {
    let mut indices = Vec::with_capacity(number_of_vertices * 6 / 4);
    for i in 0..number_of_vertices / 4 {
        let offset = i as u32 * 4;
        indices.extend_from_slice(&[
            offset,
            offset + 1,
            offset + 2,
            offset + 2,
            offset + 3,
            offset,
        ]);
    }
    indices
}
