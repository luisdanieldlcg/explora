use client::window::Window;

fn main() {
    tracing_subscriber::fmt::init();
    let mut window = Window::new();
    window.grab_cursor(true);
    window.run();
}
