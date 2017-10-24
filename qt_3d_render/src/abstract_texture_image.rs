/// C++ type: <span style='color: green;'>```Qt3DRender::QAbstractTextureImage```</span>
#[repr(C)]
pub struct AbstractTextureImage(u8);

impl AbstractTextureImage {
  /// C++ method: <span style='color: green;'>```int Qt3DRender::QAbstractTextureImage::layer() const```</span>
  ///
  ///
  pub fn layer(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTextureImage_layer(self as *const ::abstract_texture_image::AbstractTextureImage) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QAbstractTextureImage::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTextureImage_metaObject(self as *const ::abstract_texture_image::AbstractTextureImage) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QAbstractTextureImage::mipLevel() const```</span>
  ///
  ///
  pub fn mip_level(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTextureImage_mipLevel(self as *const ::abstract_texture_image::AbstractTextureImage) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QAbstractTextureImage::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTextureImage_qt_metacall(self as *mut ::abstract_texture_image::AbstractTextureImage, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QAbstractTextureImage::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTextureImage_qt_metacast(self as *mut ::abstract_texture_image::AbstractTextureImage, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTextureImage::setFace(Qt3DRender::QAbstractTexture::CubeMapFace face)```</span>
  ///
  ///
  pub fn set_face(&mut self, face: &::abstract_texture::CubeMapFace) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTextureImage_setFace(self as *mut ::abstract_texture_image::AbstractTextureImage, face as *const ::abstract_texture::CubeMapFace) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTextureImage::setLayer(int layer)```</span>
  ///
  ///
  pub fn set_layer(&mut self, layer: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTextureImage_setLayer(self as *mut ::abstract_texture_image::AbstractTextureImage, layer) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QAbstractTextureImage::setMipLevel(int level)```</span>
  ///
  ///
  pub fn set_mip_level(&mut self, level: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTextureImage_setMipLevel(self as *mut ::abstract_texture_image::AbstractTextureImage, level) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAbstractTextureImage::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTextureImage_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QAbstractTextureImage::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTextureImage_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::abstract_texture_image::AbstractTextureImage {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QAbstractTextureImage_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `AbstractTextureImage`.
  pub struct Signals<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTextureImage::enabledChanged`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct EnabledChanged<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTextureImage::parentChanged`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct ParentChanged<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTextureImage::faceChanged`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.signals().face_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct FaceChanged<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
  impl<'a> ::qt_core::connection::Receiver for FaceChanged<'a> {
    type Arguments = (&'static ::abstract_texture::CubeMapFace,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2faceChanged(Qt3DRender::QAbstractTexture::CubeMapFace)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for FaceChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTextureImage::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTextureImage::layerChanged`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.signals().layer_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct LayerChanged<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
  impl<'a> ::qt_core::connection::Receiver for LayerChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2layerChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for LayerChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTextureImage::nodeDestroyed`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct NodeDestroyed<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
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
  /// Represents a built-in Qt signal `Qt3DRender::QAbstractTextureImage::mipLevelChanged`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.signals().mip_level_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct MipLevelChanged<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
  impl<'a> ::qt_core::connection::Receiver for MipLevelChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2mipLevelChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for MipLevelChanged<'a> {}
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTextureImage::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTextureImage::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTextureImage::faceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn face_changed(&self) -> FaceChanged {
      FaceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTextureImage::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTextureImage::layerChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layer_changed(&self) -> LayerChanged {
      LayerChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTextureImage::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QAbstractTextureImage::mipLevelChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mip_level_changed(&self) -> MipLevelChanged {
      MipLevelChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `AbstractTextureImage`.
  pub struct Slots<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setMipLevel`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.slots().set_mip_level()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct SetMipLevel<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetMipLevel<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMipLevel(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setFace`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.slots().set_face()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct SetFace<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetFace<'a> {
    type Arguments = (&'static ::abstract_texture::CubeMapFace,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFace(Qt3DRender::QAbstractTexture::CubeMapFace)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setLayer`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.slots().set_layer()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct SetLayer<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetLayer<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLayer(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setEnabled`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct SetEnabled<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setParent`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct SetParent<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `AbstractTextureImage` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `AbstractTextureImage` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::abstract_texture_image::AbstractTextureImage);
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
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setMipLevel`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_mip_level(&self) -> SetMipLevel {
      SetMipLevel(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setFace`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_face(&self) -> SetFace {
      SetFace(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setLayer`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_layer(&self) -> SetLayer {
      SetLayer(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QAbstractTextureImage::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
  }
  impl ::abstract_texture_image::AbstractTextureImage {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::abstract_texture_image::AbstractTextureImage {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTextureImage_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::abstract_texture_image::AbstractTextureImage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTextureImage_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::abstract_texture_image::AbstractTextureImage as *mut ::abstract_texture_image::AbstractTextureImage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::abstract_texture_image::AbstractTextureImage {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTextureImage_G_static_cast_QObject_ptr(self as *mut ::abstract_texture_image::AbstractTextureImage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTextureImage_G_static_cast_QObject_ptr(self as *const ::abstract_texture_image::AbstractTextureImage as *mut ::abstract_texture_image::AbstractTextureImage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_texture_image::AbstractTextureImage> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_texture_image::AbstractTextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::abstract_texture_image::AbstractTextureImage> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::abstract_texture_image::AbstractTextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QAbstractTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::abstract_texture_image::AbstractTextureImage {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTextureImage_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::abstract_texture_image::AbstractTextureImage as *mut ::abstract_texture_image::AbstractTextureImage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::abstract_texture_image::AbstractTextureImage {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QAbstractTextureImage_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::abstract_texture_image::AbstractTextureImage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
