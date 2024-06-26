//! Trimesh and Convex Capsule
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Capsule
#[derive(Debug, Clone)]
pub struct Capsule<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Capsule
impl<F: Float> TBridgeGlobal for Capsule<F> {
  /// constructor
  fn void() -> Self {
    Capsule::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Capsule
impl<F: Float + std::fmt::Debug> Capsule<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, l: F, q: u16, tf: bool) {
    self.ph.from_polyhedron(&ph_faces::capsule::Capsule::new(r, l, q).ph, tf);
  }
}
