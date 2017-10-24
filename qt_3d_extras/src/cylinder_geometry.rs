/// C++ type: <span style='color: green;'>```Qt3DExtras::QCylinderGeometry```</span>
#[repr(C)]
pub struct CylinderGeometry(u8);

impl CylinderGeometry {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QCylinderGeometry::indexAttribute() const```</span>
  ///
  ///
  pub fn index_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_indexAttribute(self as *const ::cylinder_geometry::CylinderGeometry) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QCylinderGeometry::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_length(self as *const ::cylinder_geometry::CylinderGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QCylinderGeometry::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_metaObject(self as *const ::cylinder_geometry::CylinderGeometry) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QCylinderGeometry::QCylinderGeometry()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::cylinder_geometry::CylinderGeometry> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QCylinderGeometry::QCylinderGeometry(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::cylinder_geometry::CylinderGeometry> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QCylinderGeometry::normalAttribute() const```</span>
  ///
  ///
  pub fn normal_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_normalAttribute(self as *const ::cylinder_geometry::CylinderGeometry) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QCylinderGeometry::positionAttribute() const```</span>
  ///
  ///
  pub fn position_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_positionAttribute(self as *const ::cylinder_geometry::CylinderGeometry) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QCylinderGeometry::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_qt_metacall(self as *mut ::cylinder_geometry::CylinderGeometry,
                                                                   arg1 as *const ::qt_core::meta_object::Call,
                                                                   arg2,
                                                                   arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QCylinderGeometry::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_qt_metacast(self as *mut ::cylinder_geometry::CylinderGeometry,
                                                                   arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QCylinderGeometry::radius() const```</span>
  ///
  ///
  pub fn radius(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_radius(self as *const ::cylinder_geometry::CylinderGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QCylinderGeometry::rings() const```</span>
  ///
  ///
  pub fn rings(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_rings(self as *const ::cylinder_geometry::CylinderGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCylinderGeometry::setLength(float length)```</span>
  ///
  ///
  pub fn set_length(&mut self, length: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_setLength(self as *mut ::cylinder_geometry::CylinderGeometry,
                                                                   length)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCylinderGeometry::setRadius(float radius)```</span>
  ///
  ///
  pub fn set_radius(&mut self, radius: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_setRadius(self as *mut ::cylinder_geometry::CylinderGeometry,
                                                                   radius)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCylinderGeometry::setRings(int rings)```</span>
  ///
  ///
  pub fn set_rings(&mut self, rings: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_setRings(self as *mut ::cylinder_geometry::CylinderGeometry,
                                                                  rings)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QCylinderGeometry::setSlices(int slices)```</span>
  ///
  ///
  pub fn set_slices(&mut self, slices: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_setSlices(self as *mut ::cylinder_geometry::CylinderGeometry,
                                                                   slices)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QCylinderGeometry::slices() const```</span>
  ///
  ///
  pub fn slices(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_slices(self as *const ::cylinder_geometry::CylinderGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QCylinderGeometry::texCoordAttribute() const```</span>
  ///
  ///
  pub fn tex_coord_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_texCoordAttribute(self as *const ::cylinder_geometry::CylinderGeometry) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QCylinderGeometry::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QCylinderGeometry::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QCylinderGeometry::updateIndices()```</span>
  ///
  ///
  pub fn update_indices(&mut self) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_updateIndices(self as *mut ::cylinder_geometry::CylinderGeometry) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QCylinderGeometry::updateVertices()```</span>
  ///
  ///
  pub fn update_vertices(&mut self) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_updateVertices(self as *mut ::cylinder_geometry::CylinderGeometry) }
  }
}

impl ::cpp_utils::CppDeletable for ::cylinder_geometry::CylinderGeometry {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QCylinderGeometry_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `CylinderGeometry`.
  pub struct Signals<'a>(&'a ::cylinder_geometry::CylinderGeometry);
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderGeometry::ringsChanged`.
  ///
  /// An object of this type can be created from `CylinderGeometry` with `object.signals().rings_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderGeometry` object.
  pub struct RingsChanged<'a>(&'a ::cylinder_geometry::CylinderGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderGeometry::radiusChanged`.
  ///
  /// An object of this type can be created from `CylinderGeometry` with `object.signals().radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderGeometry` object.
  pub struct RadiusChanged<'a>(&'a ::cylinder_geometry::CylinderGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderGeometry::lengthChanged`.
  ///
  /// An object of this type can be created from `CylinderGeometry` with `object.signals().length_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderGeometry` object.
  pub struct LengthChanged<'a>(&'a ::cylinder_geometry::CylinderGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderGeometry::slicesChanged`.
  ///
  /// An object of this type can be created from `CylinderGeometry` with `object.signals().slices_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderGeometry` object.
  pub struct SlicesChanged<'a>(&'a ::cylinder_geometry::CylinderGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QCylinderGeometry::boundingVolumePositionAttributeChanged`.
  ///
  /// An object of this type can be created from `CylinderGeometry` with `object.signals().bounding_volume_position_attribute_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderGeometry` object.
  pub struct BoundingVolumePositionAttributeChanged<'a>(&'a ::cylinder_geometry::CylinderGeometry);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderGeometry::ringsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rings_changed(&self) -> RingsChanged {
      RingsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderGeometry::radiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn radius_changed(&self) -> RadiusChanged {
      RadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderGeometry::lengthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn length_changed(&self) -> LengthChanged {
      LengthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderGeometry::slicesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slices_changed(&self) -> SlicesChanged {
      SlicesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QCylinderGeometry::boundingVolumePositionAttributeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bounding_volume_position_attribute_changed(&self) -> BoundingVolumePositionAttributeChanged {
      BoundingVolumePositionAttributeChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `CylinderGeometry`.
  pub struct Slots<'a>(&'a ::cylinder_geometry::CylinderGeometry);
  /// Represents a built-in Qt slot `Qt3DExtras::QCylinderGeometry::setBoundingVolumePositionAttribute`.
  ///
  /// An object of this type can be created from `CylinderGeometry` with `object.slots().set_bounding_volume_position_attribute()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderGeometry` object.
  pub struct SetBoundingVolumePositionAttribute<'a>(&'a ::cylinder_geometry::CylinderGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetBoundingVolumePositionAttribute<'a> {
    type Arguments = (*mut ::qt_3d_render::attribute::Attribute,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBoundingVolumePositionAttribute(Qt3DRender::QAttribute*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCylinderGeometry::setRadius`.
  ///
  /// An object of this type can be created from `CylinderGeometry` with `object.slots().set_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderGeometry` object.
  pub struct SetRadius<'a>(&'a ::cylinder_geometry::CylinderGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRadius(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCylinderGeometry::setSlices`.
  ///
  /// An object of this type can be created from `CylinderGeometry` with `object.slots().set_slices()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderGeometry` object.
  pub struct SetSlices<'a>(&'a ::cylinder_geometry::CylinderGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetSlices<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSlices(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCylinderGeometry::setLength`.
  ///
  /// An object of this type can be created from `CylinderGeometry` with `object.slots().set_length()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderGeometry` object.
  pub struct SetLength<'a>(&'a ::cylinder_geometry::CylinderGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetLength<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLength(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QCylinderGeometry::setRings`.
  ///
  /// An object of this type can be created from `CylinderGeometry` with `object.slots().set_rings()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `CylinderGeometry` object.
  pub struct SetRings<'a>(&'a ::cylinder_geometry::CylinderGeometry);
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
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCylinderGeometry::setBoundingVolumePositionAttribute`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bounding_volume_position_attribute(&self) -> SetBoundingVolumePositionAttribute {
      SetBoundingVolumePositionAttribute(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCylinderGeometry::setRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_radius(&self) -> SetRadius {
      SetRadius(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCylinderGeometry::setSlices`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_slices(&self) -> SetSlices {
      SetSlices(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCylinderGeometry::setLength`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_length(&self) -> SetLength {
      SetLength(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QCylinderGeometry::setRings`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_rings(&self) -> SetRings {
      SetRings(self.0)
    }
  }
  impl ::cylinder_geometry::CylinderGeometry {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::cylinder_geometry::CylinderGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::cylinder_geometry::CylinderGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::cylinder_geometry::CylinderGeometry as *mut ::cylinder_geometry::CylinderGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry::Geometry> for ::cylinder_geometry::CylinderGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::cylinder_geometry::CylinderGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::cylinder_geometry::CylinderGeometry as *mut ::cylinder_geometry::CylinderGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::cylinder_geometry::CylinderGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_QObject_ptr(self as *mut ::cylinder_geometry::CylinderGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_QObject_ptr(self as *const ::cylinder_geometry::CylinderGeometry as *mut ::cylinder_geometry::CylinderGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cylinder_geometry::CylinderGeometry> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cylinder_geometry::CylinderGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DExtras_QCylinderGeometry_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cylinder_geometry::CylinderGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DExtras_QCylinderGeometry_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cylinder_geometry::CylinderGeometry> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cylinder_geometry::CylinderGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DExtras_QCylinderGeometry_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cylinder_geometry::CylinderGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DExtras_QCylinderGeometry_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::cylinder_geometry::CylinderGeometry> for ::qt_3d_render::geometry::Geometry {
  unsafe fn static_cast_mut(&mut self) -> &mut ::cylinder_geometry::CylinderGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DExtras_QCylinderGeometry_ptr_Qt3DRender_QGeometry(self as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::cylinder_geometry::CylinderGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DExtras_QCylinderGeometry_ptr_Qt3DRender_QGeometry(self as *const ::qt_3d_render::geometry::Geometry as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::cylinder_geometry::CylinderGeometry {
  type Target = ::qt_3d_render::geometry::Geometry;
  fn deref(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::cylinder_geometry::CylinderGeometry as *mut ::cylinder_geometry::CylinderGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::cylinder_geometry::CylinderGeometry {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QCylinderGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::cylinder_geometry::CylinderGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
