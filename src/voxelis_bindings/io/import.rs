use pyo3::prelude::*;
use std::path::PathBuf;
use voxelis::io::import::import_model_from_vtm;

use crate::voxelis_bindings::world::PyVoxModel;

// Static utilities for model import
#[pyclass(module = "io", name = "Import", skip_from_py_object)]
#[derive(Clone, Copy)]
pub struct PyImport;

#[pymethods]
impl PyImport {
    // import model from VTM 
    #[staticmethod]
    pub fn import_from_vtm(
        path: &str,
        memory_budget: usize,
        target_chunk_size: Option<f32>
    ) -> PyResult<PyVoxModel> {
        let path_buf = PathBuf::from(path);
        let model = import_model_from_vtm(&path_buf, memory_budget, target_chunk_size);
        Ok(PyVoxModel { inner: model })
    }
}