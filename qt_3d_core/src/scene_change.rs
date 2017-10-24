/// C++ type: <span style='color: green;'>```Qt3DCore::ChangeFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ChangeFlag {
  /// C++ enum variant: <span style='color: green;'>```AllChanges = -1```</span>
  AllChanges = -1,
  /// C++ enum variant: <span style='color: green;'>```NodeCreated = 1```</span>
  NodeCreated = 1,
  /// C++ enum variant: <span style='color: green;'>```NodeDeleted = 2```</span>
  NodeDeleted = 2,
  /// C++ enum variant: <span style='color: green;'>```PropertyUpdated = 4```</span>
  PropertyUpdated = 4,
  /// C++ enum variant: <span style='color: green;'>```PropertyValueAdded = 8```</span>
  PropertyValueAdded = 8,
  /// C++ enum variant: <span style='color: green;'>```PropertyValueRemoved = 16```</span>
  PropertyValueRemoved = 16,
  /// C++ enum variant: <span style='color: green;'>```ComponentAdded = 32```</span>
  ComponentAdded = 32,
  /// C++ enum variant: <span style='color: green;'>```ComponentRemoved = 64```</span>
  ComponentRemoved = 64,
}

/// C++ type: <span style='color: green;'>```Qt3DCore::QSceneChange::DeliveryFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DeliveryFlag {
  /// C++ enum variant: <span style='color: green;'>```BackendNodes = 1```</span>
  BackendNodes = 1,
  /// C++ enum variant: <span style='color: green;'>```Nodes = 16```</span>
  Nodes = 16,
  /// C++ enum variant: <span style='color: green;'>```DeliverToAll = 17```</span>
  DeliverToAll = 17,
}

impl ::qt_core::flags::FlaggableEnum for DeliveryFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "DeliveryFlag"
  }
}

/// C++ type: <span style='color: green;'>```Qt3DCore::QSceneChange```</span>
#[repr(C)]
pub struct SceneChange(u8);

impl SceneChange {
  /// C++ method: <span style='color: green;'>```QFlags<Qt3DCore::QSceneChange::DeliveryFlag> Qt3DCore::QSceneChange::deliveryFlags() const```</span>
  ///
  ///
  pub fn delivery_flags(&self) -> ::qt_core::flags::Flags<::scene_change::DeliveryFlag> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QSceneChange_deliveryFlags(self as *const ::scene_change::SceneChange) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QSceneChange::setDeliveryFlags(QFlags<Qt3DCore::QSceneChange::DeliveryFlag> flags)```</span>
  ///
  ///
  pub fn set_delivery_flags(&mut self, flags: ::qt_core::flags::Flags<::scene_change::DeliveryFlag>) {
    unsafe {
      ::ffi::qt_3d_core_c_Qt3DCore_QSceneChange_setDeliveryFlags(self as *mut ::scene_change::SceneChange,
                                                                 flags.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DCore::QSceneChange::subjectId() const```</span>
  ///
  ///
  pub fn subject_id(&self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QSceneChange_subjectId_to_output(self as *const ::scene_change::SceneChange,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::ChangeFlag Qt3DCore::QSceneChange::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::scene_change::ChangeFlag {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QSceneChange_type(self as *const ::scene_change::SceneChange) }
  }
}

impl ::cpp_utils::CppDeletable for ::scene_change::SceneChange {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QSceneChange_delete
  }
}
