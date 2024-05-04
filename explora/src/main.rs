use explora::{config::Config, singleplayer::Singleplayer, window::Window};

fn main() {
    tracing_subscriber::fmt::init();

    let mut config = Config::load();

    if config.singleplayer {
        let _ = Singleplayer::new(&mut config);
    }

    let mut window = Window::new(config);
    window.grab_cursor(true);
    window.run();
}
