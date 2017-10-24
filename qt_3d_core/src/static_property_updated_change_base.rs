/// C++ type: <span style='color: green;'>```Qt3DCore::QStaticPropertyUpdatedChangeBase```</span>
#[repr(C)]
pub struct StaticPropertyUpdatedChangeBase(u8);

impl StaticPropertyUpdatedChangeBase {
  /// C++ method: <span style='color: green;'>```const char* Qt3DCore::QStaticPropertyUpdatedChangeBase::propertyName() const```</span>
  ///
  ///
  pub fn property_name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QStaticPropertyUpdatedChangeBase_propertyName(self as *const ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QStaticPropertyUpdatedChangeBase::setPropertyName(const char* name)```</span>
  ///
  ///
  pub unsafe fn set_property_name(&mut self, name: *const ::libc::c_char) {
    ::ffi::qt_3d_core_c_Qt3DCore_QStaticPropertyUpdatedChangeBase_setPropertyName(self as *mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase, name)
  }
}

impl ::cpp_utils::CppDeletable for ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QStaticPropertyUpdatedChangeBase_delete
  }
}

impl ::cpp_utils::DynamicCast<::static_property_updated_change_base::StaticPropertyUpdatedChangeBase> for ::property_updated_change_base::PropertyUpdatedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *mut ::property_updated_change_base::PropertyUpdatedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::static_property_updated_change_base::StaticPropertyUpdatedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *const ::property_updated_change_base::PropertyUpdatedChangeBase as *mut ::property_updated_change_base::PropertyUpdatedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::static_property_updated_change_base::StaticPropertyUpdatedChangeBase> for ::scene_change::SceneChange {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::static_property_updated_change_base::StaticPropertyUpdatedChangeBase> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_dynamic_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::property_updated_change_base::PropertyUpdatedChangeBase> for ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::property_updated_change_base::PropertyUpdatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::property_updated_change_base::PropertyUpdatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *const ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase as *mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase as *mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::static_property_updated_change_base::StaticPropertyUpdatedChangeBase> for ::property_updated_change_base::PropertyUpdatedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *mut ::property_updated_change_base::PropertyUpdatedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *const ::property_updated_change_base::PropertyUpdatedChangeBase as *mut ::property_updated_change_base::PropertyUpdatedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::static_property_updated_change_base::StaticPropertyUpdatedChangeBase> for ::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
let ffi_result = ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
  type Target = ::property_updated_change_base::PropertyUpdatedChangeBase;
  fn deref(&self) -> &::property_updated_change_base::PropertyUpdatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *const ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase as *mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
  fn deref_mut(&mut self) -> &mut ::property_updated_change_base::PropertyUpdatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QStaticPropertyUpdatedChangeBase_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
