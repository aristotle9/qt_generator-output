/// C++ type: <span style='color: green;'>```Qt3DInput::QMouseHandler```</span>
#[repr(C)]
pub struct MouseHandler(u8);

impl MouseHandler {
  /// C++ method: <span style='color: green;'>```bool Qt3DInput::QMouseHandler::containsMouse() const```</span>
  ///
  ///
  pub fn contains_mouse(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseHandler_containsMouse(self as *const ::mouse_handler::MouseHandler) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QMouseHandler::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseHandler_metaObject(self as *const ::mouse_handler::MouseHandler) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QMouseHandler::QMouseHandler()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::mouse_handler::MouseHandler> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseHandler_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QMouseHandler::QMouseHandler(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::mouse_handler::MouseHandler> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QMouseHandler_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QMouseHandler::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QMouseHandler_qt_metacall(self as *mut ::mouse_handler::MouseHandler,
                                                             arg1 as *const ::qt_core::meta_object::Call,
                                                             arg2,
                                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QMouseHandler::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QMouseHandler_qt_metacast(self as *mut ::mouse_handler::MouseHandler, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QMouseHandler::setSourceDevice(Qt3DInput::QMouseDevice* mouseDevice)```</span>
  ///
  ///
  pub unsafe fn set_source_device(&mut self, mouse_device: *mut ::mouse_device::MouseDevice) {
    ::ffi::qt_3d_input_c_Qt3DInput_QMouseHandler_setSourceDevice(self as *mut ::mouse_handler::MouseHandler,
                                                                 mouse_device)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QMouseDevice* Qt3DInput::QMouseHandler::sourceDevice() const```</span>
  ///
  ///
  pub fn source_device(&self) -> *mut ::mouse_device::MouseDevice {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QMouseHandler_sourceDevice(self as *const ::mouse_handler::MouseHandler) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QMouseHandler::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QMouseHandler_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QMouseHandler::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QMouseHandler_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::mouse_handler::MouseHandler {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QMouseHandler_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `MouseHandler`.
  pub struct Signals<'a>(&'a ::mouse_handler::MouseHandler);
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::released`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().released()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct Released<'a>(&'a ::mouse_handler::MouseHandler);
  impl<'a> ::qt_core::connection::Receiver for Released<'a> {
    type Arguments = (*mut ::mouse_event::MouseEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2released(Qt3DInput::QMouseEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Released<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::shareableChanged`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct ShareableChanged<'a>(&'a ::mouse_handler::MouseHandler);
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
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::pressed`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().pressed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct Pressed<'a>(&'a ::mouse_handler::MouseHandler);
  impl<'a> ::qt_core::connection::Receiver for Pressed<'a> {
    type Arguments = (*mut ::mouse_event::MouseEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pressed(Qt3DInput::QMouseEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Pressed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::removedFromEntity`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct RemovedFromEntity<'a>(&'a ::mouse_handler::MouseHandler);
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
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::doubleClicked`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().double_clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct DoubleClicked<'a>(&'a ::mouse_handler::MouseHandler);
  impl<'a> ::qt_core::connection::Receiver for DoubleClicked<'a> {
    type Arguments = (*mut ::mouse_event::MouseEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2doubleClicked(Qt3DInput::QMouseEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DoubleClicked<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::sourceDeviceChanged`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().source_device_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct SourceDeviceChanged<'a>(&'a ::mouse_handler::MouseHandler);
  impl<'a> ::qt_core::connection::Receiver for SourceDeviceChanged<'a> {
    type Arguments = (*mut ::mouse_device::MouseDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceDeviceChanged(Qt3DInput::QMouseDevice*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceDeviceChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::exited`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().exited()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct Exited<'a>(&'a ::mouse_handler::MouseHandler);
  impl<'a> ::qt_core::connection::Receiver for Exited<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2exited()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Exited<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::entered`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().entered()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct Entered<'a>(&'a ::mouse_handler::MouseHandler);
  impl<'a> ::qt_core::connection::Receiver for Entered<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2entered()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Entered<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::positionChanged`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().position_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct PositionChanged<'a>(&'a ::mouse_handler::MouseHandler);
  impl<'a> ::qt_core::connection::Receiver for PositionChanged<'a> {
    type Arguments = (*mut ::mouse_event::MouseEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2positionChanged(Qt3DInput::QMouseEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PositionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::addedToEntity`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct AddedToEntity<'a>(&'a ::mouse_handler::MouseHandler);
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
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::pressAndHold`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().press_and_hold()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct PressAndHold<'a>(&'a ::mouse_handler::MouseHandler);
  impl<'a> ::qt_core::connection::Receiver for PressAndHold<'a> {
    type Arguments = (*mut ::mouse_event::MouseEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pressAndHold(Qt3DInput::QMouseEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PressAndHold<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::clicked`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().clicked()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct Clicked<'a>(&'a ::mouse_handler::MouseHandler);
  impl<'a> ::qt_core::connection::Receiver for Clicked<'a> {
    type Arguments = (*mut ::mouse_event::MouseEvent,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2clicked(Qt3DInput::QMouseEvent*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Clicked<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QMouseHandler::containsMouseChanged`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.signals().contains_mouse_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct ContainsMouseChanged<'a>(&'a ::mouse_handler::MouseHandler);
  impl<'a> ::qt_core::connection::Receiver for ContainsMouseChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2containsMouseChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ContainsMouseChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::released`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn released(&self) -> Released {
      Released(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::pressed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pressed(&self) -> Pressed {
      Pressed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::doubleClicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn double_clicked(&self) -> DoubleClicked {
      DoubleClicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::sourceDeviceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_device_changed(&self) -> SourceDeviceChanged {
      SourceDeviceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::exited`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn exited(&self) -> Exited {
      Exited(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::entered`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn entered(&self) -> Entered {
      Entered(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::positionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn position_changed(&self) -> PositionChanged {
      PositionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::pressAndHold`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn press_and_hold(&self) -> PressAndHold {
      PressAndHold(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::clicked`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn clicked(&self) -> Clicked {
      Clicked(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QMouseHandler::containsMouseChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn contains_mouse_changed(&self) -> ContainsMouseChanged {
      ContainsMouseChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `MouseHandler`.
  pub struct Slots<'a>(&'a ::mouse_handler::MouseHandler);
  /// Represents a built-in Qt slot `Qt3DInput::QMouseHandler::setSourceDevice`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.slots().set_source_device()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct SetSourceDevice<'a>(&'a ::mouse_handler::MouseHandler);
  impl<'a> ::qt_core::connection::Receiver for SetSourceDevice<'a> {
    type Arguments = (*mut ::mouse_device::MouseDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceDevice(Qt3DInput::QMouseDevice*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QMouseHandler::setShareable`.
  ///
  /// An object of this type can be created from `MouseHandler` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `MouseHandler` object.
  pub struct SetShareable<'a>(&'a ::mouse_handler::MouseHandler);
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
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QMouseHandler::setSourceDevice`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_device(&self) -> SetSourceDevice {
      SetSourceDevice(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QMouseHandler::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
  }
  impl ::mouse_handler::MouseHandler {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::mouse_handler::MouseHandler {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::mouse_handler::MouseHandler) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::mouse_handler::MouseHandler as *mut ::mouse_handler::MouseHandler) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::mouse_handler::MouseHandler {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::mouse_handler::MouseHandler)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::mouse_handler::MouseHandler as *mut ::mouse_handler::MouseHandler) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::mouse_handler::MouseHandler {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_QObject_ptr(self as *mut ::mouse_handler::MouseHandler)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_QObject_ptr(self as *const ::mouse_handler::MouseHandler as *mut ::mouse_handler::MouseHandler) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mouse_handler::MouseHandler> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_handler::MouseHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mouse_handler::MouseHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mouse_handler::MouseHandler> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_handler::MouseHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mouse_handler::MouseHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mouse_handler::MouseHandler> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mouse_handler::MouseHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mouse_handler::MouseHandler {
    let ffi_result = ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DInput_QMouseHandler_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::mouse_handler::MouseHandler {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::mouse_handler::MouseHandler as *mut ::mouse_handler::MouseHandler) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::mouse_handler::MouseHandler {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QMouseHandler_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::mouse_handler::MouseHandler) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
