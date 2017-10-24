/// C++ type: <span style='color: green;'>```Qt3DExtras::QPlaneGeometry```</span>
#[repr(C)]
pub struct PlaneGeometry(u8);

impl PlaneGeometry {
  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QPlaneGeometry::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_height(self as *const ::plane_geometry::PlaneGeometry) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QPlaneGeometry::indexAttribute() const```</span>
  ///
  ///
  pub fn index_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_indexAttribute(self as *const ::plane_geometry::PlaneGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QPlaneGeometry::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_metaObject(self as *const ::plane_geometry::PlaneGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DExtras::QPlaneGeometry::mirrored() const```</span>
  ///
  ///
  pub fn mirrored(&self) -> bool {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_mirrored(self as *const ::plane_geometry::PlaneGeometry) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QPlaneGeometry::QPlaneGeometry()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::plane_geometry::PlaneGeometry> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QPlaneGeometry::QPlaneGeometry(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::plane_geometry::PlaneGeometry> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QPlaneGeometry::normalAttribute() const```</span>
  ///
  ///
  pub fn normal_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_normalAttribute(self as *const ::plane_geometry::PlaneGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QPlaneGeometry::positionAttribute() const```</span>
  ///
  ///
  pub fn position_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_positionAttribute(self as *const ::plane_geometry::PlaneGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QPlaneGeometry::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_qt_metacall(self as *mut ::plane_geometry::PlaneGeometry,
                                                                arg1 as *const ::qt_core::meta_object::Call,
                                                                arg2,
                                                                arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QPlaneGeometry::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_qt_metacast(self as *mut ::plane_geometry::PlaneGeometry, arg1)
  }

  /// C++ method: <span style='color: green;'>```QSize Qt3DExtras::QPlaneGeometry::resolution() const```</span>
  ///
  ///
  pub fn resolution(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_resolution_to_output(self as *const ::plane_geometry::PlaneGeometry, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPlaneGeometry::setHeight(float height)```</span>
  ///
  ///
  pub fn set_height(&mut self, height: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_setHeight(self as *mut ::plane_geometry::PlaneGeometry, height)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPlaneGeometry::setMirrored(bool mirrored)```</span>
  ///
  ///
  pub fn set_mirrored(&mut self, mirrored: bool) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_setMirrored(self as *mut ::plane_geometry::PlaneGeometry,
                                                                  mirrored)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPlaneGeometry::setResolution(const QSize& resolution)```</span>
  ///
  ///
  pub fn set_resolution(&mut self, resolution: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_setResolution(self as *mut ::plane_geometry::PlaneGeometry,
                                                                    resolution as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QPlaneGeometry::setWidth(float width)```</span>
  ///
  ///
  pub fn set_width(&mut self, width: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_setWidth(self as *mut ::plane_geometry::PlaneGeometry, width)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QPlaneGeometry::tangentAttribute() const```</span>
  ///
  ///
  pub fn tangent_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_tangentAttribute(self as *const ::plane_geometry::PlaneGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QPlaneGeometry::texCoordAttribute() const```</span>
  ///
  ///
  pub fn tex_coord_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_texCoordAttribute(self as *const ::plane_geometry::PlaneGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QPlaneGeometry::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QPlaneGeometry::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QPlaneGeometry::updateIndices()```</span>
  ///
  ///
  pub fn update_indices(&mut self) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_updateIndices(self as *mut ::plane_geometry::PlaneGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DExtras::QPlaneGeometry::updateVertices()```</span>
  ///
  ///
  pub fn update_vertices(&mut self) {
    unsafe {
      ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_updateVertices(self as *mut ::plane_geometry::PlaneGeometry)
    }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QPlaneGeometry::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_width(self as *const ::plane_geometry::PlaneGeometry) }
  }
}

impl ::cpp_utils::CppDeletable for ::plane_geometry::PlaneGeometry {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QPlaneGeometry_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PlaneGeometry`.
  pub struct Signals<'a>(&'a ::plane_geometry::PlaneGeometry);
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneGeometry::boundingVolumePositionAttributeChanged`.
  ///
  /// An object of this type can be created from `PlaneGeometry` with `object.signals().bounding_volume_position_attribute_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneGeometry` object.
  pub struct BoundingVolumePositionAttributeChanged<'a>(&'a ::plane_geometry::PlaneGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneGeometry::mirroredChanged`.
  ///
  /// An object of this type can be created from `PlaneGeometry` with `object.signals().mirrored_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneGeometry` object.
  pub struct MirroredChanged<'a>(&'a ::plane_geometry::PlaneGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneGeometry::resolutionChanged`.
  ///
  /// An object of this type can be created from `PlaneGeometry` with `object.signals().resolution_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneGeometry` object.
  pub struct ResolutionChanged<'a>(&'a ::plane_geometry::PlaneGeometry);
  impl<'a> ::qt_core::connection::Receiver for ResolutionChanged<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2resolutionChanged(const QSize&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ResolutionChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneGeometry::widthChanged`.
  ///
  /// An object of this type can be created from `PlaneGeometry` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneGeometry` object.
  pub struct WidthChanged<'a>(&'a ::plane_geometry::PlaneGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QPlaneGeometry::heightChanged`.
  ///
  /// An object of this type can be created from `PlaneGeometry` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneGeometry` object.
  pub struct HeightChanged<'a>(&'a ::plane_geometry::PlaneGeometry);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneGeometry::boundingVolumePositionAttributeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bounding_volume_position_attribute_changed(&self) -> BoundingVolumePositionAttributeChanged {
      BoundingVolumePositionAttributeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneGeometry::mirroredChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mirrored_changed(&self) -> MirroredChanged {
      MirroredChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneGeometry::resolutionChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn resolution_changed(&self) -> ResolutionChanged {
      ResolutionChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneGeometry::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QPlaneGeometry::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PlaneGeometry`.
  pub struct Slots<'a>(&'a ::plane_geometry::PlaneGeometry);
  /// Represents a built-in Qt slot `Qt3DExtras::QPlaneGeometry::setResolution`.
  ///
  /// An object of this type can be created from `PlaneGeometry` with `object.slots().set_resolution()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneGeometry` object.
  pub struct SetResolution<'a>(&'a ::plane_geometry::PlaneGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetResolution<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setResolution(const QSize&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPlaneGeometry::setWidth`.
  ///
  /// An object of this type can be created from `PlaneGeometry` with `object.slots().set_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneGeometry` object.
  pub struct SetWidth<'a>(&'a ::plane_geometry::PlaneGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetWidth<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWidth(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPlaneGeometry::setBoundingVolumePositionAttribute`.
  ///
  /// An object of this type can be created from `PlaneGeometry` with `object.slots().set_bounding_volume_position_attribute()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneGeometry` object.
  pub struct SetBoundingVolumePositionAttribute<'a>(&'a ::plane_geometry::PlaneGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetBoundingVolumePositionAttribute<'a> {
    type Arguments = (*mut ::qt_3d_render::attribute::Attribute,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBoundingVolumePositionAttribute(Qt3DRender::QAttribute*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPlaneGeometry::setHeight`.
  ///
  /// An object of this type can be created from `PlaneGeometry` with `object.slots().set_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneGeometry` object.
  pub struct SetHeight<'a>(&'a ::plane_geometry::PlaneGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetHeight<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHeight(float)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QPlaneGeometry::setMirrored`.
  ///
  /// An object of this type can be created from `PlaneGeometry` with `object.slots().set_mirrored()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PlaneGeometry` object.
  pub struct SetMirrored<'a>(&'a ::plane_geometry::PlaneGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetMirrored<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMirrored(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPlaneGeometry::setResolution`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_resolution(&self) -> SetResolution {
      SetResolution(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPlaneGeometry::setWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_width(&self) -> SetWidth {
      SetWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPlaneGeometry::setBoundingVolumePositionAttribute`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bounding_volume_position_attribute(&self) -> SetBoundingVolumePositionAttribute {
      SetBoundingVolumePositionAttribute(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPlaneGeometry::setHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_height(&self) -> SetHeight {
      SetHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QPlaneGeometry::setMirrored`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_mirrored(&self) -> SetMirrored {
      SetMirrored(self.0)
    }
  }
  impl ::plane_geometry::PlaneGeometry {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::plane_geometry::PlaneGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::plane_geometry::PlaneGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::plane_geometry::PlaneGeometry as *mut ::plane_geometry::PlaneGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry::Geometry> for ::plane_geometry::PlaneGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::plane_geometry::PlaneGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::plane_geometry::PlaneGeometry as *mut ::plane_geometry::PlaneGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::plane_geometry::PlaneGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_QObject_ptr(self as *mut ::plane_geometry::PlaneGeometry)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_QObject_ptr(self as *const ::plane_geometry::PlaneGeometry as *mut ::plane_geometry::PlaneGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plane_geometry::PlaneGeometry> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plane_geometry::PlaneGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DExtras_QPlaneGeometry_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plane_geometry::PlaneGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DExtras_QPlaneGeometry_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plane_geometry::PlaneGeometry> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plane_geometry::PlaneGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DExtras_QPlaneGeometry_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plane_geometry::PlaneGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DExtras_QPlaneGeometry_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::plane_geometry::PlaneGeometry> for ::qt_3d_render::geometry::Geometry {
  unsafe fn static_cast_mut(&mut self) -> &mut ::plane_geometry::PlaneGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DExtras_QPlaneGeometry_ptr_Qt3DRender_QGeometry(self as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::plane_geometry::PlaneGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DExtras_QPlaneGeometry_ptr_Qt3DRender_QGeometry(self as *const ::qt_3d_render::geometry::Geometry as *mut ::qt_3d_render::geometry::Geometry);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::plane_geometry::PlaneGeometry {
  type Target = ::qt_3d_render::geometry::Geometry;
  fn deref(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::plane_geometry::PlaneGeometry as *mut ::plane_geometry::PlaneGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::plane_geometry::PlaneGeometry {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QPlaneGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::plane_geometry::PlaneGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
