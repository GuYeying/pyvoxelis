use pyo3::prelude::*;


#[pyclass(module = "core", from_py_object, name = "Lod")]
#[derive(Clone, Copy)]
pub struct PyLod {
    pub inner: voxelis::Lod,
}

#[pymethods]
impl PyLod {
    // constructor
    #[new]
    pub fn new(lod: u8) -> Self {
        Self { inner: voxelis::Lod::new(lod) }
    }

    /// get the LOD value
    pub fn lod(&self) -> u8 {
        self.inner.lod()
    }

    /// Python str()
    pub fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    /// Python repr()
    pub fn __repr__(&self) -> String {
        format!("Lod({})", self.inner.lod())
    }
}