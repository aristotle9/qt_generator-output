/// C++ type: <span style='color: green;'>```Qt3DExtras::QSphereGeometry```</span>
#[repr(C)]
pub struct SphereGeometry(u8);

impl SphereGeometry {
  /// C++ method: <span style='color: green;'>```bool Qt3DExtras::QSphereGeometry::generateTangents() const```</span>
  ///
  ///
  pub fn generate_tangents(&self) -> bool {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_generateTangents(self as *const ::sphere_geometry::SphereGeometry) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QSphereGeometry::indexAttribute() const```</span>
  ///
  ///
  pub fn index_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_indexAttribute(self as *const ::sphere_geometry::SphereGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QSphereGeometry::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_metaObject(self as *const ::sphere_geometry::SphereGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QSphereGeometry::QSphereGeometry()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::sphere_geometry::SphereGeometry> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QSphereGeometry::QSphereGeometry(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::sphere_geometry::SphereGeometry> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QSphereGeometry::normalAttribute() const```</span>
  ///
  ///
  pub fn normal_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_normalAttribute(self as *const ::sphere_geometry::SphereGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QSphereGeometry::positionAttribute() const```</span>
  ///
  ///
  pub fn position_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_positionAttribute(self as *const ::sphere_geometry::SphereGeometry) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QSphereGeometry::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_qt_metacall(self as *mut ::sphere_geometry::SphereGeometry,
                                                                 arg1 as *const ::qt_core::meta_object::Call,
                                                                 arg2,
                                                                 arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QSphereGeometry::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_qt_metacast(self as *mut ::sphere_geometry::SphereGeometry, arg1)
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QSphereGeometry::radius() const```</span>
  ///
  ///
  pub fn radius(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_radius(self as *const ::sphere_geometry::SphereGeometry) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QSphereGeometry::rings() const```</span>
  ///
  ///
  pub fn rings(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_rings(self as *const ::sphere_geometry::SphereGeometry) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QSphereGeometry::setGenerateTangents(bool gen)```</span>
  ///
  ///
  pub fn set_generate_tangents(&mut self, gen: bool) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setGenerateTangents(self as *mut ::sphere_geometry::SphereGeometry, gen) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QSphereGeometry::setRadius(float radius)```</span>
  ///
  ///
  pub fn set_radius(&mut self, radius: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setRadius(self as *mut ::sphere_geometry::SphereGeometry, radius)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QSphereGeometry::setRings(int rings)```</span>
  ///
  ///
  pub fn set_rings(&mut self, rings: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setRings(self as *mut ::sphere_geometry::SphereGeometry, rings)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QSphereGeometry::setSlices(int slices)```</span>
  ///
  ///
  pub fn set_slices(&mut self, slices: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_setSlices(self as *mut ::sphere_geometry::SphereGeometry, slices)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DExtras::QSphereGeometry::slices() const```</span>
  ///
  ///
  pub fn slices(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_slices(self as *const ::sphere_geometry::SphereGeometry) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QSphereGeometry::tangentAttribute() const```</span>
  ///
  ///
  pub fn tangent_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_tangentAttribute(self as *const ::sphere_geometry::SphereGeometry) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QSphereGeometry::texCoordAttribute() const```</span>
  ///
  ///
  pub fn tex_coord_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_texCoordAttribute(self as *const ::sphere_geometry::SphereGeometry) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QSphereGeometry::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QSphereGeometry::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QSphereGeometry::updateIndices()```</span>
  ///
  ///
  pub fn update_indices(&mut self) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_updateIndices(self as *mut ::sphere_geometry::SphereGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QSphereGeometry::updateVertices()```</span>
  ///
  ///
  pub fn update_vertices(&mut self) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_updateVertices(self as *mut ::sphere_geometry::SphereGeometry)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::sphere_geometry::SphereGeometry {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QSphereGeometry_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `SphereGeometry`.
  pub struct Signals<'a>(&'a ::sphere_geometry::SphereGeometry);
  /// Represents a built-in Qt signal `Qt3DExtras::QSphereGeometry::slicesChanged`.
  ///
  /// An object of this type can be created from `SphereGeometry` with `object.signals().slices_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SphereGeometry` object.
  pub struct SlicesChanged<'a>(&'a ::sphere_geometry::SphereGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QSphereGeometry::generateTangentsChanged`.
  ///
  /// An object of this type can be created from `SphereGeometry` with `object.signals().generate_tangents_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SphereGeometry` object.
  pub struct GenerateTangentsChanged<'a>(&'a ::sphere_geometry::SphereGeometry);
  impl<'a> ::qt_core::connection::Receiver for GenerateTangentsChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2generateTangentsChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for GenerateTangentsChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QSphereGeometry::ringsChanged`.
  ///
  /// An object of this type can be created from `SphereGeometry` with `object.signals().rings_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SphereGeometry` object.
  pub struct RingsChanged<'a>(&'a ::sphere_geometry::SphereGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QSphereGeometry::radiusChanged`.
  ///
  /// An object of this type can be created from `SphereGeometry` with `object.signals().radius_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SphereGeometry` object.
  pub struct RadiusChanged<'a>(&'a ::sphere_geometry::SphereGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QSphereGeometry::boundingVolumePositionAttributeChanged`.
  ///
  /// An object of this type can be created from `SphereGeometry` with `object.signals().bounding_volume_position_attribute_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SphereGeometry` object.
  pub struct BoundingVolumePositionAttributeChanged<'a>(&'a ::sphere_geometry::SphereGeometry);
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
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QSphereGeometry::slicesChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn slices_changed(&self) -> SlicesChanged {
      SlicesChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QSphereGeometry::generateTangentsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn generate_tangents_changed(&self) -> GenerateTangentsChanged {
      GenerateTangentsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QSphereGeometry::ringsChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn rings_changed(&self) -> RingsChanged {
      RingsChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QSphereGeometry::radiusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn radius_changed(&self) -> RadiusChanged {
      RadiusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QSphereGeometry::boundingVolumePositionAttributeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bounding_volume_position_attribute_changed(&self) -> BoundingVolumePositionAttributeChanged {
      BoundingVolumePositionAttributeChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `SphereGeometry`.
  pub struct Slots<'a>(&'a ::sphere_geometry::SphereGeometry);
  /// Represents a built-in Qt slot `Qt3DExtras::QSphereGeometry::setSlices`.
  ///
  /// An object of this type can be created from `SphereGeometry` with `object.slots().set_slices()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SphereGeometry` object.
  pub struct SetSlices<'a>(&'a ::sphere_geometry::SphereGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetSlices<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSlices(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QSphereGeometry::setGenerateTangents`.
  ///
  /// An object of this type can be created from `SphereGeometry` with `object.slots().set_generate_tangents()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SphereGeometry` object.
  pub struct SetGenerateTangents<'a>(&'a ::sphere_geometry::SphereGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetGenerateTangents<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setGenerateTangents(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QSphereGeometry::setBoundingVolumePositionAttribute`.
  ///
  /// An object of this type can be created from `SphereGeometry` with `object.slots().set_bounding_volume_position_attribute()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SphereGeometry` object.
  pub struct SetBoundingVolumePositionAttribute<'a>(&'a ::sphere_geometry::SphereGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetBoundingVolumePositionAttribute<'a> {
    type Arguments = (*mut ::qt_3d_render::attribute::Attribute,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBoundingVolumePositionAttribute(Qt3DRender::QAttribute*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QSphereGeometry::setRings`.
  ///
  /// An object of this type can be created from `SphereGeometry` with `object.slots().set_rings()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SphereGeometry` object.
  pub struct SetRings<'a>(&'a ::sphere_geometry::SphereGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetRings<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRings(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QSphereGeometry::setRadius`.
  ///
  /// An object of this type can be created from `SphereGeometry` with `object.slots().set_radius()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `SphereGeometry` object.
  pub struct SetRadius<'a>(&'a ::sphere_geometry::SphereGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetRadius<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setRadius(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QSphereGeometry::setSlices`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_slices(&self) -> SetSlices {
      SetSlices(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QSphereGeometry::setGenerateTangents`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_generate_tangents(&self) -> SetGenerateTangents {
      SetGenerateTangents(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QSphereGeometry::setBoundingVolumePositionAttribute`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bounding_volume_position_attribute(&self) -> SetBoundingVolumePositionAttribute {
      SetBoundingVolumePositionAttribute(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QSphereGeometry::setRings`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_rings(&self) -> SetRings {
      SetRings(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QSphereGeometry::setRadius`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_radius(&self) -> SetRadius {
      SetRadius(self.0)
    }
  }
  impl ::sphere_geometry::SphereGeometry {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::sphere_geometry::SphereGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::sphere_geometry::SphereGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::sphere_geometry::SphereGeometry as *mut ::sphere_geometry::SphereGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry::Geometry> for ::sphere_geometry::SphereGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::sphere_geometry::SphereGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::sphere_geometry::SphereGeometry as *mut ::sphere_geometry::SphereGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::sphere_geometry::SphereGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_QObject_ptr(self as *mut ::sphere_geometry::SphereGeometry)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_QObject_ptr(self as *const ::sphere_geometry::SphereGeometry as *mut ::sphere_geometry::SphereGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::sphere_geometry::SphereGeometry> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::sphere_geometry::SphereGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::sphere_geometry::SphereGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::sphere_geometry::SphereGeometry> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::sphere_geometry::SphereGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::sphere_geometry::SphereGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::sphere_geometry::SphereGeometry> for ::qt_3d_render::geometry::Geometry {
  unsafe fn static_cast_mut(&mut self) -> &mut ::sphere_geometry::SphereGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_Qt3DRender_QGeometry(self as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::sphere_geometry::SphereGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DExtras_QSphereGeometry_ptr_Qt3DRender_QGeometry(self as *const ::qt_3d_render::geometry::Geometry as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::sphere_geometry::SphereGeometry {
  type Target = ::qt_3d_render::geometry::Geometry;
  fn deref(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::sphere_geometry::SphereGeometry as *mut ::sphere_geometry::SphereGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::sphere_geometry::SphereGeometry {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QSphereGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::sphere_geometry::SphereGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
