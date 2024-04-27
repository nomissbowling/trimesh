//! Trimesh and Convex Tetra
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Tetra
#[derive(Debug, Clone)]
pub struct Tetra<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Tetra
impl<F: Float> TBridgeGlobal for Tetra<F> {
  /// constructor
  fn void() -> Self {
    Tetra::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Tetra
impl<F: Float + std::fmt::Debug> Tetra<F> {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, tf: bool) {
    self.ph.from_phf(&fullerene::ph_faces::tetra::Tetra::new(r).ph.with_uv(tf));
  }
}
