/// C++ type: <span style='color: green;'>```QCloseEvent```</span>
#[repr(C)]
pub struct CloseEvent(u8);

impl CloseEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QCloseEvent::QCloseEvent()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::close_event::CloseEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QCloseEvent_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::close_event::CloseEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QCloseEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::close_event::CloseEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QCloseEvent_G_static_cast_QEvent_ptr(self as *mut ::close_event::CloseEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QCloseEvent_G_static_cast_QEvent_ptr(self as *const ::close_event::CloseEvent as *mut ::close_event::CloseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::close_event::CloseEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::close_event::CloseEvent {
    let ffi_result = ::ffi::qt_gui_c_QCloseEvent_G_static_cast_QCloseEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::close_event::CloseEvent {
    let ffi_result = ::ffi::qt_gui_c_QCloseEvent_G_static_cast_QCloseEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::close_event::CloseEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QCloseEvent_G_static_cast_QEvent_ptr(self as *const ::close_event::CloseEvent as *mut ::close_event::CloseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::close_event::CloseEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QCloseEvent_G_static_cast_QEvent_ptr(self as *mut ::close_event::CloseEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
