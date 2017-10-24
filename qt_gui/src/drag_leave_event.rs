/// C++ type: <span style='color: green;'>```QDragLeaveEvent```</span>
#[repr(C)]
pub struct DragLeaveEvent(u8);

impl DragLeaveEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QDragLeaveEvent::QDragLeaveEvent()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::drag_leave_event::DragLeaveEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragLeaveEvent_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::drag_leave_event::DragLeaveEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QDragLeaveEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::drag_leave_event::DragLeaveEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QDragLeaveEvent_G_static_cast_QEvent_ptr(self as *mut ::drag_leave_event::DragLeaveEvent)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragLeaveEvent_G_static_cast_QEvent_ptr(self as *const ::drag_leave_event::DragLeaveEvent as *mut ::drag_leave_event::DragLeaveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::drag_leave_event::DragLeaveEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::drag_leave_event::DragLeaveEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QDragLeaveEvent_G_static_cast_QDragLeaveEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::drag_leave_event::DragLeaveEvent {
    let ffi_result = ::ffi::qt_gui_c_QDragLeaveEvent_G_static_cast_QDragLeaveEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::drag_leave_event::DragLeaveEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDragLeaveEvent_G_static_cast_QEvent_ptr(self as *const ::drag_leave_event::DragLeaveEvent as *mut ::drag_leave_event::DragLeaveEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::drag_leave_event::DragLeaveEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QDragLeaveEvent_G_static_cast_QEvent_ptr(self as *mut ::drag_leave_event::DragLeaveEvent)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
