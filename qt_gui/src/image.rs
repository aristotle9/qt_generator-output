/// C++ type: <span style='color: green;'>```QImage::Format```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Format {
  /// C++ enum variant: <span style='color: green;'>```Format_Invalid = 0```</span>
  FormatInvalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Format_Mono = 1```</span>
  FormatMono = 1,
  /// C++ enum variant: <span style='color: green;'>```Format_MonoLSB = 2```</span>
  FormatMonoLSB = 2,
  /// C++ enum variant: <span style='color: green;'>```Format_Indexed8 = 3```</span>
  FormatIndexed8 = 3,
  /// C++ enum variant: <span style='color: green;'>```Format_RGB32 = 4```</span>
  FormatRGB32 = 4,
  /// C++ enum variant: <span style='color: green;'>```Format_ARGB32 = 5```</span>
  FormatARGB32 = 5,
  /// C++ enum variant: <span style='color: green;'>```Format_ARGB32_Premultiplied = 6```</span>
  FormatARGB32Premultiplied = 6,
  /// C++ enum variant: <span style='color: green;'>```Format_RGB16 = 7```</span>
  FormatRGB16 = 7,
  /// C++ enum variant: <span style='color: green;'>```Format_ARGB8565_Premultiplied = 8```</span>
  FormatARGB8565Premultiplied = 8,
  /// C++ enum variant: <span style='color: green;'>```Format_RGB666 = 9```</span>
  FormatRGB666 = 9,
  /// C++ enum variant: <span style='color: green;'>```Format_ARGB6666_Premultiplied = 10```</span>
  FormatARGB6666Premultiplied = 10,
  /// C++ enum variant: <span style='color: green;'>```Format_RGB555 = 11```</span>
  FormatRGB555 = 11,
  /// C++ enum variant: <span style='color: green;'>```Format_ARGB8555_Premultiplied = 12```</span>
  FormatARGB8555Premultiplied = 12,
  /// C++ enum variant: <span style='color: green;'>```Format_RGB888 = 13```</span>
  FormatRGB888 = 13,
  /// C++ enum variant: <span style='color: green;'>```Format_RGB444 = 14```</span>
  FormatRGB444 = 14,
  /// C++ enum variant: <span style='color: green;'>```Format_ARGB4444_Premultiplied = 15```</span>
  FormatARGB4444Premultiplied = 15,
  /// C++ enum variant: <span style='color: green;'>```Format_RGBX8888 = 16```</span>
  FormatRGBX8888 = 16,
  /// C++ enum variant: <span style='color: green;'>```Format_RGBA8888 = 17```</span>
  FormatRGBA8888 = 17,
  /// C++ enum variant: <span style='color: green;'>```Format_RGBA8888_Premultiplied = 18```</span>
  FormatRGBA8888Premultiplied = 18,
  /// C++ enum variant: <span style='color: green;'>```Format_BGR30 = 19```</span>
  FormatBGR30 = 19,
  /// C++ enum variant: <span style='color: green;'>```Format_A2BGR30_Premultiplied = 20```</span>
  FormatA2BGR30Premultiplied = 20,
  /// C++ enum variant: <span style='color: green;'>```Format_RGB30 = 21```</span>
  FormatRGB30 = 21,
  /// C++ enum variant: <span style='color: green;'>```Format_A2RGB30_Premultiplied = 22```</span>
  FormatA2RGB30Premultiplied = 22,
  /// C++ enum variant: <span style='color: green;'>```Format_Alpha8 = 23```</span>
  FormatAlpha8 = 23,
  /// C++ enum variant: <span style='color: green;'>```Format_Grayscale8 = 24```</span>
  FormatGrayscale8 = 24,
  /// C++ enum variant: <span style='color: green;'>```NImageFormats = 25```</span>
  NImageFormats = 25,
}

/// C++ type: <span style='color: green;'>```QImage```</span>
#[repr(C)]
pub struct Image(u8);

impl Image {
  /// C++ method: <span style='color: green;'>```bool QImage::allGray() const```</span>
  ///
  ///
  pub fn all_gray(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImage_allGray(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```QImage QImage::alphaChannel() const```</span>
  ///
  ///
  pub fn alpha_channel(&self) -> ::cpp_utils::CppBox<::image::Image> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QImage_alphaChannel_as_ptr(self as *const ::image::Image) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QImage::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImage_convert_to_QVariant_to_output(self as *const ::image::Image, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QImage::bitPlaneCount() const```</span>
  ///
  ///
  pub fn bit_plane_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImage_bitPlaneCount(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```const unsigned char* QImage::bits() const```</span>
  ///
  ///
  pub fn bits(&self) -> *const ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QImage_bits_const(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char* QImage::bits()```</span>
  ///
  ///
  pub fn bits_mut(&mut self) -> *mut ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QImage_bits(self as *mut ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```int QImage::byteCount() const```</span>
  ///
  ///
  pub fn byte_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImage_byteCount(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```int QImage::bytesPerLine() const```</span>
  ///
  ///
  pub fn bytes_per_line(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImage_bytesPerLine(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```qint64 QImage::cacheKey() const```</span>
  ///
  ///
  pub fn cache_key(&self) -> i64 {
    unsafe { ::ffi::qt_gui_c_QImage_cacheKey(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QImage::color(int i) const```</span>
  ///
  ///
  pub fn color(&self, i: ::libc::c_int) -> ::libc::c_uint {
    unsafe { ::ffi::qt_gui_c_QImage_color(self as *const ::image::Image, i) }
  }

  /// C++ method: <span style='color: green;'>```int QImage::colorCount() const```</span>
  ///
  ///
  pub fn color_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImage_colorCount(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```QVector<unsigned int> QImage::colorTable() const```</span>
  ///
  ///
  pub fn color_table(&self) -> ::qt_core::vector::VectorCUint {
    {
      let mut object: ::qt_core::vector::VectorCUint =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImage_colorTable_to_output(self as *const ::image::Image, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const unsigned char* QImage::constBits() const```</span>
  ///
  ///
  pub fn const_bits(&self) -> *const ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QImage_constBits(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```const unsigned char* QImage::constScanLine(int arg1) const```</span>
  ///
  ///
  pub fn const_scan_line(&self, arg1: ::libc::c_int) -> *const ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QImage_constScanLine(self as *const ::image::Image, arg1) }
  }

  /// C++ method: <span style='color: green;'>```QImage::copy```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn copy(&self, ()) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::copy() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn copy(&self, &::qt_core::rect::Rect) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::copy(const QRect& rect = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn copy(&self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::copy(int x, int y, int w, int h) const```</span>
  ///
  ///
  pub fn copy<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ImageCopyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::createHeuristicMask```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_heuristic_mask(&self, ()) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::createHeuristicMask() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_heuristic_mask(&self, bool) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::createHeuristicMask(bool clipTight = ?) const```</span>
  ///
  ///
  pub fn create_heuristic_mask<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ImageCreateHeuristicMaskArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::createMaskFromColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_mask_from_color(&self, ::libc::c_uint) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::createMaskFromColor(unsigned int color) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_mask_from_color(&self, (::libc::c_uint, &::qt_core::qt::MaskMode)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::createMaskFromColor(unsigned int color, Qt::MaskMode mode = ?) const```</span>
  ///
  ///
  pub fn create_mask_from_color<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ImageCreateMaskFromColorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QImage::depth() const```</span>
  ///
  ///
  pub fn depth(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImage_depth(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```void QImage::detach()```</span>
  ///
  ///
  pub fn detach(&mut self) {
    unsafe { ::ffi::qt_gui_c_QImage_detach(self as *mut ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QImage::devType() const```</span>
  ///
  ///
  pub fn dev_type(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImage_devType(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```double QImage::devicePixelRatio() const```</span>
  ///
  ///
  pub fn device_pixel_ratio(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QImage_devicePixelRatio(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```int QImage::dotsPerMeterX() const```</span>
  ///
  ///
  pub fn dots_per_meter_x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImage_dotsPerMeterX(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```int QImage::dotsPerMeterY() const```</span>
  ///
  ///
  pub fn dots_per_meter_y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImage_dotsPerMeterY(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```QImage::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &::qt_core::qt::GlobalColor) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QImage::fill(Qt::GlobalColor color)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, &::color::Color) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QImage::fill(const QColor& color)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn fill(&mut self, ::libc::c_uint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QImage::fill(unsigned int pixel)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ImageFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::Format QImage::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::image::Format {
    unsafe { ::ffi::qt_gui_c_QImage_format(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```static QImage QImage::fromData(const QByteArray& data)```</span>
  ///
  ///
  pub fn from_data(data: &::qt_core::byte_array::ByteArray) -> ::cpp_utils::CppBox<::image::Image> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QImage_fromData_as_ptr_data(data as *const ::qt_core::byte_array::ByteArray) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QImage::fromData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_data_unsafe((&::qt_core::byte_array::ByteArray, *const ::libc::c_char)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```static QImage QImage::fromData(const QByteArray& data, const char* format = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_data_unsafe((*const ::libc::c_uchar, ::libc::c_int)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```static QImage QImage::fromData(const unsigned char* data, int size)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn from_data_unsafe((*const ::libc::c_uchar, ::libc::c_int, *const ::libc::c_char)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```static QImage QImage::fromData(const unsigned char* data, int size, const char* format = ?)```</span>
  ///
  ///
  pub unsafe fn from_data_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ImageFromDataUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```bool QImage::hasAlphaChannel() const```</span>
  ///
  ///
  pub fn has_alpha_channel(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImage_hasAlphaChannel(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```int QImage::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImage_height(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```QImage::invertPixels```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn invert_pixels(&mut self, ()) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QImage::invertPixels()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn invert_pixels(&mut self, ::image::InvertMode) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QImage::invertPixels(QImage::InvertMode arg1 = ?)```</span>
  ///
  ///
  pub fn invert_pixels<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ImageInvertPixelsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QImage::isDetached() const```</span>
  ///
  ///
  pub fn is_detached(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImage_isDetached(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```bool QImage::isGrayscale() const```</span>
  ///
  ///
  pub fn is_grayscale(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImage_isGrayscale(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```bool QImage::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImage_isNull(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```bool QImage::load(const QString& fileName)```</span>
  ///
  ///
  pub fn load(&mut self, file_name: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QImage_load_fileName(self as *mut ::image::Image,
                                           file_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QImage::loadFromData(const QByteArray& data)```</span>
  ///
  ///
  pub fn load_from_data(&mut self, data: &::qt_core::byte_array::ByteArray) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QImage_loadFromData_data(self as *mut ::image::Image,
                                               data as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QImage::loadFromData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn load_from_data_unsafe(&mut self, (&::qt_core::byte_array::ByteArray, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::loadFromData(const QByteArray& data, const char* aformat = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn load_from_data_unsafe(&mut self, (*const ::libc::c_uchar, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::loadFromData(const unsigned char* buf, int len)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn load_from_data_unsafe(&mut self, (*const ::libc::c_uchar, ::libc::c_int, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::loadFromData(const unsigned char* buf, int len, const char* format = ?)```</span>
  ///
  ///
  pub unsafe fn load_from_data_unsafe<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ImageLoadFromDataUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::load```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn load_unsafe(&mut self, (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::load(QIODevice* device, const char* format)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn load_unsafe(&mut self, (&::qt_core::string::String, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::load(const QString& fileName, const char* format = ?)```</span>
  ///
  ///
  pub unsafe fn load_unsafe<'largs, Args>(&'largs mut self, args: Args) -> bool
    where Args: overloading::ImageLoadUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::mirrored```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mirrored(&self, ()) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::mirrored() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mirrored(&self, bool) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::mirrored(bool horizontally = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn mirrored(&self, (bool, bool)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::mirrored(bool horizontally = ?, bool vertically = ?) const```</span>
  ///
  ///
  pub fn mirrored<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ImageMirroredArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::QImage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::image::Image) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(const QImage& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::size::Size, ::image::Format)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(const QSize& size, QImage::Format format)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::image::Format)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(int width, int height, QImage::Format format)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ImageNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QImage::QImage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *const ::libc::c_char)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(const QString& fileName, const char* format = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::image::Format)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(const unsigned char* data, int width, int height, QImage::Format format)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::image::Format, extern "C" fn(*mut ::libc::c_void))) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(const unsigned char* data, int width, int height, QImage::Format format, void (*FN_PTR)(void*) cleanupFunction = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::image::Format, extern "C" fn(*mut ::libc::c_void), *mut ::libc::c_void)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(const unsigned char* data, int width, int height, QImage::Format format, void (*FN_PTR)(void*) cleanupFunction = ?, void* cleanupInfo = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::image::Format)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(const unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::image::Format, extern "C" fn(*mut ::libc::c_void))) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(const unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format, void (*FN_PTR)(void*) cleanupFunction = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::image::Format, extern "C" fn(*mut ::libc::c_void), *mut ::libc::c_void)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(const unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format, void (*FN_PTR)(void*) cleanupFunction = ?, void* cleanupInfo = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::image::Format)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(unsigned char* data, int width, int height, QImage::Format format)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::image::Format, extern "C" fn(*mut ::libc::c_void))) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(unsigned char* data, int width, int height, QImage::Format format, void (*FN_PTR)(void*) cleanupFunction = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::image::Format, extern "C" fn(*mut ::libc::c_void), *mut ::libc::c_void)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(unsigned char* data, int width, int height, QImage::Format format, void (*FN_PTR)(void*) cleanupFunction = ?, void* cleanupInfo = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::image::Format)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::image::Format, extern "C" fn(*mut ::libc::c_void))) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format, void (*FN_PTR)(void*) cleanupFunction = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 13
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::image::Format, extern "C" fn(*mut ::libc::c_void), *mut ::libc::c_void)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImage::QImage(unsigned char* data, int width, int height, int bytesPerLine, QImage::Format format, void (*FN_PTR)(void*) cleanupFunction = ?, void* cleanupInfo = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ImageNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPoint QImage::offset() const```</span>
  ///
  ///
  pub fn offset(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImage_offset_to_output(self as *const ::image::Image, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QImage& QImage::operator=(const QImage& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, arg1: &'l1 ::image::Image) -> &'l0 mut ::image::Image {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QImage_operator_assign(self as *mut ::image::Image, arg1 as *const ::image::Image) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QImage::operator==(const QImage& arg1) const```</span>
  ///
  ///
  pub fn op_eq(&self, arg1: &::image::Image) -> bool {
    unsafe { ::ffi::qt_gui_c_QImage_operator_eq(self as *const ::image::Image, arg1 as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```bool QImage::operator!=(const QImage& arg1) const```</span>
  ///
  ///
  pub fn op_neq(&self, arg1: &::image::Image) -> bool {
    unsafe { ::ffi::qt_gui_c_QImage_operator_neq(self as *const ::image::Image, arg1 as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```virtual QPaintEngine* QImage::paintEngine() const```</span>
  ///
  ///
  pub fn paint_engine(&self) -> *mut ::paint_engine::PaintEngine {
    unsafe { ::ffi::qt_gui_c_QImage_paintEngine(self as *const ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```QImage::pixel```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pixel(&self, &::qt_core::point::Point) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QImage::pixel(const QPoint& pt) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pixel(&self, (::libc::c_int, ::libc::c_int)) -> ::libc::c_uint```<br>
  /// C++ method: <span style='color: green;'>```unsigned int QImage::pixel(int x, int y) const```</span>
  ///
  ///
  pub fn pixel<'largs, Args>(&'largs self, args: Args) -> ::libc::c_uint
    where Args: overloading::ImagePixelArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::pixelColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pixel_color(&self, &::qt_core::point::Point) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QImage::pixelColor(const QPoint& pt) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pixel_color(&self, (::libc::c_int, ::libc::c_int)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QImage::pixelColor(int x, int y) const```</span>
  ///
  ///
  pub fn pixel_color<'largs, Args>(&'largs self, args: Args) -> ::color::Color
    where Args: overloading::ImagePixelColorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPixelFormat QImage::pixelFormat() const```</span>
  ///
  ///
  pub fn pixel_format(&self) -> ::pixel_format::PixelFormat {
    {
      let mut object: ::pixel_format::PixelFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImage_pixelFormat_to_output(self as *const ::image::Image, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QImage::pixelIndex```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn pixel_index(&self, &::qt_core::point::Point) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QImage::pixelIndex(const QPoint& pt) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn pixel_index(&self, (::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QImage::pixelIndex(int x, int y) const```</span>
  ///
  ///
  pub fn pixel_index<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ImagePixelIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRect QImage::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImage_rect_to_output(self as *const ::image::Image, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QImage::reinterpretAsFormat(QImage::Format f)```</span>
  ///
  ///
  pub fn reinterpret_as_format(&mut self, f: ::image::Format) -> bool {
    unsafe { ::ffi::qt_gui_c_QImage_reinterpretAsFormat(self as *mut ::image::Image, f) }
  }

  /// C++ method: <span style='color: green;'>```QImage QImage::rgbSwapped() const```</span>
  ///
  ///
  pub fn rgb_swapped(&self) -> ::cpp_utils::CppBox<::image::Image> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QImage_rgbSwapped_as_ptr(self as *const ::image::Image) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```bool QImage::save(const QString& fileName) const```</span>
  ///
  ///
  pub fn save(&self, file_name: &::qt_core::string::String) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QImage_save_fileName(self as *const ::image::Image,
                                           file_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QImage::save```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn save_unsafe(&self, *mut ::qt_core::io_device::IODevice) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::save(QIODevice* device) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn save_unsafe(&self, (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::save(QIODevice* device, const char* format = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn save_unsafe(&self, (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::save(QIODevice* device, const char* format = ?, int quality = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn save_unsafe(&self, (&::qt_core::string::String, *const ::libc::c_char)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::save(const QString& fileName, const char* format = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn save_unsafe(&self, (&::qt_core::string::String, *const ::libc::c_char, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::save(const QString& fileName, const char* format = ?, int quality = ?) const```</span>
  ///
  ///
  pub unsafe fn save_unsafe<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::ImageSaveUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::scaled```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scaled(&self, &::qt_core::size::Size) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::scaled(const QSize& s) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scaled(&self, (&::qt_core::size::Size, &::qt_core::qt::AspectRatioMode)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::scaled(const QSize& s, Qt::AspectRatioMode aspectMode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn scaled(&self, (&::qt_core::size::Size, &::qt_core::qt::AspectRatioMode, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::scaled(const QSize& s, Qt::AspectRatioMode aspectMode = ?, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn scaled(&self, (::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::scaled(int w, int h) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn scaled(&self, (::libc::c_int, ::libc::c_int, &::qt_core::qt::AspectRatioMode)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::scaled(int w, int h, Qt::AspectRatioMode aspectMode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn scaled(&self, (::libc::c_int, ::libc::c_int, &::qt_core::qt::AspectRatioMode, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::scaled(int w, int h, Qt::AspectRatioMode aspectMode = ?, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  pub fn scaled<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ImageScaledArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::scaledToHeight```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scaled_to_height(&self, ::libc::c_int) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::scaledToHeight(int h) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scaled_to_height(&self, (::libc::c_int, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::scaledToHeight(int h, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  pub fn scaled_to_height<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ImageScaledToHeightArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::scaledToWidth```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scaled_to_width(&self, ::libc::c_int) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::scaledToWidth(int w) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scaled_to_width(&self, (::libc::c_int, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::scaledToWidth(int w, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  pub fn scaled_to_width<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ImageScaledToWidthArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const unsigned char* QImage::scanLine(int arg1) const```</span>
  ///
  ///
  pub fn scan_line(&self, arg1: ::libc::c_int) -> *const ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QImage_scanLine_const(self as *const ::image::Image, arg1) }
  }

  /// C++ method: <span style='color: green;'>```unsigned char* QImage::scanLine(int arg1)```</span>
  ///
  ///
  pub fn scan_line_mut(&mut self, arg1: ::libc::c_int) -> *mut ::libc::c_uchar {
    unsafe { ::ffi::qt_gui_c_QImage_scanLine(self as *mut ::image::Image, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QImage::setAlphaChannel(const QImage& alphaChannel)```</span>
  ///
  ///
  pub fn set_alpha_channel(&mut self, alpha_channel: &::image::Image) {
    unsafe {
      ::ffi::qt_gui_c_QImage_setAlphaChannel(self as *mut ::image::Image,
                                             alpha_channel as *const ::image::Image)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImage::setColor(int i, unsigned int c)```</span>
  ///
  ///
  pub fn set_color(&mut self, i: ::libc::c_int, c: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QImage_setColor(self as *mut ::image::Image, i, c) }
  }

  /// C++ method: <span style='color: green;'>```void QImage::setColorCount(int arg1)```</span>
  ///
  ///
  pub fn set_color_count(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QImage_setColorCount(self as *mut ::image::Image, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QImage::setColorTable(const QVector<unsigned int> colors)```</span>
  ///
  ///
  pub fn set_color_table(&mut self, colors: &::qt_core::vector::VectorCUint) {
    unsafe {
      ::ffi::qt_gui_c_QImage_setColorTable(self as *mut ::image::Image,
                                           colors as *const ::qt_core::vector::VectorCUint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImage::setDevicePixelRatio(double scaleFactor)```</span>
  ///
  ///
  pub fn set_device_pixel_ratio(&mut self, scale_factor: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QImage_setDevicePixelRatio(self as *mut ::image::Image, scale_factor) }
  }

  /// C++ method: <span style='color: green;'>```void QImage::setDotsPerMeterX(int arg1)```</span>
  ///
  ///
  pub fn set_dots_per_meter_x(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QImage_setDotsPerMeterX(self as *mut ::image::Image, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QImage::setDotsPerMeterY(int arg1)```</span>
  ///
  ///
  pub fn set_dots_per_meter_y(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QImage_setDotsPerMeterY(self as *mut ::image::Image, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QImage::setOffset(const QPoint& arg1)```</span>
  ///
  ///
  pub fn set_offset(&mut self, arg1: &::qt_core::point::Point) {
    unsafe {
      ::ffi::qt_gui_c_QImage_setOffset(self as *mut ::image::Image,
                                       arg1 as *const ::qt_core::point::Point)
    }
  }

  /// C++ method: <span style='color: green;'>```QImage::setPixel```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_pixel(&mut self, (&::qt_core::point::Point, ::libc::c_uint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QImage::setPixel(const QPoint& pt, unsigned int index_or_rgb)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_pixel(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_uint)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QImage::setPixel(int x, int y, unsigned int index_or_rgb)```</span>
  ///
  ///
  pub fn set_pixel<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ImageSetPixelArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::setPixelColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_pixel_color(&mut self, (&::qt_core::point::Point, &::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QImage::setPixelColor(const QPoint& pt, const QColor& c)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_pixel_color(&mut self, (::libc::c_int, ::libc::c_int, &::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QImage::setPixelColor(int x, int y, const QColor& c)```</span>
  ///
  ///
  pub fn set_pixel_color<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ImageSetPixelColorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QImage::setText(const QString& key, const QString& value)```</span>
  ///
  ///
  pub fn set_text(&mut self, key: &::qt_core::string::String, value: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QImage_setText(self as *mut ::image::Image,
                                     key as *const ::qt_core::string::String,
                                     value as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QImage::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImage_size_to_output(self as *const ::image::Image, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QImage::swap(QImage& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::image::Image) {
    unsafe { ::ffi::qt_gui_c_QImage_swap(self as *mut ::image::Image, other as *mut ::image::Image) }
  }

  /// C++ method: <span style='color: green;'>```QImage::text```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn text(&self, ()) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QImage::text() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn text(&self, &::qt_core::string::String) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QImage::text(const QString& key = ?) const```</span>
  ///
  ///
  pub fn text<'largs, Args>(&'largs self, args: Args) -> ::qt_core::string::String
    where Args: overloading::ImageTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStringList QImage::textKeys() const```</span>
  ///
  ///
  pub fn text_keys(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImage_textKeys_to_output(self as *const ::image::Image, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QPixelFormat QImage::toPixelFormat(QImage::Format format)```</span>
  ///
  ///
  pub fn to_pixel_format(format: &::image::Format) -> ::pixel_format::PixelFormat {
    {
      let mut object: ::pixel_format::PixelFormat =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImage_toPixelFormat_to_output(format as *const ::image::Format, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QImage::transformed```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn transformed(&self, &::matrix::Matrix) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::transformed(const QMatrix& matrix) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn transformed(&self, (&::matrix::Matrix, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::transformed(const QMatrix& matrix, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn transformed(&self, &::transform::Transform) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::transformed(const QTransform& matrix) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn transformed(&self, (&::transform::Transform, &::qt_core::qt::TransformationMode)) -> ::cpp_utils::CppBox<::image::Image>```<br>
  /// C++ method: <span style='color: green;'>```QImage QImage::transformed(const QTransform& matrix, Qt::TransformationMode mode = ?) const```</span>
  ///
  ///
  pub fn transformed<'largs, Args>(&'largs self, args: Args) -> ::cpp_utils::CppBox<::image::Image>
    where Args: overloading::ImageTransformedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QImage::trueMatrix```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn true_matrix((&::matrix::Matrix, ::libc::c_int, ::libc::c_int)) -> ::matrix::Matrix```<br>
  /// C++ method: <span style='color: green;'>```static QMatrix QImage::trueMatrix(const QMatrix& arg1, int w, int h)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn true_matrix((&::transform::Transform, ::libc::c_int, ::libc::c_int)) -> ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```static QTransform QImage::trueMatrix(const QTransform& arg1, int w, int h)```</span>
  ///
  ///
  pub fn true_matrix<Args>(args: Args) -> Args::ReturnType
    where Args: overloading::ImageTrueMatrixArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QImage::valid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn valid(&self, &::qt_core::point::Point) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::valid(const QPoint& pt) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn valid(&self, (::libc::c_int, ::libc::c_int)) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QImage::valid(int x, int y) const```</span>
  ///
  ///
  pub fn valid<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::ImageValidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QImage::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImage_width(self as *const ::image::Image) }
  }
}

impl ::cpp_utils::CppDeletable for ::image::Image {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QImage_delete
  }
}

/// C++ type: <span style='color: green;'>```QImage::InvertMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum InvertMode {
  /// C++ enum variant: <span style='color: green;'>```InvertRgb = 0```</span>
  Rgb = 0,
  /// C++ enum variant: <span style='color: green;'>```InvertRgba = 1```</span>
  Rgba = 1,
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::image::Image)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QImage& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::image::Image)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QImage& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QImage& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::qt_core::data_stream::DataStream,
                        arg2: &'l1 mut ::image::Image)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QImage_G_operator_shr(arg1 as *mut ::qt_core::data_stream::DataStream,
                                          arg2 as *mut ::image::Image)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```void swap(QImage& value1, QImage& value2)```</span>
///
///
pub fn swap(value1: &mut ::image::Image, value2: &mut ::image::Image) {
  unsafe { ::ffi::qt_gui_c_QImage_G_swap(value1 as *mut ::image::Image, value2 as *mut ::image::Image) }
}

impl ::cpp_utils::DynamicCast<::image::Image> for ::paint_device::PaintDevice {
  fn dynamic_cast_mut(&mut self) -> ::std::option::Option<&mut ::image::Image> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QImage_G_dynamic_cast_QImage_ptr(self as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_mut() }
  }

  fn dynamic_cast(&self) -> ::std::option::Option<&::image::Image> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QImage_G_dynamic_cast_QImage_ptr(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice) };
    unsafe { ffi_result.as_ref() }
  }
}

impl ::cpp_utils::StaticCast<::paint_device::PaintDevice> for ::image::Image {
  fn static_cast_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QImage_G_static_cast_QPaintDevice_ptr(self as *mut ::image::Image) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  fn static_cast(&self) -> &::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QImage_G_static_cast_QPaintDevice_ptr(self as *const ::image::Image as *mut ::image::Image)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::UnsafeStaticCast<::image::Image> for ::paint_device::PaintDevice {
  unsafe fn static_cast_mut(&mut self) -> &mut ::image::Image {
    let ffi_result = ::ffi::qt_gui_c_QImage_G_static_cast_QImage_ptr(self as *mut ::paint_device::PaintDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  unsafe fn static_cast(&self) -> &::image::Image {
    let ffi_result = ::ffi::qt_gui_c_QImage_G_static_cast_QImage_ptr(self as *const ::paint_device::PaintDevice as *mut ::paint_device::PaintDevice);
    ffi_result.as_ref().expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::Deref for ::image::Image {
  type Target = ::paint_device::PaintDevice;
  fn deref(&self) -> &::paint_device::PaintDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QImage_G_static_cast_QPaintDevice_ptr(self as *const ::image::Image as *mut ::image::Image)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::std::ops::DerefMut for ::image::Image {
  fn deref_mut(&mut self) -> &mut ::paint_device::PaintDevice {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QImage_G_static_cast_QPaintDevice_ptr(self as *mut ::image::Image) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Image::copy](../struct.Image.html#method.copy) method.
  pub trait ImageCopyArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'largs> ImageCopyArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QImage_copy_as_ptr_no_args(original_self as *const ::image::Image) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageCopyArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let rect = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QImage_copy_as_ptr_rect(original_self as *const ::image::Image,
                                                rect as *const ::qt_core::rect::Rect)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageCopyArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QImage_copy_as_ptr_x_y_w_h(original_self as *const ::image::Image, x, y, w, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::create_heuristic_mask](../struct.Image.html#method.create_heuristic_mask) method.
  pub trait ImageCreateHeuristicMaskArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'largs> ImageCreateHeuristicMaskArgs<'largs> for bool {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let clip_tight = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QImage_createHeuristicMask_as_ptr_clipTight(original_self as *const ::image::Image,
                                                                      clip_tight)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageCreateHeuristicMaskArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {

      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QImage_createHeuristicMask_as_ptr_no_args(original_self as *const ::image::Image) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::create_mask_from_color](../struct.Image.html#method.create_mask_from_color) method.
  pub trait ImageCreateMaskFromColorArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'largs> ImageCreateMaskFromColorArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let color = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QImage_createMaskFromColor_as_ptr_color(original_self as *const ::image::Image, color)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageCreateMaskFromColorArgs<'largs> for (::libc::c_uint, &'largs ::qt_core::qt::MaskMode) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let color = self.0;
      let mode = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QImage_createMaskFromColor_as_ptr_color_mode(original_self as *const ::image::Image,
                                                                       color,
                                                                       mode as *const ::qt_core::qt::MaskMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::fill](../struct.Image.html#method.fill) method.
  pub trait ImageFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::image::Image) -> ();
  }
  impl<'largs> ImageFillArgs<'largs> for &'largs ::color::Color {
    fn exec(self, original_self: &'largs mut ::image::Image) -> () {
      let color = self;
      unsafe {
        ::ffi::qt_gui_c_QImage_fill_QColor(original_self as *mut ::image::Image,
                                           color as *const ::color::Color)
      }
    }
  }
  impl<'largs> ImageFillArgs<'largs> for &'largs ::qt_core::qt::GlobalColor {
    fn exec(self, original_self: &'largs mut ::image::Image) -> () {
      let color = self;
      unsafe {
        ::ffi::qt_gui_c_QImage_fill_Qt_GlobalColor(original_self as *mut ::image::Image,
                                                   color as *const ::qt_core::qt::GlobalColor)
      }
    }
  }
  impl<'largs> ImageFillArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::image::Image) -> () {
      let pixel = self;
      unsafe { ::ffi::qt_gui_c_QImage_fill_unsigned_int(original_self as *mut ::image::Image, pixel) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::from_data_unsafe](../struct.Image.html#method.from_data_unsafe) method.
  pub trait ImageFromDataUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'a> ImageFromDataUnsafeArgs for (&'a ::qt_core::byte_array::ByteArray, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let format = self.1;
      let ffi_result =
        ::ffi::qt_gui_c_QImage_fromData_as_ptr_data_format(data as *const ::qt_core::byte_array::ByteArray, format);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageFromDataUnsafeArgs for (*const ::libc::c_uchar, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_gui_c_QImage_fromData_as_ptr_data_size(data, size);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageFromDataUnsafeArgs for (*const ::libc::c_uchar, ::libc::c_int, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let size = self.1;
      let format = self.2;
      let ffi_result = ::ffi::qt_gui_c_QImage_fromData_as_ptr_data_size_format(data, size, format);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Image::invert_pixels](../struct.Image.html#method.invert_pixels) method.
  pub trait ImageInvertPixelsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::image::Image) -> ();
  }
  impl<'largs> ImageInvertPixelsArgs<'largs> for ::image::InvertMode {
    fn exec(self, original_self: &'largs mut ::image::Image) -> () {
      let arg1 = self;
      unsafe { ::ffi::qt_gui_c_QImage_invertPixels_arg1(original_self as *mut ::image::Image, arg1) }
    }
  }
  impl<'largs> ImageInvertPixelsArgs<'largs> for () {
    fn exec(self, original_self: &'largs mut ::image::Image) -> () {

      unsafe { ::ffi::qt_gui_c_QImage_invertPixels_no_args(original_self as *mut ::image::Image) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::load_from_data_unsafe](../struct.Image.html#method.load_from_data_unsafe) method.
  pub trait ImageLoadFromDataUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::image::Image) -> bool;
  }
  impl<'largs> ImageLoadFromDataUnsafeArgs<'largs> for (*const ::libc::c_uchar, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::image::Image) -> bool {
      let buf = self.0;
      let len = self.1;
      ::ffi::qt_gui_c_QImage_loadFromData_buf_len(original_self as *mut ::image::Image, buf, len)
    }
  }
  impl<'largs> ImageLoadFromDataUnsafeArgs<'largs> for (*const ::libc::c_uchar, ::libc::c_int, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::image::Image) -> bool {
      let buf = self.0;
      let len = self.1;
      let format = self.2;
      ::ffi::qt_gui_c_QImage_loadFromData_buf_len_format(original_self as *mut ::image::Image, buf, len, format)
    }
  }
  impl<'largs> ImageLoadFromDataUnsafeArgs<'largs> for (&'largs ::qt_core::byte_array::ByteArray, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::image::Image) -> bool {
      let data = self.0;
      let aformat = self.1;
      ::ffi::qt_gui_c_QImage_loadFromData_data_aformat(original_self as *mut ::image::Image,
                                                       data as *const ::qt_core::byte_array::ByteArray,
                                                       aformat)
    }
  }
  /// This trait represents a set of arguments accepted by [Image::load_unsafe](../struct.Image.html#method.load_unsafe) method.
  pub trait ImageLoadUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::image::Image) -> bool;
  }
  impl<'largs> ImageLoadUnsafeArgs<'largs> for (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::image::Image) -> bool {
      let device = self.0;
      let format = self.1;
      ::ffi::qt_gui_c_QImage_load_device_format(original_self as *mut ::image::Image, device, format)
    }
  }
  impl<'largs> ImageLoadUnsafeArgs<'largs> for (&'largs ::qt_core::string::String, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs mut ::image::Image) -> bool {
      let file_name = self.0;
      let format = self.1;
      ::ffi::qt_gui_c_QImage_load_fileName_format(original_self as *mut ::image::Image,
                                                  file_name as *const ::qt_core::string::String,
                                                  format)
    }
  }
  /// This trait represents a set of arguments accepted by [Image::mirrored](../struct.Image.html#method.mirrored) method.
  pub trait ImageMirroredArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'largs> ImageMirroredArgs<'largs> for bool {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let horizontally = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QImage_mirrored_as_ptr_horizontally(original_self as *const ::image::Image, horizontally)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageMirroredArgs<'largs> for (bool, bool) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let horizontally = self.0;
      let vertically = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QImage_mirrored_as_ptr_horizontally_vertically(original_self as *const ::image::Image,
                                                                         horizontally,
                                                                         vertically)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageMirroredArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {

      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QImage_mirrored_as_ptr_no_args(original_self as *const ::image::Image) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::new](../struct.Image.html#method.new) method.
  pub trait ImageNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'a> ImageNewArgs for &'a ::image::Image {
    fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QImage_new_const_QImage_ref(arg1 as *const ::image::Image) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> ImageNewArgs for (&'a ::qt_core::size::Size, ::image::Format) {
    fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let size = self.0;
      let format = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QImage_new_const_QSize_ref_QImage_Format(size as *const ::qt_core::size::Size, format)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> ImageNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let file_name = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QImage_new_const_QString_ref(file_name as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ImageNewArgs for (::libc::c_int, ::libc::c_int, ::image::Format) {
    fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let width = self.0;
      let height = self.1;
      let format = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QImage_new_int_int_QImage_Format(width, height, format) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ImageNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QImage_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::new_unsafe](../struct.Image.html#method.new_unsafe) method.
  pub trait ImageNewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'a> ImageNewUnsafeArgs for (&'a ::qt_core::string::String, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let file_name = self.0;
      let format = self.1;
      let ffi_result =
        ::ffi::qt_gui_c_QImage_new_const_QString_ref_const_char_ptr(file_name as *const ::qt_core::string::String,
                                                                    format);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs for (*const ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::image::Format) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let format = self.3;
      let ffi_result =
        ::ffi::qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_QImage_Format(data, width, height, format);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs
    for (*const ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::image::Format, extern "C" fn(*mut ::libc::c_void)) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let format = self.3;
      let cleanup_function = self.4;
      let ffi_result =
        ::ffi::qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_QImage_Format_void_func_void_ptr(data,
                                                                                                    width,
                                                                                                    height,
                                                                                                    format,
                                                                                                    cleanup_function);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs
    for (*const ::libc::c_uchar,
                                   ::libc::c_int,
                                   ::libc::c_int,
                                   ::image::Format,
                                   extern "C" fn(*mut ::libc::c_void),
                                   *mut ::libc::c_void) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let format = self.3;
      let cleanup_function = self.4;
      let cleanup_info = self.5;
      let ffi_result = ::ffi::qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_QImage_Format_void_func_void_ptr_void_ptr(data, width, height, format, cleanup_function, cleanup_info);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs for (*const ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::image::Format) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let bytes_per_line = self.3;
      let format = self.4;
      let ffi_result = ::ffi::qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_int_QImage_Format(data,
                                                                                                    width,
                                                                                                    height,
                                                                                                    bytes_per_line,
                                                                                                    format);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs
    for (*const ::libc::c_uchar,
                                   ::libc::c_int,
                                   ::libc::c_int,
                                   ::libc::c_int,
                                   ::image::Format,
                                   extern "C" fn(*mut ::libc::c_void)) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let bytes_per_line = self.3;
      let format = self.4;
      let cleanup_function = self.5;
      let ffi_result = ::ffi::qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_int_QImage_Format_void_func_void_ptr(data, width, height, bytes_per_line, format, cleanup_function);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs
    for (*const ::libc::c_uchar,
                                   ::libc::c_int,
                                   ::libc::c_int,
                                   ::libc::c_int,
                                   ::image::Format,
                                   extern "C" fn(*mut ::libc::c_void),
                                   *mut ::libc::c_void) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let bytes_per_line = self.3;
      let format = self.4;
      let cleanup_function = self.5;
      let cleanup_info = self.6;
      let ffi_result = ::ffi::qt_gui_c_QImage_new_const_unsigned_char_ptr_int_int_int_QImage_Format_void_func_void_ptr_void_ptr(data, width, height, bytes_per_line, format, cleanup_function, cleanup_info);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs for (*mut ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::image::Format) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let format = self.3;
      let ffi_result = ::ffi::qt_gui_c_QImage_new_unsigned_char_ptr_int_int_QImage_Format(data, width, height, format);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs
    for (*mut ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::image::Format, extern "C" fn(*mut ::libc::c_void)) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let format = self.3;
      let cleanup_function = self.4;
      let ffi_result =
        ::ffi::qt_gui_c_QImage_new_unsigned_char_ptr_int_int_QImage_Format_void_func_void_ptr(data,
                                                                                              width,
                                                                                              height,
                                                                                              format,
                                                                                              cleanup_function);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs
    for (*mut ::libc::c_uchar,
                                   ::libc::c_int,
                                   ::libc::c_int,
                                   ::image::Format,
                                   extern "C" fn(*mut ::libc::c_void),
                                   *mut ::libc::c_void) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let format = self.3;
      let cleanup_function = self.4;
      let cleanup_info = self.5;
      let ffi_result = ::ffi::qt_gui_c_QImage_new_unsigned_char_ptr_int_int_QImage_Format_void_func_void_ptr_void_ptr(data, width, height, format, cleanup_function, cleanup_info);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs for (*mut ::libc::c_uchar, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::image::Format) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let bytes_per_line = self.3;
      let format = self.4;
      let ffi_result = ::ffi::qt_gui_c_QImage_new_unsigned_char_ptr_int_int_int_QImage_Format(data,
                                                                                              width,
                                                                                              height,
                                                                                              bytes_per_line,
                                                                                              format);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs
    for (*mut ::libc::c_uchar,
                                   ::libc::c_int,
                                   ::libc::c_int,
                                   ::libc::c_int,
                                   ::image::Format,
                                   extern "C" fn(*mut ::libc::c_void)) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let bytes_per_line = self.3;
      let format = self.4;
      let cleanup_function = self.5;
      let ffi_result =
        ::ffi::qt_gui_c_QImage_new_unsigned_char_ptr_int_int_int_QImage_Format_void_func_void_ptr(data,
                                                                                                  width,
                                                                                                  height,
                                                                                                  bytes_per_line,
                                                                                                  format,
                                                                                                  cleanup_function);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl ImageNewUnsafeArgs
    for (*mut ::libc::c_uchar,
                                   ::libc::c_int,
                                   ::libc::c_int,
                                   ::libc::c_int,
                                   ::image::Format,
                                   extern "C" fn(*mut ::libc::c_void),
                                   *mut ::libc::c_void) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::image::Image> {
      let data = self.0;
      let width = self.1;
      let height = self.2;
      let bytes_per_line = self.3;
      let format = self.4;
      let cleanup_function = self.5;
      let cleanup_info = self.6;
      let ffi_result = ::ffi::qt_gui_c_QImage_new_unsigned_char_ptr_int_int_int_QImage_Format_void_func_void_ptr_void_ptr(data, width, height, bytes_per_line, format, cleanup_function, cleanup_info);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Image::pixel](../struct.Image.html#method.pixel) method.
  pub trait ImagePixelArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::libc::c_uint;
  }
  impl<'largs> ImagePixelArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::image::Image) -> ::libc::c_uint {
      let pt = self;
      unsafe {
        ::ffi::qt_gui_c_QImage_pixel_pt(original_self as *const ::image::Image,
                                        pt as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> ImagePixelArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::libc::c_uint {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_gui_c_QImage_pixel_x_y(original_self as *const ::image::Image, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::pixel_color](../struct.Image.html#method.pixel_color) method.
  pub trait ImagePixelColorArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::color::Color;
  }
  impl<'largs> ImagePixelColorArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::image::Image) -> ::color::Color {
      let pt = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QImage_pixelColor_to_output_pt(original_self as *const ::image::Image,
                                                         pt as *const ::qt_core::point::Point,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ImagePixelColorArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::color::Color {
      let x = self.0;
      let y = self.1;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QImage_pixelColor_to_output_x_y(original_self as *const ::image::Image, x, y, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::pixel_index](../struct.Image.html#method.pixel_index) method.
  pub trait ImagePixelIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::libc::c_int;
  }
  impl<'largs> ImagePixelIndexArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::image::Image) -> ::libc::c_int {
      let pt = self;
      unsafe {
        ::ffi::qt_gui_c_QImage_pixelIndex_pt(original_self as *const ::image::Image,
                                             pt as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> ImagePixelIndexArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::libc::c_int {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_gui_c_QImage_pixelIndex_x_y(original_self as *const ::image::Image, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::save_unsafe](../struct.Image.html#method.save_unsafe) method.
  pub trait ImageSaveUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::image::Image) -> bool;
  }
  impl<'largs> ImageSaveUnsafeArgs<'largs> for *mut ::qt_core::io_device::IODevice {
    unsafe fn exec(self, original_self: &'largs ::image::Image) -> bool {
      let device = self;
      ::ffi::qt_gui_c_QImage_save_device(original_self as *const ::image::Image, device)
    }
  }
  impl<'largs> ImageSaveUnsafeArgs<'largs> for (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs ::image::Image) -> bool {
      let device = self.0;
      let format = self.1;
      ::ffi::qt_gui_c_QImage_save_device_format(original_self as *const ::image::Image, device, format)
    }
  }
  impl<'largs> ImageSaveUnsafeArgs<'largs>
    for (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::image::Image) -> bool {
      let device = self.0;
      let format = self.1;
      let quality = self.2;
      ::ffi::qt_gui_c_QImage_save_device_format_quality(original_self as *const ::image::Image,
                                                        device,
                                                        format,
                                                        quality)
    }
  }
  impl<'largs> ImageSaveUnsafeArgs<'largs> for (&'largs ::qt_core::string::String, *const ::libc::c_char) {
    unsafe fn exec(self, original_self: &'largs ::image::Image) -> bool {
      let file_name = self.0;
      let format = self.1;
      ::ffi::qt_gui_c_QImage_save_fileName_format(original_self as *const ::image::Image,
                                                  file_name as *const ::qt_core::string::String,
                                                  format)
    }
  }
  impl<'largs> ImageSaveUnsafeArgs<'largs> for (&'largs ::qt_core::string::String, *const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::image::Image) -> bool {
      let file_name = self.0;
      let format = self.1;
      let quality = self.2;
      ::ffi::qt_gui_c_QImage_save_fileName_format_quality(original_self as *const ::image::Image,
                                                          file_name as *const ::qt_core::string::String,
                                                          format,
                                                          quality)
    }
  }
  /// This trait represents a set of arguments accepted by [Image::scaled](../struct.Image.html#method.scaled) method.
  pub trait ImageScaledArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'largs> ImageScaledArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let s = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QImage_scaled_as_ptr_s(original_self as *const ::image::Image,
                                               s as *const ::qt_core::size::Size)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageScaledArgs<'largs> for (&'largs ::qt_core::size::Size, &'largs ::qt_core::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let s = self.0;
      let aspect_mode = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QImage_scaled_as_ptr_s_aspectMode(original_self as *const ::image::Image,
                                                            s as *const ::qt_core::size::Size,
                                                            aspect_mode as *const ::qt_core::qt::AspectRatioMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageScaledArgs<'largs>
    for (&'largs ::qt_core::size::Size,
                                                &'largs ::qt_core::qt::AspectRatioMode,
                                                &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let s = self.0;
      let aspect_mode = self.1;
      let mode = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QImage_scaled_as_ptr_s_aspectMode_mode(original_self as *const ::image::Image,
                                                                 s as *const ::qt_core::size::Size,
                                                                 aspect_mode as *const ::qt_core::qt::AspectRatioMode,
                                                                 mode as *const ::qt_core::qt::TransformationMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageScaledArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let w = self.0;
      let h = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QImage_scaled_as_ptr_w_h(original_self as *const ::image::Image, w, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageScaledArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::qt::AspectRatioMode) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let w = self.0;
      let h = self.1;
      let aspect_mode = self.2;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QImage_scaled_as_ptr_w_h_aspectMode(original_self as *const ::image::Image,
                                                              w,
                                                              h,
                                                              aspect_mode as *const ::qt_core::qt::AspectRatioMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageScaledArgs<'largs>
    for (::libc::c_int,
                                                ::libc::c_int,
                                                &'largs ::qt_core::qt::AspectRatioMode,
                                                &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let w = self.0;
      let h = self.1;
      let aspect_mode = self.2;
      let mode = self.3;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QImage_scaled_as_ptr_w_h_aspectMode_mode(original_self as *const ::image::Image, w, h, aspect_mode as *const ::qt_core::qt::AspectRatioMode, mode as *const ::qt_core::qt::TransformationMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::scaled_to_height](../struct.Image.html#method.scaled_to_height) method.
  pub trait ImageScaledToHeightArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'largs> ImageScaledToHeightArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let h = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QImage_scaledToHeight_as_ptr_h(original_self as *const ::image::Image, h) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageScaledToHeightArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let h = self.0;
      let mode = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QImage_scaledToHeight_as_ptr_h_mode(original_self as *const ::image::Image,
                                                              h,
                                                              mode as *const ::qt_core::qt::TransformationMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::scaled_to_width](../struct.Image.html#method.scaled_to_width) method.
  pub trait ImageScaledToWidthArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'largs> ImageScaledToWidthArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let w = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QImage_scaledToWidth_as_ptr_w(original_self as *const ::image::Image, w) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageScaledToWidthArgs<'largs> for (::libc::c_int, &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let w = self.0;
      let mode = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QImage_scaledToWidth_as_ptr_w_mode(original_self as *const ::image::Image,
                                                             w,
                                                             mode as *const ::qt_core::qt::TransformationMode)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::set_pixel](../struct.Image.html#method.set_pixel) method.
  pub trait ImageSetPixelArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::image::Image) -> ();
  }
  impl<'largs> ImageSetPixelArgs<'largs> for (&'largs ::qt_core::point::Point, ::libc::c_uint) {
    fn exec(self, original_self: &'largs mut ::image::Image) -> () {
      let pt = self.0;
      let index_or_rgb = self.1;
      unsafe {
        ::ffi::qt_gui_c_QImage_setPixel_pt_index_or_rgb(original_self as *mut ::image::Image,
                                                        pt as *const ::qt_core::point::Point,
                                                        index_or_rgb)
      }
    }
  }
  impl<'largs> ImageSetPixelArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_uint) {
    fn exec(self, original_self: &'largs mut ::image::Image) -> () {
      let x = self.0;
      let y = self.1;
      let index_or_rgb = self.2;
      unsafe {
        ::ffi::qt_gui_c_QImage_setPixel_x_y_index_or_rgb(original_self as *mut ::image::Image, x, y, index_or_rgb)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::set_pixel_color](../struct.Image.html#method.set_pixel_color) method.
  pub trait ImageSetPixelColorArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::image::Image) -> ();
  }
  impl<'largs> ImageSetPixelColorArgs<'largs> for (&'largs ::qt_core::point::Point, &'largs ::color::Color) {
    fn exec(self, original_self: &'largs mut ::image::Image) -> () {
      let pt = self.0;
      let c = self.1;
      unsafe {
        ::ffi::qt_gui_c_QImage_setPixelColor_pt_c(original_self as *mut ::image::Image,
                                                  pt as *const ::qt_core::point::Point,
                                                  c as *const ::color::Color)
      }
    }
  }
  impl<'largs> ImageSetPixelColorArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::color::Color) {
    fn exec(self, original_self: &'largs mut ::image::Image) -> () {
      let x = self.0;
      let y = self.1;
      let c = self.2;
      unsafe {
        ::ffi::qt_gui_c_QImage_setPixelColor_x_y_c(original_self as *mut ::image::Image,
                                                   x,
                                                   y,
                                                   c as *const ::color::Color)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::text](../struct.Image.html#method.text) method.
  pub trait ImageTextArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::qt_core::string::String;
  }
  impl<'largs> ImageTextArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::image::Image) -> ::qt_core::string::String {
      let key = self;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QImage_text_to_output_key(original_self as *const ::image::Image,
                                                    key as *const ::qt_core::string::String,
                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ImageTextArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::image::Image) -> ::qt_core::string::String {

      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QImage_text_to_output_no_args(original_self as *const ::image::Image, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::transformed](../struct.Image.html#method.transformed) method.
  pub trait ImageTransformedArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image>;
  }
  impl<'largs> ImageTransformedArgs<'largs> for &'largs ::matrix::Matrix {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let matrix = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QImage_transformed_as_ptr_QMatrix(original_self as *const ::image::Image,
                                                          matrix as *const ::matrix::Matrix)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageTransformedArgs<'largs> for (&'largs ::matrix::Matrix, &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let matrix = self.0;
      let mode = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QImage_transformed_as_ptr_QMatrix_Qt_TransformationMode(original_self as *const ::image::Image, matrix as *const ::matrix::Matrix, mode as *const ::qt_core::qt::TransformationMode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageTransformedArgs<'largs> for &'largs ::transform::Transform {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let matrix = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QImage_transformed_as_ptr_QTransform(original_self as *const ::image::Image,
                                                             matrix as *const ::transform::Transform)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> ImageTransformedArgs<'largs>
    for (&'largs ::transform::Transform, &'largs ::qt_core::qt::TransformationMode) {
    fn exec(self, original_self: &'largs ::image::Image) -> ::cpp_utils::CppBox<::image::Image> {
      let matrix = self.0;
      let mode = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QImage_transformed_as_ptr_QTransform_Qt_TransformationMode(original_self as *const ::image::Image, matrix as *const ::transform::Transform, mode as *const ::qt_core::qt::TransformationMode) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::true_matrix](../struct.Image.html#method.true_matrix) method.
  pub trait ImageTrueMatrixArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> ImageTrueMatrixArgs for (&'a ::matrix::Matrix, ::libc::c_int, ::libc::c_int) {
    type ReturnType = ::matrix::Matrix;
    fn exec(self) -> ::matrix::Matrix {
      let arg1 = self.0;
      let w = self.1;
      let h = self.2;
      {
        let mut object: ::matrix::Matrix =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QImage_trueMatrix_to_output_QMatrix_int_int(arg1 as *const ::matrix::Matrix,
                                                                      w,
                                                                      h,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl<'a> ImageTrueMatrixArgs for (&'a ::transform::Transform, ::libc::c_int, ::libc::c_int) {
    type ReturnType = ::transform::Transform;
    fn exec(self) -> ::transform::Transform {
      let arg1 = self.0;
      let w = self.1;
      let h = self.2;
      {
        let mut object: ::transform::Transform =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QImage_trueMatrix_to_output_QTransform_int_int(arg1 as *const ::transform::Transform,
                                                                         w,
                                                                         h,
                                                                         &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Image::valid](../struct.Image.html#method.valid) method.
  pub trait ImageValidArgs<'largs> {
    fn exec(self, original_self: &'largs ::image::Image) -> bool;
  }
  impl<'largs> ImageValidArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs ::image::Image) -> bool {
      let pt = self;
      unsafe {
        ::ffi::qt_gui_c_QImage_valid_pt(original_self as *const ::image::Image,
                                        pt as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> ImageValidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::image::Image) -> bool {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_gui_c_QImage_valid_x_y(original_self as *const ::image::Image, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::image::Image) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QImage_G_operator_shl(arg1 as *mut ::qt_core::data_stream::DataStream,
                                              arg2 as *const ::image::Image)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::image::Image) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QImage_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                          arg2 as *const ::image::Image,
                                                          &mut object);
        }
        object
      }
    }
  }
}
