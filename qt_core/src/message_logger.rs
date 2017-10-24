/// C++ type: <span style='color: green;'>```QMessageLogger```</span>
#[repr(C)]
pub struct MessageLogger([u8; ::type_sizes::QT_CORE_MESSAGE_LOGGER_MESSAGE_LOGGER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MessageLogger {
  unsafe fn new_uninitialized() -> MessageLogger {
    MessageLogger(::std::mem::uninitialized())
  }
}

impl MessageLogger {
  /// C++ method: <span style='color: green;'>```QMessageLogger::critical```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn critical(&self, ()) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug QMessageLogger::critical() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn critical(&self, &::logging_category::LoggingCategory) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug QMessageLogger::critical(const QLoggingCategory& cat) const```</span>
  ///
  ///
  pub fn critical<'largs, Args>(&'largs self, args: Args) -> ::debug::Debug
    where Args: overloading::MessageLoggerCriticalArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMessageLogger::debug```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn debug(&self, ()) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug QMessageLogger::debug() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn debug(&self, &::logging_category::LoggingCategory) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug QMessageLogger::debug(const QLoggingCategory& cat) const```</span>
  ///
  ///
  pub fn debug<'largs, Args>(&'largs self, args: Args) -> ::debug::Debug
    where Args: overloading::MessageLoggerDebugArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMessageLogger::info```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn info(&self, ()) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug QMessageLogger::info() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn info(&self, &::logging_category::LoggingCategory) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug QMessageLogger::info(const QLoggingCategory& cat) const```</span>
  ///
  ///
  pub fn info<'largs, Args>(&'largs self, args: Args) -> ::debug::Debug
    where Args: overloading::MessageLoggerInfoArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QMessageLogger::QMessageLogger()```</span>
  ///
  ///
  pub fn new() -> ::message_logger::MessageLogger {
    {
      let mut object: ::message_logger::MessageLogger =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMessageLogger_constructor_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMessageLogger::QMessageLogger```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_char, ::libc::c_int, *const ::libc::c_char)) -> ::message_logger::MessageLogger```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMessageLogger::QMessageLogger(const char* file, int line, const char* function)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_char, ::libc::c_int, *const ::libc::c_char, *const ::libc::c_char)) -> ::message_logger::MessageLogger```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMessageLogger::QMessageLogger(const char* file, int line, const char* function, const char* category)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::message_logger::MessageLogger
    where Args: overloading::MessageLoggerNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMessageLogger::warning```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn warning(&self, ()) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug QMessageLogger::warning() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn warning(&self, &::logging_category::LoggingCategory) -> ::debug::Debug```<br>
  /// C++ method: <span style='color: green;'>```QDebug QMessageLogger::warning(const QLoggingCategory& cat) const```</span>
  ///
  ///
  pub fn warning<'largs, Args>(&'largs self, args: Args) -> ::debug::Debug
    where Args: overloading::MessageLoggerWarningArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::message_logger::MessageLogger {
  /// C++ method: <span style='color: green;'>```[destructor] void QMessageLogger::~QMessageLogger()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMessageLogger_destructor(self as *mut ::message_logger::MessageLogger) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MessageLogger::critical](../struct.MessageLogger.html#method.critical) method.
  pub trait MessageLoggerCriticalArgs<'largs> {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug;
  }
  impl<'largs> MessageLoggerCriticalArgs<'largs> for &'largs ::logging_category::LoggingCategory {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug {
      let cat = self;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMessageLogger_critical_to_output_cat(original_self as *const ::message_logger::MessageLogger, cat as *const ::logging_category::LoggingCategory, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MessageLoggerCriticalArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug {

      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMessageLogger_critical_to_output_no_args(original_self as *const ::message_logger::MessageLogger, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MessageLogger::debug](../struct.MessageLogger.html#method.debug) method.
  pub trait MessageLoggerDebugArgs<'largs> {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug;
  }
  impl<'largs> MessageLoggerDebugArgs<'largs> for &'largs ::logging_category::LoggingCategory {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug {
      let cat = self;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMessageLogger_debug_to_output_cat(original_self as *const ::message_logger::MessageLogger, cat as *const ::logging_category::LoggingCategory, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MessageLoggerDebugArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug {

      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMessageLogger_debug_to_output_no_args(original_self as *const ::message_logger::MessageLogger, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MessageLogger::info](../struct.MessageLogger.html#method.info) method.
  pub trait MessageLoggerInfoArgs<'largs> {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug;
  }
  impl<'largs> MessageLoggerInfoArgs<'largs> for &'largs ::logging_category::LoggingCategory {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug {
      let cat = self;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMessageLogger_info_to_output_cat(original_self as *const ::message_logger::MessageLogger,
                                                             cat as *const ::logging_category::LoggingCategory,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MessageLoggerInfoArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug {

      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMessageLogger_info_to_output_no_args(original_self as *const ::message_logger::MessageLogger, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MessageLogger::new_unsafe](../struct.MessageLogger.html#method.new_unsafe) method.
  pub trait MessageLoggerNewUnsafeArgs {
    unsafe fn exec(self) -> ::message_logger::MessageLogger;
  }
  impl MessageLoggerNewUnsafeArgs for (*const ::libc::c_char, ::libc::c_int, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::message_logger::MessageLogger {
      let file = self.0;
      let line = self.1;
      let function = self.2;
      {
        let mut object: ::message_logger::MessageLogger =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QMessageLogger_constructor_file_line_function(file, line, function, &mut object);
        object
      }
    }
  }
  impl MessageLoggerNewUnsafeArgs
    for (*const ::libc::c_char, ::libc::c_int, *const ::libc::c_char, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::message_logger::MessageLogger {
      let file = self.0;
      let line = self.1;
      let function = self.2;
      let category = self.3;
      {
        let mut object: ::message_logger::MessageLogger =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QMessageLogger_constructor_file_line_function_category(file,
                                                                                line,
                                                                                function,
                                                                                category,
                                                                                &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MessageLogger::warning](../struct.MessageLogger.html#method.warning) method.
  pub trait MessageLoggerWarningArgs<'largs> {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug;
  }
  impl<'largs> MessageLoggerWarningArgs<'largs> for &'largs ::logging_category::LoggingCategory {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug {
      let cat = self;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMessageLogger_warning_to_output_cat(original_self as *const ::message_logger::MessageLogger, cat as *const ::logging_category::LoggingCategory, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MessageLoggerWarningArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::message_logger::MessageLogger) -> ::debug::Debug {

      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMessageLogger_warning_to_output_no_args(original_self as *const ::message_logger::MessageLogger, &mut object);
        }
        object
      }
    }
  }
}
