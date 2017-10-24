//! Bindings for [QtUiTools](http://doc.qt.io/qt-5/qtuitools-module.html) library.
//!
//! This crate was generated using [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust).
//!
//! This is work in progress, so the API will significantly change in the future.
//! Some methods are missing, and some are inconvenient to use.
//! Some methods are unsafe even though they are not marked as unsafe.
//! Users must carefully track ownership of the objects, as usual Rust guarantees
//! do not take effect. This will hopefully improve in the future.
//! Please report any issues to the
//! [issue tracker](https://github.com/rust-qt/cpp_to_rust/issues).
//!
//! This crate was generated for **Qt 5.9.1**.
//! If Qt compatibility guarantees take effect, it should be compatible
//! with future minor releases and with past and future patch releases,
//! but API added in future releases won't be available. The crate is not compatible
//! with past minor Qt releases. If you need to use a Qt version incompatible with this crate,
//! use [qt_generator](https://github.com/rust-qt/cpp_to_rust/tree/master/qt_generator/qt_generator)
//! to generate crates for your Qt version.
//!
//! Refer to `qt_core` crate documentation for general information about Qt crates.

pub extern crate libc;
pub extern crate cpp_utils;

pub extern crate qt_core;

pub extern crate qt_gui;

pub extern crate qt_widgets;

#[allow(dead_code)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

mod type_sizes {
  include!(concat!(env!("OUT_DIR"), "/type_sizes.rs"));
}

/// Entities from `QUiLoader` C++ header
pub mod ui_loader;
