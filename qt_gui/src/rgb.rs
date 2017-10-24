/// C++ method: <span style='color: green;'>```int qAlpha(unsigned int rgb)```</span>
///
///
pub fn alpha(rgb: ::libc::c_uint) -> ::libc::c_int {
  unsafe { ::ffi::qt_gui_c_QRgb_G_qAlpha(rgb) }
}

/// C++ method: <span style='color: green;'>```int qBlue(unsigned int rgb)```</span>
///
///
pub fn blue(rgb: ::libc::c_uint) -> ::libc::c_int {
  unsafe { ::ffi::qt_gui_c_QRgb_G_qBlue(rgb) }
}

/// C++ method: <span style='color: green;'>```qGray```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn gray((::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
/// C++ method: <span style='color: green;'>```int qGray(int r, int g, int b)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn gray(::libc::c_uint) -> ::libc::c_int```<br>
/// C++ method: <span style='color: green;'>```int qGray(unsigned int rgb)```</span>
///
///
pub fn gray<Args>(args: Args) -> ::libc::c_int
  where Args: overloading::GrayArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```int qGreen(unsigned int rgb)```</span>
///
///
pub fn green(rgb: ::libc::c_uint) -> ::libc::c_int {
  unsafe { ::ffi::qt_gui_c_QRgb_G_qGreen(rgb) }
}

/// C++ method: <span style='color: green;'>```bool qIsGray(unsigned int rgb)```</span>
///
///
pub fn is_gray(rgb: ::libc::c_uint) -> bool {
  unsafe { ::ffi::qt_gui_c_QRgb_G_qIsGray(rgb) }
}

/// C++ method: <span style='color: green;'>```unsigned int qPremultiply(unsigned int x)```</span>
///
///
pub fn premultiply(x: ::libc::c_uint) -> ::libc::c_uint {
  unsafe { ::ffi::qt_gui_c_QRgb_G_qPremultiply(x) }
}

/// C++ method: <span style='color: green;'>```int qRed(unsigned int rgb)```</span>
///
///
pub fn red(rgb: ::libc::c_uint) -> ::libc::c_int {
  unsafe { ::ffi::qt_gui_c_QRgb_G_qRed(rgb) }
}

/// C++ method: <span style='color: green;'>```unsigned int qRgb(int r, int g, int b)```</span>
///
///
pub fn rgb(r: ::libc::c_int, g: ::libc::c_int, b: ::libc::c_int) -> ::libc::c_uint {
  unsafe { ::ffi::qt_gui_c_QRgb_G_qRgb(r, g, b) }
}

/// C++ method: <span style='color: green;'>```unsigned int qRgba(int r, int g, int b, int a)```</span>
///
///
pub fn rgba(r: ::libc::c_int, g: ::libc::c_int, b: ::libc::c_int, a: ::libc::c_int) -> ::libc::c_uint {
  unsafe { ::ffi::qt_gui_c_QRgb_G_qRgba(r, g, b, a) }
}

/// C++ method: <span style='color: green;'>```unsigned int qUnpremultiply(unsigned int p)```</span>
///
///
pub fn unpremultiply(p: ::libc::c_uint) -> ::libc::c_uint {
  unsafe { ::ffi::qt_gui_c_QRgb_G_qUnpremultiply(p) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [gray](../fn.gray.html) method.
  pub trait GrayArgs {
    fn exec(self) -> ::libc::c_int;
  }
  impl GrayArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::libc::c_int {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      unsafe { ::ffi::qt_gui_c_QRgb_G_qGray_r_g_b(r, g, b) }
    }
  }
  impl GrayArgs for ::libc::c_uint {
    fn exec(self) -> ::libc::c_int {
      let rgb = self;
      unsafe { ::ffi::qt_gui_c_QRgb_G_qGray_rgb(rgb) }
    }
  }
}
