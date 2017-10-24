/// C++ type: <span style='color: green;'>```Qt3DCore::QNodeDestroyedChange```</span>
#[repr(C)]
pub struct NodeDestroyedChange(u8);

impl NodeDestroyedChange {
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QNodeDestroyedChange::QNodeDestroyedChange(const Qt3DCore::QNode* node, const QVector<Qt3DCore::QNodeIdTypePair>& subtreeIdsAndTypes)```</span>
  ///
  ///
  pub unsafe fn new(node: *const ::node::Node,
                    subtree_ids_and_types: &::vector::VectorNodeNodeIdTypePair)
                    -> ::cpp_utils::CppBox<::node_destroyed_change::NodeDestroyedChange> {
    let ffi_result = ::ffi::qt_3d_core_c_Qt3DCore_QNodeDestroyedChange_new(node, subtree_ids_and_types as *const ::vector::VectorNodeNodeIdTypePair);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DCore::QNodeIdTypePair> Qt3DCore::QNodeDestroyedChange::subtreeIdsAndTypes() const```</span>
  ///
  ///
  pub fn subtree_ids_and_types(&self) -> ::vector::VectorNodeNodeIdTypePair {
    {
      let mut object: ::vector::VectorNodeNodeIdTypePair =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QNodeDestroyedChange_subtreeIdsAndTypes_to_output(self as *const ::node_destroyed_change::NodeDestroyedChange, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::node_destroyed_change::NodeDestroyedChange {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QNodeDestroyedChange_delete
  }
}

impl ::cpp_utils::DynamicCast<::node_destroyed_change::NodeDestroyedChange> for ::scene_change::SceneChange {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::node_destroyed_change::NodeDestroyedChange> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeDestroyedChange_G_dynamic_cast_Qt3DCore_QNodeDestroyedChange_ptr(self as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::node_destroyed_change::NodeDestroyedChange> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeDestroyedChange_G_dynamic_cast_Qt3DCore_QNodeDestroyedChange_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::node_destroyed_change::NodeDestroyedChange {
  fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeDestroyedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::node_destroyed_change::NodeDestroyedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeDestroyedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::node_destroyed_change::NodeDestroyedChange as *mut ::node_destroyed_change::NodeDestroyedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::node_destroyed_change::NodeDestroyedChange> for ::scene_change::SceneChange {
  unsafe fn static_cast_mut(&mut self) -> &mut ::node_destroyed_change::NodeDestroyedChange {
    let ffi_result = ::ffi::qt_3d_core_c_QNodeDestroyedChange_G_static_cast_Qt3DCore_QNodeDestroyedChange_ptr(self as *mut ::scene_change::SceneChange);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::node_destroyed_change::NodeDestroyedChange {
    let ffi_result = ::ffi::qt_3d_core_c_QNodeDestroyedChange_G_static_cast_Qt3DCore_QNodeDestroyedChange_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::node_destroyed_change::NodeDestroyedChange {
  type Target = ::scene_change::SceneChange;
  fn deref(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeDestroyedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::node_destroyed_change::NodeDestroyedChange as *mut ::node_destroyed_change::NodeDestroyedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::node_destroyed_change::NodeDestroyedChange {
  fn deref_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QNodeDestroyedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::node_destroyed_change::NodeDestroyedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
