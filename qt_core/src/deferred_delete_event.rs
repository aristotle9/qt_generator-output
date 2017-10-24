/// C++ type: <span style='color: green;'>```QDeferredDeleteEvent```</span>
#[repr(C)]
pub struct DeferredDeleteEvent(u8);

impl DeferredDeleteEvent {
  /// C++ method: <span style='color: green;'>```int QDeferredDeleteEvent::loopLevel() const```</span>
  ///
  ///
  pub fn loop_level(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QDeferredDeleteEvent_loopLevel(self as *const ::deferred_delete_event::DeferredDeleteEvent)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QDeferredDeleteEvent::QDeferredDeleteEvent()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::deferred_delete_event::DeferredDeleteEvent> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDeferredDeleteEvent_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl ::cpp_utils::CppDeletable for ::deferred_delete_event::DeferredDeleteEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QDeferredDeleteEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::deferred_delete_event::DeferredDeleteEvent> for ::event::Event {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::deferred_delete_event::DeferredDeleteEvent> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QDeferredDeleteEvent_G_dynamic_cast_QDeferredDeleteEvent_ptr(self as *mut ::event::Event)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::deferred_delete_event::DeferredDeleteEvent> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDeferredDeleteEvent_G_dynamic_cast_QDeferredDeleteEvent_ptr(self as *const ::event::Event as *mut ::event::Event) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::event::Event> for ::deferred_delete_event::DeferredDeleteEvent {
  fn static_cast_mut(&mut self) -> &mut ::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDeferredDeleteEvent_G_static_cast_QEvent_ptr(self as *mut ::deferred_delete_event::DeferredDeleteEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDeferredDeleteEvent_G_static_cast_QEvent_ptr(self as *const ::deferred_delete_event::DeferredDeleteEvent as *mut ::deferred_delete_event::DeferredDeleteEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::deferred_delete_event::DeferredDeleteEvent> for ::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::deferred_delete_event::DeferredDeleteEvent {
    let ffi_result =
      ::ffi::qt_core_c_QDeferredDeleteEvent_G_static_cast_QDeferredDeleteEvent_ptr(self as *mut ::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::deferred_delete_event::DeferredDeleteEvent {
    let ffi_result = ::ffi::qt_core_c_QDeferredDeleteEvent_G_static_cast_QDeferredDeleteEvent_ptr(self as *const ::event::Event as *mut ::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::deferred_delete_event::DeferredDeleteEvent {
  type Target = ::event::Event;
  fn deref(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDeferredDeleteEvent_G_static_cast_QEvent_ptr(self as *const ::deferred_delete_event::DeferredDeleteEvent as *mut ::deferred_delete_event::DeferredDeleteEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::deferred_delete_event::DeferredDeleteEvent {
  fn deref_mut(&mut self) -> &mut ::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QDeferredDeleteEvent_G_static_cast_QEvent_ptr(self as *mut ::deferred_delete_event::DeferredDeleteEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
