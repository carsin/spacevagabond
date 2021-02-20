use crate::gpu::{
    main_pipeline::{Instance, MainPipeline, Mesh, MeshData, Vertex, View},
    GpuInfo,
};
use std::sync::{Arc, Mutex};
use winit::window::Window;

#[derive(Default, Debug)]
pub struct Input {
    pub move_l: bool,
    pub move_r: bool,
    pub move_f: bool,
    pub move_b: bool,
}

// TEST PLAYER
#[derive(Default)]
struct Player {
    position: na::Vector2<f32>,
    velocity: na::Vector2<f32>,
    angle: f32,
}
// END TEST PLAYER

pub struct Game {
    pub input: Input,

    gpu_info: Arc<Mutex<GpuInfo>>,
    main_pipeline: MainPipeline,

    // Test things
    test_mesh: Mesh,
    test_player: Player,
}

impl Game {
    pub async fn new(gpu_info: Arc<Mutex<GpuInfo>>) -> Self {
        let mut main_pipeline =
            MainPipeline::new(gpu_info.clone(), View::new(na::Matrix3::identity()));

        // Test mesh: square with different colored vertices
        let test_mesh = main_pipeline.create_mesh(&MeshData {
            vertices: &[
                Vertex::new([-0.5, -0.5], [1.0, 1.0, 1.0, 1.0]),
                Vertex::new([0.5, -0.5], [1.0, 0.0, 1.0, 1.0]),
                Vertex::new([-0.5, 0.5], [1.0, 1.0, 1.0, 1.0]),
                Vertex::new([0.5, 0.5], [1.0, 0.0, 0.0, 1.0]),
            ],
            indices: &[0, 1, 2, 2, 1, 3],
        });

        Self {
            input: Input::default(),

            gpu_info,
            main_pipeline,

            test_mesh,
            test_player: Player::default(),
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        let direction = na::Vector2::new(
            self.input.move_r as i32 as f32 - self.input.move_l as i32 as f32,
            self.input.move_f as i32 as f32 - self.input.move_b as i32 as f32,
        );

        self.test_player.velocity = self.test_player.velocity.lerp(&(direction * 5.0), delta_time * 2.0);
        self.test_player.position += self.test_player.velocity * delta_time;
        self.test_player.angle =
            (self.test_player.angle + delta_time) % (std::f32::consts::PI * 2.0);
    }

    pub fn render(&mut self, window: &Window) {
        // Update camera
        let size = window.inner_size();
        let aspect = size.width as f32 / size.height as f32;
        self.main_pipeline.view = View::new(
            na::Matrix3::new_nonuniform_scaling(&if aspect >= 1.0 {
                na::Vector2::new(1.0, aspect)
            } else {
                na::Vector2::new(1.0 / aspect, 1.0)
            })
            .append_scaling(0.2),
        );

        // Get target frame to render
        let target = &self
            .gpu_info
            .lock()
            .unwrap()
            .swapchain
            .get_current_frame()
            .unwrap()
            .output
            .view;

        // Do rendering
        self.main_pipeline.render(
            target,
            &mut self.test_mesh,
            &[Instance::new(
                na::Similarity2::new(
                    self.test_player.position,
                    self.test_player.angle,
                    1.0 / (1.0 + self.test_player.velocity.magnitude()),
                )
                .to_homogeneous(),
            )],
        );
    }
}
