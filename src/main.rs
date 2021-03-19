extern crate nalgebra as na;

mod game;
mod gfx;
mod gpu;
mod player;

use game::Game;
use gfx::GameRenderer;
use gpu::GpuInfo;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("INFO"));

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1366, 768))
        .build(&event_loop)
        .unwrap();

    // Retrieve gpu information for rendering
    let gpu_info = Arc::new(Mutex::new(
        GpuInfo::new(
            &window,
            &na::Vector2::new(window.inner_size().width, window.inner_size().height),
        )
        .await,
    ));

    // Game
    let mut game = Game::new();
    let mut game_renderer = GameRenderer::new(gpu_info.clone());

    // Timing
    let mut last_tick = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            // Exit window when close button is pressed
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,

            // Handle keyboard input
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                window_id,
            } if window_id == window.id() => {
                let pressed = input.state == ElementState::Pressed;

                if let Some(keycode) = input.virtual_keycode {
                    use VirtualKeyCode::*;
                    // TODO: scancode instead of virtual keycode
                    match keycode {
                        // Close game
                        Escape => *control_flow = ControlFlow::Exit,
                        // Game input
                        A => game.input.move_l = pressed,
                        D => game.input.move_r = pressed,
                        S => game.input.move_b = pressed,
                        W => game.input.move_f = pressed,
                        _ => (),
                    }
                }
            }

            // If there are no remaining window events to handle, update the game
            Event::MainEventsCleared => {
                // calculate delta
                let current_tick = Instant::now();
                let delta_time = current_tick.duration_since(last_tick).as_secs_f32();
                last_tick = current_tick;

                window.set_title(&format!("Delta: {}", delta_time));

                game.update(delta_time);
                window.request_redraw(); // Queue a RedrawRequested event & render the game
            }

            // Render the game
            Event::RedrawRequested(_) => {
                game_renderer.render(&game, &window);
            }
            _ => (),
        }
    });
}
