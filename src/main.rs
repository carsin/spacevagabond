use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod game;

fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut game = game::Game::new();
    game.running = true;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,

            // If there are no remaining window events to handle, update the game
            Event::MainEventsCleared => {
                game.update();
                window.request_redraw(); // Queue a RedrawRequested event & render the game
            },

            // Render the game
            Event::RedrawRequested(_) => {
                game.render(&window);
            },
            _ => ()
        }
    });
}
