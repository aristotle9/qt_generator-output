/// C++ type: <span style='color: green;'>```Qt3DInput::QAbstractPhysicalDevice```</span>
#[repr(C)]
pub struct AbstractPhysicalDevice(u8);

impl AbstractPhysicalDevice {
  /// C++ method: <span style='color: green;'>```void Qt3DInput::QAbstractPhysicalDevice::addAxisSetting(Qt3DInput::QAxisSetting* axisSetting)```</span>
  ///
  ///
  pub unsafe fn add_axis_setting(&mut self, axis_setting: *mut ::axis_setting::AxisSetting) {
    ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_addAxisSetting(self as *mut ::abstract_physical_device::AbstractPhysicalDevice, axis_setting)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QAbstractPhysicalDevice::axisCount() const```</span>
  ///
  ///
  pub fn axis_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisCount(self as *const ::abstract_physical_device::AbstractPhysicalDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QAbstractPhysicalDevice::axisIdentifier(const QString& name) const```</span>
  ///
  ///
  pub fn axis_identifier(&self, name: &::qt_core::string::String) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisIdentifier(self as *const ::abstract_physical_device::AbstractPhysicalDevice, name as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList Qt3DInput::QAbstractPhysicalDevice::axisNames() const```</span>
  ///
  ///
  pub fn axis_names(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisNames_to_output(self as *const ::abstract_physical_device::AbstractPhysicalDevice, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*> Qt3DInput::QAbstractPhysicalDevice::axisSettings() const```</span>
  ///
  ///
  pub fn axis_settings(&self) -> ::vector::VectorAxisSettingMutPtr {
    {
      let mut object: ::vector::VectorAxisSettingMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_axisSettings_to_output(self as *const ::abstract_physical_device::AbstractPhysicalDevice, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QAbstractPhysicalDevice::buttonCount() const```</span>
  ///
  ///
  pub fn button_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_buttonCount(self as *const ::abstract_physical_device::AbstractPhysicalDevice) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QAbstractPhysicalDevice::buttonIdentifier(const QString& name) const```</span>
  ///
  ///
  pub fn button_identifier(&self, name: &::qt_core::string::String) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_buttonIdentifier(self as *const ::abstract_physical_device::AbstractPhysicalDevice, name as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```virtual QStringList Qt3DInput::QAbstractPhysicalDevice::buttonNames() const```</span>
  ///
  ///
  pub fn button_names(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_buttonNames_to_output(self as *const ::abstract_physical_device::AbstractPhysicalDevice, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QAbstractPhysicalDevice::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_metaObject(self as *const ::abstract_physical_device::AbstractPhysicalDevice) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QAbstractPhysicalDevice::QAbstractPhysicalDevice()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::abstract_physical_device::AbstractPhysicalDevice> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QAbstractPhysicalDevice::QAbstractPhysicalDevice(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::abstract_physical_device::AbstractPhysicalDevice> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QAbstractPhysicalDevice::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_qt_metacall(self as *mut ::abstract_physical_device::AbstractPhysicalDevice, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QAbstractPhysicalDevice::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_qt_metacast(self as *mut ::abstract_physical_device::AbstractPhysicalDevice, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DInput::QAbstractPhysicalDevice::removeAxisSetting(Qt3DInput::QAxisSetting* axisSetting)```</span>
  ///
  ///
  pub unsafe fn remove_axis_setting(&mut self, axis_setting: *mut ::axis_setting::AxisSetting) {
    ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_removeAxisSetting(self as *mut ::abstract_physical_device::AbstractPhysicalDevice, axis_setting)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QAbstractPhysicalDevice::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QAbstractPhysicalDevice::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_physical_device::AbstractPhysicalDevice {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QAbstractPhysicalDevice_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractPhysicalDevice`.
  pub struct Signals<'a>(&'a ::abstract_physical_device::AbstractPhysicalDevice);
  /// Represents a built-in Qt signal `Qt3DInput::QAbstractPhysicalDevice::nodeDestroyed`.
  ///
  /// An object of this type can be created from `AbstractPhysicalDevice` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractPhysicalDevice` object.
  pub struct NodeDestroyed<'a>(&'a ::abstract_physical_device::AbstractPhysicalDevice);
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
  /// Represents a built-in Qt signal `Qt3DInput::QAbstractPhysicalDevice::parentChanged`.
  ///
  /// An object of this type can be created from `AbstractPhysicalDevice` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractPhysicalDevice` object.
  pub struct ParentChanged<'a>(&'a ::abstract_physical_device::AbstractPhysicalDevice);
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
  /// Represents a built-in Qt signal `Qt3DInput::QAbstractPhysicalDevice::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `AbstractPhysicalDevice` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractPhysicalDevice` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::abstract_physical_device::AbstractPhysicalDevice);
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
  /// Represents a built-in Qt signal `Qt3DInput::QAbstractPhysicalDevice::enabledChanged`.
  ///
  /// An object of this type can be created from `AbstractPhysicalDevice` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractPhysicalDevice` object.
  pub struct EnabledChanged<'a>(&'a ::abstract_physical_device::AbstractPhysicalDevice);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAbstractPhysicalDevice::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAbstractPhysicalDevice::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAbstractPhysicalDevice::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAbstractPhysicalDevice::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractPhysicalDevice`.
  pub struct Slots<'a>(&'a ::abstract_physical_device::AbstractPhysicalDevice);
  /// Represents a built-in Qt slot `Qt3DInput::QAbstractPhysicalDevice::setParent`.
  ///
  /// An object of this type can be created from `AbstractPhysicalDevice` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractPhysicalDevice` object.
  pub struct SetParent<'a>(&'a ::abstract_physical_device::AbstractPhysicalDevice);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAbstractPhysicalDevice::setEnabled`.
  ///
  /// An object of this type can be created from `AbstractPhysicalDevice` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractPhysicalDevice` object.
  pub struct SetEnabled<'a>(&'a ::abstract_physical_device::AbstractPhysicalDevice);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAbstractPhysicalDevice::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `AbstractPhysicalDevice` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractPhysicalDevice` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::abstract_physical_device::AbstractPhysicalDevice);
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
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAbstractPhysicalDevice::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAbstractPhysicalDevice::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAbstractPhysicalDevice::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
  }
  impl ::abstract_physical_device::AbstractPhysicalDevice {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::abstract_physical_device::AbstractPhysicalDevice {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::abstract_physical_device::AbstractPhysicalDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::abstract_physical_device::AbstractPhysicalDevice as *mut ::abstract_physical_device::AbstractPhysicalDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_physical_device::AbstractPhysicalDevice {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_QObject_ptr(self as *mut ::abstract_physical_device::AbstractPhysicalDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_QObject_ptr(self as *const ::abstract_physical_device::AbstractPhysicalDevice as *mut ::abstract_physical_device::AbstractPhysicalDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_physical_device::AbstractPhysicalDevice> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_physical_device::AbstractPhysicalDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_physical_device::AbstractPhysicalDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_physical_device::AbstractPhysicalDevice> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_physical_device::AbstractPhysicalDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_physical_device::AbstractPhysicalDevice {
    let ffi_result = ::ffi::qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DInput_QAbstractPhysicalDevice_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_physical_device::AbstractPhysicalDevice {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::abstract_physical_device::AbstractPhysicalDevice as *mut ::abstract_physical_device::AbstractPhysicalDevice) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_physical_device::AbstractPhysicalDevice {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAbstractPhysicalDevice_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::abstract_physical_device::AbstractPhysicalDevice) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
