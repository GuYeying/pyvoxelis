use pyo3::prelude::*;
use pyo3::exceptions::{PyValueError, PyIndexError};
use voxelis::TraversalDepth;


#[pyclass(module = "core", from_py_object, name = "TraversalDepth")]
#[derive(Clone, Copy)]
pub struct PyTraversalDepth {
    pub inner: TraversalDepth,
}

#[pymethods]
impl PyTraversalDepth {
    // constructor with validation
    #[new]
    pub fn new(current: u8, max: u8) -> PyResult<Self> {
        if current > max {
            return Err(PyValueError::new_err("Current depth cannot be greater than max depth"));
        }
        if max >= voxelis::interner::MAX_ALLOWED_DEPTH as u8 {
            return Err(PyValueError::new_err("Max depth exceeds allowed limit"));
        }

        Ok(Self {
            inner: TraversalDepth::new(current, max)
        })
    }

    // get current
    pub fn current(&self) -> u8 {
        self.inner.current()
    }

    // get max
    pub fn max(&self) -> u8 {
        self.inner.max()
    }

    // current +1
    pub fn increment(&self) -> PyResult<Self> {
        let new_current = self.current() + 1;
        if new_current > self.max() {
            return Err(PyIndexError::new_err("Cannot increment: current exceeds max depth"));
        }
        Ok(Self {
            inner: self.inner.increment()
        })
    }

    // current -1
    pub fn decrement(&self) -> PyResult<Self> {
        if self.current() == 0 {
            return Err(PyIndexError::new_err("Cannot decrement: current is already zero"));
        }
        Ok(Self {
            inner: self.inner.decrement()
        })
    }

    // Python str()
    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }

    // Python repr()
    fn __repr__(&self) -> String {
        format!("TraversalDepth(current={}, max={})", self.current(), self.max())
    }
}