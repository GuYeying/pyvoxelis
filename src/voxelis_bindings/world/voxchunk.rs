use pyo3::prelude::*;
use glam::IVec3;
use std::collections::HashMap;

use voxelis::spatial::{
    VoxOpsBatch, VoxOpsConfig, VoxOpsDirty,VoxOpsRead, VoxOpsState, VoxOpsWrite,
};
use voxelis::world::VoxChunk;
use voxelis::{Batch, Lod, MaxDepth};

use crate::voxelis_bindings::core::PyMaxDepth;

use super::super::PyVoxInternerI32 as PyVoxInterner;
use super::super::core::PyBatchI32 as PyBatch;

#[pyclass(module = "world", name = "VoxChunk", skip_from_py_object)]
pub struct PyVoxChunk {
    pub(crate) inner: *mut VoxChunk,
}

unsafe impl Send for PyVoxChunk {}
unsafe impl Sync for PyVoxChunk {}

impl PyVoxChunk {
    pub fn from_ptr(ptr: &mut VoxChunk) -> Self {
        Self { inner: ptr as *mut _ }
    }
}

#[pymethods]
impl PyVoxChunk {
    #[new]
    fn new() -> PyResult<Self> {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Use model.get_or_create_chunk()",
        ))
    }

    // -------------------------------------------------------------------------
    //  Getter
    // -------------------------------------------------------------------------
    #[getter]
    fn position(&self) -> (i32, i32, i32) {
        let inner = unsafe { &*self.inner };
        let p = inner.get_position();
        (p.x, p.y, p.z)
    }

    #[getter]
    fn world_position(&self) -> (f32, f32, f32) {
        let inner = unsafe { &*self.inner };
        let p = inner.get_world_position();
        (p.x, p.y, p.z)
    }

    #[getter]
    fn chunk_size(&self) -> f32 {
        let inner = unsafe { &*self.inner };
        inner.chunk_size()
    }

    #[getter]
    fn is_empty(&self) -> bool {
        let inner = unsafe { &*self.inner };
        inner.is_empty()
    }

    #[getter]
    fn is_leaf(&self) -> bool {
        let inner = unsafe { &*self.inner };
        inner.is_leaf()
    }

    #[getter]
    fn is_dirty(&self) -> bool {
        let inner = unsafe { &*self.inner };
        inner.is_dirty()
    }

    #[getter]
    fn root_id(&self) -> u32 {
        let inner = unsafe { &*self.inner };
        inner.get_root_id().index()
    }

    // -------------------------------------------------------------------------
    // 配置查询
    // -------------------------------------------------------------------------
    fn voxel_size(&self, lod_level: u8) -> f32 {
        let inner = unsafe { &*self.inner };
        let lod = Lod::new(lod_level);
        inner.voxel_size(lod)
    }

    fn max_depth(&self, lod_level: u8) -> PyMaxDepth {
        let inner = unsafe { &*self.inner };
        let lod = Lod::new(lod_level);
        let raw_depth = inner.max_depth(lod).max();
        PyMaxDepth::new(raw_depth).unwrap()
    }


    fn voxels_per_axis(&self, lod_level: u8) -> u32 {
        let inner = unsafe { &*self.inner };
        let lod = Lod::new(lod_level);
        inner.voxels_per_axis(lod)
    }

    // -------------------------------------------------------------------------
    // 可变操作
    // -------------------------------------------------------------------------
    fn set_position(&mut self, x: i32, y: i32, z: i32) {
        let inner = unsafe { &mut *self.inner };
        inner.set_position(x, y, z);
    }

    fn mark_dirty(&mut self) {
        let inner = unsafe { &mut *self.inner };
        inner.mark_dirty();
    }

    fn clear_dirty(&mut self) {
        let inner = unsafe { &mut *self.inner };
        inner.clear_dirty();
    }

    // -------------------------------------------------------------------------
    // Voxel read and write
    // -------------------------------------------------------------------------
    fn get(&self, interner: &PyVoxInterner, x: i32, y: i32, z: i32) -> Option<i32> {
        let inner = unsafe { &*self.inner };
        let pos = IVec3::new(x, y, z);
        let guard = interner.inner.read();
        inner.get(&*guard, pos)
    }

    fn set(&mut self, interner: &PyVoxInterner, x: i32, y: i32, z: i32, value: i32) -> bool {
        let inner = unsafe { &mut *self.inner };
        let pos = IVec3::new(x, y, z);
        let mut guard = interner.inner.write();
        inner.set(&mut *guard, pos, value)
    }

    fn fill(&mut self, interner: &PyVoxInterner, value: i32) {
        let inner = unsafe { &mut *self.inner };
        let mut guard = interner.inner.write();
        inner.fill(&mut *guard, value);
    }

    fn clear(&mut self, interner: &PyVoxInterner) {
        let inner = unsafe { &mut *self.inner };
        let mut guard = interner.inner.write();
        inner.clear(&mut *guard);
    }

    // -------------------------------------------------------------------------
    // Batch
    // -------------------------------------------------------------------------
    fn create_batch(&self) -> PyBatch {
        let inner: &VoxChunk = unsafe { &*self.inner };
        let batch: Batch<i32> = inner.create_batch();
        PyBatch::wrap(batch)
    }

    fn apply_batch(&mut self, interner: &PyVoxInterner, batch: &PyBatch) -> bool {
        let inner = unsafe { &mut *self.inner };
        let mut guard = interner.inner.write();
        inner.apply_batch(&mut *guard, &batch.inner)
    }

    // -------------------------------------------------------------------------
    // test data generation
    // -------------------------------------------------------------------------
    fn generate_test_data(&mut self, interner: &PyVoxInterner) {
        let inner = unsafe { &mut *self.inner };
        let mut guard = interner.inner.write();
        inner.generate_test_data(&mut *guard);
    }

    fn generate_test_sphere(
        &mut self,
        interner: &PyVoxInterner,
        cx: i32,
        cy: i32,
        cz: i32,
        radius: i32,
        value: i32,
    ) {
        let inner = unsafe { &mut *self.inner };
        let mut guard = interner.inner.write();
        inner.generate_test_sphere(&mut *guard, IVec3::new(cx, cy, cz), radius, value);
    }

    // -------------------------------------------------------------------------
    // mesh generate
    // -------------------------------------------------------------------------
    fn generate_mesh(
        &self,
        interner: &PyVoxInterner,
        lod_level: u8,
        ox: f32,
        oy: f32,
        oz: f32,
    ) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<u32>) {
        let inner = unsafe { &*self.inner };
        let guard = interner.inner.read();
        let lod = Lod::new(lod_level);
        let offset = glam::Vec3::new(ox, oy, oz);

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();

        inner.generate_mesh_arrays(
            &*guard,
            &mut vertices,
            &mut normals,
            &mut indices,
            offset,
            lod,
        );

        let vertices: Vec<_> = vertices.iter().map(|v| [v.x, v.y, v.z]).collect();
        let normals: Vec<_> = normals.iter().map(|n| [n.x, n.y, n.z]).collect();

        (vertices, normals, indices)
    }

    #[staticmethod]
    fn with_position(
        chunk_size: f32,
        max_depth: u8,
        x: i32,
        y: i32,
        z: i32,
    ) -> Self {
        let max_depth = MaxDepth::new(max_depth);
        let chunk = VoxChunk::with_position(chunk_size, max_depth, x, y, z);
        Self {
            inner: Box::into_raw(Box::new(chunk)),
        }
    }




    #[staticmethod]
    fn deserialize(
        interner: &PyVoxInterner,
        leaf_patterns: HashMap<u32, (u64, i32)>,
        patterns: HashMap<u32, (u64, [u32; 8], i32)>,
        data: &[u8],
        chunk_size: f32,
        max_depth: u8,
    ) -> PyResult<Self> {
        use std::io::BufReader;
        use voxelis::world::VoxChunk;

        let max_depth = voxelis::MaxDepth::new(max_depth);
        let mut reader = BufReader::new(data);


        let leaf_patterns: rustc_hash::FxHashMap<u32, (voxelis::BlockId, i32)> = leaf_patterns
            .into_iter()
            .map(|(k, (raw_id, v))| (k, (voxelis::BlockId::from_raw(raw_id), v)))
            .collect();

        let patterns: rustc_hash::FxHashMap<u32, (voxelis::BlockId, [u32; 8], i32)> = patterns
            .into_iter()
            .map(|(k, (raw_id, arr, v))| (k, (voxelis::BlockId::from_raw(raw_id), arr, v)))
            .collect();

        let mut guard = interner.inner.write();

        let chunk = VoxChunk::deserialize(
            &mut *guard,
            &leaf_patterns,
            &patterns,
            &mut reader,
            chunk_size,
            max_depth,
        );

        Ok(Self {
            inner: Box::into_raw(Box::new(chunk)),
        })
    }




    // -------------------------------------------------------------------------
    // serialize
    // -------------------------------------------------------------------------
    fn serialize(&self, id_map: HashMap<u32, u32>) -> Vec<u8> {
        let inner = unsafe { &*self.inner };
        let mut data = Vec::new();
        
        // Convert standard HashMap from Python to FxHashMap for Rust
        let fx_map: rustc_hash::FxHashMap<u32, u32> = id_map.into_iter().collect();
        
        inner.serialize(&fx_map, &mut data);
        data
    }
}