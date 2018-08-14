use std::sync::Arc;

use fnv::{FnvBuildHasher, FnvHashMap};
use gfx::{self, Slice};
use gfx_device_gl;
use indexmap::IndexMap;

type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;

use consts::{ConstHandle, GlobalConsts};
use pipeline::Pipeline;
use renderer::{HdrDepthFormat, HdrFormat, Renderer};
use shader::Shader;
use voxel::{mesh::VertexBuffer, Material, MaterialKind, Model, ModelConsts, Vertex};

type VoxelPipelineData = voxel_pipeline::Data<gfx_device_gl::Resources>;
type WaterPipelineData = water_pipeline::Data<gfx_device_gl::Resources>;

gfx_defines! {
    pipeline voxel_pipeline {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        model_consts: gfx::ConstantBuffer<ModelConsts> = "model_consts",
        global_consts: gfx::ConstantBuffer<GlobalConsts> = "global_consts",
        out_color: gfx::BlendTarget<HdrFormat> = ("target", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
        out_depth: gfx::DepthTarget<HdrDepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }

    pipeline water_pipeline {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        model_consts: gfx::ConstantBuffer<ModelConsts> = "model_consts",
        global_consts: gfx::ConstantBuffer<GlobalConsts> = "global_consts",
        out_color: gfx::BlendTarget<HdrFormat> = ("target", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
        out_depth: gfx::DepthTarget<HdrDepthFormat> = gfx::preset::depth::LESS_EQUAL_WRITE,
    }
}

struct DrawPacket {
    vbuf: VertexBuffer,
    slice: Slice<gfx_device_gl::Resources>,
    model_consts: gfx::handle::Buffer<gfx_device_gl::Resources, ModelConsts>,
    global_consts: gfx::handle::Buffer<gfx_device_gl::Resources, GlobalConsts>,
}

pub struct VoxelPipeline {
    voxel_pipeline: Pipeline<voxel_pipeline::Init<'static>>,
    water_pipeline: Pipeline<water_pipeline::Init<'static>>,
    draw_queue: FnvIndexMap<MaterialKind, Vec<DrawPacket>>,
}

impl VoxelPipeline {
    pub fn new(renderer: &mut Renderer) -> Self {
        let voxel_pipeline = Pipeline::new(
            renderer.factory_mut(),
            voxel_pipeline::new(),
            &Shader::from_file("shaders/voxel/voxel.vert").expect("Could not load voxel vertex shader"),
            &Shader::from_file("shaders/voxel/voxel.frag").expect("Could not load voxel fragment shader"),
        );

        let water_pipeline = Pipeline::new(
            renderer.factory_mut(),
            water_pipeline::new(),
            &Shader::from_file("shaders/voxel/water.vert").expect("Could not load voxel vertex shader"),
            &Shader::from_file("shaders/voxel/water.frag").expect("Could not load voxel fragment shader"),
        );

        VoxelPipeline {
            voxel_pipeline,
            water_pipeline,
            draw_queue: FnvIndexMap::with_capacity_and_hasher(4, Default::default()),
        }
    }

    pub fn draw_model(
        &mut self,
        model: &Model,
        model_consts: &ConstHandle<ModelConsts>,
        global_consts: &ConstHandle<GlobalConsts>,
    ) {
        model.vbufs().iter().for_each(|(mat, data)| {
            let queued = self.draw_queue.entry(*mat).or_insert(Vec::new());
            let (vbuf, slice) = data;
            queued.push(DrawPacket {
                vbuf: vbuf.clone(),
                slice: slice.clone(),
                model_consts: model_consts.buffer().clone(),
                global_consts: global_consts.buffer().clone(),
            })
        });
    }

    pub fn flush(&mut self, renderer: &mut Renderer) {
        let out_color = renderer.hdr_render_view().clone();
        let out_depth = renderer.hdr_depth_view().clone();
        let encoder = renderer.encoder_mut();
        let vox_pso = self.voxel_pipeline.pso();
        let water_pso = self.water_pipeline.pso();
        self.draw_queue.sort_keys();
        self.draw_queue.iter_mut().for_each(|(mat, ref mut packets)| {
            packets.drain(..).for_each(|packet| match *mat {
                MaterialKind::Water => {
                    let pipe_data = &WaterPipelineData {
                        vbuf: packet.vbuf,
                        model_consts: packet.model_consts,
                        global_consts: packet.global_consts,
                        out_color: out_color.clone(),
                        out_depth: out_depth.clone(),
                    };
                    encoder.draw(&packet.slice, water_pso, pipe_data);
                },
                _ => {
                    let pipe_data = &VoxelPipelineData {
                        vbuf: packet.vbuf,
                        model_consts: packet.model_consts,
                        global_consts: packet.global_consts,
                        out_color: out_color.clone(),
                        out_depth: out_depth.clone(),
                    };
                    encoder.draw(&packet.slice, vox_pso, pipe_data);
                },
            });
        });
    }
}
