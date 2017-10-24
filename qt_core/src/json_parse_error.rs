/// C++ type: <span style='color: green;'>```QJsonParseError```</span>
#[repr(C)]
pub struct JsonParseError([u8; ::type_sizes::QT_CORE_JSON_PARSE_ERROR_JSON_PARSE_ERROR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for JsonParseError {
  unsafe fn new_uninitialized() -> JsonParseError {
    JsonParseError(::std::mem::uninitialized())
  }
}

impl JsonParseError {
  /// C++ method: <span style='color: green;'>```QJsonParseError::ParseError QJsonParseError::error() const```</span>
  ///
  ///
  pub fn error(&self) -> ::json_parse_error::ParseError {
    unsafe { ::ffi::qt_core_c_QJsonParseError_error(self as *const ::json_parse_error::JsonParseError) }
  }

  /// C++ method: <span style='color: green;'>```QString QJsonParseError::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonParseError_errorString_to_output(self as *const ::json_parse_error::JsonParseError,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QJsonParseError::offset() const```</span>
  ///
  ///
  pub fn offset(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QJsonParseError_offset(self as *const ::json_parse_error::JsonParseError) }
  }

  /// C++ method: <span style='color: green;'>```void QJsonParseError::set_error(QJsonParseError::ParseError value)```</span>
  ///
  ///
  pub fn set_error(&mut self, value: ::json_parse_error::ParseError) {
    unsafe { ::ffi::qt_core_c_QJsonParseError_set_error(self as *mut ::json_parse_error::JsonParseError, value) }
  }

  /// C++ method: <span style='color: green;'>```void QJsonParseError::set_offset(int value)```</span>
  ///
  ///
  pub fn set_offset(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QJsonParseError_set_offset(self as *mut ::json_parse_error::JsonParseError, value) }
  }
}

impl Drop for ::json_parse_error::JsonParseError {
  /// C++ method: <span style='color: green;'>```[destructor] void QJsonParseError::~QJsonParseError()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonParseError_destructor(self as *mut ::json_parse_error::JsonParseError) }
  }
}

/// C++ type: <span style='color: green;'>```QJsonParseError::ParseError```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ParseError {
  /// C++ enum variant: <span style='color: green;'>```NoError = 0```</span>
  NoError = 0,
  /// C++ enum variant: <span style='color: green;'>```UnterminatedObject = 1```</span>
  UnterminatedObject = 1,
  /// C++ enum variant: <span style='color: green;'>```MissingNameSeparator = 2```</span>
  MissingNameSeparator = 2,
  /// C++ enum variant: <span style='color: green;'>```UnterminatedArray = 3```</span>
  UnterminatedArray = 3,
  /// C++ enum variant: <span style='color: green;'>```MissingValueSeparator = 4```</span>
  MissingValueSeparator = 4,
  /// C++ enum variant: <span style='color: green;'>```IllegalValue = 5```</span>
  IllegalValue = 5,
  /// C++ enum variant: <span style='color: green;'>```TerminationByNumber = 6```</span>
  TerminationByNumber = 6,
  /// C++ enum variant: <span style='color: green;'>```IllegalNumber = 7```</span>
  IllegalNumber = 7,
  /// C++ enum variant: <span style='color: green;'>```IllegalEscapeSequence = 8```</span>
  IllegalEscapeSequence = 8,
  /// C++ enum variant: <span style='color: green;'>```IllegalUTF8String = 9```</span>
  IllegalUTF8String = 9,
  /// C++ enum variant: <span style='color: green;'>```UnterminatedString = 10```</span>
  UnterminatedString = 10,
  /// C++ enum variant: <span style='color: green;'>```MissingObject = 11```</span>
  MissingObject = 11,
  /// C++ enum variant: <span style='color: green;'>```DeepNesting = 12```</span>
  DeepNesting = 12,
  /// C++ enum variant: <span style='color: green;'>```DocumentTooLarge = 13```</span>
  DocumentTooLarge = 13,
  /// C++ enum variant: <span style='color: green;'>```GarbageAtEnd = 14```</span>
  GarbageAtEnd = 14,
}
