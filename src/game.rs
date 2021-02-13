use winit::window::Window;

pub struct Game {
    pub running: bool,
    frame_count: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            running: false,
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
