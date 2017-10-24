/// C++ type: <span style='color: green;'>```QPictureIO```</span>
#[repr(C)]
pub struct PictureIO([u8; ::type_sizes::QT_GUI_PICTURE_IO_PICTURE_I_O]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PictureIO {
  unsafe fn new_uninitialized() -> PictureIO {
    PictureIO(::std::mem::uninitialized())
  }
}

impl PictureIO {
  /// C++ method: <span style='color: green;'>```static void QPictureIO::defineIOHandler(const char* format, const char* header, const char* flags, void (*FN_PTR)(QPictureIO*) read_picture, void (*FN_PTR)(QPictureIO*) write_picture)```</span>
  ///
  ///
  pub unsafe fn define_io_handler(format: *const ::libc::c_char,
                                  header: *const ::libc::c_char,
                                  flags: *const ::libc::c_char,
                                  read_picture: extern "C" fn(*mut ::picture_io::PictureIO),
                                  write_picture: extern "C" fn(*mut ::picture_io::PictureIO)) {
    ::ffi::qt_gui_c_QPictureIO_defineIOHandler(format, header, flags, read_picture, write_picture)
  }

  /// C++ method: <span style='color: green;'>```QString QPictureIO::description() const```</span>
  ///
  ///
  pub fn description(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPictureIO_description_to_output(self as *const ::picture_io::PictureIO, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QPictureIO::fileName() const```</span>
  ///
  ///
  pub fn file_name(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPictureIO_fileName_to_output(self as *const ::picture_io::PictureIO, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const char* QPictureIO::format() const```</span>
  ///
  ///
  pub fn format(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_gui_c_QPictureIO_format(self as *const ::picture_io::PictureIO) }
  }

  /// C++ method: <span style='color: green;'>```float QPictureIO::gamma() const```</span>
  ///
  ///
  pub fn gamma(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QPictureIO_gamma(self as *const ::picture_io::PictureIO) }
  }

  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QPictureIO::inputFormats()```</span>
  ///
  ///
  pub fn input_formats() -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPictureIO_inputFormats_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QIODevice* QPictureIO::ioDevice() const```</span>
  ///
  ///
  pub fn io_device(&self) -> *mut ::qt_core::io_device::IODevice {
    unsafe { ::ffi::qt_gui_c_QPictureIO_ioDevice(self as *const ::picture_io::PictureIO) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPictureIO::QPictureIO()```</span>
  ///
  ///
  pub fn new() -> ::picture_io::PictureIO {
    {
      let mut object: ::picture_io::PictureIO =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPictureIO_constructor_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPictureIO::QPictureIO```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe((*mut ::qt_core::io_device::IODevice, *const ::libc::c_char)) -> ::picture_io::PictureIO```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPictureIO::QPictureIO(QIODevice* ioDevice, const char* format)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((&::qt_core::string::String, *const ::libc::c_char)) -> ::picture_io::PictureIO```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPictureIO::QPictureIO(const QString& fileName, const char* format)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::picture_io::PictureIO
    where Args: overloading::PictureIONewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QList<QByteArray> QPictureIO::outputFormats()```</span>
  ///
  ///
  pub fn output_formats() -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPictureIO_outputFormats_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const char* QPictureIO::parameters() const```</span>
  ///
  ///
  pub fn parameters(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_gui_c_QPictureIO_parameters(self as *const ::picture_io::PictureIO) }
  }

  /// C++ method: <span style='color: green;'>```const QPicture& QPictureIO::picture() const```</span>
  ///
  ///
  pub fn picture<'l0>(&'l0 self) -> &'l0 ::picture::Picture {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPictureIO_picture(self as *const ::picture_io::PictureIO) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QByteArray QPictureIO::pictureFormat(const QString& fileName)```</span>
  ///
  ///
  pub fn picture_format(file_name: &::qt_core::string::String) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPictureIO_pictureFormat_to_output_fileName(file_name as *const ::qt_core::string::String,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QByteArray QPictureIO::pictureFormat(QIODevice* arg1)```</span>
  ///
  ///
  pub unsafe fn picture_format_unsafe(arg1: *mut ::qt_core::io_device::IODevice) -> ::qt_core::byte_array::ByteArray {
    {
      let mut object: ::qt_core::byte_array::ByteArray =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QPictureIO_pictureFormat_to_output_arg1(arg1, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QPictureIO::quality() const```</span>
  ///
  ///
  pub fn quality(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPictureIO_quality(self as *const ::picture_io::PictureIO) }
  }

  /// C++ method: <span style='color: green;'>```bool QPictureIO::read()```</span>
  ///
  ///
  pub fn read(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPictureIO_read(self as *mut ::picture_io::PictureIO) }
  }

  /// C++ method: <span style='color: green;'>```void QPictureIO::setDescription(const QString& arg1)```</span>
  ///
  ///
  pub fn set_description(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QPictureIO_setDescription(self as *mut ::picture_io::PictureIO,
                                                arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPictureIO::setFileName(const QString& arg1)```</span>
  ///
  ///
  pub fn set_file_name(&mut self, arg1: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QPictureIO_setFileName(self as *mut ::picture_io::PictureIO,
                                             arg1 as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPictureIO::setFormat(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn set_format(&mut self, arg1: *const ::libc::c_char) {
    ::ffi::qt_gui_c_QPictureIO_setFormat(self as *mut ::picture_io::PictureIO, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QPictureIO::setGamma(float arg1)```</span>
  ///
  ///
  pub fn set_gamma(&mut self, arg1: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QPictureIO_setGamma(self as *mut ::picture_io::PictureIO, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QPictureIO::setIODevice(QIODevice* arg1)```</span>
  ///
  ///
  pub unsafe fn set_io_device(&mut self, arg1: *mut ::qt_core::io_device::IODevice) {
    ::ffi::qt_gui_c_QPictureIO_setIODevice(self as *mut ::picture_io::PictureIO, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QPictureIO::setParameters(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn set_parameters(&mut self, arg1: *const ::libc::c_char) {
    ::ffi::qt_gui_c_QPictureIO_setParameters(self as *mut ::picture_io::PictureIO, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QPictureIO::setPicture(const QPicture& arg1)```</span>
  ///
  ///
  pub fn set_picture(&mut self, arg1: &::picture::Picture) {
    unsafe {
      ::ffi::qt_gui_c_QPictureIO_setPicture(self as *mut ::picture_io::PictureIO,
                                            arg1 as *const ::picture::Picture)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPictureIO::setQuality(int arg1)```</span>
  ///
  ///
  pub fn set_quality(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QPictureIO_setQuality(self as *mut ::picture_io::PictureIO, arg1) }
  }

  /// C++ method: <span style='color: green;'>```void QPictureIO::setStatus(int arg1)```</span>
  ///
  ///
  pub fn set_status(&mut self, arg1: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QPictureIO_setStatus(self as *mut ::picture_io::PictureIO, arg1) }
  }

  /// C++ method: <span style='color: green;'>```int QPictureIO::status() const```</span>
  ///
  ///
  pub fn status(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPictureIO_status(self as *const ::picture_io::PictureIO) }
  }

  /// C++ method: <span style='color: green;'>```bool QPictureIO::write()```</span>
  ///
  ///
  pub fn write(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPictureIO_write(self as *mut ::picture_io::PictureIO) }
  }
}

impl Drop for ::picture_io::PictureIO {
  /// C++ method: <span style='color: green;'>```[destructor] void QPictureIO::~QPictureIO()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPictureIO_destructor(self as *mut ::picture_io::PictureIO) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PictureIO::new_unsafe](../struct.PictureIO.html#method.new_unsafe) method.
  pub trait PictureIONewUnsafeArgs {
    unsafe fn exec(self) -> ::picture_io::PictureIO;
  }
  impl<'a> PictureIONewUnsafeArgs for (&'a ::qt_core::string::String, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::picture_io::PictureIO {
      let file_name = self.0;
      let format = self.1;
      {
        let mut object: ::picture_io::PictureIO =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_gui_c_QPictureIO_constructor_fileName_format(file_name as *const ::qt_core::string::String,
                                                               format,
                                                               &mut object);
        object
      }
    }
  }
  impl PictureIONewUnsafeArgs for (*mut ::qt_core::io_device::IODevice, *const ::libc::c_char) {
    unsafe fn exec(self) -> ::picture_io::PictureIO {
      let io_device = self.0;
      let format = self.1;
      {
        let mut object: ::picture_io::PictureIO =
          ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_gui_c_QPictureIO_constructor_ioDevice_format(io_device, format, &mut object);
        object
      }
    }
  }
}
