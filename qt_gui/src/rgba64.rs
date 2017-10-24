/// C++ type: <span style='color: green;'>```QRgba64```</span>
#[repr(C)]
pub struct Rgba64([u8; ::type_sizes::QT_GUI_RGBA_64_RGBA_64]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Rgba64 {
  unsafe fn new_uninitialized() -> Rgba64 {
    Rgba64(::std::mem::uninitialized())
  }
}

impl Rgba64 {
  /// C++ method: <span style='color: green;'>```quint16 QRgba64::alpha() const```</span>
  ///
  ///
  pub fn alpha(&self) -> u16 {
    unsafe { ::ffi::qt_gui_c_QRgba64_alpha(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```quint8 QRgba64::alpha8() const```</span>
  ///
  ///
  pub fn alpha8(&self) -> u8 {
    unsafe { ::ffi::qt_gui_c_QRgba64_alpha8(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```quint64 QRgba64::operator unsigned long long() const```</span>
  ///
  ///
  pub fn as_unsigned_long_long(&self) -> u64 {
    unsafe { ::ffi::qt_gui_c_QRgba64_convert_to_unsigned_long_long(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```quint16 QRgba64::blue() const```</span>
  ///
  ///
  pub fn blue(&self) -> u16 {
    unsafe { ::ffi::qt_gui_c_QRgba64_blue(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```quint8 QRgba64::blue8() const```</span>
  ///
  ///
  pub fn blue8(&self) -> u8 {
    unsafe { ::ffi::qt_gui_c_QRgba64_blue8(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```static QRgba64 QRgba64::fromArgb32(unsigned int rgb)```</span>
  ///
  ///
  pub fn from_argb32(rgb: ::libc::c_uint) -> ::rgba64::Rgba64 {
    {
      let mut object: ::rgba64::Rgba64 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRgba64_fromArgb32_to_output(rgb, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QRgba64 QRgba64::fromRgba(quint8 red, quint8 green, quint8 blue, quint8 alpha)```</span>
  ///
  ///
  pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: u8) -> ::rgba64::Rgba64 {
    {
      let mut object: ::rgba64::Rgba64 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRgba64_fromRgba_to_output(red, green, blue, alpha, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRgba64::fromRgba64```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_rgba64((u16, u16, u16, u16)) -> ::rgba64::Rgba64```<br>
  /// C++ method: <span style='color: green;'>```static QRgba64 QRgba64::fromRgba64(quint16 red, quint16 green, quint16 blue, quint16 alpha)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_rgba64(u64) -> ::rgba64::Rgba64```<br>
  /// C++ method: <span style='color: green;'>```static QRgba64 QRgba64::fromRgba64(quint64 c)```</span>
  ///
  ///
  pub fn from_rgba64<Args>(args: Args) -> ::rgba64::Rgba64
    where Args: overloading::Rgba64FromRgba64Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```quint16 QRgba64::green() const```</span>
  ///
  ///
  pub fn green(&self) -> u16 {
    unsafe { ::ffi::qt_gui_c_QRgba64_green(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```quint8 QRgba64::green8() const```</span>
  ///
  ///
  pub fn green8(&self) -> u8 {
    unsafe { ::ffi::qt_gui_c_QRgba64_green8(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```bool QRgba64::isOpaque() const```</span>
  ///
  ///
  pub fn is_opaque(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QRgba64_isOpaque(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```bool QRgba64::isTransparent() const```</span>
  ///
  ///
  pub fn is_transparent(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QRgba64_isTransparent(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QRgba64::QRgba64()```</span>
  ///
  ///
  pub fn new() -> ::rgba64::Rgba64 {
    {
      let mut object: ::rgba64::Rgba64 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRgba64_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRgba64 QRgba64::operator=(quint64 _rgba)```</span>
  ///
  ///
  pub fn op_assign(&mut self, rgba: u64) -> ::rgba64::Rgba64 {
    {
      let mut object: ::rgba64::Rgba64 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRgba64_operator_assign_to_output(self as *mut ::rgba64::Rgba64, rgba, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRgba64 QRgba64::premultiplied() const```</span>
  ///
  ///
  pub fn premultiplied(&self) -> ::rgba64::Rgba64 {
    {
      let mut object: ::rgba64::Rgba64 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRgba64_premultiplied_to_output(self as *const ::rgba64::Rgba64, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```quint16 QRgba64::red() const```</span>
  ///
  ///
  pub fn red(&self) -> u16 {
    unsafe { ::ffi::qt_gui_c_QRgba64_red(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```quint8 QRgba64::red8() const```</span>
  ///
  ///
  pub fn red8(&self) -> u8 {
    unsafe { ::ffi::qt_gui_c_QRgba64_red8(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```void QRgba64::setAlpha(quint16 _alpha)```</span>
  ///
  ///
  pub fn set_alpha(&mut self, alpha: u16) {
    unsafe { ::ffi::qt_gui_c_QRgba64_setAlpha(self as *mut ::rgba64::Rgba64, alpha) }
  }

  /// C++ method: <span style='color: green;'>```void QRgba64::setBlue(quint16 _blue)```</span>
  ///
  ///
  pub fn set_blue(&mut self, blue: u16) {
    unsafe { ::ffi::qt_gui_c_QRgba64_setBlue(self as *mut ::rgba64::Rgba64, blue) }
  }

  /// C++ method: <span style='color: green;'>```void QRgba64::setGreen(quint16 _green)```</span>
  ///
  ///
  pub fn set_green(&mut self, green: u16) {
    unsafe { ::ffi::qt_gui_c_QRgba64_setGreen(self as *mut ::rgba64::Rgba64, green) }
  }

  /// C++ method: <span style='color: green;'>```void QRgba64::setRed(quint16 _red)```</span>
  ///
  ///
  pub fn set_red(&mut self, red: u16) {
    unsafe { ::ffi::qt_gui_c_QRgba64_setRed(self as *mut ::rgba64::Rgba64, red) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QRgba64::toArgb32() const```</span>
  ///
  ///
  pub fn to_argb32(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_gui_c_QRgba64_toArgb32(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```unsigned short QRgba64::toRgb16() const```</span>
  ///
  ///
  pub fn to_rgb16(&self) -> ::libc::c_ushort {
    unsafe { ::ffi::qt_gui_c_QRgba64_toRgb16(self as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```QRgba64 QRgba64::unpremultiplied() const```</span>
  ///
  ///
  pub fn unpremultiplied(&self) -> ::rgba64::Rgba64 {
    {
      let mut object: ::rgba64::Rgba64 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QRgba64_unpremultiplied_to_output(self as *const ::rgba64::Rgba64, &mut object);
      }
      object
    }
  }
}

impl Drop for ::rgba64::Rgba64 {
  /// C++ method: <span style='color: green;'>```[destructor] void QRgba64::~QRgba64()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QRgba64_destructor(self as *mut ::rgba64::Rgba64) }
  }
}

/// C++ method: <span style='color: green;'>```unsigned int qAlpha(QRgba64 rgb)```</span>
///
///
pub fn alpha(rgb: &::rgba64::Rgba64) -> ::libc::c_uint {
  unsafe { ::ffi::qt_gui_c_QRgba64_G_qAlpha(rgb as *const ::rgba64::Rgba64) }
}

/// C++ method: <span style='color: green;'>```unsigned int qBlue(QRgba64 rgb)```</span>
///
///
pub fn blue(rgb: &::rgba64::Rgba64) -> ::libc::c_uint {
  unsafe { ::ffi::qt_gui_c_QRgba64_G_qBlue(rgb as *const ::rgba64::Rgba64) }
}

/// C++ method: <span style='color: green;'>```unsigned int qGreen(QRgba64 rgb)```</span>
///
///
pub fn green(rgb: &::rgba64::Rgba64) -> ::libc::c_uint {
  unsafe { ::ffi::qt_gui_c_QRgba64_G_qGreen(rgb as *const ::rgba64::Rgba64) }
}

/// C++ method: <span style='color: green;'>```QRgba64 qPremultiply(QRgba64 c)```</span>
///
///
pub fn premultiply(c: &::rgba64::Rgba64) -> ::rgba64::Rgba64 {
  {
    let mut object: ::rgba64::Rgba64 = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QRgba64_G_qPremultiply_to_output(c as *const ::rgba64::Rgba64, &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```unsigned int qRed(QRgba64 rgb)```</span>
///
///
pub fn red(rgb: &::rgba64::Rgba64) -> ::libc::c_uint {
  unsafe { ::ffi::qt_gui_c_QRgba64_G_qRed(rgb as *const ::rgba64::Rgba64) }
}

/// C++ method: <span style='color: green;'>```qRgba64```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn rgba64((u16, u16, u16, u16)) -> ::rgba64::Rgba64```<br>
/// C++ method: <span style='color: green;'>```QRgba64 qRgba64(quint16 r, quint16 g, quint16 b, quint16 a)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn rgba64(u64) -> ::rgba64::Rgba64```<br>
/// C++ method: <span style='color: green;'>```QRgba64 qRgba64(quint64 c)```</span>
///
///
pub fn rgba64<Args>(args: Args) -> ::rgba64::Rgba64
  where Args: overloading::Rgba64Args
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QRgba64 qUnpremultiply(QRgba64 c)```</span>
///
///
pub fn unpremultiply(c: &::rgba64::Rgba64) -> ::rgba64::Rgba64 {
  {
    let mut object: ::rgba64::Rgba64 = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QRgba64_G_qUnpremultiply_to_output(c as *const ::rgba64::Rgba64, &mut object);
    }
    object
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Rgba64::from_rgba64](../struct.Rgba64.html#method.from_rgba64) method.
  pub trait Rgba64FromRgba64Args {
    fn exec(self) -> ::rgba64::Rgba64;
  }
  impl Rgba64FromRgba64Args for u64 {
    fn exec(self) -> ::rgba64::Rgba64 {
      let c = self;
      {
        let mut object: ::rgba64::Rgba64 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRgba64_fromRgba64_to_output_c(c, &mut object);
        }
        object
      }
    }
  }
  impl Rgba64FromRgba64Args for (u16, u16, u16, u16) {
    fn exec(self) -> ::rgba64::Rgba64 {
      let red = self.0;
      let green = self.1;
      let blue = self.2;
      let alpha = self.3;
      {
        let mut object: ::rgba64::Rgba64 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRgba64_fromRgba64_to_output_red_green_blue_alpha(red, green, blue, alpha, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [rgba64](../fn.rgba64.html) method.
  pub trait Rgba64Args {
    fn exec(self) -> ::rgba64::Rgba64;
  }
  impl Rgba64Args for u64 {
    fn exec(self) -> ::rgba64::Rgba64 {
      let c = self;
      {
        let mut object: ::rgba64::Rgba64 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRgba64_G_qRgba64_to_output_c(c, &mut object);
        }
        object
      }
    }
  }
  impl Rgba64Args for (u16, u16, u16, u16) {
    fn exec(self) -> ::rgba64::Rgba64 {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      {
        let mut object: ::rgba64::Rgba64 =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QRgba64_G_qRgba64_to_output_r_g_b_a(r, g, b, a, &mut object);
        }
        object
      }
    }
  }
}
