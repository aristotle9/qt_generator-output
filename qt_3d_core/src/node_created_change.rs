/// C++ type: <span style='color: green;'>```Qt3DCore::QNodeCreatedChangeBase```</span>
#[repr(C)]
pub struct NodeCreatedChangeBase(u8);

impl NodeCreatedChangeBase {
  /// C++ method: <span style='color: green;'>```bool Qt3DCore::QNodeCreatedChangeBase::isNodeEnabled() const```</span>
  ///
  ///
  pub fn is_node_enabled(&self) -> bool {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_isNodeEnabled(self as *const ::node_created_change::NodeCreatedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```const QMetaObject* Qt3DCore::QNodeCreatedChangeBase::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_metaObject(self as *const ::node_created_change::NodeCreatedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QNodeCreatedChangeBase::QNodeCreatedChangeBase(const Qt3DCore::QNode* node)```</span>
  ///
  ///
  pub unsafe fn new(node: *const ::node::Node) -> ::cpp_utils::CppBox<::node_created_change::NodeCreatedChangeBase> {
    let ffi_result = ::ffi::qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_new(node);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DCore::QNodeCreatedChangeBase::parentId() const```</span>
  ///
  ///
  pub fn parent_id(&self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_parentId_to_output(self as *const ::node_created_change::NodeCreatedChangeBase, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::node_created_change::NodeCreatedChangeBase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QNodeCreatedChangeBase_delete
  }
}

impl ::cpp_utils::DynamicCast<::node_created_change::NodeCreatedChangeBase> for ::scene_change::SceneChange {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::node_created_change::NodeCreatedChangeBase> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeCreatedChange_G_dynamic_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::node_created_change::NodeCreatedChangeBase> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeCreatedChange_G_dynamic_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::node_created_change::NodeCreatedChangeBase {
  fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeCreatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::node_created_change::NodeCreatedChangeBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeCreatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::node_created_change::NodeCreatedChangeBase as *mut ::node_created_change::NodeCreatedChangeBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::node_created_change::NodeCreatedChangeBase> for ::scene_change::SceneChange {
  unsafe fn static_cast_mut(&mut self) -> &mut ::node_created_change::NodeCreatedChangeBase {
    let ffi_result = ::ffi::qt_3d_core_c_QNodeCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *mut ::scene_change::SceneChange);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::node_created_change::NodeCreatedChangeBase {
    let ffi_result = ::ffi::qt_3d_core_c_QNodeCreatedChange_G_static_cast_Qt3DCore_QNodeCreatedChangeBase_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::node_created_change::NodeCreatedChangeBase {
  type Target = ::scene_change::SceneChange;
  fn deref(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeCreatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::node_created_change::NodeCreatedChangeBase as *mut ::node_created_change::NodeCreatedChangeBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::node_created_change::NodeCreatedChangeBase {
  fn deref_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeCreatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::node_created_change::NodeCreatedChangeBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
