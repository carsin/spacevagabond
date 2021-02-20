use std::sync::{Arc, Mutex};

use super::gpu;
use gpu::{
    pipelines::default::{DefaultPipeline, View},
    render_context::RenderContext,
};
use gpu::{
    pipelines::default::{Instance, MeshData, Vertex},
    GpuInfo,
};
use winit::window::Window;

pub struct Game {
    gpu_info: Arc<Mutex<GpuInfo>>,
    default_pipeline: DefaultPipeline,
}

impl Game {
    pub async fn new(gpu_info: Arc<Mutex<GpuInfo>>) -> Self {
        let mut default_pipeline =
            DefaultPipeline::new(gpu_info.clone(), View::new(na::Matrix3::identity()));

        let mesh_id = default_pipeline
            .add_mesh(&MeshData {
                vertices: &[
                    Vertex::new(na::Vector2::new(0.0, 0.0), [1.0, 1.0, 1.0, 1.0]),
                    Vertex::new(na::Vector2::new(1.0, 0.0), [1.0, 1.0, 1.0, 1.0]),
                    Vertex::new(na::Vector2::new(0.0, 1.0), [1.0, 1.0, 1.0, 1.0]),
                    Vertex::new(na::Vector2::new(1.0, 1.0), [1.0, 1.0, 1.0, 1.0]),
                ],
                indices: &[0, 1, 2, 2, 1, 3],
            })
            .unwrap();

        let instance_a = default_pipeline
            .add_mesh_instance(
                mesh_id,
                Instance::new(na::Similarity2::new(na::Vector2::new(0.5, 0.5), 0.4, 0.2)),
            )
            .unwrap();

        let instance_b = default_pipeline
            .add_mesh_instance(
                mesh_id,
                Instance::new(na::Similarity2::new(na::Vector2::new(-0.5, -0.2), 0.3, 0.1)),
            )
            .unwrap();

        Self {
            gpu_info,
            default_pipeline,
        }
    }

    pub fn update(&mut self, delta_time: f32) {}

    pub fn render(&mut self, window: &Window) {
        let size = window.inner_size();
        let aspect = size.width as f32 / size.height as f32;

        self.default_pipeline.view.camera =
            na::Matrix3::new_nonuniform_scaling(&if aspect >= 1.0 {
                na::Vector2::new(1.0, aspect)
            } else {
                na::Vector2::new(1.0 / aspect, 1.0)
            })
            .into();
        self.default_pipeline.render();
    }
}
