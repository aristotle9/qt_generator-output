/// C++ type: <span style='color: green;'>```Qt3DRender::QTextureData```</span>
#[repr(C)]
pub struct TextureData([u8; ::type_sizes::QT_3D_RENDER_TEXTURE_DATA_TEXTURE_DATA]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextureData {
  unsafe fn new_uninitialized() -> TextureData {
    TextureData(::std::mem::uninitialized())
  }
}

impl TextureData {
  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::addImageData(const QSharedPointer<Qt3DRender::QTextureImageData>& imageData)```</span>
  ///
  ///
  pub fn add_image_data(&mut self, image_data: &::shared_pointer::SharedPointerTextureImageData) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_addImageData(self as *mut ::texture_data::TextureData, image_data as *const ::shared_pointer::SharedPointerTextureImageData) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QTextureData::depth() const```</span>
  ///
  ///
  pub fn depth(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_depth(self as *const ::texture_data::TextureData) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QTextureData::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_height(self as *const ::texture_data::TextureData) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSharedPointer<Qt3DRender::QTextureImageData>> Qt3DRender::QTextureData::imageData() const```</span>
  ///
  ///
  pub fn image_data(&self) -> ::vector::VectorSharedPointerSharedPointerTextureImageData {
    {
      let mut object: ::vector::VectorSharedPointerSharedPointerTextureImageData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_imageData_to_output(self as *const ::texture_data::TextureData,
                                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool Qt3DRender::QTextureData::isAutoMipMapGenerationEnabled() const```</span>
  ///
  ///
  pub fn is_auto_mip_map_generation_enabled(&self) -> bool {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_isAutoMipMapGenerationEnabled(self as *const ::texture_data::TextureData) }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QTextureData::layers() const```</span>
  ///
  ///
  pub fn layers(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_layers(self as *const ::texture_data::TextureData) }
  }

  /// C++ method: <span style='color: green;'>```float Qt3DRender::QTextureData::maximumAnisotropy() const```</span>
  ///
  ///
  pub fn maximum_anisotropy(&self) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_maximumAnisotropy(self as *const ::texture_data::TextureData)
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void Qt3DRender::QTextureData::QTextureData()```</span>
  ///
  ///
  pub fn new() -> ::texture_data::TextureData {
    {
      let mut object: ::texture_data::TextureData =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setAutoMipMapGenerationEnabled(bool isAutoMipMapGenerationEnabled)```</span>
  ///
  ///
  pub fn set_auto_mip_map_generation_enabled(&mut self, is_auto_mip_map_generation_enabled: bool) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setAutoMipMapGenerationEnabled(self as *mut ::texture_data::TextureData, is_auto_mip_map_generation_enabled) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setComparisonFunction(Qt3DRender::QAbstractTexture::ComparisonFunction comparisonFunction)```</span>
  ///
  ///
  pub fn set_comparison_function(&mut self, comparison_function: &::abstract_texture::ComparisonFunction) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setComparisonFunction(self as *mut ::texture_data::TextureData, comparison_function as *const ::abstract_texture::ComparisonFunction) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setComparisonMode(Qt3DRender::QAbstractTexture::ComparisonMode comparisonMode)```</span>
  ///
  ///
  pub fn set_comparison_mode(&mut self, comparison_mode: &::abstract_texture::ComparisonMode) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setComparisonMode(self as *mut ::texture_data::TextureData, comparison_mode as *const ::abstract_texture::ComparisonMode) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setDepth(int depth)```</span>
  ///
  ///
  pub fn set_depth(&mut self, depth: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setDepth(self as *mut ::texture_data::TextureData, depth) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setFormat(Qt3DRender::QAbstractTexture::TextureFormat arg1)```</span>
  ///
  ///
  pub fn set_format(&mut self, arg1: &::abstract_texture::TextureFormat) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setFormat(self as *mut ::texture_data::TextureData,
                                                              arg1 as *const ::abstract_texture::TextureFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setHeight(int height)```</span>
  ///
  ///
  pub fn set_height(&mut self, height: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setHeight(self as *mut ::texture_data::TextureData, height) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setLayers(int layers)```</span>
  ///
  ///
  pub fn set_layers(&mut self, layers: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setLayers(self as *mut ::texture_data::TextureData, layers) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setMagnificationFilter(Qt3DRender::QAbstractTexture::Filter filter)```</span>
  ///
  ///
  pub fn set_magnification_filter(&mut self, filter: &::abstract_texture::Filter) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setMagnificationFilter(self as *mut ::texture_data::TextureData, filter as *const ::abstract_texture::Filter)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setMaximumAnisotropy(float maximumAnisotropy)```</span>
  ///
  ///
  pub fn set_maximum_anisotropy(&mut self, maximum_anisotropy: ::libc::c_float) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setMaximumAnisotropy(self as *mut ::texture_data::TextureData,
                                                                         maximum_anisotropy)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setMinificationFilter(Qt3DRender::QAbstractTexture::Filter filter)```</span>
  ///
  ///
  pub fn set_minification_filter(&mut self, filter: &::abstract_texture::Filter) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setMinificationFilter(self as *mut ::texture_data::TextureData,
                                                                          filter as *const ::abstract_texture::Filter)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setTarget(Qt3DRender::QAbstractTexture::Target target)```</span>
  ///
  ///
  pub fn set_target(&mut self, target: &::abstract_texture::Target) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setTarget(self as *mut ::texture_data::TextureData,
                                                              target as *const ::abstract_texture::Target)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setWidth(int width)```</span>
  ///
  ///
  pub fn set_width(&mut self, width: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setWidth(self as *mut ::texture_data::TextureData, width) }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setWrapModeX(Qt3DRender::QTextureWrapMode::WrapMode wrapModeX)```</span>
  ///
  ///
  pub fn set_wrap_mode_x(&mut self, wrap_mode_x: &::texture_wrap_mode::WrapMode) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setWrapModeX(self as *mut ::texture_data::TextureData,
                                                                 wrap_mode_x as *const ::texture_wrap_mode::WrapMode)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setWrapModeY(Qt3DRender::QTextureWrapMode::WrapMode wrapModeY)```</span>
  ///
  ///
  pub fn set_wrap_mode_y(&mut self, wrap_mode_y: &::texture_wrap_mode::WrapMode) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setWrapModeY(self as *mut ::texture_data::TextureData,
                                                                 wrap_mode_y as *const ::texture_wrap_mode::WrapMode)
    }
  }

  /// C++ method: <span style='color: green;'>```void Qt3DRender::QTextureData::setWrapModeZ(Qt3DRender::QTextureWrapMode::WrapMode wrapModeZ)```</span>
  ///
  ///
  pub fn set_wrap_mode_z(&mut self, wrap_mode_z: &::texture_wrap_mode::WrapMode) {
    unsafe {
      ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_setWrapModeZ(self as *mut ::texture_data::TextureData,
                                                                 wrap_mode_z as *const ::texture_wrap_mode::WrapMode)
    }
  }

  /// C++ method: <span style='color: green;'>```int Qt3DRender::QTextureData::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_width(self as *const ::texture_data::TextureData) }
  }
}

impl Drop for ::texture_data::TextureData {
  /// C++ method: <span style='color: green;'>```[destructor] void Qt3DRender::QTextureData::~QTextureData()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_render_c_Qt3DRender_QTextureData_destructor(self as *mut ::texture_data::TextureData) }
  }
}
