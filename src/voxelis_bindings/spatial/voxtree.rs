use pyo3::prelude::*;

use voxelis::spatial::VoxTree;
use glam::IVec3;

use voxelis::spatial::VoxOpsRead;
use voxelis::spatial::VoxOpsWrite;
use voxelis::spatial::VoxOpsBatch;
use voxelis::spatial::VoxOpsConfig;
use voxelis::spatial::VoxOpsState;
use voxelis::spatial::VoxOpsDirty;
use voxelis::spatial::VoxOpsMesh;

use super::super::PyBlockId;

use crate::{
    PyVoxInternerU8,
    PyVoxInternerU16,
    PyVoxInternerU32,
    PyBatchU8,
    PyBatchU16,
    PyBatchU32,
};

macro_rules! define_vox_tree {
    ($rust_name:ident, $py_name:literal, $batch_name:ident, $interner_ty:ty, $value_ty:ty) => {
        #[pyclass(module = "spatial", name = $py_name)]
        pub struct $rust_name {
            pub inner: VoxTree,
        }

        #[pymethods]
        impl $rust_name {
            #[new]
            pub fn new(max_depth: u8) -> Self {
                Self {
                    inner: VoxTree::new(voxelis::MaxDepth::new(max_depth)),
                }
            }

            pub fn create_batch(&self) -> $batch_name {
                $batch_name {
                    inner: self.inner.create_batch()
                }
            }

            pub fn apply_batch(
                &mut self,
                interner: &mut $interner_ty,
                batch: &$batch_name,
            ) -> PyResult<()> {
                self.inner.apply_batch(&mut interner.inner, &batch.inner);
                Ok(())
            }

            pub fn get(
                &self,
                interner: &$interner_ty,
                x: i32, y: i32, z: i32
            ) -> Option<$value_ty> {
                self.inner.get(&interner.inner, IVec3::new(x, y, z))
            }

            pub fn set(
                &mut self,
                interner: &mut $interner_ty,
                x: i32, y: i32, z: i32,
                value: $value_ty
            ) -> bool {
                self.inner.set(&mut interner.inner, IVec3::new(x, y, z), value)
            }

            pub fn fill(&mut self, interner: &mut $interner_ty, value: $value_ty) {
                self.inner.fill(&mut interner.inner, value);
            }

            pub fn clear(&mut self, interner: &mut $interner_ty) {
                self.inner.clear(&mut interner.inner);
            }


            // =========================================
            // Config
            // =========================================

            pub fn max_depth(&self, lod: u8) -> u8 {
                self.inner
                    .max_depth(voxelis::Lod::new(lod))
                    .max()
            }

            pub fn voxels_per_axis(&self, lod: u8) -> u32 {
                self.inner
                    .voxels_per_axis(voxelis::Lod::new(lod))
            }

            // =========================================
            // State
            // =========================================

            pub fn is_empty(&self) -> bool {
                self.inner.is_empty()
            }

            pub fn is_leaf(&self) -> bool {
                self.inner.is_leaf()
            }

            // =========================================
            // Dirty
            // =========================================

            pub fn is_dirty(&self) -> bool {
                self.inner.is_dirty()
            }

            pub fn mark_dirty(&mut self) {
                self.inner.mark_dirty();
            }

            pub fn clear_dirty(&mut self) {
                self.inner.clear_dirty();
            }

            // =========================================
            // Root
            // =========================================

            pub fn get_root_id(&self) -> PyBlockId {
                PyBlockId {
                    inner: self.inner.get_root_id()
                }
            }
            //safe
            pub fn set_root_id(
                &mut self,
                interner: &mut $interner_ty,
                root_id: PyRef<'_, PyBlockId>,
            ) {
                self.inner.set_root_id(
                    &mut interner.inner,
                    root_id.inner,
                );
            }
            // =========================================
            // Mesh
            // =========================================

            pub fn to_vec(
                &self,
                interner: &$interner_ty,
                lod: u8,
            ) -> Vec<$value_ty> {

                self.inner.to_vec(
                    &interner.inner,
                    voxelis::Lod::new(lod),
                )
            }

        }
    };
}

define_vox_tree!(PyVoxTreeU8,  "VoxTreeU8",  PyBatchU8,  PyVoxInternerU8,  u8);
define_vox_tree!(PyVoxTreeU16, "VoxTreeU16", PyBatchU16, PyVoxInternerU16, u16);
define_vox_tree!(PyVoxTreeU32, "VoxTreeU32", PyBatchU32, PyVoxInternerU32, u32);