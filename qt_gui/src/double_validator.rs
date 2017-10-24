/// C++ type: <span style='color: green;'>```QDoubleValidator```</span>
#[repr(C)]
pub struct DoubleValidator(u8);

impl DoubleValidator {
  /// C++ method: <span style='color: green;'>```double QDoubleValidator::bottom() const```</span>
  ///
  ///
  pub fn bottom(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QDoubleValidator_bottom(self as *const ::double_validator::DoubleValidator) }
  }

  /// C++ method: <span style='color: green;'>```int QDoubleValidator::decimals() const```</span>
  ///
  ///
  pub fn decimals(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QDoubleValidator_decimals(self as *const ::double_validator::DoubleValidator) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QDoubleValidator::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QDoubleValidator_metaObject(self as *const ::double_validator::DoubleValidator) }
  }

  /// C++ method: <span style='color: green;'>```QDoubleValidator::QDoubleValidator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDoubleValidator::QDoubleValidator()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_int)) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDoubleValidator::QDoubleValidator(double bottom, double top, int decimals)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator>
    where Args: overloading::DoubleValidatorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDoubleValidator::QDoubleValidator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDoubleValidator::QDoubleValidator(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::libc::c_double, ::libc::c_double, ::libc::c_int, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QDoubleValidator::QDoubleValidator(double bottom, double top, int decimals, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator>
    where Args: overloading::DoubleValidatorNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QDoubleValidator::Notation QDoubleValidator::notation() const```</span>
  ///
  ///
  pub fn notation(&self) -> ::double_validator::Notation {
    unsafe { ::ffi::qt_gui_c_QDoubleValidator_notation(self as *const ::double_validator::DoubleValidator) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QDoubleValidator::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QDoubleValidator_qt_metacall(self as *mut ::double_validator::DoubleValidator,
                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                 arg2,
                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QDoubleValidator::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QDoubleValidator_qt_metacast(self as *mut ::double_validator::DoubleValidator, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QDoubleValidator::setBottom(double arg1)```</span>
  ///
  ///
  pub fn set_bottom(&mut self, arg1: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QDoubleValidator_setBottom(self as *mut ::double_validator::DoubleValidator, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QDoubleValidator::setDecimals(int arg1)```</span>
  ///
  ///
  pub fn set_decimals(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QDoubleValidator_setDecimals(self as *mut ::double_validator::DoubleValidator, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QDoubleValidator::setNotation(QDoubleValidator::Notation arg1)```</span>
  ///
  ///
  pub fn set_notation(&mut self, arg1: ::double_validator::Notation) {
    unsafe { ::ffi::qt_gui_c_QDoubleValidator_setNotation(self as *mut ::double_validator::DoubleValidator, arg1) }
  }

  /// C++ method: <span style='color: green;'>```QDoubleValidator::setRange```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_range(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QDoubleValidator::setRange(double bottom, double top)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_range(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QDoubleValidator::setRange(double bottom, double top, int decimals = ?)```</span>
  ///
  ///
  pub fn set_range<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::DoubleValidatorSetRangeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QDoubleValidator::setTop(double arg1)```</span>
  ///
  ///
  pub fn set_top(&mut self, arg1: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QDoubleValidator_setTop(self as *mut ::double_validator::DoubleValidator, arg1) }
  }

  /// C++ method: <span style='color: green;'>```double QDoubleValidator::top() const```</span>
  ///
  ///
  pub fn top(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QDoubleValidator_top(self as *const ::double_validator::DoubleValidator) }
  }

  /// C++ method: <span style='color: green;'>```static QString QDoubleValidator::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QDoubleValidator_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QDoubleValidator::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QDoubleValidator_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::double_validator::DoubleValidator {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QDoubleValidator_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `DoubleValidator`.
  pub struct Signals<'a>(&'a ::double_validator::DoubleValidator);
  /// Represents a built-in Qt signal `QDoubleValidator::changed`.
  ///
  /// An object of this type can be created from `DoubleValidator` with `object.signals().changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleValidator` object.
  pub struct Changed<'a>(&'a ::double_validator::DoubleValidator);
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
  /// Represents a built-in Qt signal `QDoubleValidator::topChanged`.
  ///
  /// An object of this type can be created from `DoubleValidator` with `object.signals().top_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleValidator` object.
  pub struct TopChanged<'a>(&'a ::double_validator::DoubleValidator);
  impl<'a> ::qt_core::connection::Receiver for TopChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2topChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TopChanged<'a> {}
  /// Represents a built-in Qt signal `QDoubleValidator::notationChanged`.
  ///
  /// An object of this type can be created from `DoubleValidator` with `object.signals().notation_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleValidator` object.
  pub struct NotationChanged<'a>(&'a ::double_validator::DoubleValidator);
  impl<'a> ::qt_core::connection::Receiver for NotationChanged<'a> {
    type Arguments = (&'static ::double_validator::Notation,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2notationChanged(QDoubleValidator::Notation)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NotationChanged<'a> {}
  /// Represents a built-in Qt signal `QDoubleValidator::decimalsChanged`.
  ///
  /// An object of this type can be created from `DoubleValidator` with `object.signals().decimals_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleValidator` object.
  pub struct DecimalsChanged<'a>(&'a ::double_validator::DoubleValidator);
  impl<'a> ::qt_core::connection::Receiver for DecimalsChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2decimalsChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DecimalsChanged<'a> {}
  /// Represents a built-in Qt signal `QDoubleValidator::bottomChanged`.
  ///
  /// An object of this type can be created from `DoubleValidator` with `object.signals().bottom_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `DoubleValidator` object.
  pub struct BottomChanged<'a>(&'a ::double_validator::DoubleValidator);
  impl<'a> ::qt_core::connection::Receiver for BottomChanged<'a> {
    type Arguments = (::libc::c_double,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2bottomChanged(double)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BottomChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QDoubleValidator::changed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn changed(&self) -> Changed {
      Changed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDoubleValidator::topChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn top_changed(&self) -> TopChanged {
      TopChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDoubleValidator::notationChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn notation_changed(&self) -> NotationChanged {
      NotationChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDoubleValidator::decimalsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn decimals_changed(&self) -> DecimalsChanged {
      DecimalsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QDoubleValidator::bottomChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bottom_changed(&self) -> BottomChanged {
      BottomChanged(self.0)
    }
  }
  impl ::double_validator::DoubleValidator {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QDoubleValidator::Notation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Notation {
  /// C++ enum variant: <span style='color: green;'>```StandardNotation = 0```</span>
  Standard = 0,
  /// C++ enum variant: <span style='color: green;'>```ScientificNotation = 1```</span>
  Scientific = 1,
}

impl ::cpp_utils::DynamicCast<::double_validator::DoubleValidator> for ::validator::Validator {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::double_validator::DoubleValidator> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QDoubleValidator_G_dynamic_cast_QDoubleValidator_ptr(self as *mut ::validator::Validator)
      };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::double_validator::DoubleValidator> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDoubleValidator_G_dynamic_cast_QDoubleValidator_ptr(self as *const ::validator::Validator as *mut ::validator::Validator) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::double_validator::DoubleValidator {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QDoubleValidator_G_static_cast_QObject_ptr(self as *mut ::double_validator::DoubleValidator)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDoubleValidator_G_static_cast_QObject_ptr(self as *const ::double_validator::DoubleValidator as *mut ::double_validator::DoubleValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::validator::Validator> for ::double_validator::DoubleValidator {
  fn static_cast_mut(&mut self) -> &mut ::validator::Validator {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QDoubleValidator_G_static_cast_QValidator_ptr(self as *mut ::double_validator::DoubleValidator)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::validator::Validator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDoubleValidator_G_static_cast_QValidator_ptr(self as *const ::double_validator::DoubleValidator as *mut ::double_validator::DoubleValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::double_validator::DoubleValidator> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::double_validator::DoubleValidator {
    let ffi_result = ::ffi::qt_gui_c_QDoubleValidator_G_static_cast_QDoubleValidator_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::double_validator::DoubleValidator {
    let ffi_result = ::ffi::qt_gui_c_QDoubleValidator_G_static_cast_QDoubleValidator_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::double_validator::DoubleValidator> for ::validator::Validator {
  unsafe fn static_cast_mut(&mut self) -> &mut ::double_validator::DoubleValidator {
    let ffi_result = ::ffi::qt_gui_c_QDoubleValidator_G_static_cast_QDoubleValidator_ptr_QValidator(self as *mut ::validator::Validator);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::double_validator::DoubleValidator {
    let ffi_result = ::ffi::qt_gui_c_QDoubleValidator_G_static_cast_QDoubleValidator_ptr_QValidator(self as *const ::validator::Validator as *mut ::validator::Validator);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::double_validator::DoubleValidator {
  type Target = ::validator::Validator;
  fn deref(&self) -> &::validator::Validator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QDoubleValidator_G_static_cast_QValidator_ptr(self as *const ::double_validator::DoubleValidator as *mut ::double_validator::DoubleValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::double_validator::DoubleValidator {
  fn deref_mut(&mut self) -> &mut ::validator::Validator {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QDoubleValidator_G_static_cast_QValidator_ptr(self as *mut ::double_validator::DoubleValidator)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [DoubleValidator::new](../struct.DoubleValidator.html#method.new) method.
  pub trait DoubleValidatorNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator>;
  }
  impl DoubleValidatorNewArgs for (::libc::c_double, ::libc::c_double, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator> {
      let bottom = self.0;
      let top = self.1;
      let decimals = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QDoubleValidator_new_bottom_top_decimals(bottom, top, decimals) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl DoubleValidatorNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QDoubleValidator_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [DoubleValidator::new_unsafe](../struct.DoubleValidator.html#method.new_unsafe) method.
  pub trait DoubleValidatorNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator>;
  }
  impl DoubleValidatorNewUnsafeArgs
    for (::libc::c_double, ::libc::c_double, ::libc::c_int, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator> {
      let bottom = self.0;
      let top = self.1;
      let decimals = self.2;
      let parent = self.3;
      let ffi_result = ::ffi::qt_gui_c_QDoubleValidator_new_bottom_top_decimals_parent(bottom, top, decimals, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl DoubleValidatorNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::double_validator::DoubleValidator> {
      let parent = self;
      let ffi_result = ::ffi::qt_gui_c_QDoubleValidator_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [DoubleValidator::set_range](../struct.DoubleValidator.html#method.set_range) method.
  pub trait DoubleValidatorSetRangeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::double_validator::DoubleValidator) -> ();
  }
  impl<'largs> DoubleValidatorSetRangeArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::double_validator::DoubleValidator) -> () {
      let bottom = self.0;
      let top = self.1;
      unsafe {
        ::ffi::qt_gui_c_QDoubleValidator_setRange_bottom_top(original_self as *mut ::double_validator::DoubleValidator, bottom, top)
      }
    }
  }
  impl<'largs> DoubleValidatorSetRangeArgs<'largs> for (::libc::c_double, ::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::double_validator::DoubleValidator) -> () {
      let bottom = self.0;
      let top = self.1;
      let decimals = self.2;
      unsafe { ::ffi::qt_gui_c_QDoubleValidator_setRange_bottom_top_decimals(original_self as *mut ::double_validator::DoubleValidator, bottom, top, decimals) }
    }
  }
}
