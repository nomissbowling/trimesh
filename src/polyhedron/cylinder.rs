//! Trimesh and Convex Cylinder
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Cylinder
#[derive(Debug, Clone)]
pub struct Cylinder<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Cylinder
impl<F: Float> TBridgeGlobal for Cylinder<F> {
  /// constructor
  fn void() -> Self {
    Cylinder::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Cylinder
impl<F: Float + std::fmt::Debug> Cylinder<F> {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, l: F, q: u16, tf: bool) {
    self.ph.from_phf(&ph_faces::cylinder::Cylinder::new(r, l, q).ph.with_uv(tf));
  }
}
