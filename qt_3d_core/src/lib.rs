//! Bindings for [Qt3DCore](http://doc.qt.io/qt-5/qt3dcore-module.html) library.
//!
//! This crate was generated using [cpp_to_rust](https://github.com/rust-qt/cpp_to_rust).

pub extern crate libc;
pub extern crate cpp_utils;

pub extern crate qt_core;

pub extern crate qt_gui;

#[allow(dead_code)]
mod ffi {
  include!(concat!(env!("OUT_DIR"), "/ffi.rs"));
}

mod type_sizes {
  include!(concat!(env!("OUT_DIR"), "/type_sizes.rs"));
}

/// Entities from `QAbstractAspect` C++ header
pub mod abstract_aspect;
/// Entities from `QAspectEngine` C++ header
pub mod aspect_engine;
/// Entities from `QAspectJob` C++ header
pub mod aspect_job;
/// Entities from `QBackendNode` C++ header
pub mod backend_node;
/// Entities from `QComponent` C++ header
pub mod component;
/// Entities from `QComponentAddedChange` C++ header
pub mod component_added_change;
/// Entities from `QComponentRemovedChange` C++ header
pub mod component_removed_change;
/// Entities from `QDynamicPropertyUpdatedChange` C++ header
pub mod dynamic_property_updated_change;
/// Entities from `QEntity` C++ header
pub mod entity;
/// Entities from `QNode` C++ header
pub mod node;
/// Entities from `QNodeCreatedChange` C++ header
pub mod node_created_change;
/// Entities from `QNodeDestroyedChange` C++ header
pub mod node_destroyed_change;
/// Entities from `QNodeId` C++ header
pub mod node_id;
/// Entities from `QPropertyNodeAddedChange` C++ header
pub mod property_node_added_change;
/// Entities from `QPropertyNodeRemovedChange` C++ header
pub mod property_node_removed_change;
/// Entities from `QPropertyUpdatedChange` C++ header
pub mod property_updated_change;
/// Entities from `QPropertyUpdatedChangeBase` C++ header
pub mod property_updated_change_base;
/// Entities from `QPropertyValueAddedChange` C++ header
pub mod property_value_added_change;
/// Entities from `QPropertyValueAddedChangeBase` C++ header
pub mod property_value_added_change_base;
/// Entities from `QPropertyValueRemovedChange` C++ header
pub mod property_value_removed_change;
/// Entities from `QPropertyValueRemovedChangeBase` C++ header
pub mod property_value_removed_change_base;
/// Entities from `QSceneChange` C++ header
pub mod scene_change;
/// Entities from `QSharedPointer` C++ header
pub mod shared_pointer;
/// Binding Qt signals to Rust closures or extern functions.
///
/// Types in this module allow to connect Qt signals with certain argument types to a Rust closure.
///
/// There is one slot type for each distinct set of argument types present in this crate. However, if argument types were present in a dependency crate, the corresponding slot type is located in the dependency's `slots` module.
pub mod slots;
/// Entities from `QStaticPropertyUpdatedChangeBase` C++ header
pub mod static_property_updated_change_base;
/// Entities from `QStaticPropertyValueAddedChangeBase` C++ header
pub mod static_property_value_added_change_base;
/// Entities from `QStaticPropertyValueRemovedChangeBase` C++ header
pub mod static_property_value_removed_change_base;
/// Entities from `QTransform` C++ header
pub mod transform;
/// Entities from `QVector` C++ header
pub mod vector;
/// Entities from `QWeakPointer` C++ header
pub mod weak_pointer;
