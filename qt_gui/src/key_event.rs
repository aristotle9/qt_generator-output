/// C++ type: <span style='color: green;'>```QKeyEvent```</span>
#[repr(C)]
pub struct KeyEvent(u8);

impl KeyEvent {
  /// C++ method: <span style='color: green;'>```int QKeyEvent::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QKeyEvent_count(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```bool QKeyEvent::isAutoRepeat() const```</span>
  ///
  ///
  pub fn is_auto_repeat(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QKeyEvent_isAutoRepeat(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```int QKeyEvent::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QKeyEvent_key(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```bool QKeyEvent::matches(QKeySequence::StandardKey key) const```</span>
  ///
  ///
  pub fn matches(&self, key: &::key_sequence::StandardKey) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QKeyEvent_matches(self as *const ::key_event::KeyEvent,
                                        key as *const ::key_sequence::StandardKey)
    }
  }

  /// C++ method: <span style='color: green;'>```quint32 QKeyEvent::nativeModifiers() const```</span>
  ///
  ///
  pub fn native_modifiers(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QKeyEvent_nativeModifiers(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```quint32 QKeyEvent::nativeScanCode() const```</span>
  ///
  ///
  pub fn native_scan_code(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QKeyEvent_nativeScanCode(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```quint32 QKeyEvent::nativeVirtualKey() const```</span>
  ///
  ///
  pub fn native_virtual_key(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QKeyEvent_nativeVirtualKey(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```QString QKeyEvent::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QKeyEvent_text_to_output(self as *const ::key_event::KeyEvent, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::key_event::KeyEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QKeyEvent_delete
  }
}

impl ::cpp_utils::DynamicCast<::key_event::KeyEvent> for ::input_event::InputEvent {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::key_event::KeyEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QKeyEvent_G_dynamic_cast_QKeyEvent_ptr(self as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::key_event::KeyEvent> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QKeyEvent_G_dynamic_cast_QKeyEvent_ptr(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::key_event::KeyEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QKeyEvent_G_static_cast_QEvent_ptr(self as *mut ::key_event::KeyEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QKeyEvent_G_static_cast_QEvent_ptr(self as *const ::key_event::KeyEvent as *mut ::key_event::KeyEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::input_event::InputEvent> for ::key_event::KeyEvent {
  fn static_cast_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QKeyEvent_G_static_cast_QInputEvent_ptr(self as *mut ::key_event::KeyEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QKeyEvent_G_static_cast_QInputEvent_ptr(self as *const ::key_event::KeyEvent as *mut ::key_event::KeyEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::key_event::KeyEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::key_event::KeyEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QKeyEvent_G_static_cast_QKeyEvent_ptr_QEvent(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::key_event::KeyEvent {
    let ffi_result = ::ffi::qt_gui_c_QKeyEvent_G_static_cast_QKeyEvent_ptr_QEvent(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::key_event::KeyEvent> for ::input_event::InputEvent {
  unsafe fn static_cast_mut(&mut self) -> &mut ::key_event::KeyEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QKeyEvent_G_static_cast_QKeyEvent_ptr_QInputEvent(self as *mut ::input_event::InputEvent);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::key_event::KeyEvent {
    let ffi_result = ::ffi::qt_gui_c_QKeyEvent_G_static_cast_QKeyEvent_ptr_QInputEvent(self as *const ::input_event::InputEvent as *mut ::input_event::InputEvent);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::key_event::KeyEvent {
  type Target = ::input_event::InputEvent;
  fn deref(&self) -> &::input_event::InputEvent {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QKeyEvent_G_static_cast_QInputEvent_ptr(self as *const ::key_event::KeyEvent as *mut ::key_event::KeyEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::key_event::KeyEvent {
  fn deref_mut(&mut self) -> &mut ::input_event::InputEvent {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QKeyEvent_G_static_cast_QInputEvent_ptr(self as *mut ::key_event::KeyEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
