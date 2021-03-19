use crate::player::{Player, PlayerControls};

#[derive(Default, Debug)]
pub struct Input {
    pub move_l: bool,
    pub move_r: bool,
    pub move_f: bool,
    pub move_b: bool,
}

impl Input {
    // Extract the player controls from the current input state
    // At the moment, there is clear duplication between here and the player mod, but that is not a mistake
    // It is coincidental that the two structs can be translated directly between each other right now, but that may change in the future
    // By keeping them as separate structs, we are future proofing in case we want to change one or the other
    pub fn player_controls(&self) -> PlayerControls {
        PlayerControls {
            move_l: self.move_l,
            move_r: self.move_r,
            move_f: self.move_f,
            move_b: self.move_b,
        }
    }
}

pub struct Game {
    player: Player,
    pub input: Input, // Any possible player game input, which is translated and relayed to wherever it's needed
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(),
            input: Input::default(),
        }
    }

    // The root game update function, everything in the game that requires regular updates is called from here at some level
    // e.g. player update, entity update, world update, processing interactions between any of those, etc.
    pub fn update(&mut self, delta: f32) {
        self.player.update(delta, &self.input.player_controls());
    }

    pub fn player(&self) -> &Player {
        &self.player
    }
}
