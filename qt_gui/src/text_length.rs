/// C++ type: <span style='color: green;'>```QTextLength```</span>
#[repr(C)]
pub struct TextLength([u8; ::type_sizes::QT_GUI_TEXT_LENGTH_TEXT_LENGTH]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextLength {
  unsafe fn new_uninitialized() -> TextLength {
    TextLength(::std::mem::uninitialized())
  }
}

impl TextLength {
  /// C++ method: <span style='color: green;'>```QVariant QTextLength::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLength_convert_to_QVariant_to_output(self as *const ::text_length::TextLength,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextLength::QTextLength```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::text_length::TextLength```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextLength::QTextLength()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::text_length::Type, ::libc::c_double)) -> ::text_length::TextLength```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextLength::QTextLength(QTextLength::Type type, double value)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::text_length::TextLength
    where Args: overloading::TextLengthNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QTextLength::operator==(const QTextLength& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::text_length::TextLength) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextLength_operator_eq(self as *const ::text_length::TextLength,
                                              other as *const ::text_length::TextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextLength::operator!=(const QTextLength& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::text_length::TextLength) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QTextLength_operator_neq(self as *const ::text_length::TextLength,
                                               other as *const ::text_length::TextLength)
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextLength::rawValue() const```</span>
  ///
  ///
  pub fn raw_value(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLength_rawValue(self as *const ::text_length::TextLength) }
  }

  /// C++ method: <span style='color: green;'>```QTextLength::Type QTextLength::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::text_length::Type {
    unsafe { ::ffi::qt_gui_c_QTextLength_type(self as *const ::text_length::TextLength) }
  }

  /// C++ method: <span style='color: green;'>```double QTextLength::value(double maximumLength) const```</span>
  ///
  ///
  pub fn value(&self, maximum_length: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLength_value(self as *const ::text_length::TextLength, maximum_length) }
  }
}

impl Drop for ::text_length::TextLength {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextLength::~QTextLength()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextLength_destructor(self as *mut ::text_length::TextLength) }
  }
}

/// C++ type: <span style='color: green;'>```QTextLength::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```VariableLength = 0```</span>
  Variable = 0,
  /// C++ enum variant: <span style='color: green;'>```FixedLength = 1```</span>
  Fixed = 1,
  /// C++ enum variant: <span style='color: green;'>```PercentageLength = 2```</span>
  Percentage = 2,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextLength::new](../struct.TextLength.html#method.new) method.
  pub trait TextLengthNewArgs {
    fn exec(self) -> ::text_length::TextLength;
  }
  impl TextLengthNewArgs for () {
    fn exec(self) -> ::text_length::TextLength {

      {
        let mut object: ::text_length::TextLength =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextLength_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl TextLengthNewArgs for (::text_length::Type, ::libc::c_double) {
    fn exec(self) -> ::text_length::TextLength {
      let type_ = self.0;
      let value = self.1;
      {
        let mut object: ::text_length::TextLength =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextLength_constructor_type_value(type_, value, &mut object);
        }
        object
      }
    }
  }
}
