//! TriMeshManager
//!

use std::collections::HashMap;
use num::Float;
use anyslot::anyslot::*;
use crate::polyhedron::{
  self, tetra::*, cube::*, octa::*,
  sphere::*, cylinder::*, capsule::*, cone::*,
  torus::*, pipe::*, // polyhedron::pin
  revolution::*,
  Icosahedron,
  {Dodecahedron, DodecahedronCenter},
  {C60, C60Center}};

/// TriMeshManager
pub struct TriMeshManager<F: Float> {
  /// Tetra
  pub tetra: HashMap<String, Tetra<F>>,
  /// Cube
  pub cube: HashMap<String, Cube<F>>,
  /// CubeCenter
  pub cube_center: HashMap<String, CubeCenter<F>>,
  /// Octa
  pub octa: HashMap<String, Octa<F>>,
  /// RSphere
  pub r_sphere: HashMap<String, RSphere<F>>,
  /// Cylinder
  pub cylinder: HashMap<String, Cylinder<F>>,
  /// Capsule
  pub capsule: HashMap<String, Capsule<F>>,
  /// Cone
  pub cone: HashMap<String, Cone<F>>,
  /// Torus
  pub torus: HashMap<String, Torus<F>>,
  /// RTorus
  pub r_torus: HashMap<String, RTorus<F>>,
  /// Ring
  pub ring: HashMap<String, Ring<F>>,
  /// Tube
  pub tube: HashMap<String, Tube<F>>,
  /// HalfPipe
  pub half_pipe: HashMap<String, HalfPipe<F>>,
  /// Pin
  pub pin: HashMap<String, polyhedron::pin::Pin<F>>,
  /// Revolution
  pub revolution: HashMap<String, Revolution<F>>,
  /// Icosahedron
  pub icosahedron: HashMap<String, Icosahedron<F>>,
  /// Dodecahedron
  pub dodecahedron: HashMap<String, Dodecahedron<F>>,
  /// DodecahedronCenter
  pub dodecahedron_center: HashMap<String, DodecahedronCenter<F>>,
  /// C60
  pub c60: HashMap<String, C60<F>>,
  /// C60Center
  pub c60_center: HashMap<String, C60Center<F>>
}

/// TBridgeGlobal for TriMeshManager
impl<F: Float> TBridgeGlobal for TriMeshManager<F> {
  /// constructor
  fn void() -> Self {
    TriMeshManager::<F>{
      tetra: HashMap::<String, Tetra<F>>::new(),
      cube: HashMap::<String, Cube<F>>::new(),
      cube_center: HashMap::<String, CubeCenter<F>>::new(),
      octa: HashMap::<String, Octa<F>>::new(),
      r_sphere: HashMap::<String, RSphere<F>>::new(),
      cylinder: HashMap::<String, Cylinder<F>>::new(),
      capsule: HashMap::<String, Capsule<F>>::new(),
      cone: HashMap::<String, Cone<F>>::new(),
      torus: HashMap::<String, Torus<F>>::new(),
      r_torus: HashMap::<String, RTorus<F>>::new(),
      ring: HashMap::<String, Ring<F>>::new(),
      tube: HashMap::<String, Tube<F>>::new(),
      half_pipe: HashMap::<String, HalfPipe<F>>::new(),
      pin: HashMap::<String, polyhedron::pin::Pin<F>>::new(),
      revolution: HashMap::<String, Revolution<F>>::new(),
      icosahedron: HashMap::<String, Icosahedron<F>>::new(),
      dodecahedron: HashMap::<String, Dodecahedron<F>>::new(),
      dodecahedron_center: HashMap::<String, DodecahedronCenter<F>>::new(),
      c60: HashMap::<String, C60<F>>::new(),
      c60_center: HashMap::<String, C60Center<F>>::new()
    }
  }
}

/// TriMeshManager
impl<F: Float> TriMeshManager<F> {
  /// setup
  pub fn setup(&mut self) {
    // nothing
  }
}

/// setter for TriMeshManager
#[macro_export]
macro_rules! tms {
  ($tm: ident, $hm: ident, $t: ty, $k: expr) => {{
    let key = format!("{}_{}", stringify!($hm), $k);
    $tm.$hm.insert(key.clone(), <$t>::void());
    $tm.$hm.get_mut(&key).expect(key.as_str())
  }}
}
pub use tms;

/// getter mut for TriMeshManager
#[macro_export]
macro_rules! tmg {
  ($tm: ident, $hm: ident, $k: expr) => {{
    let key = format!("{}_{}", stringify!($hm), $k);
    $tm.$hm.get_mut(&key).expect(key.as_str())
  }}
}
pub use tmg;
