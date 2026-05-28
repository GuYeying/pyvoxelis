use pyo3::prelude::*;
use glam::IVec3;
use voxelis::world::VoxWorld;

// ===========================================================================
// VoxWorld wrapper (read-only, cannot be constructed from Python, can only be returned)
// ===========================================================================
#[pyclass(module = "world", name = "VoxWorld", skip_from_py_object)]
pub struct PyVoxWorld {
    pub inner: VoxWorld,
}

#[pymethods]
impl PyVoxWorld {
    // ban direct instantiation from Python
    #[new]
    fn new() -> PyResult<Self> {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Cannot instantiate VoxWorld directly. Use create() or create_with_size()",
        ))
    }

    // -----------------------------------------------------------------------
    // static constructors (factory methods)
    // -----------------------------------------------------------------------
    #[staticmethod]
    fn create() -> Self {
        Self {
            inner: VoxWorld::new(),
        }
    }

    #[staticmethod]
    fn create_with_size(x: i32, y: i32, z: i32) -> Self {
        let size = IVec3::new(x, y, z);
        Self {
            inner: VoxWorld::with_size(size),
        }
    }

    // -----------------------------------------------------------------------
    // read-only properties
    // -----------------------------------------------------------------------
    #[getter]
    fn chunks_size(&self) -> (i32, i32, i32) {
        let s = self.inner.chunks_size;
        (s.x, s.y, s.z)
    }

    #[getter]
    fn chunks_len(&self) -> usize {
        self.inner.chunks_len
    }

    // -----------------------------------------------------------------------
    // methods that modify the world
    // -----------------------------------------------------------------------
    fn clear(&mut self) {
        self.inner.clear();
    }

    fn resize(&mut self, x: i32, y: i32, z: i32) {
        let size = IVec3::new(x, y, z);
        self.inner.resize(size);
    }

    fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        self.inner.serialize(&mut data);
        data
    }

    // -----------------------------------------------------------------------
    // debug output
    // -----------------------------------------------------------------------
    fn __repr__(&self) -> String {
        format!(
            "VoxWorld(size={:?}, chunks={})",
            self.chunks_size(),
            self.inner.chunks.len()
        )
    }
}