use winit::window::Window;

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

    pub fn render(&self, window: &Window) {
        window.set_title(&format!("Frame {}", self.frame_count));
    }

}
