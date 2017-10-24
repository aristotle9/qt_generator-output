/// C++ type: <span style='color: green;'>```QFontMetrics```</span>
#[repr(C)]
pub struct FontMetrics([u8; ::type_sizes::QT_GUI_FONT_METRICS_FONT_METRICS]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for FontMetrics {
  unsafe fn new_uninitialized() -> FontMetrics {
    FontMetrics(::std::mem::uninitialized())
  }
}

impl FontMetrics {
  /// C++ method: <span style='color: green;'>```int QFontMetrics::ascent() const```</span>
  ///
  ///
  pub fn ascent(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_ascent(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::averageCharWidth() const```</span>
  ///
  ///
  pub fn average_char_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_averageCharWidth(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```QFontMetrics::boundingRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn bounding_rect(&self, &::qt_core::char::Char) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QFontMetrics::boundingRect(QChar arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn bounding_rect(&self, (&::qt_core::rect::Rect, ::libc::c_int, &::qt_core::string::String)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QFontMetrics::boundingRect(const QRect& r, int flags, const QString& text) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn bounding_rect(&self, (&::qt_core::rect::Rect, ::libc::c_int, &::qt_core::string::String, ::libc::c_int)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QFontMetrics::boundingRect(const QRect& r, int flags, const QString& text, int tabstops = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn bounding_rect(&self, &::qt_core::string::String) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QFontMetrics::boundingRect(const QString& text) const```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn bounding_rect(&self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_core::string::String)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QFontMetrics::boundingRect(int x, int y, int w, int h, int flags, const QString& text) const```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn bounding_rect(&self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_core::string::String, ::libc::c_int)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QFontMetrics::boundingRect(int x, int y, int w, int h, int flags, const QString& text, int tabstops = ?) const```</span>
  ///
  ///
  pub fn bounding_rect<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect::Rect
    where Args: overloading::FontMetricsBoundingRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFontMetrics::boundingRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn bounding_rect_unsafe(&self, (&::qt_core::rect::Rect, ::libc::c_int, &::qt_core::string::String, ::libc::c_int, *mut ::libc::c_int)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QFontMetrics::boundingRect(const QRect& r, int flags, const QString& text, int tabstops = ?, int* tabarray = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn bounding_rect_unsafe(&self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_core::string::String, ::libc::c_int, *mut ::libc::c_int)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QFontMetrics::boundingRect(int x, int y, int w, int h, int flags, const QString& text, int tabstops = ?, int* tabarray = ?) const```</span>
  ///
  ///
  pub unsafe fn bounding_rect_unsafe<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect::Rect
    where Args: overloading::FontMetricsBoundingRectUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QFontMetrics::capHeight() const```</span>
  ///
  ///
  pub fn cap_height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_capHeight(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::charWidth(const QString& str, int pos) const```</span>
  ///
  ///
  pub fn char_width(&self, str: &::qt_core::string::String, pos: ::libc::c_int) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QFontMetrics_charWidth(self as *const ::font_metrics::FontMetrics,
                                             str as *const ::qt_core::string::String,
                                             pos)
    }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::descent() const```</span>
  ///
  ///
  pub fn descent(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_descent(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```QFontMetrics::elidedText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn elided_text(&self, (&::qt_core::string::String, &::qt_core::qt::TextElideMode, ::libc::c_int)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QFontMetrics::elidedText(const QString& text, Qt::TextElideMode mode, int width) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn elided_text(&self, (&::qt_core::string::String, &::qt_core::qt::TextElideMode, ::libc::c_int, ::libc::c_int)) -> ::qt_core::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QFontMetrics::elidedText(const QString& text, Qt::TextElideMode mode, int width, int flags = ?) const```</span>
  ///
  ///
  pub fn elided_text<'largs, Args>(&'largs self, args: Args) -> ::qt_core::string::String
    where Args: overloading::FontMetricsElidedTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QFontMetrics::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_height(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```bool QFontMetrics::inFont(QChar arg1) const```</span>
  ///
  ///
  pub fn in_font(&self, arg1: &::qt_core::char::Char) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QFontMetrics_inFont(self as *const ::font_metrics::FontMetrics,
                                          arg1 as *const ::qt_core::char::Char)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFontMetrics::inFontUcs4(unsigned int ucs4) const```</span>
  ///
  ///
  pub fn in_font_ucs4(&self, ucs4: ::libc::c_uint) -> bool {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_inFontUcs4(self as *const ::font_metrics::FontMetrics, ucs4) }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::leading() const```</span>
  ///
  ///
  pub fn leading(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_leading(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::leftBearing(QChar arg1) const```</span>
  ///
  ///
  pub fn left_bearing(&self, arg1: &::qt_core::char::Char) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QFontMetrics_leftBearing(self as *const ::font_metrics::FontMetrics,
                                               arg1 as *const ::qt_core::char::Char)
    }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::lineSpacing() const```</span>
  ///
  ///
  pub fn line_spacing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_lineSpacing(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::lineWidth() const```</span>
  ///
  ///
  pub fn line_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_lineWidth(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::maxWidth() const```</span>
  ///
  ///
  pub fn max_width(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_maxWidth(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::minLeftBearing() const```</span>
  ///
  ///
  pub fn min_left_bearing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_minLeftBearing(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::minRightBearing() const```</span>
  ///
  ///
  pub fn min_right_bearing(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_minRightBearing(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```QFontMetrics::QFontMetrics```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(&::font::Font) -> ::font_metrics::FontMetrics```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFontMetrics::QFontMetrics(const QFont& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::font_metrics::FontMetrics) -> ::font_metrics::FontMetrics```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QFontMetrics::QFontMetrics(const QFontMetrics& arg1)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::font_metrics::FontMetrics
    where Args: overloading::FontMetricsNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QFontMetrics::QFontMetrics(const QFont& arg1, QPaintDevice* pd)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(arg1: &::font::Font, pd: *mut ::paint_device::PaintDevice) -> ::font_metrics::FontMetrics {
    {
      let mut object: ::font_metrics::FontMetrics =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QFontMetrics_constructor_QFont_QPaintDevice(arg1 as *const ::font::Font, pd, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFontMetrics& QFontMetrics::operator=(const QFontMetrics& arg1)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             arg1: &'l1 ::font_metrics::FontMetrics)
                             -> &'l0 mut ::font_metrics::FontMetrics {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QFontMetrics_operator_assign(self as *mut ::font_metrics::FontMetrics,
                                                   arg1 as *const ::font_metrics::FontMetrics)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QFontMetrics::operator==(const QFontMetrics& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::font_metrics::FontMetrics) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QFontMetrics_operator_eq(self as *const ::font_metrics::FontMetrics,
                                               other as *const ::font_metrics::FontMetrics)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QFontMetrics::operator!=(const QFontMetrics& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::font_metrics::FontMetrics) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QFontMetrics_operator_neq(self as *const ::font_metrics::FontMetrics,
                                                other as *const ::font_metrics::FontMetrics)
    }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::overlinePos() const```</span>
  ///
  ///
  pub fn overline_pos(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_overlinePos(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::rightBearing(QChar arg1) const```</span>
  ///
  ///
  pub fn right_bearing(&self, arg1: &::qt_core::char::Char) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QFontMetrics_rightBearing(self as *const ::font_metrics::FontMetrics,
                                                arg1 as *const ::qt_core::char::Char)
    }
  }

  /// C++ method: <span style='color: green;'>```QFontMetrics::size```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn size(&self, (::libc::c_int, &::qt_core::string::String)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QFontMetrics::size(int flags, const QString& str) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn size(&self, (::libc::c_int, &::qt_core::string::String, ::libc::c_int)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QFontMetrics::size(int flags, const QString& str, int tabstops = ?) const```</span>
  ///
  ///
  pub fn size<'largs, Args>(&'largs self, args: Args) -> ::qt_core::size::Size
    where Args: overloading::FontMetricsSizeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSize QFontMetrics::size(int flags, const QString& str, int tabstops = ?, int* tabarray = ?) const```</span>
  ///
  ///
  pub unsafe fn size_unsafe(&self,
                            flags: ::libc::c_int,
                            str: &::qt_core::string::String,
                            tabstops: ::libc::c_int,
                            tabarray: *mut ::libc::c_int)
                            -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_gui_c_QFontMetrics_size_to_output_flags_str_tabstops_tabarray(self as *const ::font_metrics::FontMetrics, flags, str as *const ::qt_core::string::String, tabstops, tabarray, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::strikeOutPos() const```</span>
  ///
  ///
  pub fn strike_out_pos(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_strikeOutPos(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```void QFontMetrics::swap(QFontMetrics& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::font_metrics::FontMetrics) {
    unsafe {
      ::ffi::qt_gui_c_QFontMetrics_swap(self as *mut ::font_metrics::FontMetrics,
                                        other as *mut ::font_metrics::FontMetrics)
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QFontMetrics::tightBoundingRect(const QString& text) const```</span>
  ///
  ///
  pub fn tight_bounding_rect(&self, text: &::qt_core::string::String) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QFontMetrics_tightBoundingRect_to_output(self as *const ::font_metrics::FontMetrics,
                                                                 text as *const ::qt_core::string::String,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QFontMetrics::underlinePos() const```</span>
  ///
  ///
  pub fn underline_pos(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_underlinePos(self as *const ::font_metrics::FontMetrics) }
  }

  /// C++ method: <span style='color: green;'>```QFontMetrics::width```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn width(&self, &::qt_core::char::Char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QFontMetrics::width(QChar arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn width(&self, &::qt_core::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QFontMetrics::width(const QString& arg1) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn width(&self, (&::qt_core::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QFontMetrics::width(const QString& arg1, int len = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn width(&self, (&::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QFontMetrics::width(const QString& arg1, int len, int flags) const```</span>
  ///
  ///
  pub fn width<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::FontMetricsWidthArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QFontMetrics::xHeight() const```</span>
  ///
  ///
  pub fn x_height(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_xHeight(self as *const ::font_metrics::FontMetrics) }
  }
}

impl Drop for ::font_metrics::FontMetrics {
  /// C++ method: <span style='color: green;'>```[destructor] void QFontMetrics::~QFontMetrics()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QFontMetrics_destructor(self as *mut ::font_metrics::FontMetrics) }
  }
}

/// C++ method: <span style='color: green;'>```swap```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn swap((&mut ::font_metrics::FontMetrics, &mut ::font_metrics::FontMetrics)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QFontMetrics& value1, QFontMetrics& value2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn swap((&mut ::font_metrics_f::FontMetricsF, &mut ::font_metrics_f::FontMetricsF)) -> ()```<br>
/// C++ method: <span style='color: green;'>```void swap(QFontMetricsF& value1, QFontMetricsF& value2)```</span>
///
///
pub fn swap<Args>(args: Args) -> ()
  where Args: overloading::SwapArgs
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [FontMetrics::bounding_rect](../struct.FontMetrics.html#method.bounding_rect) method.
  pub trait FontMetricsBoundingRectArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::rect::Rect;
  }
  impl<'largs> FontMetricsBoundingRectArgs<'largs> for &'largs ::qt_core::char::Char {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::rect::Rect {
      let arg1 = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_boundingRect_to_output_arg1(original_self as *const ::font_metrics::FontMetrics, arg1 as *const ::qt_core::char::Char, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsBoundingRectArgs<'largs>
    for (&'largs ::qt_core::rect::Rect, ::libc::c_int, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::rect::Rect {
      let r = self.0;
      let flags = self.1;
      let text = self.2;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_boundingRect_to_output_r_flags_text(original_self as *const ::font_metrics::FontMetrics, r as *const ::qt_core::rect::Rect, flags, text as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsBoundingRectArgs<'largs>
    for (&'largs ::qt_core::rect::Rect, ::libc::c_int, &'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::rect::Rect {
      let r = self.0;
      let flags = self.1;
      let text = self.2;
      let tabstops = self.3;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_boundingRect_to_output_r_flags_text_tabstops(original_self as *const ::font_metrics::FontMetrics, r as *const ::qt_core::rect::Rect, flags, text as *const ::qt_core::string::String, tabstops, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsBoundingRectArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::rect::Rect {
      let text = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_boundingRect_to_output_text(original_self as *const ::font_metrics::FontMetrics, text as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsBoundingRectArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::rect::Rect {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let flags = self.4;
      let text = self.5;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_boundingRect_to_output_x_y_w_h_flags_text(original_self as *const ::font_metrics::FontMetrics, x, y, w, h, flags, text as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsBoundingRectArgs<'largs>
    for (::libc::c_int,
                                                            ::libc::c_int,
                                                            ::libc::c_int,
                                                            ::libc::c_int,
                                                            ::libc::c_int,
                                                            &'largs ::qt_core::string::String,
                                                            ::libc::c_int) {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::rect::Rect {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let flags = self.4;
      let text = self.5;
      let tabstops = self.6;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_boundingRect_to_output_x_y_w_h_flags_text_tabstops(original_self as *const ::font_metrics::FontMetrics, x, y, w, h, flags, text as *const ::qt_core::string::String, tabstops, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontMetrics::bounding_rect_unsafe](../struct.FontMetrics.html#method.bounding_rect_unsafe) method.
  pub trait FontMetricsBoundingRectUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::rect::Rect;
  }
  impl<'largs> FontMetricsBoundingRectUnsafeArgs<'largs>
    for (&'largs ::qt_core::rect::Rect,
                                                                  ::libc::c_int,
                                                                  &'largs ::qt_core::string::String,
                                                                  ::libc::c_int,
                                                                  *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::rect::Rect {
      let r = self.0;
      let flags = self.1;
      let text = self.2;
      let tabstops = self.3;
      let tabarray = self.4;
      {
        let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_gui_c_QFontMetrics_boundingRect_to_output_r_flags_text_tabstops_tabarray(original_self as *const ::font_metrics::FontMetrics, r as *const ::qt_core::rect::Rect, flags, text as *const ::qt_core::string::String, tabstops, tabarray, &mut object);
        object
      }
    }
  }
  impl<'largs> FontMetricsBoundingRectUnsafeArgs<'largs>
    for (::libc::c_int,
                                                                  ::libc::c_int,
                                                                  ::libc::c_int,
                                                                  ::libc::c_int,
                                                                  ::libc::c_int,
                                                                  &'largs ::qt_core::string::String,
                                                                  ::libc::c_int,
                                                                  *mut ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::rect::Rect {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let flags = self.4;
      let text = self.5;
      let tabstops = self.6;
      let tabarray = self.7;
      {
        let mut object: ::qt_core::rect::Rect = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_gui_c_QFontMetrics_boundingRect_to_output_x_y_w_h_flags_text_tabstops_tabarray(original_self as *const ::font_metrics::FontMetrics, x, y, w, h, flags, text as *const ::qt_core::string::String, tabstops, tabarray, &mut object);
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontMetrics::elided_text](../struct.FontMetrics.html#method.elided_text) method.
  pub trait FontMetricsElidedTextArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::string::String;
  }
  impl<'largs> FontMetricsElidedTextArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::qt::TextElideMode, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::string::String {
      let text = self.0;
      let mode = self.1;
      let width = self.2;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_elidedText_to_output_text_mode_width(original_self as *const ::font_metrics::FontMetrics, text as *const ::qt_core::string::String, mode as *const ::qt_core::qt::TextElideMode, width, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsElidedTextArgs<'largs>
    for (&'largs ::qt_core::string::String, &'largs ::qt_core::qt::TextElideMode, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::string::String {
      let text = self.0;
      let mode = self.1;
      let width = self.2;
      let flags = self.3;
      {
        let mut object: ::qt_core::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_elidedText_to_output_text_mode_width_flags(original_self as *const ::font_metrics::FontMetrics, text as *const ::qt_core::string::String, mode as *const ::qt_core::qt::TextElideMode, width, flags, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontMetrics::new](../struct.FontMetrics.html#method.new) method.
  pub trait FontMetricsNewArgs {
    fn exec(self) -> ::font_metrics::FontMetrics;
  }
  impl<'a> FontMetricsNewArgs for &'a ::font::Font {
    fn exec(self) -> ::font_metrics::FontMetrics {
      let arg1 = self;
      {
        let mut object: ::font_metrics::FontMetrics =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_constructor_QFont(arg1 as *const ::font::Font, &mut object);
        }
        object
      }
    }
  }
  impl<'a> FontMetricsNewArgs for &'a ::font_metrics::FontMetrics {
    fn exec(self) -> ::font_metrics::FontMetrics {
      let arg1 = self;
      {
        let mut object: ::font_metrics::FontMetrics =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_constructor_QFontMetrics(arg1 as *const ::font_metrics::FontMetrics,
                                                                &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontMetrics::size](../struct.FontMetrics.html#method.size) method.
  pub trait FontMetricsSizeArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::size::Size;
  }
  impl<'largs> FontMetricsSizeArgs<'largs> for (::libc::c_int, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::size::Size {
      let flags = self.0;
      let str = self.1;
      {
        let mut object: ::qt_core::size::Size =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_size_to_output_flags_str(original_self as *const ::font_metrics::FontMetrics,
                                                                flags,
                                                                str as *const ::qt_core::string::String,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> FontMetricsSizeArgs<'largs> for (::libc::c_int, &'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::qt_core::size::Size {
      let flags = self.0;
      let str = self.1;
      let tabstops = self.2;
      {
        let mut object: ::qt_core::size::Size =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QFontMetrics_size_to_output_flags_str_tabstops(original_self as *const ::font_metrics::FontMetrics, flags, str as *const ::qt_core::string::String, tabstops, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [FontMetrics::width](../struct.FontMetrics.html#method.width) method.
  pub trait FontMetricsWidthArgs<'largs> {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::libc::c_int;
  }
  impl<'largs> FontMetricsWidthArgs<'largs> for &'largs ::qt_core::char::Char {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::libc::c_int {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QFontMetrics_width_QChar(original_self as *const ::font_metrics::FontMetrics,
                                                 arg1 as *const ::qt_core::char::Char)
      }
    }
  }
  impl<'largs> FontMetricsWidthArgs<'largs> for &'largs ::qt_core::string::String {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::libc::c_int {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QFontMetrics_width_QString(original_self as *const ::font_metrics::FontMetrics,
                                                   arg1 as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> FontMetricsWidthArgs<'largs> for (&'largs ::qt_core::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::libc::c_int {
      let arg1 = self.0;
      let len = self.1;
      unsafe {
        ::ffi::qt_gui_c_QFontMetrics_width_QString_int(original_self as *const ::font_metrics::FontMetrics,
                                                       arg1 as *const ::qt_core::string::String,
                                                       len)
      }
    }
  }
  impl<'largs> FontMetricsWidthArgs<'largs> for (&'largs ::qt_core::string::String, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::font_metrics::FontMetrics) -> ::libc::c_int {
      let arg1 = self.0;
      let len = self.1;
      let flags = self.2;
      unsafe {
        ::ffi::qt_gui_c_QFontMetrics_width_QString_int_int(original_self as *const ::font_metrics::FontMetrics,
                                                           arg1 as *const ::qt_core::string::String,
                                                           len,
                                                           flags)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [swap](../fn.swap.html) method.
  pub trait SwapArgs {
    fn exec(self) -> ();
  }
  impl<'a> SwapArgs for (&'a mut ::font_metrics_f::FontMetricsF, &'a mut ::font_metrics_f::FontMetricsF) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe {
        ::ffi::qt_gui_c_QFontMetrics_G_swap_QFontMetricsF_QFontMetricsF(value1 as *mut ::font_metrics_f::FontMetricsF,
                                                                        value2 as *mut ::font_metrics_f::FontMetricsF)
      }
    }
  }
  impl<'a> SwapArgs for (&'a mut ::font_metrics::FontMetrics, &'a mut ::font_metrics::FontMetrics) {
    fn exec(self) -> () {
      let value1 = self.0;
      let value2 = self.1;
      unsafe {
        ::ffi::qt_gui_c_QFontMetrics_G_swap_QFontMetrics_QFontMetrics(value1 as *mut ::font_metrics::FontMetrics,
                                                                      value2 as *mut ::font_metrics::FontMetrics)
      }
    }
  }
}
