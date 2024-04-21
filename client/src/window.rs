use winit::{
    event_loop::EventLoop,
    window::{Window as WinitWindow, WindowBuilder},
};

use crate::renderer::Renderer;

pub struct Window {
    window: WinitWindow,
    event_loop: Option<EventLoop<()>>,
    renderer: Renderer,
}

impl Window {
    
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();

        let window = WindowBuilder::new()
            .with_title("explora")
            .build(&event_loop)
            .unwrap();
        let renderer = pollster::block_on(Renderer::new(&window));
        Self {
            window,
            event_loop: Some(event_loop),
            renderer,
        }
    }

    pub fn run(&mut self) {
        tracing::info!("Running explora");
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
                        }
                        winit::event::WindowEvent::CloseRequested => {
                            tracing::info!("Application close requested.");
                            elwt.exit();
                        }
                        winit::event::WindowEvent::ScaleFactorChanged { .. } => {
                            let size = self.window.inner_size();
                            self.renderer.resize(size.width, size.height);
                        }
                        _ => (),
                    }
                }
                winit::event::Event::AboutToWait => {
                    self.renderer.render();
                }
                _ => (),
            });
    }
}
