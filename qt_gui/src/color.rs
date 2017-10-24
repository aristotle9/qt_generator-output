/// C++ type: <span style='color: green;'>```QColor```</span>
#[repr(C)]
pub struct Color([u8; ::type_sizes::QT_GUI_COLOR_COLOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Color {
  unsafe fn new_uninitialized() -> Color {
    Color(::std::mem::uninitialized())
  }
}

impl Color {
  /// C++ method: <span style='color: green;'>```int QColor::alpha() const```</span>
  ///
  ///
  pub fn alpha(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_alpha(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::alphaF() const```</span>
  ///
  ///
  pub fn alpha_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_alphaF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QColor::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QColor_convert_to_QVariant_to_output(self as *const ::color::Color, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QColor::black() const```</span>
  ///
  ///
  pub fn black(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_black(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::blackF() const```</span>
  ///
  ///
  pub fn black_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_blackF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```int QColor::blue() const```</span>
  ///
  ///
  pub fn blue(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_blue(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::blueF() const```</span>
  ///
  ///
  pub fn blue_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_blueF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```static QStringList QColor::colorNames()```</span>
  ///
  ///
  pub fn color_names() -> ::qt_core::string_list::StringList {
    {
      let mut object: ::qt_core::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QColor_colorNames_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QColor::convertTo(QColor::Spec colorSpec) const```</span>
  ///
  ///
  pub fn convert_to(&self, color_spec: ::color::Spec) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QColor_convertTo_to_output(self as *const ::color::Color, color_spec, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QColor::cyan() const```</span>
  ///
  ///
  pub fn cyan(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_cyan(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::cyanF() const```</span>
  ///
  ///
  pub fn cyan_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_cyanF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```QColor::dark```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn dark(&self, ()) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QColor::dark() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn dark(&self, ::libc::c_int) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QColor::dark(int f = ?) const```</span>
  ///
  ///
  pub fn dark<'largs, Args>(&'largs self, args: Args) -> ::color::Color
    where Args: overloading::ColorDarkArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::darker```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn darker(&self, ()) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QColor::darker() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn darker(&self, ::libc::c_int) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QColor::darker(int f = ?) const```</span>
  ///
  ///
  pub fn darker<'largs, Args>(&'largs self, args: Args) -> ::color::Color
    where Args: overloading::ColorDarkerArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::fromCmyk```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_cmyk((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromCmyk(int c, int m, int y, int k)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_cmyk((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromCmyk(int c, int m, int y, int k, int a = ?)```</span>
  ///
  ///
  pub fn from_cmyk<Args>(args: Args) -> ::color::Color
    where Args: overloading::ColorFromCmykArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColor::fromCmykF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_cmyk_f((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromCmykF(double c, double m, double y, double k)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_cmyk_f((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromCmykF(double c, double m, double y, double k, double a = ?)```</span>
  ///
  ///
  pub fn from_cmyk_f<Args>(args: Args) -> ::color::Color
    where Args: overloading::ColorFromCmykFArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColor::fromHsl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_hsl((::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromHsl(int h, int s, int l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_hsl((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromHsl(int h, int s, int l, int a = ?)```</span>
  ///
  ///
  pub fn from_hsl<Args>(args: Args) -> ::color::Color
    where Args: overloading::ColorFromHslArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColor::fromHslF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_hsl_f((::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromHslF(double h, double s, double l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_hsl_f((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromHslF(double h, double s, double l, double a = ?)```</span>
  ///
  ///
  pub fn from_hsl_f<Args>(args: Args) -> ::color::Color
    where Args: overloading::ColorFromHslFArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColor::fromHsv```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_hsv((::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromHsv(int h, int s, int v)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_hsv((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromHsv(int h, int s, int v, int a = ?)```</span>
  ///
  ///
  pub fn from_hsv<Args>(args: Args) -> ::color::Color
    where Args: overloading::ColorFromHsvArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColor::fromHsvF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_hsv_f((::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromHsvF(double h, double s, double v)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_hsv_f((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromHsvF(double h, double s, double v, double a = ?)```</span>
  ///
  ///
  pub fn from_hsv_f<Args>(args: Args) -> ::color::Color
    where Args: overloading::ColorFromHsvFArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColor::fromRgb```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_rgb((::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromRgb(int r, int g, int b)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_rgb((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromRgb(int r, int g, int b, int a = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn from_rgb(::libc::c_uint) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromRgb(unsigned int rgb)```</span>
  ///
  ///
  pub fn from_rgb<Args>(args: Args) -> ::color::Color
    where Args: overloading::ColorFromRgbArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColor::fromRgbF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_rgb_f((::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromRgbF(double r, double g, double b)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_rgb_f((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromRgbF(double r, double g, double b, double a = ?)```</span>
  ///
  ///
  pub fn from_rgb_f<Args>(args: Args) -> ::color::Color
    where Args: overloading::ColorFromRgbFArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromRgba(unsigned int rgba)```</span>
  ///
  ///
  pub fn from_rgba(rgba: ::libc::c_uint) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QColor_fromRgba_to_output(rgba, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor::fromRgba64```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_rgba64(&::rgba64::Rgba64) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromRgba64(QRgba64 rgba)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_rgba64((::libc::c_ushort, ::libc::c_ushort, ::libc::c_ushort)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromRgba64(unsigned short r, unsigned short g, unsigned short b)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn from_rgba64((::libc::c_ushort, ::libc::c_ushort, ::libc::c_ushort, ::libc::c_ushort)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```static QColor QColor::fromRgba64(unsigned short r, unsigned short g, unsigned short b, unsigned short a = ?)```</span>
  ///
  ///
  pub fn from_rgba64<Args>(args: Args) -> ::color::Color
    where Args: overloading::ColorFromRgba64Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColor::getCmyk```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_cmyk(&mut self, (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getCmyk(int* c, int* m, int* y, int* k)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_cmyk(&mut self, (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getCmyk(int* c, int* m, int* y, int* k, int* a = ?)```</span>
  ///
  ///
  pub unsafe fn get_cmyk<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorGetCmykArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::getCmykF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_cmyk_f(&mut self, (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getCmykF(double* c, double* m, double* y, double* k)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_cmyk_f(&mut self, (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getCmykF(double* c, double* m, double* y, double* k, double* a = ?)```</span>
  ///
  ///
  pub unsafe fn get_cmyk_f<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorGetCmykFArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::getHsl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_hsl(&self, (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getHsl(int* h, int* s, int* l) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_hsl(&self, (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getHsl(int* h, int* s, int* l, int* a = ?) const```</span>
  ///
  ///
  pub unsafe fn get_hsl<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::ColorGetHslArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::getHslF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_hsl_f(&self, (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getHslF(double* h, double* s, double* l) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_hsl_f(&self, (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getHslF(double* h, double* s, double* l, double* a = ?) const```</span>
  ///
  ///
  pub unsafe fn get_hsl_f<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::ColorGetHslFArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::getHsv```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_hsv(&self, (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getHsv(int* h, int* s, int* v) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_hsv(&self, (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getHsv(int* h, int* s, int* v, int* a = ?) const```</span>
  ///
  ///
  pub unsafe fn get_hsv<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::ColorGetHsvArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::getHsvF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_hsv_f(&self, (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getHsvF(double* h, double* s, double* v) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_hsv_f(&self, (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getHsvF(double* h, double* s, double* v, double* a = ?) const```</span>
  ///
  ///
  pub unsafe fn get_hsv_f<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::ColorGetHsvFArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::getRgb```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_rgb(&self, (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getRgb(int* r, int* g, int* b) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_rgb(&self, (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getRgb(int* r, int* g, int* b, int* a = ?) const```</span>
  ///
  ///
  pub unsafe fn get_rgb<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::ColorGetRgbArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::getRgbF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_rgb_f(&self, (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getRgbF(double* r, double* g, double* b) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_rgb_f(&self, (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::getRgbF(double* r, double* g, double* b, double* a = ?) const```</span>
  ///
  ///
  pub unsafe fn get_rgb_f<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::ColorGetRgbFArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QColor::green() const```</span>
  ///
  ///
  pub fn green(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_green(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::greenF() const```</span>
  ///
  ///
  pub fn green_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_greenF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```int QColor::hslHue() const```</span>
  ///
  ///
  pub fn hsl_hue(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_hslHue(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::hslHueF() const```</span>
  ///
  ///
  pub fn hsl_hue_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_hslHueF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```int QColor::hslSaturation() const```</span>
  ///
  ///
  pub fn hsl_saturation(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_hslSaturation(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::hslSaturationF() const```</span>
  ///
  ///
  pub fn hsl_saturation_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_hslSaturationF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```int QColor::hsvHue() const```</span>
  ///
  ///
  pub fn hsv_hue(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_hsvHue(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::hsvHueF() const```</span>
  ///
  ///
  pub fn hsv_hue_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_hsvHueF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```int QColor::hsvSaturation() const```</span>
  ///
  ///
  pub fn hsv_saturation(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_hsvSaturation(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::hsvSaturationF() const```</span>
  ///
  ///
  pub fn hsv_saturation_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_hsvSaturationF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```int QColor::hue() const```</span>
  ///
  ///
  pub fn hue(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_hue(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::hueF() const```</span>
  ///
  ///
  pub fn hue_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_hueF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```bool QColor::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QColor_isValid(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```QColor::isValidColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn is_valid_color(&::qt_core::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QColor::isValidColor(QLatin1String arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn is_valid_color(&::qt_core::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```static bool QColor::isValidColor(const QString& name)```</span>
  ///
  ///
  pub fn is_valid_color<Args>(args: Args) -> bool
    where Args: overloading::ColorIsValidColorArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QColor::light```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn light(&self, ()) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QColor::light() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn light(&self, ::libc::c_int) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QColor::light(int f = ?) const```</span>
  ///
  ///
  pub fn light<'largs, Args>(&'largs self, args: Args) -> ::color::Color
    where Args: overloading::ColorLightArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::lighter```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn lighter(&self, ()) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QColor::lighter() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn lighter(&self, ::libc::c_int) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QColor::lighter(int f = ?) const```</span>
  ///
  ///
  pub fn lighter<'largs, Args>(&'largs self, args: Args) -> ::color::Color
    where Args: overloading::ColorLighterArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QColor::lightness() const```</span>
  ///
  ///
  pub fn lightness(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_lightness(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::lightnessF() const```</span>
  ///
  ///
  pub fn lightness_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_lightnessF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```int QColor::magenta() const```</span>
  ///
  ///
  pub fn magenta(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_magenta(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::magentaF() const```</span>
  ///
  ///
  pub fn magenta_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_magentaF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```QColor::name```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn name(&self, ()) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QColor::name() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn name(&self, ::color::NameFormat) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QColor::name(QColor::NameFormat format) const```</span>
  ///
  ///
  pub fn name<'largs, Args>(&'largs self, args: Args) -> ::qt_core::string::String
    where Args: overloading::ColorNameArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::QColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColor::QColor()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::color::Spec) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColor::QColor(QColor::Spec spec)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::latin1_string::Latin1String) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColor::QColor(QLatin1String name)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::rgba64::Rgba64) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColor::QColor(QRgba64 rgba64)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new(&::qt_core::qt::GlobalColor) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColor::QColor(Qt::GlobalColor color)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new(&::color::Color) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColor::QColor(const QColor& color)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColor::QColor(const QString& name)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColor::QColor(int r, int g, int b)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColor::QColor(int r, int g, int b, int a = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn new(::libc::c_uint) -> ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QColor::QColor(unsigned int rgb)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::color::Color
    where Args: overloading::ColorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QColor::QColor(const char* aname)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(aname: *const ::libc::c_char) -> ::color::Color {
    {
      let mut object: ::color::Color = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QColor_constructor_char(aname, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::qt_core::qt::GlobalColor) -> &'l0 mut ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor& QColor::operator=(Qt::GlobalColor color)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::color::Color) -> &'l0 mut ::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor& QColor::operator=(const QColor& arg1)```</span>
  ///
  ///
  pub fn op_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::color::Color
    where Args: overloading::ColorOpAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QColor::operator==(const QColor& c) const```</span>
  ///
  ///
  pub fn op_eq(&self, c: &::color::Color) -> bool {
    unsafe { ::ffi::qt_gui_c_QColor_operator_eq(self as *const ::color::Color, c as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```bool QColor::operator!=(const QColor& c) const```</span>
  ///
  ///
  pub fn op_neq(&self, c: &::color::Color) -> bool {
    unsafe { ::ffi::qt_gui_c_QColor_operator_neq(self as *const ::color::Color, c as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```int QColor::red() const```</span>
  ///
  ///
  pub fn red(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_red(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::redF() const```</span>
  ///
  ///
  pub fn red_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_redF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QColor::rgb() const```</span>
  ///
  ///
  pub fn rgb(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_gui_c_QColor_rgb(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```unsigned int QColor::rgba() const```</span>
  ///
  ///
  pub fn rgba(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_gui_c_QColor_rgba(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```QRgba64 QColor::rgba64() const```</span>
  ///
  ///
  pub fn rgba64(&self) -> ::rgba64::Rgba64 {
    {
      let mut object: ::rgba64::Rgba64 =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QColor_rgba64_to_output(self as *const ::color::Color, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QColor::saturation() const```</span>
  ///
  ///
  pub fn saturation(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_saturation(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::saturationF() const```</span>
  ///
  ///
  pub fn saturation_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_saturationF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```void QColor::setAlpha(int alpha)```</span>
  ///
  ///
  pub fn set_alpha(&mut self, alpha: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QColor_setAlpha(self as *mut ::color::Color, alpha) }
  }

  /// C++ method: <span style='color: green;'>```void QColor::setAlphaF(double alpha)```</span>
  ///
  ///
  pub fn set_alpha_f(&mut self, alpha: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QColor_setAlphaF(self as *mut ::color::Color, alpha) }
  }

  /// C++ method: <span style='color: green;'>```void QColor::setBlue(int blue)```</span>
  ///
  ///
  pub fn set_blue(&mut self, blue: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QColor_setBlue(self as *mut ::color::Color, blue) }
  }

  /// C++ method: <span style='color: green;'>```void QColor::setBlueF(double blue)```</span>
  ///
  ///
  pub fn set_blue_f(&mut self, blue: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QColor_setBlueF(self as *mut ::color::Color, blue) }
  }

  /// C++ method: <span style='color: green;'>```QColor::setCmyk```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_cmyk(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setCmyk(int c, int m, int y, int k)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_cmyk(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setCmyk(int c, int m, int y, int k, int a = ?)```</span>
  ///
  ///
  pub fn set_cmyk<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorSetCmykArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::setCmykF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_cmyk_f(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setCmykF(double c, double m, double y, double k)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_cmyk_f(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setCmykF(double c, double m, double y, double k, double a = ?)```</span>
  ///
  ///
  pub fn set_cmyk_f<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorSetCmykFArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QColor::setGreen(int green)```</span>
  ///
  ///
  pub fn set_green(&mut self, green: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QColor_setGreen(self as *mut ::color::Color, green) }
  }

  /// C++ method: <span style='color: green;'>```void QColor::setGreenF(double green)```</span>
  ///
  ///
  pub fn set_green_f(&mut self, green: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QColor_setGreenF(self as *mut ::color::Color, green) }
  }

  /// C++ method: <span style='color: green;'>```QColor::setHsl```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_hsl(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setHsl(int h, int s, int l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_hsl(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setHsl(int h, int s, int l, int a = ?)```</span>
  ///
  ///
  pub fn set_hsl<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorSetHslArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::setHslF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_hsl_f(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setHslF(double h, double s, double l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_hsl_f(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setHslF(double h, double s, double l, double a = ?)```</span>
  ///
  ///
  pub fn set_hsl_f<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorSetHslFArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::setHsv```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_hsv(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setHsv(int h, int s, int v)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_hsv(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setHsv(int h, int s, int v, int a = ?)```</span>
  ///
  ///
  pub fn set_hsv<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorSetHsvArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::setHsvF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_hsv_f(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setHsvF(double h, double s, double v)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_hsv_f(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setHsvF(double h, double s, double v, double a = ?)```</span>
  ///
  ///
  pub fn set_hsv_f<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorSetHsvFArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::setNamedColor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_named_color(&mut self, &::qt_core::latin1_string::Latin1String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setNamedColor(QLatin1String name)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_named_color(&mut self, &::qt_core::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setNamedColor(const QString& name)```</span>
  ///
  ///
  pub fn set_named_color<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorSetNamedColorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QColor::setRed(int red)```</span>
  ///
  ///
  pub fn set_red(&mut self, red: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QColor_setRed(self as *mut ::color::Color, red) }
  }

  /// C++ method: <span style='color: green;'>```void QColor::setRedF(double red)```</span>
  ///
  ///
  pub fn set_red_f(&mut self, red: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QColor_setRedF(self as *mut ::color::Color, red) }
  }

  /// C++ method: <span style='color: green;'>```QColor::setRgb```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_rgb(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setRgb(int r, int g, int b)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_rgb(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setRgb(int r, int g, int b, int a = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_rgb(&mut self, ::libc::c_uint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setRgb(unsigned int rgb)```</span>
  ///
  ///
  pub fn set_rgb<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorSetRgbArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor::setRgbF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_rgb_f(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setRgbF(double r, double g, double b)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_rgb_f(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QColor::setRgbF(double r, double g, double b, double a = ?)```</span>
  ///
  ///
  pub fn set_rgb_f<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ColorSetRgbFArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QColor::setRgba(unsigned int rgba)```</span>
  ///
  ///
  pub fn set_rgba(&mut self, rgba: ::libc::c_uint) {
    unsafe { ::ffi::qt_gui_c_QColor_setRgba(self as *mut ::color::Color, rgba) }
  }

  /// C++ method: <span style='color: green;'>```void QColor::setRgba64(QRgba64 rgba)```</span>
  ///
  ///
  pub fn set_rgba64(&mut self, rgba: &::rgba64::Rgba64) {
    unsafe { ::ffi::qt_gui_c_QColor_setRgba64(self as *mut ::color::Color, rgba as *const ::rgba64::Rgba64) }
  }

  /// C++ method: <span style='color: green;'>```QColor::Spec QColor::spec() const```</span>
  ///
  ///
  pub fn spec(&self) -> ::color::Spec {
    unsafe { ::ffi::qt_gui_c_QColor_spec(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```QColor QColor::toCmyk() const```</span>
  ///
  ///
  pub fn to_cmyk(&self) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QColor_toCmyk_to_output(self as *const ::color::Color, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QColor::toHsl() const```</span>
  ///
  ///
  pub fn to_hsl(&self) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QColor_toHsl_to_output(self as *const ::color::Color, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QColor::toHsv() const```</span>
  ///
  ///
  pub fn to_hsv(&self) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QColor_toHsv_to_output(self as *const ::color::Color, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QColor::toRgb() const```</span>
  ///
  ///
  pub fn to_rgb(&self) -> ::color::Color {
    {
      let mut object: ::color::Color = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QColor_toRgb_to_output(self as *const ::color::Color, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QColor::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_value(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::valueF() const```</span>
  ///
  ///
  pub fn value_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_valueF(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```int QColor::yellow() const```</span>
  ///
  ///
  pub fn yellow(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QColor_yellow(self as *const ::color::Color) }
  }

  /// C++ method: <span style='color: green;'>```double QColor::yellowF() const```</span>
  ///
  ///
  pub fn yellow_f(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QColor_yellowF(self as *const ::color::Color) }
  }
}

impl Drop for ::color::Color {
  /// C++ method: <span style='color: green;'>```[destructor] void QColor::~QColor()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QColor_destructor(self as *mut ::color::Color) }
  }
}

/// C++ type: <span style='color: green;'>```QColor::NameFormat```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum NameFormat {
  /// C++ enum variant: <span style='color: green;'>```HexRgb = 0```</span>
  Rgb = 0,
  /// C++ enum variant: <span style='color: green;'>```HexArgb = 1```</span>
  Argb = 1,
}

/// C++ type: <span style='color: green;'>```QColor::Spec```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Spec {
  /// C++ enum variant: <span style='color: green;'>```Invalid = 0```</span>
  Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```Rgb = 1```</span>
  Rgb = 1,
  /// C++ enum variant: <span style='color: green;'>```Hsv = 2```</span>
  Hsv = 2,
  /// C++ enum variant: <span style='color: green;'>```Cmyk = 3```</span>
  Cmyk = 3,
  /// C++ enum variant: <span style='color: green;'>```Hsl = 4```</span>
  Hsl = 4,
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::color::Color)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QColor& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::color::Color)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QColor& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QColor& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::qt_core::data_stream::DataStream,
                        arg2: &'l1 mut ::color::Color)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QColor_G_operator_shr(arg1 as *mut ::qt_core::data_stream::DataStream,
                                          arg2 as *mut ::color::Color)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Color::dark](../struct.Color.html#method.dark) method.
  pub trait ColorDarkArgs<'largs> {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color;
  }
  impl<'largs> ColorDarkArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color {
      let f = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_dark_to_output_f(original_self as *const ::color::Color, f, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ColorDarkArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color {

      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_dark_to_output_no_args(original_self as *const ::color::Color, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::darker](../struct.Color.html#method.darker) method.
  pub trait ColorDarkerArgs<'largs> {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color;
  }
  impl<'largs> ColorDarkerArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color {
      let f = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_darker_to_output_f(original_self as *const ::color::Color, f, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ColorDarkerArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color {

      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_darker_to_output_no_args(original_self as *const ::color::Color, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::from_cmyk](../struct.Color.html#method.from_cmyk) method.
  pub trait ColorFromCmykArgs {
    fn exec(self) -> ::color::Color;
  }
  impl ColorFromCmykArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::color::Color {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromCmyk_to_output_c_m_y_k(c, m, y, k, &mut object);
        }
        object
      }
    }
  }
  impl ColorFromCmykArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::color::Color {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      let a = self.4;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromCmyk_to_output_c_m_y_k_a(c, m, y, k, a, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::from_cmyk_f](../struct.Color.html#method.from_cmyk_f) method.
  pub trait ColorFromCmykFArgs {
    fn exec(self) -> ::color::Color;
  }
  impl ColorFromCmykFArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::color::Color {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromCmykF_to_output_c_m_y_k(c, m, y, k, &mut object);
        }
        object
      }
    }
  }
  impl ColorFromCmykFArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::color::Color {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      let a = self.4;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromCmykF_to_output_c_m_y_k_a(c, m, y, k, a, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::from_hsl](../struct.Color.html#method.from_hsl) method.
  pub trait ColorFromHslArgs {
    fn exec(self) -> ::color::Color;
  }
  impl ColorFromHslArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::color::Color {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromHsl_to_output_h_s_l(h, s, l, &mut object);
        }
        object
      }
    }
  }
  impl ColorFromHslArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::color::Color {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      let a = self.3;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromHsl_to_output_h_s_l_a(h, s, l, a, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::from_hsl_f](../struct.Color.html#method.from_hsl_f) method.
  pub trait ColorFromHslFArgs {
    fn exec(self) -> ::color::Color;
  }
  impl ColorFromHslFArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::color::Color {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromHslF_to_output_h_s_l(h, s, l, &mut object);
        }
        object
      }
    }
  }
  impl ColorFromHslFArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::color::Color {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      let a = self.3;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromHslF_to_output_h_s_l_a(h, s, l, a, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::from_hsv](../struct.Color.html#method.from_hsv) method.
  pub trait ColorFromHsvArgs {
    fn exec(self) -> ::color::Color;
  }
  impl ColorFromHsvArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::color::Color {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromHsv_to_output_h_s_v(h, s, v, &mut object);
        }
        object
      }
    }
  }
  impl ColorFromHsvArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::color::Color {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      let a = self.3;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromHsv_to_output_h_s_v_a(h, s, v, a, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::from_hsv_f](../struct.Color.html#method.from_hsv_f) method.
  pub trait ColorFromHsvFArgs {
    fn exec(self) -> ::color::Color;
  }
  impl ColorFromHsvFArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::color::Color {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromHsvF_to_output_h_s_v(h, s, v, &mut object);
        }
        object
      }
    }
  }
  impl ColorFromHsvFArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::color::Color {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      let a = self.3;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromHsvF_to_output_h_s_v_a(h, s, v, a, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::from_rgb](../struct.Color.html#method.from_rgb) method.
  pub trait ColorFromRgbArgs {
    fn exec(self) -> ::color::Color;
  }
  impl ColorFromRgbArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::color::Color {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromRgb_to_output_r_g_b(r, g, b, &mut object);
        }
        object
      }
    }
  }
  impl ColorFromRgbArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::color::Color {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromRgb_to_output_r_g_b_a(r, g, b, a, &mut object);
        }
        object
      }
    }
  }
  impl ColorFromRgbArgs for ::libc::c_uint {
    fn exec(self) -> ::color::Color {
      let rgb = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromRgb_to_output_rgb(rgb, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::from_rgb_f](../struct.Color.html#method.from_rgb_f) method.
  pub trait ColorFromRgbFArgs {
    fn exec(self) -> ::color::Color;
  }
  impl ColorFromRgbFArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::color::Color {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromRgbF_to_output_r_g_b(r, g, b, &mut object);
        }
        object
      }
    }
  }
  impl ColorFromRgbFArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::color::Color {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromRgbF_to_output_r_g_b_a(r, g, b, a, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::from_rgba64](../struct.Color.html#method.from_rgba64) method.
  pub trait ColorFromRgba64Args {
    fn exec(self) -> ::color::Color;
  }
  impl ColorFromRgba64Args for (::libc::c_ushort, ::libc::c_ushort, ::libc::c_ushort) {
    fn exec(self) -> ::color::Color {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromRgba64_to_output_r_g_b(r, g, b, &mut object);
        }
        object
      }
    }
  }
  impl ColorFromRgba64Args for (::libc::c_ushort, ::libc::c_ushort, ::libc::c_ushort, ::libc::c_ushort) {
    fn exec(self) -> ::color::Color {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromRgba64_to_output_r_g_b_a(r, g, b, a, &mut object);
        }
        object
      }
    }
  }
  impl<'a> ColorFromRgba64Args for &'a ::rgba64::Rgba64 {
    fn exec(self) -> ::color::Color {
      let rgba = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_fromRgba64_to_output_rgba(rgba as *const ::rgba64::Rgba64, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::get_cmyk](../struct.Color.html#method.get_cmyk) method.
  pub trait ColorGetCmykArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::color::Color) -> ();
  }
  impl<'largs> ColorGetCmykArgs<'largs>
    for (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      ::ffi::qt_gui_c_QColor_getCmyk_c_m_y_k(original_self as *mut ::color::Color, c, m, y, k)
    }
  }
  impl<'largs> ColorGetCmykArgs<'largs>
    for (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      let a = self.4;
      ::ffi::qt_gui_c_QColor_getCmyk_c_m_y_k_a(original_self as *mut ::color::Color, c, m, y, k, a)
    }
  }
  /// This trait represents a set of arguments accepted by [Color::get_cmyk_f](../struct.Color.html#method.get_cmyk_f) method.
  pub trait ColorGetCmykFArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::color::Color) -> ();
  }
  impl<'largs> ColorGetCmykFArgs<'largs>
    for (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      ::ffi::qt_gui_c_QColor_getCmykF_c_m_y_k(original_self as *mut ::color::Color, c, m, y, k)
    }
  }
  impl<'largs> ColorGetCmykFArgs<'largs>
    for (*mut ::libc::c_double,
                                                  *mut ::libc::c_double,
                                                  *mut ::libc::c_double,
                                                  *mut ::libc::c_double,
                                                  *mut ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      let a = self.4;
      ::ffi::qt_gui_c_QColor_getCmykF_c_m_y_k_a(original_self as *mut ::color::Color, c, m, y, k, a)
    }
  }
  /// This trait represents a set of arguments accepted by [Color::get_hsl](../struct.Color.html#method.get_hsl) method.
  pub trait ColorGetHslArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> ();
  }
  impl<'largs> ColorGetHslArgs<'largs> for (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      ::ffi::qt_gui_c_QColor_getHsl_h_s_l(original_self as *const ::color::Color, h, s, l)
    }
  }
  impl<'largs> ColorGetHslArgs<'largs>
    for (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      let a = self.3;
      ::ffi::qt_gui_c_QColor_getHsl_h_s_l_a(original_self as *const ::color::Color, h, s, l, a)
    }
  }
  /// This trait represents a set of arguments accepted by [Color::get_hsl_f](../struct.Color.html#method.get_hsl_f) method.
  pub trait ColorGetHslFArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> ();
  }
  impl<'largs> ColorGetHslFArgs<'largs> for (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      ::ffi::qt_gui_c_QColor_getHslF_h_s_l(original_self as *const ::color::Color, h, s, l)
    }
  }
  impl<'largs> ColorGetHslFArgs<'largs>
    for (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      let a = self.3;
      ::ffi::qt_gui_c_QColor_getHslF_h_s_l_a(original_self as *const ::color::Color, h, s, l, a)
    }
  }
  /// This trait represents a set of arguments accepted by [Color::get_hsv](../struct.Color.html#method.get_hsv) method.
  pub trait ColorGetHsvArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> ();
  }
  impl<'largs> ColorGetHsvArgs<'largs> for (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      ::ffi::qt_gui_c_QColor_getHsv_h_s_v(original_self as *const ::color::Color, h, s, v)
    }
  }
  impl<'largs> ColorGetHsvArgs<'largs>
    for (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      let a = self.3;
      ::ffi::qt_gui_c_QColor_getHsv_h_s_v_a(original_self as *const ::color::Color, h, s, v, a)
    }
  }
  /// This trait represents a set of arguments accepted by [Color::get_hsv_f](../struct.Color.html#method.get_hsv_f) method.
  pub trait ColorGetHsvFArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> ();
  }
  impl<'largs> ColorGetHsvFArgs<'largs> for (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      ::ffi::qt_gui_c_QColor_getHsvF_h_s_v(original_self as *const ::color::Color, h, s, v)
    }
  }
  impl<'largs> ColorGetHsvFArgs<'largs>
    for (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      let a = self.3;
      ::ffi::qt_gui_c_QColor_getHsvF_h_s_v_a(original_self as *const ::color::Color, h, s, v, a)
    }
  }
  /// This trait represents a set of arguments accepted by [Color::get_rgb](../struct.Color.html#method.get_rgb) method.
  pub trait ColorGetRgbArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> ();
  }
  impl<'largs> ColorGetRgbArgs<'largs> for (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      ::ffi::qt_gui_c_QColor_getRgb_r_g_b(original_self as *const ::color::Color, r, g, b)
    }
  }
  impl<'largs> ColorGetRgbArgs<'largs>
    for (*mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int, *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      ::ffi::qt_gui_c_QColor_getRgb_r_g_b_a(original_self as *const ::color::Color, r, g, b, a)
    }
  }
  /// This trait represents a set of arguments accepted by [Color::get_rgb_f](../struct.Color.html#method.get_rgb_f) method.
  pub trait ColorGetRgbFArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> ();
  }
  impl<'largs> ColorGetRgbFArgs<'largs> for (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      ::ffi::qt_gui_c_QColor_getRgbF_r_g_b(original_self as *const ::color::Color, r, g, b)
    }
  }
  impl<'largs> ColorGetRgbFArgs<'largs>
    for (*mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double, *mut ::libc::c_double) {
    unsafe fn exec(self, original_self: &'largs ::color::Color) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      ::ffi::qt_gui_c_QColor_getRgbF_r_g_b_a(original_self as *const ::color::Color, r, g, b, a)
    }
  }
  /// This trait represents a set of arguments accepted by [Color::is_valid_color](../struct.Color.html#method.is_valid_color) method.
  pub trait ColorIsValidColorArgs {
    fn exec(self) -> bool;
  }
  impl<'a> ColorIsValidColorArgs for &'a ::qt_core::latin1_string::Latin1String {
    fn exec(self) -> bool {
      let arg1 = self;
      unsafe { ::ffi::qt_gui_c_QColor_isValidColor_arg1(arg1 as *const ::qt_core::latin1_string::Latin1String) }
    }
  }
  impl<'a> ColorIsValidColorArgs for &'a ::qt_core::string::String {
    fn exec(self) -> bool {
      let name = self;
      unsafe { ::ffi::qt_gui_c_QColor_isValidColor_name(name as *const ::qt_core::string::String) }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::light](../struct.Color.html#method.light) method.
  pub trait ColorLightArgs<'largs> {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color;
  }
  impl<'largs> ColorLightArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color {
      let f = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_light_to_output_f(original_self as *const ::color::Color, f, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ColorLightArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color {

      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_light_to_output_no_args(original_self as *const ::color::Color, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::lighter](../struct.Color.html#method.lighter) method.
  pub trait ColorLighterArgs<'largs> {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color;
  }
  impl<'largs> ColorLighterArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color {
      let f = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_lighter_to_output_f(original_self as *const ::color::Color, f, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ColorLighterArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::color::Color) -> ::color::Color {

      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_lighter_to_output_no_args(original_self as *const ::color::Color, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::name](../struct.Color.html#method.name) method.
  pub trait ColorNameArgs<'largs> {
    fn exec(self, original_self: &'largs ::color::Color) -> ::qt_core::string::String;
  }
  impl<'largs> ColorNameArgs<'largs> for ::color::NameFormat {
    fn exec(self, original_self: &'largs ::color::Color) -> ::qt_core::string::String {
      let format = self;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_name_to_output_format(original_self as *const ::color::Color, format, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ColorNameArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::color::Color) -> ::qt_core::string::String {

      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_name_to_output_no_args(original_self as *const ::color::Color, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::new](../struct.Color.html#method.new) method.
  pub trait ColorNewArgs {
    fn exec(self) -> ::color::Color;
  }
  impl<'a> ColorNewArgs for &'a ::color::Color {
    fn exec(self) -> ::color::Color {
      let color = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_constructor_QColor(color as *const ::color::Color, &mut object);
        }
        object
      }
    }
  }
  impl ColorNewArgs for ::color::Spec {
    fn exec(self) -> ::color::Color {
      let spec = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_constructor_QColor_Spec(spec, &mut object);
        }
        object
      }
    }
  }
  impl<'a> ColorNewArgs for &'a ::qt_core::latin1_string::Latin1String {
    fn exec(self) -> ::color::Color {
      let name = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_constructor_QLatin1String(name as *const ::qt_core::latin1_string::Latin1String,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'a> ColorNewArgs for &'a ::rgba64::Rgba64 {
    fn exec(self) -> ::color::Color {
      let rgba64 = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_constructor_QRgba64(rgba64 as *const ::rgba64::Rgba64, &mut object);
        }
        object
      }
    }
  }
  impl<'a> ColorNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::color::Color {
      let name = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_constructor_QString(name as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'a> ColorNewArgs for &'a ::qt_core::qt::GlobalColor {
    fn exec(self) -> ::color::Color {
      let color = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_constructor_Qt_GlobalColor(color as *const ::qt_core::qt::GlobalColor, &mut object);
        }
        object
      }
    }
  }
  impl ColorNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::color::Color {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_constructor_int_int_int(r, g, b, &mut object);
        }
        object
      }
    }
  }
  impl ColorNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::color::Color {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_constructor_int_int_int_int(r, g, b, a, &mut object);
        }
        object
      }
    }
  }
  impl ColorNewArgs for () {
    fn exec(self) -> ::color::Color {

      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl ColorNewArgs for ::libc::c_uint {
    fn exec(self) -> ::color::Color {
      let rgb = self;
      {
        let mut object: ::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_constructor_unsigned_int(rgb, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::op_assign](../struct.Color.html#method.op_assign) method.
  pub trait ColorOpAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::color::Color) -> &'largs mut ::color::Color;
  }
  impl<'largs> ColorOpAssignArgs<'largs> for &'largs ::color::Color {
    fn exec(self, original_self: &'largs mut ::color::Color) -> &'largs mut ::color::Color {
      let arg1 = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QColor_operator_assign_arg1(original_self as *mut ::color::Color,
                                                    arg1 as *const ::color::Color)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ColorOpAssignArgs<'largs> for &'largs ::qt_core::qt::GlobalColor {
    fn exec(self, original_self: &'largs mut ::color::Color) -> &'largs mut ::color::Color {
      let color = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QColor_operator_assign_color(original_self as *mut ::color::Color,
                                                     color as *const ::qt_core::qt::GlobalColor)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Color::set_cmyk](../struct.Color.html#method.set_cmyk) method.
  pub trait ColorSetCmykArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::color::Color) -> ();
  }
  impl<'largs> ColorSetCmykArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      unsafe { ::ffi::qt_gui_c_QColor_setCmyk_c_m_y_k(original_self as *mut ::color::Color, c, m, y, k) }
    }
  }
  impl<'largs> ColorSetCmykArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      let a = self.4;
      unsafe { ::ffi::qt_gui_c_QColor_setCmyk_c_m_y_k_a(original_self as *mut ::color::Color, c, m, y, k, a) }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::set_cmyk_f](../struct.Color.html#method.set_cmyk_f) method.
  pub trait ColorSetCmykFArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::color::Color) -> ();
  }
  impl<'largs> ColorSetCmykFArgs<'largs> for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      unsafe { ::ffi::qt_gui_c_QColor_setCmykF_c_m_y_k(original_self as *mut ::color::Color, c, m, y, k) }
    }
  }
  impl<'largs> ColorSetCmykFArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let c = self.0;
      let m = self.1;
      let y = self.2;
      let k = self.3;
      let a = self.4;
      unsafe { ::ffi::qt_gui_c_QColor_setCmykF_c_m_y_k_a(original_self as *mut ::color::Color, c, m, y, k, a) }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::set_hsl](../struct.Color.html#method.set_hsl) method.
  pub trait ColorSetHslArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::color::Color) -> ();
  }
  impl<'largs> ColorSetHslArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      unsafe { ::ffi::qt_gui_c_QColor_setHsl_h_s_l(original_self as *mut ::color::Color, h, s, l) }
    }
  }
  impl<'largs> ColorSetHslArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      let a = self.3;
      unsafe { ::ffi::qt_gui_c_QColor_setHsl_h_s_l_a(original_self as *mut ::color::Color, h, s, l, a) }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::set_hsl_f](../struct.Color.html#method.set_hsl_f) method.
  pub trait ColorSetHslFArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::color::Color) -> ();
  }
  impl<'largs> ColorSetHslFArgs<'largs> for (::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      unsafe { ::ffi::qt_gui_c_QColor_setHslF_h_s_l(original_self as *mut ::color::Color, h, s, l) }
    }
  }
  impl<'largs> ColorSetHslFArgs<'largs> for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let l = self.2;
      let a = self.3;
      unsafe { ::ffi::qt_gui_c_QColor_setHslF_h_s_l_a(original_self as *mut ::color::Color, h, s, l, a) }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::set_hsv](../struct.Color.html#method.set_hsv) method.
  pub trait ColorSetHsvArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::color::Color) -> ();
  }
  impl<'largs> ColorSetHsvArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      unsafe { ::ffi::qt_gui_c_QColor_setHsv_h_s_v(original_self as *mut ::color::Color, h, s, v) }
    }
  }
  impl<'largs> ColorSetHsvArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      let a = self.3;
      unsafe { ::ffi::qt_gui_c_QColor_setHsv_h_s_v_a(original_self as *mut ::color::Color, h, s, v, a) }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::set_hsv_f](../struct.Color.html#method.set_hsv_f) method.
  pub trait ColorSetHsvFArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::color::Color) -> ();
  }
  impl<'largs> ColorSetHsvFArgs<'largs> for (::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      unsafe { ::ffi::qt_gui_c_QColor_setHsvF_h_s_v(original_self as *mut ::color::Color, h, s, v) }
    }
  }
  impl<'largs> ColorSetHsvFArgs<'largs> for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let h = self.0;
      let s = self.1;
      let v = self.2;
      let a = self.3;
      unsafe { ::ffi::qt_gui_c_QColor_setHsvF_h_s_v_a(original_self as *mut ::color::Color, h, s, v, a) }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::set_named_color](../struct.Color.html#method.set_named_color) method.
  pub trait ColorSetNamedColorArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::color::Color) -> ();
  }
  impl<'largs> ColorSetNamedColorArgs<'largs> for &'largs ::qt_core::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let name = self;
      unsafe {
        ::ffi::qt_gui_c_QColor_setNamedColor_QLatin1String(original_self as *mut ::color::Color,
                                                           name as *const ::qt_core::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> ColorSetNamedColorArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let name = self;
      unsafe {
        ::ffi::qt_gui_c_QColor_setNamedColor_QString(original_self as *mut ::color::Color,
                                                     name as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::set_rgb](../struct.Color.html#method.set_rgb) method.
  pub trait ColorSetRgbArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::color::Color) -> ();
  }
  impl<'largs> ColorSetRgbArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      unsafe { ::ffi::qt_gui_c_QColor_setRgb_r_g_b(original_self as *mut ::color::Color, r, g, b) }
    }
  }
  impl<'largs> ColorSetRgbArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      unsafe { ::ffi::qt_gui_c_QColor_setRgb_r_g_b_a(original_self as *mut ::color::Color, r, g, b, a) }
    }
  }
  impl<'largs> ColorSetRgbArgs<'largs> for ::libc::c_uint {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let rgb = self;
      unsafe { ::ffi::qt_gui_c_QColor_setRgb_rgb(original_self as *mut ::color::Color, rgb) }
    }
  }
  /// This trait represents a set of arguments accepted by [Color::set_rgb_f](../struct.Color.html#method.set_rgb_f) method.
  pub trait ColorSetRgbFArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::color::Color) -> ();
  }
  impl<'largs> ColorSetRgbFArgs<'largs> for (::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      unsafe { ::ffi::qt_gui_c_QColor_setRgbF_r_g_b(original_self as *mut ::color::Color, r, g, b) }
    }
  }
  impl<'largs> ColorSetRgbFArgs<'largs> for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::color::Color) -> () {
      let r = self.0;
      let g = self.1;
      let b = self.2;
      let a = self.3;
      unsafe { ::ffi::qt_gui_c_QColor_setRgbF_r_g_b_a(original_self as *mut ::color::Color, r, g, b, a) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::color::Color) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QColor_G_operator_shl(arg1 as *mut ::qt_core::data_stream::DataStream,
                                              arg2 as *const ::color::Color)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::color::Color) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QColor_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                          arg2 as *const ::color::Color,
                                                          &mut object);
        }
        object
      }
    }
  }
}
