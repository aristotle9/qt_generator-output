/// C++ type: <span style='color: green;'>```QShortcutEvent```</span>
#[repr(C)]
pub struct ShortcutEvent(u8);

impl ShortcutEvent {
  /// C++ method: <span style='color: green;'>```bool QShortcutEvent::isAmbiguous() const```</span>
  ///
  ///
  pub fn is_ambiguous(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QShortcutEvent_isAmbiguous(self as *const ::shortcut_event::ShortcutEvent) }
  }

  /// C++ method: <span style='color: green;'>```const QKeySequence& QShortcutEvent::key() const```</span>
  ///
  ///
  pub fn key<'l0>(&'l0 self) -> &'l0 ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QShortcutEvent_key(self as *const ::shortcut_event::ShortcutEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QShortcutEvent::QShortcutEvent```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new((&::key_sequence::KeySequence, ::libc::c_int)) -> ::cpp_utils::CppBox<::shortcut_event::ShortcutEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QShortcutEvent::QShortcutEvent(const QKeySequence& key, int id)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::key_sequence::KeySequence, ::libc::c_int, bool)) -> ::cpp_utils::CppBox<::shortcut_event::ShortcutEvent>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QShortcutEvent::QShortcutEvent(const QKeySequence& key, int id, bool ambiguous = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::shortcut_event::ShortcutEvent>
    where Args: overloading::ShortcutEventNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QShortcutEvent::shortcutId() const```</span>
  ///
  ///
  pub fn shortcut_id(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QShortcutEvent_shortcutId(self as *const ::shortcut_event::ShortcutEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::shortcut_event::ShortcutEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QShortcutEvent_delete
  }
}

impl ::cpp_utils::StaticCast<::qt_core::event::Event> for ::shortcut_event::ShortcutEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QShortcutEvent_G_static_cast_QEvent_ptr(self as *mut ::shortcut_event::ShortcutEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QShortcutEvent_G_static_cast_QEvent_ptr(self as *const ::shortcut_event::ShortcutEvent as *mut ::shortcut_event::ShortcutEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::shortcut_event::ShortcutEvent> for ::qt_core::event::Event {
  unsafe fn static_cast_mut(&mut self) -> &mut ::shortcut_event::ShortcutEvent {
    let ffi_result =
      ::ffi::qt_gui_c_QShortcutEvent_G_static_cast_QShortcutEvent_ptr(self as *mut ::qt_core::event::Event);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::shortcut_event::ShortcutEvent {
    let ffi_result = ::ffi::qt_gui_c_QShortcutEvent_G_static_cast_QShortcutEvent_ptr(self as *const ::qt_core::event::Event as *mut ::qt_core::event::Event);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::shortcut_event::ShortcutEvent {
  type Target = ::qt_core::event::Event;
  fn deref(&self) -> &::qt_core::event::Event {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QShortcutEvent_G_static_cast_QEvent_ptr(self as *const ::shortcut_event::ShortcutEvent as *mut ::shortcut_event::ShortcutEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::shortcut_event::ShortcutEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::event::Event {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QShortcutEvent_G_static_cast_QEvent_ptr(self as *mut ::shortcut_event::ShortcutEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ShortcutEvent::new](../struct.ShortcutEvent.html#method.new) method.
  pub trait ShortcutEventNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::shortcut_event::ShortcutEvent>;
  }
  impl<'a> ShortcutEventNewArgs for (&'a ::key_sequence::KeySequence, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::shortcut_event::ShortcutEvent> {
      let key = self.0;
      let id = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QShortcutEvent_new_key_id(key as *const ::key_sequence::KeySequence, id) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> ShortcutEventNewArgs for (&'a ::key_sequence::KeySequence, ::libc::c_int, bool) {
    fn exec(self) -> ::cpp_utils::CppBox<::shortcut_event::ShortcutEvent> {
      let key = self.0;
      let id = self.1;
      let ambiguous = self.2;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QShortcutEvent_new_key_id_ambiguous(key as *const ::key_sequence::KeySequence, id, ambiguous)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
