/// C++ type: <span style='color: green;'>```Qt3DInput::QKeyboardHandler```</span>
#[repr(C)]
pub struct KeyboardHandler(u8);

impl KeyboardHandler {
  /// C++ method: <span style='color: green;'>```bool Qt3DInput::QKeyboardHandler::focus() const```</span>
  ///
  ///
  pub fn focus(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_focus(self as *const ::keyboard_handler::KeyboardHandler) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QKeyboardHandler::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_metaObject(self as *const ::keyboard_handler::KeyboardHandler)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QKeyboardHandler::QKeyboardHandler()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::keyboard_handler::KeyboardHandler> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QKeyboardHandler::QKeyboardHandler(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::keyboard_handler::KeyboardHandler> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QKeyboardHandler::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_qt_metacall(self as *mut ::keyboard_handler::KeyboardHandler,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QKeyboardHandler::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_qt_metacast(self as *mut ::keyboard_handler::KeyboardHandler, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QKeyboardHandler::setFocus(bool focus)```</span>
  ///
  ///
  pub fn set_focus(&mut self, focus: bool) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_setFocus(self as *mut ::keyboard_handler::KeyboardHandler, focus)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QKeyboardHandler::setSourceDevice(Qt3DInput::QKeyboardDevice* keyboardDevice)```</span>
  ///
  ///
  pub unsafe fn set_source_device(&mut self, keyboard_device: *mut ::keyboard_device::KeyboardDevice) {
    ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_setSourceDevice(self as *mut ::keyboard_handler::KeyboardHandler,
                                                                    keyboard_device)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QKeyboardDevice* Qt3DInput::QKeyboardHandler::sourceDevice() const```</span>
  ///
  ///
  pub fn source_device(&self) -> *mut ::keyboard_device::KeyboardDevice {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_sourceDevice(self as *const ::keyboard_handler::KeyboardHandler)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QKeyboardHandler::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QKeyboardHandler::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::keyboard_handler::KeyboardHandler {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QKeyboardHandler_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `KeyboardHandler`.
  pub struct Signals<'a>(&'a ::keyboard_handler::KeyboardHandler);
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::released`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().released()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Released<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Released<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2released(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Released<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit2Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().digit2_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Digit2Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Digit2Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2digit2Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Digit2Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::context3Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().context3_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Context3Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Context3Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2context3Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Context3Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::asteriskPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().asterisk_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct AsteriskPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for AsteriskPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2asteriskPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AsteriskPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::leftPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().left_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct LeftPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for LeftPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2leftPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LeftPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::rightPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().right_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct RightPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for RightPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2rightPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RightPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::flipPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().flip_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct FlipPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for FlipPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2flipPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FlipPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit5Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().digit5_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Digit5Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Digit5Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2digit5Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Digit5Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit9Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().digit9_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Digit9Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Digit9Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2digit9Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Digit9Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::cancelPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().cancel_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct CancelPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for CancelPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cancelPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CancelPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::removedFromEntity`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct RemovedFromEntity<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for RemovedFromEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2removedFromEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RemovedFromEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit8Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().digit8_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Digit8Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Digit8Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2digit8Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Digit8Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::context2Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().context2_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Context2Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Context2Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2context2Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Context2Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit3Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().digit3_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Digit3Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Digit3Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2digit3Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Digit3Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::numberSignPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().number_sign_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct NumberSignPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for NumberSignPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2numberSignPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NumberSignPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::addedToEntity`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct AddedToEntity<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for AddedToEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2addedToEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AddedToEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::upPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().up_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct UpPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for UpPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2upPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for UpPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::backPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().back_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct BackPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for BackPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2backPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BackPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit7Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().digit7_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Digit7Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Digit7Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2digit7Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Digit7Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::context4Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().context4_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Context4Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Context4Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2context4Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Context4Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::backtabPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().backtab_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct BacktabPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for BacktabPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2backtabPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BacktabPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::deletePressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().delete_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct DeletePressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for DeletePressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2deletePressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DeletePressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::yesPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().yes_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct YesPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for YesPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2yesPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for YesPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::context1Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().context1_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Context1Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Context1Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2context1Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Context1Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::downPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().down_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct DownPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for DownPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2downPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DownPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit1Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().digit1_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Digit1Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Digit1Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2digit1Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Digit1Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::escapePressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().escape_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct EscapePressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for EscapePressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2escapePressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EscapePressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit4Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().digit4_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Digit4Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Digit4Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2digit4Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Digit4Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::menuPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().menu_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct MenuPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for MenuPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2menuPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MenuPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::noPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().no_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct NoPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for NoPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2noPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NoPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::volumeUpPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().volume_up_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct VolumeUpPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for VolumeUpPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2volumeUpPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VolumeUpPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::selectPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().select_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct SelectPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for SelectPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2selectPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SelectPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::spacePressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().space_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct SpacePressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for SpacePressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2spacePressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SpacePressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit6Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().digit6_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Digit6Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Digit6Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2digit6Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Digit6Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::shareableChanged`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct ShareableChanged<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for ShareableChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2shareableChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ShareableChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::callPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().call_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct CallPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for CallPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2callPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CallPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::enterPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().enter_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct EnterPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for EnterPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2enterPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EnterPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::tabPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().tab_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct TabPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for TabPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2tabPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TabPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::volumeDownPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().volume_down_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct VolumeDownPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for VolumeDownPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2volumeDownPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VolumeDownPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::focusChanged`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().focus_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct FocusChanged<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for FocusChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2focusChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FocusChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::hangupPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().hangup_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct HangupPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for HangupPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2hangupPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HangupPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit0Pressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().digit0_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct Digit0Pressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for Digit0Pressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2digit0Pressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Digit0Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::returnPressed`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().return_pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct ReturnPressed<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for ReturnPressed<'a> {
    type Arguments = (*mut ::key_event::KeyEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2returnPressed(Qt3DInput::QKeyEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ReturnPressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QKeyboardHandler::sourceDeviceChanged`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.signals().source_device_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct SourceDeviceChanged<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for SourceDeviceChanged<'a> {
    type Arguments = (*mut ::keyboard_device::KeyboardDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceDeviceChanged(Qt3DInput::QKeyboardDevice*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceDeviceChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::released`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn released(&self) -> Released {
      Released(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit2Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn digit2_pressed(&self) -> Digit2Pressed {
      Digit2Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::context3Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn context3_pressed(&self) -> Context3Pressed {
      Context3Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::asteriskPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn asterisk_pressed(&self) -> AsteriskPressed {
      AsteriskPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::leftPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn left_pressed(&self) -> LeftPressed {
      LeftPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::rightPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn right_pressed(&self) -> RightPressed {
      RightPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::flipPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn flip_pressed(&self) -> FlipPressed {
      FlipPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit5Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn digit5_pressed(&self) -> Digit5Pressed {
      Digit5Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit9Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn digit9_pressed(&self) -> Digit9Pressed {
      Digit9Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::cancelPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn cancel_pressed(&self) -> CancelPressed {
      CancelPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit8Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn digit8_pressed(&self) -> Digit8Pressed {
      Digit8Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::context2Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn context2_pressed(&self) -> Context2Pressed {
      Context2Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit3Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn digit3_pressed(&self) -> Digit3Pressed {
      Digit3Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::numberSignPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn number_sign_pressed(&self) -> NumberSignPressed {
      NumberSignPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::upPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn up_pressed(&self) -> UpPressed {
      UpPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::backPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn back_pressed(&self) -> BackPressed {
      BackPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit7Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn digit7_pressed(&self) -> Digit7Pressed {
      Digit7Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::context4Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn context4_pressed(&self) -> Context4Pressed {
      Context4Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::backtabPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn backtab_pressed(&self) -> BacktabPressed {
      BacktabPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::deletePressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn delete_pressed(&self) -> DeletePressed {
      DeletePressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::yesPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn yes_pressed(&self) -> YesPressed {
      YesPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::context1Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn context1_pressed(&self) -> Context1Pressed {
      Context1Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::downPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn down_pressed(&self) -> DownPressed {
      DownPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit1Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn digit1_pressed(&self) -> Digit1Pressed {
      Digit1Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::escapePressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn escape_pressed(&self) -> EscapePressed {
      EscapePressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit4Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn digit4_pressed(&self) -> Digit4Pressed {
      Digit4Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::menuPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn menu_pressed(&self) -> MenuPressed {
      MenuPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::noPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn no_pressed(&self) -> NoPressed {
      NoPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::volumeUpPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn volume_up_pressed(&self) -> VolumeUpPressed {
      VolumeUpPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::selectPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn select_pressed(&self) -> SelectPressed {
      SelectPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::spacePressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn space_pressed(&self) -> SpacePressed {
      SpacePressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit6Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn digit6_pressed(&self) -> Digit6Pressed {
      Digit6Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::callPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn call_pressed(&self) -> CallPressed {
      CallPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::enterPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enter_pressed(&self) -> EnterPressed {
      EnterPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::tabPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tab_pressed(&self) -> TabPressed {
      TabPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::volumeDownPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn volume_down_pressed(&self) -> VolumeDownPressed {
      VolumeDownPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::focusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn focus_changed(&self) -> FocusChanged {
      FocusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::hangupPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn hangup_pressed(&self) -> HangupPressed {
      HangupPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::digit0Pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn digit0_pressed(&self) -> Digit0Pressed {
      Digit0Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::returnPressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn return_pressed(&self) -> ReturnPressed {
      ReturnPressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QKeyboardHandler::sourceDeviceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_device_changed(&self) -> SourceDeviceChanged {
      SourceDeviceChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `KeyboardHandler`.
  pub struct Slots<'a>(&'a ::keyboard_handler::KeyboardHandler);
  /// Represents a built-in Qt slot `Qt3DInput::QKeyboardHandler::setSourceDevice`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.slots().set_source_device()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct SetSourceDevice<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for SetSourceDevice<'a> {
    type Arguments = (*mut ::keyboard_device::KeyboardDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceDevice(Qt3DInput::QKeyboardDevice*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QKeyboardHandler::setFocus`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.slots().set_focus()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct SetFocus<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for SetFocus<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFocus(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QKeyboardHandler::setShareable`.
  ///
  /// An object of this type can be created from `KeyboardHandler` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `KeyboardHandler` object.
  pub struct SetShareable<'a>(&'a ::keyboard_handler::KeyboardHandler);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QKeyboardHandler::setSourceDevice`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_device(&self) -> SetSourceDevice {
      SetSourceDevice(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QKeyboardHandler::setFocus`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_focus(&self) -> SetFocus {
      SetFocus(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QKeyboardHandler::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
  }
  impl ::keyboard_handler::KeyboardHandler {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
    /// Provides access to built-in Qt slots of this type
    pub fn slots(&self) -> Slots {
      Slots(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::keyboard_handler::KeyboardHandler {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::keyboard_handler::KeyboardHandler) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::keyboard_handler::KeyboardHandler as *mut ::keyboard_handler::KeyboardHandler) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::keyboard_handler::KeyboardHandler {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::keyboard_handler::KeyboardHandler) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::keyboard_handler::KeyboardHandler as *mut ::keyboard_handler::KeyboardHandler) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::keyboard_handler::KeyboardHandler {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_QObject_ptr(self as *mut ::keyboard_handler::KeyboardHandler) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_QObject_ptr(self as *const ::keyboard_handler::KeyboardHandler as *mut ::keyboard_handler::KeyboardHandler) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::keyboard_handler::KeyboardHandler> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::keyboard_handler::KeyboardHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DInput_QKeyboardHandler_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::keyboard_handler::KeyboardHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DInput_QKeyboardHandler_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::keyboard_handler::KeyboardHandler> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::keyboard_handler::KeyboardHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DInput_QKeyboardHandler_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::keyboard_handler::KeyboardHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DInput_QKeyboardHandler_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::keyboard_handler::KeyboardHandler> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::keyboard_handler::KeyboardHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DInput_QKeyboardHandler_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::keyboard_handler::KeyboardHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DInput_QKeyboardHandler_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::keyboard_handler::KeyboardHandler {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::keyboard_handler::KeyboardHandler as *mut ::keyboard_handler::KeyboardHandler) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::keyboard_handler::KeyboardHandler {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QKeyboardHandler_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::keyboard_handler::KeyboardHandler) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
