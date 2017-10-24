/// C++ type: <span style='color: green;'>```Qt3DRender::QPaintedTextureImage```</span>
#[repr(C)]
pub struct PaintedTextureImage(u8);

impl PaintedTextureImage {
  /// C++ method: <span style='color: green;'>```int Qt3DRender::QPaintedTextureImage::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_height(self as *const ::painted_texture_image::PaintedTextureImage) }
  }

  /// C++ method: <span style='color: green;'>```virtual const QMetaObject* Qt3DRender::QPaintedTextureImage::metaObject() const```</span>
  ///
  ///
  pub fn meta_object(&self) -> *const ::qt_core::meta_object::MetaObject {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_metaObject(self as *const ::painted_texture_image::PaintedTextureImage) }
  }

  /// C++ method: <span style='color: green;'>```virtual int Qt3DRender::QPaintedTextureImage::qt_metacall(QMetaObject::Call arg1, int arg2, void** arg3)```</span>
  ///
  ///
  pub unsafe fn qt_metacall(&mut self,
                            arg1: &::qt_core::meta_object::Call,
                            arg2: ::libc::c_int,
                            arg3: *mut *mut ::libc::c_void)
                            -> ::libc::c_int {
    ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_qt_metacall(self as *mut ::painted_texture_image::PaintedTextureImage, arg1 as *const ::qt_core::meta_object::Call, arg2, arg3)
  }

  /// C++ method: <span style='color: green;'>```virtual void* Qt3DRender::QPaintedTextureImage::qt_metacast(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn qt_metacast(&mut self, arg1: *const ::libc::c_char) -> *mut ::libc::c_void {
    ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_qt_metacast(self as *mut ::painted_texture_image::PaintedTextureImage, arg1)
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPaintedTextureImage::setHeight(int h)```</span>
  ///
  ///
  pub fn set_height(&mut self, h: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_setHeight(self as *mut ::painted_texture_image::PaintedTextureImage, h) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPaintedTextureImage::setSize(QSize size)```</span>
  ///
  ///
  pub fn set_size(&mut self, size: &::qt_core::size::Size) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_setSize(self as *mut ::painted_texture_image::PaintedTextureImage, size as *const ::qt_core::size::Size) }
  }

  /// C++ method: <span style='color: green;'>```[slot] void Qt3DRender::QPaintedTextureImage::setWidth(int w)```</span>
  ///
  ///
  pub fn set_width(&mut self, w: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_setWidth(self as *mut ::painted_texture_image::PaintedTextureImage, w) }
  }

  /// C++ method: <span style='color: green;'>```QSize Qt3DRender::QPaintedTextureImage::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_size_to_output(self as *const ::painted_texture_image::PaintedTextureImage, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPaintedTextureImage::tr(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr(s: *const ::libc::c_char, c: *const ::libc::c_char, n: ::libc::c_int) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_tr_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString Qt3DRender::QPaintedTextureImage::trUtf8(const char* s, const char* c, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(s: *const ::libc::c_char,
                        c: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_trUtf8_to_output(s, c, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QPaintedTextureImage::update```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn update(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QPaintedTextureImage::update()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn update(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QPaintedTextureImage::update(const QRect& rect = ?)```</span>
  ///
  ///
  pub fn update<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PaintedTextureImageUpdateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int Qt3DRender::QPaintedTextureImage::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_width(self as *const ::painted_texture_image::PaintedTextureImage) }
  }
}

impl ::cpp_utils::CppDeletable for ::painted_texture_image::PaintedTextureImage {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_delete
  }
}

/// Types for accessing built-in Qt signals and slots present in this module
pub mod connection {
  use ::cpp_utils::StaticCast;
  /// Provides access to built-in Qt signals of `PaintedTextureImage`.
  pub struct Signals<'a>(&'a ::painted_texture_image::PaintedTextureImage);
  /// Represents a built-in Qt signal `Qt3DRender::QPaintedTextureImage::layerChanged`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.signals().layer_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct LayerChanged<'a>(&'a ::painted_texture_image::PaintedTextureImage);
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
  /// Represents a built-in Qt signal `Qt3DRender::QPaintedTextureImage::heightChanged`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.signals().height_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct HeightChanged<'a>(&'a ::painted_texture_image::PaintedTextureImage);
  impl<'a> ::qt_core::connection::Receiver for HeightChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2heightChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for HeightChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QPaintedTextureImage::sizeChanged`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.signals().size_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct SizeChanged<'a>(&'a ::painted_texture_image::PaintedTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SizeChanged<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2sizeChanged(QSize)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for SizeChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QPaintedTextureImage::faceChanged`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.signals().face_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct FaceChanged<'a>(&'a ::painted_texture_image::PaintedTextureImage);
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
  /// Represents a built-in Qt signal `Qt3DRender::QPaintedTextureImage::widthChanged`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.signals().width_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct WidthChanged<'a>(&'a ::painted_texture_image::PaintedTextureImage);
  impl<'a> ::qt_core::connection::Receiver for WidthChanged<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"2widthChanged(int)\0"
    }
  }
  impl<'a> ::qt_core::connection::Signal for WidthChanged<'a> {}
  /// Represents a built-in Qt signal `Qt3DRender::QPaintedTextureImage::mipLevelChanged`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.signals().mip_level_changed()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct MipLevelChanged<'a>(&'a ::painted_texture_image::PaintedTextureImage);
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
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPaintedTextureImage::layerChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn layer_changed(&self) -> LayerChanged {
      LayerChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPaintedTextureImage::heightChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn height_changed(&self) -> HeightChanged {
      HeightChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPaintedTextureImage::sizeChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn size_changed(&self) -> SizeChanged {
      SizeChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPaintedTextureImage::faceChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn face_changed(&self) -> FaceChanged {
      FaceChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPaintedTextureImage::widthChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn width_changed(&self) -> WidthChanged {
      WidthChanged(self.0)
    }
    /// Returns an object representing a built-in Qt signal `Qt3DRender::QPaintedTextureImage::mipLevelChanged`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn mip_level_changed(&self) -> MipLevelChanged {
      MipLevelChanged(self.0)
    }
  }
  /// Provides access to built-in Qt slots of `PaintedTextureImage`.
  pub struct Slots<'a>(&'a ::painted_texture_image::PaintedTextureImage);
  /// Represents a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setMipLevel`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.slots().set_mip_level()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct SetMipLevel<'a>(&'a ::painted_texture_image::PaintedTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetMipLevel<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setMipLevel(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setLayer`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.slots().set_layer()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct SetLayer<'a>(&'a ::painted_texture_image::PaintedTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetLayer<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setLayer(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setSize`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.slots().set_size()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct SetSize<'a>(&'a ::painted_texture_image::PaintedTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetSize<'a> {
    type Arguments = (&'static ::qt_core::size::Size,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setSize(QSize)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setHeight`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.slots().set_height()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct SetHeight<'a>(&'a ::painted_texture_image::PaintedTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetHeight<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setHeight(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setWidth`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.slots().set_width()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct SetWidth<'a>(&'a ::painted_texture_image::PaintedTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetWidth<'a> {
    type Arguments = (::libc::c_int,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setWidth(int)\0"
    }
  }
  /// Represents a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setFace`.
  ///
  /// An object of this type can be created from `PaintedTextureImage` with `object.slots().set_face()` and used for creating Qt connections using `qt_core::connection` API. After the connection is made, the object can (should) be dropped. The connection will remain active until sender or receiver are destroyed or until a manual disconnection is made.
  ///
  /// An object of this type contains a reference to the original `PaintedTextureImage` object.
  pub struct SetFace<'a>(&'a ::painted_texture_image::PaintedTextureImage);
  impl<'a> ::qt_core::connection::Receiver for SetFace<'a> {
    type Arguments = (&'static ::abstract_texture::CubeMapFace,);
    fn object(&self) -> &::qt_core::object::Object {
      self.0.static_cast()
    }
    fn receiver_id() -> &'static [u8] {
      b"1setFace(Qt3DRender::QAbstractTexture::CubeMapFace)\0"
    }
  }
  impl<'a> Slots<'a> {
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setMipLevel`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_mip_level(&self) -> SetMipLevel {
      SetMipLevel(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setLayer`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_layer(&self) -> SetLayer {
      SetLayer(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setSize`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_size(&self) -> SetSize {
      SetSize(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setHeight`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_height(&self) -> SetHeight {
      SetHeight(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setWidth`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_width(&self) -> SetWidth {
      SetWidth(self.0)
    }
    /// Returns an object representing a built-in Qt slot `Qt3DRender::QPaintedTextureImage::setFace`.
    ///
    /// Return value of this function can be used for creating Qt connections using `qt_core::connection` API.
    pub fn set_face(&self) -> SetFace {
      SetFace(self.0)
    }
  }
  impl ::painted_texture_image::PaintedTextureImage {
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

impl ::cpp_utils::DynamicCast<::painted_texture_image::PaintedTextureImage> for ::abstract_texture_image::AbstractTextureImage {
fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::painted_texture_image::PaintedTextureImage> {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPaintedTextureImage_G_dynamic_cast_Qt3DRender_QPaintedTextureImage_ptr(self as *mut ::abstract_texture_image::AbstractTextureImage) };
unsafe { ffi_result.as_mut() }}

fn dynamic_cast(&self) -> ::std::option::Option<&::painted_texture_image::PaintedTextureImage> {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPaintedTextureImage_G_dynamic_cast_Qt3DRender_QPaintedTextureImage_ptr(self as *const ::abstract_texture_image::AbstractTextureImage as *mut ::abstract_texture_image::AbstractTextureImage) };
unsafe { ffi_result.as_ref() }}

}

impl ::cpp_utils::StaticCast<::qt_3d_core::node::Node> for ::painted_texture_image::PaintedTextureImage {
  fn static_cast_mut(&mut self) -> &mut ::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DCore_QNode_ptr(self as *mut ::painted_texture_image::PaintedTextureImage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_3d_core::node::Node {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DCore_QNode_ptr(self as *const ::painted_texture_image::PaintedTextureImage as *mut ::painted_texture_image::PaintedTextureImage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::StaticCast<::abstract_texture_image::AbstractTextureImage> for ::painted_texture_image::PaintedTextureImage {
fn static_cast_mut(&mut self) -> &mut ::abstract_texture_image::AbstractTextureImage {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr(self as *mut ::painted_texture_image::PaintedTextureImage) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")}

fn static_cast(&self) -> &::abstract_texture_image::AbstractTextureImage {
let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr(self as *const ::painted_texture_image::PaintedTextureImage as *mut ::painted_texture_image::PaintedTextureImage) };
unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")}

}

impl ::cpp_utils::StaticCast<::qt_core::object::Object> for ::painted_texture_image::PaintedTextureImage {
  fn static_cast_mut(&mut self) -> &mut ::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_QObject_ptr(self as *mut ::painted_texture_image::PaintedTextureImage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::qt_core::object::Object {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_QObject_ptr(self as *const ::painted_texture_image::PaintedTextureImage as *mut ::painted_texture_image::PaintedTextureImage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::painted_texture_image::PaintedTextureImage> for ::qt_core::object::Object {
  unsafe fn static_cast_mut(&mut self) -> &mut ::painted_texture_image::PaintedTextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QPaintedTextureImage_ptr_QObject(self as *mut ::qt_core::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::painted_texture_image::PaintedTextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QPaintedTextureImage_ptr_QObject(self as *const ::qt_core::object::Object as *mut ::qt_core::object::Object);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::painted_texture_image::PaintedTextureImage> for ::qt_3d_core::node::Node {
  unsafe fn static_cast_mut(&mut self) -> &mut ::painted_texture_image::PaintedTextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QPaintedTextureImage_ptr_Qt3DCore_QNode(self as *mut ::qt_3d_core::node::Node);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::painted_texture_image::PaintedTextureImage {
    let ffi_result = ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QPaintedTextureImage_ptr_Qt3DCore_QNode(self as *const ::qt_3d_core::node::Node as *mut ::qt_3d_core::node::Node);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::painted_texture_image::PaintedTextureImage> for ::abstract_texture_image::AbstractTextureImage {
unsafe fn static_cast_mut(&mut self) -> &mut ::painted_texture_image::PaintedTextureImage {
let ffi_result = ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QPaintedTextureImage_ptr_Qt3DRender_QAbstractTextureImage(self as *mut ::abstract_texture_image::AbstractTextureImage);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")}

unsafe fn static_cast(&self) -> &::painted_texture_image::PaintedTextureImage {
let ffi_result = ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QPaintedTextureImage_ptr_Qt3DRender_QAbstractTextureImage(self as *const ::abstract_texture_image::AbstractTextureImage as *mut ::abstract_texture_image::AbstractTextureImage);
ffi_result.as_ref().expect("Attempted to convert null pointer to reference")}

}

impl ::std::ops::Deref for ::painted_texture_image::PaintedTextureImage {
  type Target = ::abstract_texture_image::AbstractTextureImage;
  fn deref(&self) -> &::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr(self as *const ::painted_texture_image::PaintedTextureImage as *mut ::painted_texture_image::PaintedTextureImage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::painted_texture_image::PaintedTextureImage {
  fn deref_mut(&mut self) -> &mut ::abstract_texture_image::AbstractTextureImage {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_QPaintedTextureImage_G_static_cast_Qt3DRender_QAbstractTextureImage_ptr(self as *mut ::painted_texture_image::PaintedTextureImage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PaintedTextureImage::update](../struct.PaintedTextureImage.html#method.update) method.
  pub trait PaintedTextureImageUpdateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painted_texture_image::PaintedTextureImage) -> ();
  }
  impl<'largs> PaintedTextureImageUpdateArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::painted_texture_image::PaintedTextureImage) -> () {

      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_update_no_args(original_self as *mut ::painted_texture_image::PaintedTextureImage) }
    }
  }
  impl<'largs> PaintedTextureImageUpdateArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::painted_texture_image::PaintedTextureImage) -> () {
      let rect = self;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QPaintedTextureImage_update_rect(original_self as *mut ::painted_texture_image::PaintedTextureImage, rect as *const ::qt_core::rect::Rect) }
    }
  }
}
