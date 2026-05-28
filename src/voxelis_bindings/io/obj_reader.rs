use pyo3::prelude::*;
use std::path::Path;
use voxelis::io::obj_reader::Obj;

// -----------------------------------------------------
// 极简封装 · 自动转 Python 列表/元组
// -----------------------------------------------------
#[pyclass(module = "io", name = "Obj", skip_from_py_object)]
pub struct PyObj {
    inner: Obj,
}

#[pymethods]
impl PyObj {
    // static method parse
    #[staticmethod]
    fn parse(path: &str) -> PyResult<Self> {
        let obj = Obj::parse(&Path::new(path));
        Ok(Self { inner: obj })
    }

    // -----------------------------------------------------
    // Automatically converts to Python list
    // -----------------------------------------------------
    #[getter]
    fn vertices(&self) -> Vec<(f64, f64, f64)> {
        self.inner.vertices.iter()
            .map(|v| (v.x, v.y, v.z))
            .collect()
    }

    #[getter]
    fn faces(&self) -> Vec<(i32, i32, i32)> {
        self.inner.faces.iter()
            .map(|f| (f.x, f.y, f.z))
            .collect()
    }

    #[getter]
    fn aabb(&self) -> ((f64, f64, f64), (f64, f64, f64)) {
        let min = self.inner.aabb.0;
        let max = self.inner.aabb.1;
        ((min.x, min.y, min.z), (max.x, max.y, max.z))
    }

    #[getter]
    fn size(&self) -> (f64, f64, f64) {
        let s = self.inner.size;
        (s.x, s.y, s.z)
    }

    // debug output
    fn __repr__(&self) -> String {
        format!(
            "Obj(vertices={}, faces={})",
            self.inner.vertices.len(),
            self.inner.faces.len()
        )
    }
}