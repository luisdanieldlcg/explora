use client::{config::Config, singleplayer::Singleplayer, window::Window};

fn main() {
    tracing_subscriber::fmt::init();

    let mut config = Config::default();

    if config.server_addr.is_none() {
        let _ =  Singleplayer::new(&mut config);
    }

    client::net::setup(config).expect("Failed to setup networking");
    let mut window = Window::new();
    window.grab_cursor(true);
    window.run();
}
