//! Trimesh and Convex Torus
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Torus
#[derive(Debug, Clone)]
pub struct Torus<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Torus
impl<F: Float> TBridgeGlobal for Torus<F> {
  /// constructor
  fn void() -> Self {
    Torus::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Torus
impl<F: Float + std::fmt::Debug + std::iter::Sum> Torus<F> {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, c: F, r: F, p: u16, q: u16, tf: bool) {
    self.ph.from_phf(&ph_faces::torus::Torus::new(c, r, p, q).ph.with_uv(tf));
  }
}

/// RTorus
#[derive(Debug, Clone)]
pub struct RTorus<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for RTorus
impl<F: Float> TBridgeGlobal for RTorus<F> {
  /// constructor
  fn void() -> Self {
    RTorus::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// RTorus
impl<F: Float + std::fmt::Debug + std::iter::Sum> RTorus<F> {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, c: F, r: F, p: u16, q: u16, tf: bool) {
    self.ph.from_phf(&ph_faces::torus::RTorus::new(c, r, p, q).ph.with_uv(tf));
  }
}

/// Ring
#[derive(Debug, Clone)]
pub struct Ring<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Ring
impl<F: Float> TBridgeGlobal for Ring<F> {
  /// constructor
  fn void() -> Self {
    Ring::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Ring
impl<F: Float + std::fmt::Debug + std::iter::Sum> Ring<F> {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, c: F, d: F, e: F, p: u16, q: u16, tf: bool) {
    self.ph.from_phf(&ph_faces::torus::Ring::new(c, d, e, p, q).ph.with_uv(tf));
  }
}
