use crate::gpu::{main_pipeline::{Instance, MainPipeline, Mesh, MeshData, Vertex, View}, GpuInfo};

use super::game;

// #[derive(Default)]
pub struct Player{
    input: game::Input,
    pub angle: f32,
    pub position: na::Vector2<f32>,
    pub velocity: na::Vector2<f32>,
    // pub mesh: ,
}

impl Player{
    pub fn new() -> Self {
        Self {
            input: game::Input::default(),
            angle: 0.0,
            position: na::Vector2::default(),
            velocity: na::Vector2::default(),
            mesh:
            // &MeshData {
            //     // Test mesh: square with different colored vertices
            //     vertices: &[
            //         Vertex::new([-0.5, -0.5], [1.0, 1.0, 1.0, 1.0]),
            //         Vertex::new([0.5, -0.5], [1.0, 0.0, 1.0, 1.0]),
            //         Vertex::new([-0.5, 0.5], [1.0, 1.0, 1.0, 1.0]),
            //         Vertex::new([0.5, 0.5], [1.0, 0.0, 0.0, 1.0]),
            //     ],
            //     indices: &[0, 1, 2, 2, 1, 3],
            }
        }

    }

    pub fn update(&mut self) {
        let direction = na::Vector2::new(
            self.input.move_r as i32 as f32 - self.input.move_l as i32 as f32,
            self.input.move_f as i32 as f32 - self.input.move_b as i32 as f32,
        );

        self.velocity = self.velocity.lerp(&(direction * 5.0), 2.0); // ??
        self.position += self.velocity;
    }
}
