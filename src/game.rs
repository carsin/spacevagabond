use super::gpu::{
    pipelines::default::{DefaultPipeline, Instance, MeshData, Vertex, View},
    GpuInfo,
};
use std::sync::{Arc, Mutex};
use winit::window::Window;

pub struct Game {
    gpu_info: Arc<Mutex<GpuInfo>>,
    default_pipeline: DefaultPipeline,
    test: f32,
}

impl Game {
    pub async fn new(gpu_info: Arc<Mutex<GpuInfo>>) -> Self {
        let mut default_pipeline =
            DefaultPipeline::new(gpu_info.clone(), View::new(na::Matrix3::identity()));

        let mesh_id = default_pipeline
            .add_mesh(&MeshData {
                vertices: &[
                    Vertex::new([0.0, 0.0], [1.0, 1.0, 1.0, 1.0]),
                    Vertex::new([1.0, 0.0], [1.0, 0.0, 1.0, 1.0]),
                    Vertex::new([0.0, 1.0], [1.0, 1.0, 1.0, 1.0]),
                    Vertex::new([1.0, 1.0], [1.0, 0.0, 0.0, 1.0]),
                ],
                indices: &[0, 1, 2, 2, 1, 3],
            })
            .unwrap();

        let _instance_a = default_pipeline
            .add_mesh_instance(
                mesh_id,
                Instance::new(na::Similarity2::new(na::Vector2::new(-0.8, -0.5), 0.4, 0.2)),
            )
            .unwrap();
        let _instance_b = default_pipeline
            .add_mesh_instance(
                mesh_id,
                Instance::new(na::Similarity2::new(na::Vector2::new(-0.5, -0.2), 1.0, 0.1)),
            )
            .unwrap();
        let _instance_c = default_pipeline
            .add_mesh_instance(
                mesh_id,
                Instance::new(na::Similarity2::new(na::Vector2::new(-0.5, -0.2), 2.0, 0.1)),
            )
            .unwrap();

        Self {
            gpu_info,
            default_pipeline,
            test: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.test += delta_time;
    }

    pub fn render(&mut self, window: &Window) {
        let size = window.inner_size();
        let aspect = size.width as f32 / size.height as f32;

        self.default_pipeline.view =
            View::new(na::Matrix3::new_nonuniform_scaling(&if aspect >= 1.0 {
                na::Vector2::new(1.0, aspect)
            } else {
                na::Vector2::new(1.0 / aspect, 1.0)
            }));

        let target = &self
            .gpu_info
            .lock()
            .unwrap()
            .swapchain
            .get_current_frame()
            .unwrap()
            .output
            .view;
        self.default_pipeline.render(target);
    }
}
