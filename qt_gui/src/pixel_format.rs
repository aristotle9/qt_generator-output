/// C++ type: <span style='color: green;'>```QPixelFormat::AlphaPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AlphaPosition {
  /// C++ enum variant: <span style='color: green;'>```AtBeginning = 0```</span>
  Beginning = 0,
  /// C++ enum variant: <span style='color: green;'>```AtEnd = 1```</span>
  End = 1,
}

/// C++ type: <span style='color: green;'>```QPixelFormat::AlphaPremultiplied```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AlphaPremultiplied {
  /// C++ enum variant: <span style='color: green;'>```NotPremultiplied = 0```</span>
  NotPremultiplied = 0,
  /// C++ enum variant: <span style='color: green;'>```Premultiplied = 1```</span>
  Premultiplied = 1,
}

/// C++ type: <span style='color: green;'>```QPixelFormat::AlphaUsage```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum AlphaUsage {
  /// C++ enum variant: <span style='color: green;'>```UsesAlpha = 0```</span>
  Uses = 0,
  /// C++ enum variant: <span style='color: green;'>```IgnoresAlpha = 1```</span>
  Ignores = 1,
}

/// C++ type: <span style='color: green;'>```QPixelFormat::ByteOrder```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ByteOrder {
  /// C++ enum variant: <span style='color: green;'>```LittleEndian = 0```</span>
  Little = 0,
  /// C++ enum variant: <span style='color: green;'>```BigEndian = 1```</span>
  Big = 1,
  /// C++ enum variant: <span style='color: green;'>```CurrentSystemEndian = 2```</span>
  CurrentSystem = 2,
}

/// C++ type: <span style='color: green;'>```QPixelFormat::ColorModel```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ColorModel {
  /// C++ enum variant: <span style='color: green;'>```RGB = 0```</span>
  RGB = 0,
  /// C++ enum variant: <span style='color: green;'>```BGR = 1```</span>
  BGR = 1,
  /// C++ enum variant: <span style='color: green;'>```Indexed = 2```</span>
  Indexed = 2,
  /// C++ enum variant: <span style='color: green;'>```Grayscale = 3```</span>
  Grayscale = 3,
  /// C++ enum variant: <span style='color: green;'>```CMYK = 4```</span>
  CMYK = 4,
  /// C++ enum variant: <span style='color: green;'>```HSL = 5```</span>
  HSL = 5,
  /// C++ enum variant: <span style='color: green;'>```HSV = 6```</span>
  HSV = 6,
  /// C++ enum variant: <span style='color: green;'>```YUV = 7```</span>
  YUV = 7,
  /// C++ enum variant: <span style='color: green;'>```Alpha = 8```</span>
  Alpha = 8,
}

/// C++ type: <span style='color: green;'>```QPixelFormat```</span>
#[repr(C)]
pub struct PixelFormat([u8; ::type_sizes::QT_GUI_PIXEL_FORMAT_PIXEL_FORMAT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PixelFormat {
  unsafe fn new_uninitialized() -> PixelFormat {
    PixelFormat(::std::mem::uninitialized())
  }
}

impl PixelFormat {
  /// C++ method: <span style='color: green;'>```QPixelFormat::AlphaPosition QPixelFormat::alphaPosition() const```</span>
  ///
  ///
  pub fn alpha_position(&self) -> ::pixel_format::AlphaPosition {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_alphaPosition(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::alphaSize() const```</span>
  ///
  ///
  pub fn alpha_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_alphaSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```QPixelFormat::AlphaUsage QPixelFormat::alphaUsage() const```</span>
  ///
  ///
  pub fn alpha_usage(&self) -> ::pixel_format::AlphaUsage {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_alphaUsage(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::bitsPerPixel() const```</span>
  ///
  ///
  pub fn bits_per_pixel(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_bitsPerPixel(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::blackSize() const```</span>
  ///
  ///
  pub fn black_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_blackSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::blueSize() const```</span>
  ///
  ///
  pub fn blue_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_blueSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::brightnessSize() const```</span>
  ///
  ///
  pub fn brightness_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_brightnessSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```QPixelFormat::ByteOrder QPixelFormat::byteOrder() const```</span>
  ///
  ///
  pub fn byte_order(&self) -> ::pixel_format::ByteOrder {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_byteOrder(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::channelCount() const```</span>
  ///
  ///
  pub fn channel_count(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_channelCount(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```QPixelFormat::ColorModel QPixelFormat::colorModel() const```</span>
  ///
  ///
  pub fn color_model(&self) -> ::pixel_format::ColorModel {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_colorModel(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::cyanSize() const```</span>
  ///
  ///
  pub fn cyan_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_cyanSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::greenSize() const```</span>
  ///
  ///
  pub fn green_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_greenSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::hueSize() const```</span>
  ///
  ///
  pub fn hue_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_hueSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::lightnessSize() const```</span>
  ///
  ///
  pub fn lightness_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_lightnessSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::magentaSize() const```</span>
  ///
  ///
  pub fn magenta_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_magentaSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```QPixelFormat::QPixelFormat```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::pixel_format::PixelFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPixelFormat::QPixelFormat()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::pixel_format::ColorModel, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::pixel_format::AlphaUsage, ::pixel_format::AlphaPosition, ::pixel_format::AlphaPremultiplied, ::pixel_format::TypeInterpretation)) -> ::pixel_format::PixelFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPixelFormat::QPixelFormat(QPixelFormat::ColorModel colorModel, unsigned char firstSize, unsigned char secondSize, unsigned char thirdSize, unsigned char fourthSize, unsigned char fifthSize, unsigned char alphaSize, QPixelFormat::AlphaUsage alphaUsage, QPixelFormat::AlphaPosition alphaPosition, QPixelFormat::AlphaPremultiplied premultiplied, QPixelFormat::TypeInterpretation typeInterpretation)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::pixel_format::ColorModel, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::pixel_format::AlphaUsage, ::pixel_format::AlphaPosition, ::pixel_format::AlphaPremultiplied, ::pixel_format::TypeInterpretation, ::pixel_format::ByteOrder)) -> ::pixel_format::PixelFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPixelFormat::QPixelFormat(QPixelFormat::ColorModel colorModel, unsigned char firstSize, unsigned char secondSize, unsigned char thirdSize, unsigned char fourthSize, unsigned char fifthSize, unsigned char alphaSize, QPixelFormat::AlphaUsage alphaUsage, QPixelFormat::AlphaPosition alphaPosition, QPixelFormat::AlphaPremultiplied premultiplied, QPixelFormat::TypeInterpretation typeInterpretation, QPixelFormat::ByteOrder byteOrder = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::pixel_format::ColorModel, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::pixel_format::AlphaUsage, ::pixel_format::AlphaPosition, ::pixel_format::AlphaPremultiplied, ::pixel_format::TypeInterpretation, ::pixel_format::ByteOrder, ::libc::c_uchar)) -> ::pixel_format::PixelFormat```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPixelFormat::QPixelFormat(QPixelFormat::ColorModel colorModel, unsigned char firstSize, unsigned char secondSize, unsigned char thirdSize, unsigned char fourthSize, unsigned char fifthSize, unsigned char alphaSize, QPixelFormat::AlphaUsage alphaUsage, QPixelFormat::AlphaPosition alphaPosition, QPixelFormat::AlphaPremultiplied premultiplied, QPixelFormat::TypeInterpretation typeInterpretation, QPixelFormat::ByteOrder byteOrder = ?, unsigned char subEnum = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::pixel_format::PixelFormat
    where Args: overloading::PixelFormatNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPixelFormat::AlphaPremultiplied QPixelFormat::premultiplied() const```</span>
  ///
  ///
  pub fn premultiplied(&self) -> ::pixel_format::AlphaPremultiplied {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_premultiplied(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::redSize() const```</span>
  ///
  ///
  pub fn red_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_redSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::saturationSize() const```</span>
  ///
  ///
  pub fn saturation_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_saturationSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::subEnum() const```</span>
  ///
  ///
  pub fn sub_enum(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_subEnum(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```QPixelFormat::TypeInterpretation QPixelFormat::typeInterpretation() const```</span>
  ///
  ///
  pub fn type_interpretation(&self) -> ::pixel_format::TypeInterpretation {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_typeInterpretation(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char QPixelFormat::yellowSize() const```</span>
  ///
  ///
  pub fn yellow_size(&self) -> ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_yellowSize(self as *const ::pixel_format::PixelFormat) }
  }

  /// C++ method: <span style='color: green;'>```QPixelFormat::YUVLayout QPixelFormat::yuvLayout() const```</span>
  ///
  ///
  pub fn yuv_layout(&self) -> ::pixel_format::YUVLayout {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_yuvLayout(self as *const ::pixel_format::PixelFormat) }
  }
}

impl Drop for ::pixel_format::PixelFormat {
  /// C++ method: <span style='color: green;'>```[destructor] void QPixelFormat::~QPixelFormat()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPixelFormat_destructor(self as *mut ::pixel_format::PixelFormat) }
  }
}

/// C++ type: <span style='color: green;'>```QPixelFormat::TypeInterpretation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum TypeInterpretation {
  /// C++ enum variant: <span style='color: green;'>```UnsignedInteger = 0```</span>
  UnsignedInteger = 0,
  /// C++ enum variant: <span style='color: green;'>```UnsignedShort = 1```</span>
  UnsignedShort = 1,
  /// C++ enum variant: <span style='color: green;'>```UnsignedByte = 2```</span>
  UnsignedByte = 2,
  /// C++ enum variant: <span style='color: green;'>```FloatingPoint = 3```</span>
  FloatingPoint = 3,
}

/// C++ type: <span style='color: green;'>```QPixelFormat::YUVLayout```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum YUVLayout {
  /// C++ enum variant: <span style='color: green;'>```YUV444 = 0```</span>
  YUV444 = 0,
  /// C++ enum variant: <span style='color: green;'>```YUV422 = 1```</span>
  YUV422 = 1,
  /// C++ enum variant: <span style='color: green;'>```YUV411 = 2```</span>
  YUV411 = 2,
  /// C++ enum variant: <span style='color: green;'>```YUV420P = 3```</span>
  YUV420P = 3,
  /// C++ enum variant: <span style='color: green;'>```YUV420SP = 4```</span>
  YUV420SP = 4,
  /// C++ enum variant: <span style='color: green;'>```YV12 = 5```</span>
  YV12 = 5,
  /// C++ enum variant: <span style='color: green;'>```UYVY = 6```</span>
  UYVY = 6,
  /// C++ enum variant: <span style='color: green;'>```YUYV = 7```</span>
  YUYV = 7,
  /// C++ enum variant: <span style='color: green;'>```NV12 = 8```</span>
  NV12 = 8,
  /// C++ enum variant: <span style='color: green;'>```NV21 = 9```</span>
  NV21 = 9,
  /// C++ enum variant: <span style='color: green;'>```IMC1 = 10```</span>
  IMC1 = 10,
  /// C++ enum variant: <span style='color: green;'>```IMC2 = 11```</span>
  IMC2 = 11,
  /// C++ enum variant: <span style='color: green;'>```IMC3 = 12```</span>
  IMC3 = 12,
  /// C++ enum variant: <span style='color: green;'>```IMC4 = 13```</span>
  IMC4 = 13,
  /// C++ enum variant: <span style='color: green;'>```Y8 = 14```</span>
  Y8 = 14,
  /// C++ enum variant: <span style='color: green;'>```Y16 = 15```</span>
  Y16 = 15,
}

/// C++ method: <span style='color: green;'>```qPixelFormatAlpha```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn pixel_format_alpha(::libc::c_uchar) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatAlpha(unsigned char channelSize)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn pixel_format_alpha((::libc::c_uchar, &::pixel_format::TypeInterpretation)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatAlpha(unsigned char channelSize, QPixelFormat::TypeInterpretation typeInt = ?)```</span>
///
///
pub fn pixel_format_alpha<Args>(args: Args) -> ::pixel_format::PixelFormat
  where Args: overloading::PixelFormatAlphaArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qPixelFormatCmyk```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn pixel_format_cmyk(::libc::c_uchar) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatCmyk(unsigned char channelSize)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn pixel_format_cmyk((::libc::c_uchar, ::libc::c_uchar)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatCmyk(unsigned char channelSize, unsigned char alfa = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn pixel_format_cmyk((::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatCmyk(unsigned char channelSize, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn pixel_format_cmyk((::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatCmyk(unsigned char channelSize, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?, QPixelFormat::AlphaPosition position = ?)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn pixel_format_cmyk((::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition, &::pixel_format::TypeInterpretation)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatCmyk(unsigned char channelSize, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?, QPixelFormat::AlphaPosition position = ?, QPixelFormat::TypeInterpretation typeInt = ?)```</span>
///
///
pub fn pixel_format_cmyk<Args>(args: Args) -> ::pixel_format::PixelFormat
  where Args: overloading::PixelFormatCmykArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qPixelFormatGrayscale```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn pixel_format_grayscale(::libc::c_uchar) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatGrayscale(unsigned char channelSize)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn pixel_format_grayscale((::libc::c_uchar, &::pixel_format::TypeInterpretation)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatGrayscale(unsigned char channelSize, QPixelFormat::TypeInterpretation typeInt = ?)```</span>
///
///
pub fn pixel_format_grayscale<Args>(args: Args) -> ::pixel_format::PixelFormat
  where Args: overloading::PixelFormatGrayscaleArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qPixelFormatHsl```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn pixel_format_hsl(::libc::c_uchar) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatHsl(unsigned char channelSize)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn pixel_format_hsl((::libc::c_uchar, ::libc::c_uchar)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatHsl(unsigned char channelSize, unsigned char alfa = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn pixel_format_hsl((::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatHsl(unsigned char channelSize, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn pixel_format_hsl((::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatHsl(unsigned char channelSize, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?, QPixelFormat::AlphaPosition position = ?)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn pixel_format_hsl((::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition, &::pixel_format::TypeInterpretation)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatHsl(unsigned char channelSize, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?, QPixelFormat::AlphaPosition position = ?, QPixelFormat::TypeInterpretation typeInt = ?)```</span>
///
///
pub fn pixel_format_hsl<Args>(args: Args) -> ::pixel_format::PixelFormat
  where Args: overloading::PixelFormatHslArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qPixelFormatHsv```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn pixel_format_hsv(::libc::c_uchar) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatHsv(unsigned char channelSize)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn pixel_format_hsv((::libc::c_uchar, ::libc::c_uchar)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatHsv(unsigned char channelSize, unsigned char alfa = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn pixel_format_hsv((::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatHsv(unsigned char channelSize, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn pixel_format_hsv((::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatHsv(unsigned char channelSize, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?, QPixelFormat::AlphaPosition position = ?)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn pixel_format_hsv((::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition, &::pixel_format::TypeInterpretation)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatHsv(unsigned char channelSize, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?, QPixelFormat::AlphaPosition position = ?, QPixelFormat::TypeInterpretation typeInt = ?)```</span>
///
///
pub fn pixel_format_hsv<Args>(args: Args) -> ::pixel_format::PixelFormat
  where Args: overloading::PixelFormatHsvArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qPixelFormatRgba```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn pixel_format_rgba((::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatRgba(unsigned char red, unsigned char green, unsigned char blue, unsigned char alfa, QPixelFormat::AlphaUsage usage, QPixelFormat::AlphaPosition position)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn pixel_format_rgba((::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition, &::pixel_format::AlphaPremultiplied)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatRgba(unsigned char red, unsigned char green, unsigned char blue, unsigned char alfa, QPixelFormat::AlphaUsage usage, QPixelFormat::AlphaPosition position, QPixelFormat::AlphaPremultiplied pmul = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn pixel_format_rgba((::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition, &::pixel_format::AlphaPremultiplied, &::pixel_format::TypeInterpretation)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatRgba(unsigned char red, unsigned char green, unsigned char blue, unsigned char alfa, QPixelFormat::AlphaUsage usage, QPixelFormat::AlphaPosition position, QPixelFormat::AlphaPremultiplied pmul = ?, QPixelFormat::TypeInterpretation typeInt = ?)```</span>
///
///
pub fn pixel_format_rgba<Args>(args: Args) -> ::pixel_format::PixelFormat
  where Args: overloading::PixelFormatRgbaArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qPixelFormatYuv```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn pixel_format_yuv(&::pixel_format::YUVLayout) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatYuv(QPixelFormat::YUVLayout layout)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn pixel_format_yuv((&::pixel_format::YUVLayout, ::libc::c_uchar)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatYuv(QPixelFormat::YUVLayout layout, unsigned char alfa = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn pixel_format_yuv((&::pixel_format::YUVLayout, ::libc::c_uchar, &::pixel_format::AlphaUsage)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatYuv(QPixelFormat::YUVLayout layout, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn pixel_format_yuv((&::pixel_format::YUVLayout, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatYuv(QPixelFormat::YUVLayout layout, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?, QPixelFormat::AlphaPosition position = ?)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn pixel_format_yuv((&::pixel_format::YUVLayout, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition, &::pixel_format::AlphaPremultiplied)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatYuv(QPixelFormat::YUVLayout layout, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?, QPixelFormat::AlphaPosition position = ?, QPixelFormat::AlphaPremultiplied p_mul = ?)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn pixel_format_yuv((&::pixel_format::YUVLayout, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition, &::pixel_format::AlphaPremultiplied, &::pixel_format::TypeInterpretation)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatYuv(QPixelFormat::YUVLayout layout, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?, QPixelFormat::AlphaPosition position = ?, QPixelFormat::AlphaPremultiplied p_mul = ?, QPixelFormat::TypeInterpretation typeInt = ?)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn pixel_format_yuv((&::pixel_format::YUVLayout, ::libc::c_uchar, &::pixel_format::AlphaUsage, &::pixel_format::AlphaPosition, &::pixel_format::AlphaPremultiplied, &::pixel_format::TypeInterpretation, &::pixel_format::ByteOrder)) -> ::pixel_format::PixelFormat```<br>
/// C++ method: <span style='color: green;'>```QPixelFormat qPixelFormatYuv(QPixelFormat::YUVLayout layout, unsigned char alfa = ?, QPixelFormat::AlphaUsage usage = ?, QPixelFormat::AlphaPosition position = ?, QPixelFormat::AlphaPremultiplied p_mul = ?, QPixelFormat::TypeInterpretation typeInt = ?, QPixelFormat::ByteOrder b_order = ?)```</span>
///
///
pub fn pixel_format_yuv<Args>(args: Args) -> ::pixel_format::PixelFormat
  where Args: overloading::PixelFormatYuvArgs
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PixelFormat::new](../struct.PixelFormat.html#method.new) method.
  pub trait PixelFormatNewArgs {
    fn exec(self) -> ::pixel_format::PixelFormat;
  }
  impl PixelFormatNewArgs
    for (::pixel_format::ColorModel,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::pixel_format::AlphaUsage,
                                   ::pixel_format::AlphaPosition,
                                   ::pixel_format::AlphaPremultiplied,
                                   ::pixel_format::TypeInterpretation) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let color_model = self.0;
      let first_size = self.1;
      let second_size = self.2;
      let third_size = self.3;
      let fourth_size = self.4;
      let fifth_size = self.5;
      let alpha_size = self.6;
      let alpha_usage = self.7;
      let alpha_position = self.8;
      let premultiplied = self.9;
      let type_interpretation = self.10;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_constructor_colorModel_firstSize_secondSize_thirdSize_fourthSize_fifthSize_alphaSize_alphaUsage_alphaPosition_premultiplied_typeInterpretation(color_model, first_size, second_size, third_size, fourth_size, fifth_size, alpha_size, alpha_usage, alpha_position, premultiplied, type_interpretation, &mut object);
        }
        object
      }
    }
  }
  impl PixelFormatNewArgs
    for (::pixel_format::ColorModel,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::pixel_format::AlphaUsage,
                                   ::pixel_format::AlphaPosition,
                                   ::pixel_format::AlphaPremultiplied,
                                   ::pixel_format::TypeInterpretation,
                                   ::pixel_format::ByteOrder) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let color_model = self.0;
      let first_size = self.1;
      let second_size = self.2;
      let third_size = self.3;
      let fourth_size = self.4;
      let fifth_size = self.5;
      let alpha_size = self.6;
      let alpha_usage = self.7;
      let alpha_position = self.8;
      let premultiplied = self.9;
      let type_interpretation = self.10;
      let byte_order = self.11;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_constructor_colorModel_firstSize_secondSize_thirdSize_fourthSize_fifthSize_alphaSize_alphaUsage_alphaPosition_premultiplied_typeInterpretation_byteOrder(color_model, first_size, second_size, third_size, fourth_size, fifth_size, alpha_size, alpha_usage, alpha_position, premultiplied, type_interpretation, byte_order, &mut object);
        }
        object
      }
    }
  }
  impl PixelFormatNewArgs
    for (::pixel_format::ColorModel,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::libc::c_uchar,
                                   ::pixel_format::AlphaUsage,
                                   ::pixel_format::AlphaPosition,
                                   ::pixel_format::AlphaPremultiplied,
                                   ::pixel_format::TypeInterpretation,
                                   ::pixel_format::ByteOrder,
                                   ::libc::c_uchar) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let color_model = self.0;
      let first_size = self.1;
      let second_size = self.2;
      let third_size = self.3;
      let fourth_size = self.4;
      let fifth_size = self.5;
      let alpha_size = self.6;
      let alpha_usage = self.7;
      let alpha_position = self.8;
      let premultiplied = self.9;
      let type_interpretation = self.10;
      let byte_order = self.11;
      let sub_enum = self.12;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_constructor_colorModel_firstSize_secondSize_thirdSize_fourthSize_fifthSize_alphaSize_alphaUsage_alphaPosition_premultiplied_typeInterpretation_byteOrder_subEnum(color_model, first_size, second_size, third_size, fourth_size, fifth_size, alpha_size, alpha_usage, alpha_position, premultiplied, type_interpretation, byte_order, sub_enum, &mut object);
        }
        object
      }
    }
  }
  impl PixelFormatNewArgs for () {
    fn exec(self) -> ::pixel_format::PixelFormat {

      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [pixel_format_alpha](../fn.pixel_format_alpha.html) method.
  pub trait PixelFormatAlphaArgs {
    fn exec(self) -> ::pixel_format::PixelFormat;
  }
  impl PixelFormatAlphaArgs for ::libc::c_uchar {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatAlpha_to_output_channelSize(channel_size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatAlphaArgs for (::libc::c_uchar, &'a ::pixel_format::TypeInterpretation) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let type_int = self.1;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatAlpha_to_output_channelSize_typeInt(channel_size, type_int as *const ::pixel_format::TypeInterpretation, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [pixel_format_cmyk](../fn.pixel_format_cmyk.html) method.
  pub trait PixelFormatCmykArgs {
    fn exec(self) -> ::pixel_format::PixelFormat;
  }
  impl PixelFormatCmykArgs for ::libc::c_uchar {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatCmyk_to_output_channelSize(channel_size, &mut object);
        }
        object
      }
    }
  }
  impl PixelFormatCmykArgs for (::libc::c_uchar, ::libc::c_uchar) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatCmyk_to_output_channelSize_alfa(channel_size, alfa, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatCmykArgs for (::libc::c_uchar, ::libc::c_uchar, &'a ::pixel_format::AlphaUsage) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      let usage = self.2;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatCmyk_to_output_channelSize_alfa_usage(channel_size, alfa, usage as *const ::pixel_format::AlphaUsage, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatCmykArgs
    for (::libc::c_uchar, ::libc::c_uchar, &'a ::pixel_format::AlphaUsage, &'a ::pixel_format::AlphaPosition) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      let usage = self.2;
      let position = self.3;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatCmyk_to_output_channelSize_alfa_usage_position(channel_size, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatCmykArgs
    for (::libc::c_uchar,
                                        ::libc::c_uchar,
                                        &'a ::pixel_format::AlphaUsage,
                                        &'a ::pixel_format::AlphaPosition,
                                        &'a ::pixel_format::TypeInterpretation) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      let usage = self.2;
      let position = self.3;
      let type_int = self.4;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatCmyk_to_output_channelSize_alfa_usage_position_typeInt(channel_size, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, type_int as *const ::pixel_format::TypeInterpretation, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [pixel_format_grayscale](../fn.pixel_format_grayscale.html) method.
  pub trait PixelFormatGrayscaleArgs {
    fn exec(self) -> ::pixel_format::PixelFormat;
  }
  impl PixelFormatGrayscaleArgs for ::libc::c_uchar {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatGrayscale_to_output_channelSize(channel_size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatGrayscaleArgs for (::libc::c_uchar, &'a ::pixel_format::TypeInterpretation) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let type_int = self.1;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatGrayscale_to_output_channelSize_typeInt(channel_size, type_int as *const ::pixel_format::TypeInterpretation, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [pixel_format_hsl](../fn.pixel_format_hsl.html) method.
  pub trait PixelFormatHslArgs {
    fn exec(self) -> ::pixel_format::PixelFormat;
  }
  impl PixelFormatHslArgs for ::libc::c_uchar {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatHsl_to_output_channelSize(channel_size, &mut object);
        }
        object
      }
    }
  }
  impl PixelFormatHslArgs for (::libc::c_uchar, ::libc::c_uchar) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatHsl_to_output_channelSize_alfa(channel_size, alfa, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatHslArgs for (::libc::c_uchar, ::libc::c_uchar, &'a ::pixel_format::AlphaUsage) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      let usage = self.2;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatHsl_to_output_channelSize_alfa_usage(channel_size, alfa, usage as *const ::pixel_format::AlphaUsage, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatHslArgs
    for (::libc::c_uchar, ::libc::c_uchar, &'a ::pixel_format::AlphaUsage, &'a ::pixel_format::AlphaPosition) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      let usage = self.2;
      let position = self.3;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatHsl_to_output_channelSize_alfa_usage_position(channel_size, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatHslArgs
    for (::libc::c_uchar,
                                       ::libc::c_uchar,
                                       &'a ::pixel_format::AlphaUsage,
                                       &'a ::pixel_format::AlphaPosition,
                                       &'a ::pixel_format::TypeInterpretation) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      let usage = self.2;
      let position = self.3;
      let type_int = self.4;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatHsl_to_output_channelSize_alfa_usage_position_typeInt(channel_size, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, type_int as *const ::pixel_format::TypeInterpretation, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [pixel_format_hsv](../fn.pixel_format_hsv.html) method.
  pub trait PixelFormatHsvArgs {
    fn exec(self) -> ::pixel_format::PixelFormat;
  }
  impl PixelFormatHsvArgs for ::libc::c_uchar {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatHsv_to_output_channelSize(channel_size, &mut object);
        }
        object
      }
    }
  }
  impl PixelFormatHsvArgs for (::libc::c_uchar, ::libc::c_uchar) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatHsv_to_output_channelSize_alfa(channel_size, alfa, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatHsvArgs for (::libc::c_uchar, ::libc::c_uchar, &'a ::pixel_format::AlphaUsage) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      let usage = self.2;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatHsv_to_output_channelSize_alfa_usage(channel_size, alfa, usage as *const ::pixel_format::AlphaUsage, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatHsvArgs
    for (::libc::c_uchar, ::libc::c_uchar, &'a ::pixel_format::AlphaUsage, &'a ::pixel_format::AlphaPosition) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      let usage = self.2;
      let position = self.3;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatHsv_to_output_channelSize_alfa_usage_position(channel_size, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatHsvArgs
    for (::libc::c_uchar,
                                       ::libc::c_uchar,
                                       &'a ::pixel_format::AlphaUsage,
                                       &'a ::pixel_format::AlphaPosition,
                                       &'a ::pixel_format::TypeInterpretation) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let channel_size = self.0;
      let alfa = self.1;
      let usage = self.2;
      let position = self.3;
      let type_int = self.4;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatHsv_to_output_channelSize_alfa_usage_position_typeInt(channel_size, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, type_int as *const ::pixel_format::TypeInterpretation, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [pixel_format_rgba](../fn.pixel_format_rgba.html) method.
  pub trait PixelFormatRgbaArgs {
    fn exec(self) -> ::pixel_format::PixelFormat;
  }
  impl<'a> PixelFormatRgbaArgs
    for (::libc::c_uchar,
                                        ::libc::c_uchar,
                                        ::libc::c_uchar,
                                        ::libc::c_uchar,
                                        &'a ::pixel_format::AlphaUsage,
                                        &'a ::pixel_format::AlphaPosition) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let red = self.0;
      let green = self.1;
      let blue = self.2;
      let alfa = self.3;
      let usage = self.4;
      let position = self.5;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatRgba_to_output_red_green_blue_alfa_usage_position(red, green, blue, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatRgbaArgs
    for (::libc::c_uchar,
                                        ::libc::c_uchar,
                                        ::libc::c_uchar,
                                        ::libc::c_uchar,
                                        &'a ::pixel_format::AlphaUsage,
                                        &'a ::pixel_format::AlphaPosition,
                                        &'a ::pixel_format::AlphaPremultiplied) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let red = self.0;
      let green = self.1;
      let blue = self.2;
      let alfa = self.3;
      let usage = self.4;
      let position = self.5;
      let pmul = self.6;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatRgba_to_output_red_green_blue_alfa_usage_position_pmul(red, green, blue, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, pmul as *const ::pixel_format::AlphaPremultiplied, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatRgbaArgs
    for (::libc::c_uchar,
                                        ::libc::c_uchar,
                                        ::libc::c_uchar,
                                        ::libc::c_uchar,
                                        &'a ::pixel_format::AlphaUsage,
                                        &'a ::pixel_format::AlphaPosition,
                                        &'a ::pixel_format::AlphaPremultiplied,
                                        &'a ::pixel_format::TypeInterpretation) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let red = self.0;
      let green = self.1;
      let blue = self.2;
      let alfa = self.3;
      let usage = self.4;
      let position = self.5;
      let pmul = self.6;
      let type_int = self.7;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatRgba_to_output_red_green_blue_alfa_usage_position_pmul_typeInt(red, green, blue, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, pmul as *const ::pixel_format::AlphaPremultiplied, type_int as *const ::pixel_format::TypeInterpretation, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [pixel_format_yuv](../fn.pixel_format_yuv.html) method.
  pub trait PixelFormatYuvArgs {
    fn exec(self) -> ::pixel_format::PixelFormat;
  }
  impl<'a> PixelFormatYuvArgs for &'a ::pixel_format::YUVLayout {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let layout = self;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout(layout as *const ::pixel_format::YUVLayout,
                                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatYuvArgs for (&'a ::pixel_format::YUVLayout, ::libc::c_uchar) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let layout = self.0;
      let alfa = self.1;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa(layout as *const ::pixel_format::YUVLayout, alfa, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatYuvArgs for (&'a ::pixel_format::YUVLayout, ::libc::c_uchar, &'a ::pixel_format::AlphaUsage) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let layout = self.0;
      let alfa = self.1;
      let usage = self.2;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa_usage(layout as *const ::pixel_format::YUVLayout, alfa, usage as *const ::pixel_format::AlphaUsage, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatYuvArgs
    for (&'a ::pixel_format::YUVLayout,
                                       ::libc::c_uchar,
                                       &'a ::pixel_format::AlphaUsage,
                                       &'a ::pixel_format::AlphaPosition) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let layout = self.0;
      let alfa = self.1;
      let usage = self.2;
      let position = self.3;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa_usage_position(layout as *const ::pixel_format::YUVLayout, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatYuvArgs
    for (&'a ::pixel_format::YUVLayout,
                                       ::libc::c_uchar,
                                       &'a ::pixel_format::AlphaUsage,
                                       &'a ::pixel_format::AlphaPosition,
                                       &'a ::pixel_format::AlphaPremultiplied) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let layout = self.0;
      let alfa = self.1;
      let usage = self.2;
      let position = self.3;
      let p_mul = self.4;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa_usage_position_p_mul(layout as *const ::pixel_format::YUVLayout, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, p_mul as *const ::pixel_format::AlphaPremultiplied, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatYuvArgs
    for (&'a ::pixel_format::YUVLayout,
                                       ::libc::c_uchar,
                                       &'a ::pixel_format::AlphaUsage,
                                       &'a ::pixel_format::AlphaPosition,
                                       &'a ::pixel_format::AlphaPremultiplied,
                                       &'a ::pixel_format::TypeInterpretation) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let layout = self.0;
      let alfa = self.1;
      let usage = self.2;
      let position = self.3;
      let p_mul = self.4;
      let type_int = self.5;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa_usage_position_p_mul_typeInt(layout as *const ::pixel_format::YUVLayout, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, p_mul as *const ::pixel_format::AlphaPremultiplied, type_int as *const ::pixel_format::TypeInterpretation, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PixelFormatYuvArgs
    for (&'a ::pixel_format::YUVLayout,
                                       ::libc::c_uchar,
                                       &'a ::pixel_format::AlphaUsage,
                                       &'a ::pixel_format::AlphaPosition,
                                       &'a ::pixel_format::AlphaPremultiplied,
                                       &'a ::pixel_format::TypeInterpretation,
                                       &'a ::pixel_format::ByteOrder) {
    fn exec(self) -> ::pixel_format::PixelFormat {
      let layout = self.0;
      let alfa = self.1;
      let usage = self.2;
      let position = self.3;
      let p_mul = self.4;
      let type_int = self.5;
      let b_order = self.6;
      {
        let mut object: ::pixel_format::PixelFormat =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPixelFormat_G_qPixelFormatYuv_to_output_layout_alfa_usage_position_p_mul_typeInt_b_order(layout as *const ::pixel_format::YUVLayout, alfa, usage as *const ::pixel_format::AlphaUsage, position as *const ::pixel_format::AlphaPosition, p_mul as *const ::pixel_format::AlphaPremultiplied, type_int as *const ::pixel_format::TypeInterpretation, b_order as *const ::pixel_format::ByteOrder, &mut object);
        }
        object
      }
    }
  }
}

pub mod private {
  /// C++ method: <span style='color: green;'>```QPixelFormat QtPrivate::QPixelFormat_createYUV(QPixelFormat::YUVLayout yuvLayout, unsigned char alphaSize, QPixelFormat::AlphaUsage alphaUsage, QPixelFormat::AlphaPosition alphaPosition, QPixelFormat::AlphaPremultiplied premultiplied, QPixelFormat::TypeInterpretation typeInterpretation, QPixelFormat::ByteOrder byteOrder)```</span>
  ///
  ///
  pub fn pixel_format_create_y_u_v(yuv_layout: &::pixel_format::YUVLayout,
                                   alpha_size: ::libc::c_uchar,
                                   alpha_usage: &::pixel_format::AlphaUsage,
                                   alpha_position: &::pixel_format::AlphaPosition,
                                   premultiplied: &::pixel_format::AlphaPremultiplied,
                                   type_interpretation: &::pixel_format::TypeInterpretation,
                                   byte_order: &::pixel_format::ByteOrder)
                                   -> ::pixel_format::PixelFormat {
    {
      let mut object: ::pixel_format::PixelFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPixelFormat_G_QtPrivate_QPixelFormat_createYUV_to_output(yuv_layout as *const ::pixel_format::YUVLayout, alpha_size, alpha_usage as *const ::pixel_format::AlphaUsage, alpha_position as *const ::pixel_format::AlphaPosition, premultiplied as *const ::pixel_format::AlphaPremultiplied, type_interpretation as *const ::pixel_format::TypeInterpretation, byte_order as *const ::pixel_format::ByteOrder, &mut object);
      }
      object
    }
  }

}
