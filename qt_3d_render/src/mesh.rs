/// C++ type: <span style='color: green;'>```Qt3DRender::QMesh```</span>
#[repr(C)]
pub struct Mesh(u8);

impl Mesh {
  /// C++ method: <span style='color: green;'>```QString Qt3DRender::QMesh::meshName() const```</span>
  ///
  ///
  pub fn mesh_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QMesh_meshName_to_output(self as *const ::mesh::Mesh, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QMesh::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QMesh_metaObject(self as *const ::mesh::Mesh) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QMesh::QMesh()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::mesh::Mesh> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QMesh_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QMesh::QMesh(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::mesh::Mesh> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QMesh_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QMesh::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QMesh_qt_metacall(self as *mut ::mesh::Mesh,
                                                       arg1 as *const ::qt_core::meta_object::Call,
                                                       arg2,
                                                       arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QMesh::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QMesh_qt_metacast(self as *mut ::mesh::Mesh, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QMesh::setMeshName(const QString& meshName)```</span>
  ///
  ///
  pub fn set_mesh_name(&mut self, mesh_name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QMesh_setMeshName(self as *mut ::mesh::Mesh,
                                                         mesh_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QMesh::setSource(const QUrl& source)```</span>
  ///
  ///
  pub fn set_source(&mut self, source: &::qt_core::url::Url) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QMesh_setSource(self as *mut ::mesh::Mesh,
                                                       source as *const ::qt_core::url::Url)
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl Qt3DRender::QMesh::source() const```</span>
  ///
  ///
  pub fn source(&self) -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QMesh_source_to_output(self as *const ::mesh::Mesh, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QMesh::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QMesh_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QMesh::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QMesh_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::mesh::Mesh {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QMesh_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Mesh`.
  pub struct Signals<'a>(&'a ::mesh::Mesh);
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::verticesPerPatchChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().vertices_per_patch_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct VerticesPerPatchChanged<'a>(&'a ::mesh::Mesh);
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
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::sourceChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().source_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SourceChanged<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SourceChanged<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceChanged(const QUrl&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::instanceCountChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().instance_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct InstanceCountChanged<'a>(&'a ::mesh::Mesh);
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
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::firstVertexChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().first_vertex_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct FirstVertexChanged<'a>(&'a ::mesh::Mesh);
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
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::primitiveRestartEnabledChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().primitive_restart_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct PrimitiveRestartEnabledChanged<'a>(&'a ::mesh::Mesh);
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
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::vertexCountChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().vertex_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct VertexCountChanged<'a>(&'a ::mesh::Mesh);
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
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::restartIndexValueChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().restart_index_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct RestartIndexValueChanged<'a>(&'a ::mesh::Mesh);
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
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::geometryChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct GeometryChanged<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for GeometryChanged<'a> {
    type Arguments = (*mut ::geometry::Geometry,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2geometryChanged(Qt3DRender::QGeometry*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GeometryChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::indexOffsetChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().index_offset_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct IndexOffsetChanged<'a>(&'a ::mesh::Mesh);
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
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::primitiveTypeChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().primitive_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct PrimitiveTypeChanged<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for PrimitiveTypeChanged<'a> {
    type Arguments = (::geometry_renderer::PrimitiveType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2primitiveTypeChanged(Qt3DRender::QGeometryRenderer::PrimitiveType)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for PrimitiveTypeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::firstInstanceChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().first_instance_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct FirstInstanceChanged<'a>(&'a ::mesh::Mesh);
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
  /// Represents a built-in Qt signal `Qt3DRender::QMesh::meshNameChanged`.
  ///
  /// An object of this type can be created from `Mesh` with `object.signals().mesh_name_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct MeshNameChanged<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for MeshNameChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2meshNameChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MeshNameChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::verticesPerPatchChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertices_per_patch_changed(&self) -> VerticesPerPatchChanged {
      VerticesPerPatchChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::sourceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_changed(&self) -> SourceChanged {
      SourceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::instanceCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn instance_count_changed(&self) -> InstanceCountChanged {
      InstanceCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::firstVertexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_vertex_changed(&self) -> FirstVertexChanged {
      FirstVertexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::primitiveRestartEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_restart_enabled_changed(&self) -> PrimitiveRestartEnabledChanged {
      PrimitiveRestartEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::vertexCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertex_count_changed(&self) -> VertexCountChanged {
      VertexCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::restartIndexValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn restart_index_value_changed(&self) -> RestartIndexValueChanged {
      RestartIndexValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::geometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_changed(&self) -> GeometryChanged {
      GeometryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::indexOffsetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn index_offset_changed(&self) -> IndexOffsetChanged {
      IndexOffsetChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::primitiveTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_type_changed(&self) -> PrimitiveTypeChanged {
      PrimitiveTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::firstInstanceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_instance_changed(&self) -> FirstInstanceChanged {
      FirstInstanceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QMesh::meshNameChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mesh_name_changed(&self) -> MeshNameChanged {
      MeshNameChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Mesh`.
  pub struct Slots<'a>(&'a ::mesh::Mesh);
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setFirstVertex`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_first_vertex()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetFirstVertex<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetFirstVertex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFirstVertex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setFirstInstance`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_first_instance()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetFirstInstance<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetFirstInstance<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFirstInstance(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setVerticesPerPatch`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_vertices_per_patch()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetVerticesPerPatch<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetVerticesPerPatch<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVerticesPerPatch(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setInstanceCount`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_instance_count()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetInstanceCount<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetInstanceCount<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setInstanceCount(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setGeometry`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_geometry()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetGeometry<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetGeometry<'a> {
    type Arguments = (*mut ::geometry::Geometry,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGeometry(Qt3DRender::QGeometry*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setSource`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_source()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetSource<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetSource<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSource(const QUrl&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setMeshName`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_mesh_name()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetMeshName<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetMeshName<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMeshName(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setVertexCount`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_vertex_count()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetVertexCount<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetVertexCount<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVertexCount(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setIndexOffset`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_index_offset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetIndexOffset<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetIndexOffset<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIndexOffset(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setPrimitiveType`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_primitive_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetPrimitiveType<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetPrimitiveType<'a> {
    type Arguments = (::geometry_renderer::PrimitiveType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPrimitiveType(Qt3DRender::QGeometryRenderer::PrimitiveType)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setRestartIndexValue`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_restart_index_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetRestartIndexValue<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetRestartIndexValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRestartIndexValue(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QMesh::setPrimitiveRestartEnabled`.
  ///
  /// An object of this type can be created from `Mesh` with `object.slots().set_primitive_restart_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Mesh` object.
  pub struct SetPrimitiveRestartEnabled<'a>(&'a ::mesh::Mesh);
  impl<'a> ::qt_core::connection::Receiver for SetPrimitiveRestartEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPrimitiveRestartEnabled(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setFirstVertex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_first_vertex(&self) -> SetFirstVertex {
      SetFirstVertex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setFirstInstance`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_first_instance(&self) -> SetFirstInstance {
      SetFirstInstance(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setVerticesPerPatch`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertices_per_patch(&self) -> SetVerticesPerPatch {
      SetVerticesPerPatch(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setInstanceCount`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_instance_count(&self) -> SetInstanceCount {
      SetInstanceCount(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setGeometry`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_geometry(&self) -> SetGeometry {
      SetGeometry(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setSource`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source(&self) -> SetSource {
      SetSource(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setMeshName`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_mesh_name(&self) -> SetMeshName {
      SetMeshName(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setVertexCount`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertex_count(&self) -> SetVertexCount {
      SetVertexCount(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setIndexOffset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_index_offset(&self) -> SetIndexOffset {
      SetIndexOffset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setPrimitiveType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_primitive_type(&self) -> SetPrimitiveType {
      SetPrimitiveType(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setRestartIndexValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_restart_index_value(&self) -> SetRestartIndexValue {
      SetRestartIndexValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QMesh::setPrimitiveRestartEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_primitive_restart_enabled(&self) -> SetPrimitiveRestartEnabled {
      SetPrimitiveRestartEnabled(self.0)
    }
  }
  impl ::mesh::Mesh {
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

impl ::cpp_utils::DynamicCast<::mesh::Mesh> for ::geometry_renderer::GeometryRenderer {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::mesh::Mesh> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMesh_G_dynamic_cast_Qt3DRender_QMesh_ptr(self as *mut ::geometry_renderer::GeometryRenderer) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::mesh::Mesh> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMesh_G_dynamic_cast_Qt3DRender_QMesh_ptr(self as *const ::geometry_renderer::GeometryRenderer as *mut ::geometry_renderer::GeometryRenderer) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::mesh::Mesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::mesh::Mesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::mesh::Mesh as *mut ::mesh::Mesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::mesh::Mesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::mesh::Mesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::mesh::Mesh as *mut ::mesh::Mesh)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::geometry_renderer::GeometryRenderer> for ::mesh::Mesh {
  fn static_cast_mut(&mut self) -> &mut ::geometry_renderer::GeometryRenderer {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::mesh::Mesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::mesh::Mesh as *mut ::mesh::Mesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::mesh::Mesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMesh_G_static_cast_QObject_ptr(self as *mut ::mesh::Mesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QMesh_G_static_cast_QObject_ptr(self as *const ::mesh::Mesh as *mut ::mesh::Mesh)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mesh::Mesh> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mesh::Mesh {
    let ffi_result =
      ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mesh::Mesh {
    let ffi_result = ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mesh::Mesh> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mesh::Mesh {
    let ffi_result = ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mesh::Mesh {
    let ffi_result = ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mesh::Mesh> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mesh::Mesh {
    let ffi_result = ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mesh::Mesh {
    let ffi_result = ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::mesh::Mesh> for ::geometry_renderer::GeometryRenderer {
  unsafe fn static_cast_mut(&mut self) -> &mut ::mesh::Mesh {
    let ffi_result = ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DRender_QGeometryRenderer(self as *mut ::geometry_renderer::GeometryRenderer);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::mesh::Mesh {
    let ffi_result = ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QMesh_ptr_Qt3DRender_QGeometryRenderer(self as *const ::geometry_renderer::GeometryRenderer as *mut ::geometry_renderer::GeometryRenderer);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::mesh::Mesh {
  type Target = ::geometry_renderer::GeometryRenderer;
  fn deref(&self) -> &::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::mesh::Mesh as *mut ::mesh::Mesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::mesh::Mesh {
  fn deref_mut(&mut self) -> &mut ::geometry_renderer::GeometryRenderer {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::mesh::Mesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
