/// C++ type: <span style='color: green;'>```Qt3DCore::QStaticPropertyValueRemovedChangeBase```</span>
#[repr(C)]
pub struct StaticPropertyValueRemovedChangeBase(u8);

impl StaticPropertyValueRemovedChangeBase {
  /// C++ method: <span style='color: green;'>```const char* Qt3DCore::QStaticPropertyValueRemovedChangeBase::propertyName() const```</span>
  ///
  ///
  pub fn property_name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QStaticPropertyValueRemovedChangeBase_propertyName(self as *const ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QStaticPropertyValueRemovedChangeBase::setPropertyName(const char* name)```</span>
  ///
  ///
  pub unsafe fn set_property_name(&mut self, name: *const ::libc::c_char) {
    ::ffi::qt_3d_core_c_Qt3DCore_QStaticPropertyValueRemovedChangeBase_setPropertyName(self as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase, name)
  }
}

impl ::cpp_utils::CppDeletable for ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QStaticPropertyValueRemovedChangeBase_delete
  }
}

impl ::cpp_utils::DynamicCast<::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase> for ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *const ::property_value_removed_change_base::PropertyValueRemovedChangeBase as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase> for ::scene_change::SceneChange {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::property_value_removed_change_base::PropertyValueRemovedChangeBase> for ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::property_value_removed_change_base::PropertyValueRemovedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *const ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase> for ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *const ::property_value_removed_change_base::PropertyValueRemovedChangeBase as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase> for ::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
  type Target = ::property_value_removed_change_base::PropertyValueRemovedChangeBase;
  fn deref(&self) -> &::property_value_removed_change_base::PropertyValueRemovedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *const ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
  fn deref_mut(&mut self) -> &mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
