use std::time::Instant;

use client::Client;
use vek::Vec2;
use winit::{
    event::{DeviceEvent, KeyEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{Window as WinitWindow, WindowBuilder},
};

use crate::{config::Config, key_state::KeyState, render::Renderer, scene::Scene};

pub struct Window {
    scene: Scene,
    renderer: Renderer,
    client: Client,
    event_loop: Option<EventLoop<()>>,
    window: WinitWindow,
    cursor_grabbed: bool,
}

impl Window {
    #[allow(clippy::new_without_default)]
    pub fn new(cfg: Config) -> Self {
        let event_loop = EventLoop::new().unwrap();
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        event_loop.set_control_flow(ControlFlow::Poll);

        let window = WindowBuilder::new()
            .with_title("explora")
            .build(&event_loop)
            .unwrap();
        let renderer = pollster::block_on(Renderer::new(&window));
        let size = window.inner_size();
        let scene = Scene::new(size.width as f32 / size.height as f32);
        let client = Client::new(cfg.server_addr.unwrap());
        Self {
            window,
            event_loop: Some(event_loop),
            renderer,
            cursor_grabbed: false,
            scene,
            client,
        }
    }

    pub fn run(&mut self) {
        tracing::info!("Running explora");

        let mut last_frame = Instant::now();
        let mut key_state = KeyState::default();
        const SENSITIVITY: f32 = 100.0;

        let _ = self
            .event_loop
            .take()
            .unwrap()
            .run(|event, elwt| match event {
                winit::event::Event::WindowEvent { window_id, event }
                    if window_id == self.window.id() =>
                {
                    match event {
                        winit::event::WindowEvent::Resized(size) => {
                            self.renderer.resize(size.width, size.height);
                            self.scene.resize(size.width as f32, size.height as f32);
                        }
                        winit::event::WindowEvent::CloseRequested => {
                            tracing::info!("Application close requested.");
                            elwt.exit();
                        }
                        winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                            let size = self.window.inner_size();
                            self.renderer.resize(size.width, size.height);
                            self.scene.resize(size.width as f32, size.height as f32);
                        }

                        winit::event::WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    state,
                                    physical_key: PhysicalKey::Code(code),
                                    ..
                                },
                            ..
                        } => {
                            key_state.update(code, state.is_pressed());
                        }
                        _ => (),
                    }
                }
                winit::event::Event::DeviceEvent {
                    event: DeviceEvent::MouseMotion { delta: (dx, dy) },
                    ..
                } => {
                    // map sensitivity to a range of 1 - 200. 100 being default.
                    let delta = Vec2::new(
                        dx as f32 * (SENSITIVITY / 100.0),
                        dy as f32 * (SENSITIVITY / 100.0),
                    );
                    self.scene.look(delta.x, delta.y);
                }

                winit::event::Event::AboutToWait => {
                    let dt = last_frame.elapsed();
                    self.scene.set_movement_dir(key_state.dir());
                    self.scene.tick(dt.as_secs_f32());
                    last_frame = Instant::now();
                    self.client.tick();
                    self.renderer.render(&mut self.scene);
                }
                _ => (),
            });
    }

    pub fn grab_cursor(&mut self, value: bool) {
        self.window.set_cursor_visible(!value);
        let mode = if value {
            winit::window::CursorGrabMode::Locked
        } else {
            winit::window::CursorGrabMode::None
        };
        match self.window.set_cursor_grab(mode) {
            Ok(_) => self.cursor_grabbed = value,
            Err(e) => tracing::warn!("Could not grab cursor in {:?} mode ({})", mode, e),
        }
    }
}
