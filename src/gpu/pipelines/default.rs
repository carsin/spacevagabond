use crevice::std140::{self, AsStd140, Std140};
use std::{
    collections::{HashMap, HashSet},
    convert::TryInto,
    mem::size_of,
    sync::{Arc, Mutex, Weak},
};

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::gpu::GpuInfo;

#[derive(Debug)]
pub enum Error {
}
pub type Result<T> = std::result::Result<T, Error>;

// View uniform
#[derive(AsStd140, Clone, Copy)]
pub struct View {
    pub camera: mint::ColumnMatrix3<f32>,
}
unsafe impl Zeroable for View {}
unsafe impl Pod for View {}
impl View {
    pub fn new(camera: impl Into<mint::ColumnMatrix3<f32>>) -> Self {
        Self {
            camera: camera.into(),
        }
    }
}

// A single unit of vertex information
#[derive(Clone, Copy)]
pub struct Vertex {
    pub position: mint::Vector2<f32>,
    pub color: [f32; 4],
}
unsafe impl Zeroable for Vertex {}
unsafe impl Pod for Vertex {}
impl Vertex {
    pub fn new(position: impl Into<mint::Vector2<f32>>, color: [f32; 4]) -> Self {
        Self {
            position: position.into(),
            color,
        }
    }
}

// An instance of a mesh
#[derive(Clone, Copy)]
pub struct Instance {
    pub transform: mint::ColumnMatrix3<f32>,
}
unsafe impl Zeroable for Instance {}
unsafe impl Pod for Instance {}
impl Instance {
    pub fn new(transform: impl Into<mint::ColumnMatrix3<f32>>) -> Self {
        Self {
            transform: transform.into(),
        }
    }
}

// Vertices and indices of a mesh, unrelated to the gpu
pub struct MeshData<'a> {
    pub vertices: &'a [Vertex],
    pub indices: &'a [u16],
}

// A full gpu-uploaded mesh with instance information
pub struct Mesh {
    index_count: u32,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
}

pub struct DefaultPipeline {
    pub view: View,

    gpu_info: Arc<Mutex<GpuInfo>>,
    view_bind_group_layout: wgpu::BindGroupLayout,
    layout: wgpu::PipelineLayout,
    pipeline: wgpu::RenderPipeline,
    view_buffer: wgpu::Buffer,
    view_bind_group: wgpu::BindGroup,
}

impl DefaultPipeline {
    pub fn new(gpu_info: Arc<Mutex<GpuInfo>>, view: View) -> Self {
        let gpu_info_ = gpu_info.clone();
        let GpuInfo {
            adapter,
            surface,
            device,
            ..
        } = &*gpu_info_.lock().unwrap();

        let view_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Default View Bind Group Layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(
                            size_of::<<View as AsStd140>::Std140Type>()
                                .try_into()
                                .unwrap(),
                        ),
                    },
                    count: None,
                    visibility: wgpu::ShaderStage::VERTEX,
                }],
            });

        let vert_shader =
            device.create_shader_module(&wgpu::include_spirv!("shaders/default.vert.spv"));
        let frag_shader =
            device.create_shader_module(&wgpu::include_spirv!("shaders/default.frag.spv"));

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Default Pipeline Layout"),
            bind_group_layouts: &[&view_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Default Render Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                entry_point: "main",
                module: &vert_shader,
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<Vertex>().try_into().unwrap(),
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            0 => Float2,
                            1 => Float4,
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<Instance>().try_into().unwrap(),
                        step_mode: wgpu::InputStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![
                            4 => Float3,
                            5 => Float3,
                            6 => Float3,
                        ],
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                entry_point: "main",
                module: &frag_shader,
                targets: &[wgpu::ColorTargetState {
                    format: adapter.get_swap_chain_preferred_format(surface),
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

        let view_buffer = create_view_buffer(device, &view);

        let view_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Default View Bind Group"),
            layout: &view_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &view_buffer,
                    offset: 0,
                    size: None,
                },
            }],
        });

        Self {
            view,

            gpu_info,
            view_bind_group_layout,
            layout,
            pipeline,
            view_buffer,
            view_bind_group,
        }
    }

    pub fn create_mesh(&mut self, data: &MeshData) -> Result<Mesh> {
        let GpuInfo { device, .. } = &*self.gpu_info.lock().unwrap();

        let mesh = Mesh {
            index_count: data.indices.len() as u32,
            vertex_buffer: create_vertex_buffer(device, data.vertices),
            index_buffer: create_index_buffer(device, data.indices),
            instance_buffer: create_instance_buffer(device, &[]),
        };

        Ok(mesh)
    }

    pub fn render(&mut self, target: &wgpu::TextureView, mesh: &mut Mesh, instances: &[Instance]) {
        let GpuInfo {
            device,
            queue,
            swapchain,
            ..
        } = &*self.gpu_info.lock().unwrap();

        mesh.instance_buffer = create_instance_buffer(device, instances);

        // Update uniform
        queue.write_buffer(&self.view_buffer, 0, self.view.as_std140().as_bytes());

        // Draw all instances of all meshes
        let mut cmd = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        {
            let mut render_pass = cmd.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Default Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: target,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                    resolve_target: None,
                }],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.view_bind_group, &[]);
            render_pass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, mesh.instance_buffer.slice(..));
            render_pass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..mesh.index_count, 0, 0..instances.len() as u32);
        }

        // Submit
        queue.submit(vec![cmd.finish()]);
    }
}

fn first_available_id_index(ids: &[usize]) -> usize {
    ids.iter()
        .copied()
        .enumerate()
        .find_map(|(index, id)| if index != id { Some(index) } else { None })
        .unwrap_or(ids.len())
}

fn create_view_buffer(device: &wgpu::Device, view: &View) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Default View Buffer"),
        contents: view.as_std140().as_bytes(),
        usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    })
}

fn create_vertex_buffer(device: &wgpu::Device, vertices: &[Vertex]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Default Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsage::VERTEX,
    })
}

fn create_index_buffer(device: &wgpu::Device, indices: &[u16]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Default Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsage::INDEX,
    })
}

fn create_instance_buffer(device: &wgpu::Device, instances: &[Instance]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Default Instance Buffer"),
        contents: bytemuck::cast_slice(instances),
        usage: wgpu::BufferUsage::VERTEX,
    })
}
