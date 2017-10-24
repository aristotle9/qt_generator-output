/// C++ type: <span style='color: green;'>```QTextLayout::CursorMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CursorMode {
  /// C++ enum variant: <span style='color: green;'>```SkipCharacters = 0```</span>
  Characters = 0,
  /// C++ enum variant: <span style='color: green;'>```SkipWords = 1```</span>
  Words = 1,
}

/// C++ type: <span style='color: green;'>```QTextLayout::FormatRange```</span>
#[repr(C)]
pub struct FormatRange(u8);

impl FormatRange {
  /// C++ method: <span style='color: green;'>```const QTextCharFormat& QTextLayout::FormatRange::format() const```</span>
  ///
  ///
  pub fn format<'l0>(&'l0 self) -> &'l0 ::text_char_format::TextCharFormat {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextLayout_FormatRange_format(self as *const ::text_layout::FormatRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextCharFormat& QTextLayout::FormatRange::format_mut()```</span>
  ///
  ///
  pub fn format_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_char_format::TextCharFormat {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QTextLayout_FormatRange_format_mut(self as *mut ::text_layout::FormatRange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QTextLayout::FormatRange::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextLayout_FormatRange_length(self as *const ::text_layout::FormatRange) }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::FormatRange::set_format(QTextCharFormat value)```</span>
  ///
  ///
  pub fn set_format(&mut self, value: &::text_char_format::TextCharFormat) {
    unsafe {
      ::ffi::qt_gui_c_QTextLayout_FormatRange_set_format(self as *mut ::text_layout::FormatRange,
                                                         value as *const ::text_char_format::TextCharFormat)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::FormatRange::set_length(int value)```</span>
  ///
  ///
  pub fn set_length(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextLayout_FormatRange_set_length(self as *mut ::text_layout::FormatRange, value) }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::FormatRange::set_start(int value)```</span>
  ///
  ///
  pub fn set_start(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextLayout_FormatRange_set_start(self as *mut ::text_layout::FormatRange, value) }
  }

  /// C++ method: <span style='color: green;'>```int QTextLayout::FormatRange::start() const```</span>
  ///
  ///
  pub fn start(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextLayout_FormatRange_start(self as *const ::text_layout::FormatRange) }
  }
}

impl ::cpp_utils::CppDeletable for ::text_layout::FormatRange {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTextLayout_FormatRange_delete
  }
}

/// C++ type: <span style='color: green;'>```QTextLayout```</span>
#[repr(C)]
pub struct TextLayout(u8);

impl TextLayout {
  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange> QTextLayout::additionalFormats() const```</span>
  ///
  ///
  pub fn additional_formats(&self) -> ::list::ListTextLayoutFormatRange {
    {
      let mut object: ::list::ListTextLayoutFormatRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_additionalFormats_to_output(self as *const ::text_layout::TextLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::beginLayout()```</span>
  ///
  ///
  pub fn begin_layout(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextLayout_beginLayout(self as *mut ::text_layout::TextLayout) }
  }

  /// C++ method: <span style='color: green;'>```QRectF QTextLayout::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_boundingRect_to_output(self as *const ::text_layout::TextLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QTextLayout::cacheEnabled() const```</span>
  ///
  ///
  pub fn cache_enabled(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextLayout_cacheEnabled(self as *const ::text_layout::TextLayout) }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::clearAdditionalFormats()```</span>
  ///
  ///
  pub fn clear_additional_formats(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextLayout_clearAdditionalFormats(self as *mut ::text_layout::TextLayout) }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::clearFormats()```</span>
  ///
  ///
  pub fn clear_formats(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextLayout_clearFormats(self as *mut ::text_layout::TextLayout) }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::clearLayout()```</span>
  ///
  ///
  pub fn clear_layout(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextLayout_clearLayout(self as *mut ::text_layout::TextLayout) }
  }

  /// C++ method: <span style='color: green;'>```QTextLine QTextLayout::createLine()```</span>
  ///
  ///
  pub fn create_line(&mut self) -> ::text_line::TextLine {
    {
      let mut object: ::text_line::TextLine =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_createLine_to_output(self as *mut ::text_layout::TextLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::draw```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw(&self, (*mut ::painter::Painter, &::qt_core::point_f::PointF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextLayout::draw(QPainter* p, const QPointF& pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw(&self, (*mut ::painter::Painter, &::qt_core::point_f::PointF, &::vector::VectorTextLayoutFormatRange)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextLayout::draw(QPainter* p, const QPointF& pos, const QVector<QTextLayout::FormatRange>& selections = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw(&self, (*mut ::painter::Painter, &::qt_core::point_f::PointF, &::vector::VectorTextLayoutFormatRange, &::qt_core::rect_f::RectF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextLayout::draw(QPainter* p, const QPointF& pos, const QVector<QTextLayout::FormatRange>& selections = ?, const QRectF& clip = ?) const```</span>
  ///
  ///
  pub unsafe fn draw<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::TextLayoutDrawArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextLayout::drawCursor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_cursor(&self, (*mut ::painter::Painter, &::qt_core::point_f::PointF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextLayout::drawCursor(QPainter* p, const QPointF& pos, int cursorPosition) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_cursor(&self, (*mut ::painter::Painter, &::qt_core::point_f::PointF, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextLayout::drawCursor(QPainter* p, const QPointF& pos, int cursorPosition, int width) const```</span>
  ///
  ///
  pub unsafe fn draw_cursor<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::TextLayoutDrawCursorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTextLayout::endLayout()```</span>
  ///
  ///
  pub fn end_layout(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextLayout_endLayout(self as *mut ::text_layout::TextLayout) }
  }

  /// C++ method: <span style='color: green;'>```QFont QTextLayout::font() const```</span>
  ///
  ///
  pub fn font(&self) -> ::font::Font {
    {
      let mut object: ::font::Font = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_font_to_output(self as *const ::text_layout::TextLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QTextLayout::FormatRange> QTextLayout::formats() const```</span>
  ///
  ///
  pub fn formats(&self) -> ::vector::VectorTextLayoutFormatRange {
    {
      let mut object: ::vector::VectorTextLayoutFormatRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_formats_to_output(self as *const ::text_layout::TextLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::glyphRuns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn glyph_runs(&self, ()) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QTextLayout::glyphRuns() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn glyph_runs(&self, ::libc::c_int) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QTextLayout::glyphRuns(int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn glyph_runs(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QTextLayout::glyphRuns(int from = ?, int length = ?) const```</span>
  ///
  ///
  pub fn glyph_runs<'largs, Args>(&'largs self, args: Args) -> ::list::ListGlyphRun
    where Args: overloading::TextLayoutGlyphRunsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QTextLayout::isValidCursorPosition(int pos) const```</span>
  ///
  ///
  pub fn is_valid_cursor_position(&self, pos: ::libc::c_int) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextLayout_isValidCursorPosition(self as *const ::text_layout::TextLayout, pos) }
  }

  /// C++ method: <span style='color: green;'>```int QTextLayout::leftCursorPosition(int oldPos) const```</span>
  ///
  ///
  pub fn left_cursor_position(&self, old_pos: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextLayout_leftCursorPosition(self as *const ::text_layout::TextLayout, old_pos) }
  }

  /// C++ method: <span style='color: green;'>```QTextLine QTextLayout::lineAt(int i) const```</span>
  ///
  ///
  pub fn line_at(&self, i: ::libc::c_int) -> ::text_line::TextLine {
    {
      let mut object: ::text_line::TextLine =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_lineAt_to_output(self as *const ::text_layout::TextLayout, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextLayout::lineCount() const```</span>
  ///
  ///
  pub fn line_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextLayout_lineCount(self as *const ::text_layout::TextLayout) }
  }

  /// C++ method: <span style='color: green;'>```QTextLine QTextLayout::lineForTextPosition(int pos) const```</span>
  ///
  ///
  pub fn line_for_text_position(&self, pos: ::libc::c_int) -> ::text_line::TextLine {
    {
      let mut object: ::text_line::TextLine =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_lineForTextPosition_to_output(self as *const ::text_layout::TextLayout,
                                                                  pos,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextLayout::maximumWidth() const```</span>
  ///
  ///
  pub fn maximum_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLayout_maximumWidth(self as *const ::text_layout::TextLayout) }
  }

  /// C++ method: <span style='color: green;'>```double QTextLayout::minimumWidth() const```</span>
  ///
  ///
  pub fn minimum_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLayout_minimumWidth(self as *const ::text_layout::TextLayout) }
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::QTextLayout```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::text_layout::TextLayout>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextLayout::QTextLayout()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::string::String) -> ::cpp_utils::CppBox<::text_layout::TextLayout>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextLayout::QTextLayout(const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::qt_core::string::String, &::font::Font)) -> ::cpp_utils::CppBox<::text_layout::TextLayout>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextLayout::QTextLayout(const QString& text, const QFont& font)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::text_block::TextBlock) -> ::cpp_utils::CppBox<::text_layout::TextLayout>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QTextLayout::QTextLayout(const QTextBlock& b)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::text_layout::TextLayout>
    where Args: overloading::TextLayoutNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QTextLayout::QTextLayout(const QString& text, const QFont& font, QPaintDevice* paintdevice = ?)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(text: &::qt_core::string::String,
                           font: &::font::Font,
                           paintdevice: *mut ::paint_device::PaintDevice)
                           -> ::cpp_utils::CppBox<::text_layout::TextLayout> {
    let ffi_result = ::ffi::qt_gui_c_QTextLayout_new_text_font_paintdevice(text as *const ::qt_core::string::String,
                                                                           font as *const ::font::Font,
                                                                           paintdevice);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::nextCursorPosition```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn next_cursor_position(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTextLayout::nextCursorPosition(int oldPos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn next_cursor_position(&self, (::libc::c_int, ::text_layout::CursorMode)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTextLayout::nextCursorPosition(int oldPos, QTextLayout::CursorMode mode = ?) const```</span>
  ///
  ///
  pub fn next_cursor_position<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::TextLayoutNextCursorPositionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPointF QTextLayout::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_position_to_output(self as *const ::text_layout::TextLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextLayout::preeditAreaPosition() const```</span>
  ///
  ///
  pub fn preedit_area_position(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextLayout_preeditAreaPosition(self as *const ::text_layout::TextLayout) }
  }

  /// C++ method: <span style='color: green;'>```QString QTextLayout::preeditAreaText() const```</span>
  ///
  ///
  pub fn preedit_area_text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_preeditAreaText_to_output(self as *const ::text_layout::TextLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::previousCursorPosition```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn previous_cursor_position(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTextLayout::previousCursorPosition(int oldPos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn previous_cursor_position(&self, (::libc::c_int, ::text_layout::CursorMode)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTextLayout::previousCursorPosition(int oldPos, QTextLayout::CursorMode mode = ?) const```</span>
  ///
  ///
  pub fn previous_cursor_position<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::TextLayoutPreviousCursorPositionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QTextLayout::rightCursorPosition(int oldPos) const```</span>
  ///
  ///
  pub fn right_cursor_position(&self, old_pos: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextLayout_rightCursorPosition(self as *const ::text_layout::TextLayout, old_pos) }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::setAdditionalFormats(const QList<QTextLayout::FormatRange>& overrides)```</span>
  ///
  ///
  pub fn set_additional_formats(&mut self, overrides: &::list::ListTextLayoutFormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QTextLayout_setAdditionalFormats(self as *mut ::text_layout::TextLayout,
                                                       overrides as *const ::list::ListTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::setCacheEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_cache_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_gui_c_QTextLayout_setCacheEnabled(self as *mut ::text_layout::TextLayout, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::setCursorMoveStyle(Qt::CursorMoveStyle style)```</span>
  ///
  ///
  pub fn set_cursor_move_style(&mut self, style: &::qt_core::qt::CursorMoveStyle) {
    unsafe {
      ::ffi::qt_gui_c_QTextLayout_setCursorMoveStyle(self as *mut ::text_layout::TextLayout,
                                                     style as *const ::qt_core::qt::CursorMoveStyle)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::setFlags(int flags)```</span>
  ///
  ///
  pub fn set_flags(&mut self, flags: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QTextLayout_setFlags(self as *mut ::text_layout::TextLayout, flags) }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::setFont(const QFont& f)```</span>
  ///
  ///
  pub fn set_font(&mut self, f: &::font::Font) {
    unsafe {
      ::ffi::qt_gui_c_QTextLayout_setFont(self as *mut ::text_layout::TextLayout,
                                          f as *const ::font::Font)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::setFormats(const QVector<QTextLayout::FormatRange>& overrides)```</span>
  ///
  ///
  pub fn set_formats(&mut self, overrides: &::vector::VectorTextLayoutFormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QTextLayout_setFormats(self as *mut ::text_layout::TextLayout,
                                             overrides as *const ::vector::VectorTextLayoutFormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::setPosition(const QPointF& p)```</span>
  ///
  ///
  pub fn set_position(&mut self, p: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTextLayout_setPosition(self as *mut ::text_layout::TextLayout,
                                              p as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::setPreeditArea(int position, const QString& text)```</span>
  ///
  ///
  pub fn set_preedit_area(&mut self, position: ::libc::c_int, text: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextLayout_setPreeditArea(self as *mut ::text_layout::TextLayout,
                                                 position,
                                                 text as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::setRawFont(const QRawFont& rawFont)```</span>
  ///
  ///
  pub fn set_raw_font(&mut self, raw_font: &::raw_font::RawFont) {
    unsafe {
      ::ffi::qt_gui_c_QTextLayout_setRawFont(self as *mut ::text_layout::TextLayout,
                                             raw_font as *const ::raw_font::RawFont)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::setText(const QString& string)```</span>
  ///
  ///
  pub fn set_text(&mut self, string: &::qt_core::string::String) {
    unsafe {
      ::ffi::qt_gui_c_QTextLayout_setText(self as *mut ::text_layout::TextLayout,
                                          string as *const ::qt_core::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextLayout::setTextOption(const QTextOption& option)```</span>
  ///
  ///
  pub fn set_text_option(&mut self, option: &::text_option::TextOption) {
    unsafe {
      ::ffi::qt_gui_c_QTextLayout_setTextOption(self as *mut ::text_layout::TextLayout,
                                                option as *const ::text_option::TextOption)
    }
  }

  /// C++ method: <span style='color: green;'>```QString QTextLayout::text() const```</span>
  ///
  ///
  pub fn text(&self) -> ::qt_core::string::String {
    {
      let mut object: ::qt_core::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_text_to_output(self as *const ::text_layout::TextLayout, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextOption& QTextLayout::textOption() const```</span>
  ///
  ///
  pub fn text_option<'l0>(&'l0 self) -> &'l0 ::text_option::TextOption {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QTextLayout_textOption(self as *const ::text_layout::TextLayout) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::text_layout::TextLayout {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QTextLayout_delete
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextLayout::draw](../struct.TextLayout.html#method.draw) method.
  pub trait TextLayoutDrawArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ();
  }
  impl<'largs> TextLayoutDrawArgs<'largs> for (*mut ::painter::Painter, &'largs ::qt_core::point_f::PointF) {
    unsafe fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> () {
      let p = self.0;
      let pos = self.1;
      ::ffi::qt_gui_c_QTextLayout_draw_p_pos(original_self as *const ::text_layout::TextLayout,
                                             p,
                                             pos as *const ::qt_core::point_f::PointF)
    }
  }
  impl<'largs> TextLayoutDrawArgs<'largs>
    for (*mut ::painter::Painter, &'largs ::qt_core::point_f::PointF, &'largs ::vector::VectorTextLayoutFormatRange) {
    unsafe fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> () {
      let p = self.0;
      let pos = self.1;
      let selections = self.2;
      ::ffi::qt_gui_c_QTextLayout_draw_p_pos_selections(original_self as *const ::text_layout::TextLayout,
                                                        p,
                                                        pos as *const ::qt_core::point_f::PointF,
                                                        selections as *const ::vector::VectorTextLayoutFormatRange)
    }
  }
  impl<'largs> TextLayoutDrawArgs<'largs>
    for (*mut ::painter::Painter,
                                                   &'largs ::qt_core::point_f::PointF,
                                                   &'largs ::vector::VectorTextLayoutFormatRange,
                                                   &'largs ::qt_core::rect_f::RectF) {
    unsafe fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> () {
      let p = self.0;
      let pos = self.1;
      let selections = self.2;
      let clip = self.3;
      ::ffi::qt_gui_c_QTextLayout_draw_p_pos_selections_clip(original_self as *const ::text_layout::TextLayout, p, pos as *const ::qt_core::point_f::PointF, selections as *const ::vector::VectorTextLayoutFormatRange, clip as *const ::qt_core::rect_f::RectF)
    }
  }
  /// This trait represents a set of arguments accepted by [TextLayout::draw_cursor](../struct.TextLayout.html#method.draw_cursor) method.
  pub trait TextLayoutDrawCursorArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ();
  }
  impl<'largs> TextLayoutDrawCursorArgs<'largs>
    for (*mut ::painter::Painter, &'largs ::qt_core::point_f::PointF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> () {
      let p = self.0;
      let pos = self.1;
      let cursor_position = self.2;
      ::ffi::qt_gui_c_QTextLayout_drawCursor_p_pos_cursorPosition(original_self as *const ::text_layout::TextLayout,
                                                                  p,
                                                                  pos as *const ::qt_core::point_f::PointF,
                                                                  cursor_position)
    }
  }
  impl<'largs> TextLayoutDrawCursorArgs<'largs>
    for (*mut ::painter::Painter, &'largs ::qt_core::point_f::PointF, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> () {
      let p = self.0;
      let pos = self.1;
      let cursor_position = self.2;
      let width = self.3;
      ::ffi::qt_gui_c_QTextLayout_drawCursor_p_pos_cursorPosition_width(original_self as *const ::text_layout::TextLayout, p, pos as *const ::qt_core::point_f::PointF, cursor_position, width)
    }
  }
  /// This trait represents a set of arguments accepted by [TextLayout::glyph_runs](../struct.TextLayout.html#method.glyph_runs) method.
  pub trait TextLayoutGlyphRunsArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ::list::ListGlyphRun;
  }
  impl<'largs> TextLayoutGlyphRunsArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ::list::ListGlyphRun {
      let from = self;
      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextLayout_glyphRuns_to_output_from(original_self as *const ::text_layout::TextLayout,
                                                               from,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextLayoutGlyphRunsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ::list::ListGlyphRun {
      let from = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextLayout_glyphRuns_to_output_from_length(original_self as *const ::text_layout::TextLayout, from, length, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextLayoutGlyphRunsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ::list::ListGlyphRun {

      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextLayout_glyphRuns_to_output_no_args(original_self as *const ::text_layout::TextLayout,
                                                                  &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextLayout::new](../struct.TextLayout.html#method.new) method.
  pub trait TextLayoutNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::text_layout::TextLayout>;
  }
  impl<'a> TextLayoutNewArgs for &'a ::text_block::TextBlock {
    fn exec(self) -> ::cpp_utils::CppBox<::text_layout::TextLayout> {
      let b = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextLayout_new_b(b as *const ::text_block::TextBlock) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl TextLayoutNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::text_layout::TextLayout> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextLayout_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TextLayoutNewArgs for &'a ::qt_core::string::String {
    fn exec(self) -> ::cpp_utils::CppBox<::text_layout::TextLayout> {
      let text = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QTextLayout_new_text(text as *const ::qt_core::string::String) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> TextLayoutNewArgs for (&'a ::qt_core::string::String, &'a ::font::Font) {
    fn exec(self) -> ::cpp_utils::CppBox<::text_layout::TextLayout> {
      let text = self.0;
      let font = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QTextLayout_new_text_font(text as *const ::qt_core::string::String,
                                                  font as *const ::font::Font)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [TextLayout::next_cursor_position](../struct.TextLayout.html#method.next_cursor_position) method.
  pub trait TextLayoutNextCursorPositionArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ::libc::c_int;
  }
  impl<'largs> TextLayoutNextCursorPositionArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ::libc::c_int {
      let old_pos = self;
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_nextCursorPosition_oldPos(original_self as *const ::text_layout::TextLayout,
                                                              old_pos)
      }
    }
  }
  impl<'largs> TextLayoutNextCursorPositionArgs<'largs> for (::libc::c_int, ::text_layout::CursorMode) {
    fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ::libc::c_int {
      let old_pos = self.0;
      let mode = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_nextCursorPosition_oldPos_mode(original_self as *const ::text_layout::TextLayout,
                                                                   old_pos,
                                                                   mode)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextLayout::previous_cursor_position](../struct.TextLayout.html#method.previous_cursor_position) method.
  pub trait TextLayoutPreviousCursorPositionArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ::libc::c_int;
  }
  impl<'largs> TextLayoutPreviousCursorPositionArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ::libc::c_int {
      let old_pos = self;
      unsafe {
        ::ffi::qt_gui_c_QTextLayout_previousCursorPosition_oldPos(original_self as *const ::text_layout::TextLayout,
                                                                  old_pos)
      }
    }
  }
  impl<'largs> TextLayoutPreviousCursorPositionArgs<'largs> for (::libc::c_int, ::text_layout::CursorMode) {
    fn exec(self, original_self: &'largs ::text_layout::TextLayout) -> ::libc::c_int {
      let old_pos = self.0;
      let mode = self.1;
      unsafe { ::ffi::qt_gui_c_QTextLayout_previousCursorPosition_oldPos_mode(original_self as *const ::text_layout::TextLayout, old_pos, mode) }
    }
  }
}
