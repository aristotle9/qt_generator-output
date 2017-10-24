/// C++ type: <span style='color: green;'>```Qt3DCore::QStaticPropertyValueAddedChangeBase```</span>
#[repr(C)]
pub struct StaticPropertyValueAddedChangeBase(u8);

impl StaticPropertyValueAddedChangeBase {
  /// C++ method: <span style='color: green;'>```const char* Qt3DCore::QStaticPropertyValueAddedChangeBase::propertyName() const```</span>
  ///
  ///
  pub fn property_name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QStaticPropertyValueAddedChangeBase_propertyName(self as *const ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QStaticPropertyValueAddedChangeBase::setPropertyName(const char* name)```</span>
  ///
  ///
  pub unsafe fn set_property_name(&mut self, name: *const ::libc::c_char) {
    ::ffi::qt_3d_core_c_Qt3DCore_QStaticPropertyValueAddedChangeBase_setPropertyName(self as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase, name)
  }
}

impl ::cpp_utils::CppDeletable for ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QStaticPropertyValueAddedChangeBase_delete
  }
}

impl ::cpp_utils::DynamicCast<::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase> for ::property_value_added_change_base::PropertyValueAddedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *const ::property_value_added_change_base::PropertyValueAddedChangeBase as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase> for ::scene_change::SceneChange {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::property_value_added_change_base::PropertyValueAddedChangeBase> for ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::property_value_added_change_base::PropertyValueAddedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_static_cast_Qt3DCore_QPropertyValueAddedChangeBase_ptr(self as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::property_value_added_change_base::PropertyValueAddedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_static_cast_Qt3DCore_QPropertyValueAddedChangeBase_ptr(self as *const ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase> for ::property_value_added_change_base::PropertyValueAddedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *const ::property_value_added_change_base::PropertyValueAddedChangeBase as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase> for ::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
  type Target = ::property_value_added_change_base::PropertyValueAddedChangeBase;
  fn deref(&self) -> &::property_value_added_change_base::PropertyValueAddedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_static_cast_Qt3DCore_QPropertyValueAddedChangeBase_ptr(self as *const ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
  fn deref_mut(&mut self) -> &mut ::property_value_added_change_base::PropertyValueAddedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueAddedChangeBase_G_static_cast_Qt3DCore_QPropertyValueAddedChangeBase_ptr(self as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
