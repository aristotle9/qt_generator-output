/// C++ type: <span style='color: green;'>```QOpenGLShader```</span>
#[repr(C)]
pub struct OpenGLShader(u8);

impl OpenGLShader {
  /// C++ method: <span style='color: green;'>```QOpenGLShader::compileSourceCode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn compile_source_code(&mut self, &::qt_core::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QOpenGLShader::compileSourceCode(const QByteArray& source)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn compile_source_code(&mut self, &::qt_core::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QOpenGLShader::compileSourceCode(const QString& source)```</span>
  ///
  ///
  pub fn compile_source_code<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::OpenGLShaderCompileSourceCodeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QOpenGLShader::compileSourceCode(const char* source)```</span>
  ///
  ///
  pub unsafe fn compile_source_code_unsafe(&mut self, source: *const ::libc::c_char) -> bool {
    ::ffi::qt_gui_c_QOpenGLShader_compileSourceCode_char(self as *mut ::opengl_shader::OpenGLShader, source)
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLShader::compileSourceFile(const QString& fileName)```</span>
  ///
  ///
  pub fn compile_source_file(&mut self, file_name: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLShader_compileSourceFile(self as *mut ::opengl_shader::OpenGLShader,
                                                      file_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```static bool QOpenGLShader::hasOpenGLShaders(QFlags<QOpenGLShader::ShaderTypeBit> type)```</span>
  ///
  ///
  pub fn has_opengl_shaders(type_: ::qt_core::flags::Flags<::opengl_shader::ShaderTypeBit>) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLShader_hasOpenGLShaders_type(type_.to_int() as ::libc::c_uint) }
  }

  /// C++ method: <span style='color: green;'>```static bool QOpenGLShader::hasOpenGLShaders(QFlags<QOpenGLShader::ShaderTypeBit> type, QOpenGLContext* context = ?)```</span>
  ///
  ///
  pub unsafe fn has_opengl_shaders_unsafe(type_: ::qt_core::flags::Flags<::opengl_shader::ShaderTypeBit>,
                                          context: *mut ::opengl_context::OpenGLContext)
                                          -> bool {
    ::ffi::qt_gui_c_QOpenGLShader_hasOpenGLShaders_type_context(type_.to_int() as ::libc::c_uint, context)
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLShader::isCompiled() const```</span>
  ///
  ///
  pub fn is_compiled(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLShader_isCompiled(self as *const ::opengl_shader::OpenGLShader) }
  }

  /// C++ method: <span style='color: green;'>```QString QOpenGLShader::log() const```</span>
  ///
  ///
  pub fn log(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLShader_log_to_output(self as *const ::opengl_shader::OpenGLShader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QOpenGLShader::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_gui_c_QOpenGLShader_metaObject(self as *const ::opengl_shader::OpenGLShader) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QOpenGLShader::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QOpenGLShader_qt_metacall(self as *mut ::opengl_shader::OpenGLShader,
                                              arg1 as *const ::qt_core::meta_object::Call,
                                              arg2,
                                              arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QOpenGLShader::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QOpenGLShader_qt_metacast(self as *mut ::opengl_shader::OpenGLShader, arg1)
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLShader::shaderId() const```</span>
  ///
  ///
  pub fn shader_id(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLShader_shaderId(self as *const ::opengl_shader::OpenGLShader) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QOpenGLShader::sourceCode() const```</span>
  ///
  ///
  pub fn source_code(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLShader_sourceCode_to_output(self as *const ::opengl_shader::OpenGLShader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLShader::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLShader_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLShader::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLShader_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_shader::OpenGLShader {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLShader_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OpenGLShader`.
  pub struct Signals<'a>(&'a ::opengl_shader::OpenGLShader);
  /// Represents a built-in Qt signal `QOpenGLShader::objectNameChanged`.
  ///
  /// An object of this type can be created from `OpenGLShader` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLShader` object.
  pub struct ObjectNameChanged<'a>(&'a ::opengl_shader::OpenGLShader);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `QOpenGLShader::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  impl ::opengl_shader::OpenGLShader {
    /// Provides access to built-in Qt signals of this type
    pub fn signals(&self) -> Signals {
      Signals(self)
    }
  }

}

/// C++ type: <span style='color: green;'>```QOpenGLShader::ShaderTypeBit```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ShaderTypeBit {
  /// C++ enum variant: <span style='color: green;'>```Vertex = 1```</span>
  Vertex = 1,
  /// C++ enum variant: <span style='color: green;'>```Fragment = 2```</span>
  Fragment = 2,
  /// C++ enum variant: <span style='color: green;'>```Geometry = 4```</span>
  Geometry = 4,
  /// C++ enum variant: <span style='color: green;'>```TessellationControl = 8```</span>
  TessellationControl = 8,
  /// C++ enum variant: <span style='color: green;'>```TessellationEvaluation = 16```</span>
  TessellationEvaluation = 16,
  /// C++ enum variant: <span style='color: green;'>```Compute = 32```</span>
  Compute = 32,
}

impl ::qt_core::flags::FlaggableEnum for ShaderTypeBit {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "ShaderTypeBit"
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::opengl_shader::OpenGLShader {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QOpenGLShader_G_static_cast_QObject_ptr(self as *mut ::opengl_shader::OpenGLShader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLShader_G_static_cast_QObject_ptr(self as *const ::opengl_shader::OpenGLShader as *mut ::opengl_shader::OpenGLShader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_shader::OpenGLShader> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      ::ffi::qt_gui_c_QOpenGLShader_G_static_cast_QOpenGLShader_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_shader::OpenGLShader {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLShader_G_static_cast_QOpenGLShader_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::opengl_shader::OpenGLShader {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLShader_G_static_cast_QObject_ptr(self as *const ::opengl_shader::OpenGLShader as *mut ::opengl_shader::OpenGLShader) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_shader::OpenGLShader {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QOpenGLShader_G_static_cast_QObject_ptr(self as *mut ::opengl_shader::OpenGLShader) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLShader::compile_source_code](../struct.OpenGLShader.html#method.compile_source_code) method.
  pub trait OpenGLShaderCompileSourceCodeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_shader::OpenGLShader) -> bool;
  }
  impl<'largs> OpenGLShaderCompileSourceCodeArgs<'largs> for &'largs ::qt_core::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::opengl_shader::OpenGLShader) -> bool {
      let source = self;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLShader_compileSourceCode_QByteArray(original_self as *mut ::opengl_shader::OpenGLShader, source as *const ::qt_core::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> OpenGLShaderCompileSourceCodeArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::opengl_shader::OpenGLShader) -> bool {
      let source = self;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLShader_compileSourceCode_QString(original_self as *mut ::opengl_shader::OpenGLShader,
                                                                source as *const ::qt_core::string::String)
      }
    }
  }
}
