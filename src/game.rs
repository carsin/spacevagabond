use winit::window::Window;
use std::time::Duration;

pub struct Game {
    frame_count: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
        }
    }

    pub fn update(&mut self) {
        self.frame_count += 1;
    }

    pub fn render(&self, window: &Window, delta_time: Duration) {
        window.set_title(&format!("Frame: {} Delta (ms): {}", self.frame_count, delta_time.as_millis()));
    }

}
