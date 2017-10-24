//! Bindings for [Qt3DInput](http://doc.qt.io/qt-5/qt3dinput-module.html) library.
//!
//! This crate was generated using [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust).

pub extern crate libc;
pub extern crate cpp_utils;

pub extern crate qt_core;

pub extern crate qt_gui;

pub extern crate qt_3d_core;

#[allow(dead_code)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

mod type_sizes {
  include!(concat!(env!("OUT_DIR"), "/type_sizes.rs"));
}

/// Entities from `QFrameAction` C++ header
pub mod frame_action;
/// Entities from `QLogicAspect` C++ header
pub mod logic_aspect;
