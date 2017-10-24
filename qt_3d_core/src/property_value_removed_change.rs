/// C++ type: <span style='color: green;'>```Qt3DCore::QPropertyValueRemovedChange```</span>
#[repr(C)]
pub struct PropertyValueRemovedChange(u8);

impl PropertyValueRemovedChange {
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QPropertyValueRemovedChange::QPropertyValueRemovedChange(Qt3DCore::QNodeId subjectId)```</span>
  ///
  ///
  pub fn new(subject_id: &::node_id::NodeId)
             -> ::cpp_utils::CppBox<::property_value_removed_change::PropertyValueRemovedChange> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QPropertyValueRemovedChange_new(subject_id as *const ::node_id::NodeId) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVariant Qt3DCore::QPropertyValueRemovedChange::removedValue() const```</span>
  ///
  ///
  pub fn removed_value(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QPropertyValueRemovedChange_removedValue_to_output(self as *const ::property_value_removed_change::PropertyValueRemovedChange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QPropertyValueRemovedChange::setRemovedValue(const QVariant& value)```</span>
  ///
  ///
  pub fn set_removed_value(&mut self, value: &::qt_core::variant::Variant) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QPropertyValueRemovedChange_setRemovedValue(self as *mut ::property_value_removed_change::PropertyValueRemovedChange, value as *const ::qt_core::variant::Variant) }
  }
}

impl ::cpp_utils::CppDeletable for ::property_value_removed_change::PropertyValueRemovedChange {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QPropertyValueRemovedChange_delete
  }
}

impl ::cpp_utils::DynamicCast<::property_value_removed_change::PropertyValueRemovedChange> for ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_value_removed_change::PropertyValueRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_value_removed_change::PropertyValueRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *const ::property_value_removed_change_base::PropertyValueRemovedChangeBase as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::property_value_removed_change::PropertyValueRemovedChange> for ::scene_change::SceneChange {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_value_removed_change::PropertyValueRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_value_removed_change::PropertyValueRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::property_value_removed_change::PropertyValueRemovedChange> for ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_value_removed_change::PropertyValueRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(self as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_value_removed_change::PropertyValueRemovedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_dynamic_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(self as *const ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::property_value_removed_change_base::PropertyValueRemovedChangeBase> for ::property_value_removed_change::PropertyValueRemovedChange {
fn static_cast_mut(&mut self) -> &mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *mut ::property_value_removed_change::PropertyValueRemovedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::property_value_removed_change_base::PropertyValueRemovedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChangeBase_ptr(self as *const ::property_value_removed_change::PropertyValueRemovedChange as *mut ::property_value_removed_change::PropertyValueRemovedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::property_value_removed_change::PropertyValueRemovedChange {
fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::property_value_removed_change::PropertyValueRemovedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::property_value_removed_change::PropertyValueRemovedChange as *mut ::property_value_removed_change::PropertyValueRemovedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase> for ::property_value_removed_change::PropertyValueRemovedChange {
fn static_cast_mut(&mut self) -> &mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr(self as *mut ::property_value_removed_change::PropertyValueRemovedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr(self as *const ::property_value_removed_change::PropertyValueRemovedChange as *mut ::property_value_removed_change::PropertyValueRemovedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_value_removed_change::PropertyValueRemovedChange> for ::property_value_removed_change_base::PropertyValueRemovedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_value_removed_change::PropertyValueRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_value_removed_change::PropertyValueRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QPropertyValueRemovedChangeBase(self as *const ::property_value_removed_change_base::PropertyValueRemovedChangeBase as *mut ::property_value_removed_change_base::PropertyValueRemovedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_value_removed_change::PropertyValueRemovedChange> for ::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_value_removed_change::PropertyValueRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_value_removed_change::PropertyValueRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_value_removed_change::PropertyValueRemovedChange> for ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_value_removed_change::PropertyValueRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(self as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_value_removed_change::PropertyValueRemovedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QPropertyValueRemovedChange_ptr_Qt3DCore_QStaticPropertyValueRemovedChangeBase(self as *const ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase as *mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::property_value_removed_change::PropertyValueRemovedChange {
  type Target = ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase;
  fn deref(&self) -> &::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr(self as *const ::property_value_removed_change::PropertyValueRemovedChange as *mut ::property_value_removed_change::PropertyValueRemovedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::property_value_removed_change::PropertyValueRemovedChange {
  fn deref_mut(&mut self) -> &mut ::static_property_value_removed_change_base::StaticPropertyValueRemovedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueRemovedChange_G_static_cast_Qt3DCore_QStaticPropertyValueRemovedChangeBase_ptr(self as *mut ::property_value_removed_change::PropertyValueRemovedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
