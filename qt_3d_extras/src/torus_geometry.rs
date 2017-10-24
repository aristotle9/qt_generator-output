/// C++ type: <span style='color: green;'>```Qt3DExtras::QTorusGeometry```</span>
#[repr(C)]
pub struct TorusGeometry(u8);

impl TorusGeometry {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QTorusGeometry::indexAttribute() const```</span>
  ///
  ///
  pub fn index_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_indexAttribute(self as *const ::torus_geometry::TorusGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QTorusGeometry::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_metaObject(self as *const ::torus_geometry::TorusGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QTorusGeometry::minorRadius() const```</span>
  ///
  ///
  pub fn minor_radius(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_minorRadius(self as *const ::torus_geometry::TorusGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QTorusGeometry::QTorusGeometry()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::torus_geometry::TorusGeometry> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QTorusGeometry::QTorusGeometry(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::torus_geometry::TorusGeometry> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QTorusGeometry::normalAttribute() const```</span>
  ///
  ///
  pub fn normal_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_normalAttribute(self as *const ::torus_geometry::TorusGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QTorusGeometry::positionAttribute() const```</span>
  ///
  ///
  pub fn position_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_positionAttribute(self as *const ::torus_geometry::TorusGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QTorusGeometry::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_qt_metacall(self as *mut ::torus_geometry::TorusGeometry,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QTorusGeometry::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_qt_metacast(self as *mut ::torus_geometry::TorusGeometry, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QTorusGeometry::radius() const```</span>
  ///
  ///
  pub fn radius(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_radius(self as *const ::torus_geometry::TorusGeometry) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QTorusGeometry::rings() const```</span>
  ///
  ///
  pub fn rings(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_rings(self as *const ::torus_geometry::TorusGeometry) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTorusGeometry::setMinorRadius(float minorRadius)```</span>
  ///
  ///
  pub fn set_minor_radius(&mut self, minor_radius: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_setMinorRadius(self as *mut ::torus_geometry::TorusGeometry,
                                                                     minor_radius)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTorusGeometry::setRadius(float radius)```</span>
  ///
  ///
  pub fn set_radius(&mut self, radius: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_setRadius(self as *mut ::torus_geometry::TorusGeometry, radius)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTorusGeometry::setRings(int rings)```</span>
  ///
  ///
  pub fn set_rings(&mut self, rings: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_setRings(self as *mut ::torus_geometry::TorusGeometry, rings)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QTorusGeometry::setSlices(int slices)```</span>
  ///
  ///
  pub fn set_slices(&mut self, slices: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_setSlices(self as *mut ::torus_geometry::TorusGeometry, slices)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QTorusGeometry::slices() const```</span>
  ///
  ///
  pub fn slices(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_slices(self as *const ::torus_geometry::TorusGeometry) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QTorusGeometry::texCoordAttribute() const```</span>
  ///
  ///
  pub fn tex_coord_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_texCoordAttribute(self as *const ::torus_geometry::TorusGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QTorusGeometry::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QTorusGeometry::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QTorusGeometry::updateIndices()```</span>
  ///
  ///
  pub fn update_indices(&mut self) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_updateIndices(self as *mut ::torus_geometry::TorusGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QTorusGeometry::updateVertices()```</span>
  ///
  ///
  pub fn update_vertices(&mut self) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_updateVertices(self as *mut ::torus_geometry::TorusGeometry)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::torus_geometry::TorusGeometry {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QTorusGeometry_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TorusGeometry`.
  pub struct Signals<'a>(&'a ::torus_geometry::TorusGeometry);
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusGeometry::ringsChanged`.
  ///
  /// An object of this type can be created from `TorusGeometry` with `object.signals().rings_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusGeometry` object.
  pub struct RingsChanged<'a>(&'a ::torus_geometry::TorusGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusGeometry::radiusChanged`.
  ///
  /// An object of this type can be created from `TorusGeometry` with `object.signals().radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusGeometry` object.
  pub struct RadiusChanged<'a>(&'a ::torus_geometry::TorusGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusGeometry::slicesChanged`.
  ///
  /// An object of this type can be created from `TorusGeometry` with `object.signals().slices_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusGeometry` object.
  pub struct SlicesChanged<'a>(&'a ::torus_geometry::TorusGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusGeometry::boundingVolumePositionAttributeChanged`.
  ///
  /// An object of this type can be created from `TorusGeometry` with `object.signals().bounding_volume_position_attribute_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusGeometry` object.
  pub struct BoundingVolumePositionAttributeChanged<'a>(&'a ::torus_geometry::TorusGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QTorusGeometry::minorRadiusChanged`.
  ///
  /// An object of this type can be created from `TorusGeometry` with `object.signals().minor_radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusGeometry` object.
  pub struct MinorRadiusChanged<'a>(&'a ::torus_geometry::TorusGeometry);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusGeometry::ringsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rings_changed(&self) -> RingsChanged {
      RingsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusGeometry::radiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn radius_changed(&self) -> RadiusChanged {
      RadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusGeometry::slicesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slices_changed(&self) -> SlicesChanged {
      SlicesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusGeometry::boundingVolumePositionAttributeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bounding_volume_position_attribute_changed(&self) -> BoundingVolumePositionAttributeChanged {
      BoundingVolumePositionAttributeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QTorusGeometry::minorRadiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn minor_radius_changed(&self) -> MinorRadiusChanged {
      MinorRadiusChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TorusGeometry`.
  pub struct Slots<'a>(&'a ::torus_geometry::TorusGeometry);
  /// Represents a built-in Qt slot `Qt3DExtras::QTorusGeometry::setBoundingVolumePositionAttribute`.
  ///
  /// An object of this type can be created from `TorusGeometry` with `object.slots().set_bounding_volume_position_attribute()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusGeometry` object.
  pub struct SetBoundingVolumePositionAttribute<'a>(&'a ::torus_geometry::TorusGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetBoundingVolumePositionAttribute<'a> {
    type Arguments = (*mut ::qt_3d_render::attribute::Attribute,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBoundingVolumePositionAttribute(Qt3DRender::QAttribute*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTorusGeometry::setRadius`.
  ///
  /// An object of this type can be created from `TorusGeometry` with `object.slots().set_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusGeometry` object.
  pub struct SetRadius<'a>(&'a ::torus_geometry::TorusGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRadius(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTorusGeometry::setSlices`.
  ///
  /// An object of this type can be created from `TorusGeometry` with `object.slots().set_slices()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusGeometry` object.
  pub struct SetSlices<'a>(&'a ::torus_geometry::TorusGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetSlices<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSlices(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTorusGeometry::setRings`.
  ///
  /// An object of this type can be created from `TorusGeometry` with `object.slots().set_rings()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusGeometry` object.
  pub struct SetRings<'a>(&'a ::torus_geometry::TorusGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetRings<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRings(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QTorusGeometry::setMinorRadius`.
  ///
  /// An object of this type can be created from `TorusGeometry` with `object.slots().set_minor_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TorusGeometry` object.
  pub struct SetMinorRadius<'a>(&'a ::torus_geometry::TorusGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetMinorRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMinorRadius(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTorusGeometry::setBoundingVolumePositionAttribute`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bounding_volume_position_attribute(&self) -> SetBoundingVolumePositionAttribute {
      SetBoundingVolumePositionAttribute(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTorusGeometry::setRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_radius(&self) -> SetRadius {
      SetRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTorusGeometry::setSlices`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_slices(&self) -> SetSlices {
      SetSlices(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTorusGeometry::setRings`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_rings(&self) -> SetRings {
      SetRings(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QTorusGeometry::setMinorRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_minor_radius(&self) -> SetMinorRadius {
      SetMinorRadius(self.0)
    }
  }
  impl ::torus_geometry::TorusGeometry {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::torus_geometry::TorusGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::torus_geometry::TorusGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::torus_geometry::TorusGeometry as *mut ::torus_geometry::TorusGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry::Geometry> for ::torus_geometry::TorusGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::torus_geometry::TorusGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::torus_geometry::TorusGeometry as *mut ::torus_geometry::TorusGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::torus_geometry::TorusGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_QObject_ptr(self as *mut ::torus_geometry::TorusGeometry)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_QObject_ptr(self as *const ::torus_geometry::TorusGeometry as *mut ::torus_geometry::TorusGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::torus_geometry::TorusGeometry> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::torus_geometry::TorusGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DExtras_QTorusGeometry_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::torus_geometry::TorusGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DExtras_QTorusGeometry_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::torus_geometry::TorusGeometry> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::torus_geometry::TorusGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DExtras_QTorusGeometry_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::torus_geometry::TorusGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DExtras_QTorusGeometry_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::torus_geometry::TorusGeometry> for ::qt_3d_render::geometry::Geometry {
  unsafe fn static_cast_mut(&mut self) -> &mut ::torus_geometry::TorusGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DExtras_QTorusGeometry_ptr_Qt3DRender_QGeometry(self as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::torus_geometry::TorusGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DExtras_QTorusGeometry_ptr_Qt3DRender_QGeometry(self as *const ::qt_3d_render::geometry::Geometry as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::torus_geometry::TorusGeometry {
  type Target = ::qt_3d_render::geometry::Geometry;
  fn deref(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::torus_geometry::TorusGeometry as *mut ::torus_geometry::TorusGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::torus_geometry::TorusGeometry {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QTorusGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::torus_geometry::TorusGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
