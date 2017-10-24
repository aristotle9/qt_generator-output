/// C++ type: <span style='color: green;'>```Qt3DRender::QPickingSettings::FaceOrientationPickingMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FaceOrientationPickingMode {
  /// C++ enum variant: <span style='color: green;'>```FrontFace = 1```</span>
  Front = 1,
  /// C++ enum variant: <span style='color: green;'>```BackFace = 2```</span>
  Back = 2,
  /// C++ enum variant: <span style='color: green;'>```FrontAndBackFace = 3```</span>
  FrontAndBack = 3,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QPickingSettings::PickMethod```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PickMethod {
  /// C++ enum variant: <span style='color: green;'>```BoundingVolumePicking = 0```</span>
  BoundingVolume = 0,
  /// C++ enum variant: <span style='color: green;'>```TrianglePicking = 1```</span>
  Triangle = 1,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QPickingSettings::PickResultMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PickResultMode {
  /// C++ enum variant: <span style='color: green;'>```NearestPick = 0```</span>
  NearestPick = 0,
  /// C++ enum variant: <span style='color: green;'>```AllPicks = 1```</span>
  AllPicks = 1,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QPickingSettings```</span>
#[repr(C)]
pub struct PickingSettings(u8);

impl PickingSettings {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QPickingSettings::FaceOrientationPickingMode Qt3DRender::QPickingSettings::faceOrientationPickingMode() const```</span>
  ///
  ///
  pub fn face_orientation_picking_mode(&self) -> ::picking_settings::FaceOrientationPickingMode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_faceOrientationPickingMode(self as *const ::picking_settings::PickingSettings) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QPickingSettings::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_metaObject(self as *const ::picking_settings::PickingSettings)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPickingSettings::QPickingSettings()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::picking_settings::PickingSettings> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QPickingSettings::QPickingSettings(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::picking_settings::PickingSettings> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QPickingSettings::PickMethod Qt3DRender::QPickingSettings::pickMethod() const```</span>
  ///
  ///
  pub fn pick_method(&self) -> ::picking_settings::PickMethod {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_pickMethod(self as *const ::picking_settings::PickingSettings)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QPickingSettings::PickResultMode Qt3DRender::QPickingSettings::pickResultMode() const```</span>
  ///
  ///
  pub fn pick_result_mode(&self) -> ::picking_settings::PickResultMode {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_pickResultMode(self as *const ::picking_settings::PickingSettings) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QPickingSettings::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_qt_metacall(self as *mut ::picking_settings::PickingSettings,
                                                                  arg1 as *const ::qt_core::meta_object::Call,
                                                                  arg2,
                                                                  arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QPickingSettings::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_qt_metacast(self as *mut ::picking_settings::PickingSettings,
                                                                  arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPickingSettings::setFaceOrientationPickingMode(Qt3DRender::QPickingSettings::FaceOrientationPickingMode faceOrientationPickingMode)```</span>
  ///
  ///
pub fn set_face_orientation_picking_mode(&mut self, face_orientation_picking_mode: ::picking_settings::FaceOrientationPickingMode) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_setFaceOrientationPickingMode(self as *mut ::picking_settings::PickingSettings, face_orientation_picking_mode) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPickingSettings::setPickMethod(Qt3DRender::QPickingSettings::PickMethod pickMethod)```</span>
  ///
  ///
  pub fn set_pick_method(&mut self, pick_method: ::picking_settings::PickMethod) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_setPickMethod(self as *mut ::picking_settings::PickingSettings, pick_method)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPickingSettings::setPickResultMode(Qt3DRender::QPickingSettings::PickResultMode pickResultMode)```</span>
  ///
  ///
  pub fn set_pick_result_mode(&mut self, pick_result_mode: ::picking_settings::PickResultMode) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_setPickResultMode(self as *mut ::picking_settings::PickingSettings, pick_result_mode) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPickingSettings::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPickingSettings::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::picking_settings::PickingSettings {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QPickingSettings_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PickingSettings`.
  pub struct Signals<'a>(&'a ::picking_settings::PickingSettings);
  /// Represents a built-in Qt signal `Qt3DRender::QPickingSettings::nodeDestroyed`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct NodeDestroyed<'a>(&'a ::picking_settings::PickingSettings);
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
  /// Represents a built-in Qt signal `Qt3DRender::QPickingSettings::pickResultModeChanged`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.signals().pick_result_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct PickResultModeChanged<'a>(&'a ::picking_settings::PickingSettings);
  impl<'a> ::qt_core::connection::Receiver for PickResultModeChanged<'a> {
    type Arguments = (&'static ::picking_settings::PickResultMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pickResultModeChanged(Qt3DRender::QPickingSettings::PickResultMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PickResultModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QPickingSettings::faceOrientationPickingModeChanged`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.signals().face_orientation_picking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct FaceOrientationPickingModeChanged<'a>(&'a ::picking_settings::PickingSettings);
  impl<'a> ::qt_core::connection::Receiver for FaceOrientationPickingModeChanged<'a> {
    type Arguments = (&'static ::picking_settings::FaceOrientationPickingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2faceOrientationPickingModeChanged(Qt3DRender::QPickingSettings::FaceOrientationPickingMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FaceOrientationPickingModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QPickingSettings::pickMethodChanged`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.signals().pick_method_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct PickMethodChanged<'a>(&'a ::picking_settings::PickingSettings);
  impl<'a> ::qt_core::connection::Receiver for PickMethodChanged<'a> {
    type Arguments = (&'static ::picking_settings::PickMethod,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2pickMethodChanged(Qt3DRender::QPickingSettings::PickMethod)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PickMethodChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QPickingSettings::parentChanged`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct ParentChanged<'a>(&'a ::picking_settings::PickingSettings);
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
  /// Represents a built-in Qt signal `Qt3DRender::QPickingSettings::enabledChanged`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct EnabledChanged<'a>(&'a ::picking_settings::PickingSettings);
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
  /// Represents a built-in Qt signal `Qt3DRender::QPickingSettings::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::picking_settings::PickingSettings);
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
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPickingSettings::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPickingSettings::pickResultModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pick_result_mode_changed(&self) -> PickResultModeChanged {
      PickResultModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPickingSettings::faceOrientationPickingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn face_orientation_picking_mode_changed(&self) -> FaceOrientationPickingModeChanged {
      FaceOrientationPickingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPickingSettings::pickMethodChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn pick_method_changed(&self) -> PickMethodChanged {
      PickMethodChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPickingSettings::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPickingSettings::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPickingSettings::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PickingSettings`.
  pub struct Slots<'a>(&'a ::picking_settings::PickingSettings);
  /// Represents a built-in Qt slot `Qt3DRender::QPickingSettings::setPickMethod`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.slots().set_pick_method()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct SetPickMethod<'a>(&'a ::picking_settings::PickingSettings);
  impl<'a> ::qt_core::connection::Receiver for SetPickMethod<'a> {
    type Arguments = (::picking_settings::PickMethod,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPickMethod(Qt3DRender::QPickingSettings::PickMethod)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPickingSettings::setFaceOrientationPickingMode`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.slots().set_face_orientation_picking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct SetFaceOrientationPickingMode<'a>(&'a ::picking_settings::PickingSettings);
  impl<'a> ::qt_core::connection::Receiver for SetFaceOrientationPickingMode<'a> {
    type Arguments = (::picking_settings::FaceOrientationPickingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFaceOrientationPickingMode(Qt3DRender::QPickingSettings::FaceOrientationPickingMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPickingSettings::setPickResultMode`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.slots().set_pick_result_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct SetPickResultMode<'a>(&'a ::picking_settings::PickingSettings);
  impl<'a> ::qt_core::connection::Receiver for SetPickResultMode<'a> {
    type Arguments = (::picking_settings::PickResultMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPickResultMode(Qt3DRender::QPickingSettings::PickResultMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPickingSettings::setEnabled`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct SetEnabled<'a>(&'a ::picking_settings::PickingSettings);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPickingSettings::setParent`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct SetParent<'a>(&'a ::picking_settings::PickingSettings);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPickingSettings::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `PickingSettings` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PickingSettings` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::picking_settings::PickingSettings);
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
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPickingSettings::setPickMethod`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_pick_method(&self) -> SetPickMethod {
      SetPickMethod(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPickingSettings::setFaceOrientationPickingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_face_orientation_picking_mode(&self) -> SetFaceOrientationPickingMode {
      SetFaceOrientationPickingMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPickingSettings::setPickResultMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_pick_result_mode(&self) -> SetPickResultMode {
      SetPickResultMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPickingSettings::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPickingSettings::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPickingSettings::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
  }
  impl ::picking_settings::PickingSettings {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::picking_settings::PickingSettings {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::picking_settings::PickingSettings) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::picking_settings::PickingSettings as *mut ::picking_settings::PickingSettings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::picking_settings::PickingSettings {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickingSettings_G_static_cast_QObject_ptr(self as *mut ::picking_settings::PickingSettings) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickingSettings_G_static_cast_QObject_ptr(self as *const ::picking_settings::PickingSettings as *mut ::picking_settings::PickingSettings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::picking_settings::PickingSettings> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::picking_settings::PickingSettings {
    let ffi_result = ::ffi::qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DRender_QPickingSettings_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::picking_settings::PickingSettings {
    let ffi_result = ::ffi::qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DRender_QPickingSettings_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::picking_settings::PickingSettings> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::picking_settings::PickingSettings {
    let ffi_result = ::ffi::qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DRender_QPickingSettings_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::picking_settings::PickingSettings {
    let ffi_result = ::ffi::qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DRender_QPickingSettings_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::picking_settings::PickingSettings {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::picking_settings::PickingSettings as *mut ::picking_settings::PickingSettings) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::picking_settings::PickingSettings {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPickingSettings_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::picking_settings::PickingSettings) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
