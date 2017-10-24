/// C++ type: <span style='color: green;'>```Qt3DRender::QShaderProgram```</span>
#[repr(C)]
pub struct ShaderProgram(u8);

impl ShaderProgram {
  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QShaderProgram::computeShaderCode() const```</span>
  ///
  ///
  pub fn compute_shader_code(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_computeShaderCode_to_output(self as *const ::shader_program::ShaderProgram, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QShaderProgram::fragmentShaderCode() const```</span>
  ///
  ///
  pub fn fragment_shader_code(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_fragmentShaderCode_to_output(self as *const ::shader_program::ShaderProgram, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QShaderProgram::geometryShaderCode() const```</span>
  ///
  ///
  pub fn geometry_shader_code(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_geometryShaderCode_to_output(self as *const ::shader_program::ShaderProgram, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QByteArray Qt3DRender::QShaderProgram::loadSource(const QUrl& sourceUrl)```</span>
  ///
  ///
  pub fn load_source(source_url: &::qt_core::url::Url) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_loadSource_to_output(source_url as *const ::qt_core::url::Url, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString Qt3DRender::QShaderProgram::log() const```</span>
  ///
  ///
  pub fn log(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_log_to_output(self as *const ::shader_program::ShaderProgram,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QShaderProgram::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_metaObject(self as *const ::shader_program::ShaderProgram)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QShaderProgram::QShaderProgram()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::shader_program::ShaderProgram> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QShaderProgram::QShaderProgram(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::shader_program::ShaderProgram> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QShaderProgram::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_qt_metacall(self as *mut ::shader_program::ShaderProgram,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QShaderProgram::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_qt_metacast(self as *mut ::shader_program::ShaderProgram, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QShaderProgram::setComputeShaderCode(const QByteArray& computeShaderCode)```</span>
  ///
  ///
  pub fn set_compute_shader_code(&mut self, compute_shader_code: &::qt_core::byte_array::ByteArray) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_setComputeShaderCode(self as *mut ::shader_program::ShaderProgram, compute_shader_code as *const ::qt_core::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QShaderProgram::setFragmentShaderCode(const QByteArray& fragmentShaderCode)```</span>
  ///
  ///
  pub fn set_fragment_shader_code(&mut self, fragment_shader_code: &::qt_core::byte_array::ByteArray) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_setFragmentShaderCode(self as *mut ::shader_program::ShaderProgram, fragment_shader_code as *const ::qt_core::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QShaderProgram::setGeometryShaderCode(const QByteArray& geometryShaderCode)```</span>
  ///
  ///
  pub fn set_geometry_shader_code(&mut self, geometry_shader_code: &::qt_core::byte_array::ByteArray) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_setGeometryShaderCode(self as *mut ::shader_program::ShaderProgram, geometry_shader_code as *const ::qt_core::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QShaderProgram::setShaderCode(Qt3DRender::QShaderProgram::ShaderType type, const QByteArray& shaderCode)```</span>
  ///
  ///
  pub fn set_shader_code(&mut self,
                         type_: ::shader_program::ShaderType,
                         shader_code: &::qt_core::byte_array::ByteArray) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_setShaderCode(self as *mut ::shader_program::ShaderProgram, type_, shader_code as *const ::qt_core::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QShaderProgram::setTessellationControlShaderCode(const QByteArray& tessellationControlShaderCode)```</span>
  ///
  ///
  pub fn set_tessellation_control_shader_code(&mut self,
                                              tessellation_control_shader_code: &::qt_core::byte_array::ByteArray) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_setTessellationControlShaderCode(self as *mut ::shader_program::ShaderProgram, tessellation_control_shader_code as *const ::qt_core::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QShaderProgram::setTessellationEvaluationShaderCode(const QByteArray& tessellationEvaluationShaderCode)```</span>
  ///
  ///
pub fn set_tessellation_evaluation_shader_code(&mut self, tessellation_evaluation_shader_code: &::qt_core::byte_array::ByteArray) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_setTessellationEvaluationShaderCode(self as *mut ::shader_program::ShaderProgram, tessellation_evaluation_shader_code as *const ::qt_core::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QShaderProgram::setVertexShaderCode(const QByteArray& vertexShaderCode)```</span>
  ///
  ///
  pub fn set_vertex_shader_code(&mut self, vertex_shader_code: &::qt_core::byte_array::ByteArray) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_setVertexShaderCode(self as *mut ::shader_program::ShaderProgram, vertex_shader_code as *const ::qt_core::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QShaderProgram::shaderCode(Qt3DRender::QShaderProgram::ShaderType type) const```</span>
  ///
  ///
  pub fn shader_code(&self, type_: ::shader_program::ShaderType) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_shaderCode_to_output(self as *const ::shader_program::ShaderProgram, type_, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QShaderProgram::Status Qt3DRender::QShaderProgram::status() const```</span>
  ///
  ///
  pub fn status(&self) -> ::shader_program::Status {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_status(self as *const ::shader_program::ShaderProgram) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QShaderProgram::tessellationControlShaderCode() const```</span>
  ///
  ///
  pub fn tessellation_control_shader_code(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_tessellationControlShaderCode_to_output(self as *const ::shader_program::ShaderProgram, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QShaderProgram::tessellationEvaluationShaderCode() const```</span>
  ///
  ///
  pub fn tessellation_evaluation_shader_code(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_tessellationEvaluationShaderCode_to_output(self as *const ::shader_program::ShaderProgram, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QShaderProgram::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QShaderProgram::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QShaderProgram::vertexShaderCode() const```</span>
  ///
  ///
  pub fn vertex_shader_code(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_vertexShaderCode_to_output(self as *const ::shader_program::ShaderProgram, &mut object);
      }
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::shader_program::ShaderProgram {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QShaderProgram_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ShaderProgram`.
  pub struct Signals<'a>(&'a ::shader_program::ShaderProgram);
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::computeShaderCodeChanged`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().compute_shader_code_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct ComputeShaderCodeChanged<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for ComputeShaderCodeChanged<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2computeShaderCodeChanged(const QByteArray&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ComputeShaderCodeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::tessellationEvaluationShaderCodeChanged`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().tessellation_evaluation_shader_code_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct TessellationEvaluationShaderCodeChanged<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for TessellationEvaluationShaderCodeChanged<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2tessellationEvaluationShaderCodeChanged(const QByteArray&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TessellationEvaluationShaderCodeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::tessellationControlShaderCodeChanged`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().tessellation_control_shader_code_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct TessellationControlShaderCodeChanged<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for TessellationControlShaderCodeChanged<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2tessellationControlShaderCodeChanged(const QByteArray&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TessellationControlShaderCodeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::fragmentShaderCodeChanged`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().fragment_shader_code_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct FragmentShaderCodeChanged<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for FragmentShaderCodeChanged<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2fragmentShaderCodeChanged(const QByteArray&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FragmentShaderCodeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::parentChanged`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct ParentChanged<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for ParentChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2parentChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ParentChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::enabledChanged`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct EnabledChanged<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for EnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2enabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::nodeDestroyed`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct NodeDestroyed<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for NodeDestroyed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2nodeDestroyed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NodeDestroyed<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for DefaultPropertyTrackingModeChanged<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2defaultPropertyTrackingModeChanged(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DefaultPropertyTrackingModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::geometryShaderCodeChanged`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().geometry_shader_code_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct GeometryShaderCodeChanged<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for GeometryShaderCodeChanged<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2geometryShaderCodeChanged(const QByteArray&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GeometryShaderCodeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::statusChanged`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().status_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct StatusChanged<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for StatusChanged<'a> {
    type Arguments = (::shader_program::Status,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2statusChanged(Qt3DRender::QShaderProgram::Status)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StatusChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::vertexShaderCodeChanged`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().vertex_shader_code_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct VertexShaderCodeChanged<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for VertexShaderCodeChanged<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2vertexShaderCodeChanged(const QByteArray&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VertexShaderCodeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QShaderProgram::logChanged`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.signals().log_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct LogChanged<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for LogChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2logChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LogChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::computeShaderCodeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn compute_shader_code_changed(&self) -> ComputeShaderCodeChanged {
      ComputeShaderCodeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::tessellationEvaluationShaderCodeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tessellation_evaluation_shader_code_changed(&self) -> TessellationEvaluationShaderCodeChanged {
      TessellationEvaluationShaderCodeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::tessellationControlShaderCodeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn tessellation_control_shader_code_changed(&self) -> TessellationControlShaderCodeChanged {
      TessellationControlShaderCodeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::fragmentShaderCodeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn fragment_shader_code_changed(&self) -> FragmentShaderCodeChanged {
      FragmentShaderCodeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::geometryShaderCodeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_shader_code_changed(&self) -> GeometryShaderCodeChanged {
      GeometryShaderCodeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::statusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn status_changed(&self) -> StatusChanged {
      StatusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::vertexShaderCodeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertex_shader_code_changed(&self) -> VertexShaderCodeChanged {
      VertexShaderCodeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QShaderProgram::logChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn log_changed(&self) -> LogChanged {
      LogChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ShaderProgram`.
  pub struct Slots<'a>(&'a ::shader_program::ShaderProgram);
  /// Represents a built-in Qt slot `Qt3DRender::QShaderProgram::setGeometryShaderCode`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.slots().set_geometry_shader_code()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct SetGeometryShaderCode<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for SetGeometryShaderCode<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGeometryShaderCode(const QByteArray&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QShaderProgram::setTessellationControlShaderCode`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.slots().set_tessellation_control_shader_code()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct SetTessellationControlShaderCode<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for SetTessellationControlShaderCode<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTessellationControlShaderCode(const QByteArray&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QShaderProgram::setVertexShaderCode`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.slots().set_vertex_shader_code()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct SetVertexShaderCode<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for SetVertexShaderCode<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVertexShaderCode(const QByteArray&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QShaderProgram::setTessellationEvaluationShaderCode`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.slots().set_tessellation_evaluation_shader_code()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct SetTessellationEvaluationShaderCode<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for SetTessellationEvaluationShaderCode<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTessellationEvaluationShaderCode(const QByteArray&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QShaderProgram::setEnabled`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct SetEnabled<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QShaderProgram::setComputeShaderCode`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.slots().set_compute_shader_code()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct SetComputeShaderCode<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for SetComputeShaderCode<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setComputeShaderCode(const QByteArray&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QShaderProgram::setParent`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct SetParent<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QShaderProgram::setFragmentShaderCode`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.slots().set_fragment_shader_code()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct SetFragmentShaderCode<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for SetFragmentShaderCode<'a> {
    type Arguments = (&'static ::qt_core::byte_array::ByteArray,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFragmentShaderCode(const QByteArray&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QShaderProgram::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `ShaderProgram` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ShaderProgram` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::shader_program::ShaderProgram);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QShaderProgram::setGeometryShaderCode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_geometry_shader_code(&self) -> SetGeometryShaderCode {
      SetGeometryShaderCode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QShaderProgram::setTessellationControlShaderCode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_tessellation_control_shader_code(&self) -> SetTessellationControlShaderCode {
      SetTessellationControlShaderCode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QShaderProgram::setVertexShaderCode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertex_shader_code(&self) -> SetVertexShaderCode {
      SetVertexShaderCode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QShaderProgram::setTessellationEvaluationShaderCode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_tessellation_evaluation_shader_code(&self) -> SetTessellationEvaluationShaderCode {
      SetTessellationEvaluationShaderCode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QShaderProgram::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QShaderProgram::setComputeShaderCode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_compute_shader_code(&self) -> SetComputeShaderCode {
      SetComputeShaderCode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QShaderProgram::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QShaderProgram::setFragmentShaderCode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_fragment_shader_code(&self) -> SetFragmentShaderCode {
      SetFragmentShaderCode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QShaderProgram::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
  }
  impl ::shader_program::ShaderProgram {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QShaderProgram::ShaderType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ShaderType {
  /// C++ enum variant: <span style='color: green;'>```Vertex = 0```</span>
  Vertex = 0,
  /// C++ enum variant: <span style='color: green;'>```Fragment = 1```</span>
  Fragment = 1,
  /// C++ enum variant: <span style='color: green;'>```TessellationControl = 2```</span>
  TessellationControl = 2,
  /// C++ enum variant: <span style='color: green;'>```TessellationEvaluation = 3```</span>
  TessellationEvaluation = 3,
  /// C++ enum variant: <span style='color: green;'>```Geometry = 4```</span>
  Geometry = 4,
  /// C++ enum variant: <span style='color: green;'>```Compute = 5```</span>
  Compute = 5,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QShaderProgram::Status```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Status {
  /// C++ enum variant: <span style='color: green;'>```NotReady = 0```</span>
  NotReady = 0,
  /// C++ enum variant: <span style='color: green;'>```Ready = 1```</span>
  Ready = 1,
  /// C++ enum variant: <span style='color: green;'>```Error = 2```</span>
  Error = 2,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::shader_program::ShaderProgram {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::shader_program::ShaderProgram) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::shader_program::ShaderProgram as *mut ::shader_program::ShaderProgram) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::shader_program::ShaderProgram {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QShaderProgram_G_static_cast_QObject_ptr(self as *mut ::shader_program::ShaderProgram)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QShaderProgram_G_static_cast_QObject_ptr(self as *const ::shader_program::ShaderProgram as *mut ::shader_program::ShaderProgram) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::shader_program::ShaderProgram> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::shader_program::ShaderProgram {
    let ffi_result = ::ffi::qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DRender_QShaderProgram_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::shader_program::ShaderProgram {
    let ffi_result = ::ffi::qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DRender_QShaderProgram_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::shader_program::ShaderProgram> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::shader_program::ShaderProgram {
    let ffi_result = ::ffi::qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DRender_QShaderProgram_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::shader_program::ShaderProgram {
    let ffi_result = ::ffi::qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DRender_QShaderProgram_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::shader_program::ShaderProgram {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::shader_program::ShaderProgram as *mut ::shader_program::ShaderProgram) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::shader_program::ShaderProgram {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QShaderProgram_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::shader_program::ShaderProgram) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
