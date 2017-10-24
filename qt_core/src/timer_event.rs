/// C++ type: <span style='color: green;'>```QTimerEvent```</span>
#[repr(C)]
pub struct TimerEvent(u8);

impl TimerEvent {
  /// C++ method: <span style='color: green;'>```[constructor] void QTimerEvent::QTimerEvent(int timerId)```</span>
  ///
  ///
  pub fn new(timer_id: ::libc::c_int) -> ::cpp_utils::CppBox<::timer_event::TimerEvent> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimerEvent_new(timer_id) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```int QTimerEvent::timerId() const```</span>
  ///
  ///
  pub fn timer_id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QTimerEvent_timerId(self as *const ::timer_event::TimerEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::timer_event::TimerEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QTimerEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::timer_event::TimerEvent> for ::event::Event {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::timer_event::TimerEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTimerEvent_G_dynamic_cast_QTimerEvent_ptr(self as *mut ::event::Event) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::timer_event::TimerEvent> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimerEvent_G_dynamic_cast_QTimerEvent_ptr(self as *const ::event::Event as *mut ::event::Event) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::event::Event> for ::timer_event::TimerEvent {
  fn static_cast_mut(&mut self) -> &mut ::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTimerEvent_G_static_cast_QEvent_ptr(self as *mut ::timer_event::TimerEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimerEvent_G_static_cast_QEvent_ptr(self as *const ::timer_event::TimerEvent as *mut ::timer_event::TimerEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::timer_event::TimerEvent> for ::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::timer_event::TimerEvent {
    let ffi_result = ::ffi::qt_core_c_QTimerEvent_G_static_cast_QTimerEvent_ptr(self as *mut ::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::timer_event::TimerEvent {
    let ffi_result = ::ffi::qt_core_c_QTimerEvent_G_static_cast_QTimerEvent_ptr(self as *const ::event::Event as *mut ::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::timer_event::TimerEvent {
  type Target = ::event::Event;
  fn deref(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QTimerEvent_G_static_cast_QEvent_ptr(self as *const ::timer_event::TimerEvent as *mut ::timer_event::TimerEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::timer_event::TimerEvent {
  fn deref_mut(&mut self) -> &mut ::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QTimerEvent_G_static_cast_QEvent_ptr(self as *mut ::timer_event::TimerEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
