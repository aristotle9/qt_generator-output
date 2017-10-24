/// C++ type: <span style='color: green;'>```Qt3DExtras::QCuboidGeometry```</span>
#[repr(C)]
pub struct CuboidGeometry(u8);

impl CuboidGeometry {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QCuboidGeometry::indexAttribute() const```</span>
  ///
  ///
  pub fn index_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_indexAttribute(self as *const ::cuboid_geometry::CuboidGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QCuboidGeometry::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_metaObject(self as *const ::cuboid_geometry::CuboidGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QCuboidGeometry::QCuboidGeometry()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::cuboid_geometry::CuboidGeometry> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QCuboidGeometry::QCuboidGeometry(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::cuboid_geometry::CuboidGeometry> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QCuboidGeometry::normalAttribute() const```</span>
  ///
  ///
  pub fn normal_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_normalAttribute(self as *const ::cuboid_geometry::CuboidGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QCuboidGeometry::positionAttribute() const```</span>
  ///
  ///
  pub fn position_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_positionAttribute(self as *const ::cuboid_geometry::CuboidGeometry) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QCuboidGeometry::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_qt_metacall(self as *mut ::cuboid_geometry::CuboidGeometry,
                                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                                 arg2,
                                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QCuboidGeometry::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_qt_metacast(self as *mut ::cuboid_geometry::CuboidGeometry, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidGeometry::setXExtent(float xExtent)```</span>
  ///
  ///
  pub fn set_x_extent(&mut self, x_extent: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setXExtent(self as *mut ::cuboid_geometry::CuboidGeometry,
                                                                  x_extent)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidGeometry::setXYMeshResolution(const QSize& resolution)```</span>
  ///
  ///
  pub fn set_x_y_mesh_resolution(&mut self, resolution: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setXYMeshResolution(self as *mut ::cuboid_geometry::CuboidGeometry, resolution as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidGeometry::setXZMeshResolution(const QSize& resolution)```</span>
  ///
  ///
  pub fn set_x_z_mesh_resolution(&mut self, resolution: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setXZMeshResolution(self as *mut ::cuboid_geometry::CuboidGeometry, resolution as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidGeometry::setYExtent(float yExtent)```</span>
  ///
  ///
  pub fn set_y_extent(&mut self, y_extent: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setYExtent(self as *mut ::cuboid_geometry::CuboidGeometry,
                                                                  y_extent)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidGeometry::setYZMeshResolution(const QSize& resolution)```</span>
  ///
  ///
  pub fn set_y_z_mesh_resolution(&mut self, resolution: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setYZMeshResolution(self as *mut ::cuboid_geometry::CuboidGeometry, resolution as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCuboidGeometry::setZExtent(float zExtent)```</span>
  ///
  ///
  pub fn set_z_extent(&mut self, z_extent: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_setZExtent(self as *mut ::cuboid_geometry::CuboidGeometry,
                                                                  z_extent)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QCuboidGeometry::tangentAttribute() const```</span>
  ///
  ///
  pub fn tangent_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_tangentAttribute(self as *const ::cuboid_geometry::CuboidGeometry) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QCuboidGeometry::texCoordAttribute() const```</span>
  ///
  ///
  pub fn tex_coord_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_texCoordAttribute(self as *const ::cuboid_geometry::CuboidGeometry) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QCuboidGeometry::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QCuboidGeometry::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QCuboidGeometry::updateIndices()```</span>
  ///
  ///
  pub fn update_indices(&mut self) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_updateIndices(self as *mut ::cuboid_geometry::CuboidGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QCuboidGeometry::updateVertices()```</span>
  ///
  ///
  pub fn update_vertices(&mut self) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_updateVertices(self as *mut ::cuboid_geometry::CuboidGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QCuboidGeometry::xExtent() const```</span>
  ///
  ///
  pub fn x_extent(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_xExtent(self as *const ::cuboid_geometry::CuboidGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize Qt3DExtras::QCuboidGeometry::xyMeshResolution() const```</span>
  ///
  ///
  pub fn xy_mesh_resolution(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_xyMeshResolution_to_output(self as *const ::cuboid_geometry::CuboidGeometry, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize Qt3DExtras::QCuboidGeometry::xzMeshResolution() const```</span>
  ///
  ///
  pub fn xz_mesh_resolution(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_xzMeshResolution_to_output(self as *const ::cuboid_geometry::CuboidGeometry, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QCuboidGeometry::yExtent() const```</span>
  ///
  ///
  pub fn y_extent(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_yExtent(self as *const ::cuboid_geometry::CuboidGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize Qt3DExtras::QCuboidGeometry::yzMeshResolution() const```</span>
  ///
  ///
  pub fn yz_mesh_resolution(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_yzMeshResolution_to_output(self as *const ::cuboid_geometry::CuboidGeometry, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QCuboidGeometry::zExtent() const```</span>
  ///
  ///
  pub fn z_extent(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_zExtent(self as *const ::cuboid_geometry::CuboidGeometry)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::cuboid_geometry::CuboidGeometry {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCuboidGeometry_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `CuboidGeometry`.
  pub struct Signals<'a>(&'a ::cuboid_geometry::CuboidGeometry);
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidGeometry::yzMeshResolutionChanged`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.signals().yz_mesh_resolution_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct YzMeshResolutionChanged<'a>(&'a ::cuboid_geometry::CuboidGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidGeometry::yExtentChanged`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.signals().y_extent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct YExtentChanged<'a>(&'a ::cuboid_geometry::CuboidGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidGeometry::xyMeshResolutionChanged`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.signals().xy_mesh_resolution_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct XyMeshResolutionChanged<'a>(&'a ::cuboid_geometry::CuboidGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidGeometry::xzMeshResolutionChanged`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.signals().xz_mesh_resolution_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct XzMeshResolutionChanged<'a>(&'a ::cuboid_geometry::CuboidGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidGeometry::zExtentChanged`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.signals().z_extent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct ZExtentChanged<'a>(&'a ::cuboid_geometry::CuboidGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidGeometry::boundingVolumePositionAttributeChanged`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.signals().bounding_volume_position_attribute_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct BoundingVolumePositionAttributeChanged<'a>(&'a ::cuboid_geometry::CuboidGeometry);
  impl<'a> ::qt_core::connection::Receiver for BoundingVolumePositionAttributeChanged<'a> {
    type Arguments = (*mut ::qt_3d_render::attribute::Attribute,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2boundingVolumePositionAttributeChanged(Qt3DRender::QAttribute*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BoundingVolumePositionAttributeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QCuboidGeometry::xExtentChanged`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.signals().x_extent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct XExtentChanged<'a>(&'a ::cuboid_geometry::CuboidGeometry);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidGeometry::yzMeshResolutionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn yz_mesh_resolution_changed(&self) -> YzMeshResolutionChanged {
      YzMeshResolutionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidGeometry::yExtentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn y_extent_changed(&self) -> YExtentChanged {
      YExtentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidGeometry::xyMeshResolutionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn xy_mesh_resolution_changed(&self) -> XyMeshResolutionChanged {
      XyMeshResolutionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidGeometry::xzMeshResolutionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn xz_mesh_resolution_changed(&self) -> XzMeshResolutionChanged {
      XzMeshResolutionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidGeometry::zExtentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn z_extent_changed(&self) -> ZExtentChanged {
      ZExtentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidGeometry::boundingVolumePositionAttributeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bounding_volume_position_attribute_changed(&self) -> BoundingVolumePositionAttributeChanged {
      BoundingVolumePositionAttributeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCuboidGeometry::xExtentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn x_extent_changed(&self) -> XExtentChanged {
      XExtentChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `CuboidGeometry`.
  pub struct Slots<'a>(&'a ::cuboid_geometry::CuboidGeometry);
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setYZMeshResolution`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.slots().set_y_z_mesh_resolution()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct SetYZMeshResolution<'a>(&'a ::cuboid_geometry::CuboidGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetYZMeshResolution<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setYZMeshResolution(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setZExtent`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.slots().set_z_extent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct SetZExtent<'a>(&'a ::cuboid_geometry::CuboidGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetZExtent<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setZExtent(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setXYMeshResolution`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.slots().set_x_y_mesh_resolution()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct SetXYMeshResolution<'a>(&'a ::cuboid_geometry::CuboidGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetXYMeshResolution<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setXYMeshResolution(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setYExtent`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.slots().set_y_extent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct SetYExtent<'a>(&'a ::cuboid_geometry::CuboidGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetYExtent<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setYExtent(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setBoundingVolumePositionAttribute`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.slots().set_bounding_volume_position_attribute()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct SetBoundingVolumePositionAttribute<'a>(&'a ::cuboid_geometry::CuboidGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetBoundingVolumePositionAttribute<'a> {
    type Arguments = (*mut ::qt_3d_render::attribute::Attribute,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBoundingVolumePositionAttribute(Qt3DRender::QAttribute*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setXExtent`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.slots().set_x_extent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct SetXExtent<'a>(&'a ::cuboid_geometry::CuboidGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetXExtent<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setXExtent(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setXZMeshResolution`.
  ///
  /// An object of this type can be created from `CuboidGeometry` with `object.slots().set_x_z_mesh_resolution()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CuboidGeometry` object.
  pub struct SetXZMeshResolution<'a>(&'a ::cuboid_geometry::CuboidGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetXZMeshResolution<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setXZMeshResolution(const QSize&)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setYZMeshResolution`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_y_z_mesh_resolution(&self) -> SetYZMeshResolution {
      SetYZMeshResolution(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setZExtent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_z_extent(&self) -> SetZExtent {
      SetZExtent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setXYMeshResolution`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_x_y_mesh_resolution(&self) -> SetXYMeshResolution {
      SetXYMeshResolution(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setYExtent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_y_extent(&self) -> SetYExtent {
      SetYExtent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setBoundingVolumePositionAttribute`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bounding_volume_position_attribute(&self) -> SetBoundingVolumePositionAttribute {
      SetBoundingVolumePositionAttribute(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setXExtent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_x_extent(&self) -> SetXExtent {
      SetXExtent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCuboidGeometry::setXZMeshResolution`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_x_z_mesh_resolution(&self) -> SetXZMeshResolution {
      SetXZMeshResolution(self.0)
    }
  }
  impl ::cuboid_geometry::CuboidGeometry {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::cuboid_geometry::CuboidGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::cuboid_geometry::CuboidGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::cuboid_geometry::CuboidGeometry as *mut ::cuboid_geometry::CuboidGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry::Geometry> for ::cuboid_geometry::CuboidGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::cuboid_geometry::CuboidGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::cuboid_geometry::CuboidGeometry as *mut ::cuboid_geometry::CuboidGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::cuboid_geometry::CuboidGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_QObject_ptr(self as *mut ::cuboid_geometry::CuboidGeometry)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_QObject_ptr(self as *const ::cuboid_geometry::CuboidGeometry as *mut ::cuboid_geometry::CuboidGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cuboid_geometry::CuboidGeometry> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cuboid_geometry::CuboidGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DExtras_QCuboidGeometry_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cuboid_geometry::CuboidGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DExtras_QCuboidGeometry_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cuboid_geometry::CuboidGeometry> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cuboid_geometry::CuboidGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DExtras_QCuboidGeometry_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cuboid_geometry::CuboidGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DExtras_QCuboidGeometry_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cuboid_geometry::CuboidGeometry> for ::qt_3d_render::geometry::Geometry {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cuboid_geometry::CuboidGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DExtras_QCuboidGeometry_ptr_Qt3DRender_QGeometry(self as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cuboid_geometry::CuboidGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DExtras_QCuboidGeometry_ptr_Qt3DRender_QGeometry(self as *const ::qt_3d_render::geometry::Geometry as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::cuboid_geometry::CuboidGeometry {
  type Target = ::qt_3d_render::geometry::Geometry;
  fn deref(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::cuboid_geometry::CuboidGeometry as *mut ::cuboid_geometry::CuboidGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::cuboid_geometry::CuboidGeometry {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCuboidGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::cuboid_geometry::CuboidGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
