use pyo3::prelude::*;
use voxelis::core::BlockId; 

// ============================================================================
// PyO3 bind BlockId
// ============================================================================
#[pyclass(module = "core", from_py_object, name = "BlockId")]
#[derive(Clone, Copy)]
pub struct PyBlockId {
    pub inner: BlockId,
}

#[pymethods]
impl PyBlockId {
    // ========================================================================
    // constructor & static methods
    // ========================================================================
    #[new]
    fn new() -> Self {
        Self { inner: BlockId::default() }
    }

    #[staticmethod]
    fn from_raw(raw: u64) -> Self {
        Self { inner: BlockId::from_raw(raw) }
    }

    #[staticmethod]
    fn new_leaf(index: u32, generation: u16) -> Self {
        Self { inner: BlockId::new_leaf(index, generation) }
    }

    #[staticmethod]
    fn new_branch(index: u32, generation: u16, types: u8, mask: u8) -> Self {
        Self { inner: BlockId::new_branch(index, generation, types, mask) }
    }

    // ========================================================================
    // const values
    // ========================================================================
    #[classattr]
    const INVALID: Self = Self { inner: BlockId::INVALID };

    #[classattr]
    const EMPTY: Self = Self { inner: BlockId::EMPTY };

    #[classattr]
    const MAX_INDEX: u32 = BlockId::MAX_INDEX;

    #[classattr]
    const MAX_GENERATION: u16 = BlockId::MAX_GENERATION;

    // ========================================================================
    // getter methods
    // ========================================================================
    fn index(&self) -> u32 {
        self.inner.index()
    }

    fn generation(&self) -> u16 {
        self.inner.generation()
    }

    fn types(&self) -> PyResult<u8> {
        if self.inner.is_branch() {
            Ok(self.inner.types())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Cannot get types from a leaf node",
            ))
        }
    }

    fn mask(&self) -> PyResult<u8> {
        if self.inner.is_branch() {
            Ok(self.inner.mask())
        } else {
            Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Cannot get mask from a leaf node",
            ))
        }
    }

    // ========================================================================
    // state check methods
    // ========================================================================
    fn has_child(&self, child_index: u8) -> PyResult<bool> {
        if !self.inner.is_branch() {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Cannot check child on a leaf node",
            ));
        }
        if child_index >= 8 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "Child index out of bounds (0-7)",
            ));
        }
        Ok(self.inner.has_child(child_index))
    }

    fn is_leaf(&self) -> bool {
        self.inner.is_leaf()
    }

    fn is_branch(&self) -> bool {
        self.inner.is_branch()
    }

    fn is_invalid(&self) -> bool {
        self.inner.is_invalid()
    }

    fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn raw(&self) -> u64 {
        self.inner.raw()
    }

    // ========================================================================
    // Python magic methods
    // ========================================================================
    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }
}


// // ============================================================================
// // 注册到模块
// // ============================================================================
// pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_class::<PyBlockId>()?;
//     Ok(())
// }