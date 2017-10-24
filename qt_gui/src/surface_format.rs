/// C++ type: <span style='color: green;'>```QSurfaceFormat::FormatOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum FormatOption {
  /// C++ enum variant: <span style='color: green;'>```StereoBuffers = 1```</span>
  StereoBuffers = 1,
  /// C++ enum variant: <span style='color: green;'>```DebugContext = 2```</span>
  DebugContext = 2,
  /// C++ enum variant: <span style='color: green;'>```DeprecatedFunctions = 4```</span>
  DeprecatedFunctions = 4,
  /// C++ enum variant: <span style='color: green;'>```ResetNotification = 8```</span>
  ResetNotification = 8,
}

impl ::qt_core::flags::FlaggableEnum for FormatOption {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "FormatOption"
  }
}

/// C++ type: <span style='color: green;'>```QSurfaceFormat::OpenGLContextProfile```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum OpenGLContextProfile {
  /// C++ enum variant: <span style='color: green;'>```NoProfile = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```CoreProfile = 1```</span>
  Core = 1,
  /// C++ enum variant: <span style='color: green;'>```CompatibilityProfile = 2```</span>
  Compatibility = 2,
}

/// C++ type: <span style='color: green;'>```QSurfaceFormat::RenderableType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RenderableType {
  /// C++ enum variant: <span style='color: green;'>```DefaultRenderableType = 0```</span>
  DefaultRenderableType = 0,
  /// C++ enum variant: <span style='color: green;'>```OpenGL = 1```</span>
  OpenGL = 1,
  /// C++ enum variant: <span style='color: green;'>```OpenGLES = 2```</span>
  OpenGLES = 2,
  /// C++ enum variant: <span style='color: green;'>```OpenVG = 4```</span>
  OpenVG = 4,
}

/// C++ type: <span style='color: green;'>```QSurfaceFormat```</span>
#[repr(C)]
pub struct SurfaceFormat([u8; ::type_sizes::QT_GUI_SURFACE_FORMAT_SURFACE_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SurfaceFormat {
  unsafe fn new_uninitialized() -> SurfaceFormat {
    SurfaceFormat(::std::mem::uninitialized())
  }
}

impl SurfaceFormat {
  /// C++ method: <span style='color: green;'>```int QSurfaceFormat::alphaBufferSize() const```</span>
  ///
  ///
  pub fn alpha_buffer_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_alphaBufferSize(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QSurfaceFormat::blueBufferSize() const```</span>
  ///
  ///
  pub fn blue_buffer_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_blueBufferSize(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```static QSurfaceFormat QSurfaceFormat::defaultFormat()```</span>
  ///
  ///
  pub fn default_format() -> ::surface_format::SurfaceFormat {
    {
      let mut object: ::surface_format::SurfaceFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSurfaceFormat_defaultFormat_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QSurfaceFormat::depthBufferSize() const```</span>
  ///
  ///
  pub fn depth_buffer_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_depthBufferSize(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QSurfaceFormat::greenBufferSize() const```</span>
  ///
  ///
  pub fn green_buffer_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_greenBufferSize(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QSurfaceFormat::hasAlpha() const```</span>
  ///
  ///
  pub fn has_alpha(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_hasAlpha(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QSurfaceFormat::majorVersion() const```</span>
  ///
  ///
  pub fn major_version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_majorVersion(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QSurfaceFormat::minorVersion() const```</span>
  ///
  ///
  pub fn minor_version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_minorVersion(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```QSurfaceFormat::QSurfaceFormat```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::surface_format::SurfaceFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSurfaceFormat::QSurfaceFormat()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::qt_core::flags::Flags<::surface_format::FormatOption>) -> ::surface_format::SurfaceFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSurfaceFormat::QSurfaceFormat(QFlags<QSurfaceFormat::FormatOption> options)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::surface_format::SurfaceFormat) -> ::surface_format::SurfaceFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QSurfaceFormat::QSurfaceFormat(const QSurfaceFormat& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::surface_format::SurfaceFormat
    where Args: overloading::SurfaceFormatNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QSurfaceFormat& QSurfaceFormat::operator=(const QSurfaceFormat& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::surface_format::SurfaceFormat)
                             -> &'l0 mut ::surface_format::SurfaceFormat {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QSurfaceFormat_operator_assign(self as *mut ::surface_format::SurfaceFormat,
                                                     other as *const ::surface_format::SurfaceFormat)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSurfaceFormat::OpenGLContextProfile QSurfaceFormat::profile() const```</span>
  ///
  ///
  pub fn profile(&self) -> ::surface_format::OpenGLContextProfile {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_profile(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QSurfaceFormat::redBufferSize() const```</span>
  ///
  ///
  pub fn red_buffer_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_redBufferSize(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```QSurfaceFormat::RenderableType QSurfaceFormat::renderableType() const```</span>
  ///
  ///
  pub fn renderable_type(&self) -> ::surface_format::RenderableType {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_renderableType(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QSurfaceFormat::samples() const```</span>
  ///
  ///
  pub fn samples(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_samples(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setAlphaBufferSize(int size)```</span>
  ///
  ///
  pub fn set_alpha_buffer_size(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setAlphaBufferSize(self as *mut ::surface_format::SurfaceFormat, size) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setBlueBufferSize(int size)```</span>
  ///
  ///
  pub fn set_blue_buffer_size(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setBlueBufferSize(self as *mut ::surface_format::SurfaceFormat, size) }
  }

  /// C++ method: <span style='color: green;'>```static void QSurfaceFormat::setDefaultFormat(const QSurfaceFormat& format)```</span>
  ///
  ///
  pub fn set_default_format(format: &::surface_format::SurfaceFormat) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setDefaultFormat(format as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setDepthBufferSize(int size)```</span>
  ///
  ///
  pub fn set_depth_buffer_size(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setDepthBufferSize(self as *mut ::surface_format::SurfaceFormat, size) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setGreenBufferSize(int size)```</span>
  ///
  ///
  pub fn set_green_buffer_size(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setGreenBufferSize(self as *mut ::surface_format::SurfaceFormat, size) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setMajorVersion(int majorVersion)```</span>
  ///
  ///
  pub fn set_major_version(&mut self, major_version: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QSurfaceFormat_setMajorVersion(self as *mut ::surface_format::SurfaceFormat, major_version)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setMinorVersion(int minorVersion)```</span>
  ///
  ///
  pub fn set_minor_version(&mut self, minor_version: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QSurfaceFormat_setMinorVersion(self as *mut ::surface_format::SurfaceFormat, minor_version)
    }
  }

  /// C++ method: <span style='color: green;'>```QSurfaceFormat::setOption```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_option(&mut self, ::surface_format::FormatOption) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setOption(QSurfaceFormat::FormatOption option)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_option(&mut self, (::surface_format::FormatOption, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setOption(QSurfaceFormat::FormatOption option, bool on = ?)```</span>
  ///
  ///
  pub fn set_option<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::SurfaceFormatSetOptionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setProfile(QSurfaceFormat::OpenGLContextProfile profile)```</span>
  ///
  ///
  pub fn set_profile(&mut self, profile: ::surface_format::OpenGLContextProfile) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setProfile(self as *mut ::surface_format::SurfaceFormat, profile) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setRedBufferSize(int size)```</span>
  ///
  ///
  pub fn set_red_buffer_size(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setRedBufferSize(self as *mut ::surface_format::SurfaceFormat, size) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setRenderableType(QSurfaceFormat::RenderableType type)```</span>
  ///
  ///
  pub fn set_renderable_type(&mut self, type_: ::surface_format::RenderableType) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setRenderableType(self as *mut ::surface_format::SurfaceFormat, type_) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setSamples(int numSamples)```</span>
  ///
  ///
  pub fn set_samples(&mut self, num_samples: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setSamples(self as *mut ::surface_format::SurfaceFormat, num_samples) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setStencilBufferSize(int size)```</span>
  ///
  ///
  pub fn set_stencil_buffer_size(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setStencilBufferSize(self as *mut ::surface_format::SurfaceFormat, size) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setStereo(bool enable)```</span>
  ///
  ///
  pub fn set_stereo(&mut self, enable: bool) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setStereo(self as *mut ::surface_format::SurfaceFormat, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setSwapBehavior(QSurfaceFormat::SwapBehavior behavior)```</span>
  ///
  ///
  pub fn set_swap_behavior(&mut self, behavior: ::surface_format::SwapBehavior) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setSwapBehavior(self as *mut ::surface_format::SurfaceFormat, behavior) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setSwapInterval(int interval)```</span>
  ///
  ///
  pub fn set_swap_interval(&mut self, interval: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setSwapInterval(self as *mut ::surface_format::SurfaceFormat, interval) }
  }

  /// C++ method: <span style='color: green;'>```void QSurfaceFormat::setVersion(int major, int minor)```</span>
  ///
  ///
  pub fn set_version(&mut self, major: ::libc::c_int, minor: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_setVersion(self as *mut ::surface_format::SurfaceFormat, major, minor) }
  }

  /// C++ method: <span style='color: green;'>```int QSurfaceFormat::stencilBufferSize() const```</span>
  ///
  ///
  pub fn stencil_buffer_size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_stencilBufferSize(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QSurfaceFormat::stereo() const```</span>
  ///
  ///
  pub fn stereo(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_stereo(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```QSurfaceFormat::SwapBehavior QSurfaceFormat::swapBehavior() const```</span>
  ///
  ///
  pub fn swap_behavior(&self) -> ::surface_format::SwapBehavior {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_swapBehavior(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```int QSurfaceFormat::swapInterval() const```</span>
  ///
  ///
  pub fn swap_interval(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_swapInterval(self as *const ::surface_format::SurfaceFormat) }
  }

  /// C++ method: <span style='color: green;'>```bool QSurfaceFormat::testOption(QSurfaceFormat::FormatOption option) const```</span>
  ///
  ///
  pub fn test_option(&self, option: ::surface_format::FormatOption) -> bool {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_testOption(self as *const ::surface_format::SurfaceFormat, option) }
  }

  /// C++ method: <span style='color: green;'>```QPair<int, int> QSurfaceFormat::version() const```</span>
  ///
  ///
  pub fn version(&self) -> ::pair::PairCIntCInt {
    {
      let mut object: ::pair::PairCIntCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSurfaceFormat_version_to_output(self as *const ::surface_format::SurfaceFormat, &mut object);
      }
      object
    }
  }
}

impl Drop for ::surface_format::SurfaceFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QSurfaceFormat::~QSurfaceFormat()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QSurfaceFormat_destructor(self as *mut ::surface_format::SurfaceFormat) }
  }
}

/// C++ type: <span style='color: green;'>```QSurfaceFormat::SwapBehavior```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum SwapBehavior {
  /// C++ enum variant: <span style='color: green;'>```DefaultSwapBehavior = 0```</span>
  DefaultSwapBehavior = 0,
  /// C++ enum variant: <span style='color: green;'>```SingleBuffer = 1```</span>
  SingleBuffer = 1,
  /// C++ enum variant: <span style='color: green;'>```DoubleBuffer = 2```</span>
  DoubleBuffer = 2,
  /// C++ enum variant: <span style='color: green;'>```TripleBuffer = 3```</span>
  TripleBuffer = 3,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SurfaceFormat::new](../struct.SurfaceFormat.html#method.new) method.
  pub trait SurfaceFormatNewArgs {
    fn exec(self) -> ::surface_format::SurfaceFormat;
  }
  impl SurfaceFormatNewArgs for () {
    fn exec(self) -> ::surface_format::SurfaceFormat {

      {
        let mut object: ::surface_format::SurfaceFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QSurfaceFormat_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl SurfaceFormatNewArgs for ::qt_core::flags::Flags<::surface_format::FormatOption> {
    fn exec(self) -> ::surface_format::SurfaceFormat {
      let options = self;
      {
        let mut object: ::surface_format::SurfaceFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QSurfaceFormat_constructor_options(options.to_int() as ::libc::c_uint, &mut object);
        }
        object
      }
    }
  }
  impl<'a> SurfaceFormatNewArgs for &'a ::surface_format::SurfaceFormat {
    fn exec(self) -> ::surface_format::SurfaceFormat {
      let other = self;
      {
        let mut object: ::surface_format::SurfaceFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QSurfaceFormat_constructor_other(other as *const ::surface_format::SurfaceFormat,
                                                           &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SurfaceFormat::set_option](../struct.SurfaceFormat.html#method.set_option) method.
  pub trait SurfaceFormatSetOptionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::surface_format::SurfaceFormat) -> ();
  }
  impl<'largs> SurfaceFormatSetOptionArgs<'largs> for ::surface_format::FormatOption {
    fn exec(self, original_self: &'largs mut ::surface_format::SurfaceFormat) -> () {
      let option = self;
      unsafe {
        ::ffi::qt_gui_c_QSurfaceFormat_setOption_option(original_self as *mut ::surface_format::SurfaceFormat,
                                                        option)
      }
    }
  }
  impl<'largs> SurfaceFormatSetOptionArgs<'largs> for (::surface_format::FormatOption, bool) {
    fn exec(self, original_self: &'largs mut ::surface_format::SurfaceFormat) -> () {
      let option = self.0;
      let on = self.1;
      unsafe {
        ::ffi::qt_gui_c_QSurfaceFormat_setOption_option_on(original_self as *mut ::surface_format::SurfaceFormat,
                                                           option,
                                                           on)
      }
    }
  }
}
