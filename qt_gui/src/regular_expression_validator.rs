/// C++ type: <span style='color: green;'>```QRegularExpressionValidator```</span>
#[repr(C)]
pub struct RegularExpressionValidator(u8);

impl RegularExpressionValidator {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QRegularExpressionValidator::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_metaObject(self as *const ::regular_expression_validator::RegularExpressionValidator) }
  }

  /// C++ method: <span style='color: green;'>```QRegularExpressionValidator::QRegularExpressionValidator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpressionValidator::QRegularExpressionValidator()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::regular_expression::RegularExpression) -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpressionValidator::QRegularExpressionValidator(const QRegularExpression& re)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator>
    where Args: overloading::RegularExpressionValidatorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QRegularExpressionValidator::QRegularExpressionValidator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpressionValidator::QRegularExpressionValidator(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::regular_expression::RegularExpression, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QRegularExpressionValidator::QRegularExpressionValidator(const QRegularExpression& re, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args)
                                 -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator>
    where Args: overloading::RegularExpressionValidatorNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QRegularExpressionValidator::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QRegularExpressionValidator_qt_metacall(self as *mut ::regular_expression_validator::RegularExpressionValidator, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QRegularExpressionValidator::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QRegularExpressionValidator_qt_metacast(self as *mut ::regular_expression_validator::RegularExpressionValidator, arg1)
  }

  /// C++ method: <span style='color: green;'>```QRegularExpression QRegularExpressionValidator::regularExpression() const```</span>
  ///
  ///
  pub fn regular_expression(&self) -> ::qt_core::regular_expression::RegularExpression {
    {
      let mut object: ::qt_core::regular_expression::RegularExpression =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRegularExpressionValidator_regularExpression_to_output(self as *const ::regular_expression_validator::RegularExpressionValidator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QRegularExpressionValidator::setRegularExpression(const QRegularExpression& re)```</span>
  ///
  ///
  pub fn set_regular_expression(&mut self, re: &::qt_core::regular_expression::RegularExpression) {
    unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_setRegularExpression(self as *mut ::regular_expression_validator::RegularExpressionValidator, re as *const ::qt_core::regular_expression::RegularExpression) }
  }

  /// C++ method: <span style='color: green;'>```static QString QRegularExpressionValidator::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QRegularExpressionValidator_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QRegularExpressionValidator::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QRegularExpressionValidator_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::regular_expression_validator::RegularExpressionValidator {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QRegularExpressionValidator_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `RegularExpressionValidator`.
  pub struct Signals<'a>(&'a ::regular_expression_validator::RegularExpressionValidator);
  /// Represents a built-in Qt signal `QRegularExpressionValidator::changed`.
  ///
  /// An object of this type can be created from `RegularExpressionValidator` with `object.signals().changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RegularExpressionValidator` object.
  pub struct Changed<'a>(&'a ::regular_expression_validator::RegularExpressionValidator);
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
  /// Represents a built-in Qt signal `QRegularExpressionValidator::regularExpressionChanged`.
  ///
  /// An object of this type can be created from `RegularExpressionValidator` with `object.signals().regular_expression_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RegularExpressionValidator` object.
  pub struct RegularExpressionChanged<'a>(&'a ::regular_expression_validator::RegularExpressionValidator);
  impl<'a> ::qt_core::connection::Receiver for RegularExpressionChanged<'a> {
    type Arguments = (&'static ::qt_core::regular_expression::RegularExpression,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2regularExpressionChanged(const QRegularExpression&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RegularExpressionChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QRegularExpressionValidator::changed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn changed(&self) -> Changed {
      Changed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QRegularExpressionValidator::regularExpressionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn regular_expression_changed(&self) -> RegularExpressionChanged {
      RegularExpressionChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `RegularExpressionValidator`.
  pub struct Slots<'a>(&'a ::regular_expression_validator::RegularExpressionValidator);
  /// Represents a built-in Qt slot `QRegularExpressionValidator::setRegularExpression`.
  ///
  /// An object of this type can be created from `RegularExpressionValidator` with `object.slots().set_regular_expression()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RegularExpressionValidator` object.
  pub struct SetRegularExpression<'a>(&'a ::regular_expression_validator::RegularExpressionValidator);
  impl<'a> ::qt_core::connection::Receiver for SetRegularExpression<'a> {
    type Arguments = (&'static ::qt_core::regular_expression::RegularExpression,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRegularExpression(const QRegularExpression&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QRegularExpressionValidator::setRegularExpression`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_regular_expression(&self) -> SetRegularExpression {
      SetRegularExpression(self.0)
    }
  }
  impl ::regular_expression_validator::RegularExpressionValidator {
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

impl ::cpp_utils::DynamicCast<::regular_expression_validator::RegularExpressionValidator> for ::validator::Validator {
  fn dynamic_cast_mut(&mut self)
                      -> ::std::option::Option<&mut ::regular_expression_validator::RegularExpressionValidator> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_G_dynamic_cast_QRegularExpressionValidator_ptr(self as *mut ::validator::Validator) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::regular_expression_validator::RegularExpressionValidator> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_G_dynamic_cast_QRegularExpressionValidator_ptr(self as *const ::validator::Validator as *mut ::validator::Validator) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::regular_expression_validator::RegularExpressionValidator {
fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_G_static_cast_QObject_ptr(self as *mut ::regular_expression_validator::RegularExpressionValidator) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_core::object::Object {
let ffi_result = unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_G_static_cast_QObject_ptr(self as *const ::regular_expression_validator::RegularExpressionValidator as *mut ::regular_expression_validator::RegularExpressionValidator) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::validator::Validator> for ::regular_expression_validator::RegularExpressionValidator {
  fn static_cast_mut(&mut self) -> &mut ::validator::Validator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_G_static_cast_QValidator_ptr(self as *mut ::regular_expression_validator::RegularExpressionValidator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::validator::Validator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_G_static_cast_QValidator_ptr(self as *const ::regular_expression_validator::RegularExpressionValidator as *mut ::regular_expression_validator::RegularExpressionValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::regular_expression_validator::RegularExpressionValidator> for ::qt_core::object::Object {
unsafe fn static_cast_mut(&mut self) -> &mut ::regular_expression_validator::RegularExpressionValidator {
let ffi_result = ::ffi::qt_gui_c_QRegularExpressionValidator_G_static_cast_QRegularExpressionValidator_ptr_QObject(self as *mut ::qt_core::object::Object);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::regular_expression_validator::RegularExpressionValidator {
let ffi_result = ::ffi::qt_gui_c_QRegularExpressionValidator_G_static_cast_QRegularExpressionValidator_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::UnsafeStaticCast<::regular_expression_validator::RegularExpressionValidator> for ::validator::Validator {
unsafe fn static_cast_mut(&mut self) -> &mut ::regular_expression_validator::RegularExpressionValidator {
let ffi_result = ::ffi::qt_gui_c_QRegularExpressionValidator_G_static_cast_QRegularExpressionValidator_ptr_QValidator(self as *mut ::validator::Validator);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::regular_expression_validator::RegularExpressionValidator {
let ffi_result = ::ffi::qt_gui_c_QRegularExpressionValidator_G_static_cast_QRegularExpressionValidator_ptr_QValidator(self as *const ::validator::Validator as *mut ::validator::Validator);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::regular_expression_validator::RegularExpressionValidator {
  type Target = ::validator::Validator;
  fn deref(&self) -> &::validator::Validator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_G_static_cast_QValidator_ptr(self as *const ::regular_expression_validator::RegularExpressionValidator as *mut ::regular_expression_validator::RegularExpressionValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::regular_expression_validator::RegularExpressionValidator {
  fn deref_mut(&mut self) -> &mut ::validator::Validator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_G_static_cast_QValidator_ptr(self as *mut ::regular_expression_validator::RegularExpressionValidator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [RegularExpressionValidator::new](../struct.RegularExpressionValidator.html#method.new) method.
  pub trait RegularExpressionValidatorNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator>;
  }
  impl RegularExpressionValidatorNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> RegularExpressionValidatorNewArgs for &'a ::qt_core::regular_expression::RegularExpression {
    fn exec(self) -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator> {
      let re = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QRegularExpressionValidator_new_re(re as *const ::qt_core::regular_expression::RegularExpression) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [RegularExpressionValidator::new_unsafe](../struct.RegularExpressionValidator.html#method.new_unsafe) method.
  pub trait RegularExpressionValidatorNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator>;
  }
  impl RegularExpressionValidatorNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator> {
      let parent = self;
      let ffi_result = ::ffi::qt_gui_c_QRegularExpressionValidator_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl<'a> RegularExpressionValidatorNewUnsafeArgs
    for (&'a ::qt_core::regular_expression::RegularExpression, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::regular_expression_validator::RegularExpressionValidator> {
      let re = self.0;
      let parent = self.1;
      let ffi_result = ::ffi::qt_gui_c_QRegularExpressionValidator_new_re_parent(re as *const ::qt_core::regular_expression::RegularExpression, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
