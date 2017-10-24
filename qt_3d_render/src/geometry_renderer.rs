/// C++ type: <span style='color: green;'>```Qt3DRender::QGeometryRenderer```</span>
#[repr(C)]
pub struct GeometryRenderer(u8);

impl GeometryRenderer {
  /// C++ method: <span style='color: green;'>```int Qt3DRender::QGeometryRenderer::firstInstance() const```</span>
  ///
  ///
  pub fn first_instance(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_firstInstance(self as *const ::geometry_renderer::GeometryRenderer) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QGeometryRenderer::firstVertex() const```</span>
  ///
  ///
  pub fn first_vertex(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_firstVertex(self as *const ::geometry_renderer::GeometryRenderer) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QGeometry* Qt3DRender::QGeometryRenderer::geometry() const```</span>
  ///
  ///
  pub fn geometry(&self) -> *mut ::geometry::Geometry {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_geometry(self as *const ::geometry_renderer::GeometryRenderer)
    }
  }

  /// C++ method: <span style='color: green;'>```QSharedPointer<Qt3DRender::QGeometryFactory> Qt3DRender::QGeometryRenderer::geometryFactory() const```</span>
  ///
  ///
  pub fn geometry_factory(&self) -> ::shared_pointer::SharedPointerGeometryFactory {
    {
      let mut object: ::shared_pointer::SharedPointerGeometryFactory =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_geometryFactory_to_output(self as *const ::geometry_renderer::GeometryRenderer, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QGeometryRenderer::indexOffset() const```</span>
  ///
  ///
  pub fn index_offset(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_indexOffset(self as *const ::geometry_renderer::GeometryRenderer) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QGeometryRenderer::instanceCount() const```</span>
  ///
  ///
  pub fn instance_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_instanceCount(self as *const ::geometry_renderer::GeometryRenderer) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QGeometryRenderer::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_metaObject(self as *const ::geometry_renderer::GeometryRenderer) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QGeometryRenderer::QGeometryRenderer()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::geometry_renderer::GeometryRenderer> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QGeometryRenderer::QGeometryRenderer(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::geometry_renderer::GeometryRenderer> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QGeometryRenderer::primitiveRestartEnabled() const```</span>
  ///
  ///
  pub fn primitive_restart_enabled(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_primitiveRestartEnabled(self as *const ::geometry_renderer::GeometryRenderer) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QGeometryRenderer::PrimitiveType Qt3DRender::QGeometryRenderer::primitiveType() const```</span>
  ///
  ///
  pub fn primitive_type(&self) -> ::geometry_renderer::PrimitiveType {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_primitiveType(self as *const ::geometry_renderer::GeometryRenderer) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QGeometryRenderer::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_qt_metacall(self as *mut ::geometry_renderer::GeometryRenderer,
                                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                                   arg2,
                                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QGeometryRenderer::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_qt_metacast(self as *mut ::geometry_renderer::GeometryRenderer,
                                                                   arg1)
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QGeometryRenderer::restartIndexValue() const```</span>
  ///
  ///
  pub fn restart_index_value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_restartIndexValue(self as *const ::geometry_renderer::GeometryRenderer) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGeometryRenderer::setFirstInstance(int firstInstance)```</span>
  ///
  ///
  pub fn set_first_instance(&mut self, first_instance: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_setFirstInstance(self as *mut ::geometry_renderer::GeometryRenderer, first_instance) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGeometryRenderer::setFirstVertex(int firstVertex)```</span>
  ///
  ///
  pub fn set_first_vertex(&mut self, first_vertex: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_setFirstVertex(self as *mut ::geometry_renderer::GeometryRenderer, first_vertex) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGeometryRenderer::setGeometry(Qt3DRender::QGeometry* geometry)```</span>
  ///
  ///
  pub unsafe fn set_geometry(&mut self, geometry: *mut ::geometry::Geometry) {
    ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_setGeometry(self as *mut ::geometry_renderer::GeometryRenderer,
                                                                   geometry)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QGeometryRenderer::setGeometryFactory(const QSharedPointer<Qt3DRender::QGeometryFactory>& factory)```</span>
  ///
  ///
  pub fn set_geometry_factory(&mut self, factory: &::shared_pointer::SharedPointerGeometryFactory) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_setGeometryFactory(self as *mut ::geometry_renderer::GeometryRenderer, factory as *const ::shared_pointer::SharedPointerGeometryFactory) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGeometryRenderer::setIndexOffset(int indexOffset)```</span>
  ///
  ///
  pub fn set_index_offset(&mut self, index_offset: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_setIndexOffset(self as *mut ::geometry_renderer::GeometryRenderer, index_offset) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGeometryRenderer::setInstanceCount(int instanceCount)```</span>
  ///
  ///
  pub fn set_instance_count(&mut self, instance_count: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_setInstanceCount(self as *mut ::geometry_renderer::GeometryRenderer, instance_count) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGeometryRenderer::setPrimitiveRestartEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_primitive_restart_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_setPrimitiveRestartEnabled(self as *mut ::geometry_renderer::GeometryRenderer, enabled) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGeometryRenderer::setPrimitiveType(Qt3DRender::QGeometryRenderer::PrimitiveType primitiveType)```</span>
  ///
  ///
  pub fn set_primitive_type(&mut self, primitive_type: ::geometry_renderer::PrimitiveType) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_setPrimitiveType(self as *mut ::geometry_renderer::GeometryRenderer, primitive_type) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGeometryRenderer::setRestartIndexValue(int index)```</span>
  ///
  ///
  pub fn set_restart_index_value(&mut self, index: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_setRestartIndexValue(self as *mut ::geometry_renderer::GeometryRenderer, index) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGeometryRenderer::setVertexCount(int vertexCount)```</span>
  ///
  ///
  pub fn set_vertex_count(&mut self, vertex_count: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_setVertexCount(self as *mut ::geometry_renderer::GeometryRenderer, vertex_count) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGeometryRenderer::setVerticesPerPatch(int verticesPerPatch)```</span>
  ///
  ///
  pub fn set_vertices_per_patch(&mut self, vertices_per_patch: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_setVerticesPerPatch(self as *mut ::geometry_renderer::GeometryRenderer, vertices_per_patch) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QGeometryRenderer::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QGeometryRenderer::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QGeometryRenderer::vertexCount() const```</span>
  ///
  ///
  pub fn vertex_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_vertexCount(self as *const ::geometry_renderer::GeometryRenderer) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QGeometryRenderer::verticesPerPatch() const```</span>
  ///
  ///
  pub fn vertices_per_patch(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_verticesPerPatch(self as *const ::geometry_renderer::GeometryRenderer) }
  }
}

impl ::cpp_utils::CppDeletable for ::geometry_renderer::GeometryRenderer {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QGeometryRenderer_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `GeometryRenderer`.
  pub struct Signals<'a>(&'a ::geometry_renderer::GeometryRenderer);
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::instanceCountChanged`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().instance_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct InstanceCountChanged<'a>(&'a ::geometry_renderer::GeometryRenderer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::firstInstanceChanged`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().first_instance_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct FirstInstanceChanged<'a>(&'a ::geometry_renderer::GeometryRenderer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::primitiveRestartEnabledChanged`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().primitive_restart_enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct PrimitiveRestartEnabledChanged<'a>(&'a ::geometry_renderer::GeometryRenderer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::vertexCountChanged`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().vertex_count_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct VertexCountChanged<'a>(&'a ::geometry_renderer::GeometryRenderer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::geometryChanged`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().geometry_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct GeometryChanged<'a>(&'a ::geometry_renderer::GeometryRenderer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::firstVertexChanged`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().first_vertex_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct FirstVertexChanged<'a>(&'a ::geometry_renderer::GeometryRenderer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::addedToEntity`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().added_to_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct AddedToEntity<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for AddedToEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2addedToEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AddedToEntity<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::verticesPerPatchChanged`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().vertices_per_patch_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct VerticesPerPatchChanged<'a>(&'a ::geometry_renderer::GeometryRenderer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::shareableChanged`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().shareable_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct ShareableChanged<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for ShareableChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2shareableChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ShareableChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::indexOffsetChanged`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().index_offset_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct IndexOffsetChanged<'a>(&'a ::geometry_renderer::GeometryRenderer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::primitiveTypeChanged`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().primitive_type_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct PrimitiveTypeChanged<'a>(&'a ::geometry_renderer::GeometryRenderer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::restartIndexValueChanged`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().restart_index_value_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct RestartIndexValueChanged<'a>(&'a ::geometry_renderer::GeometryRenderer);
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
  /// Represents a built-in Qt signal `Qt3DRender::QGeometryRenderer::removedFromEntity`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.signals().removed_from_entity()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct RemovedFromEntity<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for RemovedFromEntity<'a> {
    type Arguments = (*mut ::qt_3d_core::entity::Entity,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2removedFromEntity(Qt3DCore::QEntity*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for RemovedFromEntity<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::instanceCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn instance_count_changed(&self) -> InstanceCountChanged {
      InstanceCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::firstInstanceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_instance_changed(&self) -> FirstInstanceChanged {
      FirstInstanceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::primitiveRestartEnabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_restart_enabled_changed(&self) -> PrimitiveRestartEnabledChanged {
      PrimitiveRestartEnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::vertexCountChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertex_count_changed(&self) -> VertexCountChanged {
      VertexCountChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::geometryChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn geometry_changed(&self) -> GeometryChanged {
      GeometryChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::firstVertexChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn first_vertex_changed(&self) -> FirstVertexChanged {
      FirstVertexChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::addedToEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn added_to_entity(&self) -> AddedToEntity {
      AddedToEntity(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::verticesPerPatchChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn vertices_per_patch_changed(&self) -> VerticesPerPatchChanged {
      VerticesPerPatchChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::shareableChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn shareable_changed(&self) -> ShareableChanged {
      ShareableChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::indexOffsetChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn index_offset_changed(&self) -> IndexOffsetChanged {
      IndexOffsetChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::primitiveTypeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn primitive_type_changed(&self) -> PrimitiveTypeChanged {
      PrimitiveTypeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::restartIndexValueChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn restart_index_value_changed(&self) -> RestartIndexValueChanged {
      RestartIndexValueChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometryRenderer::removedFromEntity`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn removed_from_entity(&self) -> RemovedFromEntity {
      RemovedFromEntity(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `GeometryRenderer`.
  pub struct Slots<'a>(&'a ::geometry_renderer::GeometryRenderer);
  /// Represents a built-in Qt slot `Qt3DRender::QGeometryRenderer::setInstanceCount`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.slots().set_instance_count()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct SetInstanceCount<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetInstanceCount<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setInstanceCount(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometryRenderer::setRestartIndexValue`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.slots().set_restart_index_value()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct SetRestartIndexValue<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetRestartIndexValue<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRestartIndexValue(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometryRenderer::setVertexCount`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.slots().set_vertex_count()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct SetVertexCount<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetVertexCount<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVertexCount(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometryRenderer::setFirstInstance`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.slots().set_first_instance()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct SetFirstInstance<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetFirstInstance<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFirstInstance(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometryRenderer::setFirstVertex`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.slots().set_first_vertex()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct SetFirstVertex<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetFirstVertex<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFirstVertex(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometryRenderer::setVerticesPerPatch`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.slots().set_vertices_per_patch()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct SetVerticesPerPatch<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetVerticesPerPatch<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setVerticesPerPatch(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometryRenderer::setPrimitiveRestartEnabled`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.slots().set_primitive_restart_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct SetPrimitiveRestartEnabled<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetPrimitiveRestartEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPrimitiveRestartEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometryRenderer::setGeometry`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.slots().set_geometry()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct SetGeometry<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetGeometry<'a> {
    type Arguments = (*mut ::geometry::Geometry,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGeometry(Qt3DRender::QGeometry*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometryRenderer::setShareable`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.slots().set_shareable()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct SetShareable<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetShareable<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setShareable(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometryRenderer::setIndexOffset`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.slots().set_index_offset()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct SetIndexOffset<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetIndexOffset<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setIndexOffset(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometryRenderer::setPrimitiveType`.
  ///
  /// An object of this type can be created from `GeometryRenderer` with `object.slots().set_primitive_type()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `GeometryRenderer` object.
  pub struct SetPrimitiveType<'a>(&'a ::geometry_renderer::GeometryRenderer);
  impl<'a> ::qt_core::connection::Receiver for SetPrimitiveType<'a> {
    type Arguments = (::geometry_renderer::PrimitiveType,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setPrimitiveType(Qt3DRender::QGeometryRenderer::PrimitiveType)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometryRenderer::setInstanceCount`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_instance_count(&self) -> SetInstanceCount {
      SetInstanceCount(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometryRenderer::setRestartIndexValue`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_restart_index_value(&self) -> SetRestartIndexValue {
      SetRestartIndexValue(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometryRenderer::setVertexCount`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertex_count(&self) -> SetVertexCount {
      SetVertexCount(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometryRenderer::setFirstInstance`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_first_instance(&self) -> SetFirstInstance {
      SetFirstInstance(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometryRenderer::setFirstVertex`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_first_vertex(&self) -> SetFirstVertex {
      SetFirstVertex(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometryRenderer::setVerticesPerPatch`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_vertices_per_patch(&self) -> SetVerticesPerPatch {
      SetVerticesPerPatch(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometryRenderer::setPrimitiveRestartEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_primitive_restart_enabled(&self) -> SetPrimitiveRestartEnabled {
      SetPrimitiveRestartEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometryRenderer::setGeometry`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_geometry(&self) -> SetGeometry {
      SetGeometry(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometryRenderer::setShareable`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_shareable(&self) -> SetShareable {
      SetShareable(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometryRenderer::setIndexOffset`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_index_offset(&self) -> SetIndexOffset {
      SetIndexOffset(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometryRenderer::setPrimitiveType`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_primitive_type(&self) -> SetPrimitiveType {
      SetPrimitiveType(self.0)
    }
  }
  impl ::geometry_renderer::GeometryRenderer {
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

/// C++ type: <span style='color: green;'>```Qt3DRender::QGeometryRenderer::PrimitiveType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PrimitiveType {
  /// C++ enum variant: <span style='color: green;'>```Points = 0```</span>
  Points = 0,
  /// C++ enum variant: <span style='color: green;'>```Lines = 1```</span>
  Lines = 1,
  /// C++ enum variant: <span style='color: green;'>```LineLoop = 2```</span>
  LineLoop = 2,
  /// C++ enum variant: <span style='color: green;'>```LineStrip = 3```</span>
  LineStrip = 3,
  /// C++ enum variant: <span style='color: green;'>```Triangles = 4```</span>
  Triangles = 4,
  /// C++ enum variant: <span style='color: green;'>```TriangleStrip = 5```</span>
  TriangleStrip = 5,
  /// C++ enum variant: <span style='color: green;'>```TriangleFan = 6```</span>
  TriangleFan = 6,
  /// C++ enum variant: <span style='color: green;'>```LinesAdjacency = 10```</span>
  LinesAdjacency = 10,
  /// C++ enum variant: <span style='color: green;'>```LineStripAdjacency = 11```</span>
  LineStripAdjacency = 11,
  /// C++ enum variant: <span style='color: green;'>```TrianglesAdjacency = 12```</span>
  TrianglesAdjacency = 12,
  /// C++ enum variant: <span style='color: green;'>```TriangleStripAdjacency = 13```</span>
  TriangleStripAdjacency = 13,
  /// C++ enum variant: <span style='color: green;'>```Patches = 14```</span>
  Patches = 14,
}

impl ::cpp_utils::StaticCast<::qt_3d_core::component::Component> for ::geometry_renderer::GeometryRenderer {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::geometry_renderer::GeometryRenderer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::geometry_renderer::GeometryRenderer as *mut ::geometry_renderer::GeometryRenderer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::geometry_renderer::GeometryRenderer {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::geometry_renderer::GeometryRenderer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::geometry_renderer::GeometryRenderer as *mut ::geometry_renderer::GeometryRenderer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::geometry_renderer::GeometryRenderer {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_QObject_ptr(self as *mut ::geometry_renderer::GeometryRenderer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_QObject_ptr(self as *const ::geometry_renderer::GeometryRenderer as *mut ::geometry_renderer::GeometryRenderer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::geometry_renderer::GeometryRenderer> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::geometry_renderer::GeometryRenderer {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::geometry_renderer::GeometryRenderer {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::geometry_renderer::GeometryRenderer> for ::qt_3d_core::component::Component {
  unsafe fn static_cast_mut(&mut self) -> &mut ::geometry_renderer::GeometryRenderer {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_Qt3DCore_QComponent(self as *mut ::qt_3d_core::component::Component);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::geometry_renderer::GeometryRenderer {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_Qt3DCore_QComponent(self as *const ::qt_3d_core::component::Component as *mut ::qt_3d_core::component::Component);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::geometry_renderer::GeometryRenderer> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::geometry_renderer::GeometryRenderer {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::geometry_renderer::GeometryRenderer {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DRender_QGeometryRenderer_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::geometry_renderer::GeometryRenderer {
  type Target = ::qt_3d_core::component::Component;
  fn deref(&self) -> &::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DCore_QComponent_ptr(self as *const ::geometry_renderer::GeometryRenderer as *mut ::geometry_renderer::GeometryRenderer) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::geometry_renderer::GeometryRenderer {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::component::Component {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometryRenderer_G_static_cast_Qt3DCore_QComponent_ptr(self as *mut ::geometry_renderer::GeometryRenderer) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
