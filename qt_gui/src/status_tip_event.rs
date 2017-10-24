/// C++ type: <span style='color: green;'>```QStatusTipEvent```</span>
#[repr(C)]
pub struct StatusTipEvent(u8);

impl StatusTipEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QStatusTipEvent::QStatusTipEvent(const QString& tip)```</span>
  ///
  ///
  pub fn new(tip: &::qt_core::string::String) -> ::cpp_utils::CppBox<::status_tip_event::StatusTipEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QStatusTipEvent_new(tip as *const ::qt_core::string::String) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QString QStatusTipEvent::tip() const```</span>
  ///
  ///
  pub fn tip(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QStatusTipEvent_tip_to_output(self as *const ::status_tip_event::StatusTipEvent,
                                                      &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::status_tip_event::StatusTipEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QStatusTipEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::status_tip_event::StatusTipEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QStatusTipEvent_G_static_cast_QEvent_ptr(self as *mut ::status_tip_event::StatusTipEvent)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QStatusTipEvent_G_static_cast_QEvent_ptr(self as *const ::status_tip_event::StatusTipEvent as *mut ::status_tip_event::StatusTipEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::status_tip_event::StatusTipEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::status_tip_event::StatusTipEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QStatusTipEvent_G_static_cast_QStatusTipEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::status_tip_event::StatusTipEvent {
    let ffi_result = ::ffi::qt_gui_c_QStatusTipEvent_G_static_cast_QStatusTipEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::status_tip_event::StatusTipEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QStatusTipEvent_G_static_cast_QEvent_ptr(self as *const ::status_tip_event::StatusTipEvent as *mut ::status_tip_event::StatusTipEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::status_tip_event::StatusTipEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QStatusTipEvent_G_static_cast_QEvent_ptr(self as *mut ::status_tip_event::StatusTipEvent)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
