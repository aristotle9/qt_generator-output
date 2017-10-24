/// C++ type: <span style='color: green;'>```Qt3DExtras::QCuboidMesh```</span>
#[repr(C)]
pub struct CuboidMesh(u8);

impl CuboidMesh {
  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QCuboidMesh::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_metaObject(self as *const ::cuboid_mesh::CuboidMesh) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QCuboidMesh::QCuboidMesh()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::cuboid_mesh::CuboidMesh> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QCuboidMesh::QCuboidMesh(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::cuboid_mesh::CuboidMesh> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QCuboidMesh::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_qt_metacall(self as *mut ::cuboid_mesh::CuboidMesh,
                                                             arg1 as *const ::qt_core::meta_object::Call,
                                                             arg2,
                                                             arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QCuboidMesh::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_qt_metacast(self as *mut ::cuboid_mesh::CuboidMesh, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidMesh::setXExtent(float xExtent)```</span>
  ///
  ///
  pub fn set_x_extent(&mut self, x_extent: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setXExtent(self as *mut ::cuboid_mesh::CuboidMesh, x_extent) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidMesh::setXYMeshResolution(const QSize& resolution)```</span>
  ///
  ///
  pub fn set_x_y_mesh_resolution(&mut self, resolution: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setXYMeshResolution(self as *mut ::cuboid_mesh::CuboidMesh,
                                                                       resolution as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidMesh::setXZMeshResolution(const QSize& resolution)```</span>
  ///
  ///
  pub fn set_x_z_mesh_resolution(&mut self, resolution: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setXZMeshResolution(self as *mut ::cuboid_mesh::CuboidMesh,
                                                                       resolution as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidMesh::setYExtent(float yExtent)```</span>
  ///
  ///
  pub fn set_y_extent(&mut self, y_extent: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setYExtent(self as *mut ::cuboid_mesh::CuboidMesh, y_extent) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidMesh::setYZMeshResolution(const QSize& resolution)```</span>
  ///
  ///
  pub fn set_y_z_mesh_resolution(&mut self, resolution: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setYZMeshResolution(self as *mut ::cuboid_mesh::CuboidMesh,
                                                                       resolution as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidMesh::setZExtent(float zExtent)```</span>
  ///
  ///
  pub fn set_z_extent(&mut self, z_extent: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_setZExtent(self as *mut ::cuboid_mesh::CuboidMesh, z_extent) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QCuboidMesh::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QCuboidMesh::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QCuboidMesh::xExtent() const```</span>
  ///
  ///
  pub fn x_extent(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_xExtent(self as *const ::cuboid_mesh::CuboidMesh) }
  }

  /// C++ method: <span style='color: green;'>```QSize Qt3DExtras::QCuboidMesh::xyMeshResolution() const```</span>
  ///
  ///
  pub fn xy_mesh_resolution(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_xyMeshResolution_to_output(self as *const ::cuboid_mesh::CuboidMesh, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize Qt3DExtras::QCuboidMesh::xzMeshResolution() const```</span>
  ///
  ///
  pub fn xz_mesh_resolution(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_xzMeshResolution_to_output(self as *const ::cuboid_mesh::CuboidMesh, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QCuboidMesh::yExtent() const```</span>
  ///
  ///
  pub fn y_extent(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_yExtent(self as *const ::cuboid_mesh::CuboidMesh) }
  }

  /// C++ method: <span style='color: green;'>```QSize Qt3DExtras::QCuboidMesh::yzMeshResolution() const```</span>
  ///
  ///
  pub fn yz_mesh_resolution(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_yzMeshResolution_to_output(self as *const ::cuboid_mesh::CuboidMesh, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QCuboidMesh::zExtent() const```</span>
  ///
  ///
  pub fn z_extent(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_zExtent(self as *const ::cuboid_mesh::CuboidMesh) }
  }
}

impl ::cpp_utils::CppDeletable for ::cuboid_mesh::CuboidMesh {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidMesh_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `CuboidMesh`.
  pub struct Signals<'a>(&'a ::cuboid_mesh::CuboidMesh);
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::indexOffsetChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().index_offset_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct IndexOffsetChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::yExtentChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().y_extent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct YExtentChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for YExtentChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2yExtentChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for YExtentChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::xzMeshResolutionChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().xz_mesh_resolution_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct XzMeshResolutionChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for XzMeshResolutionChanged<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2xzMeshResolutionChanged(const QSize&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for XzMeshResolutionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::firstVertexChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().first_vertex_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct FirstVertexChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::vertexCountChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().vertex_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct VertexCountChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::primitiveTypeChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().primitive_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct PrimitiveTypeChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::zExtentChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().z_extent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct ZExtentChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for ZExtentChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2zExtentChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ZExtentChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::firstInstanceChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().first_instance_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct FirstInstanceChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::geometryChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct GeometryChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::verticesPerPatchChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().vertices_per_patch_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct VerticesPerPatchChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::primitiveRestartEnabledChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().primitive_restart_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct PrimitiveRestartEnabledChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::xyMeshResolutionChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().xy_mesh_resolution_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct XyMeshResolutionChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for XyMeshResolutionChanged<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2xyMeshResolutionChanged(const QSize&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for XyMeshResolutionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::xExtentChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().x_extent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct XExtentChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for XExtentChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2xExtentChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for XExtentChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::restartIndexValueChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().restart_index_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct RestartIndexValueChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::instanceCountChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().instance_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct InstanceCountChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidMesh::yzMeshResolutionChanged`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.signals().yz_mesh_resolution_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct YzMeshResolutionChanged<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for YzMeshResolutionChanged<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2yzMeshResolutionChanged(const QSize&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for YzMeshResolutionChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::indexOffsetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn index_offset_changed(&self) -> IndexOffsetChanged {
      IndexOffsetChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::yExtentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn y_extent_changed(&self) -> YExtentChanged {
      YExtentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::xzMeshResolutionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn xz_mesh_resolution_changed(&self) -> XzMeshResolutionChanged {
      XzMeshResolutionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::firstVertexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_vertex_changed(&self) -> FirstVertexChanged {
      FirstVertexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::vertexCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertex_count_changed(&self) -> VertexCountChanged {
      VertexCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::primitiveTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_type_changed(&self) -> PrimitiveTypeChanged {
      PrimitiveTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::zExtentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn z_extent_changed(&self) -> ZExtentChanged {
      ZExtentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::firstInstanceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_instance_changed(&self) -> FirstInstanceChanged {
      FirstInstanceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::geometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_changed(&self) -> GeometryChanged {
      GeometryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::verticesPerPatchChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertices_per_patch_changed(&self) -> VerticesPerPatchChanged {
      VerticesPerPatchChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::primitiveRestartEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_restart_enabled_changed(&self) -> PrimitiveRestartEnabledChanged {
      PrimitiveRestartEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::xyMeshResolutionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn xy_mesh_resolution_changed(&self) -> XyMeshResolutionChanged {
      XyMeshResolutionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::xExtentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn x_extent_changed(&self) -> XExtentChanged {
      XExtentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::restartIndexValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn restart_index_value_changed(&self) -> RestartIndexValueChanged {
      RestartIndexValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::instanceCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn instance_count_changed(&self) -> InstanceCountChanged {
      InstanceCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidMesh::yzMeshResolutionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn yz_mesh_resolution_changed(&self) -> YzMeshResolutionChanged {
      YzMeshResolutionChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `CuboidMesh`.
  pub struct Slots<'a>(&'a ::cuboid_mesh::CuboidMesh);
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidMesh::setYExtent`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.slots().set_y_extent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct SetYExtent<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for SetYExtent<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setYExtent(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidMesh::setXYMeshResolution`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.slots().set_x_y_mesh_resolution()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct SetXYMeshResolution<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for SetXYMeshResolution<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setXYMeshResolution(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidMesh::setXExtent`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.slots().set_x_extent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct SetXExtent<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for SetXExtent<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setXExtent(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidMesh::setXZMeshResolution`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.slots().set_x_z_mesh_resolution()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct SetXZMeshResolution<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for SetXZMeshResolution<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setXZMeshResolution(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidMesh::setZExtent`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.slots().set_z_extent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct SetZExtent<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for SetZExtent<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setZExtent(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidMesh::setVerticesPerPatch`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.slots().set_vertices_per_patch()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct SetVerticesPerPatch<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for SetVerticesPerPatch<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVerticesPerPatch(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidMesh::setYZMeshResolution`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.slots().set_y_z_mesh_resolution()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct SetYZMeshResolution<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for SetYZMeshResolution<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setYZMeshResolution(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidMesh::setFirstVertex`.
  ///
  /// An object of this type can be created from `CuboidMesh` with `object.slots().set_first_vertex()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidMesh` object.
  pub struct SetFirstVertex<'a>(&'a ::cuboid_mesh::CuboidMesh);
  impl<'a> ::qt_core::connection::Receiver for SetFirstVertex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFirstVertex(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidMesh::setYExtent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_y_extent(&self) -> SetYExtent {
      SetYExtent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidMesh::setXYMeshResolution`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_x_y_mesh_resolution(&self) -> SetXYMeshResolution {
      SetXYMeshResolution(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidMesh::setXExtent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_x_extent(&self) -> SetXExtent {
      SetXExtent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidMesh::setXZMeshResolution`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_x_z_mesh_resolution(&self) -> SetXZMeshResolution {
      SetXZMeshResolution(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidMesh::setZExtent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_z_extent(&self) -> SetZExtent {
      SetZExtent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidMesh::setVerticesPerPatch`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertices_per_patch(&self) -> SetVerticesPerPatch {
      SetVerticesPerPatch(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidMesh::setYZMeshResolution`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_y_z_mesh_resolution(&self) -> SetYZMeshResolution {
      SetYZMeshResolution(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidMesh::setFirstVertex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_first_vertex(&self) -> SetFirstVertex {
      SetFirstVertex(self.0)
    }
  }
  impl ::cuboid_mesh::CuboidMesh {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::cuboid_mesh::CuboidMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::cuboid_mesh::CuboidMesh)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::cuboid_mesh::CuboidMesh as *mut ::cuboid_mesh::CuboidMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::cuboid_mesh::CuboidMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::cuboid_mesh::CuboidMesh)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::cuboid_mesh::CuboidMesh as *mut ::cuboid_mesh::CuboidMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry_renderer::GeometryRenderer> for ::cuboid_mesh::CuboidMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::cuboid_mesh::CuboidMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::cuboid_mesh::CuboidMesh as *mut ::cuboid_mesh::CuboidMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::cuboid_mesh::CuboidMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_QObject_ptr(self as *mut ::cuboid_mesh::CuboidMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_QObject_ptr(self as *const ::cuboid_mesh::CuboidMesh as *mut ::cuboid_mesh::CuboidMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cuboid_mesh::CuboidMesh> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cuboid_mesh::CuboidMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cuboid_mesh::CuboidMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cuboid_mesh::CuboidMesh> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cuboid_mesh::CuboidMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cuboid_mesh::CuboidMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cuboid_mesh::CuboidMesh> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cuboid_mesh::CuboidMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cuboid_mesh::CuboidMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cuboid_mesh::CuboidMesh> for ::qt_3d_render::geometry_renderer::GeometryRenderer {
unsafe fn static_cast_mut(&mut self) -> &mut ::cuboid_mesh::CuboidMesh {
let ffi_result = ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_Qt3DRender_QGeometryRenderer(self as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::cuboid_mesh::CuboidMesh {
let ffi_result = ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DExtras_QCuboidMesh_ptr_Qt3DRender_QGeometryRenderer(self as *const ::qt_3d_render::geometry_renderer::GeometryRenderer as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::cuboid_mesh::CuboidMesh {
  type Target = ::qt_3d_render::geometry_renderer::GeometryRenderer;
  fn deref(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::cuboid_mesh::CuboidMesh as *mut ::cuboid_mesh::CuboidMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::cuboid_mesh::CuboidMesh {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::cuboid_mesh::CuboidMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
