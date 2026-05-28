use pyo3::prelude::*;
use glam::IVec3;


use super::super::core::{PyBlockId, PyTraversalDepth};
use super::super::{
    PyVoxInternerU8,
    PyVoxInternerU16,
    PyVoxInternerU32,
    //PyVoxInternerI32,
};

use voxelis::utils::common;
use voxelis::MaxDepth;

// -----------------------------------------------------------------------------
// Static utility class
// -----------------------------------------------------------------------------
#[pyclass(module = "utils", name = "Common", skip_from_py_object)]
#[derive(Clone, Copy)]
pub struct PyCommon;

#[pymethods]
impl PyCommon {
    // ==========================================
    // General utility functions
    // ==========================================
    #[staticmethod]
    pub fn child_index(x: i32, y: i32, z: i32, depth: &PyTraversalDepth) -> usize {
        let pos = IVec3::new(x, y, z);
        common::child_index(&pos, &depth.inner)
    }

    #[staticmethod]
    pub fn child_index2(x: i32, y: i32, z: i32, current: usize, max: usize) -> usize {
        let pos = IVec3::new(x, y, z);
        common::child_index2(&pos, current, max)
    }

    #[staticmethod]
    pub fn encode_child_index_path(x: i32, y: i32, z: i32) -> u32 {
        let pos = IVec3::new(x, y, z);
        common::encode_child_index_path(&pos)
    }

    // ==========================================
    // U8 
    // ==========================================
    #[staticmethod]
    pub fn get_at_depth_u8(
        interner: &PyVoxInternerU8,
        node_id: &PyBlockId,
        x: i32, y: i32, z: i32,
        depth: &PyTraversalDepth,
    ) -> Option<u8> {
        let pos = IVec3::new(x, y, z);
        common::get_at_depth(&interner.inner, node_id.inner, &pos, &depth.inner)
    }

    #[staticmethod]
    pub fn to_vec_u8(
        _py: Python,
        interner: &PyVoxInternerU8,
        root_id: &PyBlockId,
        max_depth: u8,
    ) -> PyResult<Vec<u8>> {
        let max_depth = MaxDepth::new(max_depth);
        let rust_vec = common::to_vec(&interner.inner, &root_id.inner, max_depth);
        Ok(rust_vec)
    }

    #[staticmethod]
    pub fn dump_structure_u8(
        interner: &PyVoxInternerU8,
        root_id: &PyBlockId,
        max_depth: usize
    ) {
        common::dump_structure(&interner.inner, root_id.inner, max_depth);
    }

    #[staticmethod]
    pub fn dump_root_u8(
        interner: &PyVoxInternerU8,
        root_id: &PyBlockId
    ) {
        common::dump_root(&interner.inner, root_id.inner);
    }

    #[staticmethod]
    pub fn dump_statistics_u8(
        interner: &PyVoxInternerU8,
        root_id: &PyBlockId
    ) {
        common::dump_statistics(&interner.inner, root_id.inner);
    }

    // ==========================================
    // U16 
    // ==========================================
    #[staticmethod]
    pub fn get_at_depth_u16(
        interner: &PyVoxInternerU16,
        node_id: &PyBlockId,
        x: i32, y: i32, z: i32,
        depth: &PyTraversalDepth,
    ) -> Option<u16> {
        let pos = IVec3::new(x, y, z);
        common::get_at_depth(&interner.inner, node_id.inner, &pos, &depth.inner)
    }

    #[staticmethod]
    pub fn to_vec_u16(
        _py: Python,
        interner: &PyVoxInternerU16,
        root_id: &PyBlockId,
        max_depth: u8,
    ) -> PyResult<Vec<u16>> {
        let max_depth = MaxDepth::new(max_depth);
        let rust_vec = common::to_vec(&interner.inner, &root_id.inner, max_depth);
        Ok(rust_vec)
    }

    #[staticmethod]
    pub fn dump_structure_u16(
        interner: &PyVoxInternerU16,
        root_id: &PyBlockId,
        max_depth: usize
    ) {
        common::dump_structure(&interner.inner, root_id.inner, max_depth);
    }

    #[staticmethod]
    pub fn dump_root_u16(
        interner: &PyVoxInternerU16,
        root_id: &PyBlockId
    ) {
        common::dump_root(&interner.inner, root_id.inner);
    }

    #[staticmethod]
    pub fn dump_statistics_u16(
        interner: &PyVoxInternerU16,
        root_id: &PyBlockId
    ) {
        common::dump_statistics(&interner.inner, root_id.inner);
    }

    // ==========================================
    // U32 
    // ==========================================
    #[staticmethod]
    pub fn get_at_depth_u32(
        interner: &PyVoxInternerU32,
        node_id: &PyBlockId,
        x: i32, y: i32, z: i32,
        depth: &PyTraversalDepth,
    ) -> Option<u32> {
        let pos = IVec3::new(x, y, z);
        common::get_at_depth(&interner.inner, node_id.inner, &pos, &depth.inner)
    }

    #[staticmethod]
    pub fn to_vec_u32(
        _py: Python,
        interner: &PyVoxInternerU32,
        root_id: &PyBlockId,
        max_depth: u8,
    ) -> PyResult<Vec<u32>> {
        let max_depth = MaxDepth::new(max_depth);
        let rust_vec = common::to_vec(&interner.inner, &root_id.inner, max_depth);
        Ok(rust_vec)
    }

    #[staticmethod]
    pub fn dump_structure_u32(
        interner: &PyVoxInternerU32,
        root_id: &PyBlockId,
        max_depth: usize
    ) {
        common::dump_structure(&interner.inner, root_id.inner, max_depth);
    }

    #[staticmethod]
    pub fn dump_root_u32(
        interner: &PyVoxInternerU32,
        root_id: &PyBlockId
    ) {
        common::dump_root(&interner.inner, root_id.inner);
    }

    #[staticmethod]
    pub fn dump_statistics_u32(
        interner: &PyVoxInternerU32,
        root_id: &PyBlockId
    ) {
        common::dump_statistics(&interner.inner, root_id.inner);
    }
}

// -----------------------------------------------------------------------------
// I32
// -----------------------------------------------------------------------------
// #[pyfunction]
// pub fn get_at_depth_i32(
//     interner: &PyVoxInternerI32,
//     node_id: &PyBlockId,
//     x: i32, y: i32, z: i32,
//     depth: &PyTraversalDepth,
// ) -> Option<i32> {
//     let guard = interner.inner.read();
//     let pos = IVec3::new(x, y, z);
//     common::get_at_depth(&guard, node_id.inner, &pos, &depth.inner)
// }

// #[pyfunction]
// pub fn to_vec_i32(
//     _py: Python,
//     interner: &PyVoxInternerI32,
//     root_id: &PyBlockId,
//     max_depth: u8,
// ) -> PyResult<Vec<i32>> {
//     let max_depth = MaxDepth::new(max_depth);
//     let guard = interner.inner.read();
//     let rust_vec = common::to_vec(&guard, &root_id.inner, max_depth);
//     Ok(rust_vec)
// }

// #[pyfunction]
// pub fn dump_structure_i32(
//     interner: &PyVoxInternerI32,
//     root_id: &PyBlockId,
//     max_depth: usize
// ) {
//     let guard = interner.inner.read();
//     common::dump_structure(&guard, root_id.inner, max_depth);
// }

// #[pyfunction]
// pub fn dump_root_i32(
//     interner: &PyVoxInternerI32,
//     root_id: &PyBlockId
// ) {
//     let guard = interner.inner.read();
//     common::dump_root(&guard, root_id.inner);
// }

// #[pyfunction]
// pub fn dump_statistics_i32(
//     interner: &PyVoxInternerI32,
//     root_id: &PyBlockId
// ) {
//     let guard = interner.inner.read();
//     common::dump_statistics(&guard, root_id.inner);
// }