use pyo3::prelude::*;


use voxelis::io::consts::{
    VTM_VERSION,
    VTM_MAGIC,
    VTC_MAGIC,
    RESERVED_1,
    RESERVED_2,
};


#[pyclass(module = "io", name = "Constants", skip_from_py_object)]
pub struct PyIOConstants;

#[pymethods]
impl PyIOConstants {
    // 禁止实例化
    #[new]
    fn new() -> PyResult<Self> {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "IOConstants cannot be instantiated",
        ))
    }

    // 版本
    #[classattr]
    pub const VTM_VERSION: u16 = VTM_VERSION;

    // 魔数标识
    #[classattr]
    pub const VTM_MAGIC: [u8; 12] = VTM_MAGIC;
    #[classattr]
    pub const VTC_MAGIC: [u8; 12] = VTC_MAGIC;

    // 保留字段
    #[classattr]
    pub const RESERVED_1: u32 = RESERVED_1;
    #[classattr]
    pub const RESERVED_2: u32 = RESERVED_2;
}