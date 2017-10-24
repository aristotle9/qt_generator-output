/// C++ type: <span style='color: green;'>```QInputMethodQueryEvent```</span>
#[repr(C)]
pub struct InputMethodQueryEvent(u8);

impl InputMethodQueryEvent {
  /// C++ method: <span style='color: green;'>```void QInputMethodQueryEvent::setValue(Qt::InputMethodQuery query, const QVariant& value)```</span>
  ///
  ///
  pub fn set_value(&mut self, query: &::qt_core::qt::InputMethodQuery, value: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_gui_c_QInputMethodQueryEvent_setValue(self as *mut ::input_method_query_event::InputMethodQueryEvent,
                                                      query as *const ::qt_core::qt::InputMethodQuery,
                                                      value as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QInputMethodQueryEvent::value(Qt::InputMethodQuery query) const```</span>
  ///
  ///
  pub fn value(&self, query: &::qt_core::qt::InputMethodQuery) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QInputMethodQueryEvent_value_to_output(self as *const ::input_method_query_event::InputMethodQueryEvent, query as *const ::qt_core::qt::InputMethodQuery, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::input_method_query_event::InputMethodQueryEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QInputMethodQueryEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::input_method_query_event::InputMethodQueryEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QInputMethodQueryEvent_G_static_cast_QEvent_ptr(self as *mut ::input_method_query_event::InputMethodQueryEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QInputMethodQueryEvent_G_static_cast_QEvent_ptr(self as *const ::input_method_query_event::InputMethodQueryEvent as *mut ::input_method_query_event::InputMethodQueryEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_method_query_event::InputMethodQueryEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_method_query_event::InputMethodQueryEvent {
    let ffi_result = ::ffi::qt_gui_c_QInputMethodQueryEvent_G_static_cast_QInputMethodQueryEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_method_query_event::InputMethodQueryEvent {
    let ffi_result = ::ffi::qt_gui_c_QInputMethodQueryEvent_G_static_cast_QInputMethodQueryEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::input_method_query_event::InputMethodQueryEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QInputMethodQueryEvent_G_static_cast_QEvent_ptr(self as *const ::input_method_query_event::InputMethodQueryEvent as *mut ::input_method_query_event::InputMethodQueryEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::input_method_query_event::InputMethodQueryEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QInputMethodQueryEvent_G_static_cast_QEvent_ptr(self as *mut ::input_method_query_event::InputMethodQueryEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
