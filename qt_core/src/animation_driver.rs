/// C++ type: <span style='color: green;'>```QAnimationDriver```</span>
#[repr(C)]
pub struct AnimationDriver(u8);

impl AnimationDriver {
  /// C++ method: <span style='color: green;'>```virtual void QAnimationDriver::advance()```</span>
  ///
  ///
  pub fn advance(&mut self) {
    unsafe { ::ffi::qt_core_c_QAnimationDriver_advance(self as *mut ::animation_driver::AnimationDriver) }
  }

  /// C++ method: <span style='color: green;'>```virtual qint64 QAnimationDriver::elapsed() const```</span>
  ///
  ///
  pub fn elapsed(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QAnimationDriver_elapsed(self as *const ::animation_driver::AnimationDriver) }
  }

  /// C++ method: <span style='color: green;'>```void QAnimationDriver::install()```</span>
  ///
  ///
  pub fn install(&mut self) {
    unsafe { ::ffi::qt_core_c_QAnimationDriver_install(self as *mut ::animation_driver::AnimationDriver) }
  }

  /// C++ method: <span style='color: green;'>```bool QAnimationDriver::isRunning() const```</span>
  ///
  ///
  pub fn is_running(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QAnimationDriver_isRunning(self as *const ::animation_driver::AnimationDriver) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QAnimationDriver::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QAnimationDriver_metaObject(self as *const ::animation_driver::AnimationDriver) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAnimationDriver::QAnimationDriver()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::animation_driver::AnimationDriver> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationDriver_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAnimationDriver::QAnimationDriver(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::object::Object) -> ::cpp_utils::CppBox<::animation_driver::AnimationDriver> {
    let ffi_result = ::ffi::qt_core_c_QAnimationDriver_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```void QAnimationDriver::setStartTime(qint64 startTime)```</span>
  ///
  ///
  pub fn set_start_time(&mut self, start_time: i64) {
    unsafe {
      ::ffi::qt_core_c_QAnimationDriver_setStartTime(self as *mut ::animation_driver::AnimationDriver, start_time)
    }
  }

  /// C++ method: <span style='color: green;'>```qint64 QAnimationDriver::startTime() const```</span>
  ///
  ///
  pub fn start_time(&self) -> i64 {
    unsafe { ::ffi::qt_core_c_QAnimationDriver_startTime(self as *const ::animation_driver::AnimationDriver) }
  }

  /// C++ method: <span style='color: green;'>```static QString QAnimationDriver::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAnimationDriver_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QAnimationDriver::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QAnimationDriver_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QAnimationDriver::uninstall()```</span>
  ///
  ///
  pub fn uninstall(&mut self) {
    unsafe { ::ffi::qt_core_c_QAnimationDriver_uninstall(self as *mut ::animation_driver::AnimationDriver) }
  }
}

impl ::cpp_utils::CppDeletable for ::animation_driver::AnimationDriver {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QAnimationDriver_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AnimationDriver`.
  pub struct Signals<'a>(&'a ::animation_driver::AnimationDriver);
  /// Represents a built-in Qt signal `QAnimationDriver::started`.
  ///
  /// An object of this type can be created from `AnimationDriver` with `object.signals().started()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationDriver` object.
  pub struct Started<'a>(&'a ::animation_driver::AnimationDriver);
  impl<'a> ::connection::Receiver for Started<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2started()\0"
    }
  }
  impl<'a> ::connection::Signal for Started<'a> {}
  /// Represents a built-in Qt signal `QAnimationDriver::stopped`.
  ///
  /// An object of this type can be created from `AnimationDriver` with `object.signals().stopped()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationDriver` object.
  pub struct Stopped<'a>(&'a ::animation_driver::AnimationDriver);
  impl<'a> ::connection::Receiver for Stopped<'a> {
    type Arguments = ();
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2stopped()\0"
    }
  }
  impl<'a> ::connection::Signal for Stopped<'a> {}
  /// Represents a built-in Qt signal `QAnimationDriver::objectNameChanged`.
  ///
  /// An object of this type can be created from `AnimationDriver` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AnimationDriver` object.
  pub struct ObjectNameChanged<'a>(&'a ::animation_driver::AnimationDriver);
  impl<'a> ::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::string::String,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::connection::Signal for ObjectNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QAnimationDriver::started`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn started(&self) -> Started {
      Started(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAnimationDriver::stopped`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stopped(&self) -> Stopped {
      Stopped(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QAnimationDriver::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::animation_driver::AnimationDriver {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::animation_driver::AnimationDriver> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::animation_driver::AnimationDriver> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QAnimationDriver_G_dynamic_cast_QAnimationDriver_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::animation_driver::AnimationDriver> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationDriver_G_dynamic_cast_QAnimationDriver_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::animation_driver::AnimationDriver {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QAnimationDriver_G_static_cast_QObject_ptr(self as *mut ::animation_driver::AnimationDriver)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationDriver_G_static_cast_QObject_ptr(self as *const ::animation_driver::AnimationDriver as *mut ::animation_driver::AnimationDriver) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::animation_driver::AnimationDriver> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::animation_driver::AnimationDriver {
    let ffi_result =
      ::ffi::qt_core_c_QAnimationDriver_G_static_cast_QAnimationDriver_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::animation_driver::AnimationDriver {
    let ffi_result = ::ffi::qt_core_c_QAnimationDriver_G_static_cast_QAnimationDriver_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::animation_driver::AnimationDriver {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAnimationDriver_G_static_cast_QObject_ptr(self as *const ::animation_driver::AnimationDriver as *mut ::animation_driver::AnimationDriver) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::animation_driver::AnimationDriver {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QAnimationDriver_G_static_cast_QObject_ptr(self as *mut ::animation_driver::AnimationDriver)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
