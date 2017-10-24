//! Bindings for [Qt3DExtras](http://doc.qt.io/qt-5/qt3dextras-module.html) library.
//!
//! This crate was generated using [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust).

pub extern crate libc;
pub extern crate cpp_utils;

pub extern crate qt_core;

pub extern crate qt_gui;

pub extern crate qt_3d_core;

pub extern crate qt_3d_render;

pub extern crate qt_3d_input;

pub extern crate qt_3d_logic;

#[allow(dead_code)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

mod type_sizes {
  include!(concat!(env!("OUT_DIR"), "/type_sizes.rs"));
}

/// Entities from `QConeGeometry` C++ header
pub mod cone_geometry;
/// Entities from `QConeMesh` C++ header
pub mod cone_mesh;
/// Entities from `QCuboidGeometry` C++ header
pub mod cuboid_geometry;
/// Entities from `QCuboidMesh` C++ header
pub mod cuboid_mesh;
/// Entities from `QCylinderGeometry` C++ header
pub mod cylinder_geometry;
/// Entities from `QCylinderMesh` C++ header
pub mod cylinder_mesh;
/// Entities from `QDiffuseMapMaterial` C++ header
pub mod diffuse_map_material;
/// Entities from `QDiffuseSpecularMapMaterial` C++ header
pub mod diffuse_specular_map_material;
/// Entities from `QExtrudedTextGeometry` C++ header
pub mod extruded_text_geometry;
/// Entities from `QExtrudedTextMesh` C++ header
pub mod extruded_text_mesh;
/// Entities from `QFirstPersonCameraController` C++ header
pub mod first_person_camera_controller;
/// Entities from `QForwardRenderer` C++ header
pub mod forward_renderer;
/// Entities from `QGoochMaterial` C++ header
pub mod gooch_material;
/// Entities from `QMetalRoughMaterial` C++ header
pub mod metal_rough_material;
/// Entities from `QMorphPhongMaterial` C++ header
pub mod morph_phong_material;
/// Entities from `QNormalDiffuseMapAlphaMaterial` C++ header
pub mod normal_diffuse_map_alpha_material;
/// Entities from `QNormalDiffuseMapMaterial` C++ header
pub mod normal_diffuse_map_material;
/// Entities from `QNormalDiffuseSpecularMapMaterial` C++ header
pub mod normal_diffuse_specular_map_material;
/// Entities from `QOrbitCameraController` C++ header
pub mod orbit_camera_controller;
/// Entities from `QPerVertexColorMaterial` C++ header
pub mod per_vertex_color_material;
/// Entities from `QPhongAlphaMaterial` C++ header
pub mod phong_alpha_material;
/// Entities from `QPhongMaterial` C++ header
pub mod phong_material;
/// Entities from `QPlaneGeometry` C++ header
pub mod plane_geometry;
/// Entities from `QPlaneMesh` C++ header
pub mod plane_mesh;
/// Entities from `Qt3DWindow` C++ header
pub mod qt_3d_window;
/// Entities from `QSkyboxEntity` C++ header
pub mod skybox_entity;
/// Binding Qt signals to Rust closures or extern functions.
///
/// Types in this module allow to connect Qt signals with certain argument types to a Rust closure.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots` module.
pub mod slots;
/// Entities from `QSphereGeometry` C++ header
pub mod sphere_geometry;
/// Entities from `QSphereMesh` C++ header
pub mod sphere_mesh;
/// Entities from `QText2DEntity` C++ header
pub mod text_2d_entity;
/// Entities from `QTextureMaterial` C++ header
pub mod texture_material;
/// Entities from `QTexturedMetalRoughMaterial` C++ header
pub mod textured_metal_rough_material;
/// Entities from `QTorusGeometry` C++ header
pub mod torus_geometry;
/// Entities from `QTorusMesh` C++ header
pub mod torus_mesh;
