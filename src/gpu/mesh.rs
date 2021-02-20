use std::{marker::PhantomData, sync::{Arc, Mutex}};

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use super::GpuInfo;

// Vertices and indices of a mesh, unrelated to the gpu
pub struct MeshData<'a, Vertex: Pod> {
    pub vertices: &'a [Vertex],
    pub indices: &'a [u16],
}

// A full gpu-uploaded mesh with instance information
pub struct Mesh<Vertex: Pod, Instance: Pod> {
    index_count: u32,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    instance_buffer: wgpu::Buffer,
    _vertex: PhantomData<Vertex>,
    _instance: PhantomData<Instance>
}

impl<Vertex: Pod, Instance: Pod> Mesh<Vertex, Instance> {
    pub fn new(gpu_info: Arc<Mutex<GpuInfo>>, data: &MeshData<Vertex>) -> Self {
        let GpuInfo { device, .. } = &*gpu_info.lock().unwrap();

        Self {
            index_count: data.indices.len() as u32,
            vertex_buffer: create_vertex_buffer(device, data.vertices),
            index_buffer: create_index_buffer(device, data.indices),
            instance_buffer: create_instance_buffer(device, &[] as &[Instance]),
            _vertex: PhantomData,
            _instance: PhantomData
        }
    }
}

fn create_vertex_buffer(device: &wgpu::Device, vertices: &[impl Pod]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(vertices),
        usage: wgpu::BufferUsage::VERTEX,
    })
}

fn create_index_buffer(device: &wgpu::Device, indices: &[u16]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(indices),
        usage: wgpu::BufferUsage::INDEX,
    })
}

fn create_instance_buffer(device: &wgpu::Device, instances: &[impl Pod]) -> wgpu::Buffer {
    device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Instance Buffer"),
        contents: bytemuck::cast_slice(instances),
        usage: wgpu::BufferUsage::VERTEX,
    })
}
