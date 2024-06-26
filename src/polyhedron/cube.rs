//! Trimesh and Convex Cube
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Cuboid
#[derive(Debug, Clone)]
pub struct Cuboid<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Cuboid
impl<F: Float> TBridgeGlobal for Cuboid<F> {
  /// constructor
  fn void() -> Self {
    Cuboid::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Cuboid
impl<F: Float + std::fmt::Debug> Cuboid<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, lxyz: [F; 3], tf: bool) {
    self.ph.from_polyhedron(&ph_faces::cube::Cuboid::new(lxyz).ph, tf);
  }
}

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
impl<F: Float + std::fmt::Debug> Cube<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, tf: bool) {
    self.ph.from_polyhedron(&ph_faces::cube::Cube::new(r).ph, tf);
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
impl<F: Float + std::fmt::Debug> CubeCenter<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, tf: bool) {
    self.ph.from_polyhedron(&ph_faces::cube::CubeCenter::new(r).ph, tf);
  }
}
