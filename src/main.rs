extern crate bytemuck;
extern crate crevice;
extern crate mint;
extern crate nalgebra as na;
extern crate raw_window_handle;
extern crate wgpu;
extern crate winit;

mod game;
mod gpu;

use game::Game;
use gpu::GpuInfo;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(1366, 768))
        .build(&event_loop)
        .unwrap();

    let gpu_info = Arc::new(Mutex::new(
        GpuInfo::new(
            &window,
            &na::Vector2::new(window.inner_size().width, window.inner_size().height),
        )
        .await,
    ));
    let mut game = Game::new(gpu_info.clone()).await;
    let mut last_tick = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,

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
                game.render(&window);
            }
            _ => (),
        }
    });
}
