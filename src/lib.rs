#![doc(html_root_url = "https://docs.rs/trimesh/0.1.2")]
//! Polyhedron on the ODE (Open Dynamics Engine) trimesh for Rust
//!

pub mod polyhedron;

/*
/// tests
#[cfg(test)]
mod tests {
  use super::polyhedron::{
    Icosahedron,
    {Dodecahedron, DodecahedronCenter},
    {C60, C60Center}};
  use anyslot::anyslot::*;

  /// [-- --nocapture] [-- --show-output]
  #[test]
  fn tes_polyhedron() {
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
}
*/
