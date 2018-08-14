use std::sync::Arc;

use fnv::FnvBuildHasher;
use gfx::{traits::FactoryExt, IndexBuffer, Slice};
use gfx_device_gl;
use indexmap::IndexMap;

type FnvIndexMap<K, V> = IndexMap<K, V, FnvBuildHasher>;

use renderer::Renderer;
use voxel::{mesh::VertexBuffer, pipeline::VoxelPipeline, MaterialKind, Mesh};

gfx_defines! {
    constant ModelConsts {
        model_mat: [[f32; 4]; 4] = "model_mat",
    }
}

pub struct Model {
    vbufs: FnvIndexMap<MaterialKind, (VertexBuffer, Slice<gfx_device_gl::Resources>)>,
}

impl Model {
    pub fn new(renderer: &mut Renderer, meshes: &FnvIndexMap<MaterialKind, Mesh>) -> Model {
        let mut vbufs = FnvIndexMap::with_capacity_and_hasher(4, Default::default());

        meshes.iter().for_each(|(mat, mesh)| {
            if mesh.vert_count() < 1 {
                return;
            }
            let vbuf = renderer.factory_mut().create_vertex_buffer(mesh.vertices());

            let slice = Slice::<gfx_device_gl::Resources> {
                start: 0,
                end: mesh.vert_count(),
                base_vertex: 0,
                instances: None,
                buffer: IndexBuffer::Auto,
            };

            vbufs.insert(*mat, (vbuf, slice));
        });
        Model { vbufs }
    }

    pub(super) fn vbufs(&self) -> &FnvIndexMap<MaterialKind, (VertexBuffer, Slice<gfx_device_gl::Resources>)> {
        &self.vbufs
    }
}
