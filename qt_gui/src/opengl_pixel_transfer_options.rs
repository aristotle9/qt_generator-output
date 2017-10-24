/// C++ type: <span style='color: green;'>```QOpenGLPixelTransferOptions```</span>
#[repr(C)]
pub struct OpenGLPixelTransferOptions(u8);

impl OpenGLPixelTransferOptions {
  /// C++ method: <span style='color: green;'>```int QOpenGLPixelTransferOptions::alignment() const```</span>
  ///
  ///
  pub fn alignment(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_alignment(self as *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLPixelTransferOptions::imageHeight() const```</span>
  ///
  ///
  pub fn image_height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_imageHeight(self as *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLPixelTransferOptions::isLeastSignificantBitFirst() const```</span>
  ///
  ///
  pub fn is_least_significant_bit_first(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_isLeastSignificantBitFirst(self as *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) }
  }

  /// C++ method: <span style='color: green;'>```bool QOpenGLPixelTransferOptions::isSwapBytesEnabled() const```</span>
  ///
  ///
  pub fn is_swap_bytes_enabled(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_isSwapBytesEnabled(self as *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLPixelTransferOptions::QOpenGLPixelTransferOptions```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::opengl_pixel_transfer_options::OpenGLPixelTransferOptions>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLPixelTransferOptions::QOpenGLPixelTransferOptions()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) -> ::cpp_utils::CppBox<::opengl_pixel_transfer_options::OpenGLPixelTransferOptions>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QOpenGLPixelTransferOptions::QOpenGLPixelTransferOptions(const QOpenGLPixelTransferOptions& arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::opengl_pixel_transfer_options::OpenGLPixelTransferOptions>
    where Args: overloading::OpenGLPixelTransferOptionsNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QOpenGLPixelTransferOptions& QOpenGLPixelTransferOptions::operator=(const QOpenGLPixelTransferOptions& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             arg1: &'l1 ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions)
                             -> &'l0 mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_operator_assign(self as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions, arg1 as *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLPixelTransferOptions::rowLength() const```</span>
  ///
  ///
  pub fn row_length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_rowLength(self as *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPixelTransferOptions::setAlignment(int alignment)```</span>
  ///
  ///
  pub fn set_alignment(&mut self, alignment: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_setAlignment(self as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions, alignment) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPixelTransferOptions::setImageHeight(int imageHeight)```</span>
  ///
  ///
  pub fn set_image_height(&mut self, image_height: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_setImageHeight(self as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions, image_height) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPixelTransferOptions::setLeastSignificantByteFirst(bool lsbFirst)```</span>
  ///
  ///
  pub fn set_least_significant_byte_first(&mut self, lsb_first: bool) {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_setLeastSignificantByteFirst(self as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions, lsb_first) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPixelTransferOptions::setRowLength(int rowLength)```</span>
  ///
  ///
  pub fn set_row_length(&mut self, row_length: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_setRowLength(self as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions, row_length) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPixelTransferOptions::setSkipImages(int skipImages)```</span>
  ///
  ///
  pub fn set_skip_images(&mut self, skip_images: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_setSkipImages(self as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions, skip_images) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPixelTransferOptions::setSkipPixels(int skipPixels)```</span>
  ///
  ///
  pub fn set_skip_pixels(&mut self, skip_pixels: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_setSkipPixels(self as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions, skip_pixels) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPixelTransferOptions::setSkipRows(int skipRows)```</span>
  ///
  ///
  pub fn set_skip_rows(&mut self, skip_rows: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_setSkipRows(self as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions, skip_rows) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPixelTransferOptions::setSwapBytesEnabled(bool swapBytes)```</span>
  ///
  ///
  pub fn set_swap_bytes_enabled(&mut self, swap_bytes: bool) {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_setSwapBytesEnabled(self as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions, swap_bytes) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLPixelTransferOptions::skipImages() const```</span>
  ///
  ///
  pub fn skip_images(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_skipImages(self as *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLPixelTransferOptions::skipPixels() const```</span>
  ///
  ///
  pub fn skip_pixels(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_skipPixels(self as *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) }
  }

  /// C++ method: <span style='color: green;'>```int QOpenGLPixelTransferOptions::skipRows() const```</span>
  ///
  ///
  pub fn skip_rows(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_skipRows(self as *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) }
  }

  /// C++ method: <span style='color: green;'>```void QOpenGLPixelTransferOptions::swap(QOpenGLPixelTransferOptions& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {
    unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_swap(self as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions, other as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) }
  }
}

impl ::cpp_utils::CppDeletable for ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_delete
  }
}

/// C++ method: <span style='color: green;'>```void swap(QOpenGLPixelTransferOptions& value1, QOpenGLPixelTransferOptions& value2)```</span>
///
///
pub fn swap(value1: &mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions,
            value2: &mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) {
  unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_G_swap(value1 as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions, value2 as *mut ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [OpenGLPixelTransferOptions::new](../struct.OpenGLPixelTransferOptions.html#method.new) method.
  pub trait OpenGLPixelTransferOptionsNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_pixel_transfer_options::OpenGLPixelTransferOptions>;
  }
  impl<'a> OpenGLPixelTransferOptionsNewArgs for &'a ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_pixel_transfer_options::OpenGLPixelTransferOptions> {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_new_arg1(arg1 as *const ::opengl_pixel_transfer_options::OpenGLPixelTransferOptions) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl OpenGLPixelTransferOptionsNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::opengl_pixel_transfer_options::OpenGLPixelTransferOptions> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QOpenGLPixelTransferOptions_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
