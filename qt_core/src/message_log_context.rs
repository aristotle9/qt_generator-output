/// C++ type: <span style='color: green;'>```QMessageLogContext```</span>
#[repr(C)]
pub struct MessageLogContext([u8; ::type_sizes::QT_CORE_MESSAGE_LOG_CONTEXT_MESSAGE_LOG_CONTEXT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MessageLogContext {
  unsafe fn new_uninitialized() -> MessageLogContext {
    MessageLogContext(::std::mem::uninitialized())
  }
}

impl MessageLogContext {
  /// C++ method: <span style='color: green;'>```const char* QMessageLogContext::category() const```</span>
  ///
  ///
  pub fn category(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMessageLogContext_category(self as *const ::message_log_context::MessageLogContext) }
  }

  /// C++ method: <span style='color: green;'>```const char* QMessageLogContext::file() const```</span>
  ///
  ///
  pub fn file(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMessageLogContext_file(self as *const ::message_log_context::MessageLogContext) }
  }

  /// C++ method: <span style='color: green;'>```const char* QMessageLogContext::function() const```</span>
  ///
  ///
  pub fn function(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMessageLogContext_function(self as *const ::message_log_context::MessageLogContext) }
  }

  /// C++ method: <span style='color: green;'>```int QMessageLogContext::line() const```</span>
  ///
  ///
  pub fn line(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMessageLogContext_line(self as *const ::message_log_context::MessageLogContext) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMessageLogContext::QMessageLogContext()```</span>
  ///
  ///
  pub fn new() -> ::message_log_context::MessageLogContext {
    {
      let mut object: ::message_log_context::MessageLogContext =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMessageLogContext_constructor_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMessageLogContext::QMessageLogContext(const char* fileName, int lineNumber, const char* functionName, const char* categoryName)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(file_name: *const ::libc::c_char,
                           line_number: ::libc::c_int,
                           function_name: *const ::libc::c_char,
                           category_name: *const ::libc::c_char)
                           -> ::message_log_context::MessageLogContext {
    {
      let mut object: ::message_log_context::MessageLogContext =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QMessageLogContext_constructor_fileName_lineNumber_functionName_categoryName(file_name,
                                                                                                    line_number,
                                                                                                    function_name,
                                                                                                    category_name,
                                                                                                    &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageLogContext::set_category(const char* value)```</span>
  ///
  ///
  pub unsafe fn set_category(&mut self, value: *const ::libc::c_char) {
    ::ffi::qt_core_c_QMessageLogContext_set_category(self as *mut ::message_log_context::MessageLogContext, value)
  }

  /// C++ method: <span style='color: green;'>```void QMessageLogContext::set_file(const char* value)```</span>
  ///
  ///
  pub unsafe fn set_file(&mut self, value: *const ::libc::c_char) {
    ::ffi::qt_core_c_QMessageLogContext_set_file(self as *mut ::message_log_context::MessageLogContext, value)
  }

  /// C++ method: <span style='color: green;'>```void QMessageLogContext::set_function(const char* value)```</span>
  ///
  ///
  pub unsafe fn set_function(&mut self, value: *const ::libc::c_char) {
    ::ffi::qt_core_c_QMessageLogContext_set_function(self as *mut ::message_log_context::MessageLogContext, value)
  }

  /// C++ method: <span style='color: green;'>```void QMessageLogContext::set_line(int value)```</span>
  ///
  ///
  pub fn set_line(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QMessageLogContext_set_line(self as *mut ::message_log_context::MessageLogContext, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageLogContext::set_version(int value)```</span>
  ///
  ///
  pub fn set_version(&mut self, value: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QMessageLogContext_set_version(self as *mut ::message_log_context::MessageLogContext, value)
    }
  }

  /// C++ method: <span style='color: green;'>```int QMessageLogContext::version() const```</span>
  ///
  ///
  pub fn version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMessageLogContext_version(self as *const ::message_log_context::MessageLogContext) }
  }
}

impl Drop for ::message_log_context::MessageLogContext {
  /// C++ method: <span style='color: green;'>```[destructor] void QMessageLogContext::~QMessageLogContext()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMessageLogContext_destructor(self as *mut ::message_log_context::MessageLogContext) }
  }
}

/// C++ type: <span style='color: green;'>```QtMsgType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MsgType {
  /// C++ enum variant: <span style='color: green;'>```QtDebugMsg = 0```</span>
  Debug = 0,
  /// C++ enum variant: <span style='color: green;'>```QtWarningMsg = 1```</span>
  Warning = 1,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```QtCriticalMsg = 2```</span>
  /// - <span style='color: green;'>```QtSystemMsg = 2```</span>
  ///
  Critical = 2,
  /// C++ enum variant: <span style='color: green;'>```QtFatalMsg = 3```</span>
  Fatal = 3,
  /// C++ enum variant: <span style='color: green;'>```QtInfoMsg = 4```</span>
  Info = 4,
}

/// C++ method: <span style='color: green;'>```QString qFormatLogMessage(QtMsgType type, const QMessageLogContext& context, const QString& buf)```</span>
///
///
pub fn format_log_message(type_: ::message_log_context::MsgType,
                          context: &::message_log_context::MessageLogContext,
                          buf: &::string::String)
                          -> ::string::String {
  {
    let mut object: ::string::String = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_core_c_QMessageLogContext_G_qFormatLogMessage_to_output(type_, context as *const ::message_log_context::MessageLogContext, buf as *const ::string::String, &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```void qSetMessagePattern(const QString& messagePattern)```</span>
///
///
pub fn set_message_pattern(message_pattern: &::string::String) {
  unsafe { ::ffi::qt_core_c_QMessageLogContext_G_qSetMessagePattern(message_pattern as *const ::string::String) }
}
