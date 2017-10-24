/// C++ type: <span style='color: green;'>```Qt3DCore::QComponentRemovedChange```</span>
#[repr(C)]
pub struct ComponentRemovedChange(u8);

impl ComponentRemovedChange {
  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DCore::QComponentRemovedChange::componentId() const```</span>
  ///
  ///
  pub fn component_id(&self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QComponentRemovedChange_componentId_to_output(self as *const ::component_removed_change::ComponentRemovedChange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QMetaObject* Qt3DCore::QComponentRemovedChange::componentMetaObject() const```</span>
  ///
  ///
  pub fn component_meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QComponentRemovedChange_componentMetaObject(self as *const ::component_removed_change::ComponentRemovedChange) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DCore::QComponentRemovedChange::entityId() const```</span>
  ///
  ///
  pub fn entity_id(&self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QComponentRemovedChange_entityId_to_output(self as *const ::component_removed_change::ComponentRemovedChange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QComponentRemovedChange::QComponentRemovedChange(const Qt3DCore::QEntity* entity, const Qt3DCore::QComponent* component)```</span>
  ///
  ///
  pub unsafe fn new(entity: *const ::entity::Entity,
                    component: *const ::component::Component)
                    -> ::cpp_utils::CppBox<::component_removed_change::ComponentRemovedChange> {
    let ffi_result = ::ffi::qt_3d_core_c_Qt3DCore_QComponentRemovedChange_new(entity, component);
    ::cpp_utils::CppBox::new(ffi_result)
  }
}

impl ::cpp_utils::CppDeletable for ::component_removed_change::ComponentRemovedChange {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QComponentRemovedChange_delete
  }
}

impl ::cpp_utils::DynamicCast<::component_removed_change::ComponentRemovedChange> for ::scene_change::SceneChange {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::component_removed_change::ComponentRemovedChange> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentRemovedChange_G_dynamic_cast_Qt3DCore_QComponentRemovedChange_ptr(self as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::component_removed_change::ComponentRemovedChange> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentRemovedChange_G_dynamic_cast_Qt3DCore_QComponentRemovedChange_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::component_removed_change::ComponentRemovedChange {
  fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::component_removed_change::ComponentRemovedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::component_removed_change::ComponentRemovedChange as *mut ::component_removed_change::ComponentRemovedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::component_removed_change::ComponentRemovedChange> for ::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::component_removed_change::ComponentRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QComponentRemovedChange_G_static_cast_Qt3DCore_QComponentRemovedChange_ptr(self as *mut ::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::component_removed_change::ComponentRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QComponentRemovedChange_G_static_cast_Qt3DCore_QComponentRemovedChange_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::component_removed_change::ComponentRemovedChange {
  type Target = ::scene_change::SceneChange;
  fn deref(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::component_removed_change::ComponentRemovedChange as *mut ::component_removed_change::ComponentRemovedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::component_removed_change::ComponentRemovedChange {
  fn deref_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QComponentRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::component_removed_change::ComponentRemovedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
