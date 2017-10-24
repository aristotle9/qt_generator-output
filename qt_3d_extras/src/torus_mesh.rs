/// C++ type: <span style='color: green;'>```Qt3DExtras::QTorusMesh```</span>
#[repr(C)]
pub struct TorusMesh(u8);

impl TorusMesh {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QTorusMesh::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_metaObject(self as *const ::torus_mesh::TorusMesh) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QTorusMesh::minorRadius() const```</span>
  ///
  ///
  pub fn minor_radius(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_minorRadius(self as *const ::torus_mesh::TorusMesh) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QTorusMesh::QTorusMesh()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::torus_mesh::TorusMesh> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QTorusMesh::QTorusMesh(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::torus_mesh::TorusMesh> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QTorusMesh::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_qt_metacall(self as *mut ::torus_mesh::TorusMesh,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QTorusMesh::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_qt_metacast(self as *mut ::torus_mesh::TorusMesh, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QTorusMesh::radius() const```</span>
  ///
  ///
  pub fn radius(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_radius(self as *const ::torus_mesh::TorusMesh) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QTorusMesh::rings() const```</span>
  ///
  ///
  pub fn rings(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_rings(self as *const ::torus_mesh::TorusMesh) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTorusMesh::setMinorRadius(float minorRadius)```</span>
  ///
  ///
  pub fn set_minor_radius(&mut self, minor_radius: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_setMinorRadius(self as *mut ::torus_mesh::TorusMesh, minor_radius)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTorusMesh::setRadius(float radius)```</span>
  ///
  ///
  pub fn set_radius(&mut self, radius: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_setRadius(self as *mut ::torus_mesh::TorusMesh, radius) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTorusMesh::setRings(int rings)```</span>
  ///
  ///
  pub fn set_rings(&mut self, rings: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_setRings(self as *mut ::torus_mesh::TorusMesh, rings) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTorusMesh::setSlices(int slices)```</span>
  ///
  ///
  pub fn set_slices(&mut self, slices: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_setSlices(self as *mut ::torus_mesh::TorusMesh, slices) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QTorusMesh::slices() const```</span>
  ///
  ///
  pub fn slices(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_slices(self as *const ::torus_mesh::TorusMesh) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QTorusMesh::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QTorusMesh::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::torus_mesh::TorusMesh {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusMesh_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TorusMesh`.
  pub struct Signals<'a>(&'a ::torus_mesh::TorusMesh);
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::firstInstanceChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().first_instance_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct FirstInstanceChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::restartIndexValueChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().restart_index_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct RestartIndexValueChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::minorRadiusChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().minor_radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct MinorRadiusChanged<'a>(&'a ::torus_mesh::TorusMesh);
  impl<'a> ::qt_core::connection::Receiver for MinorRadiusChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2minorRadiusChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MinorRadiusChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::radiusChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct RadiusChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::indexOffsetChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().index_offset_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct IndexOffsetChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::slicesChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().slices_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct SlicesChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::verticesPerPatchChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().vertices_per_patch_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct VerticesPerPatchChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::primitiveTypeChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().primitive_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct PrimitiveTypeChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::vertexCountChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().vertex_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct VertexCountChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::ringsChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().rings_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct RingsChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::geometryChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct GeometryChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::primitiveRestartEnabledChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().primitive_restart_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct PrimitiveRestartEnabledChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::firstVertexChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().first_vertex_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct FirstVertexChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusMesh::instanceCountChanged`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.signals().instance_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct InstanceCountChanged<'a>(&'a ::torus_mesh::TorusMesh);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::firstInstanceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_instance_changed(&self) -> FirstInstanceChanged {
      FirstInstanceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::restartIndexValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn restart_index_value_changed(&self) -> RestartIndexValueChanged {
      RestartIndexValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::minorRadiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn minor_radius_changed(&self) -> MinorRadiusChanged {
      MinorRadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::radiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn radius_changed(&self) -> RadiusChanged {
      RadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::indexOffsetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn index_offset_changed(&self) -> IndexOffsetChanged {
      IndexOffsetChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::slicesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slices_changed(&self) -> SlicesChanged {
      SlicesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::verticesPerPatchChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertices_per_patch_changed(&self) -> VerticesPerPatchChanged {
      VerticesPerPatchChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::primitiveTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_type_changed(&self) -> PrimitiveTypeChanged {
      PrimitiveTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::vertexCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertex_count_changed(&self) -> VertexCountChanged {
      VertexCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::ringsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rings_changed(&self) -> RingsChanged {
      RingsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::geometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_changed(&self) -> GeometryChanged {
      GeometryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::primitiveRestartEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_restart_enabled_changed(&self) -> PrimitiveRestartEnabledChanged {
      PrimitiveRestartEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::firstVertexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_vertex_changed(&self) -> FirstVertexChanged {
      FirstVertexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusMesh::instanceCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn instance_count_changed(&self) -> InstanceCountChanged {
      InstanceCountChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TorusMesh`.
  pub struct Slots<'a>(&'a ::torus_mesh::TorusMesh);
  /// Represents a built-in Qt slot `Qt3DExtras::QTorusMesh::setVerticesPerPatch`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.slots().set_vertices_per_patch()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct SetVerticesPerPatch<'a>(&'a ::torus_mesh::TorusMesh);
  impl<'a> ::qt_core::connection::Receiver for SetVerticesPerPatch<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVerticesPerPatch(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTorusMesh::setFirstVertex`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.slots().set_first_vertex()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct SetFirstVertex<'a>(&'a ::torus_mesh::TorusMesh);
  impl<'a> ::qt_core::connection::Receiver for SetFirstVertex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFirstVertex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTorusMesh::setRadius`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.slots().set_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct SetRadius<'a>(&'a ::torus_mesh::TorusMesh);
  impl<'a> ::qt_core::connection::Receiver for SetRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRadius(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTorusMesh::setSlices`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.slots().set_slices()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct SetSlices<'a>(&'a ::torus_mesh::TorusMesh);
  impl<'a> ::qt_core::connection::Receiver for SetSlices<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSlices(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTorusMesh::setMinorRadius`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.slots().set_minor_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct SetMinorRadius<'a>(&'a ::torus_mesh::TorusMesh);
  impl<'a> ::qt_core::connection::Receiver for SetMinorRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinorRadius(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTorusMesh::setRings`.
  ///
  /// An object of this type can be created from `TorusMesh` with `object.slots().set_rings()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusMesh` object.
  pub struct SetRings<'a>(&'a ::torus_mesh::TorusMesh);
  impl<'a> ::qt_core::connection::Receiver for SetRings<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRings(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTorusMesh::setVerticesPerPatch`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertices_per_patch(&self) -> SetVerticesPerPatch {
      SetVerticesPerPatch(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTorusMesh::setFirstVertex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_first_vertex(&self) -> SetFirstVertex {
      SetFirstVertex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTorusMesh::setRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_radius(&self) -> SetRadius {
      SetRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTorusMesh::setSlices`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_slices(&self) -> SetSlices {
      SetSlices(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTorusMesh::setMinorRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minor_radius(&self) -> SetMinorRadius {
      SetMinorRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTorusMesh::setRings`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_rings(&self) -> SetRings {
      SetRings(self.0)
    }
  }
  impl ::torus_mesh::TorusMesh {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::torus_mesh::TorusMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::torus_mesh::TorusMesh)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::torus_mesh::TorusMesh as *mut ::torus_mesh::TorusMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::torus_mesh::TorusMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::torus_mesh::TorusMesh)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::torus_mesh::TorusMesh as *mut ::torus_mesh::TorusMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry_renderer::GeometryRenderer> for ::torus_mesh::TorusMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::torus_mesh::TorusMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::torus_mesh::TorusMesh as *mut ::torus_mesh::TorusMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::torus_mesh::TorusMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_QObject_ptr(self as *mut ::torus_mesh::TorusMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_QObject_ptr(self as *const ::torus_mesh::TorusMesh as *mut ::torus_mesh::TorusMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::torus_mesh::TorusMesh> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::torus_mesh::TorusMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::torus_mesh::TorusMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::torus_mesh::TorusMesh> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::torus_mesh::TorusMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::torus_mesh::TorusMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::torus_mesh::TorusMesh> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::torus_mesh::TorusMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::torus_mesh::TorusMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::torus_mesh::TorusMesh> for ::qt_3d_render::geometry_renderer::GeometryRenderer {
  unsafe fn static_cast_mut(&mut self) -> &mut ::torus_mesh::TorusMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_Qt3DRender_QGeometryRenderer(self as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::torus_mesh::TorusMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DExtras_QTorusMesh_ptr_Qt3DRender_QGeometryRenderer(self as *const ::qt_3d_render::geometry_renderer::GeometryRenderer as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::torus_mesh::TorusMesh {
  type Target = ::qt_3d_render::geometry_renderer::GeometryRenderer;
  fn deref(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::torus_mesh::TorusMesh as *mut ::torus_mesh::TorusMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::torus_mesh::TorusMesh {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::torus_mesh::TorusMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
