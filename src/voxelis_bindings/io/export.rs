use pyo3::prelude::*;
use std::path::PathBuf;
use voxelis::Lod;
use voxelis::io::export::{export_model_to_obj, export_model_to_vtm};

use crate::voxelis_bindings::world::PyVoxModel;

/// Static utility class for model exporting
#[pyclass(module = "io", name = "Export", skip_from_py_object)]
#[derive(Clone, Copy)]
pub struct PyExport;

#[pymethods]
impl PyExport {

    #[staticmethod]
    pub fn export_to_obj(
        name: &str,
        path: &str,
        model: &PyVoxModel,
        lod_level: u8,
    ) -> PyResult<()> {
        let lod = Lod::new(lod_level);
        let path_buf = PathBuf::from(path);
        
        export_model_to_obj(
            name.to_string(),
            &path_buf,
            &model.inner,
            lod
        );
        
        Ok(())
    }

    // 导出为 VTM
    #[staticmethod]
    pub fn export_to_vtm(
        name: &str,
        path: &str,
        model: &PyVoxModel,
    ) -> PyResult<()> {
        let path_buf = PathBuf::from(path);
        
        export_model_to_vtm(
            name.to_string(),
            &path_buf,
            &model.inner
        );
        
        Ok(())
    }
}