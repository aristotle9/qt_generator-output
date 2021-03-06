/// C++ type: <span style='color: green;'>```Qt3DExtras::QCylinderMesh```</span>
#[repr(C)]
pub struct CylinderMesh(u8);

impl CylinderMesh {
  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QCylinderMesh::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_length(self as *const ::cylinder_mesh::CylinderMesh) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QCylinderMesh::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_metaObject(self as *const ::cylinder_mesh::CylinderMesh) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QCylinderMesh::QCylinderMesh()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::cylinder_mesh::CylinderMesh> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QCylinderMesh::QCylinderMesh(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::cylinder_mesh::CylinderMesh> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QCylinderMesh::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_qt_metacall(self as *mut ::cylinder_mesh::CylinderMesh,
                                                               arg1 as *const ::qt_core::meta_object::Call,
                                                               arg2,
                                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QCylinderMesh::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_qt_metacast(self as *mut ::cylinder_mesh::CylinderMesh, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QCylinderMesh::radius() const```</span>
  ///
  ///
  pub fn radius(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_radius(self as *const ::cylinder_mesh::CylinderMesh) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QCylinderMesh::rings() const```</span>
  ///
  ///
  pub fn rings(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_rings(self as *const ::cylinder_mesh::CylinderMesh) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCylinderMesh::setLength(float length)```</span>
  ///
  ///
  pub fn set_length(&mut self, length: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_setLength(self as *mut ::cylinder_mesh::CylinderMesh, length)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCylinderMesh::setRadius(float radius)```</span>
  ///
  ///
  pub fn set_radius(&mut self, radius: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_setRadius(self as *mut ::cylinder_mesh::CylinderMesh, radius)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCylinderMesh::setRings(int rings)```</span>
  ///
  ///
  pub fn set_rings(&mut self, rings: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_setRings(self as *mut ::cylinder_mesh::CylinderMesh, rings)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCylinderMesh::setSlices(int slices)```</span>
  ///
  ///
  pub fn set_slices(&mut self, slices: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_setSlices(self as *mut ::cylinder_mesh::CylinderMesh, slices)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QCylinderMesh::slices() const```</span>
  ///
  ///
  pub fn slices(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_slices(self as *const ::cylinder_mesh::CylinderMesh) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QCylinderMesh::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QCylinderMesh::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::cylinder_mesh::CylinderMesh {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderMesh_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `CylinderMesh`.
  pub struct Signals<'a>(&'a ::cylinder_mesh::CylinderMesh);
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::lengthChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().length_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct LengthChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
  impl<'a> ::qt_core::connection::Receiver for LengthChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2lengthChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LengthChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::primitiveTypeChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().primitive_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct PrimitiveTypeChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::ringsChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().rings_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct RingsChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
  impl<'a> ::qt_core::connection::Receiver for RingsChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2ringsChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RingsChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::geometryChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct GeometryChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::indexOffsetChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().index_offset_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct IndexOffsetChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::primitiveRestartEnabledChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().primitive_restart_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct PrimitiveRestartEnabledChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::firstVertexChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().first_vertex_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct FirstVertexChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::radiusChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct RadiusChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
  impl<'a> ::qt_core::connection::Receiver for RadiusChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2radiusChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RadiusChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::verticesPerPatchChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().vertices_per_patch_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct VerticesPerPatchChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::vertexCountChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().vertex_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct VertexCountChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::restartIndexValueChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().restart_index_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct RestartIndexValueChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::instanceCountChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().instance_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct InstanceCountChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::firstInstanceChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().first_instance_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct FirstInstanceChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderMesh::slicesChanged`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.signals().slices_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct SlicesChanged<'a>(&'a ::cylinder_mesh::CylinderMesh);
  impl<'a> ::qt_core::connection::Receiver for SlicesChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2slicesChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SlicesChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::lengthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn length_changed(&self) -> LengthChanged {
      LengthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::primitiveTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_type_changed(&self) -> PrimitiveTypeChanged {
      PrimitiveTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::ringsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rings_changed(&self) -> RingsChanged {
      RingsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::geometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_changed(&self) -> GeometryChanged {
      GeometryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::indexOffsetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn index_offset_changed(&self) -> IndexOffsetChanged {
      IndexOffsetChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::primitiveRestartEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_restart_enabled_changed(&self) -> PrimitiveRestartEnabledChanged {
      PrimitiveRestartEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::firstVertexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_vertex_changed(&self) -> FirstVertexChanged {
      FirstVertexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::radiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn radius_changed(&self) -> RadiusChanged {
      RadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::verticesPerPatchChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertices_per_patch_changed(&self) -> VerticesPerPatchChanged {
      VerticesPerPatchChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::vertexCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertex_count_changed(&self) -> VertexCountChanged {
      VertexCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::restartIndexValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn restart_index_value_changed(&self) -> RestartIndexValueChanged {
      RestartIndexValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::instanceCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn instance_count_changed(&self) -> InstanceCountChanged {
      InstanceCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::firstInstanceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_instance_changed(&self) -> FirstInstanceChanged {
      FirstInstanceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderMesh::slicesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slices_changed(&self) -> SlicesChanged {
      SlicesChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `CylinderMesh`.
  pub struct Slots<'a>(&'a ::cylinder_mesh::CylinderMesh);
  /// Represents a built-in Qt slot `Qt3DExtras::QCylinderMesh::setSlices`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.slots().set_slices()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct SetSlices<'a>(&'a ::cylinder_mesh::CylinderMesh);
  impl<'a> ::qt_core::connection::Receiver for SetSlices<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSlices(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCylinderMesh::setLength`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.slots().set_length()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct SetLength<'a>(&'a ::cylinder_mesh::CylinderMesh);
  impl<'a> ::qt_core::connection::Receiver for SetLength<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLength(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCylinderMesh::setFirstVertex`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.slots().set_first_vertex()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct SetFirstVertex<'a>(&'a ::cylinder_mesh::CylinderMesh);
  impl<'a> ::qt_core::connection::Receiver for SetFirstVertex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFirstVertex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCylinderMesh::setRadius`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.slots().set_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct SetRadius<'a>(&'a ::cylinder_mesh::CylinderMesh);
  impl<'a> ::qt_core::connection::Receiver for SetRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRadius(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCylinderMesh::setRings`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.slots().set_rings()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct SetRings<'a>(&'a ::cylinder_mesh::CylinderMesh);
  impl<'a> ::qt_core::connection::Receiver for SetRings<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRings(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCylinderMesh::setVerticesPerPatch`.
  ///
  /// An object of this type can be created from `CylinderMesh` with `object.slots().set_vertices_per_patch()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderMesh` object.
  pub struct SetVerticesPerPatch<'a>(&'a ::cylinder_mesh::CylinderMesh);
  impl<'a> ::qt_core::connection::Receiver for SetVerticesPerPatch<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVerticesPerPatch(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCylinderMesh::setSlices`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_slices(&self) -> SetSlices {
      SetSlices(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCylinderMesh::setLength`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_length(&self) -> SetLength {
      SetLength(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCylinderMesh::setFirstVertex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_first_vertex(&self) -> SetFirstVertex {
      SetFirstVertex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCylinderMesh::setRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_radius(&self) -> SetRadius {
      SetRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCylinderMesh::setRings`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_rings(&self) -> SetRings {
      SetRings(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCylinderMesh::setVerticesPerPatch`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertices_per_patch(&self) -> SetVerticesPerPatch {
      SetVerticesPerPatch(self.0)
    }
  }
  impl ::cylinder_mesh::CylinderMesh {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::cylinder_mesh::CylinderMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::cylinder_mesh::CylinderMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::cylinder_mesh::CylinderMesh as *mut ::cylinder_mesh::CylinderMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::cylinder_mesh::CylinderMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::cylinder_mesh::CylinderMesh)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::cylinder_mesh::CylinderMesh as *mut ::cylinder_mesh::CylinderMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry_renderer::GeometryRenderer> for ::cylinder_mesh::CylinderMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::cylinder_mesh::CylinderMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::cylinder_mesh::CylinderMesh as *mut ::cylinder_mesh::CylinderMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::cylinder_mesh::CylinderMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_QObject_ptr(self as *mut ::cylinder_mesh::CylinderMesh)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_QObject_ptr(self as *const ::cylinder_mesh::CylinderMesh as *mut ::cylinder_mesh::CylinderMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cylinder_mesh::CylinderMesh> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cylinder_mesh::CylinderMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DExtras_QCylinderMesh_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cylinder_mesh::CylinderMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DExtras_QCylinderMesh_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cylinder_mesh::CylinderMesh> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cylinder_mesh::CylinderMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DExtras_QCylinderMesh_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cylinder_mesh::CylinderMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DExtras_QCylinderMesh_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cylinder_mesh::CylinderMesh> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cylinder_mesh::CylinderMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DExtras_QCylinderMesh_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cylinder_mesh::CylinderMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DExtras_QCylinderMesh_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cylinder_mesh::CylinderMesh> for ::qt_3d_render::geometry_renderer::GeometryRenderer {
unsafe fn static_cast_mut(&mut self) -> &mut ::cylinder_mesh::CylinderMesh {
let ffi_result = ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DExtras_QCylinderMesh_ptr_Qt3DRender_QGeometryRenderer(self as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::cylinder_mesh::CylinderMesh {
let ffi_result = ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DExtras_QCylinderMesh_ptr_Qt3DRender_QGeometryRenderer(self as *const ::qt_3d_render::geometry_renderer::GeometryRenderer as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::cylinder_mesh::CylinderMesh {
  type Target = ::qt_3d_render::geometry_renderer::GeometryRenderer;
  fn deref(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::cylinder_mesh::CylinderMesh as *mut ::cylinder_mesh::CylinderMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::cylinder_mesh::CylinderMesh {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::cylinder_mesh::CylinderMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
