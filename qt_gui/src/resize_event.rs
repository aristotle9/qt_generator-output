/// C++ type: <span style='color: green;'>```QResizeEvent```</span>
#[repr(C)]
pub struct ResizeEvent(u8);

impl ResizeEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QResizeEvent::QResizeEvent(const QSize& size, const QSize& oldSize)```</span>
  ///
  ///
  pub fn new(size: &::qt_core::size::Size,
             old_size: &::qt_core::size::Size)
             -> ::cpp_utils::CppBox<::resize_event::ResizeEvent> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QResizeEvent_new(size as *const ::qt_core::size::Size,
                                       old_size as *const ::qt_core::size::Size)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QResizeEvent::oldSize() const```</span>
  ///
  ///
  pub fn old_size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QResizeEvent_oldSize(self as *const ::resize_event::ResizeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSize& QResizeEvent::size() const```</span>
  ///
  ///
  pub fn size<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QResizeEvent_size(self as *const ::resize_event::ResizeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::resize_event::ResizeEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QResizeEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::resize_event::ResizeEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QResizeEvent_G_static_cast_QEvent_ptr(self as *mut ::resize_event::ResizeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QResizeEvent_G_static_cast_QEvent_ptr(self as *const ::resize_event::ResizeEvent as *mut ::resize_event::ResizeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::resize_event::ResizeEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::resize_event::ResizeEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QResizeEvent_G_static_cast_QResizeEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::resize_event::ResizeEvent {
    let ffi_result = ::ffi::qt_gui_c_QResizeEvent_G_static_cast_QResizeEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::resize_event::ResizeEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QResizeEvent_G_static_cast_QEvent_ptr(self as *const ::resize_event::ResizeEvent as *mut ::resize_event::ResizeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::resize_event::ResizeEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QResizeEvent_G_static_cast_QEvent_ptr(self as *mut ::resize_event::ResizeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
