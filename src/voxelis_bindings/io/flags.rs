use pyo3::{exceptions::PyTypeError, prelude::*};
use voxelis::io::Flags;
#[pyclass(module = "io", name = "Flags", skip_from_py_object)]
pub struct PyFlags;

#[pymethods]
impl PyFlags {
    #[new]
    fn new() -> PyResult<Self> {
        Err(PyTypeError::new_err("Flags cannot be instantiated"))
    }

    #[classattr]
    pub const NONE: u16 = Flags::NONE.bits();
    #[classattr]
    pub const COMPRESSED: u16 = Flags::COMPRESSED.bits();
    #[classattr]
    pub const DEFAULT: u16 = Flags::DEFAULT.bits();
}