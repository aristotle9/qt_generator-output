/// C++ type: <span style='color: green;'>```QIconDragEvent```</span>
#[repr(C)]
pub struct IconDragEvent(u8);

impl IconDragEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QIconDragEvent::QIconDragEvent()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::icon_drag_event::IconDragEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QIconDragEvent_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::icon_drag_event::IconDragEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QIconDragEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::icon_drag_event::IconDragEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QIconDragEvent_G_static_cast_QEvent_ptr(self as *mut ::icon_drag_event::IconDragEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QIconDragEvent_G_static_cast_QEvent_ptr(self as *const ::icon_drag_event::IconDragEvent as *mut ::icon_drag_event::IconDragEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::icon_drag_event::IconDragEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::icon_drag_event::IconDragEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QIconDragEvent_G_static_cast_QIconDragEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::icon_drag_event::IconDragEvent {
    let ffi_result = ::ffi::qt_gui_c_QIconDragEvent_G_static_cast_QIconDragEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::icon_drag_event::IconDragEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QIconDragEvent_G_static_cast_QEvent_ptr(self as *const ::icon_drag_event::IconDragEvent as *mut ::icon_drag_event::IconDragEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::icon_drag_event::IconDragEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QIconDragEvent_G_static_cast_QEvent_ptr(self as *mut ::icon_drag_event::IconDragEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
