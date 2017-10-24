/// C++ type: <span style='color: green;'>```Qt3DExtras::QExtrudedTextGeometry```</span>
#[repr(C)]
pub struct ExtrudedTextGeometry(u8);

impl ExtrudedTextGeometry {
  /// C++ method: <span style='color: green;'>```float Qt3DExtras::QExtrudedTextGeometry::extrusionLength() const```</span>
  ///
  ///
  pub fn extrusion_length(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_extrusionLength(self as *const ::extruded_text_geometry::ExtrudedTextGeometry) }
  }

  /// C++ method: <span style='color: green;'>```QFont Qt3DExtras::QExtrudedTextGeometry::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::qt_gui::font::Font {
    {
      let mut object: ::qt_gui::font::Font =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_font_to_output(self as *const ::extruded_text_geometry::ExtrudedTextGeometry, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QExtrudedTextGeometry::indexAttribute() const```</span>
  ///
  ///
  pub fn index_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_indexAttribute(self as *const ::extruded_text_geometry::ExtrudedTextGeometry) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DExtras::QExtrudedTextGeometry::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_metaObject(self as *const ::extruded_text_geometry::ExtrudedTextGeometry) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QExtrudedTextGeometry::QExtrudedTextGeometry()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::extruded_text_geometry::ExtrudedTextGeometry> {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DExtras::QExtrudedTextGeometry::QExtrudedTextGeometry(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::extruded_text_geometry::ExtrudedTextGeometry> {
    let ffi_result = ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QExtrudedTextGeometry::normalAttribute() const```</span>
  ///
  ///
  pub fn normal_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_normalAttribute(self as *const ::extruded_text_geometry::ExtrudedTextGeometry) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DExtras::QExtrudedTextGeometry::positionAttribute() const```</span>
  ///
  ///
  pub fn position_attribute(&self) -> *mut ::qt_3d_render::attribute::Attribute {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_positionAttribute(self as *const ::extruded_text_geometry::ExtrudedTextGeometry) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DExtras::QExtrudedTextGeometry::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_qt_metacall(self as *mut ::extruded_text_geometry::ExtrudedTextGeometry, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DExtras::QExtrudedTextGeometry::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_qt_metacast(self as *mut ::extruded_text_geometry::ExtrudedTextGeometry, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QExtrudedTextGeometry::setDepth(float extrusionLength)```</span>
  ///
  ///
  pub fn set_depth(&mut self, extrusion_length: ::libc::c_float) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_setDepth(self as *mut ::extruded_text_geometry::ExtrudedTextGeometry, extrusion_length) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QExtrudedTextGeometry::setFont(const QFont& font)```</span>
  ///
  ///
  pub fn set_font(&mut self, font: &::qt_gui::font::Font) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_setFont(self as *mut ::extruded_text_geometry::ExtrudedTextGeometry, font as *const ::qt_gui::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DExtras::QExtrudedTextGeometry::setText(const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, text: &::qt_core::string::String) {
    unsafe { ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_setText(self as *mut ::extruded_text_geometry::ExtrudedTextGeometry, text as *const ::qt_core::string::String) }
  }

  /// C++ method: <span style='color: green;'>```QString Qt3DExtras::QExtrudedTextGeometry::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_text_to_output(self as *const ::extruded_text_geometry::ExtrudedTextGeometry, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QExtrudedTextGeometry::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DExtras::QExtrudedTextGeometry::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::extruded_text_geometry::ExtrudedTextGeometry {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_extras_c_Qt3DExtras_QExtrudedTextGeometry_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `ExtrudedTextGeometry`.
  pub struct Signals<'a>(&'a ::extruded_text_geometry::ExtrudedTextGeometry);
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextGeometry::boundingVolumePositionAttributeChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextGeometry` with `object.signals().bounding_volume_position_attribute_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextGeometry` object.
  pub struct BoundingVolumePositionAttributeChanged<'a>(&'a ::extruded_text_geometry::ExtrudedTextGeometry);
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
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextGeometry::textChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextGeometry` with `object.signals().text_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextGeometry` object.
  pub struct TextChanged<'a>(&'a ::extruded_text_geometry::ExtrudedTextGeometry);
  impl<'a> ::qt_core::connection::Receiver for TextChanged<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textChanged(const QString&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextGeometry::fontChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextGeometry` with `object.signals().font_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextGeometry` object.
  pub struct FontChanged<'a>(&'a ::extruded_text_geometry::ExtrudedTextGeometry);
  impl<'a> ::qt_core::connection::Receiver for FontChanged<'a> {
    type Arguments = (&'static ::qt_gui::font::Font,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2fontChanged(const QFont&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FontChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DExtras::QExtrudedTextGeometry::depthChanged`.
  ///
  /// An object of this type can be created from `ExtrudedTextGeometry` with `object.signals().depth_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextGeometry` object.
  pub struct DepthChanged<'a>(&'a ::extruded_text_geometry::ExtrudedTextGeometry);
  impl<'a> ::qt_core::connection::Receiver for DepthChanged<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2depthChanged(float)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DepthChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextGeometry::boundingVolumePositionAttributeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bounding_volume_position_attribute_changed(&self) -> BoundingVolumePositionAttributeChanged {
      BoundingVolumePositionAttributeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextGeometry::textChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn text_changed(&self) -> TextChanged {
      TextChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextGeometry::fontChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn font_changed(&self) -> FontChanged {
      FontChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DExtras::QExtrudedTextGeometry::depthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn depth_changed(&self) -> DepthChanged {
      DepthChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `ExtrudedTextGeometry`.
  pub struct Slots<'a>(&'a ::extruded_text_geometry::ExtrudedTextGeometry);
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextGeometry::setFont`.
  ///
  /// An object of this type can be created from `ExtrudedTextGeometry` with `object.slots().set_font()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextGeometry` object.
  pub struct SetFont<'a>(&'a ::extruded_text_geometry::ExtrudedTextGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetFont<'a> {
    type Arguments = (&'static ::qt_gui::font::Font,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFont(const QFont&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextGeometry::setBoundingVolumePositionAttribute`.
  ///
  /// An object of this type can be created from `ExtrudedTextGeometry` with `object.slots().set_bounding_volume_position_attribute()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextGeometry` object.
  pub struct SetBoundingVolumePositionAttribute<'a>(&'a ::extruded_text_geometry::ExtrudedTextGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetBoundingVolumePositionAttribute<'a> {
    type Arguments = (*mut ::qt_3d_render::attribute::Attribute,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBoundingVolumePositionAttribute(Qt3DRender::QAttribute*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextGeometry::setText`.
  ///
  /// An object of this type can be created from `ExtrudedTextGeometry` with `object.slots().set_text()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextGeometry` object.
  pub struct SetText<'a>(&'a ::extruded_text_geometry::ExtrudedTextGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetText<'a> {
    type Arguments = (&'static ::qt_core::string::String,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setText(const QString&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DExtras::QExtrudedTextGeometry::setDepth`.
  ///
  /// An object of this type can be created from `ExtrudedTextGeometry` with `object.slots().set_depth()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `ExtrudedTextGeometry` object.
  pub struct SetDepth<'a>(&'a ::extruded_text_geometry::ExtrudedTextGeometry);
  impl<'a> ::qt_core::connection::Receiver for SetDepth<'a> {
    type Arguments = (::libc::c_float,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDepth(float)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextGeometry::setFont`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_font(&self) -> SetFont {
      SetFont(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextGeometry::setBoundingVolumePositionAttribute`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bounding_volume_position_attribute(&self) -> SetBoundingVolumePositionAttribute {
      SetBoundingVolumePositionAttribute(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextGeometry::setText`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_text(&self) -> SetText {
      SetText(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DExtras::QExtrudedTextGeometry::setDepth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_depth(&self) -> SetDepth {
      SetDepth(self.0)
    }
  }
  impl ::extruded_text_geometry::ExtrudedTextGeometry {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::extruded_text_geometry::ExtrudedTextGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::extruded_text_geometry::ExtrudedTextGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::extruded_text_geometry::ExtrudedTextGeometry as *mut ::extruded_text_geometry::ExtrudedTextGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_render::geometry::Geometry> for ::extruded_text_geometry::ExtrudedTextGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::extruded_text_geometry::ExtrudedTextGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::extruded_text_geometry::ExtrudedTextGeometry as *mut ::extruded_text_geometry::ExtrudedTextGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::extruded_text_geometry::ExtrudedTextGeometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_QObject_ptr(self as *mut ::extruded_text_geometry::ExtrudedTextGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_QObject_ptr(self as *const ::extruded_text_geometry::ExtrudedTextGeometry as *mut ::extruded_text_geometry::ExtrudedTextGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::extruded_text_geometry::ExtrudedTextGeometry> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::extruded_text_geometry::ExtrudedTextGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DExtras_QExtrudedTextGeometry_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::extruded_text_geometry::ExtrudedTextGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DExtras_QExtrudedTextGeometry_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::extruded_text_geometry::ExtrudedTextGeometry> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::extruded_text_geometry::ExtrudedTextGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DExtras_QExtrudedTextGeometry_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::extruded_text_geometry::ExtrudedTextGeometry {
    let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DExtras_QExtrudedTextGeometry_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::extruded_text_geometry::ExtrudedTextGeometry> for ::qt_3d_render::geometry::Geometry {
unsafe fn static_cast_mut(&mut self) -> &mut ::extruded_text_geometry::ExtrudedTextGeometry {
let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DExtras_QExtrudedTextGeometry_ptr_Qt3DRender_QGeometry(self as *mut ::qt_3d_render::geometry::Geometry);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::extruded_text_geometry::ExtrudedTextGeometry {
let ffi_result = ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DExtras_QExtrudedTextGeometry_ptr_Qt3DRender_QGeometry(self as *const ::qt_3d_render::geometry::Geometry as *mut ::qt_3d_render::geometry::Geometry);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::extruded_text_geometry::ExtrudedTextGeometry {
  type Target = ::qt_3d_render::geometry::Geometry;
  fn deref(&self) -> &::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *const ::extruded_text_geometry::ExtrudedTextGeometry as *mut ::extruded_text_geometry::ExtrudedTextGeometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::extruded_text_geometry::ExtrudedTextGeometry {
  fn deref_mut(&mut self) -> &mut ::qt_3d_render::geometry::Geometry {
    let ffi_result = unsafe { ::ffi::qt_3d_extras_c_QExtrudedTextGeometry_G_static_cast_Qt3DRender_QGeometry_ptr(self as *mut ::extruded_text_geometry::ExtrudedTextGeometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
