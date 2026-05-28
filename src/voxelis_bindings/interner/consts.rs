use pyo3::prelude::*;
use pyo3::exceptions::PyTypeError;

use voxelis::interner::{
    MAX_ALLOWED_DEPTH,
    MAX_CHILDREN,
    NODE_TYPE_LEAF,
    NODE_TYPE_BRANCH,
    PATTERNS_TYPE_BRANCH,
    PATTERNS_TYPE_LEAF,
    CHILD_ABSENT,
    PREALLOCATED_STACK_SIZE,
};

#[pyclass(module = "interner", name = "InternerConstants")]
pub struct PyInternerConstants;

#[pymethods]
impl PyInternerConstants {
    // ban instantiation
    #[new]
    fn new() -> PyResult<Self> {
        Err(PyTypeError::new_err("Constants cannot be instantiated"))
    }

    // ------------------------------
    // class attributes for constants
    // ------------------------------
    #[classattr]
    pub const MAX_ALLOWED_DEPTH: usize = MAX_ALLOWED_DEPTH;

    #[classattr]
    pub const MAX_CHILDREN: usize = MAX_CHILDREN;

    #[classattr]
    pub const NODE_TYPE_LEAF: u8 = NODE_TYPE_LEAF;

    #[classattr]
    pub const NODE_TYPE_BRANCH: u8 = NODE_TYPE_BRANCH;

    #[classattr]
    pub const PATTERNS_TYPE_BRANCH: usize = PATTERNS_TYPE_BRANCH;

    #[classattr]
    pub const PATTERNS_TYPE_LEAF: usize = PATTERNS_TYPE_LEAF;

    #[classattr]
    pub const CHILD_ABSENT: u8 = CHILD_ABSENT;

    #[classattr]
    pub const PREALLOCATED_STACK_SIZE: usize = PREALLOCATED_STACK_SIZE;
}