//! Trimesh and Convex Cube
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Cube
#[derive(Debug, Clone)]
pub struct Cube<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Cube
impl<F: Float> TBridgeGlobal for Cube<F> {
  /// constructor
  fn void() -> Self {
    Cube::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Cube
impl<F: Float + std::fmt::Debug> Cube<F> {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, tf: bool) {
    self.ph.from_phf(&ph_faces::cube::Cube::new(r).ph.with_uv(tf));
  }
}

/// CubeCenter
#[derive(Debug, Clone)]
pub struct CubeCenter<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for CubeCenter
impl<F: Float> TBridgeGlobal for CubeCenter<F> {
  /// constructor
  fn void() -> Self {
    CubeCenter::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// CubeCenter
impl<F: Float + std::fmt::Debug> CubeCenter<F> {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, tf: bool) {
    self.ph.from_phf(&ph_faces::cube::CubeCenter::new(r).ph.with_uv(tf));
  }
}
