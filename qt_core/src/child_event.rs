/// C++ type: <span style='color: green;'>```QChildEvent```</span>
#[repr(C)]
pub struct ChildEvent(u8);

impl ChildEvent {
  /// C++ method: <span style='color: green;'>```bool QChildEvent::added() const```</span>
  ///
  ///
  pub fn added(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChildEvent_added(self as *const ::child_event::ChildEvent) }
  }

  /// C++ method: <span style='color: green;'>```QObject* QChildEvent::child() const```</span>
  ///
  ///
  pub fn child(&self) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QChildEvent_child(self as *const ::child_event::ChildEvent) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QChildEvent::QChildEvent(QEvent::Type type, QObject* child)```</span>
  ///
  ///
  pub unsafe fn new(type_: ::event::Type,
                    child: *mut ::object::Object)
                    -> ::cpp_utils::CppBox<::child_event::ChildEvent> {
    let ffi_result = ::ffi::qt_core_c_QChildEvent_new(type_, child);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```bool QChildEvent::polished() const```</span>
  ///
  ///
  pub fn polished(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChildEvent_polished(self as *const ::child_event::ChildEvent) }
  }

  /// C++ method: <span style='color: green;'>```bool QChildEvent::removed() const```</span>
  ///
  ///
  pub fn removed(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QChildEvent_removed(self as *const ::child_event::ChildEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::child_event::ChildEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QChildEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::child_event::ChildEvent> for ::event::Event {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::child_event::ChildEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QChildEvent_G_dynamic_cast_QChildEvent_ptr(self as *mut ::event::Event) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::child_event::ChildEvent> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QChildEvent_G_dynamic_cast_QChildEvent_ptr(self as *const ::event::Event as *mut ::event::Event) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::event::Event> for ::child_event::ChildEvent {
  fn static_cast_mut(&mut self) -> &mut ::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QChildEvent_G_static_cast_QEvent_ptr(self as *mut ::child_event::ChildEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QChildEvent_G_static_cast_QEvent_ptr(self as *const ::child_event::ChildEvent as *mut ::child_event::ChildEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::child_event::ChildEvent> for ::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::child_event::ChildEvent {
    let ffi_result = ::ffi::qt_core_c_QChildEvent_G_static_cast_QChildEvent_ptr(self as *mut ::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::child_event::ChildEvent {
    let ffi_result = ::ffi::qt_core_c_QChildEvent_G_static_cast_QChildEvent_ptr(self as *const ::event::Event as *mut ::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::child_event::ChildEvent {
  type Target = ::event::Event;
  fn deref(&self) -> &::event::Event {
    let ffi_result = unsafe { ::ffi::qt_core_c_QChildEvent_G_static_cast_QEvent_ptr(self as *const ::child_event::ChildEvent as *mut ::child_event::ChildEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::child_event::ChildEvent {
  fn deref_mut(&mut self) -> &mut ::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QChildEvent_G_static_cast_QEvent_ptr(self as *mut ::child_event::ChildEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
