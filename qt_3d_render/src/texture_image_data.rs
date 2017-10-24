/// C++ type: <span style='color: green;'>```Qt3DRender::QTextureImageData```</span>
#[repr(C)]
pub struct TextureImageData([u8; ::type_sizes::QT_3D_RENDER_TEXTURE_IMAGE_DATA_TEXTURE_IMAGE_DATA]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextureImageData {
  unsafe fn new_uninitialized() -> TextureImageData {
    TextureImageData(::std::mem::uninitialized())
  }
}

impl TextureImageData {
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::cleanup()```</span>
  ///
  ///
  pub fn cleanup(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_cleanup(self as *mut ::texture_image_data::TextureImageData)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureImageData::data```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn data(&self, ()) -> ::qt_core::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QTextureImageData::data() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn data(&self, ::libc::c_int) -> ::qt_core::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QTextureImageData::data(int layer = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn data(&self, (::libc::c_int, ::libc::c_int)) -> ::qt_core::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QTextureImageData::data(int layer = ?, int face = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn data(&self, (::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::qt_core::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray Qt3DRender::QTextureImageData::data(int layer = ?, int face = ?, int mipmapLevel = ?) const```</span>
  ///
  ///
  pub fn data<'largs, Args>(&'largs self, args: Args) -> ::qt_core::byte_array::ByteArray
    where Args: overloading::TextureImageDataDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int Qt3DRender::QTextureImageData::depth() const```</span>
  ///
  ///
  pub fn depth(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_depth(self as *const ::texture_image_data::TextureImageData)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QTextureImageData::faces() const```</span>
  ///
  ///
  pub fn faces(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_faces(self as *const ::texture_image_data::TextureImageData)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QTextureImageData::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_height(self as *const ::texture_image_data::TextureImageData)
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QTextureImageData::isCompressed() const```</span>
  ///
  ///
  pub fn is_compressed(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_isCompressed(self as *const ::texture_image_data::TextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QTextureImageData::layers() const```</span>
  ///
  ///
  pub fn layers(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_layers(self as *const ::texture_image_data::TextureImageData)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QTextureImageData::mipLevels() const```</span>
  ///
  ///
  pub fn mip_levels(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_mipLevels(self as *const ::texture_image_data::TextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTextureImageData::QTextureImageData()```</span>
  ///
  ///
  pub fn new() -> ::texture_image_data::TextureImageData {
    {
      let mut object: ::texture_image_data::TextureImageData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureImageData& Qt3DRender::QTextureImageData::operator=(const Qt3DRender::QTextureImageData& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::texture_image_data::TextureImageData)
                             -> &'l0 mut ::texture_image_data::TextureImageData {
    let ffi_result = unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_operator_assign(self as *mut ::texture_image_data::TextureImageData, other as *const ::texture_image_data::TextureImageData) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DRender::QTextureImageData::setData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::qt_core::byte_array::ByteArray, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setData(const QByteArray& data, int blockSize)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::qt_core::byte_array::ByteArray, ::libc::c_int, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setData(const QByteArray& data, int blockSize, bool isCompressed = ?)```</span>
  ///
  ///
  pub fn set_data<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextureImageDataSetDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setDepth(int depth)```</span>
  ///
  ///
  pub fn set_depth(&mut self, depth: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setDepth(self as *mut ::texture_image_data::TextureImageData,
                                                                  depth)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setFaces(int faces)```</span>
  ///
  ///
  pub fn set_faces(&mut self, faces: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setFaces(self as *mut ::texture_image_data::TextureImageData,
                                                                  faces)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setFormat(QOpenGLTexture::TextureFormat format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::qt_gui::opengl_texture::TextureFormat) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setFormat(self as *mut ::texture_image_data::TextureImageData, format as *const ::qt_gui::opengl_texture::TextureFormat) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setHeight(int height)```</span>
  ///
  ///
  pub fn set_height(&mut self, height: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setHeight(self as *mut ::texture_image_data::TextureImageData, height)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setImage(const QImage& arg1)```</span>
  ///
  ///
  pub fn set_image(&mut self, arg1: &::qt_gui::image::Image) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setImage(self as *mut ::texture_image_data::TextureImageData,
                                                                  arg1 as *const ::qt_gui::image::Image)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setLayers(int layers)```</span>
  ///
  ///
  pub fn set_layers(&mut self, layers: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setLayers(self as *mut ::texture_image_data::TextureImageData, layers)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setMipLevels(int mipLevels)```</span>
  ///
  ///
  pub fn set_mip_levels(&mut self, mip_levels: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setMipLevels(self as *mut ::texture_image_data::TextureImageData, mip_levels) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setPixelFormat(QOpenGLTexture::PixelFormat pixelFormat)```</span>
  ///
  ///
  pub fn set_pixel_format(&mut self, pixel_format: &::qt_gui::opengl_texture::PixelFormat) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setPixelFormat(self as *mut ::texture_image_data::TextureImageData, pixel_format as *const ::qt_gui::opengl_texture::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setPixelType(QOpenGLTexture::PixelType pixelType)```</span>
  ///
  ///
  pub fn set_pixel_type(&mut self, pixel_type: &::qt_gui::opengl_texture::PixelType) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setPixelType(self as *mut ::texture_image_data::TextureImageData, pixel_type as *const ::qt_gui::opengl_texture::PixelType) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setTarget(QOpenGLTexture::Target target)```</span>
  ///
  ///
  pub fn set_target(&mut self, target: &::qt_gui::opengl_texture::Target) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setTarget(self as *mut ::texture_image_data::TextureImageData, target as *const ::qt_gui::opengl_texture::Target)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureImageData::setWidth(int width)```</span>
  ///
  ///
  pub fn set_width(&mut self, width: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setWidth(self as *mut ::texture_image_data::TextureImageData,
                                                                  width)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QTextureImageData::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_width(self as *const ::texture_image_data::TextureImageData)
    }
  }
}

impl Drop for ::texture_image_data::TextureImageData {
  /// C++ method: <span style='color: green;'>```[destructor] void Qt3DRender::QTextureImageData::~QTextureImageData()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_destructor(self as *mut ::texture_image_data::TextureImageData)
    }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextureImageData::data](../struct.TextureImageData.html#method.data) method.
  pub trait TextureImageDataDataArgs<'largs> {
    fn exec(self, original_self: &'largs ::texture_image_data::TextureImageData) -> ::qt_core::byte_array::ByteArray;
  }
  impl<'largs> TextureImageDataDataArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::texture_image_data::TextureImageData) -> ::qt_core::byte_array::ByteArray {
      let layer = self;
      {
        let mut object: ::qt_core::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_layer(original_self as *const ::texture_image_data::TextureImageData, layer, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextureImageDataDataArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::texture_image_data::TextureImageData) -> ::qt_core::byte_array::ByteArray {
      let layer = self.0;
      let face = self.1;
      {
        let mut object: ::qt_core::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_layer_face(original_self as *const ::texture_image_data::TextureImageData, layer, face, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextureImageDataDataArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::texture_image_data::TextureImageData) -> ::qt_core::byte_array::ByteArray {
      let layer = self.0;
      let face = self.1;
      let mipmap_level = self.2;
      {
        let mut object: ::qt_core::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_layer_face_mipmapLevel(original_self as *const ::texture_image_data::TextureImageData, layer, face, mipmap_level, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextureImageDataDataArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::texture_image_data::TextureImageData) -> ::qt_core::byte_array::ByteArray {

      {
        let mut object: ::qt_core::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_data_to_output_no_args(original_self as *const ::texture_image_data::TextureImageData, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextureImageData::set_data](../struct.TextureImageData.html#method.set_data) method.
  pub trait TextureImageDataSetDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::texture_image_data::TextureImageData) -> ();
  }
  impl<'largs> TextureImageDataSetDataArgs<'largs> for (&'largs ::qt_core::byte_array::ByteArray, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::texture_image_data::TextureImageData) -> () {
      let data = self.0;
      let block_size = self.1;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setData_data_blockSize(original_self as *mut ::texture_image_data::TextureImageData, data as *const ::qt_core::byte_array::ByteArray, block_size) }
    }
  }
  impl<'largs> TextureImageDataSetDataArgs<'largs> for (&'largs ::qt_core::byte_array::ByteArray, ::libc::c_int, bool) {
    fn exec(self, original_self: &'largs mut ::texture_image_data::TextureImageData) -> () {
      let data = self.0;
      let block_size = self.1;
      let is_compressed = self.2;
      unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureImageData_setData_data_blockSize_isCompressed(original_self as *mut ::texture_image_data::TextureImageData, data as *const ::qt_core::byte_array::ByteArray, block_size, is_compressed) }
    }
  }
}
