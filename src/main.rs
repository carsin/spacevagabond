use std::time::{Duration, Instant};
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
    let mut last_tick = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // calculate delta
        let current_tick = Instant::now();
        let delta_time = current_tick.duration_since(last_tick);
        last_tick = current_tick;

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
