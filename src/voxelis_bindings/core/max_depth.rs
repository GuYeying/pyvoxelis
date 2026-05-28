use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use voxelis::MaxDepth;
use super::PyLod;


#[pyclass(module = "core", from_py_object, name = "MaxDepth")]
#[derive(Clone, Copy)]
pub struct PyMaxDepth {
    pub inner: MaxDepth,
}

#[pymethods]
impl PyMaxDepth {
    // constructor with validation
    #[new]
    pub fn new(max: u8) -> PyResult<Self> {
        if max < voxelis::interner::MAX_ALLOWED_DEPTH as u8 {
            Ok(Self {
                inner: MaxDepth::new(max)
            })
        } else {
            Err(PyValueError::new_err("Max depth exceeds allowed limit"))
        }
    }

    // get depth u8
    pub fn max(&self) -> u8 {
        self.inner.max()
    }

    // get depth as usize
    pub fn as_usize(&self) -> usize {
        self.inner.as_usize()
    }

    // get depth for specific LOD
    pub fn for_lod(&self, lod: &PyLod) -> Self {
        Self {
            inner: self.inner.for_lod(lod.inner)
        }
    }

    // Python str()
    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    // Python repr()
    fn __repr__(&self) -> String {
        format!("MaxDepth({})", self.inner.max())
    }
}