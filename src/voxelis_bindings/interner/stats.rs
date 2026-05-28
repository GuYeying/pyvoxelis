
#[cfg(feature = "memory_stats")]
use pyo3::prelude::*;

#[cfg(feature = "memory_stats")]
use voxelis::interner::InternerStats;

/// Memory pool statistics (read-only)
#[cfg(feature = "memory_stats")]
#[pyclass(module = "interner", name = "InternerStats", skip_from_py_object)]
#[derive(Debug, Default, Clone, Copy)]
pub struct PyInternerStats {
    pub inner: InternerStats,
}

#[cfg(feature = "memory_stats")]
#[pymethods]
impl PyInternerStats {

    #[new]
    pub fn new() -> Self {
        Self::default()
    }

    // ============================================================
    // only exposed  getter attributes 
    // ============================================================
    #[getter]
    pub fn requested_budget(&self) -> usize {
        self.inner.requested_budget
    }

    #[getter]
    pub fn actual_budget(&self) -> usize {
        self.inner.actual_budget
    }

    #[getter]
    pub fn node_size(&self) -> usize {
        self.inner.node_size
    }

    #[getter]
    pub fn nodes_capacity(&self) -> usize {
        self.inner.nodes_capacity
    }

    #[getter]
    pub fn total_allocations(&self) -> usize {
        self.inner.total_allocations
    }

    #[getter]
    pub fn total_deallocations(&self) -> usize {
        self.inner.total_deallocations
    }

    #[getter]
    pub fn allocated_nodes(&self) -> usize {
        self.inner.allocated_nodes
    }

    #[getter]
    pub fn recycled_nodes(&self) -> usize {
        self.inner.recycled_nodes
    }

    #[getter]
    pub fn alive_nodes(&self) -> usize {
        self.inner.alive_nodes
    }

    #[getter]
    pub fn patterns(&self) -> usize {
        self.inner.patterns
    }

    #[getter]
    pub fn total_cache_hits(&self) -> usize {
        self.inner.total_cache_hits
    }

    #[getter]
    pub fn total_cache_misses(&self) -> usize {
        self.inner.total_cache_misses
    }

    #[getter]
    pub fn branch_cache_hits(&self) -> usize {
        self.inner.branch_cache_hits
    }

    #[getter]
    pub fn branch_cache_misses(&self) -> usize {
        self.inner.branch_cache_misses
    }

    #[getter]
    pub fn leaf_cache_hits(&self) -> usize {
        self.inner.leaf_cache_hits
    }

    #[getter]
    pub fn leaf_cache_misses(&self) -> usize {
        self.inner.leaf_cache_misses
    }

    #[getter]
    pub fn collapsed_branches(&self) -> usize {
        self.inner.collapsed_branches
    }

    #[getter]
    pub fn leaf_nodes(&self) -> usize {
        self.inner.leaf_nodes
    }

    #[getter]
    pub fn branch_nodes(&self) -> usize {
        self.inner.branch_nodes
    }

    #[getter]
    pub fn max_alive_nodes(&self) -> usize {
        self.inner.max_alive_nodes
    }

    #[getter]
    pub fn max_node_id(&self) -> usize {
        self.inner.max_node_id
    }

    #[getter]
    pub fn max_branch_ref_count(&self) -> usize {
        self.inner.max_branch_ref_count
    }

    #[getter]
    pub fn max_leaf_ref_count(&self) -> usize {
        self.inner.max_leaf_ref_count
    }

    #[getter]
    pub fn max_generation(&self) -> usize {
        self.inner.max_generation
    }

    #[getter]
    pub fn generations_overflows(&self) -> usize {
        self.inner.generations_overflows
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}