use pyo3::prelude::*;
use voxelis::{Batch, MaxDepth};
use voxelis::spatial::VoxOpsWrite;
use glam::IVec3;

// ------------------------------
// 1. Universal Batch Macro(u8/u16/u32)
// ------------------------------
macro_rules! define_batch {
    ($rust_name:ident, $py_name:literal, $interner_ty:ty, $value_ty:ty) => {
        #[pyclass(module = "core", name = $py_name)]
        pub struct $rust_name {
            pub inner: Batch<$value_ty>,
        }

        #[pymethods]
        impl $rust_name {
            #[new]
            pub fn new(max_depth: u8) -> Self {
                Self {
                    inner: Batch::new(MaxDepth::new(max_depth)),
                }
            }

            pub fn fill(&mut self, interner: &mut $interner_ty, value: $value_ty) {
                self.inner.fill(&mut interner.inner, value);
            }

            pub fn set(&mut self, interner: &mut $interner_ty, x: i32, y: i32, z: i32, value: $value_ty) {
                self.inner.set(&mut interner.inner, IVec3::new(x, y, z), value);
            }

            pub fn clear(&mut self, interner: &mut $interner_ty) {
                self.inner.clear(&mut interner.inner);
            }

            pub fn just_set(&mut self, x: i32, y: i32, z: i32, value: $value_ty) -> bool {
                self.inner.just_set(IVec3::new(x, y, z), value)
            }

            pub fn size(&self) -> usize {
                self.inner.size()
            }

            pub fn has_patches(&self) -> bool {
                self.inner.has_patches()
            }

            pub fn to_fill(&self) -> Option<$value_ty> {
                self.inner.to_fill()
            }
        }
    };
}

// ------------------------------
// 2. 生成 u8/u16/u32
// ------------------------------
use super::super::PyVoxInternerU8;
use super::super::PyVoxInternerU16;
use super::super::PyVoxInternerU32;

define_batch!(PyBatchU8,  "BatchU8",  PyVoxInternerU8,  u8);
define_batch!(PyBatchU16, "BatchU16", PyVoxInternerU16, u16);
define_batch!(PyBatchU32, "BatchU32", PyVoxInternerU32, u32);

// ------------------------------
// 3.  I32 independently implemented with Arc<RwLock> support and private constructor.
// ------------------------------
use crate::voxelis_bindings::PyVoxInternerI32;

#[pyclass(module = "core", name = "BatchI32")]
pub struct PyBatchI32 {
    pub inner: Batch<i32>,
}

#[pymethods]
impl PyBatchI32 {
    // Python 公开构造
    #[new]
    pub fn new(max_depth: u8) -> Self {
        Self {
            inner: Batch::new(MaxDepth::new(max_depth)),
        }
    }

    // 适配 Arc<RwLock> 线程安全 interner
    pub fn fill(&mut self, interner: &PyVoxInternerI32, value: i32) {
        let mut guard = interner.inner.write();
        self.inner.fill(&mut *guard, value);
    }

    pub fn set(&mut self, interner: &PyVoxInternerI32, x: i32, y: i32, z: i32, value: i32) {
        let mut guard = interner.inner.write();
        self.inner.set(&mut *guard, IVec3::new(x, y, z), value);
    }

    pub fn clear(&mut self, interner: &PyVoxInternerI32) {
        let mut guard = interner.inner.write();
        self.inner.clear(&mut *guard);
    }

    // 通用方法
    pub fn just_set(&mut self, x: i32, y: i32, z: i32, value: i32) -> bool {
        self.inner.just_set(IVec3::new(x, y, z), value)
    }

    pub fn size(&self) -> usize {
        self.inner.size()
    }

    pub fn has_patches(&self) -> bool {
        self.inner.has_patches()
    }

    pub fn to_fill(&self) -> Option<i32> {
        self.inner.to_fill()
    }
}

// ------------------------------
// 4. For internal Rust use only: Constructor not exposed to Python (used for returning wrapped values).
// ------------------------------
impl PyBatchI32 {
    #[allow(dead_code)]
    pub fn wrap(inner: Batch<i32>) -> Self {
        Self { inner }
    }
}