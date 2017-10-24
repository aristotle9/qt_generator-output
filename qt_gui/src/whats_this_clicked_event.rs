/// C++ type: <span style='color: green;'>```QWhatsThisClickedEvent```</span>
#[repr(C)]
pub struct WhatsThisClickedEvent(u8);

impl WhatsThisClickedEvent {
  /// C++ method: <span style='color: green;'>```QString QWhatsThisClickedEvent::href() const```</span>
  ///
  ///
  pub fn href(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QWhatsThisClickedEvent_href_to_output(self as *const ::whats_this_clicked_event::WhatsThisClickedEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QWhatsThisClickedEvent::QWhatsThisClickedEvent(const QString& href)```</span>
  ///
  ///
  pub fn new(href: &::qt_core::string::String)
             -> ::cpp_utils::CppBox<::whats_this_clicked_event::WhatsThisClickedEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWhatsThisClickedEvent_new(href as *const ::qt_core::string::String) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::whats_this_clicked_event::WhatsThisClickedEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QWhatsThisClickedEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::whats_this_clicked_event::WhatsThisClickedEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWhatsThisClickedEvent_G_static_cast_QEvent_ptr(self as *mut ::whats_this_clicked_event::WhatsThisClickedEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWhatsThisClickedEvent_G_static_cast_QEvent_ptr(self as *const ::whats_this_clicked_event::WhatsThisClickedEvent as *mut ::whats_this_clicked_event::WhatsThisClickedEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::whats_this_clicked_event::WhatsThisClickedEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::whats_this_clicked_event::WhatsThisClickedEvent {
    let ffi_result = ::ffi::qt_gui_c_QWhatsThisClickedEvent_G_static_cast_QWhatsThisClickedEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::whats_this_clicked_event::WhatsThisClickedEvent {
    let ffi_result = ::ffi::qt_gui_c_QWhatsThisClickedEvent_G_static_cast_QWhatsThisClickedEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::whats_this_clicked_event::WhatsThisClickedEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWhatsThisClickedEvent_G_static_cast_QEvent_ptr(self as *const ::whats_this_clicked_event::WhatsThisClickedEvent as *mut ::whats_this_clicked_event::WhatsThisClickedEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::whats_this_clicked_event::WhatsThisClickedEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QWhatsThisClickedEvent_G_static_cast_QEvent_ptr(self as *mut ::whats_this_clicked_event::WhatsThisClickedEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
