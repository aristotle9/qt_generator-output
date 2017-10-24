/// C++ type: <span style='color: green;'>```QOpenGLTexture::BindingTarget```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum BindingTarget {
  /// C++ enum variant: <span style='color: green;'>```BindingTarget1D = 32872```</span>
  BindingTarget1D = 32872,
  /// C++ enum variant: <span style='color: green;'>```BindingTarget2D = 32873```</span>
  BindingTarget2D = 32873,
  /// C++ enum variant: <span style='color: green;'>```BindingTarget3D = 32874```</span>
  BindingTarget3D = 32874,
  /// C++ enum variant: <span style='color: green;'>```BindingTargetRectangle = 34038```</span>
  BindingTargetRectangle = 34038,
  /// C++ enum variant: <span style='color: green;'>```BindingTargetCubeMap = 34068```</span>
  BindingTargetCubeMap = 34068,
  /// C++ enum variant: <span style='color: green;'>```BindingTarget1DArray = 35868```</span>
  BindingTarget1DArray = 35868,
  /// C++ enum variant: <span style='color: green;'>```BindingTarget2DArray = 35869```</span>
  BindingTarget2DArray = 35869,
  /// C++ enum variant: <span style='color: green;'>```BindingTargetBuffer = 35884```</span>
  BindingTargetBuffer = 35884,
  /// C++ enum variant: <span style='color: green;'>```BindingTargetCubeMapArray = 36874```</span>
  BindingTargetCubeMapArray = 36874,
  /// C++ enum variant: <span style='color: green;'>```BindingTarget2DMultisample = 37124```</span>
  BindingTarget2DMultisample = 37124,
  /// C++ enum variant: <span style='color: green;'>```BindingTarget2DMultisampleArray = 37125```</span>
  BindingTarget2DMultisampleArray = 37125,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::ComparisonFunction```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ComparisonFunction {
  /// C++ enum variant: <span style='color: green;'>```CompareNever = 512```</span>
  CompareNever = 512,
  /// C++ enum variant: <span style='color: green;'>```CompareLess = 513```</span>
  CompareLess = 513,
  /// C++ enum variant: <span style='color: green;'>```CompareEqual = 514```</span>
  CompareEqual = 514,
  /// C++ enum variant: <span style='color: green;'>```CompareLessEqual = 515```</span>
  CompareLessEqual = 515,
  /// C++ enum variant: <span style='color: green;'>```CompareGreater = 516```</span>
  CompareGreater = 516,
  /// C++ enum variant: <span style='color: green;'>```CommpareNotEqual = 517```</span>
  CommpareNotEqual = 517,
  /// C++ enum variant: <span style='color: green;'>```CompareGreaterEqual = 518```</span>
  CompareGreaterEqual = 518,
  /// C++ enum variant: <span style='color: green;'>```CompareAlways = 519```</span>
  CompareAlways = 519,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::ComparisonMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ComparisonMode {
  /// C++ enum variant: <span style='color: green;'>```CompareNone = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```CompareRefToTexture = 34894```</span>
  RefToTexture = 34894,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::CoordinateDirection```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CoordinateDirection {
  /// C++ enum variant: <span style='color: green;'>```DirectionS = 10242```</span>
  S = 10242,
  /// C++ enum variant: <span style='color: green;'>```DirectionT = 10243```</span>
  T = 10243,
  /// C++ enum variant: <span style='color: green;'>```DirectionR = 32882```</span>
  R = 32882,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::CubeMapFace```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CubeMapFace {
  /// C++ enum variant: <span style='color: green;'>```CubeMapPositiveX = 34069```</span>
  PositiveX = 34069,
  /// C++ enum variant: <span style='color: green;'>```CubeMapNegativeX = 34070```</span>
  NegativeX = 34070,
  /// C++ enum variant: <span style='color: green;'>```CubeMapPositiveY = 34071```</span>
  PositiveY = 34071,
  /// C++ enum variant: <span style='color: green;'>```CubeMapNegativeY = 34072```</span>
  NegativeY = 34072,
  /// C++ enum variant: <span style='color: green;'>```CubeMapPositiveZ = 34073```</span>
  PositiveZ = 34073,
  /// C++ enum variant: <span style='color: green;'>```CubeMapNegativeZ = 34074```</span>
  NegativeZ = 34074,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::DepthStencilMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DepthStencilMode {
  /// C++ enum variant: <span style='color: green;'>```StencilMode = 6401```</span>
  Stencil = 6401,
  /// C++ enum variant: <span style='color: green;'>```DepthMode = 6402```</span>
  Depth = 6402,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::Feature```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Feature {
  /// C++ enum variant: <span style='color: green;'>```ImmutableStorage = 1```</span>
  ImmutableStorage = 1,
  /// C++ enum variant: <span style='color: green;'>```ImmutableMultisampleStorage = 2```</span>
  ImmutableMultisampleStorage = 2,
  /// C++ enum variant: <span style='color: green;'>```TextureRectangle = 4```</span>
  TextureRectangle = 4,
  /// C++ enum variant: <span style='color: green;'>```TextureArrays = 8```</span>
  TextureArrays = 8,
  /// C++ enum variant: <span style='color: green;'>```Texture3D = 16```</span>
  Texture3D = 16,
  /// C++ enum variant: <span style='color: green;'>```TextureMultisample = 32```</span>
  TextureMultisample = 32,
  /// C++ enum variant: <span style='color: green;'>```TextureBuffer = 64```</span>
  TextureBuffer = 64,
  /// C++ enum variant: <span style='color: green;'>```TextureCubeMapArrays = 128```</span>
  TextureCubeMapArrays = 128,
  /// C++ enum variant: <span style='color: green;'>```Swizzle = 256```</span>
  Swizzle = 256,
  /// C++ enum variant: <span style='color: green;'>```StencilTexturing = 512```</span>
  StencilTexturing = 512,
  /// C++ enum variant: <span style='color: green;'>```AnisotropicFiltering = 1024```</span>
  AnisotropicFiltering = 1024,
  /// C++ enum variant: <span style='color: green;'>```NPOTTextures = 2048```</span>
  NPOTTextures = 2048,
  /// C++ enum variant: <span style='color: green;'>```NPOTTextureRepeat = 4096```</span>
  NPOTTextureRepeat = 4096,
  /// C++ enum variant: <span style='color: green;'>```Texture1D = 8192```</span>
  Texture1D = 8192,
  /// C++ enum variant: <span style='color: green;'>```TextureComparisonOperators = 16384```</span>
  TextureComparisonOperators = 16384,
  /// C++ enum variant: <span style='color: green;'>```TextureMipMapLevel = 32768```</span>
  TextureMipMapLevel = 32768,
  /// C++ enum variant: <span style='color: green;'>```MaxFeatureFlag = 65536```</span>
  MaxFeatureFlag = 65536,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::Filter```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Filter {
  /// C++ enum variant: <span style='color: green;'>```Nearest = 9728```</span>
  Nearest = 9728,
  /// C++ enum variant: <span style='color: green;'>```Linear = 9729```</span>
  Linear = 9729,
  /// C++ enum variant: <span style='color: green;'>```NearestMipMapNearest = 9984```</span>
  NearestMipMapNearest = 9984,
  /// C++ enum variant: <span style='color: green;'>```LinearMipMapNearest = 9985```</span>
  LinearMipMapNearest = 9985,
  /// C++ enum variant: <span style='color: green;'>```NearestMipMapLinear = 9986```</span>
  NearestMipMapLinear = 9986,
  /// C++ enum variant: <span style='color: green;'>```LinearMipMapLinear = 9987```</span>
  LinearMipMapLinear = 9987,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::MipMapGeneration```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum MipMapGeneration {
  /// C++ enum variant: <span style='color: green;'>```GenerateMipMaps = 0```</span>
  GenerateMipMaps = 0,
  /// C++ enum variant: <span style='color: green;'>```DontGenerateMipMaps = 1```</span>
  DontGenerateMipMaps = 1,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture```</span>
#[repr(C)]
pub struct OpenGLTexture(u8);

impl OpenGLTexture {
  /// C++ method: <span style='color: green;'>```QOpenGLTexture::allocateStorage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn allocate_storage(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::allocateStorage()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn allocate_storage(&mut self, (::opengl_texture::PixelFormat, ::opengl_texture::PixelType)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::allocateStorage(QOpenGLTexture::PixelFormat pixelFormat, QOpenGLTexture::PixelType pixelType)```</span>
  ///
  ///
  pub fn allocate_storage<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureAllocateStorageArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLTexture::bind```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn bind(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::bind()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn bind(&mut self, ::libc::c_uint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::bind(unsigned int unit)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn bind(&mut self, (::libc::c_uint, ::opengl_texture::TextureUnitReset)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::bind(unsigned int unit, QOpenGLTexture::TextureUnitReset reset = ?)```</span>
  ///
  ///
  pub fn bind<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureBindArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor QOpenGLTexture::borderColor() const```</span>
  ///
  ///
  pub fn border_color(&self) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_borderColor_to_output(self as *const ::opengl_texture::OpenGLTexture,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::borderColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn border_color_unsafe(&self, *mut ::libc::c_float) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::borderColor(float* border) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn border_color_unsafe(&self, *mut ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::borderColor(int* border) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn border_color_unsafe(&self, *mut ::libc::c_uint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::borderColor(unsigned int* border) const```</span>
  ///
  ///
  pub unsafe fn border_color_unsafe<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::OpenGLTextureBorderColorUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLTexture::boundTextureId```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn bound_texture_id(::opengl_texture::BindingTarget) -> u32```<br>
  /// C++ method: <span style='color: green;'>```static GLuint QOpenGLTexture::boundTextureId(QOpenGLTexture::BindingTarget target)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn bound_texture_id((::libc::c_uint, ::opengl_texture::BindingTarget)) -> u32```<br>
  /// C++ method: <span style='color: green;'>```static GLuint QOpenGLTexture::boundTextureId(unsigned int unit, QOpenGLTexture::BindingTarget target)```</span>
  ///
  ///
  pub fn bound_texture_id<Args>(args: Args) -> u32
    where Args: overloading::OpenGLTextureBoundTextureIdArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QOpenGLTexture::ComparisonFunction QOpenGLTexture::comparisonFunction() const```</span>
  ///
  ///
  pub fn comparison_function(&self) -> ::opengl_texture::ComparisonFunction {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_comparisonFunction(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::ComparisonMode QOpenGLTexture::comparisonMode() const```</span>
  ///
  ///
  pub fn comparison_mode(&self) -> ::opengl_texture::ComparisonMode {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_comparisonMode(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTexture::create()```</span>
  ///
  ///
  pub fn create(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_create(self as *mut ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture* QOpenGLTexture::createTextureView(QOpenGLTexture::Target target, QOpenGLTexture::TextureFormat viewFormat, int minimumMipmapLevel, int maximumMipmapLevel, int minimumLayer, int maximumLayer) const```</span>
  ///
  ///
  pub fn create_texture_view(&self,
                             target: ::opengl_texture::Target,
                             view_format: ::opengl_texture::TextureFormat,
                             minimum_mipmap_level: ::libc::c_int,
                             maximum_mipmap_level: ::libc::c_int,
                             minimum_layer: ::libc::c_int,
                             maximum_layer: ::libc::c_int)
                             -> *mut ::opengl_texture::OpenGLTexture {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_createTextureView(self as *const ::opengl_texture::OpenGLTexture,
                                                       target,
                                                       view_format,
                                                       minimum_mipmap_level,
                                                       maximum_mipmap_level,
                                                       minimum_layer,
                                                       maximum_layer)
    }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLTexture::depth() const```</span>
  ///
  ///
  pub fn depth(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_depth(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::DepthStencilMode QOpenGLTexture::depthStencilMode() const```</span>
  ///
  ///
  pub fn depth_stencil_mode(&self) -> ::opengl_texture::DepthStencilMode {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_depthStencilMode(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::destroy()```</span>
  ///
  ///
  pub fn destroy(&mut self) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_destroy(self as *mut ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLTexture::faces() const```</span>
  ///
  ///
  pub fn faces(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_faces(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::TextureFormat QOpenGLTexture::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::opengl_texture::TextureFormat {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_format(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::generateMipMaps```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn generate_mip_maps(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::generateMipMaps()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn generate_mip_maps(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::generateMipMaps(int baseLevel)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn generate_mip_maps(&mut self, (::libc::c_int, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::generateMipMaps(int baseLevel, bool resetBaseLevel = ?)```</span>
  ///
  ///
  pub fn generate_mip_maps<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureGenerateMipMapsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static bool QOpenGLTexture::hasFeature(QOpenGLTexture::Feature feature)```</span>
  ///
  ///
  pub fn has_feature(feature: ::opengl_texture::Feature) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_hasFeature(feature) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLTexture::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_height(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTexture::isAutoMipMapGenerationEnabled() const```</span>
  ///
  ///
  pub fn is_auto_mip_map_generation_enabled(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_isAutoMipMapGenerationEnabled(self as *const ::opengl_texture::OpenGLTexture)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTexture::isBound() const```</span>
  ///
  ///
  pub fn is_bound(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_isBound_no_args(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTexture::isBound(unsigned int unit)```</span>
  ///
  ///
  pub fn is_bound_mut(&mut self, unit: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_isBound_unit(self as *mut ::opengl_texture::OpenGLTexture, unit) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTexture::isCreated() const```</span>
  ///
  ///
  pub fn is_created(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_isCreated(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTexture::isFixedSamplePositions() const```</span>
  ///
  ///
  pub fn is_fixed_sample_positions(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_isFixedSamplePositions(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTexture::isStorageAllocated() const```</span>
  ///
  ///
  pub fn is_storage_allocated(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_isStorageAllocated(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLTexture::isTextureView() const```</span>
  ///
  ///
  pub fn is_texture_view(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_isTextureView(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLTexture::layers() const```</span>
  ///
  ///
  pub fn layers(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_layers(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```QPair<float, float> QOpenGLTexture::levelOfDetailRange() const```</span>
  ///
  ///
  pub fn level_of_detail_range(&self) -> ::pair::PairCFloatCFloat {
    {
      let mut object: ::pair::PairCFloatCFloat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_levelOfDetailRange_to_output(self as *const ::opengl_texture::OpenGLTexture,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float QOpenGLTexture::levelofDetailBias() const```</span>
  ///
  ///
  pub fn levelof_detail_bias(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_levelofDetailBias(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::Filter QOpenGLTexture::magnificationFilter() const```</span>
  ///
  ///
  pub fn magnification_filter(&self) -> ::opengl_texture::Filter {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_magnificationFilter(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```float QOpenGLTexture::maximumAnisotropy() const```</span>
  ///
  ///
  pub fn maximum_anisotropy(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_maximumAnisotropy(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```float QOpenGLTexture::maximumLevelOfDetail() const```</span>
  ///
  ///
  pub fn maximum_level_of_detail(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_maximumLevelOfDetail(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLTexture::maximumMipLevels() const```</span>
  ///
  ///
  pub fn maximum_mip_levels(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_maximumMipLevels(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```QPair<QOpenGLTexture::Filter, QOpenGLTexture::Filter> QOpenGLTexture::minMagFilters() const```</span>
  ///
  ///
  pub fn min_mag_filters(&self) -> ::pair::PairOpenglTextureFilterOpenglTextureFilter {
    {
      let mut object: ::pair::PairOpenglTextureFilterOpenglTextureFilter =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_minMagFilters_to_output(self as *const ::opengl_texture::OpenGLTexture,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::Filter QOpenGLTexture::minificationFilter() const```</span>
  ///
  ///
  pub fn minification_filter(&self) -> ::opengl_texture::Filter {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_minificationFilter(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```float QOpenGLTexture::minimumLevelOfDetail() const```</span>
  ///
  ///
  pub fn minimum_level_of_detail(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_minimumLevelOfDetail(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLTexture::mipBaseLevel() const```</span>
  ///
  ///
  pub fn mip_base_level(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_mipBaseLevel(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```QPair<int, int> QOpenGLTexture::mipLevelRange() const```</span>
  ///
  ///
  pub fn mip_level_range(&self) -> ::pair::PairCIntCInt {
    {
      let mut object: ::pair::PairCIntCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_mipLevelRange_to_output(self as *const ::opengl_texture::OpenGLTexture,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLTexture::mipLevels() const```</span>
  ///
  ///
  pub fn mip_levels(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_mipLevels(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLTexture::mipMaxLevel() const```</span>
  ///
  ///
  pub fn mip_max_level(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_mipMaxLevel(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::QOpenGLTexture```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(::opengl_texture::Target) -> ::cpp_utils::CppBox<::opengl_texture::OpenGLTexture>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLTexture::QOpenGLTexture(QOpenGLTexture::Target target)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::image::Image) -> ::cpp_utils::CppBox<::opengl_texture::OpenGLTexture>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLTexture::QOpenGLTexture(const QImage& image)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::image::Image, ::opengl_texture::MipMapGeneration)) -> ::cpp_utils::CppBox<::opengl_texture::OpenGLTexture>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLTexture::QOpenGLTexture(const QImage& image, QOpenGLTexture::MipMapGeneration genMipMaps = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::opengl_texture::OpenGLTexture>
    where Args: overloading::OpenGLTextureNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QOpenGLTexture::release```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn release(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::release()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn release(&mut self, ::libc::c_uint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::release(unsigned int unit)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn release(&mut self, (::libc::c_uint, ::opengl_texture::TextureUnitReset)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::release(unsigned int unit, QOpenGLTexture::TextureUnitReset reset = ?)```</span>
  ///
  ///
  pub fn release<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureReleaseArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QOpenGLTexture::samples() const```</span>
  ///
  ///
  pub fn samples(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_samples(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setAutoMipMapGenerationEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_auto_mip_map_generation_enabled(&mut self, enabled: bool) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_setAutoMipMapGenerationEnabled(self as *mut ::opengl_texture::OpenGLTexture,
                                                                    enabled)
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::setBorderColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_border_color(&mut self, &::color::Color) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setBorderColor(QColor color)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_border_color(&mut self, (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setBorderColor(float r, float g, float b, float a)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_border_color(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setBorderColor(int r, int g, int b, int a)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_border_color(&mut self, (::libc::c_uint, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setBorderColor(unsigned int r, unsigned int g, unsigned int b, unsigned int a)```</span>
  ///
  ///
  pub fn set_border_color<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureSetBorderColorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setComparisonFunction(QOpenGLTexture::ComparisonFunction function)```</span>
  ///
  ///
  pub fn set_comparison_function(&mut self, function: ::opengl_texture::ComparisonFunction) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_setComparisonFunction(self as *mut ::opengl_texture::OpenGLTexture, function)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setComparisonMode(QOpenGLTexture::ComparisonMode mode)```</span>
  ///
  ///
  pub fn set_comparison_mode(&mut self, mode: ::opengl_texture::ComparisonMode) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setComparisonMode(self as *mut ::opengl_texture::OpenGLTexture, mode) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::setCompressedData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, *const ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int dataSize, const void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, *const ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int dataSize, const void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, *mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int dataSize, void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, *mut ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int dataSize, void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, *const ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, const void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, *const ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, const void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, *mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, *mut ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int dataSize, void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::libc::c_int, *const ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, const void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::libc::c_int, *const ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, const void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::libc::c_int, *mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::libc::c_int, *mut ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, *const ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, const void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, *const ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, const void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, *mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, *mut ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int dataSize, void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::libc::c_int, *const ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int layerCount, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, const void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn set_compressed_data(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::libc::c_int, *const ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setCompressedData(int mipLevel, int layer, int layerCount, QOpenGLTexture::CubeMapFace cubeFace, int dataSize, const void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  pub unsafe fn set_compressed_data<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureSetCompressedDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLTexture::setData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_data(&mut self, &::image::Image) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(const QImage& image)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_data(&mut self, (&::image::Image, ::opengl_texture::MipMapGeneration)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(const QImage& image, QOpenGLTexture::MipMapGeneration genMipMaps = ?)```</span>
  ///
  ///
  pub fn set_data<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureSetDataArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLTexture::setData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *mut ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *mut ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *mut ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, int layer, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, int layer, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 14
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, int layer, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 15
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, int layer, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 16
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *mut ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, int layer, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 17
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, int layer, int layerCount, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data)```</span>
  ///
  ///
  ///
  /// ## Variant 18
  ///
  /// Rust arguments: ```fn set_data_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void, *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setData(int mipLevel, int layer, int layerCount, QOpenGLTexture::CubeMapFace cubeFace, QOpenGLTexture::PixelFormat sourceFormat, QOpenGLTexture::PixelType sourceType, const void* data, const QOpenGLPixelTransferOptions* options = ?)```</span>
  ///
  ///
  pub unsafe fn set_data_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureSetDataUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setDepthStencilMode(QOpenGLTexture::DepthStencilMode mode)```</span>
  ///
  ///
  pub fn set_depth_stencil_mode(&mut self, mode: ::opengl_texture::DepthStencilMode) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setDepthStencilMode(self as *mut ::opengl_texture::OpenGLTexture, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setFixedSamplePositions(bool fixed)```</span>
  ///
  ///
  pub fn set_fixed_sample_positions(&mut self, fixed: bool) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_setFixedSamplePositions(self as *mut ::opengl_texture::OpenGLTexture, fixed)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setFormat(QOpenGLTexture::TextureFormat format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: ::opengl_texture::TextureFormat) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setFormat(self as *mut ::opengl_texture::OpenGLTexture, format) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setLayers(int layers)```</span>
  ///
  ///
  pub fn set_layers(&mut self, layers: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setLayers(self as *mut ::opengl_texture::OpenGLTexture, layers) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setLevelOfDetailRange(float min, float max)```</span>
  ///
  ///
  pub fn set_level_of_detail_range(&mut self, min: ::libc::c_float, max: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_setLevelOfDetailRange(self as *mut ::opengl_texture::OpenGLTexture, min, max)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setLevelofDetailBias(float bias)```</span>
  ///
  ///
  pub fn set_levelof_detail_bias(&mut self, bias: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setLevelofDetailBias(self as *mut ::opengl_texture::OpenGLTexture, bias) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setMagnificationFilter(QOpenGLTexture::Filter filter)```</span>
  ///
  ///
  pub fn set_magnification_filter(&mut self, filter: ::opengl_texture::Filter) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_setMagnificationFilter(self as *mut ::opengl_texture::OpenGLTexture, filter)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setMaximumAnisotropy(float anisotropy)```</span>
  ///
  ///
  pub fn set_maximum_anisotropy(&mut self, anisotropy: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_setMaximumAnisotropy(self as *mut ::opengl_texture::OpenGLTexture, anisotropy)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setMaximumLevelOfDetail(float value)```</span>
  ///
  ///
  pub fn set_maximum_level_of_detail(&mut self, value: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_setMaximumLevelOfDetail(self as *mut ::opengl_texture::OpenGLTexture, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setMinMagFilters(QOpenGLTexture::Filter minificationFilter, QOpenGLTexture::Filter magnificationFilter)```</span>
  ///
  ///
  pub fn set_min_mag_filters(&mut self,
                             minification_filter: ::opengl_texture::Filter,
                             magnification_filter: ::opengl_texture::Filter) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_setMinMagFilters(self as *mut ::opengl_texture::OpenGLTexture,
                                                      minification_filter,
                                                      magnification_filter)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setMinificationFilter(QOpenGLTexture::Filter filter)```</span>
  ///
  ///
  pub fn set_minification_filter(&mut self, filter: ::opengl_texture::Filter) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_setMinificationFilter(self as *mut ::opengl_texture::OpenGLTexture, filter)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setMinimumLevelOfDetail(float value)```</span>
  ///
  ///
  pub fn set_minimum_level_of_detail(&mut self, value: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_setMinimumLevelOfDetail(self as *mut ::opengl_texture::OpenGLTexture, value)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setMipBaseLevel(int baseLevel)```</span>
  ///
  ///
  pub fn set_mip_base_level(&mut self, base_level: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setMipBaseLevel(self as *mut ::opengl_texture::OpenGLTexture, base_level) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setMipLevelRange(int baseLevel, int maxLevel)```</span>
  ///
  ///
  pub fn set_mip_level_range(&mut self, base_level: ::libc::c_int, max_level: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QOpenGLTexture_setMipLevelRange(self as *mut ::opengl_texture::OpenGLTexture,
                                                      base_level,
                                                      max_level)
    }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setMipLevels(int levels)```</span>
  ///
  ///
  pub fn set_mip_levels(&mut self, levels: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setMipLevels(self as *mut ::opengl_texture::OpenGLTexture, levels) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setMipMaxLevel(int maxLevel)```</span>
  ///
  ///
  pub fn set_mip_max_level(&mut self, max_level: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setMipMaxLevel(self as *mut ::opengl_texture::OpenGLTexture, max_level) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setSamples(int samples)```</span>
  ///
  ///
  pub fn set_samples(&mut self, samples: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setSamples(self as *mut ::opengl_texture::OpenGLTexture, samples) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::setSize```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_size(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setSize(int width)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_size(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setSize(int width, int height = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_size(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setSize(int width, int height = ?, int depth = ?)```</span>
  ///
  ///
  pub fn set_size<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureSetSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLTexture::setSwizzleMask```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_swizzle_mask(&mut self, (::opengl_texture::SwizzleComponent, ::opengl_texture::SwizzleValue)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setSwizzleMask(QOpenGLTexture::SwizzleComponent component, QOpenGLTexture::SwizzleValue value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_swizzle_mask(&mut self, (::opengl_texture::SwizzleValue, ::opengl_texture::SwizzleValue, ::opengl_texture::SwizzleValue, ::opengl_texture::SwizzleValue)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setSwizzleMask(QOpenGLTexture::SwizzleValue r, QOpenGLTexture::SwizzleValue g, QOpenGLTexture::SwizzleValue b, QOpenGLTexture::SwizzleValue a)```</span>
  ///
  ///
  pub fn set_swizzle_mask<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureSetSwizzleMaskArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLTexture::setWrapMode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_wrap_mode(&mut self, (::opengl_texture::CoordinateDirection, ::opengl_texture::WrapMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setWrapMode(QOpenGLTexture::CoordinateDirection direction, QOpenGLTexture::WrapMode mode)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_wrap_mode(&mut self, ::opengl_texture::WrapMode) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QOpenGLTexture::setWrapMode(QOpenGLTexture::WrapMode mode)```</span>
  ///
  ///
  pub fn set_wrap_mode<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::OpenGLTextureSetWrapModeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLTexture::SwizzleValue QOpenGLTexture::swizzleMask(QOpenGLTexture::SwizzleComponent component) const```</span>
  ///
  ///
  pub fn swizzle_mask(&self, component: ::opengl_texture::SwizzleComponent) -> ::opengl_texture::SwizzleValue {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_swizzleMask(self as *const ::opengl_texture::OpenGLTexture, component) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::Target QOpenGLTexture::target() const```</span>
  ///
  ///
  pub fn target(&self) -> ::opengl_texture::Target {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_target(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```GLuint QOpenGLTexture::textureId() const```</span>
  ///
  ///
  pub fn texture_id(&self) -> u32 {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_textureId(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLTexture::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_width(self as *const ::opengl_texture::OpenGLTexture) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLTexture::WrapMode QOpenGLTexture::wrapMode(QOpenGLTexture::CoordinateDirection direction) const```</span>
  ///
  ///
  pub fn wrap_mode(&self, direction: ::opengl_texture::CoordinateDirection) -> ::opengl_texture::WrapMode {
    unsafe { ::ffi::qt_gui_c_QOpenGLTexture_wrapMode(self as *const ::opengl_texture::OpenGLTexture, direction) }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_texture::OpenGLTexture {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLTexture_delete
  }
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::PixelFormat```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PixelFormat {
  /// C++ enum variant: <span style='color: green;'>```NoSourceFormat = 0```</span>
  NoSourceFormat = 0,
  /// C++ enum variant: <span style='color: green;'>```Stencil = 6401```</span>
  Stencil = 6401,
  /// C++ enum variant: <span style='color: green;'>```Depth = 6402```</span>
  Depth = 6402,
  /// C++ enum variant: <span style='color: green;'>```Red = 6403```</span>
  Red = 6403,
  /// C++ enum variant: <span style='color: green;'>```Alpha = 6406```</span>
  Alpha = 6406,
  /// C++ enum variant: <span style='color: green;'>```RGB = 6407```</span>
  RGB = 6407,
  /// C++ enum variant: <span style='color: green;'>```RGBA = 6408```</span>
  RGBA = 6408,
  /// C++ enum variant: <span style='color: green;'>```Luminance = 6409```</span>
  Luminance = 6409,
  /// C++ enum variant: <span style='color: green;'>```LuminanceAlpha = 6410```</span>
  LuminanceAlpha = 6410,
  /// C++ enum variant: <span style='color: green;'>```BGR = 32992```</span>
  BGR = 32992,
  /// C++ enum variant: <span style='color: green;'>```BGRA = 32993```</span>
  BGRA = 32993,
  /// C++ enum variant: <span style='color: green;'>```RG = 33319```</span>
  RG = 33319,
  /// C++ enum variant: <span style='color: green;'>```RG_Integer = 33320```</span>
  RGInteger = 33320,
  /// C++ enum variant: <span style='color: green;'>```DepthStencil = 34041```</span>
  DepthStencil = 34041,
  /// C++ enum variant: <span style='color: green;'>```Red_Integer = 36244```</span>
  RedInteger = 36244,
  /// C++ enum variant: <span style='color: green;'>```RGB_Integer = 36248```</span>
  RGBInteger = 36248,
  /// C++ enum variant: <span style='color: green;'>```RGBA_Integer = 36249```</span>
  RGBAInteger = 36249,
  /// C++ enum variant: <span style='color: green;'>```BGR_Integer = 36250```</span>
  BGRInteger = 36250,
  /// C++ enum variant: <span style='color: green;'>```BGRA_Integer = 36251```</span>
  BGRAInteger = 36251,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::PixelType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PixelType {
  /// C++ enum variant: <span style='color: green;'>```NoPixelType = 0```</span>
  NoPixelType = 0,
  /// C++ enum variant: <span style='color: green;'>```Int8 = 5120```</span>
  Int8 = 5120,
  /// C++ enum variant: <span style='color: green;'>```UInt8 = 5121```</span>
  UInt8 = 5121,
  /// C++ enum variant: <span style='color: green;'>```Int16 = 5122```</span>
  Int16 = 5122,
  /// C++ enum variant: <span style='color: green;'>```UInt16 = 5123```</span>
  UInt16 = 5123,
  /// C++ enum variant: <span style='color: green;'>```Int32 = 5124```</span>
  Int32 = 5124,
  /// C++ enum variant: <span style='color: green;'>```UInt32 = 5125```</span>
  UInt32 = 5125,
  /// C++ enum variant: <span style='color: green;'>```Float32 = 5126```</span>
  Float32 = 5126,
  /// C++ enum variant: <span style='color: green;'>```Float16 = 5131```</span>
  Float16 = 5131,
  /// C++ enum variant: <span style='color: green;'>```UInt8_RG3B2 = 32818```</span>
  UInt8RG3B2 = 32818,
  /// C++ enum variant: <span style='color: green;'>```UInt16_RGBA4 = 32819```</span>
  UInt16RGBA4 = 32819,
  /// C++ enum variant: <span style='color: green;'>```UInt16_RGB5A1 = 32820```</span>
  UInt16RGB5A1 = 32820,
  /// C++ enum variant: <span style='color: green;'>```UInt32_RGBA8 = 32821```</span>
  UInt32RGBA8 = 32821,
  /// C++ enum variant: <span style='color: green;'>```UInt32_RGB10A2 = 32822```</span>
  UInt32RGB10A2 = 32822,
  /// C++ enum variant: <span style='color: green;'>```UInt8_RG3B2_Rev = 33634```</span>
  UInt8RG3B2Rev = 33634,
  /// C++ enum variant: <span style='color: green;'>```UInt16_R5G6B5 = 33635```</span>
  UInt16R5G6B5 = 33635,
  /// C++ enum variant: <span style='color: green;'>```UInt16_R5G6B5_Rev = 33636```</span>
  UInt16R5G6B5Rev = 33636,
  /// C++ enum variant: <span style='color: green;'>```UInt16_RGBA4_Rev = 33637```</span>
  UInt16RGBA4Rev = 33637,
  /// C++ enum variant: <span style='color: green;'>```UInt16_RGB5A1_Rev = 33638```</span>
  UInt16RGB5A1Rev = 33638,
  /// C++ enum variant: <span style='color: green;'>```UInt32_RGBA8_Rev = 33639```</span>
  UInt32RGBA8Rev = 33639,
  /// C++ enum variant: <span style='color: green;'>```UInt32_RGB10A2_Rev = 33640```</span>
  UInt32RGB10A2Rev = 33640,
  /// C++ enum variant: <span style='color: green;'>```UInt32_D24S8 = 34042```</span>
  UInt32D24S8 = 34042,
  /// C++ enum variant: <span style='color: green;'>```UInt32_RG11B10F = 35899```</span>
  UInt32RG11B10F = 35899,
  /// C++ enum variant: <span style='color: green;'>```UInt32_RGB9_E5 = 35902```</span>
  UInt32RGB9E5 = 35902,
  /// C++ enum variant: <span style='color: green;'>```Float16OES = 36193```</span>
  Float16OES = 36193,
  /// C++ enum variant: <span style='color: green;'>```Float32_D32_UInt32_S8_X24 = 36269```</span>
  Float32D32UInt32S8X24 = 36269,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::SwizzleComponent```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SwizzleComponent {
  /// C++ enum variant: <span style='color: green;'>```SwizzleRed = 36418```</span>
  Red = 36418,
  /// C++ enum variant: <span style='color: green;'>```SwizzleGreen = 36419```</span>
  Green = 36419,
  /// C++ enum variant: <span style='color: green;'>```SwizzleBlue = 36420```</span>
  Blue = 36420,
  /// C++ enum variant: <span style='color: green;'>```SwizzleAlpha = 36421```</span>
  Alpha = 36421,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::SwizzleValue```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SwizzleValue {
  /// C++ enum variant: <span style='color: green;'>```ZeroValue = 0```</span>
  Zero = 0,
  /// C++ enum variant: <span style='color: green;'>```OneValue = 1```</span>
  One = 1,
  /// C++ enum variant: <span style='color: green;'>```RedValue = 6403```</span>
  Red = 6403,
  /// C++ enum variant: <span style='color: green;'>```GreenValue = 6404```</span>
  Green = 6404,
  /// C++ enum variant: <span style='color: green;'>```BlueValue = 6405```</span>
  Blue = 6405,
  /// C++ enum variant: <span style='color: green;'>```AlphaValue = 6406```</span>
  Alpha = 6406,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::Target```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Target {
  /// C++ enum variant: <span style='color: green;'>```Target1D = 3552```</span>
  Target1D = 3552,
  /// C++ enum variant: <span style='color: green;'>```Target2D = 3553```</span>
  Target2D = 3553,
  /// C++ enum variant: <span style='color: green;'>```Target3D = 32879```</span>
  Target3D = 32879,
  /// C++ enum variant: <span style='color: green;'>```TargetRectangle = 34037```</span>
  TargetRectangle = 34037,
  /// C++ enum variant: <span style='color: green;'>```TargetCubeMap = 34067```</span>
  TargetCubeMap = 34067,
  /// C++ enum variant: <span style='color: green;'>```Target1DArray = 35864```</span>
  Target1DArray = 35864,
  /// C++ enum variant: <span style='color: green;'>```Target2DArray = 35866```</span>
  Target2DArray = 35866,
  /// C++ enum variant: <span style='color: green;'>```TargetBuffer = 35882```</span>
  TargetBuffer = 35882,
  /// C++ enum variant: <span style='color: green;'>```TargetCubeMapArray = 36873```</span>
  TargetCubeMapArray = 36873,
  /// C++ enum variant: <span style='color: green;'>```Target2DMultisample = 37120```</span>
  Target2DMultisample = 37120,
  /// C++ enum variant: <span style='color: green;'>```Target2DMultisampleArray = 37122```</span>
  Target2DMultisampleArray = 37122,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::TextureFormat```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TextureFormat {
  /// C++ enum variant: <span style='color: green;'>```NoFormat = 0```</span>
  NoFormat = 0,
  /// C++ enum variant: <span style='color: green;'>```DepthFormat = 6402```</span>
  DepthFormat = 6402,
  /// C++ enum variant: <span style='color: green;'>```AlphaFormat = 6406```</span>
  AlphaFormat = 6406,
  /// C++ enum variant: <span style='color: green;'>```RGBFormat = 6407```</span>
  RGBFormat = 6407,
  /// C++ enum variant: <span style='color: green;'>```RGBAFormat = 6408```</span>
  RGBAFormat = 6408,
  /// C++ enum variant: <span style='color: green;'>```LuminanceFormat = 6409```</span>
  LuminanceFormat = 6409,
  /// C++ enum variant: <span style='color: green;'>```LuminanceAlphaFormat = 6410```</span>
  LuminanceAlphaFormat = 6410,
  /// C++ enum variant: <span style='color: green;'>```RG3B2 = 10768```</span>
  RG3B2 = 10768,
  /// C++ enum variant: <span style='color: green;'>```RGB8_UNorm = 32849```</span>
  RGB8UNorm = 32849,
  /// C++ enum variant: <span style='color: green;'>```RGB16_UNorm = 32852```</span>
  RGB16UNorm = 32852,
  /// C++ enum variant: <span style='color: green;'>```RGBA4 = 32854```</span>
  RGBA4 = 32854,
  /// C++ enum variant: <span style='color: green;'>```RGB5A1 = 32855```</span>
  RGB5A1 = 32855,
  /// C++ enum variant: <span style='color: green;'>```RGBA8_UNorm = 32856```</span>
  RGBA8UNorm = 32856,
  /// C++ enum variant: <span style='color: green;'>```RGBA16_UNorm = 32859```</span>
  RGBA16UNorm = 32859,
  /// C++ enum variant: <span style='color: green;'>```D16 = 33189```</span>
  D16 = 33189,
  /// C++ enum variant: <span style='color: green;'>```D24 = 33190```</span>
  D24 = 33190,
  /// C++ enum variant: <span style='color: green;'>```D32 = 33191```</span>
  D32 = 33191,
  /// C++ enum variant: <span style='color: green;'>```R8_UNorm = 33321```</span>
  R8UNorm = 33321,
  /// C++ enum variant: <span style='color: green;'>```R16_UNorm = 33322```</span>
  R16UNorm = 33322,
  /// C++ enum variant: <span style='color: green;'>```RG8_UNorm = 33323```</span>
  RG8UNorm = 33323,
  /// C++ enum variant: <span style='color: green;'>```RG16_UNorm = 33324```</span>
  RG16UNorm = 33324,
  /// C++ enum variant: <span style='color: green;'>```R16F = 33325```</span>
  R16F = 33325,
  /// C++ enum variant: <span style='color: green;'>```R32F = 33326```</span>
  R32F = 33326,
  /// C++ enum variant: <span style='color: green;'>```RG16F = 33327```</span>
  RG16F = 33327,
  /// C++ enum variant: <span style='color: green;'>```RG32F = 33328```</span>
  RG32F = 33328,
  /// C++ enum variant: <span style='color: green;'>```R8I = 33329```</span>
  R8I = 33329,
  /// C++ enum variant: <span style='color: green;'>```R8U = 33330```</span>
  R8U = 33330,
  /// C++ enum variant: <span style='color: green;'>```R16I = 33331```</span>
  R16I = 33331,
  /// C++ enum variant: <span style='color: green;'>```R16U = 33332```</span>
  R16U = 33332,
  /// C++ enum variant: <span style='color: green;'>```R32I = 33333```</span>
  R32I = 33333,
  /// C++ enum variant: <span style='color: green;'>```R32U = 33334```</span>
  R32U = 33334,
  /// C++ enum variant: <span style='color: green;'>```RG8I = 33335```</span>
  RG8I = 33335,
  /// C++ enum variant: <span style='color: green;'>```RG8U = 33336```</span>
  RG8U = 33336,
  /// C++ enum variant: <span style='color: green;'>```RG16I = 33337```</span>
  RG16I = 33337,
  /// C++ enum variant: <span style='color: green;'>```RG16U = 33338```</span>
  RG16U = 33338,
  /// C++ enum variant: <span style='color: green;'>```RG32I = 33339```</span>
  RG32I = 33339,
  /// C++ enum variant: <span style='color: green;'>```RG32U = 33340```</span>
  RG32U = 33340,
  /// C++ enum variant: <span style='color: green;'>```RGB_DXT1 = 33776```</span>
  RGBDXT1 = 33776,
  /// C++ enum variant: <span style='color: green;'>```RGBA_DXT1 = 33777```</span>
  RGBADXT1 = 33777,
  /// C++ enum variant: <span style='color: green;'>```RGBA_DXT3 = 33778```</span>
  RGBADXT3 = 33778,
  /// C++ enum variant: <span style='color: green;'>```RGBA_DXT5 = 33779```</span>
  RGBADXT5 = 33779,
  /// C++ enum variant: <span style='color: green;'>```RGBA32F = 34836```</span>
  RGBA32F = 34836,
  /// C++ enum variant: <span style='color: green;'>```RGB32F = 34837```</span>
  RGB32F = 34837,
  /// C++ enum variant: <span style='color: green;'>```RGBA16F = 34842```</span>
  RGBA16F = 34842,
  /// C++ enum variant: <span style='color: green;'>```RGB16F = 34843```</span>
  RGB16F = 34843,
  /// C++ enum variant: <span style='color: green;'>```D24S8 = 35056```</span>
  D24S8 = 35056,
  /// C++ enum variant: <span style='color: green;'>```RG11B10F = 35898```</span>
  RG11B10F = 35898,
  /// C++ enum variant: <span style='color: green;'>```RGB9E5 = 35901```</span>
  RGB9E5 = 35901,
  /// C++ enum variant: <span style='color: green;'>```SRGB8 = 35905```</span>
  SRGB8 = 35905,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8 = 35907```</span>
  SRGB8Alpha8 = 35907,
  /// C++ enum variant: <span style='color: green;'>```SRGB_DXT1 = 35916```</span>
  SRGBDXT1 = 35916,
  /// C++ enum variant: <span style='color: green;'>```SRGB_Alpha_DXT1 = 35917```</span>
  SRGBAlphaDXT1 = 35917,
  /// C++ enum variant: <span style='color: green;'>```SRGB_Alpha_DXT3 = 35918```</span>
  SRGBAlphaDXT3 = 35918,
  /// C++ enum variant: <span style='color: green;'>```SRGB_Alpha_DXT5 = 35919```</span>
  SRGBAlphaDXT5 = 35919,
  /// C++ enum variant: <span style='color: green;'>```D32F = 36012```</span>
  D32F = 36012,
  /// C++ enum variant: <span style='color: green;'>```D32FS8X24 = 36013```</span>
  D32FS8X24 = 36013,
  /// C++ enum variant: <span style='color: green;'>```S8 = 36168```</span>
  S8 = 36168,
  /// C++ enum variant: <span style='color: green;'>```R5G6B5 = 36194```</span>
  R5G6B5 = 36194,
  /// C++ enum variant: <span style='color: green;'>```RGB8_ETC1 = 36196```</span>
  RGB8ETC1 = 36196,
  /// C++ enum variant: <span style='color: green;'>```RGBA32U = 36208```</span>
  RGBA32U = 36208,
  /// C++ enum variant: <span style='color: green;'>```RGB32U = 36209```</span>
  RGB32U = 36209,
  /// C++ enum variant: <span style='color: green;'>```RGBA16U = 36214```</span>
  RGBA16U = 36214,
  /// C++ enum variant: <span style='color: green;'>```RGB16U = 36215```</span>
  RGB16U = 36215,
  /// C++ enum variant: <span style='color: green;'>```RGBA8U = 36220```</span>
  RGBA8U = 36220,
  /// C++ enum variant: <span style='color: green;'>```RGB8U = 36221```</span>
  RGB8U = 36221,
  /// C++ enum variant: <span style='color: green;'>```RGBA32I = 36226```</span>
  RGBA32I = 36226,
  /// C++ enum variant: <span style='color: green;'>```RGB32I = 36227```</span>
  RGB32I = 36227,
  /// C++ enum variant: <span style='color: green;'>```RGBA16I = 36232```</span>
  RGBA16I = 36232,
  /// C++ enum variant: <span style='color: green;'>```RGB16I = 36233```</span>
  RGB16I = 36233,
  /// C++ enum variant: <span style='color: green;'>```RGBA8I = 36238```</span>
  RGBA8I = 36238,
  /// C++ enum variant: <span style='color: green;'>```RGB8I = 36239```</span>
  RGB8I = 36239,
  /// C++ enum variant: <span style='color: green;'>```R_ATI1N_UNorm = 36283```</span>
  RATI1NUNorm = 36283,
  /// C++ enum variant: <span style='color: green;'>```R_ATI1N_SNorm = 36284```</span>
  RATI1NSNorm = 36284,
  /// C++ enum variant: <span style='color: green;'>```RG_ATI2N_UNorm = 36285```</span>
  RGATI2NUNorm = 36285,
  /// C++ enum variant: <span style='color: green;'>```RG_ATI2N_SNorm = 36286```</span>
  RGATI2NSNorm = 36286,
  /// C++ enum variant: <span style='color: green;'>```RGB_BP_UNorm = 36492```</span>
  RGBBPUNorm = 36492,
  /// C++ enum variant: <span style='color: green;'>```SRGB_BP_UNorm = 36493```</span>
  SRGBBPUNorm = 36493,
  /// C++ enum variant: <span style='color: green;'>```RGB_BP_SIGNED_FLOAT = 36494```</span>
  RGBBPSIGNEDFLOAT = 36494,
  /// C++ enum variant: <span style='color: green;'>```RGB_BP_UNSIGNED_FLOAT = 36495```</span>
  RGBBPUNSIGNEDFLOAT = 36495,
  /// C++ enum variant: <span style='color: green;'>```R8_SNorm = 36756```</span>
  R8SNorm = 36756,
  /// C++ enum variant: <span style='color: green;'>```RG8_SNorm = 36757```</span>
  RG8SNorm = 36757,
  /// C++ enum variant: <span style='color: green;'>```RGB8_SNorm = 36758```</span>
  RGB8SNorm = 36758,
  /// C++ enum variant: <span style='color: green;'>```RGBA8_SNorm = 36759```</span>
  RGBA8SNorm = 36759,
  /// C++ enum variant: <span style='color: green;'>```R16_SNorm = 36760```</span>
  R16SNorm = 36760,
  /// C++ enum variant: <span style='color: green;'>```RG16_SNorm = 36761```</span>
  RG16SNorm = 36761,
  /// C++ enum variant: <span style='color: green;'>```RGB16_SNorm = 36762```</span>
  RGB16SNorm = 36762,
  /// C++ enum variant: <span style='color: green;'>```RGBA16_SNorm = 36763```</span>
  RGBA16SNorm = 36763,
  /// C++ enum variant: <span style='color: green;'>```RGB10A2 = 36975```</span>
  RGB10A2 = 36975,
  /// C++ enum variant: <span style='color: green;'>```R11_EAC_UNorm = 37488```</span>
  R11EACUNorm = 37488,
  /// C++ enum variant: <span style='color: green;'>```R11_EAC_SNorm = 37489```</span>
  R11EACSNorm = 37489,
  /// C++ enum variant: <span style='color: green;'>```RG11_EAC_UNorm = 37490```</span>
  RG11EACUNorm = 37490,
  /// C++ enum variant: <span style='color: green;'>```RG11_EAC_SNorm = 37491```</span>
  RG11EACSNorm = 37491,
  /// C++ enum variant: <span style='color: green;'>```RGB8_ETC2 = 37492```</span>
  RGB8ETC2 = 37492,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_ETC2 = 37493```</span>
  SRGB8ETC2 = 37493,
  /// C++ enum variant: <span style='color: green;'>```RGB8_PunchThrough_Alpha1_ETC2 = 37494```</span>
  RGB8PunchThroughAlpha1ETC2 = 37494,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_PunchThrough_Alpha1_ETC2 = 37495```</span>
  SRGB8PunchThroughAlpha1ETC2 = 37495,
  /// C++ enum variant: <span style='color: green;'>```RGBA8_ETC2_EAC = 37496```</span>
  RGBA8ETC2EAC = 37496,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ETC2_EAC = 37497```</span>
  SRGB8Alpha8ETC2EAC = 37497,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_4x4 = 37808```</span>
  RGBAASTC4X4 = 37808,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_5x4 = 37809```</span>
  RGBAASTC5X4 = 37809,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_5x5 = 37810```</span>
  RGBAASTC5X5 = 37810,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_6x5 = 37811```</span>
  RGBAASTC6X5 = 37811,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_6x6 = 37812```</span>
  RGBAASTC6X6 = 37812,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_8x5 = 37813```</span>
  RGBAASTC8X5 = 37813,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_8x6 = 37814```</span>
  RGBAASTC8X6 = 37814,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_8x8 = 37815```</span>
  RGBAASTC8X8 = 37815,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_10x5 = 37816```</span>
  RGBAASTC10X5 = 37816,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_10x6 = 37817```</span>
  RGBAASTC10X6 = 37817,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_10x8 = 37818```</span>
  RGBAASTC10X8 = 37818,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_10x10 = 37819```</span>
  RGBAASTC10X10 = 37819,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_12x10 = 37820```</span>
  RGBAASTC12X10 = 37820,
  /// C++ enum variant: <span style='color: green;'>```RGBA_ASTC_12x12 = 37821```</span>
  RGBAASTC12X12 = 37821,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_4x4 = 37840```</span>
  SRGB8Alpha8ASTC4X4 = 37840,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_5x4 = 37841```</span>
  SRGB8Alpha8ASTC5X4 = 37841,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_5x5 = 37842```</span>
  SRGB8Alpha8ASTC5X5 = 37842,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_6x5 = 37843```</span>
  SRGB8Alpha8ASTC6X5 = 37843,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_6x6 = 37844```</span>
  SRGB8Alpha8ASTC6X6 = 37844,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_8x5 = 37845```</span>
  SRGB8Alpha8ASTC8X5 = 37845,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_8x6 = 37846```</span>
  SRGB8Alpha8ASTC8X6 = 37846,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_8x8 = 37847```</span>
  SRGB8Alpha8ASTC8X8 = 37847,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_10x5 = 37848```</span>
  SRGB8Alpha8ASTC10X5 = 37848,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_10x6 = 37849```</span>
  SRGB8Alpha8ASTC10X6 = 37849,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_10x8 = 37850```</span>
  SRGB8Alpha8ASTC10X8 = 37850,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_10x10 = 37851```</span>
  SRGB8Alpha8ASTC10X10 = 37851,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_12x10 = 37852```</span>
  SRGB8Alpha8ASTC12X10 = 37852,
  /// C++ enum variant: <span style='color: green;'>```SRGB8_Alpha8_ASTC_12x12 = 37853```</span>
  SRGB8Alpha8ASTC12X12 = 37853,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::TextureUnitReset```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TextureUnitReset {
  /// C++ enum variant: <span style='color: green;'>```ResetTextureUnit = 0```</span>
  ResetTextureUnit = 0,
  /// C++ enum variant: <span style='color: green;'>```DontResetTextureUnit = 1```</span>
  DontResetTextureUnit = 1,
}

/// C++ type: <span style='color: green;'>```QOpenGLTexture::WrapMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum WrapMode {
  /// C++ enum variant: <span style='color: green;'>```Repeat = 10497```</span>
  Repeat = 10497,
  /// C++ enum variant: <span style='color: green;'>```ClampToBorder = 33069```</span>
  ClampToBorder = 33069,
  /// C++ enum variant: <span style='color: green;'>```ClampToEdge = 33071```</span>
  ClampToEdge = 33071,
  /// C++ enum variant: <span style='color: green;'>```MirroredRepeat = 33648```</span>
  MirroredRepeat = 33648,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLTexture::allocate_storage](../struct.OpenGLTexture.html#method.allocate_storage) method.
  pub trait OpenGLTextureAllocateStorageArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureAllocateStorageArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {

      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_allocateStorage_no_args(original_self as *mut ::opengl_texture::OpenGLTexture)
      }
    }
  }
  impl<'largs> OpenGLTextureAllocateStorageArgs<'largs> for (::opengl_texture::PixelFormat, ::opengl_texture::PixelType) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let pixel_format = self.0;
      let pixel_type = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_allocateStorage_pixelFormat_pixelType(original_self as *mut ::opengl_texture::OpenGLTexture, pixel_format, pixel_type) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLTexture::bind](../struct.OpenGLTexture.html#method.bind) method.
  pub trait OpenGLTextureBindArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureBindArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {

      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_bind_no_args(original_self as *mut ::opengl_texture::OpenGLTexture) }
    }
  }
  impl<'largs> OpenGLTextureBindArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let unit = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_bind_unit(original_self as *mut ::opengl_texture::OpenGLTexture, unit) }
    }
  }
  impl<'largs> OpenGLTextureBindArgs<'largs> for (::libc::c_uint, ::opengl_texture::TextureUnitReset) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let unit = self.0;
      let reset = self.1;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_bind_unit_reset(original_self as *mut ::opengl_texture::OpenGLTexture,
                                                       unit,
                                                       reset)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLTexture::border_color_unsafe](../struct.OpenGLTexture.html#method.border_color_unsafe) method.
  pub trait OpenGLTextureBorderColorUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureBorderColorUnsafeArgs<'largs> for *mut ::libc::c_float {
    unsafe fn exec(self, original_self: &'largs ::opengl_texture::OpenGLTexture) -> () {
      let border = self;
      ::ffi::qt_gui_c_QOpenGLTexture_borderColor_float(original_self as *const ::opengl_texture::OpenGLTexture,
                                                       border)
    }
  }
  impl<'largs> OpenGLTextureBorderColorUnsafeArgs<'largs> for *mut ::libc::c_int {
    unsafe fn exec(self, original_self: &'largs ::opengl_texture::OpenGLTexture) -> () {
      let border = self;
      ::ffi::qt_gui_c_QOpenGLTexture_borderColor_int(original_self as *const ::opengl_texture::OpenGLTexture,
                                                     border)
    }
  }
  impl<'largs> OpenGLTextureBorderColorUnsafeArgs<'largs> for *mut ::libc::c_uint {
    unsafe fn exec(self, original_self: &'largs ::opengl_texture::OpenGLTexture) -> () {
      let border = self;
      ::ffi::qt_gui_c_QOpenGLTexture_borderColor_unsigned_int(original_self as *const ::opengl_texture::OpenGLTexture,
                                                              border)
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLTexture::bound_texture_id](../struct.OpenGLTexture.html#method.bound_texture_id) method.
  pub trait OpenGLTextureBoundTextureIdArgs {
    fn exec(self) -> u32;
  }
  impl OpenGLTextureBoundTextureIdArgs for ::opengl_texture::BindingTarget {
    fn exec(self) -> u32 {
      let target = self;
      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_boundTextureId_target(target) }
    }
  }
  impl OpenGLTextureBoundTextureIdArgs for (::libc::c_uint, ::opengl_texture::BindingTarget) {
    fn exec(self) -> u32 {
      let unit = self.0;
      let target = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_boundTextureId_unit_target(unit, target) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLTexture::generate_mip_maps](../struct.OpenGLTexture.html#method.generate_mip_maps) method.
  pub trait OpenGLTextureGenerateMipMapsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureGenerateMipMapsArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let base_level = self;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_generateMipMaps_baseLevel(original_self as *mut ::opengl_texture::OpenGLTexture, base_level)
      }
    }
  }
  impl<'largs> OpenGLTextureGenerateMipMapsArgs<'largs> for (::libc::c_int, bool) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let base_level = self.0;
      let reset_base_level = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_generateMipMaps_baseLevel_resetBaseLevel(original_self as *mut ::opengl_texture::OpenGLTexture, base_level, reset_base_level) }
    }
  }
  impl<'largs> OpenGLTextureGenerateMipMapsArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {

      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_generateMipMaps_no_args(original_self as *mut ::opengl_texture::OpenGLTexture)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLTexture::new](../struct.OpenGLTexture.html#method.new) method.
  pub trait OpenGLTextureNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_texture::OpenGLTexture>;
  }
  impl<'a> OpenGLTextureNewArgs for &'a ::image::Image {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_texture::OpenGLTexture> {
      let image = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLTexture_new_image(image as *const ::image::Image) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> OpenGLTextureNewArgs for (&'a ::image::Image, ::opengl_texture::MipMapGeneration) {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_texture::OpenGLTexture> {
      let image = self.0;
      let gen_mip_maps = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QOpenGLTexture_new_image_genMipMaps(image as *const ::image::Image, gen_mip_maps) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl OpenGLTextureNewArgs for ::opengl_texture::Target {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_texture::OpenGLTexture> {
      let target = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLTexture_new_target(target) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLTexture::release](../struct.OpenGLTexture.html#method.release) method.
  pub trait OpenGLTextureReleaseArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureReleaseArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {

      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_release_no_args(original_self as *mut ::opengl_texture::OpenGLTexture) }
    }
  }
  impl<'largs> OpenGLTextureReleaseArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let unit = self;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_release_unit(original_self as *mut ::opengl_texture::OpenGLTexture, unit)
      }
    }
  }
  impl<'largs> OpenGLTextureReleaseArgs<'largs> for (::libc::c_uint, ::opengl_texture::TextureUnitReset) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let unit = self.0;
      let reset = self.1;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_release_unit_reset(original_self as *mut ::opengl_texture::OpenGLTexture,
                                                          unit,
                                                          reset)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLTexture::set_border_color](../struct.OpenGLTexture.html#method.set_border_color) method.
  pub trait OpenGLTextureSetBorderColorArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureSetBorderColorArgs<'largs> for &'largs ::color::Color {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let color = self;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_setBorderColor_QColor(original_self as *mut ::opengl_texture::OpenGLTexture,
                                                             color as *const ::color::Color)
      }
    }
  }
  impl<'largs> OpenGLTextureSetBorderColorArgs<'largs>
    for (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setBorderColor_float_float_float_float(original_self as *mut ::opengl_texture::OpenGLTexture, r, g, b, a) }
    }
  }
  impl<'largs> OpenGLTextureSetBorderColorArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setBorderColor_int_int_int_int(original_self as *mut ::opengl_texture::OpenGLTexture, r, g, b, a) }
    }
  }
  impl<'largs> OpenGLTextureSetBorderColorArgs<'largs>
    for (::libc::c_uint, ::libc::c_uint, ::libc::c_uint, ::libc::c_uint) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setBorderColor_unsigned_int_unsigned_int_unsigned_int_unsigned_int(original_self as *mut ::opengl_texture::OpenGLTexture, r, g, b, a) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLTexture::set_compressed_data](../struct.OpenGLTexture.html#method.set_compressed_data) method.
  pub trait OpenGLTextureSetCompressedDataArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int, *const ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let data_size = self.0;
      let data = self.1;
      ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_const_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, data_size, data)
    }
  }
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int,*const ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let data_size = self.0;
let data = self.1;
let options = self.2;
    ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, data_size, data, options)
  }
}
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::libc::c_int, *const ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let layer = self.1;
      let cube_face = self.2;
      let data_size = self.3;
      let data = self.4;
      ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_QOpenGLTexture_CubeMapFace_int_const_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, cube_face, data_size, data)
    }
  }
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int,::libc::c_int,::opengl_texture::CubeMapFace,::libc::c_int,*const ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let layer = self.1;
let cube_face = self.2;
let data_size = self.3;
let data = self.4;
let options = self.5;
    ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_QOpenGLTexture_CubeMapFace_int_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, cube_face, data_size, data, options)
  }
}
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::opengl_texture::CubeMapFace, ::libc::c_int, *mut ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let layer = self.1;
      let cube_face = self.2;
      let data_size = self.3;
      let data = self.4;
      ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_QOpenGLTexture_CubeMapFace_int_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, cube_face, data_size, data)
    }
  }
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int,::libc::c_int,::opengl_texture::CubeMapFace,::libc::c_int,*mut ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let layer = self.1;
let cube_face = self.2;
let data_size = self.3;
let data = self.4;
let options = self.5;
    ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_QOpenGLTexture_CubeMapFace_int_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, cube_face, data_size, data, options)
  }
}
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int, ::libc::c_int, *const ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let data_size = self.1;
      let data = self.2;
      ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_const_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, data_size, data)
    }
  }
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int,::libc::c_int,*const ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let data_size = self.1;
let data = self.2;
let options = self.3;
    ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, data_size, data, options)
  }
}
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs>
    for (::libc::c_int,
                                                                   ::libc::c_int,
                                                                   ::libc::c_int,
                                                                   ::opengl_texture::CubeMapFace,
                                                                   ::libc::c_int,
                                                                   *const ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let layer = self.1;
      let layer_count = self.2;
      let cube_face = self.3;
      let data_size = self.4;
      let data = self.5;
      ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_QOpenGLTexture_CubeMapFace_int_const_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, layer_count, cube_face, data_size, data)
    }
  }
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int,::libc::c_int,::libc::c_int,::opengl_texture::CubeMapFace,::libc::c_int,*const ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let layer = self.1;
let layer_count = self.2;
let cube_face = self.3;
let data_size = self.4;
let data = self.5;
let options = self.6;
    ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_QOpenGLTexture_CubeMapFace_int_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, layer_count, cube_face, data_size, data, options)
  }
}
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, *const ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let layer = self.1;
      let data_size = self.2;
      let data = self.3;
      ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_const_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, data_size, data)
    }
  }
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int,::libc::c_int,::libc::c_int,*const ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let layer = self.1;
let data_size = self.2;
let data = self.3;
let options = self.4;
    ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, data_size, data, options)
  }
}
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, *mut ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let layer = self.1;
      let data_size = self.2;
      let data = self.3;
      ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, data_size, data)
    }
  }
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int,::libc::c_int,::libc::c_int,*mut ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let layer = self.1;
let data_size = self.2;
let data = self.3;
let options = self.4;
    ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_int_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, data_size, data, options)
  }
}
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int, ::libc::c_int, *mut ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let data_size = self.1;
      let data = self.2;
      ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, data_size, data)
    }
  }
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int,::libc::c_int,*mut ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let data_size = self.1;
let data = self.2;
let options = self.3;
    ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_int_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, data_size, data, options)
  }
}
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int, *mut ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let data_size = self.0;
      let data = self.1;
      ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, data_size, data)
    }
  }
  impl<'largs> OpenGLTextureSetCompressedDataArgs<'largs> for (::libc::c_int,*mut ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let data_size = self.0;
let data = self.1;
let options = self.2;
    ::ffi::qt_gui_c_QOpenGLTexture_setCompressedData_int_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, data_size, data, options)
  }
}
  /// This trait represents a set of arguments accepted by [OpenGLTexture::set_data](../struct.OpenGLTexture.html#method.set_data) method.
  pub trait OpenGLTextureSetDataArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureSetDataArgs<'largs> for &'largs ::image::Image {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let image = self;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_setData_const_QImage_ref(original_self as *mut ::opengl_texture::OpenGLTexture,
                                                                image as *const ::image::Image)
      }
    }
  }
  impl<'largs> OpenGLTextureSetDataArgs<'largs> for (&'largs ::image::Image, ::opengl_texture::MipMapGeneration) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let image = self.0;
      let gen_mip_maps = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setData_const_QImage_ref_QOpenGLTexture_MipMapGeneration(original_self as *mut ::opengl_texture::OpenGLTexture, image as *const ::image::Image, gen_mip_maps) }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLTexture::set_data_unsafe](../struct.OpenGLTexture.html#method.set_data_unsafe) method.
  pub trait OpenGLTextureSetDataUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs>
    for (::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let source_format = self.0;
      let source_type = self.1;
      let data = self.2;
      ::ffi::qt_gui_c_QOpenGLTexture_setData_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, source_format, source_type, data)
    }
  }
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs> for (::opengl_texture::PixelFormat,::opengl_texture::PixelType,*const ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let source_format = self.0;
let source_type = self.1;
let data = self.2;
let options = self.3;
    ::ffi::qt_gui_c_QOpenGLTexture_setData_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, source_format, source_type, data, options)
  }
}
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs>
    for (::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *mut ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let source_format = self.0;
      let source_type = self.1;
      let data = self.2;
      ::ffi::qt_gui_c_QOpenGLTexture_setData_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, source_format, source_type, data)
    }
  }
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs> for (::opengl_texture::PixelFormat,::opengl_texture::PixelType,*mut ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let source_format = self.0;
let source_type = self.1;
let data = self.2;
let options = self.3;
    ::ffi::qt_gui_c_QOpenGLTexture_setData_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, source_format, source_type, data, options)
  }
}
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs>
    for (::libc::c_int, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *const ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let source_format = self.1;
      let source_type = self.2;
      let data = self.3;
      ::ffi::qt_gui_c_QOpenGLTexture_setData_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, source_format, source_type, data)
    }
  }
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs> for (::libc::c_int,::opengl_texture::PixelFormat,::opengl_texture::PixelType,*const ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let source_format = self.1;
let source_type = self.2;
let data = self.3;
let options = self.4;
    ::ffi::qt_gui_c_QOpenGLTexture_setData_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, source_format, source_type, data, options)
  }
}
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs>
    for (::libc::c_int, ::opengl_texture::PixelFormat, ::opengl_texture::PixelType, *mut ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let source_format = self.1;
      let source_type = self.2;
      let data = self.3;
      ::ffi::qt_gui_c_QOpenGLTexture_setData_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, source_format, source_type, data)
    }
  }
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs> for (::libc::c_int,::opengl_texture::PixelFormat,::opengl_texture::PixelType,*mut ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let source_format = self.1;
let source_type = self.2;
let data = self.3;
let options = self.4;
    ::ffi::qt_gui_c_QOpenGLTexture_setData_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, source_format, source_type, data, options)
  }
}
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs>
    for (::libc::c_int,
                                                               ::libc::c_int,
                                                               ::opengl_texture::CubeMapFace,
                                                               ::opengl_texture::PixelFormat,
                                                               ::opengl_texture::PixelType,
                                                               *const ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let layer = self.1;
      let cube_face = self.2;
      let source_format = self.3;
      let source_type = self.4;
      let data = self.5;
      ::ffi::qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, cube_face, source_format, source_type, data)
    }
  }
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs> for (::libc::c_int,::libc::c_int,::opengl_texture::CubeMapFace,::opengl_texture::PixelFormat,::opengl_texture::PixelType,*const ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let layer = self.1;
let cube_face = self.2;
let source_format = self.3;
let source_type = self.4;
let data = self.5;
let options = self.6;
    ::ffi::qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, cube_face, source_format, source_type, data, options)
  }
}
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs>
    for (::libc::c_int,
                                                               ::libc::c_int,
                                                               ::opengl_texture::CubeMapFace,
                                                               ::opengl_texture::PixelFormat,
                                                               ::opengl_texture::PixelType,
                                                               *mut ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let layer = self.1;
      let cube_face = self.2;
      let source_format = self.3;
      let source_type = self.4;
      let data = self.5;
      ::ffi::qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, cube_face, source_format, source_type, data)
    }
  }
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs> for (::libc::c_int,::libc::c_int,::opengl_texture::CubeMapFace,::opengl_texture::PixelFormat,::opengl_texture::PixelType,*mut ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let layer = self.1;
let cube_face = self.2;
let source_format = self.3;
let source_type = self.4;
let data = self.5;
let options = self.6;
    ::ffi::qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, cube_face, source_format, source_type, data, options)
  }
}
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs>
    for (::libc::c_int,
                                                               ::libc::c_int,
                                                               ::opengl_texture::PixelFormat,
                                                               ::opengl_texture::PixelType,
                                                               *const ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let layer = self.1;
      let source_format = self.2;
      let source_type = self.3;
      let data = self.4;
      ::ffi::qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, source_format, source_type, data)
    }
  }
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs> for (::libc::c_int,::libc::c_int,::opengl_texture::PixelFormat,::opengl_texture::PixelType,*const ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let layer = self.1;
let source_format = self.2;
let source_type = self.3;
let data = self.4;
let options = self.5;
    ::ffi::qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, source_format, source_type, data, options)
  }
}
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs>
    for (::libc::c_int,
                                                               ::libc::c_int,
                                                               ::opengl_texture::PixelFormat,
                                                               ::opengl_texture::PixelType,
                                                               *mut ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let layer = self.1;
      let source_format = self.2;
      let source_type = self.3;
      let data = self.4;
      ::ffi::qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, source_format, source_type, data)
    }
  }
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs> for (::libc::c_int,::libc::c_int,::opengl_texture::PixelFormat,::opengl_texture::PixelType,*mut ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let layer = self.1;
let source_format = self.2;
let source_type = self.3;
let data = self.4;
let options = self.5;
    ::ffi::qt_gui_c_QOpenGLTexture_setData_int_int_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, source_format, source_type, data, options)
  }
}
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs>
    for (::libc::c_int,
                                                               ::libc::c_int,
                                                               ::libc::c_int,
                                                               ::opengl_texture::CubeMapFace,
                                                               ::opengl_texture::PixelFormat,
                                                               ::opengl_texture::PixelType,
                                                               *const ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mip_level = self.0;
      let layer = self.1;
      let layer_count = self.2;
      let cube_face = self.3;
      let source_format = self.4;
      let source_type = self.5;
      let data = self.6;
      ::ffi::qt_gui_c_QOpenGLTexture_setData_int_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, layer_count, cube_face, source_format, source_type, data)
    }
  }
  impl<'largs> OpenGLTextureSetDataUnsafeArgs<'largs> for (::libc::c_int,::libc::c_int,::libc::c_int,::opengl_texture::CubeMapFace,::opengl_texture::PixelFormat,::opengl_texture::PixelType,*const ::libc::c_void,*const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {

  unsafe fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
    let mip_level = self.0;
let layer = self.1;
let layer_count = self.2;
let cube_face = self.3;
let source_format = self.4;
let source_type = self.5;
let data = self.6;
let options = self.7;
    ::ffi::qt_gui_c_QOpenGLTexture_setData_int_int_int_QOpenGLTexture_CubeMapFace_QOpenGLTexture_PixelFormat_QOpenGLTexture_PixelType_const_void_ptr_const_QOpenGLPixelTransferOptions_ptr(original_self as *mut ::opengl_texture::OpenGLTexture, mip_level, layer, layer_count, cube_face, source_format, source_type, data, options)
  }
}
  /// This trait represents a set of arguments accepted by [OpenGLTexture::set_size](../struct.OpenGLTexture.html#method.set_size) method.
  pub trait OpenGLTextureSetSizeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureSetSizeArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let width = self;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_setSize_width(original_self as *mut ::opengl_texture::OpenGLTexture, width)
      }
    }
  }
  impl<'largs> OpenGLTextureSetSizeArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let width = self.0;
      let height = self.1;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_setSize_width_height(original_self as *mut ::opengl_texture::OpenGLTexture,
                                                            width,
                                                            height)
      }
    }
  }
  impl<'largs> OpenGLTextureSetSizeArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let width = self.0;
      let height = self.1;
      let depth = self.2;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_setSize_width_height_depth(original_self as *mut ::opengl_texture::OpenGLTexture, width, height, depth)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLTexture::set_swizzle_mask](../struct.OpenGLTexture.html#method.set_swizzle_mask) method.
  pub trait OpenGLTextureSetSwizzleMaskArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureSetSwizzleMaskArgs<'largs>
    for (::opengl_texture::SwizzleComponent, ::opengl_texture::SwizzleValue) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let component = self.0;
      let value = self.1;
      unsafe { ::ffi::qt_gui_c_QOpenGLTexture_setSwizzleMask_component_value(original_self as *mut ::opengl_texture::OpenGLTexture, component, value) }
    }
  }
  impl<'largs> OpenGLTextureSetSwizzleMaskArgs<'largs>
    for (::opengl_texture::SwizzleValue,
                                                                ::opengl_texture::SwizzleValue,
                                                                ::opengl_texture::SwizzleValue,
                                                                ::opengl_texture::SwizzleValue) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_setSwizzleMask_r_g_b_a(original_self as *mut ::opengl_texture::OpenGLTexture,
                                                              r,
                                                              g,
                                                              b,
                                                              a)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [OpenGLTexture::set_wrap_mode](../struct.OpenGLTexture.html#method.set_wrap_mode) method.
  pub trait OpenGLTextureSetWrapModeArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> ();
  }
  impl<'largs> OpenGLTextureSetWrapModeArgs<'largs>
    for (::opengl_texture::CoordinateDirection, ::opengl_texture::WrapMode) {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let direction = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_setWrapMode_direction_mode(original_self as *mut ::opengl_texture::OpenGLTexture, direction, mode)
      }
    }
  }
  impl<'largs> OpenGLTextureSetWrapModeArgs<'largs> for ::opengl_texture::WrapMode {
    fn exec(self, original_self: &'largs mut ::opengl_texture::OpenGLTexture) -> () {
      let mode = self;
      unsafe {
        ::ffi::qt_gui_c_QOpenGLTexture_setWrapMode_mode(original_self as *mut ::opengl_texture::OpenGLTexture, mode)
      }
    }
  }
}
