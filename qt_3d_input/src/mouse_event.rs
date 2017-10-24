/// C++ type: <span style='color: green;'>```Qt3DInput::QMouseEvent::Buttons```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Buttons {
  /// C++ enum variant: <span style='color: green;'>```NoButton = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```LeftButton = 1```</span>
  Left = 1,
  /// C++ enum variant: <span style='color: green;'>```RightButton = 2```</span>
  Right = 2,
  /// C++ enum variant: <span style='color: green;'>```MiddleButton = 4```</span>
  Middle = 4,
  /// C++ enum variant: <span style='color: green;'>```BackButton = 8```</span>
  Back = 8,
}

/// C++ type: <span style='color: green;'>```Qt3DInput::QMouseEvent::Modifiers```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Modifiers {
  /// C++ enum variant: <span style='color: green;'>```NoModifier = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```ShiftModifier = 33554432```</span>
  Shift = 33554432,
  /// C++ enum variant: <span style='color: green;'>```ControlModifier = 67108864```</span>
  Control = 67108864,
  /// C++ enum variant: <span style='color: green;'>```AltModifier = 134217728```</span>
  Alt = 134217728,
  /// C++ enum variant: <span style='color: green;'>```MetaModifier = 268435456```</span>
  Meta = 268435456,
  /// C++ enum variant: <span style='color: green;'>```KeypadModifier = 536870912```</span>
  Keypad = 536870912,
}

/// C++ type: <span style='color: green;'>```Qt3DInput::QMouseEvent```</span>
#[repr(C)]
pub struct MouseEvent(u8);

impl MouseEvent {
  /// C++ method: <span style='color: green;'>```Qt3DInput::QMouseEvent::Buttons Qt3DInput::QMouseEvent::button() const```</span>
  ///
  ///
  pub fn button(&self) -> ::mouse_event::Buttons {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_button(self as *const ::mouse_event::MouseEvent) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DInput::QMouseEvent::buttons() const```</span>
  ///
  ///
  pub fn buttons(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_buttons(self as *const ::mouse_event::MouseEvent) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DInput::QMouseEvent::isAccepted() const```</span>
  ///
  ///
  pub fn is_accepted(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_isAccepted(self as *const ::mouse_event::MouseEvent) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QMouseEvent::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_metaObject(self as *const ::mouse_event::MouseEvent) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QMouseEvent::Modifiers Qt3DInput::QMouseEvent::modifiers() const```</span>
  ///
  ///
  pub fn modifiers(&self) -> ::mouse_event::Modifiers {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_modifiers(self as *const ::mouse_event::MouseEvent) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QMouseEvent::QMouseEvent(const QMouseEvent& e)```</span>
  ///
  ///
  pub fn new(e: &::qt_gui::mouse_event::MouseEvent) -> ::cpp_utils::CppBox<::mouse_event::MouseEvent> {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_new(e as *const ::qt_gui::mouse_event::MouseEvent) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QMouseEvent::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_qt_metacall(self as *mut ::mouse_event::MouseEvent,
                                                           arg1 as *const ::qt_core::meta_object::Call,
                                                           arg2,
                                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QMouseEvent::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_qt_metacast(self as *mut ::mouse_event::MouseEvent, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DInput::QMouseEvent::setAccepted(bool accepted)```</span>
  ///
  ///
  pub fn set_accepted(&mut self, accepted: bool) {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_setAccepted(self as *mut ::mouse_event::MouseEvent, accepted) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QMouseEvent::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QMouseEvent::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DInput::QMouseEvent::wasHeld() const```</span>
  ///
  ///
  pub fn was_held(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_wasHeld(self as *const ::mouse_event::MouseEvent) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DInput::QMouseEvent::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_x(self as *const ::mouse_event::MouseEvent) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DInput::QMouseEvent::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_y(self as *const ::mouse_event::MouseEvent) }
  }
}

impl ::cpp_utils::CppDeletable for ::mouse_event::MouseEvent {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QMouseEvent_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `MouseEvent`.
  pub struct Signals<'a>(&'a ::mouse_event::MouseEvent);
  /// Represents a built-in Qt signal `Qt3DInput::QMouseEvent::objectNameChanged`.
  ///
  /// An object of this type can be created from `MouseEvent` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseEvent` object.
  pub struct ObjectNameChanged<'a>(&'a ::mouse_event::MouseEvent);
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
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseEvent::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::mouse_event::MouseEvent {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::mouse_event::MouseEvent {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QMouseEvent_G_static_cast_QObject_ptr(self as *mut ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseEvent_G_static_cast_QObject_ptr(self as *const ::mouse_event::MouseEvent as *mut ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mouse_event::MouseEvent> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_event::MouseEvent {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseEvent_G_static_cast_Qt3DInput_QMouseEvent_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mouse_event::MouseEvent {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseEvent_G_static_cast_Qt3DInput_QMouseEvent_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::mouse_event::MouseEvent {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseEvent_G_static_cast_QObject_ptr(self as *const ::mouse_event::MouseEvent as *mut ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::mouse_event::MouseEvent {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QMouseEvent_G_static_cast_QObject_ptr(self as *mut ::mouse_event::MouseEvent) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
