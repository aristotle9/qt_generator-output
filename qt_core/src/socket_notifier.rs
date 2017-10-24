/// C++ type: <span style='color: green;'>```QSocketNotifier```</span>
#[repr(C)]
pub struct SocketNotifier(u8);

impl SocketNotifier {
  /// C++ method: <span style='color: green;'>```bool QSocketNotifier::isEnabled() const```</span>
  ///
  ///
  pub fn is_enabled(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSocketNotifier_isEnabled(self as *const ::socket_notifier::SocketNotifier) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QSocketNotifier::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QSocketNotifier_metaObject(self as *const ::socket_notifier::SocketNotifier) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSocketNotifier::QSocketNotifier(qintptr socket, QSocketNotifier::Type arg2)```</span>
  ///
  ///
  pub fn new(socket: isize, arg2: ::socket_notifier::Type) -> ::cpp_utils::CppBox<::socket_notifier::SocketNotifier> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSocketNotifier_new_socket_arg2(socket, arg2) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSocketNotifier::QSocketNotifier(qintptr socket, QSocketNotifier::Type arg2, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(socket: isize,
                           arg2: ::socket_notifier::Type,
                           parent: *mut ::object::Object)
                           -> ::cpp_utils::CppBox<::socket_notifier::SocketNotifier> {
    let ffi_result = ::ffi::qt_core_c_QSocketNotifier_new_socket_arg2_parent(socket, arg2, parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```[slot] void QSocketNotifier::setEnabled(bool arg1)```</span>
  ///
  ///
  pub fn set_enabled(&mut self, arg1: bool) {
    unsafe { ::ffi::qt_core_c_QSocketNotifier_setEnabled(self as *mut ::socket_notifier::SocketNotifier, arg1) }
  }

  /// C++ method: <span style='color: green;'>```qintptr QSocketNotifier::socket() const```</span>
  ///
  ///
  pub fn socket(&self) -> isize {
    unsafe { ::ffi::qt_core_c_QSocketNotifier_socket(self as *const ::socket_notifier::SocketNotifier) }
  }

  /// C++ method: <span style='color: green;'>```static QString QSocketNotifier::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSocketNotifier_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QSocketNotifier::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QSocketNotifier_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSocketNotifier::Type QSocketNotifier::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::socket_notifier::Type {
    unsafe { ::ffi::qt_core_c_QSocketNotifier_type(self as *const ::socket_notifier::SocketNotifier) }
  }
}

impl ::cpp_utils::CppDeletable for ::socket_notifier::SocketNotifier {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_core_c_QSocketNotifier_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SocketNotifier`.
  pub struct Signals<'a>(&'a ::socket_notifier::SocketNotifier);
  /// Represents a built-in Qt signal `QSocketNotifier::objectNameChanged`.
  ///
  /// An object of this type can be created from `SocketNotifier` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SocketNotifier` object.
  pub struct ObjectNameChanged<'a>(&'a ::socket_notifier::SocketNotifier);
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
  /// Represents a built-in Qt signal `QSocketNotifier::activated`.
  ///
  /// An object of this type can be created from `SocketNotifier` with `object.signals().activated()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SocketNotifier` object.
  pub struct Activated<'a>(&'a ::socket_notifier::SocketNotifier);
  impl<'a> ::connection::Receiver for Activated<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2activated(int)\0"
    }
  }
  impl<'a> ::connection::Signal for Activated<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QSocketNotifier::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QSocketNotifier::activated`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn activated(&self) -> Activated {
      Activated(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SocketNotifier`.
  pub struct Slots<'a>(&'a ::socket_notifier::SocketNotifier);
  /// Represents a built-in Qt slot `QSocketNotifier::setEnabled`.
  ///
  /// An object of this type can be created from `SocketNotifier` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SocketNotifier` object.
  pub struct SetEnabled<'a>(&'a ::socket_notifier::SocketNotifier);
  impl<'a> ::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QSocketNotifier::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
  }
  impl ::socket_notifier::SocketNotifier {
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

/// C++ type: <span style='color: green;'>```QSocketNotifier::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```Read = 0```</span>
  Read = 0,
  /// C++ enum variant: <span style='color: green;'>```Write = 1```</span>
  Write = 1,
  /// C++ enum variant: <span style='color: green;'>```Exception = 2```</span>
  Exception = 2,
}

impl ::cpp_utils::DynamicCast<::socket_notifier::SocketNotifier> for ::object::Object {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::socket_notifier::SocketNotifier> {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QSocketNotifier_G_dynamic_cast_QSocketNotifier_ptr(self as *mut ::object::Object) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::socket_notifier::SocketNotifier> {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSocketNotifier_G_dynamic_cast_QSocketNotifier_ptr(self as *const ::object::Object as *mut ::object::Object) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::object::Object> for ::socket_notifier::SocketNotifier {
  fn static_cast_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QSocketNotifier_G_static_cast_QObject_ptr(self as *mut ::socket_notifier::SocketNotifier)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSocketNotifier_G_static_cast_QObject_ptr(self as *const ::socket_notifier::SocketNotifier as *mut ::socket_notifier::SocketNotifier) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::socket_notifier::SocketNotifier> for ::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::socket_notifier::SocketNotifier {
    let ffi_result =
      ::ffi::qt_core_c_QSocketNotifier_G_static_cast_QSocketNotifier_ptr(self as *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::socket_notifier::SocketNotifier {
    let ffi_result = ::ffi::qt_core_c_QSocketNotifier_G_static_cast_QSocketNotifier_ptr(self as *const ::object::Object as *mut ::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::socket_notifier::SocketNotifier {
  type Target = ::object::Object;
  fn deref(&self) -> &::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSocketNotifier_G_static_cast_QObject_ptr(self as *const ::socket_notifier::SocketNotifier as *mut ::socket_notifier::SocketNotifier) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::socket_notifier::SocketNotifier {
  fn deref_mut(&mut self) -> &mut ::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QSocketNotifier_G_static_cast_QObject_ptr(self as *mut ::socket_notifier::SocketNotifier)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
