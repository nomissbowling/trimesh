//! Trimesh and Convex Sphere
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// RSphere
#[derive(Debug, Clone)]
pub struct RSphere<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for RSphere
impl<F: Float> TBridgeGlobal for RSphere<F> {
  /// constructor
  fn void() -> Self {
    RSphere::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// RSphere
impl<F: Float + std::fmt::Debug> RSphere<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, q: u16, tf: bool) {
    self.ph.from_phf(&ph_faces::sphere::RSphere::new(r, q).ph.with_uv(tf));
  }
}
