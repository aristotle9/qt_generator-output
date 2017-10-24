/// C++ type: <span style='color: green;'>```Qt3DExtras::QConeGeometry```</span>
#[repr(C)]
pub struct ConeGeometry(u8);

impl ConeGeometry {
  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QConeGeometry::bottomRadius() const```</span>
  ///
  ///
  pub fn bottom_radius(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_bottomRadius(self as *const ::cone_geometry::ConeGeometry) }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DExtras::QConeGeometry::hasBottomEndcap() const```</span>
  ///
  ///
  pub fn has_bottom_endcap(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_hasBottomEndcap(self as *const ::cone_geometry::ConeGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DExtras::QConeGeometry::hasTopEndcap() const```</span>
  ///
  ///
  pub fn has_top_endcap(&self) -> bool {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_hasTopEndcap(self as *const ::cone_geometry::ConeGeometry) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QConeGeometry::indexAttribute() const```</span>
  ///
  ///
  pub fn index_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_indexAttribute(self as *const ::cone_geometry::ConeGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QConeGeometry::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_length(self as *const ::cone_geometry::ConeGeometry) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QConeGeometry::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_metaObject(self as *const ::cone_geometry::ConeGeometry) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QConeGeometry::QConeGeometry()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::cone_geometry::ConeGeometry> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QConeGeometry::QConeGeometry(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::cone_geometry::ConeGeometry> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QConeGeometry::normalAttribute() const```</span>
  ///
  ///
  pub fn normal_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_normalAttribute(self as *const ::cone_geometry::ConeGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QConeGeometry::positionAttribute() const```</span>
  ///
  ///
  pub fn position_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_positionAttribute(self as *const ::cone_geometry::ConeGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QConeGeometry::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_qt_metacall(self as *mut ::cone_geometry::ConeGeometry,
                                                               arg1 as *const ::qt_core::meta_object::Call,
                                                               arg2,
                                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QConeGeometry::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_qt_metacast(self as *mut ::cone_geometry::ConeGeometry, arg1)
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QConeGeometry::rings() const```</span>
  ///
  ///
  pub fn rings(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_rings(self as *const ::cone_geometry::ConeGeometry) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeGeometry::setBottomRadius(float bottomRadius)```</span>
  ///
  ///
  pub fn set_bottom_radius(&mut self, bottom_radius: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_setBottomRadius(self as *mut ::cone_geometry::ConeGeometry,
                                                                     bottom_radius)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeGeometry::setHasBottomEndcap(bool hasBottomEndcap)```</span>
  ///
  ///
  pub fn set_has_bottom_endcap(&mut self, has_bottom_endcap: bool) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_setHasBottomEndcap(self as *mut ::cone_geometry::ConeGeometry,
                                                                        has_bottom_endcap)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeGeometry::setHasTopEndcap(bool hasTopEndcap)```</span>
  ///
  ///
  pub fn set_has_top_endcap(&mut self, has_top_endcap: bool) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_setHasTopEndcap(self as *mut ::cone_geometry::ConeGeometry,
                                                                     has_top_endcap)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeGeometry::setLength(float length)```</span>
  ///
  ///
  pub fn set_length(&mut self, length: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_setLength(self as *mut ::cone_geometry::ConeGeometry, length)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeGeometry::setRings(int rings)```</span>
  ///
  ///
  pub fn set_rings(&mut self, rings: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_setRings(self as *mut ::cone_geometry::ConeGeometry, rings)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeGeometry::setSlices(int slices)```</span>
  ///
  ///
  pub fn set_slices(&mut self, slices: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_setSlices(self as *mut ::cone_geometry::ConeGeometry, slices)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QConeGeometry::setTopRadius(float topRadius)```</span>
  ///
  ///
  pub fn set_top_radius(&mut self, top_radius: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_setTopRadius(self as *mut ::cone_geometry::ConeGeometry,
                                                                  top_radius)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QConeGeometry::slices() const```</span>
  ///
  ///
  pub fn slices(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_slices(self as *const ::cone_geometry::ConeGeometry) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QConeGeometry::texCoordAttribute() const```</span>
  ///
  ///
  pub fn tex_coord_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_texCoordAttribute(self as *const ::cone_geometry::ConeGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QConeGeometry::topRadius() const```</span>
  ///
  ///
  pub fn top_radius(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_topRadius(self as *const ::cone_geometry::ConeGeometry) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QConeGeometry::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QConeGeometry::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QConeGeometry::updateIndices()```</span>
  ///
  ///
  pub fn update_indices(&mut self) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_updateIndices(self as *mut ::cone_geometry::ConeGeometry) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QConeGeometry::updateVertices()```</span>
  ///
  ///
  pub fn update_vertices(&mut self) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_updateVertices(self as *mut ::cone_geometry::ConeGeometry) }
  }
}

impl ::cpp_utils::CppDeletable for ::cone_geometry::ConeGeometry {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QConeGeometry_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ConeGeometry`.
  pub struct Signals<'a>(&'a ::cone_geometry::ConeGeometry);
  /// Represents a built-in Qt signal `Qt3DExtras::QConeGeometry::hasBottomEndcapChanged`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.signals().has_bottom_endcap_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct HasBottomEndcapChanged<'a>(&'a ::cone_geometry::ConeGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeGeometry::bottomRadiusChanged`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.signals().bottom_radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct BottomRadiusChanged<'a>(&'a ::cone_geometry::ConeGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeGeometry::lengthChanged`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.signals().length_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct LengthChanged<'a>(&'a ::cone_geometry::ConeGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeGeometry::boundingVolumePositionAttributeChanged`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.signals().bounding_volume_position_attribute_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct BoundingVolumePositionAttributeChanged<'a>(&'a ::cone_geometry::ConeGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeGeometry::topRadiusChanged`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.signals().top_radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct TopRadiusChanged<'a>(&'a ::cone_geometry::ConeGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeGeometry::slicesChanged`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.signals().slices_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct SlicesChanged<'a>(&'a ::cone_geometry::ConeGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeGeometry::ringsChanged`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.signals().rings_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct RingsChanged<'a>(&'a ::cone_geometry::ConeGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QConeGeometry::hasTopEndcapChanged`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.signals().has_top_endcap_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct HasTopEndcapChanged<'a>(&'a ::cone_geometry::ConeGeometry);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeGeometry::hasBottomEndcapChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn has_bottom_endcap_changed(&self) -> HasBottomEndcapChanged {
      HasBottomEndcapChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeGeometry::bottomRadiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bottom_radius_changed(&self) -> BottomRadiusChanged {
      BottomRadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeGeometry::lengthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn length_changed(&self) -> LengthChanged {
      LengthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeGeometry::boundingVolumePositionAttributeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bounding_volume_position_attribute_changed(&self) -> BoundingVolumePositionAttributeChanged {
      BoundingVolumePositionAttributeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeGeometry::topRadiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn top_radius_changed(&self) -> TopRadiusChanged {
      TopRadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeGeometry::slicesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slices_changed(&self) -> SlicesChanged {
      SlicesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeGeometry::ringsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rings_changed(&self) -> RingsChanged {
      RingsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QConeGeometry::hasTopEndcapChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn has_top_endcap_changed(&self) -> HasTopEndcapChanged {
      HasTopEndcapChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ConeGeometry`.
  pub struct Slots<'a>(&'a ::cone_geometry::ConeGeometry);
  /// Represents a built-in Qt slot `Qt3DExtras::QConeGeometry::setTopRadius`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.slots().set_top_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct SetTopRadius<'a>(&'a ::cone_geometry::ConeGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetTopRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTopRadius(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeGeometry::setBoundingVolumePositionAttribute`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.slots().set_bounding_volume_position_attribute()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct SetBoundingVolumePositionAttribute<'a>(&'a ::cone_geometry::ConeGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetBoundingVolumePositionAttribute<'a> {
    type Arguments = (*mut ::qt_3d_render::attribute::Attribute,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBoundingVolumePositionAttribute(Qt3DRender::QAttribute*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeGeometry::setLength`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.slots().set_length()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct SetLength<'a>(&'a ::cone_geometry::ConeGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetLength<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLength(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeGeometry::setRings`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.slots().set_rings()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct SetRings<'a>(&'a ::cone_geometry::ConeGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetRings<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRings(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeGeometry::setSlices`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.slots().set_slices()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct SetSlices<'a>(&'a ::cone_geometry::ConeGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetSlices<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSlices(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeGeometry::setHasBottomEndcap`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.slots().set_has_bottom_endcap()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct SetHasBottomEndcap<'a>(&'a ::cone_geometry::ConeGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetHasBottomEndcap<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHasBottomEndcap(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeGeometry::setBottomRadius`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.slots().set_bottom_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct SetBottomRadius<'a>(&'a ::cone_geometry::ConeGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetBottomRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBottomRadius(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QConeGeometry::setHasTopEndcap`.
  ///
  /// An object of this type can be created from `ConeGeometry` with `object.slots().set_has_top_endcap()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ConeGeometry` object.
  pub struct SetHasTopEndcap<'a>(&'a ::cone_geometry::ConeGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetHasTopEndcap<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHasTopEndcap(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeGeometry::setTopRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_top_radius(&self) -> SetTopRadius {
      SetTopRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeGeometry::setBoundingVolumePositionAttribute`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bounding_volume_position_attribute(&self) -> SetBoundingVolumePositionAttribute {
      SetBoundingVolumePositionAttribute(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeGeometry::setLength`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_length(&self) -> SetLength {
      SetLength(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeGeometry::setRings`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_rings(&self) -> SetRings {
      SetRings(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeGeometry::setSlices`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_slices(&self) -> SetSlices {
      SetSlices(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeGeometry::setHasBottomEndcap`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_has_bottom_endcap(&self) -> SetHasBottomEndcap {
      SetHasBottomEndcap(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeGeometry::setBottomRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bottom_radius(&self) -> SetBottomRadius {
      SetBottomRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QConeGeometry::setHasTopEndcap`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_has_top_endcap(&self) -> SetHasTopEndcap {
      SetHasTopEndcap(self.0)
    }
  }
  impl ::cone_geometry::ConeGeometry {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::cone_geometry::ConeGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::cone_geometry::ConeGeometry)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::cone_geometry::ConeGeometry as *mut ::cone_geometry::ConeGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry::Geometry> for ::cone_geometry::ConeGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::cone_geometry::ConeGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::cone_geometry::ConeGeometry as *mut ::cone_geometry::ConeGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::cone_geometry::ConeGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_QObject_ptr(self as *mut ::cone_geometry::ConeGeometry)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_QObject_ptr(self as *const ::cone_geometry::ConeGeometry as *mut ::cone_geometry::ConeGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cone_geometry::ConeGeometry> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cone_geometry::ConeGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DExtras_QConeGeometry_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cone_geometry::ConeGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DExtras_QConeGeometry_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cone_geometry::ConeGeometry> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cone_geometry::ConeGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DExtras_QConeGeometry_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cone_geometry::ConeGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DExtras_QConeGeometry_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cone_geometry::ConeGeometry> for ::qt_3d_render::geometry::Geometry {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cone_geometry::ConeGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DExtras_QConeGeometry_ptr_Qt3DRender_QGeometry(self as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cone_geometry::ConeGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DExtras_QConeGeometry_ptr_Qt3DRender_QGeometry(self as *const ::qt_3d_render::geometry::Geometry as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::cone_geometry::ConeGeometry {
  type Target = ::qt_3d_render::geometry::Geometry;
  fn deref(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::cone_geometry::ConeGeometry as *mut ::cone_geometry::ConeGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::cone_geometry::ConeGeometry {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QConeGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::cone_geometry::ConeGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
