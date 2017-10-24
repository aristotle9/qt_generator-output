/// C++ type: <span style='color: green;'>```QHideEvent```</span>
#[repr(C)]
pub struct HideEvent(u8);

impl HideEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QHideEvent::QHideEvent()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::hide_event::HideEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHideEvent_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::hide_event::HideEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QHideEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::hide_event::HideEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QHideEvent_G_static_cast_QEvent_ptr(self as *mut ::hide_event::HideEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHideEvent_G_static_cast_QEvent_ptr(self as *const ::hide_event::HideEvent as *mut ::hide_event::HideEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::hide_event::HideEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::hide_event::HideEvent {
    let ffi_result = ::ffi::qt_gui_c_QHideEvent_G_static_cast_QHideEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::hide_event::HideEvent {
    let ffi_result = ::ffi::qt_gui_c_QHideEvent_G_static_cast_QHideEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::hide_event::HideEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QHideEvent_G_static_cast_QEvent_ptr(self as *const ::hide_event::HideEvent as *mut ::hide_event::HideEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::hide_event::HideEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QHideEvent_G_static_cast_QEvent_ptr(self as *mut ::hide_event::HideEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
