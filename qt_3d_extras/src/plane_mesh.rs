/// C++ type: <span style='color: green;'>```Qt3DExtras::QPlaneMesh```</span>
#[repr(C)]
pub struct PlaneMesh(u8);

impl PlaneMesh {
  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QPlaneMesh::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_height(self as *const ::plane_mesh::PlaneMesh) }
  }

  /// C++ method: <span style='color: green;'>```QSize Qt3DExtras::QPlaneMesh::meshResolution() const```</span>
  ///
  ///
  pub fn mesh_resolution(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_meshResolution_to_output(self as *const ::plane_mesh::PlaneMesh,
                                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QPlaneMesh::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_metaObject(self as *const ::plane_mesh::PlaneMesh) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DExtras::QPlaneMesh::mirrored() const```</span>
  ///
  ///
  pub fn mirrored(&self) -> bool {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_mirrored(self as *const ::plane_mesh::PlaneMesh) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QPlaneMesh::QPlaneMesh()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::plane_mesh::PlaneMesh> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QPlaneMesh::QPlaneMesh(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::plane_mesh::PlaneMesh> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QPlaneMesh::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_qt_metacall(self as *mut ::plane_mesh::PlaneMesh,
                                                            arg1 as *const ::qt_core::meta_object::Call,
                                                            arg2,
                                                            arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QPlaneMesh::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_qt_metacast(self as *mut ::plane_mesh::PlaneMesh, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPlaneMesh::setHeight(float height)```</span>
  ///
  ///
  pub fn set_height(&mut self, height: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_setHeight(self as *mut ::plane_mesh::PlaneMesh, height) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPlaneMesh::setMeshResolution(const QSize& resolution)```</span>
  ///
  ///
  pub fn set_mesh_resolution(&mut self, resolution: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_setMeshResolution(self as *mut ::plane_mesh::PlaneMesh,
                                                                    resolution as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPlaneMesh::setMirrored(bool mirrored)```</span>
  ///
  ///
  pub fn set_mirrored(&mut self, mirrored: bool) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_setMirrored(self as *mut ::plane_mesh::PlaneMesh, mirrored) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPlaneMesh::setWidth(float width)```</span>
  ///
  ///
  pub fn set_width(&mut self, width: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_setWidth(self as *mut ::plane_mesh::PlaneMesh, width) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QPlaneMesh::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QPlaneMesh::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QPlaneMesh::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_width(self as *const ::plane_mesh::PlaneMesh) }
  }
}

impl ::cpp_utils::CppDeletable for ::plane_mesh::PlaneMesh {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneMesh_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PlaneMesh`.
  pub struct Signals<'a>(&'a ::plane_mesh::PlaneMesh);
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::vertexCountChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().vertex_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct VertexCountChanged<'a>(&'a ::plane_mesh::PlaneMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::restartIndexValueChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().restart_index_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct RestartIndexValueChanged<'a>(&'a ::plane_mesh::PlaneMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::widthChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct WidthChanged<'a>(&'a ::plane_mesh::PlaneMesh);
  impl<'a> ::qt_core::connection::Receiver for WidthChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2widthChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WidthChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::firstInstanceChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().first_instance_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct FirstInstanceChanged<'a>(&'a ::plane_mesh::PlaneMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::mirroredChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().mirrored_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct MirroredChanged<'a>(&'a ::plane_mesh::PlaneMesh);
  impl<'a> ::qt_core::connection::Receiver for MirroredChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2mirroredChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MirroredChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::firstVertexChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().first_vertex_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct FirstVertexChanged<'a>(&'a ::plane_mesh::PlaneMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::geometryChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct GeometryChanged<'a>(&'a ::plane_mesh::PlaneMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::indexOffsetChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().index_offset_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct IndexOffsetChanged<'a>(&'a ::plane_mesh::PlaneMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::primitiveTypeChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().primitive_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct PrimitiveTypeChanged<'a>(&'a ::plane_mesh::PlaneMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::meshResolutionChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().mesh_resolution_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct MeshResolutionChanged<'a>(&'a ::plane_mesh::PlaneMesh);
  impl<'a> ::qt_core::connection::Receiver for MeshResolutionChanged<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2meshResolutionChanged(const QSize&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MeshResolutionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::primitiveRestartEnabledChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().primitive_restart_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct PrimitiveRestartEnabledChanged<'a>(&'a ::plane_mesh::PlaneMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::heightChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct HeightChanged<'a>(&'a ::plane_mesh::PlaneMesh);
  impl<'a> ::qt_core::connection::Receiver for HeightChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2heightChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HeightChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::instanceCountChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().instance_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct InstanceCountChanged<'a>(&'a ::plane_mesh::PlaneMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneMesh::verticesPerPatchChanged`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.signals().vertices_per_patch_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct VerticesPerPatchChanged<'a>(&'a ::plane_mesh::PlaneMesh);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::vertexCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertex_count_changed(&self) -> VertexCountChanged {
      VertexCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::restartIndexValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn restart_index_value_changed(&self) -> RestartIndexValueChanged {
      RestartIndexValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::firstInstanceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_instance_changed(&self) -> FirstInstanceChanged {
      FirstInstanceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::mirroredChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mirrored_changed(&self) -> MirroredChanged {
      MirroredChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::firstVertexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_vertex_changed(&self) -> FirstVertexChanged {
      FirstVertexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::geometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_changed(&self) -> GeometryChanged {
      GeometryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::indexOffsetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn index_offset_changed(&self) -> IndexOffsetChanged {
      IndexOffsetChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::primitiveTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_type_changed(&self) -> PrimitiveTypeChanged {
      PrimitiveTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::meshResolutionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mesh_resolution_changed(&self) -> MeshResolutionChanged {
      MeshResolutionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::primitiveRestartEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_restart_enabled_changed(&self) -> PrimitiveRestartEnabledChanged {
      PrimitiveRestartEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::instanceCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn instance_count_changed(&self) -> InstanceCountChanged {
      InstanceCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneMesh::verticesPerPatchChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertices_per_patch_changed(&self) -> VerticesPerPatchChanged {
      VerticesPerPatchChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PlaneMesh`.
  pub struct Slots<'a>(&'a ::plane_mesh::PlaneMesh);
  /// Represents a built-in Qt slot `Qt3DExtras::QPlaneMesh::setMirrored`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.slots().set_mirrored()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct SetMirrored<'a>(&'a ::plane_mesh::PlaneMesh);
  impl<'a> ::qt_core::connection::Receiver for SetMirrored<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMirrored(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPlaneMesh::setHeight`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.slots().set_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct SetHeight<'a>(&'a ::plane_mesh::PlaneMesh);
  impl<'a> ::qt_core::connection::Receiver for SetHeight<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHeight(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPlaneMesh::setVerticesPerPatch`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.slots().set_vertices_per_patch()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct SetVerticesPerPatch<'a>(&'a ::plane_mesh::PlaneMesh);
  impl<'a> ::qt_core::connection::Receiver for SetVerticesPerPatch<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVerticesPerPatch(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPlaneMesh::setFirstVertex`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.slots().set_first_vertex()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct SetFirstVertex<'a>(&'a ::plane_mesh::PlaneMesh);
  impl<'a> ::qt_core::connection::Receiver for SetFirstVertex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFirstVertex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPlaneMesh::setWidth`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.slots().set_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct SetWidth<'a>(&'a ::plane_mesh::PlaneMesh);
  impl<'a> ::qt_core::connection::Receiver for SetWidth<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWidth(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPlaneMesh::setMeshResolution`.
  ///
  /// An object of this type can be created from `PlaneMesh` with `object.slots().set_mesh_resolution()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneMesh` object.
  pub struct SetMeshResolution<'a>(&'a ::plane_mesh::PlaneMesh);
  impl<'a> ::qt_core::connection::Receiver for SetMeshResolution<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMeshResolution(const QSize&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPlaneMesh::setMirrored`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_mirrored(&self) -> SetMirrored {
      SetMirrored(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPlaneMesh::setHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_height(&self) -> SetHeight {
      SetHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPlaneMesh::setVerticesPerPatch`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertices_per_patch(&self) -> SetVerticesPerPatch {
      SetVerticesPerPatch(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPlaneMesh::setFirstVertex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_first_vertex(&self) -> SetFirstVertex {
      SetFirstVertex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPlaneMesh::setWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_width(&self) -> SetWidth {
      SetWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPlaneMesh::setMeshResolution`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_mesh_resolution(&self) -> SetMeshResolution {
      SetMeshResolution(self.0)
    }
  }
  impl ::plane_mesh::PlaneMesh {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::plane_mesh::PlaneMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::plane_mesh::PlaneMesh)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::plane_mesh::PlaneMesh as *mut ::plane_mesh::PlaneMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::plane_mesh::PlaneMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::plane_mesh::PlaneMesh)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::plane_mesh::PlaneMesh as *mut ::plane_mesh::PlaneMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry_renderer::GeometryRenderer> for ::plane_mesh::PlaneMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::plane_mesh::PlaneMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::plane_mesh::PlaneMesh as *mut ::plane_mesh::PlaneMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::plane_mesh::PlaneMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_QObject_ptr(self as *mut ::plane_mesh::PlaneMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_QObject_ptr(self as *const ::plane_mesh::PlaneMesh as *mut ::plane_mesh::PlaneMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plane_mesh::PlaneMesh> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plane_mesh::PlaneMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plane_mesh::PlaneMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plane_mesh::PlaneMesh> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plane_mesh::PlaneMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plane_mesh::PlaneMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plane_mesh::PlaneMesh> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plane_mesh::PlaneMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plane_mesh::PlaneMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plane_mesh::PlaneMesh> for ::qt_3d_render::geometry_renderer::GeometryRenderer {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plane_mesh::PlaneMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_Qt3DRender_QGeometryRenderer(self as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plane_mesh::PlaneMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DExtras_QPlaneMesh_ptr_Qt3DRender_QGeometryRenderer(self as *const ::qt_3d_render::geometry_renderer::GeometryRenderer as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::plane_mesh::PlaneMesh {
  type Target = ::qt_3d_render::geometry_renderer::GeometryRenderer;
  fn deref(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::plane_mesh::PlaneMesh as *mut ::plane_mesh::PlaneMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::plane_mesh::PlaneMesh {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::plane_mesh::PlaneMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
