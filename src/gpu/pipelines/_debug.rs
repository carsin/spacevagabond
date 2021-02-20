use bytemuck::{Pod, Zeroable};
use slotmap::SlotMap;
use std::{collections::HashMap, convert::TryInto};
use std::{
    collections::HashSet,
    mem::size_of,
    sync::{Arc, Mutex},
};
use wgpu::util::DeviceExt;

pub enum Error {}

#[derive(Clone, Copy)]
pub struct Vertex {
    position: na::Vector2<f32>,
}
unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}

#[derive(Clone, Copy)]
pub struct Instance {
    color: [f32; 4],
    transform: na::Matrix4<f32>,
    camera: na::Matrix4<f32>,
}
unsafe impl Zeroable for Instance {}
unsafe impl Pod for Instance {}

#[derive(Clone)]
struct MeshData {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shape {
    Rectangle,
}

pub type InstanceId = slotmap::DefaultKey;

pub struct InstanceRef {
    ty: Shape,
    id: InstanceId,
}

struct Mesh {
    device_ref: Arc<Mutex<wgpu::Device>>,
    vertex_buf: wgpu::Buffer,
}

macro_rules! mesh_data {
    {
        vertices: [
            $( {[$vertex_x:expr, $vertex_y:expr]} ),*
        ],
        indices: [
            $( $index:expr ),*
        ]
    } => {
        MeshData {
            vertices: vec![
                $( Vertex { position: na::Vector2::new($vertex_x as f32, $vertex_y as f32) } ),*
            ],
            indices: vec![
                $( $index ),*
            ]
        }
    }
}

pub struct DebugPipeline {
    device_ref: Arc<Mutex<wgpu::Device>>,
    layout: wgpu::PipelineLayout,
    pipeline: wgpu::RenderPipeline,
    meshes: HashMap<Shape, Mesh>,
}

impl DebugPipeline {
    pub fn new(device_ref: Arc<Mutex<wgpu::Device>>) -> Self {
        let device = device_ref.lock().unwrap();

        let vert_shader =
            device.create_shader_module(&wgpu::include_spirv!("shaders/debug.vert.spv"));
        let frag_shader =
            device.create_shader_module(&wgpu::include_spirv!("shaders/debug.frag.spv"));

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                entry_point: "main",
                module: &vert_shader,
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<Vertex>().try_into().unwrap(),
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            0 => Float2
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<Instance>().try_into().unwrap(),
                        step_mode: wgpu::InputStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![
                            1 => Float4,
                            4 => Float4,
                            5 => Float4,
                            6 => Float4,
                            7 => Float4,
                            8 => Float4,
                            9 => Float4,
                            10 => Float4,
                            11 => Float4
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                entry_point: "main",
                module: &frag_shader,
                targets: &[wgpu::ColorTargetState {
                    format: wgpu::TextureFormat::Rgba8UnormSrgb,
                    alpha_blend: wgpu::BlendState::REPLACE,
                    color_blend: wgpu::BlendState::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                cull_mode: wgpu::CullMode::None, // TODO: correct cull mode
                front_face: wgpu::FrontFace::Cw,
                polygon_mode: wgpu::PolygonMode::Fill, // TODO: line
                strip_index_format: None,
                topology: wgpu::PrimitiveTopology::TriangleList,
            },
            depth_stencil: None,
            multisample: Default::default(),
        });

        // Register meshes
        let mesh_data = vec![(
            Shape::Rectangle,
            mesh_data! {
                vertices: [
                    {[0, 0]},
                    {[1, 0]},
                    {[0, 1]},
                    {[1, 1]}
                ],
                indices: [
                    0, 1, 2,
                    2, 1, 3
                ]
            },
        )]
        .into_iter()
        .collect::<HashMap<_, _>>();

        let meshes = mesh_data
            .into_iter()
            .map(|(ty, data)| (ty, Mesh {
                
            }))
            .collect();

        Self {
            device_ref,
            layout,
            pipeline,
            meshes,
        }
    }

    pub fn add_shape(&mut self, shape: Shape, instance)
    
    fn create_vertex_buf(&self, data: &MeshData) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: &[
                bytemuck::cast_slice(&data.vertices),
                bytemuck::cast_slice(&data.indices),
            ]
            .concat(),
            usage: wgpu::BufferUsage::VERTEX,
        })
    }

    fn create_instance_buf(&self, instances: &[Instance]) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(instances),
            usage: wgpu::BufferUsage::VERTEX,
        })
    }
}
