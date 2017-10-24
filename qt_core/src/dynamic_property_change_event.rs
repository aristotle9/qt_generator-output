/// C++ type: <span style='color: green;'>```QDynamicPropertyChangeEvent```</span>
#[repr(C)]
pub struct DynamicPropertyChangeEvent(u8);

impl DynamicPropertyChangeEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QDynamicPropertyChangeEvent::QDynamicPropertyChangeEvent(const QByteArray& name)```</span>
  ///
  ///
  pub fn new(name: &::byte_array::ByteArray)
             -> ::cpp_utils::CppBox<::dynamic_property_change_event::DynamicPropertyChangeEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QDynamicPropertyChangeEvent_new(name as *const ::byte_array::ByteArray) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QDynamicPropertyChangeEvent::propertyName() const```</span>
  ///
  ///
  pub fn property_name(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QDynamicPropertyChangeEvent_propertyName_to_output(self as *const ::dynamic_property_change_event::DynamicPropertyChangeEvent, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::dynamic_property_change_event::DynamicPropertyChangeEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QDynamicPropertyChangeEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::dynamic_property_change_event::DynamicPropertyChangeEvent> for ::event::Event {
  fn dynamic_cast_mut(&mut self)
                      -> ::std::option::Option<&mut ::dynamic_property_change_event::DynamicPropertyChangeEvent> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDynamicPropertyChangeEvent_G_dynamic_cast_QDynamicPropertyChangeEvent_ptr(self as *mut ::event::Event) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::dynamic_property_change_event::DynamicPropertyChangeEvent> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDynamicPropertyChangeEvent_G_dynamic_cast_QDynamicPropertyChangeEvent_ptr(self as *const ::event::Event as *mut ::event::Event) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::event::Event> for ::dynamic_property_change_event::DynamicPropertyChangeEvent {
  fn static_cast_mut(&mut self) -> &mut ::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDynamicPropertyChangeEvent_G_static_cast_QEvent_ptr(self as *mut ::dynamic_property_change_event::DynamicPropertyChangeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDynamicPropertyChangeEvent_G_static_cast_QEvent_ptr(self as *const ::dynamic_property_change_event::DynamicPropertyChangeEvent as *mut ::dynamic_property_change_event::DynamicPropertyChangeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::dynamic_property_change_event::DynamicPropertyChangeEvent> for ::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::dynamic_property_change_event::DynamicPropertyChangeEvent {
    let ffi_result = ::ffi::qt_core_c_QDynamicPropertyChangeEvent_G_static_cast_QDynamicPropertyChangeEvent_ptr(self as *mut ::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::dynamic_property_change_event::DynamicPropertyChangeEvent {
    let ffi_result = ::ffi::qt_core_c_QDynamicPropertyChangeEvent_G_static_cast_QDynamicPropertyChangeEvent_ptr(self as *const ::event::Event as *mut ::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::dynamic_property_change_event::DynamicPropertyChangeEvent {
  type Target = ::event::Event;
  fn deref(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDynamicPropertyChangeEvent_G_static_cast_QEvent_ptr(self as *const ::dynamic_property_change_event::DynamicPropertyChangeEvent as *mut ::dynamic_property_change_event::DynamicPropertyChangeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::dynamic_property_change_event::DynamicPropertyChangeEvent {
  fn deref_mut(&mut self) -> &mut ::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDynamicPropertyChangeEvent_G_static_cast_QEvent_ptr(self as *mut ::dynamic_property_change_event::DynamicPropertyChangeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
