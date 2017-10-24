/// C++ type: <span style='color: green;'>```QObjectCleanupHandler```</span>
#[repr(C)]
pub struct ObjectCleanupHandler(u8);

impl ObjectCleanupHandler {
  /// C++ method: <span style='color: green;'>```QObject* QObjectCleanupHandler::add(QObject* object)```</span>
  ///
  ///
  pub unsafe fn add(&mut self, object: *mut ::object::Object) -> *mut ::object::Object {
    ::ffi::qt_core_c_QObjectCleanupHandler_add(self as *mut ::object_cleanup_handler::ObjectCleanupHandler,
                                               object)
  }

  /// C++ method: <span style='color: green;'>```void QObjectCleanupHandler::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QObjectCleanupHandler_clear(self as *mut ::object_cleanup_handler::ObjectCleanupHandler) }
  }

  /// C++ method: <span style='color: green;'>```bool QObjectCleanupHandler::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_core_c_QObjectCleanupHandler_isEmpty(self as *const ::object_cleanup_handler::ObjectCleanupHandler)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QObjectCleanupHandler::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_core_c_QObjectCleanupHandler_metaObject(self as *const ::object_cleanup_handler::ObjectCleanupHandler)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QObjectCleanupHandler::QObjectCleanupHandler()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::object_cleanup_handler::ObjectCleanupHandler> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QObjectCleanupHandler_new() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QObjectCleanupHandler::remove(QObject* object)```</span>
  ///
  ///
  pub unsafe fn remove(&mut self, object: *mut ::object::Object) {
    ::ffi::qt_core_c_QObjectCleanupHandler_remove(self as *mut ::object_cleanup_handler::ObjectCleanupHandler,
                                                  object)
  }

  /// C++ method: <span style='color: green;'>```static QString QObjectCleanupHandler::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QObjectCleanupHandler_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QObjectCleanupHandler::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QObjectCleanupHandler_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::object_cleanup_handler::ObjectCleanupHandler {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QObjectCleanupHandler_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ObjectCleanupHandler`.
  pub struct Signals<'a>(&'a ::object_cleanup_handler::ObjectCleanupHandler);
  /// Represents a built-in Qt signal `QObjectCleanupHandler::objectNameChanged`.
  ///
  /// An object of this type can be created from `ObjectCleanupHandler` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectCleanupHandler` object.
  pub struct ObjectNameChanged<'a>(&'a ::object_cleanup_handler::ObjectCleanupHandler);
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
    /// Returns an object representing a built-in Qt signal `QObjectCleanupHandler::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ObjectCleanupHandler`.
  pub struct Slots<'a>(&'a ::object_cleanup_handler::ObjectCleanupHandler);
  /// Represents a built-in Qt slot `QObjectCleanupHandler::objectDestroyed`.
  ///
  /// An object of this type can be created from `ObjectCleanupHandler` with `object.slots().object_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ObjectCleanupHandler` object.
  pub struct ObjectDestroyed<'a>(&'a ::object_cleanup_handler::ObjectCleanupHandler);
  impl<'a> ::connection::Receiver for ObjectDestroyed<'a> {
    type Arguments = (*mut ::object::Object,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1objectDestroyed(QObject*)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QObjectCleanupHandler::objectDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_destroyed(&self) -> ObjectDestroyed {
      ObjectDestroyed(self.0)
    }
  }
  impl ::object_cleanup_handler::ObjectCleanupHandler {
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

impl ::cpp_utils::DynamicCast<::object_cleanup_handler::ObjectCleanupHandler> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::object_cleanup_handler::ObjectCleanupHandler> {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QObjectCleanupHandler_G_dynamic_cast_QObjectCleanupHandler_ptr(self as *mut ::object::Object)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::object_cleanup_handler::ObjectCleanupHandler> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QObjectCleanupHandler_G_dynamic_cast_QObjectCleanupHandler_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::object_cleanup_handler::ObjectCleanupHandler {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QObjectCleanupHandler_G_static_cast_QObject_ptr(self as *mut ::object_cleanup_handler::ObjectCleanupHandler) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QObjectCleanupHandler_G_static_cast_QObject_ptr(self as *const ::object_cleanup_handler::ObjectCleanupHandler as *mut ::object_cleanup_handler::ObjectCleanupHandler) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::object_cleanup_handler::ObjectCleanupHandler> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::object_cleanup_handler::ObjectCleanupHandler {
    let ffi_result =
      ::ffi::qt_core_c_QObjectCleanupHandler_G_static_cast_QObjectCleanupHandler_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::object_cleanup_handler::ObjectCleanupHandler {
    let ffi_result = ::ffi::qt_core_c_QObjectCleanupHandler_G_static_cast_QObjectCleanupHandler_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::object_cleanup_handler::ObjectCleanupHandler {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QObjectCleanupHandler_G_static_cast_QObject_ptr(self as *const ::object_cleanup_handler::ObjectCleanupHandler as *mut ::object_cleanup_handler::ObjectCleanupHandler) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::object_cleanup_handler::ObjectCleanupHandler {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QObjectCleanupHandler_G_static_cast_QObject_ptr(self as *mut ::object_cleanup_handler::ObjectCleanupHandler) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
