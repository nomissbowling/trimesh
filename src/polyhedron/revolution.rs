//! Trimesh and Convex Revolution
//!

use anyslot::anyslot::*;
use num::Float;

use crate::polyhedron::*;

/// Revolution
#[derive(Debug, Clone)]
pub struct Revolution<F: Float> {
  /// polyhedron
  pub ph: Polyhedron<F>
}

/// TBridgeGlobal for Revolution
impl<F: Float> TBridgeGlobal for Revolution<F> {
  /// constructor
  fn void() -> Self {
    Revolution::<F>{ph: Polyhedron::<F>::void()}
  }
}

/// Revolution
impl<F: Float + std::fmt::Debug> Revolution<F> where F: std::iter::Sum {
  /// make trimeshvi and convexfvp
  /// - fo: (bottom, top) false: fixed end, true: open end
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup<Func>(&mut self, r: F, p: u16, q: u16, fo: (bool, bool),
    f: Func, tf: bool) where Func: FnMut(u16, u16) -> (F, F) { // mut f
    let e = ph_faces::revolution::Revolution::new(r, p, q, fo, f);
    self.ph.from_polyhedron(&e.ph, tf);
  }
  /// make trimeshvi and convexfvp
  /// - fo: (bottom, top) false: fixed end, true: open end
  /// - tf: true: on the one texture, false: texture each face
  pub fn setup_from_tbl(&mut self, r: F, p: u16, q: u16, fo: (bool, bool),
    tbl: &Vec<(F, F)>, tf: bool) {
    let e = ph_faces::revolution::Revolution::from_tbl(r, p, q, fo, tbl);
    self.ph.from_polyhedron(&e.ph, tf);
  }
}
