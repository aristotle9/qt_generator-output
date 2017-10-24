/// C++ type: <span style='color: green;'>```QImageWriter```</span>
#[repr(C)]
pub struct ImageWriter(u8);

impl ImageWriter {
  /// C++ method: <span style='color: green;'>```bool QImageWriter::canWrite() const```</span>
  ///
  ///
  pub fn can_write(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageWriter_canWrite(self as *const ::image_writer::ImageWriter) }
  }

  /// C++ method: <span style='color: green;'>```int QImageWriter::compression() const```</span>
  ///
  ///
  pub fn compression(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImageWriter_compression(self as *const ::image_writer::ImageWriter) }
  }

  /// C++ method: <span style='color: green;'>```QString QImageWriter::description() const```</span>
  ///
  ///
  pub fn description(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageWriter_description_to_output(self as *const ::image_writer::ImageWriter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QIODevice* QImageWriter::device() const```</span>
  ///
  ///
  pub fn device(&self) -> *mut ::qt_core::io_device::IODevice {
    unsafe { ::ffi::qt_gui_c_QImageWriter_device(self as *const ::image_writer::ImageWriter) }
  }

  /// C++ method: <span style='color: green;'>```QImageWriter::ImageWriterError QImageWriter::error() const```</span>
  ///
  ///
  pub fn error(&self) -> ::image_writer::ImageWriterError {
    unsafe { ::ffi::qt_gui_c_QImageWriter_error(self as *const ::image_writer::ImageWriter) }
  }

  /// C++ method: <span style='color: green;'>```QString QImageWriter::errorString() const```</span>
  ///
  ///
  pub fn error_string(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageWriter_errorString_to_output(self as *const ::image_writer::ImageWriter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QImageWriter::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageWriter_fileName_to_output(self as *const ::image_writer::ImageWriter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QImageWriter::format() const```</span>
  ///
  ///
  pub fn format(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageWriter_format_to_output(self as *const ::image_writer::ImageWriter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float QImageWriter::gamma() const```</span>
  ///
  ///
  pub fn gamma(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QImageWriter_gamma(self as *const ::image_writer::ImageWriter) }
  }

  /// C++ method: <span style='color: green;'>```QImageWriter::QImageWriter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::image_writer::ImageWriter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImageWriter::QImageWriter()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::image_writer::ImageWriter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImageWriter::QImageWriter(const QString& fileName)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, &::qt_core::byte_array::ByteArray)) -> ::cpp_utils::CppBox<::image_writer::ImageWriter>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QImageWriter::QImageWriter(const QString& fileName, const QByteArray& format = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::image_writer::ImageWriter>
    where Args: overloading::ImageWriterNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QImageWriter::QImageWriter(QIODevice* device, const QByteArray& format)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(device: *mut ::qt_core::io_device::IODevice,
                           format: &::qt_core::byte_array::ByteArray)
                           -> ::cpp_utils::CppBox<::image_writer::ImageWriter> {
    let ffi_result =
      ::ffi::qt_gui_c_QImageWriter_new_device_format(device, format as *const ::qt_core::byte_array::ByteArray);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```bool QImageWriter::optimizedWrite() const```</span>
  ///
  ///
  pub fn optimized_write(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageWriter_optimizedWrite(self as *const ::image_writer::ImageWriter) }
  }

  /// C++ method: <span style='color: green;'>```bool QImageWriter::progressiveScanWrite() const```</span>
  ///
  ///
  pub fn progressive_scan_write(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QImageWriter_progressiveScanWrite(self as *const ::image_writer::ImageWriter) }
  }

  /// C++ method: <span style='color: green;'>```int QImageWriter::quality() const```</span>
  ///
  ///
  pub fn quality(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QImageWriter_quality(self as *const ::image_writer::ImageWriter) }
  }

  /// C++ method: <span style='color: green;'>```void QImageWriter::setCompression(int compression)```</span>
  ///
  ///
  pub fn set_compression(&mut self, compression: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QImageWriter_setCompression(self as *mut ::image_writer::ImageWriter, compression) }
  }

  /// C++ method: <span style='color: green;'>```void QImageWriter::setDescription(const QString& description)```</span>
  ///
  ///
  pub fn set_description(&mut self, description: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QImageWriter_setDescription(self as *mut ::image_writer::ImageWriter,
                                                  description as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageWriter::setDevice(QIODevice* device)```</span>
  ///
  ///
  pub unsafe fn set_device(&mut self, device: *mut ::qt_core::io_device::IODevice) {
    ::ffi::qt_gui_c_QImageWriter_setDevice(self as *mut ::image_writer::ImageWriter, device)
  }

  /// C++ method: <span style='color: green;'>```void QImageWriter::setFileName(const QString& fileName)```</span>
  ///
  ///
  pub fn set_file_name(&mut self, file_name: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QImageWriter_setFileName(self as *mut ::image_writer::ImageWriter,
                                               file_name as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageWriter::setFormat(const QByteArray& format)```</span>
  ///
  ///
  pub fn set_format(&mut self, format: &::qt_core::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_gui_c_QImageWriter_setFormat(self as *mut ::image_writer::ImageWriter,
                                             format as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageWriter::setGamma(float gamma)```</span>
  ///
  ///
  pub fn set_gamma(&mut self, gamma: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QImageWriter_setGamma(self as *mut ::image_writer::ImageWriter, gamma) }
  }

  /// C++ method: <span style='color: green;'>```void QImageWriter::setOptimizedWrite(bool optimize)```</span>
  ///
  ///
  pub fn set_optimized_write(&mut self, optimize: bool) {
    unsafe { ::ffi::qt_gui_c_QImageWriter_setOptimizedWrite(self as *mut ::image_writer::ImageWriter, optimize) }
  }

  /// C++ method: <span style='color: green;'>```void QImageWriter::setProgressiveScanWrite(bool progressive)```</span>
  ///
  ///
  pub fn set_progressive_scan_write(&mut self, progressive: bool) {
    unsafe {
      ::ffi::qt_gui_c_QImageWriter_setProgressiveScanWrite(self as *mut ::image_writer::ImageWriter, progressive)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageWriter::setQuality(int quality)```</span>
  ///
  ///
  pub fn set_quality(&mut self, quality: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QImageWriter_setQuality(self as *mut ::image_writer::ImageWriter, quality) }
  }

  /// C++ method: <span style='color: green;'>```void QImageWriter::setSubType(const QByteArray& type)```</span>
  ///
  ///
  pub fn set_sub_type(&mut self, type_: &::qt_core::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_gui_c_QImageWriter_setSubType(self as *mut ::image_writer::ImageWriter,
                                              type_ as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QImageWriter::setText(const QString& key, const QString& text)```</span>
  ///
  ///
  pub fn set_text(&mut self, key: &::qt_core::string::String, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QImageWriter_setText(self as *mut ::image_writer::ImageWriter,
                                           key as *const ::qt_core::string::String,
                                           text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QImageWriter::subType() const```</span>
  ///
  ///
  pub fn sub_type(&self) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageWriter_subType_to_output(self as *const ::image_writer::ImageWriter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QImageWriter::supportedImageFormats()```</span>
  ///
  ///
  pub fn supported_image_formats() -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageWriter_supportedImageFormats_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QImageWriter::supportedMimeTypes()```</span>
  ///
  ///
  pub fn supported_mime_types() -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageWriter_supportedMimeTypes_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray> QImageWriter::supportedSubTypes() const```</span>
  ///
  ///
  pub fn supported_sub_types(&self) -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QImageWriter_supportedSubTypes_to_output(self as *const ::image_writer::ImageWriter,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QImageWriter::supportsOption(QImageIOHandler::ImageOption option) const```</span>
  ///
  ///
  pub fn supports_option(&self, option: &::image_io_handler::ImageOption) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QImageWriter_supportsOption(self as *const ::image_writer::ImageWriter,
                                                  option as *const ::image_io_handler::ImageOption)
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QImageWriter::tr(const char* sourceText, const char* disambiguation, int n)```</span>
  ///
  ///
  pub unsafe fn tr(source_text: *const ::libc::c_char,
                   disambiguation: *const ::libc::c_char,
                   n: ::libc::c_int)
                   -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QImageWriter_tr_to_output(source_text, disambiguation, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QString QImageWriter::trUtf8(const char* sourceText, const char* disambiguation, int n)```</span>
  ///
  ///
  pub unsafe fn tr_utf8(source_text: *const ::libc::c_char,
                        disambiguation: *const ::libc::c_char,
                        n: ::libc::c_int)
                        -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QImageWriter_trUtf8_to_output(source_text, disambiguation, n, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QImageWriter::write(const QImage& image)```</span>
  ///
  ///
  pub fn write(&mut self, image: &::image::Image) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QImageWriter_write(self as *mut ::image_writer::ImageWriter,
                                         image as *const ::image::Image)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::image_writer::ImageWriter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QImageWriter_delete
  }
}

/// C++ type: <span style='color: green;'>```QImageWriter::ImageWriterError```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ImageWriterError {
  /// C++ enum variant: <span style='color: green;'>```UnknownError = 0```</span>
  Unknown = 0,
  /// C++ enum variant: <span style='color: green;'>```DeviceError = 1```</span>
  Device = 1,
  /// C++ enum variant: <span style='color: green;'>```UnsupportedFormatError = 2```</span>
  UnsupportedFormat = 2,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ImageWriter::new](../struct.ImageWriter.html#method.new) method.
  pub trait ImageWriterNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::image_writer::ImageWriter>;
  }
  impl<'a> ImageWriterNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::image_writer::ImageWriter> {
      let file_name = self;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QImageWriter_new_fileName(file_name as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> ImageWriterNewArgs for (&'a ::qt_core::string::String, &'a ::qt_core::byte_array::ByteArray) {
    fn exec(self) -> ::cpp_utils::CppBox<::image_writer::ImageWriter> {
      let file_name = self.0;
      let format = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QImageWriter_new_fileName_format(file_name as *const ::qt_core::string::String,
                                                           format as *const ::qt_core::byte_array::ByteArray)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl ImageWriterNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::image_writer::ImageWriter> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QImageWriter_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
