/// C++ type: <span style='color: green;'>```Qt3DCore::QPropertyNodeRemovedChange```</span>
#[repr(C)]
pub struct PropertyNodeRemovedChange(u8);

impl PropertyNodeRemovedChange {
  /// C++ method: <span style='color: green;'>```const QMetaObject* Qt3DCore::QPropertyNodeRemovedChange::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QPropertyNodeRemovedChange_metaObject(self as *const ::property_node_removed_change::PropertyNodeRemovedChange) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QPropertyNodeRemovedChange::QPropertyNodeRemovedChange(Qt3DCore::QNodeId subjectId, Qt3DCore::QNode* node)```</span>
  ///
  ///
  pub unsafe fn new(subject_id: &::node_id::NodeId,
                    node: *mut ::node::Node)
                    -> ::cpp_utils::CppBox<::property_node_removed_change::PropertyNodeRemovedChange> {
    let ffi_result =
      ::ffi::qt_3d_core_c_Qt3DCore_QPropertyNodeRemovedChange_new(subject_id as *const ::node_id::NodeId, node);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DCore::QNodeId Qt3DCore::QPropertyNodeRemovedChange::removedNodeId() const```</span>
  ///
  ///
  pub fn removed_node_id(&self) -> ::node_id::NodeId {
    {
      let mut object: ::node_id::NodeId =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QPropertyNodeRemovedChange_removedNodeId_to_output(self as *const ::property_node_removed_change::PropertyNodeRemovedChange, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::property_node_removed_change::PropertyNodeRemovedChange {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QPropertyNodeRemovedChange_delete
  }
}

impl ::cpp_utils::DynamicCast<::property_node_removed_change::PropertyNodeRemovedChange> for ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_node_removed_change::PropertyNodeRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_node_removed_change::PropertyNodeRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *const ::property_value_removed_change_base::PropertyValueRemovedChangeBase as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::property_node_removed_change::PropertyNodeRemovedChange> for ::scene_change::SceneChange {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_node_removed_change::PropertyNodeRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_node_removed_change::PropertyNodeRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::property_node_removed_change::PropertyNodeRemovedChange> for ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_node_removed_change::PropertyNodeRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(self as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_node_removed_change::PropertyNodeRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(self as *const ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::property_value_removed_change_base::PropertyValueRemovedChangeBase> for ::property_node_removed_change::PropertyNodeRemovedChange {
fn static_cast_mut(&mut self) -> &mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *mut ::property_node_removed_change::PropertyNodeRemovedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::property_value_removed_change_base::PropertyValueRemovedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *const ::property_node_removed_change::PropertyNodeRemovedChange as *mut ::property_node_removed_change::PropertyNodeRemovedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::property_node_removed_change::PropertyNodeRemovedChange {
fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::property_node_removed_change::PropertyNodeRemovedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::property_node_removed_change::PropertyNodeRemovedChange as *mut ::property_node_removed_change::PropertyNodeRemovedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase> for ::property_node_removed_change::PropertyNodeRemovedChange {
fn static_cast_mut(&mut self) -> &mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr(self as *mut ::property_node_removed_change::PropertyNodeRemovedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr(self as *const ::property_node_removed_change::PropertyNodeRemovedChange as *mut ::property_node_removed_change::PropertyNodeRemovedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_node_removed_change::PropertyNodeRemovedChange> for ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_node_removed_change::PropertyNodeRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_node_removed_change::PropertyNodeRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *const ::property_value_removed_change_base::PropertyValueRemovedChangeBase as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_node_removed_change::PropertyNodeRemovedChange> for ::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_node_removed_change::PropertyNodeRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_node_removed_change::PropertyNodeRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_node_removed_change::PropertyNodeRemovedChange> for ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_node_removed_change::PropertyNodeRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(self as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_node_removed_change::PropertyNodeRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QPropertyNodeRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(self as *const ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::property_node_removed_change::PropertyNodeRemovedChange {
  type Target = ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase;
  fn deref(&self) -> &::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr(self as *const ::property_node_removed_change::PropertyNodeRemovedChange as *mut ::property_node_removed_change::PropertyNodeRemovedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::property_node_removed_change::PropertyNodeRemovedChange {
  fn deref_mut(&mut self) -> &mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyNodeRemovedChange_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr(self as *mut ::property_node_removed_change::PropertyNodeRemovedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
