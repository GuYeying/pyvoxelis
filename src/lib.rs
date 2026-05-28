use pyo3::prelude::*;
use pyo3::types::PyDict;

mod voxelis_bindings;

// Core
use voxelis_bindings::core::{
    PyBatchU8, PyBatchU16, PyBatchU32,
    PyBlockId, PyLod, PyMaxDepth, PyTraversalDepth,
};

// Interner
use voxelis_bindings::interner::{
    PyInternerConstants, PyPatternsHashmap,
    PyVoxInternerU8, PyVoxInternerU16, PyVoxInternerU32, PyVoxInternerI32
};
#[cfg(feature = "memory_stats")]
use voxelis_bindings::interner::PyInternerStats;

// IO
use voxelis_bindings::io::consts::PyIOConstants;
use voxelis_bindings::io::export::PyExport;
use voxelis_bindings::io::import::PyImport;
use voxelis_bindings::io::flags::PyFlags;
use voxelis_bindings::io::obj_reader::PyObj;
use voxelis_bindings::io::varints::PyVarint;

// Spatial
use voxelis_bindings::spatial::{PyVoxTreeU8, PyVoxTreeU16, PyVoxTreeU32, PyAabb2d};

// Utils
use voxelis_bindings::utils::common::PyCommon;

// World
use voxelis_bindings::world::{PyVoxChunk, PyVoxModel, PyVoxWorld};


#[pymodule]
fn pyvoxelis(py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    // get sys.modules
    let sys = py.import("sys")?;
    let modules: Bound<PyDict> = Bound::cast_into(sys.getattr("modules")?)?;

    // ============================
    // 1. core child module
    // ============================
    let core = PyModule::new(py, "pyvoxelis.core")?;
    core.add_class::<PyBlockId>()?;
    core.add_class::<PyLod>()?;
    core.add_class::<PyMaxDepth>()?;
    core.add_class::<PyTraversalDepth>()?;
    core.add_class::<PyBatchU8>()?;
    core.add_class::<PyBatchU16>()?;
    core.add_class::<PyBatchU32>()?;
    modules.set_item("pyvoxelis.core", &core)?;
    module.add_submodule(&core)?;

    // ============================
    // 2. interner child module
    // ============================
    let interner = PyModule::new(py, "pyvoxelis.interner")?;
    interner.add_class::<PyInternerConstants>()?;
    interner.add_class::<PyPatternsHashmap>()?;
    interner.add_class::<PyVoxInternerU8>()?;
    interner.add_class::<PyVoxInternerU16>()?;
    interner.add_class::<PyVoxInternerU32>()?;
    interner.add_class::<PyVoxInternerI32>()?;
    #[cfg(feature = "memory_stats")]
    interner.add_class::<PyInternerStats>()?;
    modules.set_item("pyvoxelis.interner", &interner)?;
    module.add_submodule(&interner)?;

    // ============================
    // 3. io child module
    // ============================
    let io = PyModule::new(py, "pyvoxelis.io")?;
    io.add_class::<PyImport>()?;
    io.add_class::<PyExport>()?;
    io.add_class::<PyVarint>()?;
    io.add_class::<PyFlags>()?;
    io.add_class::<PyObj>()?;
    io.add_class::<PyIOConstants>()?;
    modules.set_item("pyvoxelis.io", &io)?;
    module.add_submodule(&io)?;

    // ============================
    // 4. spatial child module
    // ============================
    let spatial = PyModule::new(py, "pyvoxelis.spatial")?;
    spatial.add_class::<PyVoxTreeU8>()?;
    spatial.add_class::<PyVoxTreeU16>()?;
    spatial.add_class::<PyVoxTreeU32>()?;
    spatial.add_class::<PyAabb2d>()?;
    modules.set_item("pyvoxelis.spatial", &spatial)?;
    module.add_submodule(&spatial)?;

    // ============================
    // 5. utils child module
    // ============================
    let utils = PyModule::new(py, "pyvoxelis.utils")?;
    utils.add_class::<PyCommon>()?;
    modules.set_item("pyvoxelis.utils", &utils)?;
    module.add_submodule(&utils)?;

    // ============================
    // 6. world child module
    // ============================
    let world = PyModule::new(py, "pyvoxelis.world")?;
    world.add_class::<PyVoxChunk>()?;
    world.add_class::<PyVoxModel>()?;
    world.add_class::<PyVoxWorld>()?;
    modules.set_item("pyvoxelis.world", &world)?;
    module.add_submodule(&world)?;

    Ok(())
}