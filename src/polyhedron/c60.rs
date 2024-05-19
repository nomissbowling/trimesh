//! Trimesh and Convex C60 from Fullerene
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// C60
#[derive(Debug, Clone)]
pub struct C60<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for C60
impl<F: Float> TBridgeGlobal for C60<F> {
  /// constructor
  fn void() -> Self {
    C60::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// C60
impl<F: Float + std::fmt::Debug> C60<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, tf: bool) {
    self.ph.from_phf(&fullerene::C60::new(r).ph.with_uv(tf));
  }
}

/// C60Center
#[derive(Debug, Clone)]
pub struct C60Center<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for C60Center
impl<F: Float> TBridgeGlobal for C60Center<F> {
  /// constructor
  fn void() -> Self {
    C60Center::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// C60Center
impl<F: Float + std::fmt::Debug> C60Center<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, tf: bool) {
    self.ph.from_phf(&fullerene::C60Center::new(r).ph.with_uv(tf));
  }
}
