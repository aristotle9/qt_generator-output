/// C++ type: <span style='color: green;'>```Qt3DInput::QKeyEvent```</span>
#[repr(C)]
pub struct KeyEvent(u8);

impl KeyEvent {
  /// C++ method: <span style='color: green;'>```int Qt3DInput::QKeyEvent::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_count(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DInput::QKeyEvent::isAccepted() const```</span>
  ///
  ///
  pub fn is_accepted(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_isAccepted(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DInput::QKeyEvent::isAutoRepeat() const```</span>
  ///
  ///
  pub fn is_auto_repeat(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_isAutoRepeat(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DInput::QKeyEvent::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_key(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DInput::QKeyEvent::matches(QKeySequence::StandardKey key_) const```</span>
  ///
  ///
  pub fn matches(&self, key: &::qt_gui::key_sequence::StandardKey) -> bool {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_matches(self as *const ::key_event::KeyEvent,
                                                       key as *const ::qt_gui::key_sequence::StandardKey)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QKeyEvent::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_metaObject(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DInput::QKeyEvent::modifiers() const```</span>
  ///
  ///
  pub fn modifiers(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_modifiers(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```quint32 Qt3DInput::QKeyEvent::nativeScanCode() const```</span>
  ///
  ///
  pub fn native_scan_code(&self) -> u32 {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_nativeScanCode(self as *const ::key_event::KeyEvent) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QKeyEvent::QKeyEvent(const QKeyEvent& ke)```</span>
  ///
  ///
  pub fn new(ke: &::qt_gui::key_event::KeyEvent) -> ::cpp_utils::CppBox<::key_event::KeyEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_new(ke as *const ::qt_gui::key_event::KeyEvent) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QKeyEvent::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_qt_metacall(self as *mut ::key_event::KeyEvent,
                                                         arg1 as *const ::qt_core::meta_object::Call,
                                                         arg2,
                                                         arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QKeyEvent::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_qt_metacast(self as *mut ::key_event::KeyEvent, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DInput::QKeyEvent::setAccepted(bool accepted)```</span>
  ///
  ///
  pub fn set_accepted(&mut self, accepted: bool) {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_setAccepted(self as *mut ::key_event::KeyEvent, accepted) }
  }

  /// C++ method: <span style='color: green;'>```QString Qt3DInput::QKeyEvent::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_text_to_output(self as *const ::key_event::KeyEvent, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QKeyEvent::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QKeyEvent::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::key_event::KeyEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QKeyEvent_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `KeyEvent`.
  pub struct Signals<'a>(&'a ::key_event::KeyEvent);
  /// Represents a built-in Qt signal `Qt3DInput::QKeyEvent::objectNameChanged`.
  ///
  /// An object of this type can be created from `KeyEvent` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyEvent` object.
  pub struct ObjectNameChanged<'a>(&'a ::key_event::KeyEvent);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyEvent::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::key_event::KeyEvent {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::key_event::KeyEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QKeyEvent_G_static_cast_QObject_ptr(self as *mut ::key_event::KeyEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyEvent_G_static_cast_QObject_ptr(self as *const ::key_event::KeyEvent as *mut ::key_event::KeyEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::key_event::KeyEvent> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::key_event::KeyEvent {
    let ffi_result =
      ::ffi::qt_3d_input_c_QKeyEvent_G_static_cast_Qt3DInput_QKeyEvent_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::key_event::KeyEvent {
    let ffi_result = ::ffi::qt_3d_input_c_QKeyEvent_G_static_cast_Qt3DInput_QKeyEvent_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::key_event::KeyEvent {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyEvent_G_static_cast_QObject_ptr(self as *const ::key_event::KeyEvent as *mut ::key_event::KeyEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::key_event::KeyEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QKeyEvent_G_static_cast_QObject_ptr(self as *mut ::key_event::KeyEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
