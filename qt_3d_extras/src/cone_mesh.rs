/// C++ type: <span style='color: green;'>```Qt3DExtras::QConeMesh```</span>
#[repr(C)]
pub struct ConeMesh(u8);

impl ConeMesh {
  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QConeMesh::bottomRadius() const```</span>
  ///
  ///
  pub fn bottom_radius(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_bottomRadius(self as *const ::cone_mesh::ConeMesh) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DExtras::QConeMesh::hasBottomEndcap() const```</span>
  ///
  ///
  pub fn has_bottom_endcap(&self) -> bool {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_hasBottomEndcap(self as *const ::cone_mesh::ConeMesh) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DExtras::QConeMesh::hasTopEndcap() const```</span>
  ///
  ///
  pub fn has_top_endcap(&self) -> bool {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_hasTopEndcap(self as *const ::cone_mesh::ConeMesh) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QConeMesh::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_length(self as *const ::cone_mesh::ConeMesh) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QConeMesh::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_metaObject(self as *const ::cone_mesh::ConeMesh) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QConeMesh::QConeMesh()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::cone_mesh::ConeMesh> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QConeMesh::QConeMesh(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::cone_mesh::ConeMesh> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QConeMesh::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_qt_metacall(self as *mut ::cone_mesh::ConeMesh,
                                                           arg1 as *const ::qt_core::meta_object::Call,
                                                           arg2,
                                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QConeMesh::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_qt_metacast(self as *mut ::cone_mesh::ConeMesh, arg1)
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QConeMesh::rings() const```</span>
  ///
  ///
  pub fn rings(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_rings(self as *const ::cone_mesh::ConeMesh) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeMesh::setBottomRadius(float bottomRadius)```</span>
  ///
  ///
  pub fn set_bottom_radius(&mut self, bottom_radius: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_setBottomRadius(self as *mut ::cone_mesh::ConeMesh, bottom_radius)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeMesh::setHasBottomEndcap(bool hasBottomEndcap)```</span>
  ///
  ///
  pub fn set_has_bottom_endcap(&mut self, has_bottom_endcap: bool) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_setHasBottomEndcap(self as *mut ::cone_mesh::ConeMesh,
                                                                    has_bottom_endcap)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeMesh::setHasTopEndcap(bool hasTopEndcap)```</span>
  ///
  ///
  pub fn set_has_top_endcap(&mut self, has_top_endcap: bool) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_setHasTopEndcap(self as *mut ::cone_mesh::ConeMesh, has_top_endcap)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeMesh::setLength(float length)```</span>
  ///
  ///
  pub fn set_length(&mut self, length: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_setLength(self as *mut ::cone_mesh::ConeMesh, length) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeMesh::setRings(int rings)```</span>
  ///
  ///
  pub fn set_rings(&mut self, rings: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_setRings(self as *mut ::cone_mesh::ConeMesh, rings) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeMesh::setSlices(int slices)```</span>
  ///
  ///
  pub fn set_slices(&mut self, slices: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_setSlices(self as *mut ::cone_mesh::ConeMesh, slices) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeMesh::setTopRadius(float topRadius)```</span>
  ///
  ///
  pub fn set_top_radius(&mut self, top_radius: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_setTopRadius(self as *mut ::cone_mesh::ConeMesh, top_radius) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QConeMesh::slices() const```</span>
  ///
  ///
  pub fn slices(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_slices(self as *const ::cone_mesh::ConeMesh) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QConeMesh::topRadius() const```</span>
  ///
  ///
  pub fn top_radius(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_topRadius(self as *const ::cone_mesh::ConeMesh) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QConeMesh::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QConeMesh::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::cone_mesh::ConeMesh {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QConeMesh_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ConeMesh`.
  pub struct Signals<'a>(&'a ::cone_mesh::ConeMesh);
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::restartIndexValueChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().restart_index_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct RestartIndexValueChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::hasBottomEndcapChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().has_bottom_endcap_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct HasBottomEndcapChanged<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for HasBottomEndcapChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2hasBottomEndcapChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HasBottomEndcapChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::lengthChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().length_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct LengthChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::geometryChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct GeometryChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::hasTopEndcapChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().has_top_endcap_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct HasTopEndcapChanged<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for HasTopEndcapChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2hasTopEndcapChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HasTopEndcapChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::firstInstanceChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().first_instance_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct FirstInstanceChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::ringsChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().rings_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct RingsChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::indexOffsetChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().index_offset_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct IndexOffsetChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::firstVertexChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().first_vertex_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct FirstVertexChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::topRadiusChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().top_radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct TopRadiusChanged<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for TopRadiusChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2topRadiusChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TopRadiusChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::vertexCountChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().vertex_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct VertexCountChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::instanceCountChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().instance_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct InstanceCountChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::primitiveTypeChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().primitive_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct PrimitiveTypeChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::primitiveRestartEnabledChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().primitive_restart_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct PrimitiveRestartEnabledChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::bottomRadiusChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().bottom_radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct BottomRadiusChanged<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for BottomRadiusChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2bottomRadiusChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BottomRadiusChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::verticesPerPatchChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().vertices_per_patch_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct VerticesPerPatchChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeMesh::slicesChanged`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.signals().slices_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct SlicesChanged<'a>(&'a ::cone_mesh::ConeMesh);
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
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::restartIndexValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn restart_index_value_changed(&self) -> RestartIndexValueChanged {
      RestartIndexValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::hasBottomEndcapChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn has_bottom_endcap_changed(&self) -> HasBottomEndcapChanged {
      HasBottomEndcapChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::lengthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn length_changed(&self) -> LengthChanged {
      LengthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::geometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_changed(&self) -> GeometryChanged {
      GeometryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::hasTopEndcapChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn has_top_endcap_changed(&self) -> HasTopEndcapChanged {
      HasTopEndcapChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::firstInstanceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_instance_changed(&self) -> FirstInstanceChanged {
      FirstInstanceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::ringsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rings_changed(&self) -> RingsChanged {
      RingsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::indexOffsetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn index_offset_changed(&self) -> IndexOffsetChanged {
      IndexOffsetChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::firstVertexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_vertex_changed(&self) -> FirstVertexChanged {
      FirstVertexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::topRadiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn top_radius_changed(&self) -> TopRadiusChanged {
      TopRadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::vertexCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertex_count_changed(&self) -> VertexCountChanged {
      VertexCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::instanceCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn instance_count_changed(&self) -> InstanceCountChanged {
      InstanceCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::primitiveTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_type_changed(&self) -> PrimitiveTypeChanged {
      PrimitiveTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::primitiveRestartEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_restart_enabled_changed(&self) -> PrimitiveRestartEnabledChanged {
      PrimitiveRestartEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::bottomRadiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bottom_radius_changed(&self) -> BottomRadiusChanged {
      BottomRadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::verticesPerPatchChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertices_per_patch_changed(&self) -> VerticesPerPatchChanged {
      VerticesPerPatchChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeMesh::slicesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slices_changed(&self) -> SlicesChanged {
      SlicesChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ConeMesh`.
  pub struct Slots<'a>(&'a ::cone_mesh::ConeMesh);
  /// Represents a built-in Qt slot `Qt3DExtras::QConeMesh::setTopRadius`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.slots().set_top_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct SetTopRadius<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for SetTopRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTopRadius(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeMesh::setLength`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.slots().set_length()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct SetLength<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for SetLength<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLength(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeMesh::setBottomRadius`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.slots().set_bottom_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct SetBottomRadius<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for SetBottomRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBottomRadius(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeMesh::setRings`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.slots().set_rings()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct SetRings<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for SetRings<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRings(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeMesh::setSlices`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.slots().set_slices()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct SetSlices<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for SetSlices<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSlices(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeMesh::setHasBottomEndcap`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.slots().set_has_bottom_endcap()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct SetHasBottomEndcap<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for SetHasBottomEndcap<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHasBottomEndcap(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeMesh::setFirstVertex`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.slots().set_first_vertex()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct SetFirstVertex<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for SetFirstVertex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFirstVertex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeMesh::setHasTopEndcap`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.slots().set_has_top_endcap()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct SetHasTopEndcap<'a>(&'a ::cone_mesh::ConeMesh);
  impl<'a> ::qt_core::connection::Receiver for SetHasTopEndcap<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHasTopEndcap(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeMesh::setVerticesPerPatch`.
  ///
  /// An object of this type can be created from `ConeMesh` with `object.slots().set_vertices_per_patch()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeMesh` object.
  pub struct SetVerticesPerPatch<'a>(&'a ::cone_mesh::ConeMesh);
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
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeMesh::setTopRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_top_radius(&self) -> SetTopRadius {
      SetTopRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeMesh::setLength`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_length(&self) -> SetLength {
      SetLength(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeMesh::setBottomRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bottom_radius(&self) -> SetBottomRadius {
      SetBottomRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeMesh::setRings`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_rings(&self) -> SetRings {
      SetRings(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeMesh::setSlices`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_slices(&self) -> SetSlices {
      SetSlices(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeMesh::setHasBottomEndcap`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_has_bottom_endcap(&self) -> SetHasBottomEndcap {
      SetHasBottomEndcap(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeMesh::setFirstVertex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_first_vertex(&self) -> SetFirstVertex {
      SetFirstVertex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeMesh::setHasTopEndcap`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_has_top_endcap(&self) -> SetHasTopEndcap {
      SetHasTopEndcap(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeMesh::setVerticesPerPatch`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertices_per_patch(&self) -> SetVerticesPerPatch {
      SetVerticesPerPatch(self.0)
    }
  }
  impl ::cone_mesh::ConeMesh {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::cone_mesh::ConeMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::cone_mesh::ConeMesh)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::cone_mesh::ConeMesh as *mut ::cone_mesh::ConeMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::cone_mesh::ConeMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::cone_mesh::ConeMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::cone_mesh::ConeMesh as *mut ::cone_mesh::ConeMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry_renderer::GeometryRenderer> for ::cone_mesh::ConeMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::cone_mesh::ConeMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::cone_mesh::ConeMesh as *mut ::cone_mesh::ConeMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::cone_mesh::ConeMesh {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_QObject_ptr(self as *mut ::cone_mesh::ConeMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_QObject_ptr(self as *const ::cone_mesh::ConeMesh as *mut ::cone_mesh::ConeMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cone_mesh::ConeMesh> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cone_mesh::ConeMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cone_mesh::ConeMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cone_mesh::ConeMesh> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cone_mesh::ConeMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cone_mesh::ConeMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cone_mesh::ConeMesh> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cone_mesh::ConeMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cone_mesh::ConeMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cone_mesh::ConeMesh> for ::qt_3d_render::geometry_renderer::GeometryRenderer {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cone_mesh::ConeMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_Qt3DRender_QGeometryRenderer(self as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cone_mesh::ConeMesh {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DExtras_QConeMesh_ptr_Qt3DRender_QGeometryRenderer(self as *const ::qt_3d_render::geometry_renderer::GeometryRenderer as *mut ::qt_3d_render::geometry_renderer::GeometryRenderer);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::cone_mesh::ConeMesh {
  type Target = ::qt_3d_render::geometry_renderer::GeometryRenderer;
  fn deref(&self) -> &::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *const ::cone_mesh::ConeMesh as *mut ::cone_mesh::ConeMesh) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::cone_mesh::ConeMesh {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry_renderer::GeometryRenderer {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeMesh_G_static_cast_Qt3DRender_QGeometryRenderer_ptr(self as *mut ::cone_mesh::ConeMesh) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
