/// C++ type: <span style='color: green;'>```QRegExpValidator```</span>
#[repr(C)]
pub struct RegExpValidator(u8);

impl RegExpValidator {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QRegExpValidator::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QRegExpValidator_metaObject(self as *const ::reg_exp_validator::RegExpValidator) }
  }

  /// C++ method: <span style='color: green;'>```QRegExpValidator::QRegExpValidator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegExpValidator::QRegExpValidator()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::reg_exp::RegExp) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegExpValidator::QRegExpValidator(const QRegExp& rx)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator>
    where Args: overloading::RegExpValidatorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QRegExpValidator::QRegExpValidator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegExpValidator::QRegExpValidator(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::reg_exp::RegExp, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegExpValidator::QRegExpValidator(const QRegExp& rx, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator>
    where Args: overloading::RegExpValidatorNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QRegExpValidator::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QRegExpValidator_qt_metacall(self as *mut ::reg_exp_validator::RegExpValidator,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QRegExpValidator::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QRegExpValidator_qt_metacast(self as *mut ::reg_exp_validator::RegExpValidator, arg1)
  }

  /// C++ method: <span style='color: green;'>```const QRegExp& QRegExpValidator::regExp() const```</span>
  ///
  ///
  pub fn reg_exp<'l0>(&'l0 self) -> &'l0 ::qt_core::reg_exp::RegExp {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QRegExpValidator_regExp(self as *const ::reg_exp_validator::RegExpValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QRegExpValidator::setRegExp(const QRegExp& rx)```</span>
  ///
  ///
  pub fn set_reg_exp(&mut self, rx: &::qt_core::reg_exp::RegExp) {
    unsafe {
      ::ffi::qt_gui_c_QRegExpValidator_setRegExp(self as *mut ::reg_exp_validator::RegExpValidator,
                                                 rx as *const ::qt_core::reg_exp::RegExp)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QRegExpValidator::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QRegExpValidator_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QRegExpValidator::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QRegExpValidator_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::reg_exp_validator::RegExpValidator {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QRegExpValidator_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `RegExpValidator`.
  pub struct Signals<'a>(&'a ::reg_exp_validator::RegExpValidator);
  /// Represents a built-in Qt signal `QRegExpValidator::regExpChanged`.
  ///
  /// An object of this type can be created from `RegExpValidator` with `object.signals().reg_exp_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RegExpValidator` object.
  pub struct RegExpChanged<'a>(&'a ::reg_exp_validator::RegExpValidator);
  impl<'a> ::qt_core::connection::Receiver for RegExpChanged<'a> {
    type Arguments = (&'static ::qt_core::reg_exp::RegExp,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2regExpChanged(const QRegExp&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RegExpChanged<'a> {}
  /// Represents a built-in Qt signal `QRegExpValidator::changed`.
  ///
  /// An object of this type can be created from `RegExpValidator` with `object.signals().changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RegExpValidator` object.
  pub struct Changed<'a>(&'a ::reg_exp_validator::RegExpValidator);
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
    /// Returns an object representing a built-in Qt signal `QRegExpValidator::regExpChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn reg_exp_changed(&self) -> RegExpChanged {
      RegExpChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QRegExpValidator::changed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn changed(&self) -> Changed {
      Changed(self.0)
    }
  }
  impl ::reg_exp_validator::RegExpValidator {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::reg_exp_validator::RegExpValidator> for ::validator::Validator {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::reg_exp_validator::RegExpValidator> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QRegExpValidator_G_dynamic_cast_QRegExpValidator_ptr(self as *mut ::validator::Validator)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::reg_exp_validator::RegExpValidator> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRegExpValidator_G_dynamic_cast_QRegExpValidator_ptr(self as *const ::validator::Validator as *mut ::validator::Validator) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::reg_exp_validator::RegExpValidator {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QRegExpValidator_G_static_cast_QObject_ptr(self as *mut ::reg_exp_validator::RegExpValidator)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRegExpValidator_G_static_cast_QObject_ptr(self as *const ::reg_exp_validator::RegExpValidator as *mut ::reg_exp_validator::RegExpValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::validator::Validator> for ::reg_exp_validator::RegExpValidator {
  fn static_cast_mut(&mut self) -> &mut ::validator::Validator {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QRegExpValidator_G_static_cast_QValidator_ptr(self as *mut ::reg_exp_validator::RegExpValidator)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::validator::Validator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRegExpValidator_G_static_cast_QValidator_ptr(self as *const ::reg_exp_validator::RegExpValidator as *mut ::reg_exp_validator::RegExpValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::reg_exp_validator::RegExpValidator> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::reg_exp_validator::RegExpValidator {
    let ffi_result = ::ffi::qt_gui_c_QRegExpValidator_G_static_cast_QRegExpValidator_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::reg_exp_validator::RegExpValidator {
    let ffi_result = ::ffi::qt_gui_c_QRegExpValidator_G_static_cast_QRegExpValidator_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::reg_exp_validator::RegExpValidator> for ::validator::Validator {
  unsafe fn static_cast_mut(&mut self) -> &mut ::reg_exp_validator::RegExpValidator {
    let ffi_result = ::ffi::qt_gui_c_QRegExpValidator_G_static_cast_QRegExpValidator_ptr_QValidator(self as *mut ::validator::Validator);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::reg_exp_validator::RegExpValidator {
    let ffi_result = ::ffi::qt_gui_c_QRegExpValidator_G_static_cast_QRegExpValidator_ptr_QValidator(self as *const ::validator::Validator as *mut ::validator::Validator);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::reg_exp_validator::RegExpValidator {
  type Target = ::validator::Validator;
  fn deref(&self) -> &::validator::Validator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRegExpValidator_G_static_cast_QValidator_ptr(self as *const ::reg_exp_validator::RegExpValidator as *mut ::reg_exp_validator::RegExpValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::reg_exp_validator::RegExpValidator {
  fn deref_mut(&mut self) -> &mut ::validator::Validator {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QRegExpValidator_G_static_cast_QValidator_ptr(self as *mut ::reg_exp_validator::RegExpValidator)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [RegExpValidator::new](../struct.RegExpValidator.html#method.new) method.
  pub trait RegExpValidatorNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator>;
  }
  impl RegExpValidatorNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegExpValidator_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> RegExpValidatorNewArgs for &'a ::qt_core::reg_exp::RegExp {
    fn exec(self) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator> {
      let rx = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegExpValidator_new_rx(rx as *const ::qt_core::reg_exp::RegExp) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [RegExpValidator::new_unsafe](../struct.RegExpValidator.html#method.new_unsafe) method.
  pub trait RegExpValidatorNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator>;
  }
  impl RegExpValidatorNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator> {
      let parent = self;
      let ffi_result = ::ffi::qt_gui_c_QRegExpValidator_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> RegExpValidatorNewUnsafeArgs for (&'a ::qt_core::reg_exp::RegExp, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::reg_exp_validator::RegExpValidator> {
      let rx = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_gui_c_QRegExpValidator_new_rx_parent(rx as *const ::qt_core::reg_exp::RegExp, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
