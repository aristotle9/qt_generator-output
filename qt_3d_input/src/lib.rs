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

/// Entities from `QAbstractActionInput` C++ header
pub mod abstract_action_input;
/// Entities from `QAbstractAxisInput` C++ header
pub mod abstract_axis_input;
/// Entities from `QAbstractPhysicalDevice` C++ header
pub mod abstract_physical_device;
/// Entities from `QAction` C++ header
pub mod action;
/// Entities from `QActionInput` C++ header
pub mod action_input;
/// Entities from `QAnalogAxisInput` C++ header
pub mod analog_axis_input;
/// Entities from `QAxis` C++ header
pub mod axis;
/// Entities from `QAxisAccumulator` C++ header
pub mod axis_accumulator;
/// Entities from `QAxisSetting` C++ header
pub mod axis_setting;
/// Entities from `QButtonAxisInput` C++ header
pub mod button_axis_input;
/// Entities from `QInputAspect` C++ header
pub mod input_aspect;
/// Entities from `QInputChord` C++ header
pub mod input_chord;
/// Entities from `QInputSequence` C++ header
pub mod input_sequence;
/// Entities from `QInputSettings` C++ header
pub mod input_settings;
/// Entities from `QKeyEvent` C++ header
pub mod key_event;
/// Entities from `QKeyboardDevice` C++ header
pub mod keyboard_device;
/// Entities from `QKeyboardHandler` C++ header
pub mod keyboard_handler;
/// Entities from `QLogicalDevice` C++ header
pub mod logical_device;
/// Entities from `QMouseDevice` C++ header
pub mod mouse_device;
/// Entities from `QMouseEvent` C++ header
pub mod mouse_event;
/// Entities from `QMouseHandler` C++ header
pub mod mouse_handler;
/// Entities from `QPhysicalDeviceCreatedChange` C++ header
pub mod physical_device_created_change;
/// Binding Qt signals to Rust closures or extern functions.
///
/// Types in this module allow to connect Qt signals with certain argument types to a Rust closure.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots` module.
pub mod slots;
/// Entities from `QVector` C++ header
pub mod vector;
