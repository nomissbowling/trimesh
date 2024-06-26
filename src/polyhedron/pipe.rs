//! Trimesh and Convex Pipe
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Tube
#[derive(Debug, Clone)]
pub struct Tube<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Tube
impl<F: Float> TBridgeGlobal for Tube<F> {
  /// constructor
  fn void() -> Self {
    Tube::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Tube
impl<F: Float + std::fmt::Debug> Tube<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - odm: outer diameter
  /// - idm: inner diameter
  /// - l: length
  /// - q: quality
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, odm: F, idm: F, l: F, q: u16, tf: bool) {
    let e = ph_faces::pipe::Tube::new(odm, idm, l, q);
    self.ph.from_polyhedron(&e.ph, tf);
  }
}

/// HalfPipe
#[derive(Debug, Clone)]
pub struct HalfPipe<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for HalfPipe
impl<F: Float> TBridgeGlobal for HalfPipe<F> {
  /// constructor
  fn void() -> Self {
    HalfPipe::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// HalfPipe
impl<F: Float + std::fmt::Debug> HalfPipe<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - a: arc angle
  /// - odm: outer diameter
  /// - idm: inner diameter
  /// - l: length
  /// - q: quality
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup(&mut self, a: F, odm: F, idm: F, l: F, q: u16, tf: bool) {
    let e = ph_faces::pipe::HalfPipe::new(a, odm, idm, l, q);
    self.ph.from_polyhedron(&e.ph, tf);
  }
}
