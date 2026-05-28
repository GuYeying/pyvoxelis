use pyo3::prelude::*;
use glam::IVec3;

use voxelis::world::{VoxModel};
use voxelis::core::MaxDepth;

use super::super::world::PyVoxChunk;
use super::super::PyVoxInternerI32;

#[pyclass(module = "world", name = "VoxModel")]
pub struct PyVoxModel {
    pub inner: VoxModel,
}

#[pymethods]
impl PyVoxModel {
    #[staticmethod]
    fn empty(max_depth: u8, chunk_world_size: f32, memory_budget: usize) -> Self {
        let max_depth = MaxDepth::new(max_depth);
        let inner = VoxModel::empty(max_depth, chunk_world_size, memory_budget);
        Self { inner }
    }

    #[staticmethod]
    fn new(max_depth: u8, chunk_world_size: f32, memory_budget: usize) -> Self {
        let max_depth = MaxDepth::new(max_depth);
        let inner = VoxModel::new(max_depth, chunk_world_size, memory_budget);
        Self { inner }
    }

    #[staticmethod]
    fn with_dimensions(
        max_depth: u8,
        chunk_world_size: f32,
        world_bounds: (i32, i32, i32),
        memory_budget: usize,
    ) -> Self {
        let max_depth = MaxDepth::new(max_depth);
        let world_bounds = IVec3::new(world_bounds.0, world_bounds.1, world_bounds.2);
        let inner = VoxModel::with_dimensions(max_depth, chunk_world_size, world_bounds, memory_budget);
        Self { inner }
    }


    fn get_or_create_chunk(&mut self, x: i32, y: i32, z: i32) -> PyVoxChunk {
        let pos = IVec3::new(x, y, z);
        let chunk = self.inner.get_or_create_chunk(pos);

        // safe transmute：&mut VoxChunk → PyVoxChunk
        unsafe { std::mem::transmute_copy(&chunk) }
    }
    

    fn clear(&mut self) {
        self.inner.clear();
    }

    fn resize(&mut self, bounds: (i32, i32, i32)) {
        let bounds = IVec3::new(bounds.0, bounds.1, bounds.2);
        self.inner.resize(bounds);
    }

    fn get_bounds_size(&self) -> usize {
        self.inner.get_bounds_size()
    }

    fn is_position_in_bounds(&self, pos: (i32, i32, i32)) -> bool {
        let pos = IVec3::new(pos.0, pos.1, pos.2);
        self.inner.is_position_in_bounds(pos)
    }

    #[getter]
    fn max_depth(&self) -> u8 {
        self.inner.max_depth().max()
    }

    #[getter]
    fn voxels_per_axis(&self) -> usize {
        self.inner.voxels_per_axis()
    }

    #[getter]
    fn world_bounds(&self) -> (i32, i32, i32) {
        let b = self.inner.world_bounds;
        (b.x, b.y, b.z)
    }

    fn get_interner(&self) -> PyResult<PyVoxInternerI32> {
        let interner= self.inner.get_interner();
        Ok(PyVoxInternerI32 { inner: interner })
    }

    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        self.inner.serialize(&mut data);
        data
    }

    fn deserialize(&mut self, data: &[u8]) -> PyResult<()> {
        self.inner.deserialize(data);
        Ok(())
    }
}