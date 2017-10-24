/// C++ type: <span style='color: green;'>```QValidator::State```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum State {
  /// C++ enum variant: <span style='color: green;'>```Invalid = 0```</span>
  Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Intermediate = 1```</span>
  Intermediate = 1,
  /// C++ enum variant: <span style='color: green;'>```Acceptable = 2```</span>
  Acceptable = 2,
}

/// C++ type: <span style='color: green;'>```QValidator```</span>
#[repr(C)]
pub struct Validator(u8);

impl Validator {
  /// C++ method: <span style='color: green;'>```virtual void QValidator::fixup(QString& arg1) const```</span>
  ///
  ///
  pub fn fixup(&self, arg1: &mut ::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QValidator_fixup(self as *const ::validator::Validator,
                                       arg1 as *mut ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QLocale QValidator::locale() const```</span>
  ///
  ///
  pub fn locale(&self) -> ::qt_core::locale::Locale {
    {
      let mut object: ::qt_core::locale::Locale =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QValidator_locale_to_output(self as *const ::validator::Validator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QValidator::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QValidator_metaObject(self as *const ::validator::Validator) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QValidator::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QValidator_qt_metacall(self as *mut ::validator::Validator,
                                           arg1 as *const ::qt_core::meta_object::Call,
                                           arg2,
                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QValidator::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QValidator_qt_metacast(self as *mut ::validator::Validator, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QValidator::setLocale(const QLocale& locale)```</span>
  ///
  ///
  pub fn set_locale(&mut self, locale: &::qt_core::locale::Locale) {
    unsafe {
      ::ffi::qt_gui_c_QValidator_setLocale(self as *mut ::validator::Validator,
                                           locale as *const ::qt_core::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QValidator::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QValidator_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QValidator::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QValidator_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QValidator::State QValidator::validate(QString& arg1, int& arg2) const```</span>
  ///
  ///
  pub fn validate(&self, arg1: &mut ::qt_core::string::String, arg2: &mut ::libc::c_int) -> ::validator::State {
    unsafe {
      ::ffi::qt_gui_c_QValidator_validate(self as *const ::validator::Validator,
                                          arg1 as *mut ::qt_core::string::String,
                                          arg2 as *mut ::libc::c_int)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::validator::Validator {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QValidator_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Validator`.
  pub struct Signals<'a>(&'a ::validator::Validator);
  /// Represents a built-in Qt signal `QValidator::objectNameChanged`.
  ///
  /// An object of this type can be created from `Validator` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Validator` object.
  pub struct ObjectNameChanged<'a>(&'a ::validator::Validator);
  impl<'a> ::qt_core::connection::Receiver for ObjectNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2objectNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ObjectNameChanged<'a> {}
  /// Represents a built-in Qt signal `QValidator::changed`.
  ///
  /// An object of this type can be created from `Validator` with `object.signals().changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Validator` object.
  pub struct Changed<'a>(&'a ::validator::Validator);
  impl<'a> ::qt_core::connection::Receiver for Changed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2changed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for Changed<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QValidator::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QValidator::changed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn changed(&self) -> Changed {
      Changed(self.0)
    }
  }
  impl ::validator::Validator {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::validator::Validator {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QValidator_G_static_cast_QObject_ptr(self as *mut ::validator::Validator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QValidator_G_static_cast_QObject_ptr(self as *const ::validator::Validator as *mut ::validator::Validator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::validator::Validator> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::validator::Validator {
    let ffi_result = ::ffi::qt_gui_c_QValidator_G_static_cast_QValidator_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::validator::Validator {
    let ffi_result = ::ffi::qt_gui_c_QValidator_G_static_cast_QValidator_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::validator::Validator {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QValidator_G_static_cast_QObject_ptr(self as *const ::validator::Validator as *mut ::validator::Validator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::validator::Validator {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QValidator_G_static_cast_QObject_ptr(self as *mut ::validator::Validator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
