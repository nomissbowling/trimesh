//! Trimesh and Convex Cone
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Cone
#[derive(Debug, Clone)]
pub struct Cone<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Cone
impl<F: Float> TBridgeGlobal for Cone<F> {
  /// constructor
  fn void() -> Self {
    Cone::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Cone
impl<F: Float + std::fmt::Debug> Cone<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, h: F, q: u16, tf: bool) {
    self.ph.from_polyhedron(&ph_faces::cone::Cone::new(r, h, q).ph, tf);
  }
}
