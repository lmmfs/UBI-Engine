use ubi::prelude::*;

fn main() {
    let window_data = WindowData {
        name: "MyApp",
        width: 800,
        height: 600,
    };

    let windsdl = Windsdl::create(window_data).unwrap();
    let mut app = Application::new(windsdl);

    app.run();
}