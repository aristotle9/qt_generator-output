/// C++ type: <span style='color: green;'>```QFontMetricsF```</span>
#[repr(C)]
pub struct FontMetricsF([u8; ::type_sizes::QT_GUI_FONT_METRICS_F_FONT_METRICS_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for FontMetricsF {
  unsafe fn new_uninitialized() -> FontMetricsF {
    FontMetricsF(::std::mem::uninitialized())
  }
}

impl FontMetricsF {
  /// C++ method: <span style='color: green;'>```double QFontMetricsF::ascent() const```</span>
  ///
  ///
  pub fn ascent(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_ascent(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::averageCharWidth() const```</span>
  ///
  ///
  pub fn average_char_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_averageCharWidth(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```QFontMetricsF::boundingRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn bounding_rect(&self, &::qt_core::char::Char) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QFontMetricsF::boundingRect(QChar arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn bounding_rect(&self, (&::qt_core::rect_f::RectF, ::libc::c_int, &::qt_core::string::String)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QFontMetricsF::boundingRect(const QRectF& r, int flags, const QString& string) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn bounding_rect(&self, (&::qt_core::rect_f::RectF, ::libc::c_int, &::qt_core::string::String, ::libc::c_int)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QFontMetricsF::boundingRect(const QRectF& r, int flags, const QString& string, int tabstops = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn bounding_rect(&self, &::qt_core::string::String) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QFontMetricsF::boundingRect(const QString& string) const```</span>
  ///
  ///
  pub fn bounding_rect<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect_f::RectF
    where Args: overloading::FontMetricsFBoundingRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF QFontMetricsF::boundingRect(const QRectF& r, int flags, const QString& string, int tabstops = ?, int* tabarray = ?) const```</span>
  ///
  ///
  pub unsafe fn bounding_rect_unsafe(&self,
                                     r: &::qt_core::rect_f::RectF,
                                     flags: ::libc::c_int,
                                     string: &::qt_core::string::String,
                                     tabstops: ::libc::c_int,
                                     tabarray: *mut ::libc::c_int)
                                     -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QFontMetricsF_boundingRect_to_output_r_flags_string_tabstops_tabarray(self as *const ::font_metrics_f::FontMetricsF, r as *const ::qt_core::rect_f::RectF, flags, string as *const ::qt_core::string::String, tabstops, tabarray, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::capHeight() const```</span>
  ///
  ///
  pub fn cap_height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_capHeight(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::descent() const```</span>
  ///
  ///
  pub fn descent(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_descent(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```QFontMetricsF::elidedText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn elided_text(&self, (&::qt_core::string::String, &::qt_core::qt::TextElideMode, ::libc::c_double)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QFontMetricsF::elidedText(const QString& text, Qt::TextElideMode mode, double width) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn elided_text(&self, (&::qt_core::string::String, &::qt_core::qt::TextElideMode, ::libc::c_double, ::libc::c_int)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QFontMetricsF::elidedText(const QString& text, Qt::TextElideMode mode, double width, int flags = ?) const```</span>
  ///
  ///
  pub fn elided_text<'largs, Args>(&'largs self, args: Args) -> ::qt_core::string::String
    where Args: overloading::FontMetricsFElidedTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QFontMetricsF::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_height(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```bool QFontMetricsF::inFont(QChar arg1) const```</span>
  ///
  ///
  pub fn in_font(&self, arg1: &::qt_core::char::Char) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QFontMetricsF_inFont(self as *const ::font_metrics_f::FontMetricsF,
                                           arg1 as *const ::qt_core::char::Char)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFontMetricsF::inFontUcs4(unsigned int ucs4) const```</span>
  ///
  ///
  pub fn in_font_ucs4(&self, ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_inFontUcs4(self as *const ::font_metrics_f::FontMetricsF, ucs4) }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::leading() const```</span>
  ///
  ///
  pub fn leading(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_leading(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::leftBearing(QChar arg1) const```</span>
  ///
  ///
  pub fn left_bearing(&self, arg1: &::qt_core::char::Char) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_gui_c_QFontMetricsF_leftBearing(self as *const ::font_metrics_f::FontMetricsF,
                                                arg1 as *const ::qt_core::char::Char)
    }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::lineSpacing() const```</span>
  ///
  ///
  pub fn line_spacing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_lineSpacing(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::lineWidth() const```</span>
  ///
  ///
  pub fn line_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_lineWidth(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::maxWidth() const```</span>
  ///
  ///
  pub fn max_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_maxWidth(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::minLeftBearing() const```</span>
  ///
  ///
  pub fn min_left_bearing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_minLeftBearing(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::minRightBearing() const```</span>
  ///
  ///
  pub fn min_right_bearing(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_minRightBearing(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```QFontMetricsF::QFontMetricsF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(&::font::Font) -> ::font_metrics_f::FontMetricsF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFontMetricsF::QFontMetricsF(const QFont& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::font_metrics::FontMetrics) -> ::font_metrics_f::FontMetricsF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFontMetricsF::QFontMetricsF(const QFontMetrics& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::font_metrics_f::FontMetricsF) -> ::font_metrics_f::FontMetricsF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFontMetricsF::QFontMetricsF(const QFontMetricsF& arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::font_metrics_f::FontMetricsF
    where Args: overloading::FontMetricsFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QFontMetricsF::QFontMetricsF(const QFont& arg1, QPaintDevice* pd)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(arg1: &::font::Font,
                           pd: *mut ::paint_device::PaintDevice)
                           -> ::font_metrics_f::FontMetricsF {
    {
      let mut object: ::font_metrics_f::FontMetricsF =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QFontMetricsF_constructor_QFont_QPaintDevice(arg1 as *const ::font::Font, pd, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFontMetricsF::operator=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::font_metrics::FontMetrics) -> &'l0 mut ::font_metrics_f::FontMetricsF```<br>
  /// C++ method: <span style='color: green;'>```QFontMetricsF& QFontMetricsF::operator=(const QFontMetrics& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_assign(&mut self, &'l1 ::font_metrics_f::FontMetricsF) -> &'l0 mut ::font_metrics_f::FontMetricsF```<br>
  /// C++ method: <span style='color: green;'>```QFontMetricsF& QFontMetricsF::operator=(const QFontMetricsF& arg1)```</span>
  ///
  ///
  pub fn op_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::font_metrics_f::FontMetricsF
    where Args: overloading::FontMetricsFOpAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QFontMetricsF::operator==(const QFontMetricsF& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::font_metrics_f::FontMetricsF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QFontMetricsF_operator_eq(self as *const ::font_metrics_f::FontMetricsF,
                                                other as *const ::font_metrics_f::FontMetricsF)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFontMetricsF::operator!=(const QFontMetricsF& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::font_metrics_f::FontMetricsF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QFontMetricsF_operator_neq(self as *const ::font_metrics_f::FontMetricsF,
                                                 other as *const ::font_metrics_f::FontMetricsF)
    }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::overlinePos() const```</span>
  ///
  ///
  pub fn overline_pos(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_overlinePos(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::rightBearing(QChar arg1) const```</span>
  ///
  ///
  pub fn right_bearing(&self, arg1: &::qt_core::char::Char) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_gui_c_QFontMetricsF_rightBearing(self as *const ::font_metrics_f::FontMetricsF,
                                                 arg1 as *const ::qt_core::char::Char)
    }
  }

  /// C++ method: <span style='color: green;'>```QFontMetricsF::size```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn size(&self, (::libc::c_int, &::qt_core::string::String)) -> ::qt_core::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```QSizeF QFontMetricsF::size(int flags, const QString& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn size(&self, (::libc::c_int, &::qt_core::string::String, ::libc::c_int)) -> ::qt_core::size_f::SizeF```<br>
  /// C++ method: <span style='color: green;'>```QSizeF QFontMetricsF::size(int flags, const QString& str, int tabstops = ?) const```</span>
  ///
  ///
  pub fn size<'largs, Args>(&'largs self, args: Args) -> ::qt_core::size_f::SizeF
    where Args: overloading::FontMetricsFSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSizeF QFontMetricsF::size(int flags, const QString& str, int tabstops = ?, int* tabarray = ?) const```</span>
  ///
  ///
  pub unsafe fn size_unsafe(&self,
                            flags: ::libc::c_int,
                            str: &::qt_core::string::String,
                            tabstops: ::libc::c_int,
                            tabarray: *mut ::libc::c_int)
                            -> ::qt_core::size_f::SizeF {
    {
      let mut object: ::qt_core::size_f::SizeF =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QFontMetricsF_size_to_output_flags_str_tabstops_tabarray(self as *const ::font_metrics_f::FontMetricsF, flags, str as *const ::qt_core::string::String, tabstops, tabarray, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::strikeOutPos() const```</span>
  ///
  ///
  pub fn strike_out_pos(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_strikeOutPos(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```void QFontMetricsF::swap(QFontMetricsF& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::font_metrics_f::FontMetricsF) {
    unsafe {
      ::ffi::qt_gui_c_QFontMetricsF_swap(self as *mut ::font_metrics_f::FontMetricsF,
                                         other as *mut ::font_metrics_f::FontMetricsF)
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QFontMetricsF::tightBoundingRect(const QString& text) const```</span>
  ///
  ///
  pub fn tight_bounding_rect(&self, text: &::qt_core::string::String) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontMetricsF_tightBoundingRect_to_output(self as *const ::font_metrics_f::FontMetricsF,
                                                                  text as *const ::qt_core::string::String,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QFontMetricsF::underlinePos() const```</span>
  ///
  ///
  pub fn underline_pos(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_underlinePos(self as *const ::font_metrics_f::FontMetricsF) }
  }

  /// C++ method: <span style='color: green;'>```QFontMetricsF::width```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn width(&self, &::qt_core::char::Char) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QFontMetricsF::width(QChar arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn width(&self, &::qt_core::string::String) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QFontMetricsF::width(const QString& string) const```</span>
  ///
  ///
  pub fn width<'largs, Args>(&'largs self, args: Args) -> ::libc::c_double
    where Args: overloading::FontMetricsFWidthArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QFontMetricsF::xHeight() const```</span>
  ///
  ///
  pub fn x_height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_xHeight(self as *const ::font_metrics_f::FontMetricsF) }
  }
}

impl Drop for ::font_metrics_f::FontMetricsF {
  /// C++ method: <span style='color: green;'>```[destructor] void QFontMetricsF::~QFontMetricsF()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QFontMetricsF_destructor(self as *mut ::font_metrics_f::FontMetricsF) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FontMetricsF::bounding_rect](../struct.FontMetricsF.html#method.bounding_rect) method.
  pub trait FontMetricsFBoundingRectArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::qt_core::rect_f::RectF;
  }
  impl<'largs> FontMetricsFBoundingRectArgs<'largs> for &'largs ::qt_core::char::Char {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::qt_core::rect_f::RectF {
      let arg1 = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetricsF_boundingRect_to_output_arg1(original_self as *const ::font_metrics_f::FontMetricsF, arg1 as *const ::qt_core::char::Char, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsFBoundingRectArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::qt_core::rect_f::RectF {
      let r = self.0;
      let flags = self.1;
      let string = self.2;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetricsF_boundingRect_to_output_r_flags_string(original_self as *const ::font_metrics_f::FontMetricsF, r as *const ::qt_core::rect_f::RectF, flags, string as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsFBoundingRectArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int, &'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::qt_core::rect_f::RectF {
      let r = self.0;
      let flags = self.1;
      let string = self.2;
      let tabstops = self.3;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetricsF_boundingRect_to_output_r_flags_string_tabstops(original_self as *const ::font_metrics_f::FontMetricsF, r as *const ::qt_core::rect_f::RectF, flags, string as *const ::qt_core::string::String, tabstops, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsFBoundingRectArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::qt_core::rect_f::RectF {
      let string = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetricsF_boundingRect_to_output_string(original_self as *const ::font_metrics_f::FontMetricsF, string as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontMetricsF::elided_text](../struct.FontMetricsF.html#method.elided_text) method.
  pub trait FontMetricsFElidedTextArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::qt_core::string::String;
  }
  impl<'largs> FontMetricsFElidedTextArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::qt::TextElideMode, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::qt_core::string::String {
      let text = self.0;
      let mode = self.1;
      let width = self.2;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetricsF_elidedText_to_output_text_mode_width(original_self as *const ::font_metrics_f::FontMetricsF, text as *const ::qt_core::string::String, mode as *const ::qt_core::qt::TextElideMode, width, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsFElidedTextArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::qt::TextElideMode, ::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::qt_core::string::String {
      let text = self.0;
      let mode = self.1;
      let width = self.2;
      let flags = self.3;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetricsF_elidedText_to_output_text_mode_width_flags(original_self as *const ::font_metrics_f::FontMetricsF, text as *const ::qt_core::string::String, mode as *const ::qt_core::qt::TextElideMode, width, flags, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontMetricsF::new](../struct.FontMetricsF.html#method.new) method.
  pub trait FontMetricsFNewArgs {
    fn exec(self) -> ::font_metrics_f::FontMetricsF;
  }
  impl<'a> FontMetricsFNewArgs for &'a ::font::Font {
    fn exec(self) -> ::font_metrics_f::FontMetricsF {
      let arg1 = self;
      {
        let mut object: ::font_metrics_f::FontMetricsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetricsF_constructor_QFont(arg1 as *const ::font::Font, &mut object);
        }
        object
      }
    }
  }
  impl<'a> FontMetricsFNewArgs for &'a ::font_metrics::FontMetrics {
    fn exec(self) -> ::font_metrics_f::FontMetricsF {
      let arg1 = self;
      {
        let mut object: ::font_metrics_f::FontMetricsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetricsF_constructor_QFontMetrics(arg1 as *const ::font_metrics::FontMetrics,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'a> FontMetricsFNewArgs for &'a ::font_metrics_f::FontMetricsF {
    fn exec(self) -> ::font_metrics_f::FontMetricsF {
      let arg1 = self;
      {
        let mut object: ::font_metrics_f::FontMetricsF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetricsF_constructor_QFontMetricsF(arg1 as *const ::font_metrics_f::FontMetricsF,
                                                                  &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontMetricsF::op_assign](../struct.FontMetricsF.html#method.op_assign) method.
  pub trait FontMetricsFOpAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::font_metrics_f::FontMetricsF)
            -> &'largs mut ::font_metrics_f::FontMetricsF;
  }
  impl<'largs> FontMetricsFOpAssignArgs<'largs> for &'largs ::font_metrics::FontMetrics {
    fn exec(self,
            original_self: &'largs mut ::font_metrics_f::FontMetricsF)
            -> &'largs mut ::font_metrics_f::FontMetricsF {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QFontMetricsF_operator_assign_QFontMetrics(original_self as *mut ::font_metrics_f::FontMetricsF, arg1 as *const ::font_metrics::FontMetrics) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> FontMetricsFOpAssignArgs<'largs> for &'largs ::font_metrics_f::FontMetricsF {
    fn exec(self,
            original_self: &'largs mut ::font_metrics_f::FontMetricsF)
            -> &'largs mut ::font_metrics_f::FontMetricsF {
      let arg1 = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QFontMetricsF_operator_assign_QFontMetricsF(original_self as *mut ::font_metrics_f::FontMetricsF, arg1 as *const ::font_metrics_f::FontMetricsF) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [FontMetricsF::size](../struct.FontMetricsF.html#method.size) method.
  pub trait FontMetricsFSizeArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::qt_core::size_f::SizeF;
  }
  impl<'largs> FontMetricsFSizeArgs<'largs> for (::libc::c_int, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::qt_core::size_f::SizeF {
      let flags = self.0;
      let str = self.1;
      {
        let mut object: ::qt_core::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetricsF_size_to_output_flags_str(original_self as *const ::font_metrics_f::FontMetricsF, flags, str as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsFSizeArgs<'largs> for (::libc::c_int, &'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::qt_core::size_f::SizeF {
      let flags = self.0;
      let str = self.1;
      let tabstops = self.2;
      {
        let mut object: ::qt_core::size_f::SizeF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetricsF_size_to_output_flags_str_tabstops(original_self as *const ::font_metrics_f::FontMetricsF, flags, str as *const ::qt_core::string::String, tabstops, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontMetricsF::width](../struct.FontMetricsF.html#method.width) method.
  pub trait FontMetricsFWidthArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::libc::c_double;
  }
  impl<'largs> FontMetricsFWidthArgs<'largs> for &'largs ::qt_core::char::Char {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::libc::c_double {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QFontMetricsF_width_arg1(original_self as *const ::font_metrics_f::FontMetricsF,
                                                 arg1 as *const ::qt_core::char::Char)
      }
    }
  }
  impl<'largs> FontMetricsFWidthArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::font_metrics_f::FontMetricsF) -> ::libc::c_double {
      let string = self;
      unsafe {
        ::ffi::qt_gui_c_QFontMetricsF_width_string(original_self as *const ::font_metrics_f::FontMetricsF,
                                                   string as *const ::qt_core::string::String)
      }
    }
  }
}
