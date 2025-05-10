use ubi::prelude::*;

struct GameLayer {}
impl GameLayer {
    fn new() -> Self {
        Self {}
    }
}
impl Layer for GameLayer {
    fn on_attach(&mut self) {
        println!("GameLayer attached");
    }
    fn on_detach(&mut self) {
        println!("GameLayer detached");
    }
    fn on_update(&mut self, _: &mut Vec<Event>) {
        todo!()
    }
    fn on_event(&mut self, event: &mut Event) {
        println!("GameLayer received: {}", event);
        if let Event::KeyPressed(_) = event {
            event.set_handled(true); // Example of handling an event
        }
    }
}

fn main() {
    //let mut app = Application::with_sdl2(WindowData::default());
    let windsdl = SdlWindow::create(WindowData::default()).unwrap();
    let window_rc = windsdl.window.clone();
    let mut app = Application::new(windsdl);

    let game_layer = Box::new(GameLayer::new());
    let gui_layer = Box::new(EguiLayer::new(window_rc));

    app.push_layer(game_layer);
    app.push_overlay(gui_layer);
    app.run().expect("error running");
}
