/// C++ type: <span style='color: green;'>```Qt3DExtras::QFirstPersonCameraController```</span>
#[repr(C)]
pub struct FirstPersonCameraController(u8);

impl FirstPersonCameraController {
  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QFirstPersonCameraController::acceleration() const```</span>
  ///
  ///
  pub fn acceleration(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_acceleration(self as *const ::first_person_camera_controller::FirstPersonCameraController) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QCamera* Qt3DExtras::QFirstPersonCameraController::camera() const```</span>
  ///
  ///
  pub fn camera(&self) -> *mut ::qt_3d_render::camera::Camera {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_camera(self as *const ::first_person_camera_controller::FirstPersonCameraController) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QFirstPersonCameraController::deceleration() const```</span>
  ///
  ///
  pub fn deceleration(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_deceleration(self as *const ::first_person_camera_controller::FirstPersonCameraController) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QFirstPersonCameraController::linearSpeed() const```</span>
  ///
  ///
  pub fn linear_speed(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_linearSpeed(self as *const ::first_person_camera_controller::FirstPersonCameraController) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QFirstPersonCameraController::lookSpeed() const```</span>
  ///
  ///
  pub fn look_speed(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_lookSpeed(self as *const ::first_person_camera_controller::FirstPersonCameraController) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QFirstPersonCameraController::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_metaObject(self as *const ::first_person_camera_controller::FirstPersonCameraController) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QFirstPersonCameraController::QFirstPersonCameraController()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::first_person_camera_controller::FirstPersonCameraController> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QFirstPersonCameraController::QFirstPersonCameraController(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::first_person_camera_controller::FirstPersonCameraController> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QFirstPersonCameraController::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_qt_metacall(self as *mut ::first_person_camera_controller::FirstPersonCameraController, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QFirstPersonCameraController::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_qt_metacast(self as *mut ::first_person_camera_controller::FirstPersonCameraController, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QFirstPersonCameraController::setAcceleration(float acceleration)```</span>
  ///
  ///
  pub fn set_acceleration(&mut self, acceleration: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_setAcceleration(self as *mut ::first_person_camera_controller::FirstPersonCameraController, acceleration) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QFirstPersonCameraController::setCamera(Qt3DRender::QCamera* camera)```</span>
  ///
  ///
  pub unsafe fn set_camera(&mut self, camera: *mut ::qt_3d_render::camera::Camera) {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_setCamera(self as *mut ::first_person_camera_controller::FirstPersonCameraController, camera)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QFirstPersonCameraController::setDeceleration(float deceleration)```</span>
  ///
  ///
  pub fn set_deceleration(&mut self, deceleration: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_setDeceleration(self as *mut ::first_person_camera_controller::FirstPersonCameraController, deceleration) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QFirstPersonCameraController::setLinearSpeed(float linearSpeed)```</span>
  ///
  ///
  pub fn set_linear_speed(&mut self, linear_speed: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_setLinearSpeed(self as *mut ::first_person_camera_controller::FirstPersonCameraController, linear_speed) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QFirstPersonCameraController::setLookSpeed(float lookSpeed)```</span>
  ///
  ///
  pub fn set_look_speed(&mut self, look_speed: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_setLookSpeed(self as *mut ::first_person_camera_controller::FirstPersonCameraController, look_speed) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QFirstPersonCameraController::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QFirstPersonCameraController::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::first_person_camera_controller::FirstPersonCameraController {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QFirstPersonCameraController_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `FirstPersonCameraController`.
  pub struct Signals<'a>(&'a ::first_person_camera_controller::FirstPersonCameraController);
  /// Represents a built-in Qt signal `Qt3DExtras::QFirstPersonCameraController::cameraChanged`.
  ///
  /// An object of this type can be created from `FirstPersonCameraController` with `object.signals().camera_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FirstPersonCameraController` object.
  pub struct CameraChanged<'a>(&'a ::first_person_camera_controller::FirstPersonCameraController);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QFirstPersonCameraController::decelerationChanged`.
  ///
  /// An object of this type can be created from `FirstPersonCameraController` with `object.signals().deceleration_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FirstPersonCameraController` object.
  pub struct DecelerationChanged<'a>(&'a ::first_person_camera_controller::FirstPersonCameraController);
  impl<'a> ::qt_core::connection::Receiver for DecelerationChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2decelerationChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DecelerationChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QFirstPersonCameraController::accelerationChanged`.
  ///
  /// An object of this type can be created from `FirstPersonCameraController` with `object.signals().acceleration_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FirstPersonCameraController` object.
  pub struct AccelerationChanged<'a>(&'a ::first_person_camera_controller::FirstPersonCameraController);
  impl<'a> ::qt_core::connection::Receiver for AccelerationChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2accelerationChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AccelerationChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QFirstPersonCameraController::lookSpeedChanged`.
  ///
  /// An object of this type can be created from `FirstPersonCameraController` with `object.signals().look_speed_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FirstPersonCameraController` object.
  pub struct LookSpeedChanged<'a>(&'a ::first_person_camera_controller::FirstPersonCameraController);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QFirstPersonCameraController::linearSpeedChanged`.
  ///
  /// An object of this type can be created from `FirstPersonCameraController` with `object.signals().linear_speed_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `FirstPersonCameraController` object.
  pub struct LinearSpeedChanged<'a>(&'a ::first_person_camera_controller::FirstPersonCameraController);
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
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QFirstPersonCameraController::cameraChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn camera_changed(&self) -> CameraChanged {
      CameraChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QFirstPersonCameraController::decelerationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn deceleration_changed(&self) -> DecelerationChanged {
      DecelerationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QFirstPersonCameraController::accelerationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn acceleration_changed(&self) -> AccelerationChanged {
      AccelerationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QFirstPersonCameraController::lookSpeedChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn look_speed_changed(&self) -> LookSpeedChanged {
      LookSpeedChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QFirstPersonCameraController::linearSpeedChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn linear_speed_changed(&self) -> LinearSpeedChanged {
      LinearSpeedChanged(self.0)
    }
  }
  impl ::first_person_camera_controller::FirstPersonCameraController {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_3d_core::entity::Entity> for ::first_person_camera_controller::FirstPersonCameraController {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::entity::Entity {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DCore_QEntity_ptr(self as *mut ::first_person_camera_controller::FirstPersonCameraController) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::entity::Entity {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DCore_QEntity_ptr(self as *const ::first_person_camera_controller::FirstPersonCameraController as *mut ::first_person_camera_controller::FirstPersonCameraController) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::first_person_camera_controller::FirstPersonCameraController {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::first_person_camera_controller::FirstPersonCameraController) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_core::node::Node {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::first_person_camera_controller::FirstPersonCameraController as *mut ::first_person_camera_controller::FirstPersonCameraController) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::first_person_camera_controller::FirstPersonCameraController {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_QObject_ptr(self as *mut ::first_person_camera_controller::FirstPersonCameraController) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_QObject_ptr(self as *const ::first_person_camera_controller::FirstPersonCameraController as *mut ::first_person_camera_controller::FirstPersonCameraController) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::first_person_camera_controller::FirstPersonCameraController> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::first_person_camera_controller::FirstPersonCameraController {
let ffi_result = ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DExtras_QFirstPersonCameraController_ptr_QObject(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::first_person_camera_controller::FirstPersonCameraController {
let ffi_result = ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DExtras_QFirstPersonCameraController_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::first_person_camera_controller::FirstPersonCameraController> for ::qt_3d_core::entity::Entity {
unsafe fn static_cast_mut(&mut self) -> &mut ::first_person_camera_controller::FirstPersonCameraController {
let ffi_result = ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DExtras_QFirstPersonCameraController_ptr_Qt3DCore_QEntity(self as *mut ::qt_3d_core::entity::Entity);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::first_person_camera_controller::FirstPersonCameraController {
let ffi_result = ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DExtras_QFirstPersonCameraController_ptr_Qt3DCore_QEntity(self as *const ::qt_3d_core::entity::Entity as *mut ::qt_3d_core::entity::Entity);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::first_person_camera_controller::FirstPersonCameraController> for ::qt_3d_core::node::Node {
unsafe fn static_cast_mut(&mut self) -> &mut ::first_person_camera_controller::FirstPersonCameraController {
let ffi_result = ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DExtras_QFirstPersonCameraController_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::first_person_camera_controller::FirstPersonCameraController {
let ffi_result = ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DExtras_QFirstPersonCameraController_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::first_person_camera_controller::FirstPersonCameraController {
  type Target = ::qt_3d_core::entity::Entity;
  fn deref(&self) -> &::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DCore_QEntity_ptr(self as *const ::first_person_camera_controller::FirstPersonCameraController as *mut ::first_person_camera_controller::FirstPersonCameraController) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::first_person_camera_controller::FirstPersonCameraController {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::entity::Entity {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QFirstPersonCameraController_G_static_cast_Qt3DCore_QEntity_ptr(self as *mut ::first_person_camera_controller::FirstPersonCameraController) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
