use crate::{
    game::Game,
    gpu::{
        main_pipeline::{Instance, MainPipeline, Mesh, Vertex, View},
        GpuInfo,
    },
};
use std::sync::{Arc, Mutex};
use winit::window::Window;

pub struct GameRenderer {
    gpu_info: Arc<Mutex<GpuInfo>>,
    main_pipeline: MainPipeline,

    // Temporary mesh for testing rendering
    // In the future, all meshes should be located at some level within this module
    // Meshes should not be created outside of here, for organizational purposes
    test_mesh: Mesh,
}

impl GameRenderer {
    pub fn new(gpu_info: Arc<Mutex<GpuInfo>>) -> Self {
        // A pipeline is a set of steps to render a particular type of mesh
        // The main pipeline contains instructions to render a basic mesh with no special information, just vertices and indices
        // We initialize the pipeline with an identity ("default") view matrix for now which will be overridden later in the update function
        let mut main_pipeline =
            MainPipeline::new(gpu_info.clone(), View::new(na::Matrix3::identity())); // --???

        Self {
            gpu_info,
            test_mesh: main_pipeline.create_mesh(
                &[
                    Vertex::new(na::Vector2::new(0.0, 0.0), [1.0, 1.0, 1.0, 1.0]),
                    Vertex::new(na::Vector2::new(1.0, 0.0), [1.0, 1.0, 1.0, 1.0]),
                    Vertex::new(na::Vector2::new(0.0, 1.0), [1.0, 1.0, 1.0, 1.0]),
                    Vertex::new(na::Vector2::new(1.0, 1.0), [1.0, 1.0, 1.0, 1.0]),
                ],
                &[0, 1, 2, 2, 1, 3],
            ),
            main_pipeline,
        }
    }

    // Render simply takes a reference to a game and draws it
    // Any information that needs to be accessed here should be publicly exposed in Game
    pub fn render(&mut self, game: &Game, window: &Window) {
        // Update camera
        // Step 1. Aspect correction
        // the viewport coordinates are between -1 and 1 for each axis, but the window's width and height is not always the same
        // This causes the image to appear stretched (usually on the x axis, since width is often greater than height), so we create a matrix that corrects this
        let size = window.inner_size();
        let aspect = size.width as f32 / size.height as f32;
        let mut transform = na::Matrix3::new_nonuniform_scaling(&if aspect >= 1.0 {
            na::Vector2::new(1.0, aspect)
        } else {
            na::Vector2::new(1.0 / aspect, 1.0)
        });
        // Step 2. Apply zoom
        transform.append_scaling_mut(0.2);
        self.main_pipeline.view = View::new(
            // --???
            // The primary matrix accounts for the aspect ratio
            transform,
        );

        // Acquire target framebuffer to render into
        let target = &self
            .gpu_info
            .lock()
            .unwrap()
            .swapchain
            .get_current_frame()
            .unwrap()
            .output
            .view;

        // Render test mesh
        self.main_pipeline.render(
            target,
            &[(
                &self.test_mesh,
                &[Instance::new(
                    na::Matrix3::identity().append_translation(&game.player().position),
                )],
            )],
        )
    }
}
