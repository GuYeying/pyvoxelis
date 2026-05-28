use pyo3::prelude::*;

use voxelis::interner::PatternsHashmap;
use super::super::PyBlockId;


// ======================================================================
// PatternsHashmap
// ======================================================================
#[pyclass(module = "interner", name = "PatternsHashmap")]
pub struct PyPatternsHashmap {
    inner: PatternsHashmap,
}

#[pymethods]
impl PyPatternsHashmap {
    #[new]
    fn new() -> Self {
        Self {
            inner: PatternsHashmap::default(),
        }
    }

    /// insert
    pub fn insert(&mut self, hash: u64, block_id: &PyBlockId) {
        self.inner.insert(hash, block_id.inner);
    }

    /// get
    pub fn get(&self, hash: u64) -> Option<PyBlockId> {
        self.inner.get(&hash).copied().map(|inner| PyBlockId { inner })
    }

    /// delete
    pub fn remove(&mut self, hash: u64) -> Option<PyBlockId> {
        self.inner.remove(&hash).map(|inner| PyBlockId { inner })
    }

    /// clear
    pub fn clear(&mut self) {
        self.inner.clear();
    }



    /// empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn keys(&self) -> Vec<u64> {
        self.inner.keys().copied().collect()
    }

    fn values(&self) -> Vec<PyBlockId> {

        self.inner
            .values()
            .copied()
            .map(|inner| PyBlockId { inner })
            .collect()
    }


    fn items(&self) -> Vec<(u64, PyBlockId)> {

        self.inner
            .iter()
            .map(|(k, v)| {
                (*k, PyBlockId { inner: *v })
            })
            .collect()
    }

    fn reserve(&mut self, additional: usize) {
        self.inner.reserve(additional);
    }

    fn __contains__(&self, hash: u64) -> bool {
        self.inner.contains_key(&hash)
    }

    fn __len__(&self) -> usize {
        self.inner.len()
    }

    fn __getitem__(&self, hash: u64) -> PyResult<PyBlockId> {
        self.inner
            .get(&hash)
            .copied()
            .map(|inner| PyBlockId { inner })
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                    format!("hash {} not found", hash)
                )
            })
    }

    fn __setitem__(
        &mut self,
        hash: u64,
        block_id: PyRef<'_, PyBlockId>) {

        self.inner.insert(hash, block_id.inner);
    }

    fn __delitem__(&mut self, hash: u64) -> PyResult<()> {

        self.inner
            .remove(&hash)
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyKeyError, _>(
                    format!("hash {} not found", hash)
                )
            })?;

        Ok(())
    }

    pub fn __repr__(&self) -> String {
        format!(
            "PatternsHashmap: len={}, capacity={}, load_factor={:.2}",
            self.inner.len(),
            self.inner.capacity(),
            self.inner.len() as f64 / self.inner.capacity() as f64)
    }

}