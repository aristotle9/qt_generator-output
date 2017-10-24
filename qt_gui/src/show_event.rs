/// C++ type: <span style='color: green;'>```QShowEvent```</span>
#[repr(C)]
pub struct ShowEvent(u8);

impl ShowEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QShowEvent::QShowEvent()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::show_event::ShowEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QShowEvent_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::show_event::ShowEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QShowEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::show_event::ShowEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QShowEvent_G_static_cast_QEvent_ptr(self as *mut ::show_event::ShowEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QShowEvent_G_static_cast_QEvent_ptr(self as *const ::show_event::ShowEvent as *mut ::show_event::ShowEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::show_event::ShowEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::show_event::ShowEvent {
    let ffi_result = ::ffi::qt_gui_c_QShowEvent_G_static_cast_QShowEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::show_event::ShowEvent {
    let ffi_result = ::ffi::qt_gui_c_QShowEvent_G_static_cast_QShowEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::show_event::ShowEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QShowEvent_G_static_cast_QEvent_ptr(self as *const ::show_event::ShowEvent as *mut ::show_event::ShowEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::show_event::ShowEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QShowEvent_G_static_cast_QEvent_ptr(self as *mut ::show_event::ShowEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
