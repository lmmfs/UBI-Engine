use ubi::prelude::*;

fn main() {
    let window_data = WindowData {
        name: "MyApp",
        width: 800,
        height: 600,
    };
    let mut app = Application::with_sdl2(window_data);
    app.run();
}