/// C++ type: <span style='color: green;'>```QToolBarChangeEvent```</span>
#[repr(C)]
pub struct ToolBarChangeEvent(u8);

impl ToolBarChangeEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QToolBarChangeEvent::QToolBarChangeEvent(bool t)```</span>
  ///
  ///
  pub fn new(t: bool) -> ::cpp_utils::CppBox<::tool_bar_change_event::ToolBarChangeEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QToolBarChangeEvent_new(t) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```bool QToolBarChangeEvent::toggle() const```</span>
  ///
  ///
  pub fn toggle(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QToolBarChangeEvent_toggle(self as *const ::tool_bar_change_event::ToolBarChangeEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::tool_bar_change_event::ToolBarChangeEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QToolBarChangeEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::tool_bar_change_event::ToolBarChangeEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QToolBarChangeEvent_G_static_cast_QEvent_ptr(self as *mut ::tool_bar_change_event::ToolBarChangeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QToolBarChangeEvent_G_static_cast_QEvent_ptr(self as *const ::tool_bar_change_event::ToolBarChangeEvent as *mut ::tool_bar_change_event::ToolBarChangeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::tool_bar_change_event::ToolBarChangeEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::tool_bar_change_event::ToolBarChangeEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QToolBarChangeEvent_G_static_cast_QToolBarChangeEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::tool_bar_change_event::ToolBarChangeEvent {
    let ffi_result = ::ffi::qt_gui_c_QToolBarChangeEvent_G_static_cast_QToolBarChangeEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::tool_bar_change_event::ToolBarChangeEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QToolBarChangeEvent_G_static_cast_QEvent_ptr(self as *const ::tool_bar_change_event::ToolBarChangeEvent as *mut ::tool_bar_change_event::ToolBarChangeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::tool_bar_change_event::ToolBarChangeEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QToolBarChangeEvent_G_static_cast_QEvent_ptr(self as *mut ::tool_bar_change_event::ToolBarChangeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
