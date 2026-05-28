use pyo3::prelude::*;
use pyo3::types::{PyAny, PyList};
use pyo3::{Bound, PyRef};

use voxelis::{VoxInterner, interner::Children};

use super::super::PyBlockId;

mod consts;
mod hash;
mod stats;

pub use consts::PyInternerConstants;
pub use hash::PyPatternsHashmap;

#[cfg(feature = "memory_stats")]
pub use stats::PyInternerStats;

// ============================================================================
// PyChildren
// ============================================================================

#[pyclass(from_py_object, module = "core", name = "Children")]
#[derive(Clone)]
pub struct PyChildren {
    pub inner: Children,
}

#[pymethods]
impl PyChildren {

    #[new]
    fn new() -> Self {
        Self {
            inner: voxelis::interner::EMPTY_CHILD
        }
    }

    #[staticmethod]
    fn from_list(list: &Bound<'_, PyAny>) -> PyResult<Self> {

        let py_list = list.cast::<PyList>()?;

        if py_list.len() != 8 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "Children must contain exactly 8 BlockIds",
            ));
        }

        let mut arr = voxelis::interner::EMPTY_CHILD;

        for i in 0..8 {
            let elem = py_list.get_item(i)?;
            let bid = elem.extract::<PyBlockId>()?;
            arr[i] = bid.inner;
        }

        Ok(Self { inner: arr })
    }

    fn to_list(&self, py: Python<'_>) -> Py<PyList> {

        let list = PyList::empty(py);

        for &id in &self.inner {
            list.append(PyBlockId { inner: id }).unwrap();
        }

        list.unbind()
    }

    fn __len__(&self) -> usize {
        8
    }

    fn __getitem__(&self, idx: usize) -> PyResult<PyBlockId> {

        if idx >= 8 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "index out of range 0..7",
            ));
        }

        Ok(PyBlockId {
            inner: self.inner[idx]
        })
    }

    fn __setitem__(
        &mut self,
        idx: usize,
        bid: PyRef<'_, PyBlockId>,
    ) -> PyResult<()> {

        if idx >= 8 {
            return Err(PyErr::new::<pyo3::exceptions::PyIndexError, _>(
                "index out of range 0..7",
            ));
        }

        self.inner[idx] = bid.inner;

        Ok(())
    }

    fn __repr__(&self) -> String {
        format!("Children({:?})", self.inner)
    }
}

// ============================================================================
// 宏：自动生成 U8/U16/U32 绑定
// ============================================================================
/*
Py<PyList> 返回类型也升级了
以前：
list.into()
现在推荐：
list.unbind()
因为：
Bound<T> 是 GIL 生命周期绑定对象
Py<T> 是脱离 GIL 的 owned 对象
unbind() 是新 API。
*/
macro_rules! define_interner {

    ($rust_name:ident, $py_name:literal, $value_ty:ty) => {

        #[pyclass(module = "interner", name = $py_name)]
        pub struct $rust_name {
            pub inner: VoxInterner<$value_ty>,
        }

        #[pymethods]
        impl $rust_name {

            #[new]
            fn __new__() -> PyResult<Self> {
                Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    concat!(
                        "Use ",
                        $py_name,
                        ".with_memory_budget(budget)"
                    ),
                ))
            }

            #[staticmethod]
            #[pyo3(signature = (budget))]
            fn with_memory_budget(budget: usize) -> Self {

                Self {
                    inner: VoxInterner::<$value_ty>::with_memory_budget(budget),
                }
            }

            #[staticmethod]
            fn node_size() -> usize {
                VoxInterner::<$value_ty>::node_size()
            }

            fn capacity(&self) -> usize {
                self.inner.capacity()
            }

            fn patterns_empty(&self) -> bool {
                self.inner.patterns_empty()
            }

            // ====================================================================
            // BlockId
            // ====================================================================

            fn get_value(
                &self,
                block_id: PyRef<'_, PyBlockId>,
            ) -> $value_ty {

                *self.inner.get_value(&block_id.inner)
            }

            fn get_children(
                &self,
                block_id: PyRef<'_, PyBlockId>,
            ) -> PyChildren {

                PyChildren {
                    inner: self.inner.get_children(&block_id.inner)
                }
            }

            fn get_child_id(
                &self,
                block_id: PyRef<'_, PyBlockId>,
                index: usize,
            ) -> PyBlockId {

                PyBlockId {
                    inner: self.inner.get_child_id(
                        &block_id.inner,
                        index
                    )
                }
            }

            fn get_ref(
                &self,
                block_id: PyRef<'_, PyBlockId>,
            ) -> u32 {

                self.inner.get_ref(&block_id.inner)
            }

            fn inc_ref(
                &mut self,
                block_id: PyRef<'_, PyBlockId>,
            ) {

                self.inner.inc_ref(&block_id.inner);
            }

            fn dec_ref(
                &mut self,
                block_id: PyRef<'_, PyBlockId>,
            ) -> bool {

                self.inner.dec_ref(&block_id.inner)
            }

            fn inc_ref_by(
                &mut self,
                block_id: PyRef<'_, PyBlockId>,
                count: u32,
            ) {

                self.inner.inc_ref_by(&block_id.inner, count);
            }

            fn dec_ref_by(
                &mut self,
                block_id: PyRef<'_, PyBlockId>,
                count: u32,
            ) {

                self.inner.dec_ref_by(&block_id.inner, count);
            }

            fn dec_ref_recursive(
                &mut self,
                block_id: PyRef<'_, PyBlockId>,
            ) {

                self.inner.dec_ref_recursive(&block_id.inner);
            }

            // ====================================================================
            // Children
            // ====================================================================

            fn inc_child_refs(
                &mut self,
                children: PyRef<'_, PyChildren>,
                index: usize,
            ) {

                self.inner.inc_child_refs(
                    &children.inner,
                    index
                );
            }

            fn inc_all_child_refs(
                &mut self,
                children: PyRef<'_, PyChildren>,
            ) {

                self.inner.inc_all_child_refs(
                    &children.inner
                );
            }

            fn dec_child_refs(
                &mut self,
                children: PyRef<'_, PyChildren>,
            ) {

                self.inner.dec_child_refs(
                    &children.inner
                );
            }

            // ====================================================================
            // Node Creation
            // ====================================================================

            fn get_or_create_leaf(
                &mut self,
                value: $value_ty,
            ) -> PyBlockId {

                PyBlockId {
                    inner: self.inner.get_or_create_leaf(value)
                }
            }

            fn get_or_create_branch(
                &mut self,
                children: PyRef<'_, PyChildren>,
                types: u8,
                mask: u8,
            ) -> PyBlockId {

                PyBlockId {
                    inner: self.inner.get_or_create_branch(
                        children.inner,
                        types,
                        mask,
                    )
                }
            }

            fn create_empty_branch(&mut self) -> PyBlockId {

                PyBlockId {
                    inner: self.inner.create_empty_branch()
                }
            }

            fn create_branch(
                &mut self,
                children: PyRef<'_, PyChildren>,
                types: u8,
                mask: u8,
            ) -> PyBlockId {

                PyBlockId {
                    inner: self.inner.create_branch(
                        children.inner,
                        types,
                        mask,
                    )
                }
            }

            fn update_branch(
                &mut self,
                block_id: PyRef<'_, PyBlockId>,
                child_id: PyRef<'_, PyBlockId>,
                child_index: usize,
                types: u8,
                mask: u8,
            ) -> PyBlockId {

                PyBlockId {
                    inner: self.inner.update_branch(
                        &block_id.inner,
                        &child_id.inner,
                        child_index,
                        types,
                        mask,
                    )
                }
            }

            // ====================================================================
            // Deserialize
            // ====================================================================

            fn deserialize_leaf(
                &mut self,
                index: u32,
                value: $value_ty,
            ) -> PyBlockId {

                PyBlockId {
                    inner: self.inner.deserialize_leaf(index, value)
                }
            }

            fn deserialize_branch(
                &mut self,
                block_id: PyRef<'_, PyBlockId>,
                children: PyRef<'_, PyChildren>,
                types: u8,
                mask: u8,
                average: $value_ty,
            ) {

                self.inner.deserialize_branch(
                    block_id.inner,
                    children.inner,
                    types,
                    mask,
                    average,
                );
            }

            // ====================================================================
            // Validation
            // ====================================================================

            fn is_valid_block_id(
                &self,
                block_id: PyRef<'_, PyBlockId>,
            ) -> bool {

                self.inner.is_valid_block_id(&block_id.inner)
            }

            fn ensure_valid_children(
                &self,
                children: PyRef<'_, PyChildren>,
            ) {

                self.inner.ensure_valid_children(
                    &children.inner
                );
            }

            fn count_nodes(
                &self,
                block_id: PyRef<'_, PyBlockId>,
            ) -> u32 {

                self.inner.count_nodes(block_id.inner)
            }

            // ====================================================================
            // Stats
            // ====================================================================

            #[cfg(feature = "memory_stats")]
            fn stats(&self) -> PyInternerStats {
                PyInternerStats(self.inner.stats())
            }
        }
    };
}

// ============================================================================
// Generate Types
// ============================================================================

define_interner!(PyVoxInternerU8,  "VoxInternerU8",  u8);
define_interner!(PyVoxInternerU16, "VoxInternerU16", u16);
define_interner!(PyVoxInternerU32, "VoxInternerU32", u32);




// ============================================================================
// Thread Safe Interner (I32)
// ============================================================================
use std::sync::Arc;
use parking_lot::RwLock;

#[pyclass(
    skip_from_py_object,
    weakref,
    module = "interner",
    name = "VoxInternerI32"
)]
#[derive(Clone)]
pub struct PyVoxInternerI32 {
    pub inner: Arc<RwLock<VoxInterner<i32>>>,
}

#[pymethods]
impl PyVoxInternerI32 {

    #[new]
    fn __new__() -> PyResult<Self> {

        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Use model.get_interner()",
        ))
    }

    #[staticmethod]
    fn with_memory_budget(budget: usize) -> Self {

        Self {
            inner: Arc::new(
                RwLock::new(
                    VoxInterner::<i32>::with_memory_budget(budget)
                )
            )
        }
    }

    // ========================================================================
    // Static
    // ========================================================================

    #[staticmethod]
    fn node_size() -> usize {
        VoxInterner::<i32>::node_size()
    }

    // ========================================================================
    // Info
    // ========================================================================

    fn capacity(&self) -> usize {
        self.inner.read().capacity()
    }

    fn patterns_empty(&self) -> bool {
        self.inner.read().patterns_empty()
    }

    // ========================================================================
    // Value
    // ========================================================================

    fn get_value(
        &self,
        block_id: PyRef<'_, PyBlockId>,
    ) -> i32 {

        let guard = self.inner.read();

        *guard.get_value(&block_id.inner)
    }

    fn get_children(
        &self,
        block_id: PyRef<'_, PyBlockId>,
    ) -> PyChildren {

        let guard = self.inner.read();

        PyChildren {
            inner: guard.get_children(&block_id.inner)
        }
    }

    fn get_child_id(
        &self,
        block_id: PyRef<'_, PyBlockId>,
        index: usize,
    ) -> PyBlockId {

        let guard = self.inner.read();

        PyBlockId {
            inner: guard.get_child_id(
                &block_id.inner,
                index,
            )
        }
    }

    fn get_ref(
        &self,
        block_id: PyRef<'_, PyBlockId>,
    ) -> u32 {

        self.inner
            .read()
            .get_ref(&block_id.inner)
    }

    // ========================================================================
    // Ref Count
    // ========================================================================

    fn inc_ref(
        &self,
        block_id: PyRef<'_, PyBlockId>,
    ) {

        self.inner
            .write()
            .inc_ref(&block_id.inner);
    }

    fn dec_ref(
        &self,
        block_id: PyRef<'_, PyBlockId>,
    ) -> bool {

        self.inner
            .write()
            .dec_ref(&block_id.inner)
    }

    fn inc_ref_by(
        &self,
        block_id: PyRef<'_, PyBlockId>,
        count: u32,
    ) {

        self.inner
            .write()
            .inc_ref_by(
                &block_id.inner,
                count,
            );
    }

    fn dec_ref_by(
        &self,
        block_id: PyRef<'_, PyBlockId>,
        count: u32,
    ) {

        self.inner
            .write()
            .dec_ref_by(
                &block_id.inner,
                count,
            );
    }

    fn dec_ref_recursive(
        &self,
        block_id: PyRef<'_, PyBlockId>,
    ) {

        self.inner
            .write()
            .dec_ref_recursive(
                &block_id.inner,
            );
    }

    // ========================================================================
    // Child Ref
    // ========================================================================

    fn inc_child_refs(
        &self,
        children: PyRef<'_, PyChildren>,
        index: usize,
    ) {

        self.inner
            .write()
            .inc_child_refs(
                &children.inner,
                index,
            );
    }

    fn inc_all_child_refs(
        &self,
        children: PyRef<'_, PyChildren>,
    ) {

        self.inner
            .write()
            .inc_all_child_refs(
                &children.inner,
            );
    }

    fn dec_child_refs(
        &self,
        children: PyRef<'_, PyChildren>,
    ) {

        self.inner
            .write()
            .dec_child_refs(
                &children.inner,
            );
    }

    // ========================================================================
    // Node Create
    // ========================================================================

    fn get_or_create_leaf(
        &self,
        value: i32,
    ) -> PyBlockId {

        let mut guard = self.inner.write();

        PyBlockId {
            inner: guard.get_or_create_leaf(value)
        }
    }

    fn get_or_create_branch(
        &self,
        children: PyRef<'_, PyChildren>,
        types: u8,
        mask: u8,
    ) -> PyBlockId {

        let mut guard = self.inner.write();

        PyBlockId {
            inner: guard.get_or_create_branch(
                children.inner,
                types,
                mask,
            )
        }
    }

    fn create_empty_branch(&self) -> PyBlockId {

        let mut guard = self.inner.write();

        PyBlockId {
            inner: guard.create_empty_branch()
        }
    }

    fn create_branch(
        &self,
        children: PyRef<'_, PyChildren>,
        types: u8,
        mask: u8,
    ) -> PyBlockId {

        let mut guard = self.inner.write();

        PyBlockId {
            inner: guard.create_branch(
                children.inner,
                types,
                mask,
            )
        }
    }

    fn update_branch(
        &self,
        block_id: PyRef<'_, PyBlockId>,
        child_id: PyRef<'_, PyBlockId>,
        child_index: usize,
        types: u8,
        mask: u8,
    ) -> PyBlockId {

        let mut guard = self.inner.write();

        PyBlockId {
            inner: guard.update_branch(
                &block_id.inner,
                &child_id.inner,
                child_index,
                types,
                mask,
            )
        }
    }

    // ========================================================================
    // Deserialize
    // ========================================================================

    fn deserialize_leaf(
        &self,
        index: u32,
        value: i32,
    ) -> PyBlockId {

        let mut guard = self.inner.write();

        PyBlockId {
            inner: guard.deserialize_leaf(
                index,
                value,
            )
        }
    }

    fn deserialize_branch(
        &self,
        block_id: PyRef<'_, PyBlockId>,
        children: PyRef<'_, PyChildren>,
        types: u8,
        mask: u8,
        average: i32,
    ) {

        self.inner
            .write()
            .deserialize_branch(
                block_id.inner,
                children.inner,
                types,
                mask,
                average,
            );
    }

    // ========================================================================
    // Validation
    // ========================================================================

    fn is_valid_block_id(
        &self,
        block_id: PyRef<'_, PyBlockId>,
    ) -> bool {

        self.inner
            .read()
            .is_valid_block_id(
                &block_id.inner
            )
    }

    fn ensure_valid_children(
        &self,
        children: PyRef<'_, PyChildren>,
    ) {

        self.inner
            .read()
            .ensure_valid_children(
                &children.inner
            );
    }

    fn count_nodes(
        &self,
        block_id: PyRef<'_, PyBlockId>,
    ) -> u32 {

        self.inner
            .read()
            .count_nodes(
                block_id.inner
            )
    }

    // ========================================================================
    // Stats
    // ========================================================================

    #[cfg(feature = "memory_stats")]
    fn stats(&self) -> PyInternerStats {

        PyInternerStats(
            self.inner
                .read()
                .stats()
        )
    }

    //Python 显式共享
    fn clone_handle(&self) -> Self {
        Self {inner: self.inner.clone()
        }
    }
}