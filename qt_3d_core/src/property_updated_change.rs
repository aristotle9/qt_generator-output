/// C++ type: <span style='color: green;'>```Qt3DCore::QPropertyUpdatedChange```</span>
#[repr(C)]
pub struct PropertyUpdatedChange(u8);

impl PropertyUpdatedChange {
  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DCore::QPropertyUpdatedChange::QPropertyUpdatedChange(Qt3DCore::QNodeId subjectId)```</span>
  ///
  ///
  pub fn new(subject_id: &::node_id::NodeId) -> ::cpp_utils::CppBox<::property_updated_change::PropertyUpdatedChange> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QPropertyUpdatedChange_new(subject_id as *const ::node_id::NodeId) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DCore::QPropertyUpdatedChange::setValue(const QVariant& value)```</span>
  ///
  ///
  pub fn set_value(&mut self, value: &::qt_core::variant::Variant) {
    unsafe { ::ffi::qt_3d_core_c_Qt3DCore_QPropertyUpdatedChange_setValue(self as *mut ::property_updated_change::PropertyUpdatedChange, value as *const ::qt_core::variant::Variant) }
  }

  /// C++ method: <span style='color: green;'>```QVariant Qt3DCore::QPropertyUpdatedChange::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_core_c_Qt3DCore_QPropertyUpdatedChange_value_to_output(self as *const ::property_updated_change::PropertyUpdatedChange, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::property_updated_change::PropertyUpdatedChange {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_core_c_Qt3DCore_QPropertyUpdatedChange_delete
  }
}

impl ::cpp_utils::DynamicCast<::property_updated_change::PropertyUpdatedChange> for ::property_updated_change_base::PropertyUpdatedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_updated_change::PropertyUpdatedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *mut ::property_updated_change_base::PropertyUpdatedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_updated_change::PropertyUpdatedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *const ::property_updated_change_base::PropertyUpdatedChangeBase as *mut ::property_updated_change_base::PropertyUpdatedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::DynamicCast<::property_updated_change::PropertyUpdatedChange> for ::scene_change::SceneChange {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_updated_change::PropertyUpdatedChange> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::property_updated_change::PropertyUpdatedChange> {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::DynamicCast<::property_updated_change::PropertyUpdatedChange> for ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::property_updated_change::PropertyUpdatedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QStaticPropertyUpdatedChangeBase(self as *mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::property_updated_change::PropertyUpdatedChange> {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_dynamic_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QStaticPropertyUpdatedChangeBase(self as *const ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase as *mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::property_updated_change_base::PropertyUpdatedChangeBase> for ::property_updated_change::PropertyUpdatedChange {
fn static_cast_mut(&mut self) -> &mut ::property_updated_change_base::PropertyUpdatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *mut ::property_updated_change::PropertyUpdatedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::property_updated_change_base::PropertyUpdatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChangeBase_ptr(self as *const ::property_updated_change::PropertyUpdatedChange as *mut ::property_updated_change::PropertyUpdatedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::scene_change::SceneChange> for ::property_updated_change::PropertyUpdatedChange {
  fn static_cast_mut(&mut self) -> &mut ::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *mut ::property_updated_change::PropertyUpdatedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::scene_change::SceneChange {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QSceneChange_ptr(self as *const ::property_updated_change::PropertyUpdatedChange as *mut ::property_updated_change::PropertyUpdatedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::static_property_updated_change_base::StaticPropertyUpdatedChangeBase> for ::property_updated_change::PropertyUpdatedChange {
fn static_cast_mut(&mut self) -> &mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr(self as *mut ::property_updated_change::PropertyUpdatedChange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr(self as *const ::property_updated_change::PropertyUpdatedChange as *mut ::property_updated_change::PropertyUpdatedChange) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_updated_change::PropertyUpdatedChange> for ::property_updated_change_base::PropertyUpdatedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_updated_change::PropertyUpdatedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *mut ::property_updated_change_base::PropertyUpdatedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_updated_change::PropertyUpdatedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QPropertyUpdatedChangeBase(self as *const ::property_updated_change_base::PropertyUpdatedChangeBase as *mut ::property_updated_change_base::PropertyUpdatedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::property_updated_change::PropertyUpdatedChange> for ::scene_change::SceneChange {
  unsafe fn static_cast_mut(&mut self) -> &mut ::property_updated_change::PropertyUpdatedChange {
    let ffi_result = ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(self as *mut ::scene_change::SceneChange);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::property_updated_change::PropertyUpdatedChange {
    let ffi_result = ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QSceneChange(self as *const ::scene_change::SceneChange as *mut ::scene_change::SceneChange);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::property_updated_change::PropertyUpdatedChange> for ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
unsafe fn static_cast_mut(&mut self) -> &mut ::property_updated_change::PropertyUpdatedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QStaticPropertyUpdatedChangeBase(self as *mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::property_updated_change::PropertyUpdatedChange {
let ffi_result = ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QPropertyUpdatedChange_ptr_Qt3DCore_QStaticPropertyUpdatedChangeBase(self as *const ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase as *mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::property_updated_change::PropertyUpdatedChange {
  type Target = ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase;
  fn deref(&self) -> &::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr(self as *const ::property_updated_change::PropertyUpdatedChange as *mut ::property_updated_change::PropertyUpdatedChange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::property_updated_change::PropertyUpdatedChange {
  fn deref_mut(&mut self) -> &mut ::static_property_updated_change_base::StaticPropertyUpdatedChangeBase {
    let ffi_result = unsafe { ::ffi::qt_3d_core_c_QPropertyUpdatedChange_G_static_cast_Qt3DCore_QStaticPropertyUpdatedChangeBase_ptr(self as *mut ::property_updated_change::PropertyUpdatedChange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
