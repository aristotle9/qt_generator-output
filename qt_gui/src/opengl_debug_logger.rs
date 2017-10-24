/// C++ type: <span style='color: green;'>```QOpenGLDebugLogger::LoggingMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum LoggingMode {
  /// C++ enum variant: <span style='color: green;'>```AsynchronousLogging = 0```</span>
  Asynchronous = 0,
  /// C++ enum variant: <span style='color: green;'>```SynchronousLogging = 1```</span>
  Synchronous = 1,
}

/// C++ type: <span style='color: green;'>```QOpenGLDebugLogger```</span>
#[repr(C)]
pub struct OpenGLDebugLogger(u8);

impl OpenGLDebugLogger {
  /// C++ method: <span style='color: green;'>```bool QOpenGLDebugLogger::initialize()```</span>
  ///
  ///
  pub fn initialize(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_initialize(self as *mut ::opengl_debug_logger::OpenGLDebugLogger) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLDebugLogger::isLogging() const```</span>
  ///
  ///
  pub fn is_logging(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_isLogging(self as *const ::opengl_debug_logger::OpenGLDebugLogger) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void QOpenGLDebugLogger::logMessage(const QOpenGLDebugMessage& debugMessage)```</span>
  ///
  ///
  pub fn log_message(&mut self, debug_message: &::opengl_debug_message::OpenGLDebugMessage) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLDebugLogger_logMessage(self as *mut ::opengl_debug_logger::OpenGLDebugLogger, debug_message as *const ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage> QOpenGLDebugLogger::loggedMessages() const```</span>
  ///
  ///
  pub fn logged_messages(&self) -> ::list::ListOpenglDebugMessage {
    {
      let mut object: ::list::ListOpenglDebugMessage =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLDebugLogger_loggedMessages_to_output(self as *const ::opengl_debug_logger::OpenGLDebugLogger, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugLogger::LoggingMode QOpenGLDebugLogger::loggingMode() const```</span>
  ///
  ///
  pub fn logging_mode(&self) -> ::opengl_debug_logger::LoggingMode {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_loggingMode(self as *const ::opengl_debug_logger::OpenGLDebugLogger) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QOpenGLDebugLogger::maximumMessageLength() const```</span>
  ///
  ///
  pub fn maximum_message_length(&self) -> i64 {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLDebugLogger_maximumMessageLength(self as *const ::opengl_debug_logger::OpenGLDebugLogger)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QOpenGLDebugLogger::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_metaObject(self as *const ::opengl_debug_logger::OpenGLDebugLogger) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLDebugLogger::QOpenGLDebugLogger()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::opengl_debug_logger::OpenGLDebugLogger> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLDebugLogger::QOpenGLDebugLogger(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::opengl_debug_logger::OpenGLDebugLogger> {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLDebugLogger_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLDebugLogger::popGroup()```</span>
  ///
  ///
  pub fn pop_group(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_popGroup(self as *mut ::opengl_debug_logger::OpenGLDebugLogger) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugLogger::pushGroup```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn push_group(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLDebugLogger::pushGroup(const QString& name)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn push_group(&mut self, (&::qt_core::string::String, u32)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLDebugLogger::pushGroup(const QString& name, GLuint id = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn push_group(&mut self, (&::qt_core::string::String, u32, &::opengl_debug_message::Source)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLDebugLogger::pushGroup(const QString& name, GLuint id = ?, QOpenGLDebugMessage::Source source = ?)```</span>
  ///
  ///
  pub fn push_group<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLDebugLoggerPushGroupArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual int QOpenGLDebugLogger::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QOpenGLDebugLogger_qt_metacall(self as *mut ::opengl_debug_logger::OpenGLDebugLogger,
                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                   arg2,
                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QOpenGLDebugLogger::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QOpenGLDebugLogger_qt_metacast(self as *mut ::opengl_debug_logger::OpenGLDebugLogger, arg1)
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugLogger::startLogging```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn start_logging(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QOpenGLDebugLogger::startLogging()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn start_logging(&mut self, ::opengl_debug_logger::LoggingMode) -> ()```<br>
  /// C++ method: <span style='color: green;'>```[slot] void QOpenGLDebugLogger::startLogging(QOpenGLDebugLogger::LoggingMode loggingMode = ?)```</span>
  ///
  ///
  pub fn start_logging<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLDebugLoggerStartLoggingArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[slot] void QOpenGLDebugLogger::stopLogging()```</span>
  ///
  ///
  pub fn stop_logging(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_stopLogging(self as *mut ::opengl_debug_logger::OpenGLDebugLogger) }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLDebugLogger::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLDebugLogger_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLDebugLogger::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLDebugLogger_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_debug_logger::OpenGLDebugLogger {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLDebugLogger_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OpenGLDebugLogger`.
  pub struct Signals<'a>(&'a ::opengl_debug_logger::OpenGLDebugLogger);
  /// Represents a built-in Qt signal `QOpenGLDebugLogger::objectNameChanged`.
  ///
  /// An object of this type can be created from `OpenGLDebugLogger` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLDebugLogger` object.
  pub struct ObjectNameChanged<'a>(&'a ::opengl_debug_logger::OpenGLDebugLogger);
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
  /// Represents a built-in Qt signal `QOpenGLDebugLogger::messageLogged`.
  ///
  /// An object of this type can be created from `OpenGLDebugLogger` with `object.signals().message_logged()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLDebugLogger` object.
  pub struct MessageLogged<'a>(&'a ::opengl_debug_logger::OpenGLDebugLogger);
  impl<'a> ::qt_core::connection::Receiver for MessageLogged<'a> {
    type Arguments = (&'static ::opengl_debug_message::OpenGLDebugMessage,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2messageLogged(const QOpenGLDebugMessage&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MessageLogged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QOpenGLDebugLogger::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `QOpenGLDebugLogger::messageLogged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn message_logged(&self) -> MessageLogged {
      MessageLogged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `OpenGLDebugLogger`.
  pub struct Slots<'a>(&'a ::opengl_debug_logger::OpenGLDebugLogger);
  /// Represents a built-in Qt slot `QOpenGLDebugLogger::startLogging`.
  ///
  /// An object of this type can be created from `OpenGLDebugLogger` with `object.slots().start_logging()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLDebugLogger` object.
  pub struct StartLogging<'a>(&'a ::opengl_debug_logger::OpenGLDebugLogger);
  impl<'a> ::qt_core::connection::Receiver for StartLogging<'a> {
    type Arguments = (::opengl_debug_logger::LoggingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1startLogging(QOpenGLDebugLogger::LoggingMode)\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLDebugLogger::logMessage`.
  ///
  /// An object of this type can be created from `OpenGLDebugLogger` with `object.slots().log_message()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLDebugLogger` object.
  pub struct LogMessage<'a>(&'a ::opengl_debug_logger::OpenGLDebugLogger);
  impl<'a> ::qt_core::connection::Receiver for LogMessage<'a> {
    type Arguments = (&'static ::opengl_debug_message::OpenGLDebugMessage,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1logMessage(const QOpenGLDebugMessage&)\0"
    }
  }
  /// Represents a built-in Qt slot `QOpenGLDebugLogger::stopLogging`.
  ///
  /// An object of this type can be created from `OpenGLDebugLogger` with `object.slots().stop_logging()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLDebugLogger` object.
  pub struct StopLogging<'a>(&'a ::opengl_debug_logger::OpenGLDebugLogger);
  impl<'a> ::qt_core::connection::Receiver for StopLogging<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1stopLogging()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QOpenGLDebugLogger::startLogging`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn start_logging(&self) -> StartLogging {
      StartLogging(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLDebugLogger::logMessage`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn log_message(&self) -> LogMessage {
      LogMessage(self.0)
    }
    /// Returns an object representing a built-in Qt slot `QOpenGLDebugLogger::stopLogging`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn stop_logging(&self) -> StopLogging {
      StopLogging(self.0)
    }
  }
  impl ::opengl_debug_logger::OpenGLDebugLogger {
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

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::opengl_debug_message::Severity)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, QOpenGLDebugMessage::Severity severity)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::opengl_debug_message::Source)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, QOpenGLDebugMessage::Source source)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::opengl_debug_message::Type)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, QOpenGLDebugMessage::Type type)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::opengl_debug_message::OpenGLDebugMessage)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug debug, const QOpenGLDebugMessage& message)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> ::qt_core::debug::Debug
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```void swap(QOpenGLDebugMessage& value1, QOpenGLDebugMessage& value2)```</span>
///
///
pub fn swap(value1: &mut ::opengl_debug_message::OpenGLDebugMessage,
            value2: &mut ::opengl_debug_message::OpenGLDebugMessage) {
  unsafe {
    ::ffi::qt_gui_c_QOpenGLDebugLogger_G_swap(value1 as *mut ::opengl_debug_message::OpenGLDebugMessage,
                                              value2 as *mut ::opengl_debug_message::OpenGLDebugMessage)
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::opengl_debug_logger::OpenGLDebugLogger {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_G_static_cast_QObject_ptr(self as *mut ::opengl_debug_logger::OpenGLDebugLogger) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_G_static_cast_QObject_ptr(self as *const ::opengl_debug_logger::OpenGLDebugLogger as *mut ::opengl_debug_logger::OpenGLDebugLogger) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_debug_logger::OpenGLDebugLogger> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_debug_logger::OpenGLDebugLogger {
    let ffi_result =
      ::ffi::qt_gui_c_QOpenGLDebugLogger_G_static_cast_QOpenGLDebugLogger_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_debug_logger::OpenGLDebugLogger {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLDebugLogger_G_static_cast_QOpenGLDebugLogger_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::opengl_debug_logger::OpenGLDebugLogger {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_G_static_cast_QObject_ptr(self as *const ::opengl_debug_logger::OpenGLDebugLogger as *mut ::opengl_debug_logger::OpenGLDebugLogger) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_debug_logger::OpenGLDebugLogger {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_G_static_cast_QObject_ptr(self as *mut ::opengl_debug_logger::OpenGLDebugLogger) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLDebugLogger::push_group](../struct.OpenGLDebugLogger.html#method.push_group) method.
  pub trait OpenGLDebugLoggerPushGroupArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_debug_logger::OpenGLDebugLogger) -> ();
  }
  impl<'largs> OpenGLDebugLoggerPushGroupArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::opengl_debug_logger::OpenGLDebugLogger) -> () {
      let name = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_pushGroup_name(original_self as *mut ::opengl_debug_logger::OpenGLDebugLogger, name as *const ::qt_core::string::String) }
    }
  }
  impl<'largs> OpenGLDebugLoggerPushGroupArgs<'largs> for (&'largs ::qt_core::string::String, u32) {
    fn exec(self, original_self: &'largs mut ::opengl_debug_logger::OpenGLDebugLogger) -> () {
      let name = self.0;
      let id = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_pushGroup_name_id(original_self as *mut ::opengl_debug_logger::OpenGLDebugLogger, name as *const ::qt_core::string::String, id) }
    }
  }
  impl<'largs> OpenGLDebugLoggerPushGroupArgs<'largs>
    for (&'largs ::qt_core::string::String, u32, &'largs ::opengl_debug_message::Source) {
    fn exec(self, original_self: &'largs mut ::opengl_debug_logger::OpenGLDebugLogger) -> () {
      let name = self.0;
      let id = self.1;
      let source = self.2;
      unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_pushGroup_name_id_source(original_self as *mut ::opengl_debug_logger::OpenGLDebugLogger, name as *const ::qt_core::string::String, id, source as *const ::opengl_debug_message::Source) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLDebugLogger::start_logging](../struct.OpenGLDebugLogger.html#method.start_logging) method.
  pub trait OpenGLDebugLoggerStartLoggingArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_debug_logger::OpenGLDebugLogger) -> ();
  }
  impl<'largs> OpenGLDebugLoggerStartLoggingArgs<'largs> for ::opengl_debug_logger::LoggingMode {
    fn exec(self, original_self: &'largs mut ::opengl_debug_logger::OpenGLDebugLogger) -> () {
      let logging_mode = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_startLogging_loggingMode(original_self as *mut ::opengl_debug_logger::OpenGLDebugLogger, logging_mode) }
    }
  }
  impl<'largs> OpenGLDebugLoggerStartLoggingArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::opengl_debug_logger::OpenGLDebugLogger) -> () {

      unsafe { ::ffi::qt_gui_c_QOpenGLDebugLogger_startLogging_no_args(original_self as *mut ::opengl_debug_logger::OpenGLDebugLogger) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    fn exec(self) -> ::qt_core::debug::Debug;
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::opengl_debug_message::OpenGLDebugMessage) {
    fn exec(self) -> ::qt_core::debug::Debug {
      let debug = self.0;
      let message = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_message(debug as *const ::qt_core::debug::Debug, message as *const ::opengl_debug_message::OpenGLDebugMessage, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::opengl_debug_message::Severity) {
    fn exec(self) -> ::qt_core::debug::Debug {
      let debug = self.0;
      let severity = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_severity(debug as *const ::qt_core::debug::Debug, severity as *const ::opengl_debug_message::Severity, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::opengl_debug_message::Source) {
    fn exec(self) -> ::qt_core::debug::Debug {
      let debug = self.0;
      let source = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_source(debug as *const ::qt_core::debug::Debug, source as *const ::opengl_debug_message::Source, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::opengl_debug_message::Type) {
    fn exec(self) -> ::qt_core::debug::Debug {
      let debug = self.0;
      let type_ = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugLogger_G_operator_shl_to_output_debug_type(debug as *const ::qt_core::debug::Debug, type_ as *const ::opengl_debug_message::Type, &mut object);
        }
        object
      }
    }
  }
}
