/// C++ type: <span style='color: green;'>```QExposeEvent```</span>
#[repr(C)]
pub struct ExposeEvent(u8);

impl ExposeEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QExposeEvent::QExposeEvent(const QRegion& rgn)```</span>
  ///
  ///
  pub fn new(rgn: &::region::Region) -> ::cpp_utils::CppBox<::expose_event::ExposeEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QExposeEvent_new(rgn as *const ::region::Region) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```const QRegion& QExposeEvent::region() const```</span>
  ///
  ///
  pub fn region<'l0>(&'l0 self) -> &'l0 ::region::Region {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QExposeEvent_region(self as *const ::expose_event::ExposeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::expose_event::ExposeEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QExposeEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::expose_event::ExposeEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QExposeEvent_G_static_cast_QEvent_ptr(self as *mut ::expose_event::ExposeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QExposeEvent_G_static_cast_QEvent_ptr(self as *const ::expose_event::ExposeEvent as *mut ::expose_event::ExposeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::expose_event::ExposeEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::expose_event::ExposeEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QExposeEvent_G_static_cast_QExposeEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::expose_event::ExposeEvent {
    let ffi_result = ::ffi::qt_gui_c_QExposeEvent_G_static_cast_QExposeEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::expose_event::ExposeEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QExposeEvent_G_static_cast_QEvent_ptr(self as *const ::expose_event::ExposeEvent as *mut ::expose_event::ExposeEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::expose_event::ExposeEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QExposeEvent_G_static_cast_QEvent_ptr(self as *mut ::expose_event::ExposeEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
