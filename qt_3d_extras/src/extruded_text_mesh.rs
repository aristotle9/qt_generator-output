/// C++ type: <span style='color: green;'>```Qt3DExtras::QExtrudedTextMesh```</span>
#[repr(C)]
pub struct ExtrudedTextMesh(u8);

impl ExtrudedTextMesh {
  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QExtrudedTextMesh::depth() const```</span>
  ///
  ///
  pub fn depth(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_depth(self as *const ::extruded_text_mesh::ExtrudedTextMesh)
    }
  }

  /// C++ method: <span style='color: green;'>```QFont Qt3DExtras::QExtrudedTextMesh::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_font_to_output(self as *const ::extruded_text_mesh::ExtrudedTextMesh, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QExtrudedTextMesh::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_metaObject(self as *const ::extruded_text_mesh::ExtrudedTextMesh) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QExtrudedTextMesh::QExtrudedTextMesh()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::extruded_text_mesh::ExtrudedTextMesh> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QExtrudedTextMesh::QExtrudedTextMesh(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::extruded_text_mesh::ExtrudedTextMesh> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QExtrudedTextMesh::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_qt_metacall(self as *mut ::extruded_text_mesh::ExtrudedTextMesh, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QExtrudedTextMesh::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_qt_metacast(self as *mut ::extruded_text_mesh::ExtrudedTextMesh, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QExtrudedTextMesh::setDepth(float depth)```</span>
  ///
  ///
  pub fn set_depth(&mut self, depth: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_setDepth(self as *mut ::extruded_text_mesh::ExtrudedTextMesh,
                                                                  depth)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QExtrudedTextMesh::setFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, font: &::qt_gui::font::Font) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_setFont(self as *mut ::extruded_text_mesh::ExtrudedTextMesh,
                                                                 font as *const ::qt_gui::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QExtrudedTextMesh::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_setText(self as *mut ::extruded_text_mesh::ExtrudedTextMesh,
                                                                 text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QString Qt3DExtras::QExtrudedTextMesh::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_text_to_output(self as *const ::extruded_text_mesh::ExtrudedTextMesh, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QExtrudedTextMesh::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QExtrudedTextMesh::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::extruded_text_mesh::ExtrudedTextMesh {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextMesh_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ExtrudedTextMesh`.
  pub struct Signals<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::fontChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().font_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct FontChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for FontChanged<'a> {
    type Arguments = (&'static ::qt_gui::font::Font,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2fontChanged(const QFont&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FontChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::primitiveTypeChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().primitive_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct PrimitiveTypeChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for PrimitiveTypeChanged<'a> {
    type Arguments = (::qt_3d_render::geometry_renderer::PrimitiveType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2primitiveTypeChanged(Qt3DRender::QGeometryRenderer::PrimitiveType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PrimitiveTypeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::instanceCountChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().instance_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct InstanceCountChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for InstanceCountChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2instanceCountChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for InstanceCountChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::vertexCountChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().vertex_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct VertexCountChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for VertexCountChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2vertexCountChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VertexCountChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::geometryChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct GeometryChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for GeometryChanged<'a> {
    type Arguments = (*mut ::qt_3d_render::geometry::Geometry,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2geometryChanged(Qt3DRender::QGeometry*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GeometryChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::firstVertexChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().first_vertex_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct FirstVertexChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for FirstVertexChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2firstVertexChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FirstVertexChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::depthChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().depth_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct DepthChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for DepthChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2depthChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DepthChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::textChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct TextChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for TextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::firstInstanceChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().first_instance_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct FirstInstanceChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for FirstInstanceChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2firstInstanceChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FirstInstanceChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::verticesPerPatchChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().vertices_per_patch_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct VerticesPerPatchChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for VerticesPerPatchChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2verticesPerPatchChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for VerticesPerPatchChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::primitiveRestartEnabledChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().primitive_restart_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct PrimitiveRestartEnabledChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for PrimitiveRestartEnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2primitiveRestartEnabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PrimitiveRestartEnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::indexOffsetChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().index_offset_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct IndexOffsetChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for IndexOffsetChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2indexOffsetChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for IndexOffsetChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::restartIndexValueChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.signals().restart_index_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct RestartIndexValueChanged<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for RestartIndexValueChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2restartIndexValueChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RestartIndexValueChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::fontChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn font_changed(&self) -> FontChanged {
      FontChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::primitiveTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_type_changed(&self) -> PrimitiveTypeChanged {
      PrimitiveTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::instanceCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn instance_count_changed(&self) -> InstanceCountChanged {
      InstanceCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::vertexCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertex_count_changed(&self) -> VertexCountChanged {
      VertexCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::geometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_changed(&self) -> GeometryChanged {
      GeometryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::firstVertexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_vertex_changed(&self) -> FirstVertexChanged {
      FirstVertexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::depthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn depth_changed(&self) -> DepthChanged {
      DepthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::textChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn text_changed(&self) -> TextChanged {
      TextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::firstInstanceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_instance_changed(&self) -> FirstInstanceChanged {
      FirstInstanceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::verticesPerPatchChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertices_per_patch_changed(&self) -> VerticesPerPatchChanged {
      VerticesPerPatchChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::primitiveRestartEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_restart_enabled_changed(&self) -> PrimitiveRestartEnabledChanged {
      PrimitiveRestartEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::indexOffsetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn index_offset_changed(&self) -> IndexOffsetChanged {
      IndexOffsetChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextMesh::restartIndexValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn restart_index_value_changed(&self) -> RestartIndexValueChanged {
      RestartIndexValueChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ExtrudedTextMesh`.
  pub struct Slots<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setRestartIndexValue`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_restart_index_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetRestartIndexValue<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetRestartIndexValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRestartIndexValue(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setFirstVertex`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_first_vertex()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetFirstVertex<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetFirstVertex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFirstVertex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setText`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetText<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setGeometry`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_geometry()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetGeometry<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetGeometry<'a> {
    type Arguments = (*mut ::qt_3d_render::geometry::Geometry,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGeometry(Qt3DRender::QGeometry*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setInstanceCount`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_instance_count()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetInstanceCount<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetInstanceCount<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setInstanceCount(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setFont`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_font()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetFont<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetFont<'a> {
    type Arguments = (&'static ::qt_gui::font::Font,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFont(const QFont&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setIndexOffset`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_index_offset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetIndexOffset<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetIndexOffset<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIndexOffset(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setFirstInstance`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_first_instance()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetFirstInstance<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetFirstInstance<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFirstInstance(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setDepth`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_depth()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetDepth<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetDepth<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDepth(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setPrimitiveRestartEnabled`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_primitive_restart_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetPrimitiveRestartEnabled<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetPrimitiveRestartEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPrimitiveRestartEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setVerticesPerPatch`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_vertices_per_patch()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetVerticesPerPatch<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetVerticesPerPatch<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVerticesPerPatch(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setPrimitiveType`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_primitive_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetPrimitiveType<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetPrimitiveType<'a> {
    type Arguments = (::qt_3d_render::geometry_renderer::PrimitiveType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPrimitiveType(Qt3DRender::QGeometryRenderer::PrimitiveType)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setVertexCount`.
  ///
  /// An object of this type can be created from `ExtrudedTextMesh` with `object.slots().set_vertex_count()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextMesh` object.
  pub struct SetVertexCount<'a>(&'a ::extruded_text_mesh::ExtrudedTextMesh);
  impl<'a> ::qt_core::connection::Receiver for SetVertexCount<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVertexCount(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setRestartIndexValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_restart_index_value(&self) -> SetRestartIndexValue {
      SetRestartIndexValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setFirstVertex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_first_vertex(&self) -> SetFirstVertex {
      SetFirstVertex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_text(&self) -> SetText {
      SetText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setGeometry`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_geometry(&self) -> SetGeometry {
      SetGeometry(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setInstanceCount`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_instance_count(&self) -> SetInstanceCount {
      SetInstanceCount(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setFont`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font(&self) -> SetFont {
      SetFont(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setIndexOffset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_index_offset(&self) -> SetIndexOffset {
      SetIndexOffset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setFirstInstance`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_first_instance(&self) -> SetFirstInstance {
      SetFirstInstance(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setDepth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_depth(&self) -> SetDepth {
      SetDepth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setPrimitiveRestartEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_primitive_restart_enabled(&self) -> SetPrimitiveRestartEnabled {
      SetPrimitiveRestartEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setVerticesPerPatch`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertices_per_patch(&self) -> SetVerticesPerPatch {
      SetVerticesPerPatch(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setPrimitiveType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_primitive_type(&self) -> SetPrimitiveType {
      SetPrimitiveType(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextMesh::setVertexCount`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertex_count(&self) -> SetVertexCount {
      SetVertexCount(self.0)
    }
  }
  impl ::extruded_text_mesh::ExtrudedTextMesh {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::extruded_text_mesh::ExtrudedTextMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::extruded_text_mesh::ExtrudedTextMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::extruded_text_mesh::ExtrudedTextMesh as *mut ::extruded_text_mesh::ExtrudedTextMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::extruded_text_mesh::ExtrudedTextMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::extruded_text_mesh::ExtrudedTextMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::extruded_text_mesh::ExtrudedTextMesh as *mut ::extruded_text_mesh::ExtrudedTextMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry_renderer::GeometryRenderer> for ::extruded_text_mesh::ExtrudedTextMesh {
fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::extruded_text_mesh::ExtrudedTextMesh) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::extruded_text_mesh::ExtrudedTextMesh as *mut ::extruded_text_mesh::ExtrudedTextMesh) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::extruded_text_mesh::ExtrudedTextMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_QObject_ptr(self as *mut ::extruded_text_mesh::ExtrudedTextMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_QObject_ptr(self as *const ::extruded_text_mesh::ExtrudedTextMesh as *mut ::extruded_text_mesh::ExtrudedTextMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::extruded_text_mesh::ExtrudedTextMesh> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::extruded_text_mesh::ExtrudedTextMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::extruded_text_mesh::ExtrudedTextMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::extruded_text_mesh::ExtrudedTextMesh> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::extruded_text_mesh::ExtrudedTextMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::extruded_text_mesh::ExtrudedTextMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::extruded_text_mesh::ExtrudedTextMesh> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::extruded_text_mesh::ExtrudedTextMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::extruded_text_mesh::ExtrudedTextMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::extruded_text_mesh::ExtrudedTextMesh> for ::qt_3d_render::geometry_renderer::GeometryRenderer {
unsafe fn static_cast_mut(&mut self) -> &mut ::extruded_text_mesh::ExtrudedTextMesh {
let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_Qt3DRender_QGeometryRenderer(self as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::extruded_text_mesh::ExtrudedTextMesh {
let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DExtras_QExtrudedTextMesh_ptr_Qt3DRender_QGeometryRenderer(self as *const ::qt_3d_render::geometry_renderer::GeometryRenderer as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::extruded_text_mesh::ExtrudedTextMesh {
  type Target = ::qt_3d_render::geometry_renderer::GeometryRenderer;
  fn deref(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::extruded_text_mesh::ExtrudedTextMesh as *mut ::extruded_text_mesh::ExtrudedTextMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::extruded_text_mesh::ExtrudedTextMesh {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::extruded_text_mesh::ExtrudedTextMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
