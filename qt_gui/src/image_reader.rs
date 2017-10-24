/// C++ type: <span style='color: green;'>```QImageReader```</span>
#[repr(C)]
pub struct ImageReader([u8; ::type_sizes::QT_GUI_IMAGE_READER_IMAGE_READER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ImageReader {
  unsafe fn new_uninitialized() -> ImageReader {
    ImageReader(::std::mem::uninitialized())
  }
}

impl ImageReader {
  /// C++ method: <span style='color: green;'>```bool QImageReader::autoDetectImageFormat() const```</span>
  ///
  ///
  pub fn auto_detect_image_format(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageReader_autoDetectImageFormat(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QImageReader::autoTransform() const```</span>
  ///
  ///
  pub fn auto_transform(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageReader_autoTransform(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```QColor QImageReader::backgroundColor() const```</span>
  ///
  ///
  pub fn background_color(&self) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_backgroundColor_to_output(self as *const ::image_reader::ImageReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QImageReader::canRead() const```</span>
  ///
  ///
  pub fn can_read(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageReader_canRead(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```QRect QImageReader::clipRect() const```</span>
  ///
  ///
  pub fn clip_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_clipRect_to_output(self as *const ::image_reader::ImageReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QImageReader::currentImageNumber() const```</span>
  ///
  ///
  pub fn current_image_number(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImageReader_currentImageNumber(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```QRect QImageReader::currentImageRect() const```</span>
  ///
  ///
  pub fn current_image_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_currentImageRect_to_output(self as *const ::image_reader::ImageReader,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QImageReader::decideFormatFromContent() const```</span>
  ///
  ///
  pub fn decide_format_from_content(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageReader_decideFormatFromContent(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```QIODevice* QImageReader::device() const```</span>
  ///
  ///
  pub fn device(&self) -> *mut ::qt_core::io_device::IODevice {
    unsafe { ::ffi::qt_gui_c_QImageReader_device(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```QImageReader::ImageReaderError QImageReader::error() const```</span>
  ///
  ///
  pub fn error(&self) -> ::image_reader::ImageReaderError {
    unsafe { ::ffi::qt_gui_c_QImageReader_error(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```QString QImageReader::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_errorString_to_output(self as *const ::image_reader::ImageReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QImageReader::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_fileName_to_output(self as *const ::image_reader::ImageReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QImageReader::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_format_to_output(self as *const ::image_reader::ImageReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float QImageReader::gamma() const```</span>
  ///
  ///
  pub fn gamma(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QImageReader_gamma(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```int QImageReader::imageCount() const```</span>
  ///
  ///
  pub fn image_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImageReader_imageCount(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```static QByteArray QImageReader::imageFormat(const QString& fileName)```</span>
  ///
  ///
  pub fn image_format(file_name: &::qt_core::string::String) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_imageFormat_to_output_fileName(file_name as *const ::qt_core::string::String,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QByteArray QImageReader::imageFormat(QIODevice* device)```</span>
  ///
  ///
  pub unsafe fn image_format_unsafe(device: *mut ::qt_core::io_device::IODevice) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QImageReader_imageFormat_to_output_device(device, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QImageReader::jumpToImage(int imageNumber)```</span>
  ///
  ///
  pub fn jump_to_image(&mut self, image_number: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageReader_jumpToImage(self as *mut ::image_reader::ImageReader, image_number) }
  }

  /// C++ method: <span style='color: green;'>```bool QImageReader::jumpToNextImage()```</span>
  ///
  ///
  pub fn jump_to_next_image(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageReader_jumpToNextImage(self as *mut ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```int QImageReader::loopCount() const```</span>
  ///
  ///
  pub fn loop_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImageReader_loopCount(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```QImageReader::QImageReader```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::image_reader::ImageReader```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImageReader::QImageReader()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::image_reader::ImageReader```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImageReader::QImageReader(const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, &::qt_core::byte_array::ByteArray)) -> ::image_reader::ImageReader```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImageReader::QImageReader(const QString& fileName, const QByteArray& format = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::image_reader::ImageReader
    where Args: overloading::ImageReaderNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QImageReader::QImageReader```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*mut ::qt_core::io_device::IODevice) -> ::image_reader::ImageReader```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImageReader::QImageReader(QIODevice* device)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::qt_core::io_device::IODevice, &::qt_core::byte_array::ByteArray)) -> ::image_reader::ImageReader```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImageReader::QImageReader(QIODevice* device, const QByteArray& format = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::image_reader::ImageReader
    where Args: overloading::ImageReaderNewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```int QImageReader::nextImageDelay() const```</span>
  ///
  ///
  pub fn next_image_delay(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImageReader_nextImageDelay(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```int QImageReader::quality() const```</span>
  ///
  ///
  pub fn quality(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImageReader_quality(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```QImage QImageReader::read()```</span>
  ///
  ///
  pub fn read(&mut self) -> ::cpp_utils::CppBox<::image::Image> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QImageReader_read_as_ptr(self as *mut ::image_reader::ImageReader) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```bool QImageReader::read(QImage* image)```</span>
  ///
  ///
  pub unsafe fn read_unsafe(&mut self, image: *mut ::image::Image) -> bool {
    ::ffi::qt_gui_c_QImageReader_read(self as *mut ::image_reader::ImageReader, image)
  }

  /// C++ method: <span style='color: green;'>```QRect QImageReader::scaledClipRect() const```</span>
  ///
  ///
  pub fn scaled_clip_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_scaledClipRect_to_output(self as *const ::image_reader::ImageReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QImageReader::scaledSize() const```</span>
  ///
  ///
  pub fn scaled_size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_scaledSize_to_output(self as *const ::image_reader::ImageReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setAutoDetectImageFormat(bool enabled)```</span>
  ///
  ///
  pub fn set_auto_detect_image_format(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_gui_c_QImageReader_setAutoDetectImageFormat(self as *mut ::image_reader::ImageReader, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setAutoTransform(bool enabled)```</span>
  ///
  ///
  pub fn set_auto_transform(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_gui_c_QImageReader_setAutoTransform(self as *mut ::image_reader::ImageReader, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setBackgroundColor(const QColor& color)```</span>
  ///
  ///
  pub fn set_background_color(&mut self, color: &::color::Color) {
    unsafe {
      ::ffi::qt_gui_c_QImageReader_setBackgroundColor(self as *mut ::image_reader::ImageReader,
                                                      color as *const ::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setClipRect(const QRect& rect)```</span>
  ///
  ///
  pub fn set_clip_rect(&mut self, rect: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_gui_c_QImageReader_setClipRect(self as *mut ::image_reader::ImageReader,
                                               rect as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setDecideFormatFromContent(bool ignored)```</span>
  ///
  ///
  pub fn set_decide_format_from_content(&mut self, ignored: bool) {
    unsafe {
      ::ffi::qt_gui_c_QImageReader_setDecideFormatFromContent(self as *mut ::image_reader::ImageReader, ignored)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setDevice(QIODevice* device)```</span>
  ///
  ///
  pub unsafe fn set_device(&mut self, device: *mut ::qt_core::io_device::IODevice) {
    ::ffi::qt_gui_c_QImageReader_setDevice(self as *mut ::image_reader::ImageReader, device)
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setFileName(const QString& fileName)```</span>
  ///
  ///
  pub fn set_file_name(&mut self, file_name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QImageReader_setFileName(self as *mut ::image_reader::ImageReader,
                                               file_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setFormat(const QByteArray& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::qt_core::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_gui_c_QImageReader_setFormat(self as *mut ::image_reader::ImageReader,
                                             format as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setGamma(float gamma)```</span>
  ///
  ///
  pub fn set_gamma(&mut self, gamma: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QImageReader_setGamma(self as *mut ::image_reader::ImageReader, gamma) }
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setQuality(int quality)```</span>
  ///
  ///
  pub fn set_quality(&mut self, quality: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QImageReader_setQuality(self as *mut ::image_reader::ImageReader, quality) }
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setScaledClipRect(const QRect& rect)```</span>
  ///
  ///
  pub fn set_scaled_clip_rect(&mut self, rect: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_gui_c_QImageReader_setScaledClipRect(self as *mut ::image_reader::ImageReader,
                                                     rect as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageReader::setScaledSize(const QSize& size)```</span>
  ///
  ///
  pub fn set_scaled_size(&mut self, size: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QImageReader_setScaledSize(self as *mut ::image_reader::ImageReader,
                                                 size as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QImageReader::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_size_to_output(self as *const ::image_reader::ImageReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QImageReader::subType() const```</span>
  ///
  ///
  pub fn sub_type(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_subType_to_output(self as *const ::image_reader::ImageReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QImageReader::supportedImageFormats()```</span>
  ///
  ///
  pub fn supported_image_formats() -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_supportedImageFormats_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QImageReader::supportedMimeTypes()```</span>
  ///
  ///
  pub fn supported_mime_types() -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_supportedMimeTypes_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray> QImageReader::supportedSubTypes() const```</span>
  ///
  ///
  pub fn supported_sub_types(&self) -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_supportedSubTypes_to_output(self as *const ::image_reader::ImageReader,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QImageReader::supportsAnimation() const```</span>
  ///
  ///
  pub fn supports_animation(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageReader_supportsAnimation(self as *const ::image_reader::ImageReader) }
  }

  /// C++ method: <span style='color: green;'>```bool QImageReader::supportsOption(QImageIOHandler::ImageOption option) const```</span>
  ///
  ///
  pub fn supports_option(&self, option: &::image_io_handler::ImageOption) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QImageReader_supportsOption(self as *const ::image_reader::ImageReader,
                                                  option as *const ::image_io_handler::ImageOption)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QImageReader::text(const QString& key) const```</span>
  ///
  ///
  pub fn text(&self, key: &::qt_core::string::String) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_text_to_output(self as *const ::image_reader::ImageReader,
                                                    key as *const ::qt_core::string::String,
                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStringList QImageReader::textKeys() const```</span>
  ///
  ///
  pub fn text_keys(&self) -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageReader_textKeys_to_output(self as *const ::image_reader::ImageReader, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QImageReader::tr(const char* sourceText, const char* disambiguation, int n)```</span>
  ///
  ///
  pub unsafe fn tr(source_text: *const ::libc::c_char,
                   disambiguation: *const ::libc::c_char,
                   n: ::libc::c_int)
                   -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QImageReader_tr_to_output(source_text, disambiguation, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QImageReader::trUtf8(const char* sourceText, const char* disambiguation, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(source_text: *const ::libc::c_char,
                        disambiguation: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QImageReader_trUtf8_to_output(source_text, disambiguation, n, &mut object);
      object
    }
  }
}

impl Drop for ::image_reader::ImageReader {
  /// C++ method: <span style='color: green;'>```[destructor] void QImageReader::~QImageReader()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QImageReader_destructor(self as *mut ::image_reader::ImageReader) }
  }
}

/// C++ type: <span style='color: green;'>```QImageReader::ImageReaderError```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ImageReaderError {
  /// C++ enum variant: <span style='color: green;'>```UnknownError = 0```</span>
  Unknown = 0,
  /// C++ enum variant: <span style='color: green;'>```FileNotFoundError = 1```</span>
  FileNotFound = 1,
  /// C++ enum variant: <span style='color: green;'>```DeviceError = 2```</span>
  Device = 2,
  /// C++ enum variant: <span style='color: green;'>```UnsupportedFormatError = 3```</span>
  UnsupportedFormat = 3,
  /// C++ enum variant: <span style='color: green;'>```InvalidDataError = 4```</span>
  InvalidData = 4,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ImageReader::new](../struct.ImageReader.html#method.new) method.
  pub trait ImageReaderNewArgs {
    fn exec(self) -> ::image_reader::ImageReader;
  }
  impl<'a> ImageReaderNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::image_reader::ImageReader {
      let file_name = self;
      {
        let mut object: ::image_reader::ImageReader =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QImageReader_constructor_fileName(file_name as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> ImageReaderNewArgs for (&'a ::qt_core::string::String, &'a ::qt_core::byte_array::ByteArray) {
    fn exec(self) -> ::image_reader::ImageReader {
      let file_name = self.0;
      let format = self.1;
      {
        let mut object: ::image_reader::ImageReader =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QImageReader_constructor_fileName_format(file_name as *const ::qt_core::string::String,
                                                                   format as *const ::qt_core::byte_array::ByteArray,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl ImageReaderNewArgs for () {
    fn exec(self) -> ::image_reader::ImageReader {

      {
        let mut object: ::image_reader::ImageReader =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QImageReader_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ImageReader::new_unsafe](../struct.ImageReader.html#method.new_unsafe) method.
  pub trait ImageReaderNewUnsafeArgs {
    unsafe fn exec(self) -> ::image_reader::ImageReader;
  }
  impl ImageReaderNewUnsafeArgs for *mut ::qt_core::io_device::IODevice {
    unsafe fn exec(self) -> ::image_reader::ImageReader {
      let device = self;
      {
        let mut object: ::image_reader::ImageReader =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_gui_c_QImageReader_constructor_device(device, &mut object);
        object
      }
    }
  }
  impl<'a> ImageReaderNewUnsafeArgs for (*mut ::qt_core::io_device::IODevice, &'a ::qt_core::byte_array::ByteArray) {
    unsafe fn exec(self) -> ::image_reader::ImageReader {
      let device = self.0;
      let format = self.1;
      {
        let mut object: ::image_reader::ImageReader =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_gui_c_QImageReader_constructor_device_format(device,
                                                               format as *const ::qt_core::byte_array::ByteArray,
                                                               &mut object);
        object
      }
    }
  }
}
