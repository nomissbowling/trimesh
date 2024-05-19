//! Trimesh and Convex Octa
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Octa
#[derive(Debug, Clone)]
pub struct Octa<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Octa
impl<F: Float> TBridgeGlobal for Octa<F> {
  /// constructor
  fn void() -> Self {
    Octa::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Octa
impl<F: Float + std::fmt::Debug> Octa<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, tf: bool) {
    self.ph.from_phf(&ph_faces::octa::Octa::new(r).ph.with_uv(tf));
  }
}
