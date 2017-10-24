/// C++ type: <span style='color: green;'>```Qt3DCore::QPropertyValueRemovedChangeBase```</span>
#[repr(C)]
pub struct PropertyValueRemovedChangeBase(u8);

impl ::cpp_utils::CppDeletable for ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QPropertyValueRemovedChangeBase_delete
  }
}

impl ::cpp_utils::DynamicCast<::property_value_removed_change_base::PropertyValueRemovedChangeBase> for ::scene_change::SceneChange {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChangeBase_G_dynamic_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_value_removed_change_base::PropertyValueRemovedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChangeBase_G_dynamic_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::property_value_removed_change_base::PropertyValueRemovedChangeBase as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_value_removed_change_base::PropertyValueRemovedChangeBase> for ::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *mut ::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_value_removed_change_base::PropertyValueRemovedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
  type Target = ::scene_change::SceneChange;
  fn deref(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::property_value_removed_change_base::PropertyValueRemovedChangeBase as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
  fn deref_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
