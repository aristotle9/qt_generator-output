/// C++ type: <span style='color: green;'>```QOpenGLDebugMessage```</span>
#[repr(C)]
pub struct OpenGLDebugMessage([u8; ::type_sizes::QT_GUI_OPENGL_DEBUG_MESSAGE_OPEN_G_L_DEBUG_MESSAGE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for OpenGLDebugMessage {
  unsafe fn new_uninitialized() -> OpenGLDebugMessage {
    OpenGLDebugMessage(::std::mem::uninitialized())
  }
}

impl OpenGLDebugMessage {
  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage::createApplicationMessage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_application_message(&::qt_core::string::String) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```static QOpenGLDebugMessage QOpenGLDebugMessage::createApplicationMessage(const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_application_message((&::qt_core::string::String, u32)) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```static QOpenGLDebugMessage QOpenGLDebugMessage::createApplicationMessage(const QString& text, GLuint id = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn create_application_message((&::qt_core::string::String, u32, ::opengl_debug_message::Severity)) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```static QOpenGLDebugMessage QOpenGLDebugMessage::createApplicationMessage(const QString& text, GLuint id = ?, QOpenGLDebugMessage::Severity severity = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn create_application_message((&::qt_core::string::String, u32, ::opengl_debug_message::Severity, ::opengl_debug_message::Type)) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```static QOpenGLDebugMessage QOpenGLDebugMessage::createApplicationMessage(const QString& text, GLuint id = ?, QOpenGLDebugMessage::Severity severity = ?, QOpenGLDebugMessage::Type type = ?)```</span>
  ///
  ///
  pub fn create_application_message<Args>(args: Args) -> ::opengl_debug_message::OpenGLDebugMessage
    where Args: overloading::OpenGLDebugMessageCreateApplicationMessageArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage::createThirdPartyMessage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_third_party_message(&::qt_core::string::String) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```static QOpenGLDebugMessage QOpenGLDebugMessage::createThirdPartyMessage(const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_third_party_message((&::qt_core::string::String, u32)) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```static QOpenGLDebugMessage QOpenGLDebugMessage::createThirdPartyMessage(const QString& text, GLuint id = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn create_third_party_message((&::qt_core::string::String, u32, ::opengl_debug_message::Severity)) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```static QOpenGLDebugMessage QOpenGLDebugMessage::createThirdPartyMessage(const QString& text, GLuint id = ?, QOpenGLDebugMessage::Severity severity = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn create_third_party_message((&::qt_core::string::String, u32, ::opengl_debug_message::Severity, ::opengl_debug_message::Type)) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```static QOpenGLDebugMessage QOpenGLDebugMessage::createThirdPartyMessage(const QString& text, GLuint id = ?, QOpenGLDebugMessage::Severity severity = ?, QOpenGLDebugMessage::Type type = ?)```</span>
  ///
  ///
  pub fn create_third_party_message<Args>(args: Args) -> ::opengl_debug_message::OpenGLDebugMessage
    where Args: overloading::OpenGLDebugMessageCreateThirdPartyMessageArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```GLuint QOpenGLDebugMessage::id() const```</span>
  ///
  ///
  pub fn id(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugMessage_id(self as *const ::opengl_debug_message::OpenGLDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```QString QOpenGLDebugMessage::message() const```</span>
  ///
  ///
  pub fn message(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLDebugMessage_message_to_output(self as *const ::opengl_debug_message::OpenGLDebugMessage, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage::QOpenGLDebugMessage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLDebugMessage::QOpenGLDebugMessage()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::opengl_debug_message::OpenGLDebugMessage) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLDebugMessage::QOpenGLDebugMessage(const QOpenGLDebugMessage& debugMessage)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::opengl_debug_message::OpenGLDebugMessage
    where Args: overloading::OpenGLDebugMessageNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage& QOpenGLDebugMessage::operator=(const QOpenGLDebugMessage& debugMessage)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             debug_message: &'l1 ::opengl_debug_message::OpenGLDebugMessage)
                             -> &'l0 mut ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLDebugMessage_operator_assign(self as *mut ::opengl_debug_message::OpenGLDebugMessage, debug_message as *const ::opengl_debug_message::OpenGLDebugMessage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLDebugMessage::operator==(const QOpenGLDebugMessage& debugMessage) const```</span>
  ///
  ///
  pub fn op_eq(&self, debug_message: &::opengl_debug_message::OpenGLDebugMessage) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugMessage_operator_eq(self as *const ::opengl_debug_message::OpenGLDebugMessage, debug_message as *const ::opengl_debug_message::OpenGLDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLDebugMessage::operator!=(const QOpenGLDebugMessage& debugMessage) const```</span>
  ///
  ///
  pub fn op_neq(&self, debug_message: &::opengl_debug_message::OpenGLDebugMessage) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugMessage_operator_neq(self as *const ::opengl_debug_message::OpenGLDebugMessage, debug_message as *const ::opengl_debug_message::OpenGLDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage::Severity QOpenGLDebugMessage::severity() const```</span>
  ///
  ///
  pub fn severity(&self) -> ::opengl_debug_message::Severity {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugMessage_severity(self as *const ::opengl_debug_message::OpenGLDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage::Source QOpenGLDebugMessage::source() const```</span>
  ///
  ///
  pub fn source(&self) -> ::opengl_debug_message::Source {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugMessage_source(self as *const ::opengl_debug_message::OpenGLDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLDebugMessage::swap(QOpenGLDebugMessage& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::opengl_debug_message::OpenGLDebugMessage) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLDebugMessage_swap(self as *mut ::opengl_debug_message::OpenGLDebugMessage,
                                               other as *mut ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage::Type QOpenGLDebugMessage::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::opengl_debug_message::Type {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugMessage_type(self as *const ::opengl_debug_message::OpenGLDebugMessage) }
  }
}

impl Drop for ::opengl_debug_message::OpenGLDebugMessage {
  /// C++ method: <span style='color: green;'>```[destructor] void QOpenGLDebugMessage::~QOpenGLDebugMessage()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLDebugMessage_destructor(self as *mut ::opengl_debug_message::OpenGLDebugMessage) }
  }
}

/// C++ type: <span style='color: green;'>```QOpenGLDebugMessage::Severity```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Severity {
  /// C++ enum variant: <span style='color: green;'>```AnySeverity = -1```</span>
  Any = -1,
  /// C++ enum variant: <span style='color: green;'>```InvalidSeverity = 0```</span>
  Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```HighSeverity = 1```</span>
  High = 1,
  /// C++ enum variant: <span style='color: green;'>```MediumSeverity = 2```</span>
  Medium = 2,
  /// C++ enum variant: <span style='color: green;'>```LowSeverity = 4```</span>
  Low = 4,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```NotificationSeverity = 8```</span>
  /// - <span style='color: green;'>```LastSeverity = 8```</span>
  ///
  Notification = 8,
}

/// C++ type: <span style='color: green;'>```QOpenGLDebugMessage::Source```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Source {
  /// C++ enum variant: <span style='color: green;'>```AnySource = -1```</span>
  Any = -1,
  /// C++ enum variant: <span style='color: green;'>```InvalidSource = 0```</span>
  Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```APISource = 1```</span>
  API = 1,
  /// C++ enum variant: <span style='color: green;'>```WindowSystemSource = 2```</span>
  WindowSystem = 2,
  /// C++ enum variant: <span style='color: green;'>```ShaderCompilerSource = 4```</span>
  ShaderCompiler = 4,
  /// C++ enum variant: <span style='color: green;'>```ThirdPartySource = 8```</span>
  ThirdParty = 8,
  /// C++ enum variant: <span style='color: green;'>```ApplicationSource = 16```</span>
  Application = 16,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```OtherSource = 32```</span>
  /// - <span style='color: green;'>```LastSource = 32```</span>
  ///
  Other = 32,
}

/// C++ type: <span style='color: green;'>```QOpenGLDebugMessage::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```AnyType = -1```</span>
  Any = -1,
  /// C++ enum variant: <span style='color: green;'>```InvalidType = 0```</span>
  Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```ErrorType = 1```</span>
  Error = 1,
  /// C++ enum variant: <span style='color: green;'>```DeprecatedBehaviorType = 2```</span>
  DeprecatedBehavior = 2,
  /// C++ enum variant: <span style='color: green;'>```UndefinedBehaviorType = 4```</span>
  UndefinedBehavior = 4,
  /// C++ enum variant: <span style='color: green;'>```PortabilityType = 8```</span>
  Portability = 8,
  /// C++ enum variant: <span style='color: green;'>```PerformanceType = 16```</span>
  Performance = 16,
  /// C++ enum variant: <span style='color: green;'>```OtherType = 32```</span>
  Other = 32,
  /// C++ enum variant: <span style='color: green;'>```MarkerType = 64```</span>
  Marker = 64,
  /// C++ enum variant: <span style='color: green;'>```GroupPushType = 128```</span>
  GroupPush = 128,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```GroupPopType = 256```</span>
  /// - <span style='color: green;'>```LastType = 256```</span>
  ///
  GroupPop = 256,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLDebugMessage::create_application_message](../struct.OpenGLDebugMessage.html#method.create_application_message) method.
  pub trait OpenGLDebugMessageCreateApplicationMessageArgs {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage;
  }
  impl<'a> OpenGLDebugMessageCreateApplicationMessageArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage {
      let text = self;
      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text(text as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpenGLDebugMessageCreateApplicationMessageArgs for (&'a ::qt_core::string::String, u32) {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage {
      let text = self.0;
      let id = self.1;
      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text_id(text as *const ::qt_core::string::String, id, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpenGLDebugMessageCreateApplicationMessageArgs
    for (&'a ::qt_core::string::String, u32, ::opengl_debug_message::Severity) {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage {
      let text = self.0;
      let id = self.1;
      let severity = self.2;
      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text_id_severity(text as *const ::qt_core::string::String, id, severity, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpenGLDebugMessageCreateApplicationMessageArgs
    for (&'a ::qt_core::string::String, u32, ::opengl_debug_message::Severity, ::opengl_debug_message::Type) {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage {
      let text = self.0;
      let id = self.1;
      let severity = self.2;
      let type_ = self.3;
      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugMessage_createApplicationMessage_to_output_text_id_severity_type(text as *const ::qt_core::string::String, id, severity, type_, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLDebugMessage::create_third_party_message](../struct.OpenGLDebugMessage.html#method.create_third_party_message) method.
  pub trait OpenGLDebugMessageCreateThirdPartyMessageArgs {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage;
  }
  impl<'a> OpenGLDebugMessageCreateThirdPartyMessageArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage {
      let text = self;
      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text(text as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpenGLDebugMessageCreateThirdPartyMessageArgs for (&'a ::qt_core::string::String, u32) {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage {
      let text = self.0;
      let id = self.1;
      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text_id(text as *const ::qt_core::string::String, id, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpenGLDebugMessageCreateThirdPartyMessageArgs
    for (&'a ::qt_core::string::String, u32, ::opengl_debug_message::Severity) {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage {
      let text = self.0;
      let id = self.1;
      let severity = self.2;
      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text_id_severity(text as *const ::qt_core::string::String, id, severity, &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpenGLDebugMessageCreateThirdPartyMessageArgs
    for (&'a ::qt_core::string::String, u32, ::opengl_debug_message::Severity, ::opengl_debug_message::Type) {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage {
      let text = self.0;
      let id = self.1;
      let severity = self.2;
      let type_ = self.3;
      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugMessage_createThirdPartyMessage_to_output_text_id_severity_type(text as *const ::qt_core::string::String, id, severity, type_, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLDebugMessage::new](../struct.OpenGLDebugMessage.html#method.new) method.
  pub trait OpenGLDebugMessageNewArgs {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage;
  }
  impl<'a> OpenGLDebugMessageNewArgs for &'a ::opengl_debug_message::OpenGLDebugMessage {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage {
      let debug_message = self;
      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugMessage_constructor_debugMessage(debug_message as *const ::opengl_debug_message::OpenGLDebugMessage, &mut object);
        }
        object
      }
    }
  }
  impl OpenGLDebugMessageNewArgs for () {
    fn exec(self) -> ::opengl_debug_message::OpenGLDebugMessage {

      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QOpenGLDebugMessage_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
}
