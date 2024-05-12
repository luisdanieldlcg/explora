use explora::{config::Config, network::NetworkThread, singleplayer::Singleplayer, window::Window};

fn main() {
    tracing_subscriber::fmt::init();

    let mut config = Config::load();

    if config.singleplayer {
        let _ = Singleplayer::new(&mut config);
    }

    let network = NetworkThread::spawn(config);
    let mut window = Window::new(network);
    window.grab_cursor(true);
    window.run();
}
