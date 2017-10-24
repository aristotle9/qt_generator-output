/// C++ type: <span style='color: green;'>```Qt3DCore::QDynamicPropertyUpdatedChange```</span>
#[repr(C)]
pub struct DynamicPropertyUpdatedChange(u8);

impl DynamicPropertyUpdatedChange {
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QDynamicPropertyUpdatedChange::QDynamicPropertyUpdatedChange(Qt3DCore::QNodeId subjectId)```</span>
  ///
  ///
  pub fn new(subject_id: &::node_id::NodeId)
             -> ::cpp_utils::CppBox<::dynamic_property_updated_change::DynamicPropertyUpdatedChange> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_new(subject_id as *const ::node_id::NodeId) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray Qt3DCore::QDynamicPropertyUpdatedChange::propertyName() const```</span>
  ///
  ///
  pub fn property_name(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_propertyName_to_output(self as *const ::dynamic_property_updated_change::DynamicPropertyUpdatedChange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QDynamicPropertyUpdatedChange::setPropertyName(const QByteArray& name)```</span>
  ///
  ///
  pub fn set_property_name(&mut self, name: &::qt_core::byte_array::ByteArray) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_setPropertyName(self as *mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange, name as *const ::qt_core::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QDynamicPropertyUpdatedChange::setValue(const QVariant& value)```</span>
  ///
  ///
  pub fn set_value(&mut self, value: &::qt_core::variant::Variant) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_setValue(self as *mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange, value as *const ::qt_core::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```QVariant Qt3DCore::QDynamicPropertyUpdatedChange::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_value_to_output(self as *const ::dynamic_property_updated_change::DynamicPropertyUpdatedChange, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::dynamic_property_updated_change::DynamicPropertyUpdatedChange {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QDynamicPropertyUpdatedChange_delete
  }
}

impl ::cpp_utils::DynamicCast<::dynamic_property_updated_change::DynamicPropertyUpdatedChange> for ::property_updated_change_base::PropertyUpdatedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *mut ::property_updated_change_base::PropertyUpdatedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::dynamic_property_updated_change::DynamicPropertyUpdatedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *const ::property_updated_change_base::PropertyUpdatedChangeBase as *mut ::property_updated_change_base::PropertyUpdatedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::dynamic_property_updated_change::DynamicPropertyUpdatedChange> for ::scene_change::SceneChange {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::dynamic_property_updated_change::DynamicPropertyUpdatedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::property_updated_change_base::PropertyUpdatedChangeBase> for ::dynamic_property_updated_change::DynamicPropertyUpdatedChange {
fn static_cast_mut(&mut self) -> &mut ::property_updated_change_base::PropertyUpdatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::property_updated_change_base::PropertyUpdatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *const ::dynamic_property_updated_change::DynamicPropertyUpdatedChange as *mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::dynamic_property_updated_change::DynamicPropertyUpdatedChange {
fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::dynamic_property_updated_change::DynamicPropertyUpdatedChange as *mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::dynamic_property_updated_change::DynamicPropertyUpdatedChange> for ::property_updated_change_base::PropertyUpdatedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange {
let ffi_result = ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *mut ::property_updated_change_base::PropertyUpdatedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::dynamic_property_updated_change::DynamicPropertyUpdatedChange {
let ffi_result = ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *const ::property_updated_change_base::PropertyUpdatedChangeBase as *mut ::property_updated_change_base::PropertyUpdatedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::dynamic_property_updated_change::DynamicPropertyUpdatedChange> for ::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange {
let ffi_result = ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::dynamic_property_updated_change::DynamicPropertyUpdatedChange {
let ffi_result = ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QDynamicPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::dynamic_property_updated_change::DynamicPropertyUpdatedChange {
  type Target = ::property_updated_change_base::PropertyUpdatedChangeBase;
  fn deref(&self) -> &::property_updated_change_base::PropertyUpdatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *const ::dynamic_property_updated_change::DynamicPropertyUpdatedChange as *mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::dynamic_property_updated_change::DynamicPropertyUpdatedChange {
  fn deref_mut(&mut self) -> &mut ::property_updated_change_base::PropertyUpdatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QDynamicPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *mut ::dynamic_property_updated_change::DynamicPropertyUpdatedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
