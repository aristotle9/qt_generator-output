/// C++ type: <span style='color: green;'>```Qt3DExtras::QOrbitCameraController```</span>
#[repr(C)]
pub struct OrbitCameraController(u8);

impl OrbitCameraController {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QCamera* Qt3DExtras::QOrbitCameraController::camera() const```</span>
  ///
  ///
  pub fn camera(&self) -> *mut ::qt_3d_render::camera::Camera {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_camera(self as *const ::orbit_camera_controller::OrbitCameraController) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QOrbitCameraController::linearSpeed() const```</span>
  ///
  ///
  pub fn linear_speed(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_linearSpeed(self as *const ::orbit_camera_controller::OrbitCameraController) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QOrbitCameraController::lookSpeed() const```</span>
  ///
  ///
  pub fn look_speed(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_lookSpeed(self as *const ::orbit_camera_controller::OrbitCameraController) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QOrbitCameraController::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_metaObject(self as *const ::orbit_camera_controller::OrbitCameraController) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QOrbitCameraController::QOrbitCameraController()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::orbit_camera_controller::OrbitCameraController> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QOrbitCameraController::QOrbitCameraController(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::orbit_camera_controller::OrbitCameraController> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QOrbitCameraController::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_qt_metacall(self as *mut ::orbit_camera_controller::OrbitCameraController, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QOrbitCameraController::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_qt_metacast(self as *mut ::orbit_camera_controller::OrbitCameraController, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QOrbitCameraController::setCamera(Qt3DRender::QCamera* camera)```</span>
  ///
  ///
  pub unsafe fn set_camera(&mut self, camera: *mut ::qt_3d_render::camera::Camera) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_setCamera(self as *mut ::orbit_camera_controller::OrbitCameraController, camera)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QOrbitCameraController::setLinearSpeed(float linearSpeed)```</span>
  ///
  ///
  pub fn set_linear_speed(&mut self, linear_speed: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_setLinearSpeed(self as *mut ::orbit_camera_controller::OrbitCameraController, linear_speed) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QOrbitCameraController::setLookSpeed(float lookSpeed)```</span>
  ///
  ///
  pub fn set_look_speed(&mut self, look_speed: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_setLookSpeed(self as *mut ::orbit_camera_controller::OrbitCameraController, look_speed) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QOrbitCameraController::setZoomInLimit(float zoomInLimit)```</span>
  ///
  ///
  pub fn set_zoom_in_limit(&mut self, zoom_in_limit: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_setZoomInLimit(self as *mut ::orbit_camera_controller::OrbitCameraController, zoom_in_limit) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QOrbitCameraController::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QOrbitCameraController::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QOrbitCameraController::zoomInLimit() const```</span>
  ///
  ///
  pub fn zoom_in_limit(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_zoomInLimit(self as *const ::orbit_camera_controller::OrbitCameraController) }
  }
}

impl ::cpp_utils::CppDeletable for ::orbit_camera_controller::OrbitCameraController {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QOrbitCameraController_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OrbitCameraController`.
  pub struct Signals<'a>(&'a ::orbit_camera_controller::OrbitCameraController);
  /// Represents a built-in Qt signal `Qt3DExtras::QOrbitCameraController::zoomInLimitChanged`.
  ///
  /// An object of this type can be created from `OrbitCameraController` with `object.signals().zoom_in_limit_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OrbitCameraController` object.
  pub struct ZoomInLimitChanged<'a>(&'a ::orbit_camera_controller::OrbitCameraController);
  impl<'a> ::qt_core::connection::Receiver for ZoomInLimitChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2zoomInLimitChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ZoomInLimitChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QOrbitCameraController::lookSpeedChanged`.
  ///
  /// An object of this type can be created from `OrbitCameraController` with `object.signals().look_speed_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OrbitCameraController` object.
  pub struct LookSpeedChanged<'a>(&'a ::orbit_camera_controller::OrbitCameraController);
  impl<'a> ::qt_core::connection::Receiver for LookSpeedChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2lookSpeedChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LookSpeedChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QOrbitCameraController::cameraChanged`.
  ///
  /// An object of this type can be created from `OrbitCameraController` with `object.signals().camera_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OrbitCameraController` object.
  pub struct CameraChanged<'a>(&'a ::orbit_camera_controller::OrbitCameraController);
  impl<'a> ::qt_core::connection::Receiver for CameraChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2cameraChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for CameraChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QOrbitCameraController::linearSpeedChanged`.
  ///
  /// An object of this type can be created from `OrbitCameraController` with `object.signals().linear_speed_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OrbitCameraController` object.
  pub struct LinearSpeedChanged<'a>(&'a ::orbit_camera_controller::OrbitCameraController);
  impl<'a> ::qt_core::connection::Receiver for LinearSpeedChanged<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2linearSpeedChanged()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LinearSpeedChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QOrbitCameraController::zoomInLimitChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn zoom_in_limit_changed(&self) -> ZoomInLimitChanged {
      ZoomInLimitChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QOrbitCameraController::lookSpeedChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn look_speed_changed(&self) -> LookSpeedChanged {
      LookSpeedChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QOrbitCameraController::cameraChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn camera_changed(&self) -> CameraChanged {
      CameraChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QOrbitCameraController::linearSpeedChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn linear_speed_changed(&self) -> LinearSpeedChanged {
      LinearSpeedChanged(self.0)
    }
  }
  impl ::orbit_camera_controller::OrbitCameraController {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_3d_core::entity::Entity> for ::orbit_camera_controller::OrbitCameraController {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DCore_QEntity_ptr(self as *mut ::orbit_camera_controller::OrbitCameraController) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DCore_QEntity_ptr(self as *const ::orbit_camera_controller::OrbitCameraController as *mut ::orbit_camera_controller::OrbitCameraController) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::orbit_camera_controller::OrbitCameraController {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::orbit_camera_controller::OrbitCameraController) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::orbit_camera_controller::OrbitCameraController as *mut ::orbit_camera_controller::OrbitCameraController) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::orbit_camera_controller::OrbitCameraController {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_QObject_ptr(self as *mut ::orbit_camera_controller::OrbitCameraController) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_QObject_ptr(self as *const ::orbit_camera_controller::OrbitCameraController as *mut ::orbit_camera_controller::OrbitCameraController) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::orbit_camera_controller::OrbitCameraController> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::orbit_camera_controller::OrbitCameraController {
    let ffi_result = ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DExtras_QOrbitCameraController_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::orbit_camera_controller::OrbitCameraController {
    let ffi_result = ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DExtras_QOrbitCameraController_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::orbit_camera_controller::OrbitCameraController> for ::qt_3d_core::entity::Entity {
unsafe fn static_cast_mut(&mut self) -> &mut ::orbit_camera_controller::OrbitCameraController {
let ffi_result = ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DExtras_QOrbitCameraController_ptr_Qt3DCore_QEntity(self as *mut ::qt_3d_core::entity::Entity);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::orbit_camera_controller::OrbitCameraController {
let ffi_result = ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DExtras_QOrbitCameraController_ptr_Qt3DCore_QEntity(self as *const ::qt_3d_core::entity::Entity as *mut ::qt_3d_core::entity::Entity);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::orbit_camera_controller::OrbitCameraController> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::orbit_camera_controller::OrbitCameraController {
    let ffi_result = ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DExtras_QOrbitCameraController_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::orbit_camera_controller::OrbitCameraController {
    let ffi_result = ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DExtras_QOrbitCameraController_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::orbit_camera_controller::OrbitCameraController {
  type Target = ::qt_3d_core::entity::Entity;
  fn deref(&self) -> &::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DCore_QEntity_ptr(self as *const ::orbit_camera_controller::OrbitCameraController as *mut ::orbit_camera_controller::OrbitCameraController) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::orbit_camera_controller::OrbitCameraController {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QOrbitCameraController_G_static_cast_Qt3DCore_QEntity_ptr(self as *mut ::orbit_camera_controller::OrbitCameraController) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
