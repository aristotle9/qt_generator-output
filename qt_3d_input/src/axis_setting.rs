/// C++ type: <span style='color: green;'>```Qt3DInput::QAxisSetting```</span>
#[repr(C)]
pub struct AxisSetting(u8);

impl AxisSetting {
  /// C++ method: <span style='color: green;'>```QVector<int> Qt3DInput::QAxisSetting::axes() const```</span>
  ///
  ///
  pub fn axes(&self) -> ::qt_core::vector::VectorCInt {
    {
      let mut object: ::qt_core::vector::VectorCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_axes_to_output(self as *const ::axis_setting::AxisSetting,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DInput::QAxisSetting::deadZoneRadius() const```</span>
  ///
  ///
  pub fn dead_zone_radius(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_deadZoneRadius(self as *const ::axis_setting::AxisSetting) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DInput::QAxisSetting::isSmoothEnabled() const```</span>
  ///
  ///
  pub fn is_smooth_enabled(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_isSmoothEnabled(self as *const ::axis_setting::AxisSetting) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DInput::QAxisSetting::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_metaObject(self as *const ::axis_setting::AxisSetting) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QAxisSetting::QAxisSetting()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::axis_setting::AxisSetting> {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DInput::QAxisSetting::QAxisSetting(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::axis_setting::AxisSetting> {
    let ffi_result = ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DInput::QAxisSetting::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_qt_metacall(self as *mut ::axis_setting::AxisSetting,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DInput::QAxisSetting::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_qt_metacast(self as *mut ::axis_setting::AxisSetting, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QAxisSetting::setAxes(const QVector<int>& axes)```</span>
  ///
  ///
  pub fn set_axes(&mut self, axes: &::qt_core::vector::VectorCInt) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_setAxes(self as *mut ::axis_setting::AxisSetting,
                                                          axes as *const ::qt_core::vector::VectorCInt)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QAxisSetting::setDeadZoneRadius(float deadZoneRadius)```</span>
  ///
  ///
  pub fn set_dead_zone_radius(&mut self, dead_zone_radius: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_setDeadZoneRadius(self as *mut ::axis_setting::AxisSetting,
                                                                    dead_zone_radius)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DInput::QAxisSetting::setSmoothEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_smooth_enabled(&mut self, enabled: bool) {
    unsafe {
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_setSmoothEnabled(self as *mut ::axis_setting::AxisSetting, enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QAxisSetting::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DInput::QAxisSetting::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::axis_setting::AxisSetting {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_input_c_Qt3DInput_QAxisSetting_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AxisSetting`.
  pub struct Signals<'a>(&'a ::axis_setting::AxisSetting);
  /// Represents a built-in Qt signal `Qt3DInput::QAxisSetting::enabledChanged`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct EnabledChanged<'a>(&'a ::axis_setting::AxisSetting);
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
  /// Represents a built-in Qt signal `Qt3DInput::QAxisSetting::nodeDestroyed`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct NodeDestroyed<'a>(&'a ::axis_setting::AxisSetting);
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
  /// Represents a built-in Qt signal `Qt3DInput::QAxisSetting::smoothChanged`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.signals().smooth_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct SmoothChanged<'a>(&'a ::axis_setting::AxisSetting);
  impl<'a> ::qt_core::connection::Receiver for SmoothChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2smoothChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SmoothChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAxisSetting::deadZoneRadiusChanged`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.signals().dead_zone_radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct DeadZoneRadiusChanged<'a>(&'a ::axis_setting::AxisSetting);
  impl<'a> ::qt_core::connection::Receiver for DeadZoneRadiusChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2deadZoneRadiusChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DeadZoneRadiusChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAxisSetting::parentChanged`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct ParentChanged<'a>(&'a ::axis_setting::AxisSetting);
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
  /// Represents a built-in Qt signal `Qt3DInput::QAxisSetting::axesChanged`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.signals().axes_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct AxesChanged<'a>(&'a ::axis_setting::AxisSetting);
  impl<'a> ::qt_core::connection::Receiver for AxesChanged<'a> {
    type Arguments = (&'static ::qt_core::vector::VectorCInt,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2axesChanged(const QVector< int >&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AxesChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DInput::QAxisSetting::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::axis_setting::AxisSetting);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisSetting::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisSetting::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisSetting::smoothChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn smooth_changed(&self) -> SmoothChanged {
      SmoothChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisSetting::deadZoneRadiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn dead_zone_radius_changed(&self) -> DeadZoneRadiusChanged {
      DeadZoneRadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisSetting::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisSetting::axesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn axes_changed(&self) -> AxesChanged {
      AxesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DInput::QAxisSetting::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AxisSetting`.
  pub struct Slots<'a>(&'a ::axis_setting::AxisSetting);
  /// Represents a built-in Qt slot `Qt3DInput::QAxisSetting::setDeadZoneRadius`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.slots().set_dead_zone_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct SetDeadZoneRadius<'a>(&'a ::axis_setting::AxisSetting);
  impl<'a> ::qt_core::connection::Receiver for SetDeadZoneRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDeadZoneRadius(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAxisSetting::setEnabled`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct SetEnabled<'a>(&'a ::axis_setting::AxisSetting);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAxisSetting::setAxes`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.slots().set_axes()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct SetAxes<'a>(&'a ::axis_setting::AxisSetting);
  impl<'a> ::qt_core::connection::Receiver for SetAxes<'a> {
    type Arguments = (&'static ::qt_core::vector::VectorCInt,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAxes(const QVector< int >&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAxisSetting::setSmoothEnabled`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.slots().set_smooth_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct SetSmoothEnabled<'a>(&'a ::axis_setting::AxisSetting);
  impl<'a> ::qt_core::connection::Receiver for SetSmoothEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSmoothEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAxisSetting::setParent`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct SetParent<'a>(&'a ::axis_setting::AxisSetting);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DInput::QAxisSetting::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `AxisSetting` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AxisSetting` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::axis_setting::AxisSetting);
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
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAxisSetting::setDeadZoneRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_dead_zone_radius(&self) -> SetDeadZoneRadius {
      SetDeadZoneRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAxisSetting::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAxisSetting::setAxes`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_axes(&self) -> SetAxes {
      SetAxes(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAxisSetting::setSmoothEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_smooth_enabled(&self) -> SetSmoothEnabled {
      SetSmoothEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAxisSetting::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DInput::QAxisSetting::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
  }
  impl ::axis_setting::AxisSetting {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::axis_setting::AxisSetting {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::axis_setting::AxisSetting)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::axis_setting::AxisSetting as *mut ::axis_setting::AxisSetting) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::axis_setting::AxisSetting {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QAxisSetting_G_static_cast_QObject_ptr(self as *mut ::axis_setting::AxisSetting) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAxisSetting_G_static_cast_QObject_ptr(self as *const ::axis_setting::AxisSetting as *mut ::axis_setting::AxisSetting) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::axis_setting::AxisSetting> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::axis_setting::AxisSetting {
    let ffi_result = ::ffi::qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DInput_QAxisSetting_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::axis_setting::AxisSetting {
    let ffi_result = ::ffi::qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DInput_QAxisSetting_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::axis_setting::AxisSetting> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::axis_setting::AxisSetting {
    let ffi_result = ::ffi::qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DInput_QAxisSetting_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::axis_setting::AxisSetting {
    let ffi_result = ::ffi::qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DInput_QAxisSetting_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::axis_setting::AxisSetting {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::axis_setting::AxisSetting as *mut ::axis_setting::AxisSetting) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::axis_setting::AxisSetting {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QAxisSetting_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::axis_setting::AxisSetting)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
