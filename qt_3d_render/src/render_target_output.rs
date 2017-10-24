/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AttachmentPoint {
  /// C++ enum variant: <span style='color: green;'>```Color0 = 0```</span>
  Color0 = 0,
  /// C++ enum variant: <span style='color: green;'>```Color1 = 1```</span>
  Color1 = 1,
  /// C++ enum variant: <span style='color: green;'>```Color2 = 2```</span>
  Color2 = 2,
  /// C++ enum variant: <span style='color: green;'>```Color3 = 3```</span>
  Color3 = 3,
  /// C++ enum variant: <span style='color: green;'>```Color4 = 4```</span>
  Color4 = 4,
  /// C++ enum variant: <span style='color: green;'>```Color5 = 5```</span>
  Color5 = 5,
  /// C++ enum variant: <span style='color: green;'>```Color6 = 6```</span>
  Color6 = 6,
  /// C++ enum variant: <span style='color: green;'>```Color7 = 7```</span>
  Color7 = 7,
  /// C++ enum variant: <span style='color: green;'>```Color8 = 8```</span>
  Color8 = 8,
  /// C++ enum variant: <span style='color: green;'>```Color9 = 9```</span>
  Color9 = 9,
  /// C++ enum variant: <span style='color: green;'>```Color10 = 10```</span>
  Color10 = 10,
  /// C++ enum variant: <span style='color: green;'>```Color11 = 11```</span>
  Color11 = 11,
  /// C++ enum variant: <span style='color: green;'>```Color12 = 12```</span>
  Color12 = 12,
  /// C++ enum variant: <span style='color: green;'>```Color13 = 13```</span>
  Color13 = 13,
  /// C++ enum variant: <span style='color: green;'>```Color14 = 14```</span>
  Color14 = 14,
  /// C++ enum variant: <span style='color: green;'>```Color15 = 15```</span>
  Color15 = 15,
  /// C++ enum variant: <span style='color: green;'>```Depth = 16```</span>
  Depth = 16,
  /// C++ enum variant: <span style='color: green;'>```Stencil = 17```</span>
  Stencil = 17,
  /// C++ enum variant: <span style='color: green;'>```DepthStencil = 18```</span>
  DepthStencil = 18,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput```</span>
#[repr(C)]
pub struct RenderTargetOutput(u8);

impl RenderTargetOutput {
  /// C++ method: <span style='color: green;'>```Qt3DRender::QRenderTargetOutput::AttachmentPoint Qt3DRender::QRenderTargetOutput::attachmentPoint() const```</span>
  ///
  ///
  pub fn attachment_point(&self) -> ::render_target_output::AttachmentPoint {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_attachmentPoint(self as *const ::render_target_output::RenderTargetOutput) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QRenderTargetOutput::layer() const```</span>
  ///
  ///
  pub fn layer(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_layer(self as *const ::render_target_output::RenderTargetOutput) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QRenderTargetOutput::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_metaObject(self as *const ::render_target_output::RenderTargetOutput) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QRenderTargetOutput::mipLevel() const```</span>
  ///
  ///
  pub fn mip_level(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_mipLevel(self as *const ::render_target_output::RenderTargetOutput) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderTargetOutput::QRenderTargetOutput()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::render_target_output::RenderTargetOutput> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QRenderTargetOutput::QRenderTargetOutput(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::render_target_output::RenderTargetOutput> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QRenderTargetOutput::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_qt_metacall(self as *mut ::render_target_output::RenderTargetOutput, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QRenderTargetOutput::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_qt_metacast(self as *mut ::render_target_output::RenderTargetOutput, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderTargetOutput::setAttachmentPoint(Qt3DRender::QRenderTargetOutput::AttachmentPoint attachmentPoint)```</span>
  ///
  ///
  pub fn set_attachment_point(&mut self, attachment_point: ::render_target_output::AttachmentPoint) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setAttachmentPoint(self as *mut ::render_target_output::RenderTargetOutput, attachment_point) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderTargetOutput::setFace(Qt3DRender::QAbstractTexture::CubeMapFace face)```</span>
  ///
  ///
  pub fn set_face(&mut self, face: &::abstract_texture::CubeMapFace) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setFace(self as *mut ::render_target_output::RenderTargetOutput, face as *const ::abstract_texture::CubeMapFace) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderTargetOutput::setLayer(int layer)```</span>
  ///
  ///
  pub fn set_layer(&mut self, layer: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setLayer(self as *mut ::render_target_output::RenderTargetOutput, layer) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderTargetOutput::setMipLevel(int level)```</span>
  ///
  ///
  pub fn set_mip_level(&mut self, level: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setMipLevel(self as *mut ::render_target_output::RenderTargetOutput, level) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QRenderTargetOutput::setTexture(Qt3DRender::QAbstractTexture* texture)```</span>
  ///
  ///
  pub unsafe fn set_texture(&mut self, texture: *mut ::abstract_texture::AbstractTexture) {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_setTexture(self as *mut ::render_target_output::RenderTargetOutput, texture)
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QAbstractTexture* Qt3DRender::QRenderTargetOutput::texture() const```</span>
  ///
  ///
  pub fn texture(&self) -> *mut ::abstract_texture::AbstractTexture {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_texture(self as *const ::render_target_output::RenderTargetOutput) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderTargetOutput::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QRenderTargetOutput::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::render_target_output::RenderTargetOutput {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QRenderTargetOutput_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `RenderTargetOutput`.
  pub struct Signals<'a>(&'a ::render_target_output::RenderTargetOutput);
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTargetOutput::parentChanged`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.signals().parent_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct ParentChanged<'a>(&'a ::render_target_output::RenderTargetOutput);
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
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTargetOutput::defaultPropertyTrackingModeChanged`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.signals().default_property_tracking_mode_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct DefaultPropertyTrackingModeChanged<'a>(&'a ::render_target_output::RenderTargetOutput);
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
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTargetOutput::nodeDestroyed`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.signals().node_destroyed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct NodeDestroyed<'a>(&'a ::render_target_output::RenderTargetOutput);
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
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTargetOutput::enabledChanged`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.signals().enabled_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct EnabledChanged<'a>(&'a ::render_target_output::RenderTargetOutput);
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
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTargetOutput::layerChanged`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.signals().layer_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct LayerChanged<'a>(&'a ::render_target_output::RenderTargetOutput);
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
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTargetOutput::mipLevelChanged`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.signals().mip_level_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct MipLevelChanged<'a>(&'a ::render_target_output::RenderTargetOutput);
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
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTargetOutput::attachmentPointChanged`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.signals().attachment_point_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct AttachmentPointChanged<'a>(&'a ::render_target_output::RenderTargetOutput);
  impl<'a> ::qt_core::connection::Receiver for AttachmentPointChanged<'a> {
    type Arguments = (::render_target_output::AttachmentPoint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2attachmentPointChanged(Qt3DRender::QRenderTargetOutput::AttachmentPoint)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for AttachmentPointChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTargetOutput::textureChanged`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.signals().texture_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct TextureChanged<'a>(&'a ::render_target_output::RenderTargetOutput);
  impl<'a> ::qt_core::connection::Receiver for TextureChanged<'a> {
    type Arguments = (*mut ::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2textureChanged(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for TextureChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QRenderTargetOutput::faceChanged`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.signals().face_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct FaceChanged<'a>(&'a ::render_target_output::RenderTargetOutput);
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
  impl<'a> Signals<'a> {
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTargetOutput::parentChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn parent_changed(&self) -> ParentChanged {
      ParentChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTargetOutput::defaultPropertyTrackingModeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn default_property_tracking_mode_changed(&self) -> DefaultPropertyTrackingModeChanged {
      DefaultPropertyTrackingModeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTargetOutput::nodeDestroyed`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn node_destroyed(&self) -> NodeDestroyed {
      NodeDestroyed(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTargetOutput::enabledChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn enabled_changed(&self) -> EnabledChanged {
      EnabledChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTargetOutput::layerChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layer_changed(&self) -> LayerChanged {
      LayerChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTargetOutput::mipLevelChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mip_level_changed(&self) -> MipLevelChanged {
      MipLevelChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTargetOutput::attachmentPointChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn attachment_point_changed(&self) -> AttachmentPointChanged {
      AttachmentPointChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTargetOutput::textureChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn texture_changed(&self) -> TextureChanged {
      TextureChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QRenderTargetOutput::faceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn face_changed(&self) -> FaceChanged {
      FaceChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `RenderTargetOutput`.
  pub struct Slots<'a>(&'a ::render_target_output::RenderTargetOutput);
  /// Represents a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setFace`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.slots().set_face()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct SetFace<'a>(&'a ::render_target_output::RenderTargetOutput);
  impl<'a> ::qt_core::connection::Receiver for SetFace<'a> {
    type Arguments = (&'static ::abstract_texture::CubeMapFace,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFace(Qt3DRender::QAbstractTexture::CubeMapFace)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setParent`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.slots().set_parent()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct SetParent<'a>(&'a ::render_target_output::RenderTargetOutput);
  impl<'a> ::qt_core::connection::Receiver for SetParent<'a> {
    type Arguments = (*mut ::qt_3d_core::node::Node,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setParent(Qt3DCore::QNode*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setLayer`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.slots().set_layer()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct SetLayer<'a>(&'a ::render_target_output::RenderTargetOutput);
  impl<'a> ::qt_core::connection::Receiver for SetLayer<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLayer(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setAttachmentPoint`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.slots().set_attachment_point()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct SetAttachmentPoint<'a>(&'a ::render_target_output::RenderTargetOutput);
  impl<'a> ::qt_core::connection::Receiver for SetAttachmentPoint<'a> {
    type Arguments = (::render_target_output::AttachmentPoint,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setAttachmentPoint(Qt3DRender::QRenderTargetOutput::AttachmentPoint)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setMipLevel`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.slots().set_mip_level()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct SetMipLevel<'a>(&'a ::render_target_output::RenderTargetOutput);
  impl<'a> ::qt_core::connection::Receiver for SetMipLevel<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMipLevel(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setTexture`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.slots().set_texture()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct SetTexture<'a>(&'a ::render_target_output::RenderTargetOutput);
  impl<'a> ::qt_core::connection::Receiver for SetTexture<'a> {
    type Arguments = (*mut ::abstract_texture::AbstractTexture,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setTexture(Qt3DRender::QAbstractTexture*)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setDefaultPropertyTrackingMode`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.slots().set_default_property_tracking_mode()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct SetDefaultPropertyTrackingMode<'a>(&'a ::render_target_output::RenderTargetOutput);
  impl<'a> ::qt_core::connection::Receiver for SetDefaultPropertyTrackingMode<'a> {
    type Arguments = (::qt_3d_core::node::PropertyTrackingMode,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setDefaultPropertyTrackingMode(Qt3DCore::QNode::PropertyTrackingMode)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setEnabled`.
  ///
  /// An object of this type can be created from `RenderTargetOutput` with `object.slots().set_enabled()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `RenderTargetOutput` object.
  pub struct SetEnabled<'a>(&'a ::render_target_output::RenderTargetOutput);
  impl<'a> ::qt_core::connection::Receiver for SetEnabled<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setEnabled(bool)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setFace`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_face(&self) -> SetFace {
      SetFace(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setParent`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_parent(&self) -> SetParent {
      SetParent(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setLayer`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_layer(&self) -> SetLayer {
      SetLayer(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setAttachmentPoint`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_attachment_point(&self) -> SetAttachmentPoint {
      SetAttachmentPoint(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setMipLevel`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_mip_level(&self) -> SetMipLevel {
      SetMipLevel(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setTexture`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_texture(&self) -> SetTexture {
      SetTexture(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setDefaultPropertyTrackingMode`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_default_property_tracking_mode(&self) -> SetDefaultPropertyTrackingMode {
      SetDefaultPropertyTrackingMode(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QRenderTargetOutput::setEnabled`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_enabled(&self) -> SetEnabled {
      SetEnabled(self.0)
    }
  }
  impl ::render_target_output::RenderTargetOutput {
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

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::render_target_output::RenderTargetOutput {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::render_target_output::RenderTargetOutput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::render_target_output::RenderTargetOutput as *mut ::render_target_output::RenderTargetOutput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::render_target_output::RenderTargetOutput {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetOutput_G_static_cast_QObject_ptr(self as *mut ::render_target_output::RenderTargetOutput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetOutput_G_static_cast_QObject_ptr(self as *const ::render_target_output::RenderTargetOutput as *mut ::render_target_output::RenderTargetOutput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_target_output::RenderTargetOutput> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_target_output::RenderTargetOutput {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DRender_QRenderTargetOutput_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_target_output::RenderTargetOutput {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DRender_QRenderTargetOutput_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::render_target_output::RenderTargetOutput> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::render_target_output::RenderTargetOutput {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DRender_QRenderTargetOutput_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::render_target_output::RenderTargetOutput {
    let ffi_result = ::ffi::qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DRender_QRenderTargetOutput_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::render_target_output::RenderTargetOutput {
  type Target = ::qt_3d_core::node::Node;
  fn deref(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::render_target_output::RenderTargetOutput as *mut ::render_target_output::RenderTargetOutput) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::render_target_output::RenderTargetOutput {
  fn deref_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QRenderTargetOutput_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::render_target_output::RenderTargetOutput) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
