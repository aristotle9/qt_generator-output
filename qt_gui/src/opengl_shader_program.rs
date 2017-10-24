/// C++ type: <span style='color: green;'>```QOpenGLShaderProgram```</span>
#[repr(C)]
pub struct OpenGLShaderProgram(u8);

impl OpenGLShaderProgram {
  /// C++ method: <span style='color: green;'>```bool QOpenGLShaderProgram::addShader(QOpenGLShader* shader)```</span>
  ///
  ///
  pub unsafe fn add_shader(&mut self, shader: *mut ::opengl_shader::OpenGLShader) -> bool {
    ::ffi::qt_gui_c_QOpenGLShaderProgram_addShader(self as *mut ::opengl_shader_program::OpenGLShaderProgram,
                                                   shader)
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShaderProgram::attributeLocation```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn attribute_location(&self, &::qt_core::byte_array::ByteArray) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QOpenGLShaderProgram::attributeLocation(const QByteArray& name) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn attribute_location(&self, &::qt_core::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QOpenGLShaderProgram::attributeLocation(const QString& name) const```</span>
  ///
  ///
  pub fn attribute_location<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::OpenGLShaderProgramAttributeLocationArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QOpenGLShaderProgram::attributeLocation(const char* name) const```</span>
  ///
  ///
  pub unsafe fn attribute_location_unsafe(&self, name: *const ::libc::c_char) -> ::libc::c_int {
    ::ffi::qt_gui_c_QOpenGLShaderProgram_attributeLocation_char(self as *const ::opengl_shader_program::OpenGLShaderProgram, name)
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLShaderProgram::bind()```</span>
  ///
  ///
  pub fn bind(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_bind(self as *mut ::opengl_shader_program::OpenGLShaderProgram) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShaderProgram::bindAttributeLocation```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn bind_attribute_location(&mut self, (&::qt_core::byte_array::ByteArray, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::bindAttributeLocation(const QByteArray& name, int location)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn bind_attribute_location(&mut self, (&::qt_core::string::String, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::bindAttributeLocation(const QString& name, int location)```</span>
  ///
  ///
  pub fn bind_attribute_location<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLShaderProgramBindAttributeLocationArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::bindAttributeLocation(const char* name, int location)```</span>
  ///
  ///
  pub unsafe fn bind_attribute_location_unsafe(&mut self, name: *const ::libc::c_char, location: ::libc::c_int) {
    ::ffi::qt_gui_c_QOpenGLShaderProgram_bindAttributeLocation_char_int(self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, location)
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLShaderProgram::create()```</span>
  ///
  ///
  pub fn create(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_create(self as *mut ::opengl_shader_program::OpenGLShaderProgram) }
  }

  /// C++ method: <span style='color: green;'>```QVector<float> QOpenGLShaderProgram::defaultInnerTessellationLevels() const```</span>
  ///
  ///
  pub fn default_inner_tessellation_levels(&self) -> ::vector::VectorCFloat {
    {
      let mut object: ::vector::VectorCFloat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLShaderProgram_defaultInnerTessellationLevels_to_output(self as *const ::opengl_shader_program::OpenGLShaderProgram, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<float> QOpenGLShaderProgram::defaultOuterTessellationLevels() const```</span>
  ///
  ///
  pub fn default_outer_tessellation_levels(&self) -> ::vector::VectorCFloat {
    {
      let mut object: ::vector::VectorCFloat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLShaderProgram_defaultOuterTessellationLevels_to_output(self as *const ::opengl_shader_program::OpenGLShaderProgram, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::disableAttributeArray(int location)```</span>
  ///
  ///
  pub fn disable_attribute_array(&mut self, location: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_disableAttributeArray_location(self as *mut ::opengl_shader_program::OpenGLShaderProgram, location) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::disableAttributeArray(const char* name)```</span>
  ///
  ///
  pub unsafe fn disable_attribute_array_unsafe(&mut self, name: *const ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLShaderProgram_disableAttributeArray_name(self as *mut ::opengl_shader_program::OpenGLShaderProgram, name)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::enableAttributeArray(int location)```</span>
  ///
  ///
  pub fn enable_attribute_array(&mut self, location: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_enableAttributeArray_location(self as *mut ::opengl_shader_program::OpenGLShaderProgram, location) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::enableAttributeArray(const char* name)```</span>
  ///
  ///
  pub unsafe fn enable_attribute_array_unsafe(&mut self, name: *const ::libc::c_char) {
    ::ffi::qt_gui_c_QOpenGLShaderProgram_enableAttributeArray_name(self as *mut ::opengl_shader_program::OpenGLShaderProgram, name)
  }

  /// C++ method: <span style='color: green;'>```static bool QOpenGLShaderProgram::hasOpenGLShaderPrograms()```</span>
  ///
  ///
  pub fn has_opengl_shader_programs() -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_hasOpenGLShaderPrograms_no_args() }
  }

  /// C++ method: <span style='color: green;'>```static bool QOpenGLShaderProgram::hasOpenGLShaderPrograms(QOpenGLContext* context = ?)```</span>
  ///
  ///
  pub unsafe fn has_opengl_shader_programs_unsafe(context: *mut ::opengl_context::OpenGLContext) -> bool {
    ::ffi::qt_gui_c_QOpenGLShaderProgram_hasOpenGLShaderPrograms_context(context)
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLShaderProgram::isLinked() const```</span>
  ///
  ///
  pub fn is_linked(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLShaderProgram_isLinked(self as *const ::opengl_shader_program::OpenGLShaderProgram)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QOpenGLShaderProgram::link()```</span>
  ///
  ///
  pub fn link(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_link(self as *mut ::opengl_shader_program::OpenGLShaderProgram) }
  }

  /// C++ method: <span style='color: green;'>```QString QOpenGLShaderProgram::log() const```</span>
  ///
  ///
  pub fn log(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLShaderProgram_log_to_output(self as *const ::opengl_shader_program::OpenGLShaderProgram, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLShaderProgram::maxGeometryOutputVertices() const```</span>
  ///
  ///
  pub fn max_geometry_output_vertices(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_maxGeometryOutputVertices(self as *const ::opengl_shader_program::OpenGLShaderProgram) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* QOpenGLShaderProgram::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLShaderProgram_metaObject(self as *const ::opengl_shader_program::OpenGLShaderProgram)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLShaderProgram::QOpenGLShaderProgram()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::opengl_shader_program::OpenGLShaderProgram> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLShaderProgram::QOpenGLShaderProgram(QObject* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_core::object::Object)
                           -> ::cpp_utils::CppBox<::opengl_shader_program::OpenGLShaderProgram> {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLShaderProgram_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLShaderProgram::patchVertexCount() const```</span>
  ///
  ///
  pub fn patch_vertex_count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLShaderProgram_patchVertexCount(self as *const ::opengl_shader_program::OpenGLShaderProgram)
    }
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLShaderProgram::programId() const```</span>
  ///
  ///
  pub fn program_id(&self) -> u32 {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLShaderProgram_programId(self as *const ::opengl_shader_program::OpenGLShaderProgram)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QOpenGLShaderProgram::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_gui_c_QOpenGLShaderProgram_qt_metacall(self as *mut ::opengl_shader_program::OpenGLShaderProgram,
                                                     arg1 as *const ::qt_core::meta_object::Call,
                                                     arg2,
                                                     arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* QOpenGLShaderProgram::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_gui_c_QOpenGLShaderProgram_qt_metacast(self as *mut ::opengl_shader_program::OpenGLShaderProgram,
                                                     arg1)
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::release()```</span>
  ///
  ///
  pub fn release(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_release(self as *mut ::opengl_shader_program::OpenGLShaderProgram) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::removeAllShaders()```</span>
  ///
  ///
  pub fn remove_all_shaders(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLShaderProgram_removeAllShaders(self as *mut ::opengl_shader_program::OpenGLShaderProgram)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::removeShader(QOpenGLShader* shader)```</span>
  ///
  ///
  pub unsafe fn remove_shader(&mut self, shader: *mut ::opengl_shader::OpenGLShader) {
    ::ffi::qt_gui_c_QOpenGLShaderProgram_removeShader(self as *mut ::opengl_shader_program::OpenGLShaderProgram,
                                                      shader)
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShaderProgram::setAttributeArray```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (*const ::libc::c_char, *const ::vector_2d::Vector2D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(const char* name, const QVector2D* values)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (*const ::libc::c_char, *const ::vector_2d::Vector2D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(const char* name, const QVector2D* values, int stride = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (*const ::libc::c_char, *const ::vector_3d::Vector3D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(const char* name, const QVector3D* values)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (*const ::libc::c_char, *const ::vector_3d::Vector3D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(const char* name, const QVector3D* values, int stride = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (*const ::libc::c_char, *const ::vector_4d::Vector4D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(const char* name, const QVector4D* values)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (*const ::libc::c_char, *const ::vector_4d::Vector4D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(const char* name, const QVector4D* values, int stride = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (*const ::libc::c_char, *const ::libc::c_float, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(const char* name, const float* values, int tupleSize)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (*const ::libc::c_char, *const ::libc::c_float, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(const char* name, const float* values, int tupleSize, int stride = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (*const ::libc::c_char, ::libc::c_uint, *const ::libc::c_void, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(const char* name, unsigned int type, const void* values, int tupleSize)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (*const ::libc::c_char, ::libc::c_uint, *const ::libc::c_void, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(const char* name, unsigned int type, const void* values, int tupleSize, int stride = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (::libc::c_int, *const ::vector_2d::Vector2D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(int location, const QVector2D* values)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (::libc::c_int, *const ::vector_2d::Vector2D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(int location, const QVector2D* values, int stride = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (::libc::c_int, *const ::vector_3d::Vector3D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(int location, const QVector3D* values)```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (::libc::c_int, *const ::vector_3d::Vector3D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(int location, const QVector3D* values, int stride = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (::libc::c_int, *const ::vector_4d::Vector4D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(int location, const QVector4D* values)```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (::libc::c_int, *const ::vector_4d::Vector4D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(int location, const QVector4D* values, int stride = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (::libc::c_int, *const ::libc::c_float, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(int location, const float* values, int tupleSize)```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (::libc::c_int, *const ::libc::c_float, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(int location, const float* values, int tupleSize, int stride = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 19
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (::libc::c_int, ::libc::c_uint, *const ::libc::c_void, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(int location, unsigned int type, const void* values, int tupleSize)```</span>
  ///
  ///
  ///
  /// ## Variant 20
  ///
  /// Rust arguments: ```fn set_attribute_array(&mut self, (::libc::c_int, ::libc::c_uint, *const ::libc::c_void, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeArray(int location, unsigned int type, const void* values, int tupleSize, int stride = ?)```</span>
  ///
  ///
  pub unsafe fn set_attribute_array<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLShaderProgramSetAttributeArrayArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLShaderProgram::setAttributeBuffer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_attribute_buffer(&mut self, (::libc::c_int, ::libc::c_uint, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeBuffer(int location, unsigned int type, int offset, int tupleSize)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_attribute_buffer(&mut self, (::libc::c_int, ::libc::c_uint, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeBuffer(int location, unsigned int type, int offset, int tupleSize, int stride = ?)```</span>
  ///
  ///
  pub fn set_attribute_buffer<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLShaderProgramSetAttributeBufferArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLShaderProgram::setAttributeBuffer```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_attribute_buffer_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_uint, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeBuffer(const char* name, unsigned int type, int offset, int tupleSize)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_attribute_buffer_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_uint, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeBuffer(const char* name, unsigned int type, int offset, int tupleSize, int stride = ?)```</span>
  ///
  ///
  pub unsafe fn set_attribute_buffer_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLShaderProgramSetAttributeBufferUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLShaderProgram::setAttributeValue```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_attribute_value(&mut self, (::libc::c_int, &::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(int location, const QColor& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_attribute_value(&mut self, (::libc::c_int, &::vector_2d::Vector2D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(int location, const QVector2D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_attribute_value(&mut self, (::libc::c_int, &::vector_3d::Vector3D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(int location, const QVector3D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_attribute_value(&mut self, (::libc::c_int, &::vector_4d::Vector4D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(int location, const QVector4D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_attribute_value(&mut self, (::libc::c_int, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(int location, float value)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_attribute_value(&mut self, (::libc::c_int, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(int location, float x, float y)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_attribute_value(&mut self, (::libc::c_int, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(int location, float x, float y, float z)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn set_attribute_value(&mut self, (::libc::c_int, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(int location, float x, float y, float z, float w)```</span>
  ///
  ///
  pub fn set_attribute_value<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLShaderProgramSetAttributeValueArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLShaderProgram::setAttributeValue```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_attribute_value_unsafe(&mut self, (*const ::libc::c_char, &::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(const char* name, const QColor& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_attribute_value_unsafe(&mut self, (*const ::libc::c_char, &::vector_2d::Vector2D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(const char* name, const QVector2D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_attribute_value_unsafe(&mut self, (*const ::libc::c_char, &::vector_3d::Vector3D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(const char* name, const QVector3D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_attribute_value_unsafe(&mut self, (*const ::libc::c_char, &::vector_4d::Vector4D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(const char* name, const QVector4D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_attribute_value_unsafe(&mut self, (*const ::libc::c_char, *const ::libc::c_float, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(const char* name, const float* values, int columns, int rows)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_attribute_value_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(const char* name, float value)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_attribute_value_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(const char* name, float x, float y)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn set_attribute_value_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(const char* name, float x, float y, float z)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn set_attribute_value_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(const char* name, float x, float y, float z, float w)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn set_attribute_value_unsafe(&mut self, (::libc::c_int, *const ::libc::c_float, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setAttributeValue(int location, const float* values, int columns, int rows)```</span>
  ///
  ///
  pub unsafe fn set_attribute_value_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setDefaultInnerTessellationLevels(const QVector<float>& levels)```</span>
  ///
  ///
  pub fn set_default_inner_tessellation_levels(&mut self, levels: &::vector::VectorCFloat) {
    unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setDefaultInnerTessellationLevels(self as *mut ::opengl_shader_program::OpenGLShaderProgram, levels as *const ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setDefaultOuterTessellationLevels(const QVector<float>& levels)```</span>
  ///
  ///
  pub fn set_default_outer_tessellation_levels(&mut self, levels: &::vector::VectorCFloat) {
    unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setDefaultOuterTessellationLevels(self as *mut ::opengl_shader_program::OpenGLShaderProgram, levels as *const ::vector::VectorCFloat) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setPatchVertexCount(int count)```</span>
  ///
  ///
  pub fn set_patch_vertex_count(&mut self, count: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setPatchVertexCount(self as *mut ::opengl_shader_program::OpenGLShaderProgram, count) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShaderProgram::setUniformValue```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, i32)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, GLint value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, u32)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, GLuint value)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, &::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, const QColor& color)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, &::matrix_4x4::Matrix4X4)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, const QMatrix4x4& value)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, const QPoint& point)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, &::qt_core::point_f::PointF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, const QPointF& point)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, &::qt_core::size::Size)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, const QSize& size)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, &::qt_core::size_f::SizeF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, const QSizeF& size)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, &::transform::Transform)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, const QTransform& value)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, &::vector_2d::Vector2D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, const QVector2D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, &::vector_3d::Vector3D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, const QVector3D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, &::vector_4d::Vector4D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, const QVector4D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, float value)```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, float x, float y)```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, float x, float y, float z)```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn set_uniform_value(&mut self, (::libc::c_int, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(int location, float x, float y, float z, float w)```</span>
  ///
  ///
  pub fn set_uniform_value<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLShaderProgramSetUniformValueArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLShaderProgram::setUniformValueArray```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (*const ::libc::c_char, *const i32, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(const char* name, const GLint* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (*const ::libc::c_char, *const u32, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(const char* name, const GLuint* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (*const ::libc::c_char, *const ::matrix_4x4::Matrix4X4, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(const char* name, const QMatrix4x4* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (*const ::libc::c_char, *const ::vector_2d::Vector2D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(const char* name, const QVector2D* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (*const ::libc::c_char, *const ::vector_3d::Vector3D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(const char* name, const QVector3D* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (*const ::libc::c_char, *const ::vector_4d::Vector4D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(const char* name, const QVector4D* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (*const ::libc::c_char, *const ::libc::c_float, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(const char* name, const float* values, int count, int tupleSize)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (::libc::c_int, *const i32, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(int location, const GLint* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (::libc::c_int, *const u32, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(int location, const GLuint* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (::libc::c_int, *const ::matrix_4x4::Matrix4X4, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(int location, const QMatrix4x4* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (::libc::c_int, *const ::vector_2d::Vector2D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector2D* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (::libc::c_int, *const ::vector_3d::Vector3D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector3D* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (::libc::c_int, *const ::vector_4d::Vector4D, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(int location, const QVector4D* values, int count)```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn set_uniform_value_array(&mut self, (::libc::c_int, *const ::libc::c_float, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValueArray(int location, const float* values, int count, int tupleSize)```</span>
  ///
  ///
  pub unsafe fn set_uniform_value_array<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLShaderProgram::setUniformValue```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, i32)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, GLint value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, u32)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, GLuint value)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, &::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, const QColor& color)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, &::matrix_4x4::Matrix4X4)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, const QMatrix4x4& value)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, const QPoint& point)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, &::qt_core::point_f::PointF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, const QPointF& point)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, &::qt_core::size::Size)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, const QSize& size)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, &::qt_core::size_f::SizeF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, const QSizeF& size)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, &::transform::Transform)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, const QTransform& value)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, &::vector_2d::Vector2D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, const QVector2D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, &::vector_3d::Vector3D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, const QVector3D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, &::vector_4d::Vector4D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, const QVector4D& value)```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, float value)```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, float x, float y)```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, float x, float y, float z)```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn set_uniform_value_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLShaderProgram::setUniformValue(const char* name, float x, float y, float z, float w)```</span>
  ///
  ///
  pub unsafe fn set_uniform_value_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*> QOpenGLShaderProgram::shaders() const```</span>
  ///
  ///
  pub fn shaders(&self) -> ::list::ListOpenglShaderMutPtr {
    {
      let mut object: ::list::ListOpenglShaderMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLShaderProgram_shaders_to_output(self as *const ::opengl_shader_program::OpenGLShaderProgram, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLShaderProgram::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLShaderProgram_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QOpenGLShaderProgram::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QOpenGLShaderProgram_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShaderProgram::uniformLocation```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn uniform_location(&self, &::qt_core::byte_array::ByteArray) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QOpenGLShaderProgram::uniformLocation(const QByteArray& name) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn uniform_location(&self, &::qt_core::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QOpenGLShaderProgram::uniformLocation(const QString& name) const```</span>
  ///
  ///
  pub fn uniform_location<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::OpenGLShaderProgramUniformLocationArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QOpenGLShaderProgram::uniformLocation(const char* name) const```</span>
  ///
  ///
  pub unsafe fn uniform_location_unsafe(&self, name: *const ::libc::c_char) -> ::libc::c_int {
    ::ffi::qt_gui_c_QOpenGLShaderProgram_uniformLocation_char(self as *const ::opengl_shader_program::OpenGLShaderProgram, name)
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_shader_program::OpenGLShaderProgram {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLShaderProgram_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `OpenGLShaderProgram`.
  pub struct Signals<'a>(&'a ::opengl_shader_program::OpenGLShaderProgram);
  /// Represents a built-in Qt signal `QOpenGLShaderProgram::objectNameChanged`.
  ///
  /// An object of this type can be created from `OpenGLShaderProgram` with `object.signals().object_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLShaderProgram` object.
  pub struct ObjectNameChanged<'a>(&'a ::opengl_shader_program::OpenGLShaderProgram);
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
    /// Returns an object representing a built-in Qt signal `QOpenGLShaderProgram::objectNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn object_name_changed(&self) -> ObjectNameChanged {
      ObjectNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `OpenGLShaderProgram`.
  pub struct Slots<'a>(&'a ::opengl_shader_program::OpenGLShaderProgram);
  /// Represents a built-in Qt slot `QOpenGLShaderProgram::shaderDestroyed`.
  ///
  /// An object of this type can be created from `OpenGLShaderProgram` with `object.slots().shader_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `OpenGLShaderProgram` object.
  pub struct ShaderDestroyed<'a>(&'a ::opengl_shader_program::OpenGLShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for ShaderDestroyed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1shaderDestroyed()\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `QOpenGLShaderProgram::shaderDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shader_destroyed(&self) -> ShaderDestroyed {
      ShaderDestroyed(self.0)
    }
  }
  impl ::opengl_shader_program::OpenGLShaderProgram {
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

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::opengl_shader_program::OpenGLShaderProgram {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_G_static_cast_QObject_ptr(self as *mut ::opengl_shader_program::OpenGLShaderProgram) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_G_static_cast_QObject_ptr(self as *const ::opengl_shader_program::OpenGLShaderProgram as *mut ::opengl_shader_program::OpenGLShaderProgram) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::opengl_shader_program::OpenGLShaderProgram> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::opengl_shader_program::OpenGLShaderProgram {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLShaderProgram_G_static_cast_QOpenGLShaderProgram_ptr(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::opengl_shader_program::OpenGLShaderProgram {
    let ffi_result = ::ffi::qt_gui_c_QOpenGLShaderProgram_G_static_cast_QOpenGLShaderProgram_ptr(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::opengl_shader_program::OpenGLShaderProgram {
  type Target = ::qt_core::object::Object;
  fn deref(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_G_static_cast_QObject_ptr(self as *const ::opengl_shader_program::OpenGLShaderProgram as *mut ::opengl_shader_program::OpenGLShaderProgram) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::opengl_shader_program::OpenGLShaderProgram {
  fn deref_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_G_static_cast_QObject_ptr(self as *mut ::opengl_shader_program::OpenGLShaderProgram) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLShaderProgram::attribute_location](../struct.OpenGLShaderProgram.html#method.attribute_location) method.
  pub trait OpenGLShaderProgramAttributeLocationArgs<'largs> {
    fn exec(self, original_self: &'largs ::opengl_shader_program::OpenGLShaderProgram) -> ::libc::c_int;
  }
  impl<'largs> OpenGLShaderProgramAttributeLocationArgs<'largs> for &'largs ::qt_core::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::opengl_shader_program::OpenGLShaderProgram) -> ::libc::c_int {
      let name = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_attributeLocation_QByteArray(original_self as *const ::opengl_shader_program::OpenGLShaderProgram, name as *const ::qt_core::byte_array::ByteArray) }
    }
  }
  impl<'largs> OpenGLShaderProgramAttributeLocationArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::opengl_shader_program::OpenGLShaderProgram) -> ::libc::c_int {
      let name = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_attributeLocation_QString(original_self as *const ::opengl_shader_program::OpenGLShaderProgram, name as *const ::qt_core::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLShaderProgram::bind_attribute_location](../struct.OpenGLShaderProgram.html#method.bind_attribute_location) method.
  pub trait OpenGLShaderProgramBindAttributeLocationArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> ();
  }
  impl<'largs> OpenGLShaderProgramBindAttributeLocationArgs<'largs>
    for (&'largs ::qt_core::byte_array::ByteArray, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let location = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_bindAttributeLocation_QByteArray_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name as *const ::qt_core::byte_array::ByteArray, location) }
    }
  }
  impl<'largs> OpenGLShaderProgramBindAttributeLocationArgs<'largs>
    for (&'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let location = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_bindAttributeLocation_QString_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name as *const ::qt_core::string::String, location) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLShaderProgram::set_attribute_array](../struct.OpenGLShaderProgram.html#method.set_attribute_array) method.
  pub trait OpenGLShaderProgramSetAttributeArrayArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> ();
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::vector_2d::Vector2D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector2D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::vector_2d::Vector2D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let stride = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector2D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, stride)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::vector_3d::Vector3D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector3D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::vector_3d::Vector3D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let stride = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector3D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, stride)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::vector_4d::Vector4D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector4D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::vector_4d::Vector4D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let stride = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_QVector4D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, stride)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::libc::c_float, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let tuple_size = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_float_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, tuple_size)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::libc::c_float, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let tuple_size = self.2;
      let stride = self.3;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_float_int_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, tuple_size, stride)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_uint, *const ::libc::c_void, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let type_ = self.1;
      let values = self.2;
      let tuple_size = self.3;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_unsigned_int_void_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, type_, values, tuple_size)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_uint, *const ::libc::c_void, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let type_ = self.1;
      let values = self.2;
      let tuple_size = self.3;
      let stride = self.4;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_char_unsigned_int_void_int_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, type_, values, tuple_size, stride)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs> for (::libc::c_int, *const ::vector_2d::Vector2D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector2D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (::libc::c_int, *const ::vector_2d::Vector2D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let stride = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector2D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, stride)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs> for (::libc::c_int, *const ::vector_3d::Vector3D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector3D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (::libc::c_int, *const ::vector_3d::Vector3D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let stride = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector3D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, stride)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs> for (::libc::c_int, *const ::vector_4d::Vector4D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector4D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (::libc::c_int, *const ::vector_4d::Vector4D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let stride = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_QVector4D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, stride)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (::libc::c_int, *const ::libc::c_float, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let tuple_size = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_float_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, tuple_size)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (::libc::c_int, *const ::libc::c_float, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let tuple_size = self.2;
      let stride = self.3;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_float_int_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, tuple_size, stride)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (::libc::c_int, ::libc::c_uint, *const ::libc::c_void, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let type_ = self.1;
      let values = self.2;
      let tuple_size = self.3;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_unsigned_int_void_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, type_, values, tuple_size)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeArrayArgs<'largs>
    for (::libc::c_int, ::libc::c_uint, *const ::libc::c_void, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let type_ = self.1;
      let values = self.2;
      let tuple_size = self.3;
      let stride = self.4;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeArray_int_unsigned_int_void_int_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, type_, values, tuple_size, stride)
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLShaderProgram::set_attribute_buffer](../struct.OpenGLShaderProgram.html#method.set_attribute_buffer) method.
  pub trait OpenGLShaderProgramSetAttributeBufferArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> ();
  }
  impl<'largs> OpenGLShaderProgramSetAttributeBufferArgs<'largs>
    for (::libc::c_int, ::libc::c_uint, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let type_ = self.1;
      let offset = self.2;
      let tuple_size = self.3;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeBuffer_location_type_offset_tupleSize(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, type_, offset, tuple_size) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeBufferArgs<'largs>
    for (::libc::c_int, ::libc::c_uint, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let type_ = self.1;
      let offset = self.2;
      let tuple_size = self.3;
      let stride = self.4;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeBuffer_location_type_offset_tupleSize_stride(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, type_, offset, tuple_size, stride) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLShaderProgram::set_attribute_buffer_unsafe](../struct.OpenGLShaderProgram.html#method.set_attribute_buffer_unsafe) method.
  pub trait OpenGLShaderProgramSetAttributeBufferUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> ();
  }
  impl<'largs> OpenGLShaderProgramSetAttributeBufferUnsafeArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_uint, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let type_ = self.1;
      let offset = self.2;
      let tuple_size = self.3;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeBuffer_name_type_offset_tupleSize(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, type_, offset, tuple_size)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeBufferUnsafeArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_uint, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let type_ = self.1;
      let offset = self.2;
      let tuple_size = self.3;
      let stride = self.4;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeBuffer_name_type_offset_tupleSize_stride(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, type_, offset, tuple_size, stride)
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLShaderProgram::set_attribute_value](../struct.OpenGLShaderProgram.html#method.set_attribute_value) method.
  pub trait OpenGLShaderProgramSetAttributeValueArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> ();
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueArgs<'largs> for (::libc::c_int, &'largs ::color::Color) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_QColor(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value as *const ::color::Color) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueArgs<'largs> for (::libc::c_int, &'largs ::vector_2d::Vector2D) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_QVector2D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value as *const ::vector_2d::Vector2D) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueArgs<'largs> for (::libc::c_int, &'largs ::vector_3d::Vector3D) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_QVector3D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value as *const ::vector_3d::Vector3D) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueArgs<'largs> for (::libc::c_int, &'largs ::vector_4d::Vector4D) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_QVector4D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value as *const ::vector_4d::Vector4D) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueArgs<'largs> for (::libc::c_int, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueArgs<'largs> for (::libc::c_int, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let x = self.1;
      let y = self.2;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, x, y) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueArgs<'largs>
    for (::libc::c_int, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let x = self.1;
      let y = self.2;
      let z = self.3;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_float_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, x, y, z) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueArgs<'largs>
    for (::libc::c_int, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let x = self.1;
      let y = self.2;
      let z = self.3;
      let w = self.4;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_float_float_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, x, y, z, w) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLShaderProgram::set_attribute_value_unsafe](../struct.OpenGLShaderProgram.html#method.set_attribute_value_unsafe) method.
  pub trait OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> ();
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::color::Color) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_QColor(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value as *const ::color::Color)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::vector_2d::Vector2D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_QVector2D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value as *const ::vector_2d::Vector2D)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::vector_3d::Vector3D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_QVector3D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value as *const ::vector_3d::Vector3D)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::vector_4d::Vector4D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_QVector4D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value as *const ::vector_4d::Vector4D)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_float) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_float, ::libc::c_float) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let x = self.1;
      let y = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, x, y)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let x = self.1;
      let y = self.2;
      let z = self.3;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_float_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, x, y, z)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let x = self.1;
      let y = self.2;
      let z = self.3;
      let w = self.4;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_float_float_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, x, y, z, w)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, *const ::libc::c_float, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let columns = self.2;
      let rows = self.3;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_char_float_int_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, columns, rows)
    }
  }
  impl<'largs> OpenGLShaderProgramSetAttributeValueUnsafeArgs<'largs>
    for (::libc::c_int, *const ::libc::c_float, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let columns = self.2;
      let rows = self.3;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setAttributeValue_int_float_int_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, columns, rows)
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLShaderProgram::set_uniform_value](../struct.OpenGLShaderProgram.html#method.set_uniform_value) method.
  pub trait OpenGLShaderProgramSetUniformValueArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> ();
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, i32) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_GLint(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, u32) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_GLuint(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, &'largs ::color::Color) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let color = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QColor(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, color as *const ::color::Color) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, &'largs ::matrix_4x4::Matrix4X4) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QMatrix4x4(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value as *const ::matrix_4x4::Matrix4X4) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::point::Point) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let point = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QPoint(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, point as *const ::qt_core::point::Point) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::point_f::PointF) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let point = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QPointF(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, point as *const ::qt_core::point_f::PointF) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::size::Size) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let size = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QSize(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, size as *const ::qt_core::size::Size) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::size_f::SizeF) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let size = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QSizeF(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, size as *const ::qt_core::size_f::SizeF) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, &'largs ::transform::Transform) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QTransform(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value as *const ::transform::Transform) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, &'largs ::vector_2d::Vector2D) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QVector2D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value as *const ::vector_2d::Vector2D) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, &'largs ::vector_3d::Vector3D) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QVector3D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value as *const ::vector_3d::Vector3D) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, &'largs ::vector_4d::Vector4D) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_QVector4D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value as *const ::vector_4d::Vector4D) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, value) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs> for (::libc::c_int, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let x = self.1;
      let y = self.2;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, x, y) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs>
    for (::libc::c_int, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let x = self.1;
      let y = self.2;
      let z = self.3;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_float_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, x, y, z) }
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArgs<'largs>
    for (::libc::c_int, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let x = self.1;
      let y = self.2;
      let z = self.3;
      let w = self.4;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_int_float_float_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, x, y, z, w) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLShaderProgram::set_uniform_value_array](../struct.OpenGLShaderProgram.html#method.set_uniform_value_array) method.
  pub trait OpenGLShaderProgramSetUniformValueArrayArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> ();
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (*const ::libc::c_char, *const i32, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_GLint_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (*const ::libc::c_char, *const u32, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_GLuint_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::matrix_4x4::Matrix4X4, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_QMatrix4x4_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::vector_2d::Vector2D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_QVector2D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::vector_3d::Vector3D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_QVector3D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::vector_4d::Vector4D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_QVector4D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (*const ::libc::c_char, *const ::libc::c_float, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let values = self.1;
      let count = self.2;
      let tuple_size = self.3;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_char_float_int_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, values, count, tuple_size)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs> for (::libc::c_int, *const i32, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_GLint_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs> for (::libc::c_int, *const u32, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_GLuint_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (::libc::c_int, *const ::matrix_4x4::Matrix4X4, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_QMatrix4x4_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (::libc::c_int, *const ::vector_2d::Vector2D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_QVector2D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (::libc::c_int, *const ::vector_3d::Vector3D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_QVector3D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (::libc::c_int, *const ::vector_4d::Vector4D, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let count = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_QVector4D_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, count)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueArrayArgs<'largs>
    for (::libc::c_int, *const ::libc::c_float, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let location = self.0;
      let values = self.1;
      let count = self.2;
      let tuple_size = self.3;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValueArray_int_float_int_int(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, location, values, count, tuple_size)
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLShaderProgram::set_uniform_value_unsafe](../struct.OpenGLShaderProgram.html#method.set_uniform_value_unsafe) method.
  pub trait OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> ();
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs> for (*const ::libc::c_char, i32) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_GLint(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs> for (*const ::libc::c_char, u32) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_GLuint(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs> for (*const ::libc::c_char, &'largs ::color::Color) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let color = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QColor(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, color as *const ::color::Color)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::matrix_4x4::Matrix4X4) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QMatrix4x4(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value as *const ::matrix_4x4::Matrix4X4)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::qt_core::point::Point) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let point = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QPoint(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, point as *const ::qt_core::point::Point)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::qt_core::point_f::PointF) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let point = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QPointF(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, point as *const ::qt_core::point_f::PointF)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::qt_core::size::Size) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let size = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QSize(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, size as *const ::qt_core::size::Size)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::qt_core::size_f::SizeF) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let size = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QSizeF(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, size as *const ::qt_core::size_f::SizeF)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::transform::Transform) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QTransform(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value as *const ::transform::Transform)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::vector_2d::Vector2D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QVector2D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value as *const ::vector_2d::Vector2D)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::vector_3d::Vector3D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QVector3D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value as *const ::vector_3d::Vector3D)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, &'largs ::vector_4d::Vector4D) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_QVector4D(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value as *const ::vector_4d::Vector4D)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_float) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let value = self.1;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, value)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_float, ::libc::c_float) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let x = self.1;
      let y = self.2;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, x, y)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let x = self.1;
      let y = self.2;
      let z = self.3;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_float_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, x, y, z)
    }
  }
  impl<'largs> OpenGLShaderProgramSetUniformValueUnsafeArgs<'largs>
    for (*const ::libc::c_char, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_shader_program::OpenGLShaderProgram) -> () {
      let name = self.0;
      let x = self.1;
      let y = self.2;
      let z = self.3;
      let w = self.4;
      ::ffi::qt_gui_c_QOpenGLShaderProgram_setUniformValue_char_float_float_float_float(original_self as *mut ::opengl_shader_program::OpenGLShaderProgram, name, x, y, z, w)
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLShaderProgram::uniform_location](../struct.OpenGLShaderProgram.html#method.uniform_location) method.
  pub trait OpenGLShaderProgramUniformLocationArgs<'largs> {
    fn exec(self, original_self: &'largs ::opengl_shader_program::OpenGLShaderProgram) -> ::libc::c_int;
  }
  impl<'largs> OpenGLShaderProgramUniformLocationArgs<'largs> for &'largs ::qt_core::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::opengl_shader_program::OpenGLShaderProgram) -> ::libc::c_int {
      let name = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_uniformLocation_QByteArray(original_self as *const ::opengl_shader_program::OpenGLShaderProgram, name as *const ::qt_core::byte_array::ByteArray) }
    }
  }
  impl<'largs> OpenGLShaderProgramUniformLocationArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::opengl_shader_program::OpenGLShaderProgram) -> ::libc::c_int {
      let name = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLShaderProgram_uniformLocation_QString(original_self as *const ::opengl_shader_program::OpenGLShaderProgram, name as *const ::qt_core::string::String) }
    }
  }
}
