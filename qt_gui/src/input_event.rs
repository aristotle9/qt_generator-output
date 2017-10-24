/// C++ type: <span style='color: green;'>```QInputEvent```</span>
#[repr(C)]
pub struct InputEvent(u8);

impl InputEvent {
  /// C++ method: <span style='color: green;'>```void QInputEvent::setTimestamp(unsigned long atimestamp)```</span>
  ///
  ///
  pub fn set_timestamp(&mut self, atimestamp: ::libc::c_ulong) {
    unsafe { ::ffi::qt_gui_c_QInputEvent_setTimestamp(self as *mut ::input_event::InputEvent, atimestamp) }
  }

  /// C++ method: <span style='color: green;'>```unsigned long QInputEvent::timestamp() const```</span>
  ///
  ///
  pub fn timestamp(&self) -> ::libc::c_ulong {
    unsafe { ::ffi::qt_gui_c_QInputEvent_timestamp(self as *const ::input_event::InputEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::input_event::InputEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QInputEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::input_event::InputEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QInputEvent_G_static_cast_QEvent_ptr(self as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QInputEvent_G_static_cast_QEvent_ptr(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::input_event::InputEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result = ::ffi::qt_gui_c_QInputEvent_G_static_cast_QInputEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::input_event::InputEvent {
    let ffi_result = ::ffi::qt_gui_c_QInputEvent_G_static_cast_QInputEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::input_event::InputEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QInputEvent_G_static_cast_QEvent_ptr(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::input_event::InputEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QInputEvent_G_static_cast_QEvent_ptr(self as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
