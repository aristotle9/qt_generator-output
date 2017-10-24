/// C++ type: <span style='color: green;'>```Qt3DCore::QComponentAddedChange```</span>
#[repr(C)]
pub struct ComponentAddedChange(u8);

impl ComponentAddedChange {
  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DCore::QComponentAddedChange::componentId() const```</span>
  ///
  ///
  pub fn component_id(&self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QComponentAddedChange_componentId_to_output(self as *const ::component_added_change::ComponentAddedChange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QMetaObject* Qt3DCore::QComponentAddedChange::componentMetaObject() const```</span>
  ///
  ///
  pub fn component_meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QComponentAddedChange_componentMetaObject(self as *const ::component_added_change::ComponentAddedChange) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DCore::QComponentAddedChange::entityId() const```</span>
  ///
  ///
  pub fn entity_id(&self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QComponentAddedChange_entityId_to_output(self as *const ::component_added_change::ComponentAddedChange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QComponentAddedChange::QComponentAddedChange(const Qt3DCore::QEntity* entity, const Qt3DCore::QComponent* component)```</span>
  ///
  ///
  pub unsafe fn new(entity: *const ::entity::Entity,
                    component: *const ::component::Component)
                    -> ::cpp_utils::CppBox<::component_added_change::ComponentAddedChange> {
    let ffi_result = ::ffi::qt_3d_core_c_Qt3DCore_QComponentAddedChange_new(entity, component);
    ::cpp_utils::CppBox::new(ffi_result)
  }
}

impl ::cpp_utils::CppDeletable for ::component_added_change::ComponentAddedChange {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QComponentAddedChange_delete
  }
}

impl ::cpp_utils::DynamicCast<::component_added_change::ComponentAddedChange> for ::scene_change::SceneChange {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::component_added_change::ComponentAddedChange> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentAddedChange_G_dynamic_cast_Qt3DCore_QComponentAddedChange_ptr(self as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::component_added_change::ComponentAddedChange> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentAddedChange_G_dynamic_cast_Qt3DCore_QComponentAddedChange_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::component_added_change::ComponentAddedChange {
  fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::component_added_change::ComponentAddedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::component_added_change::ComponentAddedChange as *mut ::component_added_change::ComponentAddedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::component_added_change::ComponentAddedChange> for ::scene_change::SceneChange {
  unsafe fn static_cast_mut(&mut self) -> &mut ::component_added_change::ComponentAddedChange {
    let ffi_result = ::ffi::qt_3d_core_c_QComponentAddedChange_G_static_cast_Qt3DCore_QComponentAddedChange_ptr(self as *mut ::scene_change::SceneChange);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::component_added_change::ComponentAddedChange {
    let ffi_result = ::ffi::qt_3d_core_c_QComponentAddedChange_G_static_cast_Qt3DCore_QComponentAddedChange_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::component_added_change::ComponentAddedChange {
  type Target = ::scene_change::SceneChange;
  fn deref(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::component_added_change::ComponentAddedChange as *mut ::component_added_change::ComponentAddedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::component_added_change::ComponentAddedChange {
  fn deref_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::component_added_change::ComponentAddedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
