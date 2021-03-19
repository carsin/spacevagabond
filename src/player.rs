// Controls state for the player
pub struct PlayerControls {
    pub move_l: bool,
    pub move_r: bool,
    pub move_f: bool,
    pub move_b: bool,
}

pub struct Player {
    pub angle: f32,
    pub position: na::Vector2<f32>,
    pub velocity: na::Vector2<f32>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            angle: 0.0,
            position: na::Vector2::default(),
            velocity: na::Vector2::default(),
        }
    }

    // Every frame, takes a controls struct 
    pub fn update(&mut self, delta: f32, controls: &PlayerControls) {
        // This is just test movement code that I wrote to see if the player was rendering correctly
        // Don't worry about how it works, you can just destroy it and replace it
        let direction = na::Vector2::new(
            controls.move_r as i32 as f32 - controls.move_l as i32 as f32,
            controls.move_f as i32 as f32 - controls.move_b as i32 as f32,
        );
        // This part just tells the player's velocity to slowly interpolate to match the target velocity based on the combined control directions
        // It's not necessary to keep this, I was just testing things out, ideally we should switch to a more realistic model
        self.velocity = self.velocity.lerp(&(direction * 5.0), delta * 2.0); // --??
        self.position += self.velocity * delta;
    }
}
