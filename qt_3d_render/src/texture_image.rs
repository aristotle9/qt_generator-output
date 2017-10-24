/// C++ type: <span style='color: green;'>```Qt3DRender::QTextureImage::Status```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Status {
  /// C++ enum variant: <span style='color: green;'>```None = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```Loading = 1```</span>
  Loading = 1,
  /// C++ enum variant: <span style='color: green;'>```Ready = 2```</span>
  Ready = 2,
  /// C++ enum variant: <span style='color: green;'>```Error = 3```</span>
  Error = 3,
}

/// C++ type: <span style='color: green;'>```Qt3DRender::QTextureImage```</span>
#[repr(C)]
pub struct TextureImage(u8);

impl TextureImage {
  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QTextureImage::isMirrored() const```</span>
  ///
  ///
  pub fn is_mirrored(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_isMirrored(self as *const ::texture_image::TextureImage) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QTextureImage::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_metaObject(self as *const ::texture_image::TextureImage) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTextureImage::QTextureImage()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::texture_image::TextureImage> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTextureImage::QTextureImage(Qt3DCore::QNode* parent = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(parent: *mut ::qt_3d_core::node::Node)
                           -> ::cpp_utils::CppBox<::texture_image::TextureImage> {
    let ffi_result = ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_new_parent(parent);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QTextureImage::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_qt_metacall(self as *mut ::texture_image::TextureImage,
                                                               arg1 as *const ::qt_core::meta_object::Call,
                                                               arg2,
                                                               arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QTextureImage::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_qt_metacast(self as *mut ::texture_image::TextureImage, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QTextureImage::setMirrored(bool mirrored)```</span>
  ///
  ///
  pub fn set_mirrored(&mut self, mirrored: bool) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_setMirrored(self as *mut ::texture_image::TextureImage, mirrored)
    }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QTextureImage::setSource(const QUrl& source)```</span>
  ///
  ///
  pub fn set_source(&mut self, source: &::qt_core::url::Url) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_setSource(self as *mut ::texture_image::TextureImage,
                                                               source as *const ::qt_core::url::Url)
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl Qt3DRender::QTextureImage::source() const```</span>
  ///
  ///
  pub fn source(&self) -> ::qt_core::url::Url {
    {
      let mut object: ::qt_core::url::Url =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_source_to_output(self as *const ::texture_image::TextureImage,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureImage::Status Qt3DRender::QTextureImage::status() const```</span>
  ///
  ///
  pub fn status(&self) -> ::texture_image::Status {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_status(self as *const ::texture_image::TextureImage) }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QTextureImage::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QTextureImage::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }
}

impl ::cpp_utils::CppDeletable for ::texture_image::TextureImage {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QTextureImage_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `TextureImage`.
  pub struct Signals<'a>(&'a ::texture_image::TextureImage);
  /// Represents a built-in Qt signal `Qt3DRender::QTextureImage::sourceChanged`.
  ///
  /// An object of this type can be created from `TextureImage` with `object.signals().source_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureImage` object.
  pub struct SourceChanged<'a>(&'a ::texture_image::TextureImage);
  impl<'a> ::qt_core::connection::Receiver for SourceChanged<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sourceChanged(const QUrl&)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SourceChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureImage::faceChanged`.
  ///
  /// An object of this type can be created from `TextureImage` with `object.signals().face_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureImage` object.
  pub struct FaceChanged<'a>(&'a ::texture_image::TextureImage);
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
  /// Represents a built-in Qt signal `Qt3DRender::QTextureImage::layerChanged`.
  ///
  /// An object of this type can be created from `TextureImage` with `object.signals().layer_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureImage` object.
  pub struct LayerChanged<'a>(&'a ::texture_image::TextureImage);
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
  /// Represents a built-in Qt signal `Qt3DRender::QTextureImage::statusChanged`.
  ///
  /// An object of this type can be created from `TextureImage` with `object.signals().status_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureImage` object.
  pub struct StatusChanged<'a>(&'a ::texture_image::TextureImage);
  impl<'a> ::qt_core::connection::Receiver for StatusChanged<'a> {
    type Arguments = (::texture_image::Status,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2statusChanged(Qt3DRender::QTextureImage::Status)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for StatusChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QTextureImage::mirroredChanged`.
  ///
  /// An object of this type can be created from `TextureImage` with `object.signals().mirrored_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureImage` object.
  pub struct MirroredChanged<'a>(&'a ::texture_image::TextureImage);
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
  /// Represents a built-in Qt signal `Qt3DRender::QTextureImage::mipLevelChanged`.
  ///
  /// An object of this type can be created from `TextureImage` with `object.signals().mip_level_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureImage` object.
  pub struct MipLevelChanged<'a>(&'a ::texture_image::TextureImage);
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
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureImage::sourceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn source_changed(&self) -> SourceChanged {
      SourceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureImage::faceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn face_changed(&self) -> FaceChanged {
      FaceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureImage::layerChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layer_changed(&self) -> LayerChanged {
      LayerChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureImage::statusChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn status_changed(&self) -> StatusChanged {
      StatusChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureImage::mirroredChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mirrored_changed(&self) -> MirroredChanged {
      MirroredChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QTextureImage::mipLevelChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mip_level_changed(&self) -> MipLevelChanged {
      MipLevelChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `TextureImage`.
  pub struct Slots<'a>(&'a ::texture_image::TextureImage);
  /// Represents a built-in Qt slot `Qt3DRender::QTextureImage::setMirrored`.
  ///
  /// An object of this type can be created from `TextureImage` with `object.slots().set_mirrored()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureImage` object.
  pub struct SetMirrored<'a>(&'a ::texture_image::TextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetMirrored<'a> {
    type Arguments = (bool,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMirrored(bool)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureImage::setLayer`.
  ///
  /// An object of this type can be created from `TextureImage` with `object.slots().set_layer()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureImage` object.
  pub struct SetLayer<'a>(&'a ::texture_image::TextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetLayer<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLayer(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureImage::setSource`.
  ///
  /// An object of this type can be created from `TextureImage` with `object.slots().set_source()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureImage` object.
  pub struct SetSource<'a>(&'a ::texture_image::TextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetSource<'a> {
    type Arguments = (&'static ::qt_core::url::Url,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSource(const QUrl&)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureImage::setFace`.
  ///
  /// An object of this type can be created from `TextureImage` with `object.slots().set_face()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureImage` object.
  pub struct SetFace<'a>(&'a ::texture_image::TextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetFace<'a> {
    type Arguments = (&'static ::abstract_texture::CubeMapFace,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFace(Qt3DRender::QAbstractTexture::CubeMapFace)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QTextureImage::setMipLevel`.
  ///
  /// An object of this type can be created from `TextureImage` with `object.slots().set_mip_level()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `TextureImage` object.
  pub struct SetMipLevel<'a>(&'a ::texture_image::TextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetMipLevel<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMipLevel(int)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureImage::setMirrored`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_mirrored(&self) -> SetMirrored {
      SetMirrored(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureImage::setLayer`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_layer(&self) -> SetLayer {
      SetLayer(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureImage::setSource`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_source(&self) -> SetSource {
      SetSource(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureImage::setFace`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_face(&self) -> SetFace {
      SetFace(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QTextureImage::setMipLevel`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_mip_level(&self) -> SetMipLevel {
      SetMipLevel(self.0)
    }
  }
  impl ::texture_image::TextureImage {
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

impl ::cpp_utils::DynamicCast<::texture_image::TextureImage> for ::abstract_texture_image::AbstractTextureImage {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::texture_image::TextureImage> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImage_G_dynamic_cast_Qt3DRender_QTextureImage_ptr(self as *mut ::abstract_texture_image::AbstractTextureImage) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::texture_image::TextureImage> {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImage_G_dynamic_cast_Qt3DRender_QTextureImage_ptr(self as *const ::abstract_texture_image::AbstractTextureImage as *mut ::abstract_texture_image::AbstractTextureImage) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::texture_image::TextureImage {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::texture_image::TextureImage)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::texture_image::TextureImage as *mut ::texture_image::TextureImage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_texture_image::AbstractTextureImage> for ::texture_image::TextureImage {
  fn static_cast_mut(&mut self) -> &mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr(self as *mut ::texture_image::TextureImage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr(self as *const ::texture_image::TextureImage as *mut ::texture_image::TextureImage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::texture_image::TextureImage {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_QObject_ptr(self as *mut ::texture_image::TextureImage)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_QObject_ptr(self as *const ::texture_image::TextureImage as *mut ::texture_image::TextureImage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture_image::TextureImage> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture_image::TextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture_image::TextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture_image::TextureImage> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture_image::TextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture_image::TextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::texture_image::TextureImage> for ::abstract_texture_image::AbstractTextureImage {
  unsafe fn static_cast_mut(&mut self) -> &mut ::texture_image::TextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_Qt3DRender_QAbstractTextureImage(self as *mut ::abstract_texture_image::AbstractTextureImage);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::texture_image::TextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QTextureImage_ptr_Qt3DRender_QAbstractTextureImage(self as *const ::abstract_texture_image::AbstractTextureImage as *mut ::abstract_texture_image::AbstractTextureImage);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::texture_image::TextureImage {
  type Target = ::abstract_texture_image::AbstractTextureImage;
  fn deref(&self) -> &::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr(self as *const ::texture_image::TextureImage as *mut ::texture_image::TextureImage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::texture_image::TextureImage {
  fn deref_mut(&mut self) -> &mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr(self as *mut ::texture_image::TextureImage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
