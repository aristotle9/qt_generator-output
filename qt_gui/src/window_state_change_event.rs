/// C++ type: <span style='color: green;'>```QWindowStateChangeEvent```</span>
#[repr(C)]
pub struct WindowStateChangeEvent(u8);

impl WindowStateChangeEvent {
  /// C++ method: <span style='color: green;'>```bool QWindowStateChangeEvent::isOverride() const```</span>
  ///
  ///
  pub fn is_override(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QWindowStateChangeEvent_isOverride(self as *const ::window_state_change_event::WindowStateChangeEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::window_state_change_event::WindowStateChangeEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QWindowStateChangeEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::window_state_change_event::WindowStateChangeEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWindowStateChangeEvent_G_static_cast_QEvent_ptr(self as *mut ::window_state_change_event::WindowStateChangeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWindowStateChangeEvent_G_static_cast_QEvent_ptr(self as *const ::window_state_change_event::WindowStateChangeEvent as *mut ::window_state_change_event::WindowStateChangeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::window_state_change_event::WindowStateChangeEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::window_state_change_event::WindowStateChangeEvent {
    let ffi_result = ::ffi::qt_gui_c_QWindowStateChangeEvent_G_static_cast_QWindowStateChangeEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::window_state_change_event::WindowStateChangeEvent {
    let ffi_result = ::ffi::qt_gui_c_QWindowStateChangeEvent_G_static_cast_QWindowStateChangeEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::window_state_change_event::WindowStateChangeEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWindowStateChangeEvent_G_static_cast_QEvent_ptr(self as *const ::window_state_change_event::WindowStateChangeEvent as *mut ::window_state_change_event::WindowStateChangeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::window_state_change_event::WindowStateChangeEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWindowStateChangeEvent_G_static_cast_QEvent_ptr(self as *mut ::window_state_change_event::WindowStateChangeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
