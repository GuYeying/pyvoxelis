use pyo3::prelude::*;
use glam::Vec2;
use voxelis::spatial::Aabb2d;

#[pyclass(module = "spatial", name = "Aabb2d")]
pub struct PyAabb2d(pub Aabb2d);

#[pymethods]
impl PyAabb2d {
    #[staticmethod]
    pub fn with_min_max(min: (f32, f32), max: (f32, f32)) -> Self {
        Self(Aabb2d::with_min_max(
            Vec2::new(min.0, min.1),
            Vec2::new(max.0, max.1),
        ))
    }

    #[staticmethod]
    pub fn with_position_and_size(pos: (f32, f32), size: (f32, f32)) -> Self {
        Self(Aabb2d::with_position_and_size(
            Vec2::new(pos.0, pos.1),
            Vec2::new(size.0, size.1),
        ))
    }

    #[getter]
    pub fn min(&self) -> (f32, f32) {
        (self.0.min.x, self.0.min.y)
    }

    #[getter]
    pub fn max(&self) -> (f32, f32) {
        (self.0.max.x, self.0.max.y)
    }

    pub fn size(&self) -> (f32, f32) {
        let s = self.0.size();
        (s.x, s.y)
    }

    pub fn union(&self, other: &PyAabb2d) -> Self {
        Self(self.0.union(&other.0))
    }

    pub fn contains(&self, point: (f32, f32)) -> bool {
        self.0.contains(Vec2::new(point.0, point.1))
    }

    pub fn intersects(&self, other: &PyAabb2d) -> bool {
        self.0.intersects(&other.0)
    }

    fn __repr__(&self) -> String {
        format!(
            "Aabb2d(min=({}, {}), max=({}, {}))",
            self.0.min.x, self.0.min.y,
            self.0.max.x, self.0.max.y
        )
    }
}