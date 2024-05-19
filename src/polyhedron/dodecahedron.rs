//! Trimesh and Convex Dodecahedron from Fullerene
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Dodecahedron
#[derive(Debug, Clone)]
pub struct Dodecahedron<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Dodecahedron
impl<F: Float> TBridgeGlobal for Dodecahedron<F> {
  /// constructor
  fn void() -> Self {
    Dodecahedron::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Dodecahedron
impl<F: Float + std::fmt::Debug> Dodecahedron<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, tf: bool) {
    self.ph.from_phf(&fullerene::Dodecahedron::new(r).ph.with_uv(tf));
  }
}

/// DodecahedronCenter
#[derive(Debug, Clone)]
pub struct DodecahedronCenter<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for DodecahedronCenter
impl<F: Float> TBridgeGlobal for DodecahedronCenter<F> {
  /// constructor
  fn void() -> Self {
    DodecahedronCenter::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// DodecahedronCenter
impl<F: Float + std::fmt::Debug> DodecahedronCenter<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, r: F, tf: bool) {
    self.ph.from_phf(&fullerene::DodecahedronCenter::new(r).ph.with_uv(tf));
  }
}
