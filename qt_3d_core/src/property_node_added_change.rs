/// C++ type: <span style='color: green;'>```Qt3DCore::QPropertyNodeAddedChange```</span>
#[repr(C)]
pub struct PropertyNodeAddedChange(u8);

impl PropertyNodeAddedChange {
  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DCore::QPropertyNodeAddedChange::addedNodeId() const```</span>
  ///
  ///
  pub fn added_node_id(&self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QPropertyNodeAddedChange_addedNodeId_to_output(self as *const ::property_node_added_change::PropertyNodeAddedChange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QMetaObject* Qt3DCore::QPropertyNodeAddedChange::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QPropertyNodeAddedChange_metaObject(self as *const ::property_node_added_change::PropertyNodeAddedChange) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QPropertyNodeAddedChange::QPropertyNodeAddedChange(Qt3DCore::QNodeId subjectId, Qt3DCore::QNode* node)```</span>
  ///
  ///
  pub unsafe fn new(subject_id: &::node_id::NodeId,
                    node: *mut ::node::Node)
                    -> ::cpp_utils::CppBox<::property_node_added_change::PropertyNodeAddedChange> {
    let ffi_result =
      ::ffi::qt_3d_core_c_Qt3DCore_QPropertyNodeAddedChange_new(subject_id as *const ::node_id::NodeId, node);
    ::cpp_utils::CppBox::new(ffi_result)
  }
}

impl ::cpp_utils::CppDeletable for ::property_node_added_change::PropertyNodeAddedChange {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QPropertyNodeAddedChange_delete
  }
}

impl ::cpp_utils::DynamicCast<::property_node_added_change::PropertyNodeAddedChange> for ::property_value_added_change_base::PropertyValueAddedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_node_added_change::PropertyNodeAddedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_node_added_change::PropertyNodeAddedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *const ::property_value_added_change_base::PropertyValueAddedChangeBase as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::property_node_added_change::PropertyNodeAddedChange> for ::scene_change::SceneChange {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_node_added_change::PropertyNodeAddedChange> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::property_node_added_change::PropertyNodeAddedChange> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::property_node_added_change::PropertyNodeAddedChange> for ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_node_added_change::PropertyNodeAddedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(self as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_node_added_change::PropertyNodeAddedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(self as *const ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::property_value_added_change_base::PropertyValueAddedChangeBase> for ::property_node_added_change::PropertyNodeAddedChange {
fn static_cast_mut(&mut self) -> &mut ::property_value_added_change_base::PropertyValueAddedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChangeBase_ptr(self as *mut ::property_node_added_change::PropertyNodeAddedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::property_value_added_change_base::PropertyValueAddedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChangeBase_ptr(self as *const ::property_node_added_change::PropertyNodeAddedChange as *mut ::property_node_added_change::PropertyNodeAddedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::property_node_added_change::PropertyNodeAddedChange {
  fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::property_node_added_change::PropertyNodeAddedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::property_node_added_change::PropertyNodeAddedChange as *mut ::property_node_added_change::PropertyNodeAddedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase> for ::property_node_added_change::PropertyNodeAddedChange {
fn static_cast_mut(&mut self) -> &mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr(self as *mut ::property_node_added_change::PropertyNodeAddedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr(self as *const ::property_node_added_change::PropertyNodeAddedChange as *mut ::property_node_added_change::PropertyNodeAddedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_node_added_change::PropertyNodeAddedChange> for ::property_value_added_change_base::PropertyValueAddedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_node_added_change::PropertyNodeAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_node_added_change::PropertyNodeAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *const ::property_value_added_change_base::PropertyValueAddedChangeBase as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_node_added_change::PropertyNodeAddedChange> for ::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_node_added_change::PropertyNodeAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_node_added_change::PropertyNodeAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_node_added_change::PropertyNodeAddedChange> for ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_node_added_change::PropertyNodeAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(self as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_node_added_change::PropertyNodeAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QPropertyNodeAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(self as *const ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::property_node_added_change::PropertyNodeAddedChange {
  type Target = ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase;
  fn deref(&self) -> &::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr(self as *const ::property_node_added_change::PropertyNodeAddedChange as *mut ::property_node_added_change::PropertyNodeAddedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::property_node_added_change::PropertyNodeAddedChange {
  fn deref_mut(&mut self) -> &mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeAddedChange_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr(self as *mut ::property_node_added_change::PropertyNodeAddedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
