/// C++ type: <span style='color: green;'>```QImageIOHandler```</span>
#[repr(C)]
pub struct ImageIOHandler(u8);

impl ImageIOHandler {
  /// C++ method: <span style='color: green;'>```pure virtual bool QImageIOHandler::canRead() const```</span>
  ///
  ///
  pub fn can_read(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageIOHandler_canRead(self as *const ::image_io_handler::ImageIOHandler) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QImageIOHandler::currentImageNumber() const```</span>
  ///
  ///
  pub fn current_image_number(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImageIOHandler_currentImageNumber(self as *const ::image_io_handler::ImageIOHandler) }
  }

  /// C++ method: <span style='color: green;'>```virtual QRect QImageIOHandler::currentImageRect() const```</span>
  ///
  ///
  pub fn current_image_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageIOHandler_currentImageRect_to_output(self as *const ::image_io_handler::ImageIOHandler,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QIODevice* QImageIOHandler::device() const```</span>
  ///
  ///
  pub fn device(&self) -> *mut ::qt_core::io_device::IODevice {
    unsafe { ::ffi::qt_gui_c_QImageIOHandler_device(self as *const ::image_io_handler::ImageIOHandler) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QImageIOHandler::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageIOHandler_format_to_output(self as *const ::image_io_handler::ImageIOHandler,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QImageIOHandler::imageCount() const```</span>
  ///
  ///
  pub fn image_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImageIOHandler_imageCount(self as *const ::image_io_handler::ImageIOHandler) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QImageIOHandler::jumpToImage(int imageNumber)```</span>
  ///
  ///
  pub fn jump_to_image(&mut self, image_number: ::libc::c_int) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QImageIOHandler_jumpToImage(self as *mut ::image_io_handler::ImageIOHandler,
                                                  image_number)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QImageIOHandler::jumpToNextImage()```</span>
  ///
  ///
  pub fn jump_to_next_image(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageIOHandler_jumpToNextImage(self as *mut ::image_io_handler::ImageIOHandler) }
  }

  /// C++ method: <span style='color: green;'>```virtual int QImageIOHandler::loopCount() const```</span>
  ///
  ///
  pub fn loop_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImageIOHandler_loopCount(self as *const ::image_io_handler::ImageIOHandler) }
  }

  /// C++ method: <span style='color: green;'>```virtual QByteArray QImageIOHandler::name() const```</span>
  ///
  ///
  pub fn name(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageIOHandler_name_to_output(self as *const ::image_io_handler::ImageIOHandler,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```virtual int QImageIOHandler::nextImageDelay() const```</span>
  ///
  ///
  pub fn next_image_delay(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImageIOHandler_nextImageDelay(self as *const ::image_io_handler::ImageIOHandler) }
  }

  /// C++ method: <span style='color: green;'>```virtual QVariant QImageIOHandler::option(QImageIOHandler::ImageOption option) const```</span>
  ///
  ///
  pub fn option(&self, option: ::image_io_handler::ImageOption) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageIOHandler_option_to_output(self as *const ::image_io_handler::ImageIOHandler,
                                                         option,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QImageIOHandler::read(QImage* image)```</span>
  ///
  ///
  pub unsafe fn read(&mut self, image: *mut ::image::Image) -> bool {
    ::ffi::qt_gui_c_QImageIOHandler_read(self as *mut ::image_io_handler::ImageIOHandler, image)
  }

  /// C++ method: <span style='color: green;'>```void QImageIOHandler::setDevice(QIODevice* device)```</span>
  ///
  ///
  pub unsafe fn set_device(&mut self, device: *mut ::qt_core::io_device::IODevice) {
    ::ffi::qt_gui_c_QImageIOHandler_setDevice(self as *mut ::image_io_handler::ImageIOHandler, device)
  }

  /// C++ method: <span style='color: green;'>```void QImageIOHandler::setFormat(const QByteArray& format) const```</span>
  ///
  ///
  pub fn set_format(&self, format: &::qt_core::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_gui_c_QImageIOHandler_setFormat_const(self as *const ::image_io_handler::ImageIOHandler,
                                                      format as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageIOHandler::setFormat(const QByteArray& format)```</span>
  ///
  ///
  pub fn set_format_mut(&mut self, format: &::qt_core::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_gui_c_QImageIOHandler_setFormat(self as *mut ::image_io_handler::ImageIOHandler,
                                                format as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QImageIOHandler::setOption(QImageIOHandler::ImageOption option, const QVariant& value)```</span>
  ///
  ///
  pub fn set_option(&mut self, option: ::image_io_handler::ImageOption, value: &::qt_core::variant::Variant) {
    unsafe {
      ::ffi::qt_gui_c_QImageIOHandler_setOption(self as *mut ::image_io_handler::ImageIOHandler,
                                                option,
                                                value as *const ::qt_core::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QImageIOHandler::supportsOption(QImageIOHandler::ImageOption option) const```</span>
  ///
  ///
  pub fn supports_option(&self, option: ::image_io_handler::ImageOption) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageIOHandler_supportsOption(self as *const ::image_io_handler::ImageIOHandler, option) }
  }

  /// C++ method: <span style='color: green;'>```virtual bool QImageIOHandler::write(const QImage& image)```</span>
  ///
  ///
  pub fn write(&mut self, image: &::image::Image) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QImageIOHandler_write(self as *mut ::image_io_handler::ImageIOHandler,
                                            image as *const ::image::Image)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::image_io_handler::ImageIOHandler {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QImageIOHandler_delete
  }
}

/// C++ type: <span style='color: green;'>```QImageIOHandler::ImageOption```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ImageOption {
  /// C++ enum variant: <span style='color: green;'>```Size = 0```</span>
  Size = 0,
  /// C++ enum variant: <span style='color: green;'>```ClipRect = 1```</span>
  ClipRect = 1,
  /// C++ enum variant: <span style='color: green;'>```Description = 2```</span>
  Description = 2,
  /// C++ enum variant: <span style='color: green;'>```ScaledClipRect = 3```</span>
  ScaledClipRect = 3,
  /// C++ enum variant: <span style='color: green;'>```ScaledSize = 4```</span>
  ScaledSize = 4,
  /// C++ enum variant: <span style='color: green;'>```CompressionRatio = 5```</span>
  CompressionRatio = 5,
  /// C++ enum variant: <span style='color: green;'>```Gamma = 6```</span>
  Gamma = 6,
  /// C++ enum variant: <span style='color: green;'>```Quality = 7```</span>
  Quality = 7,
  /// C++ enum variant: <span style='color: green;'>```Name = 8```</span>
  Name = 8,
  /// C++ enum variant: <span style='color: green;'>```SubType = 9```</span>
  SubType = 9,
  /// C++ enum variant: <span style='color: green;'>```IncrementalReading = 10```</span>
  IncrementalReading = 10,
  /// C++ enum variant: <span style='color: green;'>```Endianness = 11```</span>
  Endianness = 11,
  /// C++ enum variant: <span style='color: green;'>```Animation = 12```</span>
  Animation = 12,
  /// C++ enum variant: <span style='color: green;'>```BackgroundColor = 13```</span>
  BackgroundColor = 13,
  /// C++ enum variant: <span style='color: green;'>```ImageFormat = 14```</span>
  ImageFormat = 14,
  /// C++ enum variant: <span style='color: green;'>```SupportedSubTypes = 15```</span>
  SupportedSubTypes = 15,
  /// C++ enum variant: <span style='color: green;'>```OptimizedWrite = 16```</span>
  OptimizedWrite = 16,
  /// C++ enum variant: <span style='color: green;'>```ProgressiveScanWrite = 17```</span>
  ProgressiveScanWrite = 17,
  /// C++ enum variant: <span style='color: green;'>```ImageTransformation = 18```</span>
  ImageTransformation = 18,
  /// C++ enum variant: <span style='color: green;'>```TransformedByDefault = 19```</span>
  TransformedByDefault = 19,
}

/// C++ type: <span style='color: green;'>```QImageIOHandler::Transformation```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Transformation {
  /// C++ enum variant: <span style='color: green;'>```TransformationNone = 0```</span>
  None = 0,
  /// C++ enum variant: <span style='color: green;'>```TransformationMirror = 1```</span>
  Mirror = 1,
  /// C++ enum variant: <span style='color: green;'>```TransformationFlip = 2```</span>
  Flip = 2,
  /// C++ enum variant: <span style='color: green;'>```TransformationRotate180 = 3```</span>
  Rotate180 = 3,
  /// C++ enum variant: <span style='color: green;'>```TransformationRotate90 = 4```</span>
  Rotate90 = 4,
  /// C++ enum variant: <span style='color: green;'>```TransformationMirrorAndRotate90 = 5```</span>
  MirrorAndRotate90 = 5,
  /// C++ enum variant: <span style='color: green;'>```TransformationFlipAndRotate90 = 6```</span>
  FlipAndRotate90 = 6,
  /// C++ enum variant: <span style='color: green;'>```TransformationRotate270 = 7```</span>
  Rotate270 = 7,
}
