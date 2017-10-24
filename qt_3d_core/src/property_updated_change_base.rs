/// C++ type: <span style='color: green;'>```Qt3DCore::QPropertyUpdatedChangeBase```</span>
#[repr(C)]
pub struct PropertyUpdatedChangeBase(u8);

impl ::cpp_utils::CppDeletable for ::property_updated_change_base::PropertyUpdatedChangeBase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QPropertyUpdatedChangeBase_delete
  }
}

impl ::cpp_utils::DynamicCast<::property_updated_change_base::PropertyUpdatedChangeBase> for ::scene_change::SceneChange {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_updated_change_base::PropertyUpdatedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChangeBase_G_dynamic_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_updated_change_base::PropertyUpdatedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChangeBase_G_dynamic_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::property_updated_change_base::PropertyUpdatedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::property_updated_change_base::PropertyUpdatedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::property_updated_change_base::PropertyUpdatedChangeBase as *mut ::property_updated_change_base::PropertyUpdatedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_updated_change_base::PropertyUpdatedChangeBase> for ::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_updated_change_base::PropertyUpdatedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *mut ::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_updated_change_base::PropertyUpdatedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::property_updated_change_base::PropertyUpdatedChangeBase {
  type Target = ::scene_change::SceneChange;
  fn deref(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::property_updated_change_base::PropertyUpdatedChangeBase as *mut ::property_updated_change_base::PropertyUpdatedChangeBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::property_updated_change_base::PropertyUpdatedChangeBase {
  fn deref_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::property_updated_change_base::PropertyUpdatedChangeBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
