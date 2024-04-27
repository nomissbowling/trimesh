#![doc(html_root_url = "https://docs.rs/trimesh/0.2.1")]
//! Polyhedron on the ODE (Open Dynamics Engine) trimesh for Rust
//!

pub mod polyhedron;

/// tests
#[cfg(test)]
mod tests {
  use super::polyhedron::{self,
    Icosahedron,
    Dodecahedron, DodecahedronCenter,
    C60, C60Center,
    pipe::{Tube, HalfPipe},
    tetra::Tetra};
  use anyslot::anyslot::*;

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn test_polyhedron() {
    let tf = true;
    let mut icosa = Icosahedron::<f32>::void();
    let mut dodeca = Dodecahedron::<f32>::void();
    let mut dodecac = DodecahedronCenter::<f32>::void();
    let mut c60 = C60::<f32>::void();
    let mut c60c = C60Center::<f32>::void();
    assert_eq!(icosa.setup(1.0, tf), ());
    assert_eq!(dodeca.setup(1.0, tf), ());
    assert_eq!(dodecac.setup(1.0, tf), ());
    assert_eq!(c60.setup(1.0, tf), ());
    assert_eq!(c60c.setup(1.0, tf), ());

    assert_eq!(icosa.ph.tmv.vtxCount, 60);
    assert_eq!(dodeca.ph.tmv.vtxCount, 12 * 3 * 3); // triangles
    assert_eq!(dodecac.ph.tmv.vtxCount, 12 * 5 * 3); // triangles
    assert_eq!(c60.ph.tmv.vtxCount, (12 * 3 + 20 * 4) * 3); // triangles
    assert_eq!(c60c.ph.tmv.vtxCount, (12 * 5 + 20 * 6) * 3); // triangles

    assert_eq!(icosa.ph.fvp.faceCount, 20);
    assert_eq!(dodeca.ph.fvp.faceCount, 12);
    assert_eq!(dodecac.ph.fvp.faceCount, 12);
    assert_eq!(c60.ph.fvp.faceCount, 12 + 20);
    assert_eq!(c60c.ph.fvp.faceCount, 12 + 20);
  }

  #[test]
  fn test_tube() {
    let tf = false;
    let mut tube = Tube::<f64>::void();
    assert_eq!(tube.setup(0.5, 0.4, 1.0, 12, tf), ());
  }

  #[test]
  fn test_halfpipe() {
    let tf = false;
    let mut halfpipe = HalfPipe::<f64>::void();
    assert_eq!(halfpipe.setup(4.71239, 0.5, 0.4, 1.0, 12, tf), ()); // 3pi/2
  }

  #[test]
  fn test_pin() {
    let tf = true;
    let mut pin = polyhedron::pin::Pin::<f64>::void();
    assert_eq!(pin.setup(1.0, tf), ());
  }

  #[test]
  fn test_tetra() {
    let tf = false;
    let mut tetra = Tetra::<f64>::void();
    assert_eq!(tetra.setup(1.0, tf), ());
    assert_eq!(tetra.ph.tmv.vtxCount, 4 * 3); // triangles duplex
    assert_eq!(tetra.ph.fvp.faceCount, 4);
  }
}
