/// C++ type: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput```</span>
#[repr(C)]
pub struct AbstractAxisInput(u8);

impl AbstractAxisInput {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QAbstractAxisInput::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAbstractAxisInput_metaObject(self as *const ::abstract_axis_input::AbstractAxisInput) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QAbstractAxisInput::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QAbstractAxisInput_qt_metacall(self as *mut ::abstract_axis_input::AbstractAxisInput, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QAbstractAxisInput::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QAbstractAxisInput_qt_metacast(self as *mut ::abstract_axis_input::AbstractAxisInput, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QAbstractAxisInput::setSourceDevice(Qt3DInput::QAbstractPhysicalDevice* sourceDevice)```</span>
  ///
  ///
  pub unsafe fn set_source_device(&mut self, source_device: *mut ::abstract_physical_device::AbstractPhysicalDevice) {
    ::ffi::qt_3d_input_c_Qt3DInput_QAbstractAxisInput_setSourceDevice(self as *mut ::abstract_axis_input::AbstractAxisInput, source_device)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractPhysicalDevice* Qt3DInput::QAbstractAxisInput::sourceDevice() const```</span>
  ///
  ///
  pub fn source_device(&self) -> *mut ::abstract_physical_device::AbstractPhysicalDevice {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAbstractAxisInput_sourceDevice(self as *const ::abstract_axis_input::AbstractAxisInput) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QAbstractAxisInput::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QAbstractAxisInput_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QAbstractAxisInput::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QAbstractAxisInput_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_axis_input::AbstractAxisInput {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QAbstractAxisInput_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractAxisInput`.
  pub struct Signals<'a>(&'a ::abstract_axis_input::AbstractAxisInput);
  /// Represents a built-in Qt signal `Qt3DInput::QAbstractAxisInput::nodeDestroyed`.
  ///
  /// An object of this type can be created from `AbstractAxisInput` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAxisInput` object.
  pub struct NodeDestroyed<'a>(&'a ::abstract_axis_input::AbstractAxisInput);
  impl<'a> ::qt_core::connection::Receiver for NodeDestroyed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2nodeDestroyed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NodeDestroyed<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAbstractAxisInput::parentChanged`.
  ///
  /// An object of this type can be created from `AbstractAxisInput` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAxisInput` object.
  pub struct ParentChanged<'a>(&'a ::abstract_axis_input::AbstractAxisInput);
  impl<'a> ::qt_core::connection::Receiver for ParentChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2parentChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ParentChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAbstractAxisInput::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `AbstractAxisInput` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAxisInput` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::abstract_axis_input::AbstractAxisInput);
  impl<'a> ::qt_core::connection::Receiver for DefaultPropertyTrackingModeChanged<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2defaultPropertyTrackingModeChanged(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DefaultPropertyTrackingModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAbstractAxisInput::enabledChanged`.
  ///
  /// An object of this type can be created from `AbstractAxisInput` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAxisInput` object.
  pub struct EnabledChanged<'a>(&'a ::abstract_axis_input::AbstractAxisInput);
  impl<'a> ::qt_core::connection::Receiver for EnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2enabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAbstractAxisInput::sourceDeviceChanged`.
  ///
  /// An object of this type can be created from `AbstractAxisInput` with `object.signals().source_device_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAxisInput` object.
  pub struct SourceDeviceChanged<'a>(&'a ::abstract_axis_input::AbstractAxisInput);
  impl<'a> ::qt_core::connection::Receiver for SourceDeviceChanged<'a> {
    type Arguments = (*mut ::abstract_physical_device::AbstractPhysicalDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceDeviceChanged(Qt3DInput::QAbstractPhysicalDevice*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceDeviceChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAbstractAxisInput::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAbstractAxisInput::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAbstractAxisInput::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAbstractAxisInput::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAbstractAxisInput::sourceDeviceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_device_changed(&self) -> SourceDeviceChanged {
      SourceDeviceChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractAxisInput`.
  pub struct Slots<'a>(&'a ::abstract_axis_input::AbstractAxisInput);
  /// Represents a built-in Qt slot `Qt3DInput::QAbstractAxisInput::setEnabled`.
  ///
  /// An object of this type can be created from `AbstractAxisInput` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAxisInput` object.
  pub struct SetEnabled<'a>(&'a ::abstract_axis_input::AbstractAxisInput);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAbstractAxisInput::setSourceDevice`.
  ///
  /// An object of this type can be created from `AbstractAxisInput` with `object.slots().set_source_device()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAxisInput` object.
  pub struct SetSourceDevice<'a>(&'a ::abstract_axis_input::AbstractAxisInput);
  impl<'a> ::qt_core::connection::Receiver for SetSourceDevice<'a> {
    type Arguments = (*mut ::abstract_physical_device::AbstractPhysicalDevice,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSourceDevice(Qt3DInput::QAbstractPhysicalDevice*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAbstractAxisInput::setParent`.
  ///
  /// An object of this type can be created from `AbstractAxisInput` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAxisInput` object.
  pub struct SetParent<'a>(&'a ::abstract_axis_input::AbstractAxisInput);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAbstractAxisInput::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `AbstractAxisInput` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractAxisInput` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::abstract_axis_input::AbstractAxisInput);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAbstractAxisInput::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAbstractAxisInput::setSourceDevice`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source_device(&self) -> SetSourceDevice {
      SetSourceDevice(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAbstractAxisInput::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAbstractAxisInput::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
  }
  impl ::abstract_axis_input::AbstractAxisInput {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::abstract_axis_input::AbstractAxisInput {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractAxisInput_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::abstract_axis_input::AbstractAxisInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractAxisInput_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::abstract_axis_input::AbstractAxisInput as *mut ::abstract_axis_input::AbstractAxisInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_axis_input::AbstractAxisInput {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractAxisInput_G_static_cast_QObject_ptr(self as *mut ::abstract_axis_input::AbstractAxisInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractAxisInput_G_static_cast_QObject_ptr(self as *const ::abstract_axis_input::AbstractAxisInput as *mut ::abstract_axis_input::AbstractAxisInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_axis_input::AbstractAxisInput> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QAbstractAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_axis_input::AbstractAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QAbstractAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_axis_input::AbstractAxisInput> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QAbstractAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_axis_input::AbstractAxisInput {
    let ffi_result = ::ffi::qt_3d_input_c_QAbstractAxisInput_G_static_cast_Qt3DInput_QAbstractAxisInput_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_axis_input::AbstractAxisInput {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractAxisInput_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::abstract_axis_input::AbstractAxisInput as *mut ::abstract_axis_input::AbstractAxisInput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_axis_input::AbstractAxisInput {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractAxisInput_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::abstract_axis_input::AbstractAxisInput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
