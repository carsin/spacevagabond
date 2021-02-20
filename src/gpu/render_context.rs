use std::sync::{Arc, Mutex};

use super::{
    pipelines::default::{DefaultPipeline, View},
    GpuInfo,
};

pub struct RenderContext {
    gpu_info: Arc<Mutex<GpuInfo>>,
    default_pipeline: Arc<Mutex<DefaultPipeline>>,
}

impl RenderContext {
}
