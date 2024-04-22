pub mod buffer;
pub mod mesh;
pub mod voxels;

use vek::{Mat4, Vec3};
use winit::window::Window;

use crate::{scene::Scene, vertex::TerrainVertex};

use self::{buffer::Buffer, voxels::Voxels};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Uniforms {
    proj: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
}
impl Default for Uniforms {
    fn default() -> Self {
        Self {
            proj: Mat4::identity().into_col_arrays(),
            view: Mat4::identity().into_col_arrays(),
        }
    }
}

impl Uniforms {
    pub fn new(proj: Mat4<f32>, view: Mat4<f32>) -> Self {
        Self {
            proj: proj.into_col_arrays(),
            view: view.into_col_arrays(),
        }
    }
}

/// Manages the rendering of the application.
pub struct Renderer {
    /// Surface on which the renderer will draw.
    surface: wgpu::Surface<'static>,
    /// The Logical Device, used for interacting with the GPU.
    device: wgpu::Device,
    /// A Queue handle. Used for command submission.
    queue: wgpu::Queue,
    /// The surface configuration details.
    config: wgpu::SurfaceConfiguration,
    uniforms_buffer: Buffer<Uniforms>,
    common_bg: wgpu::BindGroup,
    voxels: Voxels,
}

impl Renderer {
    pub async fn new(window: &Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = unsafe {
            instance
                .create_surface_unsafe(
                    wgpu::SurfaceTargetUnsafe::from_window(window).unwrap_unchecked(),
                )
                .unwrap()
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web, we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();
        let size = window.inner_size();

        let config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &config);

        let uniforms_buffer = Buffer::new(
            &device,
            wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            &[Uniforms::default()],
        );
        let common_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Common Bind Group Layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    // wgpu::BindGroupLayoutEntry {
                    //     binding: 1,
                    //     visibility: wgpu::ShaderStages::FRAGMENT,
                    //     ty: wgpu::BindingType::Texture {
                    //         multisampled: false,
                    //         view_dimension: wgpu::TextureViewDimension::D2,
                    //         sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    //     },
                    //     count: None,
                    // },
                    // wgpu::BindGroupLayoutEntry {
                    //     binding: 2,
                    //     visibility: wgpu::ShaderStages::FRAGMENT,
                    //     ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    //     count: None,
                    // },
                ],
            });
        let common_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Common Bind Group"),
            layout: &common_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniforms_buffer.as_entire_binding(),
                },
                // wgpu::BindGroupEntry {
                //     binding: 1,
                //     resource: wgpu::BindingResource::TextureView(&atlas_texture.view),
                // },
                // wgpu::BindGroupEntry {
                //     binding: 2,
                //     resource: wgpu::BindingResource::Sampler(&atlas_texture.sampler),
                // },
            ],
        });

        let voxels = Voxels::new(&device, &common_bind_group_layout, &config);
        Self {
            surface,
            device,
            queue,
            config,
            uniforms_buffer,
            common_bg,
            voxels,
        }
    }

    pub fn resize(&mut self, w: u32, h: u32) {
        self.config.width = w;
        self.config.height = h;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self, scene: &mut Scene) {
        let matrices = scene.camera_matrices();

        self.uniforms_buffer
            .write(&self.queue, &[Uniforms::new(matrices.proj, matrices.view)]);

        let output = self.surface.get_current_texture().unwrap();
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            self.voxels.draw(render_pass, &self.common_bg);
        }
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
