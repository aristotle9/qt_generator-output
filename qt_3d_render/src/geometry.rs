/// C++ type: <span style='color: green;'>```Qt3DRender::QGeometry```</span>
#[repr(C)]
pub struct Geometry(u8);

impl Geometry {
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QGeometry::addAttribute(Qt3DRender::QAttribute* attribute)```</span>
  ///
  ///
  pub unsafe fn add_attribute(&mut self, attribute: *mut ::attribute::Attribute) {
    ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_addAttribute(self as *mut ::geometry::Geometry, attribute)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DRender::QAttribute*> Qt3DRender::QGeometry::attributes() const```</span>
  ///
  ///
  pub fn attributes(&self) -> ::vector::VectorAttributeMutPtr {
    {
      let mut object: ::vector::VectorAttributeMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_attributes_to_output(self as *const ::geometry::Geometry,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAttribute* Qt3DRender::QGeometry::boundingVolumePositionAttribute() const```</span>
  ///
  ///
  pub fn bounding_volume_position_attribute(&self) -> *mut ::attribute::Attribute {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_boundingVolumePositionAttribute(self as *const ::geometry::Geometry)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QGeometry::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_metaObject(self as *const ::geometry::Geometry) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QGeometry::QGeometry()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::geometry::Geometry> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QGeometry::QGeometry(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node) -> ::cpp_utils::CppBox<::geometry::Geometry> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QGeometry::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_qt_metacall(self as *mut ::geometry::Geometry,
                                                           arg1 as *const ::qt_core::meta_object::Call,
                                                           arg2,
                                                           arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QGeometry::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_qt_metacast(self as *mut ::geometry::Geometry, arg1)
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QGeometry::removeAttribute(Qt3DRender::QAttribute* attribute)```</span>
  ///
  ///
  pub unsafe fn remove_attribute(&mut self, attribute: *mut ::attribute::Attribute) {
    ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_removeAttribute(self as *mut ::geometry::Geometry, attribute)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QGeometry::setBoundingVolumePositionAttribute(Qt3DRender::QAttribute* boundingVolumePositionAttribute)```</span>
  ///
  ///
pub unsafe fn set_bounding_volume_position_attribute(&mut self, bounding_volume_position_attribute: *mut ::attribute::Attribute) {
    ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_setBoundingVolumePositionAttribute(self as *mut ::geometry::Geometry,
                                                                                  bounding_volume_position_attribute)
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QGeometry::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QGeometry::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::geometry::Geometry {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QGeometry_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `Geometry`.
  pub struct Signals<'a>(&'a ::geometry::Geometry);
  /// Represents a built-in Qt signal `Qt3DRender::QGeometry::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `Geometry` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Geometry` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::geometry::Geometry);
  impl<'a> ::qt_core::connection::Receiver for DefaultPropertyTrackingModeChanged<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2defaultPropertyTrackingModeChanged(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for DefaultPropertyTrackingModeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGeometry::enabledChanged`.
  ///
  /// An object of this type can be created from `Geometry` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Geometry` object.
  pub struct EnabledChanged<'a>(&'a ::geometry::Geometry);
  impl<'a> ::qt_core::connection::Receiver for EnabledChanged<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2enabledChanged(bool)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for EnabledChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGeometry::boundingVolumePositionAttributeChanged`.
  ///
  /// An object of this type can be created from `Geometry` with `object.signals().bounding_volume_position_attribute_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Geometry` object.
  pub struct BoundingVolumePositionAttributeChanged<'a>(&'a ::geometry::Geometry);
  impl<'a> ::qt_core::connection::Receiver for BoundingVolumePositionAttributeChanged<'a> {
    type Arguments = (*mut ::attribute::Attribute,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2boundingVolumePositionAttributeChanged(Qt3DRender::QAttribute*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for BoundingVolumePositionAttributeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGeometry::nodeDestroyed`.
  ///
  /// An object of this type can be created from `Geometry` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Geometry` object.
  pub struct NodeDestroyed<'a>(&'a ::geometry::Geometry);
  impl<'a> ::qt_core::connection::Receiver for NodeDestroyed<'a> {
    type Arguments = ();
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2nodeDestroyed()\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for NodeDestroyed<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QGeometry::parentChanged`.
  ///
  /// An object of this type can be created from `Geometry` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Geometry` object.
  pub struct ParentChanged<'a>(&'a ::geometry::Geometry);
  impl<'a> ::qt_core::connection::Receiver for ParentChanged<'a> {
    type Arguments = (*mut ::qt_core::object::Object,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2parentChanged(QObject*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for ParentChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometry::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometry::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometry::boundingVolumePositionAttributeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn bounding_volume_position_attribute_changed(&self) -> BoundingVolumePositionAttributeChanged {
      BoundingVolumePositionAttributeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometry::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QGeometry::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `Geometry`.
  pub struct Slots<'a>(&'a ::geometry::Geometry);
  /// Represents a built-in Qt slot `Qt3DRender::QGeometry::setEnabled`.
  ///
  /// An object of this type can be created from `Geometry` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Geometry` object.
  pub struct SetEnabled<'a>(&'a ::geometry::Geometry);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometry::setBoundingVolumePositionAttribute`.
  ///
  /// An object of this type can be created from `Geometry` with `object.slots().set_bounding_volume_position_attribute()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Geometry` object.
  pub struct SetBoundingVolumePositionAttribute<'a>(&'a ::geometry::Geometry);
  impl<'a> ::qt_core::connection::Receiver for SetBoundingVolumePositionAttribute<'a> {
    type Arguments = (*mut ::attribute::Attribute,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setBoundingVolumePositionAttribute(Qt3DRender::QAttribute*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometry::setParent`.
  ///
  /// An object of this type can be created from `Geometry` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Geometry` object.
  pub struct SetParent<'a>(&'a ::geometry::Geometry);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QGeometry::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `Geometry` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `Geometry` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::geometry::Geometry);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometry::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometry::setBoundingVolumePositionAttribute`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_bounding_volume_position_attribute(&self) -> SetBoundingVolumePositionAttribute {
      SetBoundingVolumePositionAttribute(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometry::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QGeometry::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
  }
  impl ::geometry::Geometry {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::geometry::Geometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::geometry::Geometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::geometry::Geometry as *mut ::geometry::Geometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::geometry::Geometry {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QGeometry_G_static_cast_QObject_ptr(self as *mut ::geometry::Geometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometry_G_static_cast_QObject_ptr(self as *const ::geometry::Geometry as *mut ::geometry::Geometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::geometry::Geometry> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::geometry::Geometry {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometry_G_static_cast_Qt3DRender_QGeometry_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::geometry::Geometry {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometry_G_static_cast_Qt3DRender_QGeometry_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::geometry::Geometry> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::geometry::Geometry {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometry_G_static_cast_Qt3DRender_QGeometry_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::geometry::Geometry {
    let ffi_result = ::ffi::qt_3d_render_c_QGeometry_G_static_cast_Qt3DRender_QGeometry_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::geometry::Geometry {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::geometry::Geometry as *mut ::geometry::Geometry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::geometry::Geometry {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe { ::ffi::qt_3d_render_c_QGeometry_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::geometry::Geometry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
