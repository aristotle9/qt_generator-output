/// C++ type: <span style='color: green;'>```Qt3DCore::QPropertyValueAddedChange```</span>
#[repr(C)]
pub struct PropertyValueAddedChange(u8);

impl PropertyValueAddedChange {
  /// C++ method: <span style='color: green;'>```QVariant Qt3DCore::QPropertyValueAddedChange::addedValue() const```</span>
  ///
  ///
  pub fn added_value(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QPropertyValueAddedChange_addedValue_to_output(self as *const ::property_value_added_change::PropertyValueAddedChange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QPropertyValueAddedChange::QPropertyValueAddedChange(Qt3DCore::QNodeId subjectId)```</span>
  ///
  ///
  pub fn new(subject_id: &::node_id::NodeId)
             -> ::cpp_utils::CppBox<::property_value_added_change::PropertyValueAddedChange> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QPropertyValueAddedChange_new(subject_id as *const ::node_id::NodeId) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QPropertyValueAddedChange::setAddedValue(const QVariant& value)```</span>
  ///
  ///
  pub fn set_added_value(&mut self, value: &::qt_core::variant::Variant) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QPropertyValueAddedChange_setAddedValue(self as *mut ::property_value_added_change::PropertyValueAddedChange, value as *const ::qt_core::variant::Variant) }
  }
}

impl ::cpp_utils::CppDeletable for ::property_value_added_change::PropertyValueAddedChange {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QPropertyValueAddedChange_delete
  }
}

impl ::cpp_utils::DynamicCast<::property_value_added_change::PropertyValueAddedChange> for ::property_value_added_change_base::PropertyValueAddedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_value_added_change::PropertyValueAddedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_dynamic_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_value_added_change::PropertyValueAddedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_dynamic_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *const ::property_value_added_change_base::PropertyValueAddedChangeBase as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::property_value_added_change::PropertyValueAddedChange> for ::scene_change::SceneChange {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_value_added_change::PropertyValueAddedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_dynamic_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_value_added_change::PropertyValueAddedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_dynamic_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::property_value_added_change::PropertyValueAddedChange> for ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_value_added_change::PropertyValueAddedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_dynamic_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(self as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_value_added_change::PropertyValueAddedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_dynamic_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(self as *const ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::property_value_added_change_base::PropertyValueAddedChangeBase> for ::property_value_added_change::PropertyValueAddedChange {
fn static_cast_mut(&mut self) -> &mut ::property_value_added_change_base::PropertyValueAddedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChangeBase_ptr(self as *mut ::property_value_added_change::PropertyValueAddedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::property_value_added_change_base::PropertyValueAddedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChangeBase_ptr(self as *const ::property_value_added_change::PropertyValueAddedChange as *mut ::property_value_added_change::PropertyValueAddedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::property_value_added_change::PropertyValueAddedChange {
fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::property_value_added_change::PropertyValueAddedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::scene_change::SceneChange {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::property_value_added_change::PropertyValueAddedChange as *mut ::property_value_added_change::PropertyValueAddedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase> for ::property_value_added_change::PropertyValueAddedChange {
fn static_cast_mut(&mut self) -> &mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr(self as *mut ::property_value_added_change::PropertyValueAddedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr(self as *const ::property_value_added_change::PropertyValueAddedChange as *mut ::property_value_added_change::PropertyValueAddedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_value_added_change::PropertyValueAddedChange> for ::property_value_added_change_base::PropertyValueAddedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_value_added_change::PropertyValueAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_value_added_change::PropertyValueAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QPropertyValueAddedChangeBase(self as *const ::property_value_added_change_base::PropertyValueAddedChangeBase as *mut ::property_value_added_change_base::PropertyValueAddedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_value_added_change::PropertyValueAddedChange> for ::scene_change::SceneChange {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_value_added_change::PropertyValueAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_value_added_change::PropertyValueAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_value_added_change::PropertyValueAddedChange> for ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_value_added_change::PropertyValueAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(self as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_value_added_change::PropertyValueAddedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QPropertyValueAddedChange_ptr_Qt3DCore_QStaticPropertyValueAddedChangeBase(self as *const ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase as *mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::property_value_added_change::PropertyValueAddedChange {
  type Target = ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase;
  fn deref(&self) -> &::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr(self as *const ::property_value_added_change::PropertyValueAddedChange as *mut ::property_value_added_change::PropertyValueAddedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::property_value_added_change::PropertyValueAddedChange {
  fn deref_mut(&mut self) -> &mut ::static_property_value_added_change_base::StaticPropertyValueAddedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyValueAddedChange_G_static_cast_Qt3DCore_QStaticPropertyValueAddedChangeBase_ptr(self as *mut ::property_value_added_change::PropertyValueAddedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
