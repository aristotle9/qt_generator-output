/// C++ type: <span style='color: green;'>```QIntValidator```</span>
#[repr(C)]
pub struct IntValidator(u8);

impl IntValidator {
  /// C++ method: <span style='color: green;'>```int QIntValidator::bottom() const```</span>
  ///
  ///
  pub fn bottom(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QIntValidator_bottom(self as *const ::int_validator::IntValidator) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QIntValidator::fixup(QString& input) const```</span>
  ///
  ///
  pub fn fixup(&self, input: &mut ::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QIntValidator_fixup(self as *const ::int_validator::IntValidator,
                                          input as *mut ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QIntValidator::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QIntValidator_metaObject(self as *const ::int_validator::IntValidator) }
  }

  /// C++ method: <span style='color: green;'>```QIntValidator::QIntValidator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::int_validator::IntValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QIntValidator::QIntValidator()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::int_validator::IntValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QIntValidator::QIntValidator(int bottom, int top)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::int_validator::IntValidator>
    where Args: overloading::IntValidatorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QIntValidator::QIntValidator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::object::Object) -> ::cpp_utils::CppBox<::int_validator::IntValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QIntValidator::QIntValidator(QObject* parent = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((::libc::c_int, ::libc::c_int, *mut ::qt_core::object::Object)) -> ::cpp_utils::CppBox<::int_validator::IntValidator>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QIntValidator::QIntValidator(int bottom, int top, QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::int_validator::IntValidator>
    where Args: overloading::IntValidatorNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```virtual int QIntValidator::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QIntValidator_qt_metacall(self as *mut ::int_validator::IntValidator,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QIntValidator::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QIntValidator_qt_metacast(self as *mut ::int_validator::IntValidator, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QIntValidator::setBottom(int arg1)```</span>
  ///
  ///
  pub fn set_bottom(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QIntValidator_setBottom(self as *mut ::int_validator::IntValidator, arg1) }
  }

  /// C++ method: <span style='color: green;'>```virtual void QIntValidator::setRange(int bottom, int top)```</span>
  ///
  ///
  pub fn set_range(&mut self, bottom: ::libc::c_int, top: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QIntValidator_setRange(self as *mut ::int_validator::IntValidator, bottom, top) }
  }

  /// C++ method: <span style='color: green;'>```void QIntValidator::setTop(int arg1)```</span>
  ///
  ///
  pub fn set_top(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QIntValidator_setTop(self as *mut ::int_validator::IntValidator, arg1) }
  }

  /// C++ method: <span style='color: green;'>```int QIntValidator::top() const```</span>
  ///
  ///
  pub fn top(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QIntValidator_top(self as *const ::int_validator::IntValidator) }
  }

  /// C++ method: <span style='color: green;'>```static QString QIntValidator::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QIntValidator_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QIntValidator::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QIntValidator_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::int_validator::IntValidator {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QIntValidator_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `IntValidator`.
  pub struct Signals<'a>(&'a ::int_validator::IntValidator);
  /// Represents a built-in Qt signal `QIntValidator::bottomChanged`.
  ///
  /// An object of this type can be created from `IntValidator` with `object.signals().bottom_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IntValidator` object.
  pub struct BottomChanged<'a>(&'a ::int_validator::IntValidator);
  impl<'a> ::qt_core::connection::Receiver for BottomChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2bottomChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BottomChanged<'a> {}
  /// Represents a built-in Qt signal `QIntValidator::topChanged`.
  ///
  /// An object of this type can be created from `IntValidator` with `object.signals().top_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IntValidator` object.
  pub struct TopChanged<'a>(&'a ::int_validator::IntValidator);
  impl<'a> ::qt_core::connection::Receiver for TopChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2topChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TopChanged<'a> {}
  /// Represents a built-in Qt signal `QIntValidator::changed`.
  ///
  /// An object of this type can be created from `IntValidator` with `object.signals().changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `IntValidator` object.
  pub struct Changed<'a>(&'a ::int_validator::IntValidator);
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
    /// Returns an object representing a built-in Qt signal `QIntValidator::bottomChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bottom_changed(&self) -> BottomChanged {
      BottomChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QIntValidator::topChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn top_changed(&self) -> TopChanged {
      TopChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QIntValidator::changed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn changed(&self) -> Changed {
      Changed(self.0)
    }
  }
  impl ::int_validator::IntValidator {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

impl ::cpp_utils::DynamicCast<::int_validator::IntValidator> for ::validator::Validator {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::int_validator::IntValidator> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QIntValidator_G_dynamic_cast_QIntValidator_ptr(self as *mut ::validator::Validator) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::int_validator::IntValidator> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QIntValidator_G_dynamic_cast_QIntValidator_ptr(self as *const ::validator::Validator as *mut ::validator::Validator) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::int_validator::IntValidator {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QIntValidator_G_static_cast_QObject_ptr(self as *mut ::int_validator::IntValidator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QIntValidator_G_static_cast_QObject_ptr(self as *const ::int_validator::IntValidator as *mut ::int_validator::IntValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::validator::Validator> for ::int_validator::IntValidator {
  fn static_cast_mut(&mut self) -> &mut ::validator::Validator {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QIntValidator_G_static_cast_QValidator_ptr(self as *mut ::int_validator::IntValidator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::validator::Validator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QIntValidator_G_static_cast_QValidator_ptr(self as *const ::int_validator::IntValidator as *mut ::int_validator::IntValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::int_validator::IntValidator> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::int_validator::IntValidator {
    let ffi_result =
      ::ffi::qt_gui_c_QIntValidator_G_static_cast_QIntValidator_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::int_validator::IntValidator {
    let ffi_result = ::ffi::qt_gui_c_QIntValidator_G_static_cast_QIntValidator_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::int_validator::IntValidator> for ::validator::Validator {
  unsafe fn static_cast_mut(&mut self) -> &mut ::int_validator::IntValidator {
    let ffi_result =
      ::ffi::qt_gui_c_QIntValidator_G_static_cast_QIntValidator_ptr_QValidator(self as *mut ::validator::Validator);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::int_validator::IntValidator {
    let ffi_result = ::ffi::qt_gui_c_QIntValidator_G_static_cast_QIntValidator_ptr_QValidator(self as *const ::validator::Validator as *mut ::validator::Validator);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::int_validator::IntValidator {
  type Target = ::validator::Validator;
  fn deref(&self) -> &::validator::Validator {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QIntValidator_G_static_cast_QValidator_ptr(self as *const ::int_validator::IntValidator as *mut ::int_validator::IntValidator) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::int_validator::IntValidator {
  fn deref_mut(&mut self) -> &mut ::validator::Validator {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QIntValidator_G_static_cast_QValidator_ptr(self as *mut ::int_validator::IntValidator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [IntValidator::new](../struct.IntValidator.html#method.new) method.
  pub trait IntValidatorNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::int_validator::IntValidator>;
  }
  impl IntValidatorNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::cpp_utils::CppBox<::int_validator::IntValidator> {
      let bottom = self.0;
      let top = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QIntValidator_new_bottom_top(bottom, top) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl IntValidatorNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::int_validator::IntValidator> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QIntValidator_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [IntValidator::new_unsafe](../struct.IntValidator.html#method.new_unsafe) method.
  pub trait IntValidatorNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::int_validator::IntValidator>;
  }
  impl IntValidatorNewUnsafeArgs for (::libc::c_int, ::libc::c_int, *mut ::qt_core::object::Object) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::int_validator::IntValidator> {
      let bottom = self.0;
      let top = self.1;
      let parent = self.2;
      let ffi_result = ::ffi::qt_gui_c_QIntValidator_new_bottom_top_parent(bottom, top, parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl IntValidatorNewUnsafeArgs for *mut ::qt_core::object::Object {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::int_validator::IntValidator> {
      let parent = self;
      let ffi_result = ::ffi::qt_gui_c_QIntValidator_new_parent(parent);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
}
