use crate::gpu::{
    main_pipeline::{Instance, MainPipeline, Mesh, MeshData, Vertex, View},
    GpuInfo,
};

use std::sync::{Arc, Mutex};
use winit::window::Window;

use super::player;
use super::gfx;

#[derive(Default, Debug)]
pub struct Input {
    pub move_l: bool,
    pub move_r: bool,
    pub move_f: bool,
    pub move_b: bool,
}

pub struct Game {
    gpu_info: Arc<Mutex<GpuInfo>>,
    main_pipeline: MainPipeline,
    player: player::Player,
}

impl Game {
    pub async fn new(gpu_info: Arc<Mutex<GpuInfo>>) -> Self {
        let mut main_pipeline = MainPipeline::new(gpu_info.clone(), View::new(na::Matrix3::identity())); // ???

        Self {
            gpu_info,
            main_pipeline,
            player: player::Player::new(),
        }
    }

    pub fn render(&mut self, window: &Window) {
        // Update camera
        let size = window.inner_size();
        let aspect = size.width as f32 / size.height as f32;
        self.main_pipeline.view = View::new( // ???
            na::Matrix3::new_nonuniform_scaling(&if aspect >= 1.0 {
                na::Vector2::new(1.0, aspect)
            } else {
                na::Vector2::new(1.0 / aspect, 1.0)
            })
            .append_scaling(0.2),
        );

        let target = &self.gpu_info.lock().unwrap().swapchain.get_current_frame().unwrap().output.view; // Get target frame to render

        // Render player
        self.main_pipeline.render(
            target,
            &mut self.main_pipeline.create_mesh(&self.player.mesh),
            // ???
            &[Instance::new(
                na::Similarity2::new(
                    self.player.position,
                    self.player.angle,
                    1.0 / (1.0 + self.player.velocity.magnitude()),
                )
                .to_homogeneous(),
            )],
        );
    }
}
