//! Trimesh and Convex Pin
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Pin
#[derive(Debug, Clone)]
pub struct Pin<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Pin
impl<F: Float> TBridgeGlobal for Pin<F> {
  /// constructor
  fn void() -> Self {
    Pin::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Pin
impl<F: Float + std::fmt::Debug> Pin<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, p: u16, q: u16, tf: bool) {
    self.ph.from_polyhedron(&ph_faces::pin::Pin::new(r, p, q).ph, tf);
  }
}
