/// C++ type: <span style='color: green;'>```QTextLine::CursorPosition```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CursorPosition {
  /// C++ enum variant: <span style='color: green;'>```CursorBetweenCharacters = 0```</span>
  BetweenCharacters = 0,
  /// C++ enum variant: <span style='color: green;'>```CursorOnCharacter = 1```</span>
  OnCharacter = 1,
}

/// C++ type: <span style='color: green;'>```QTextLine::Edge```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Edge {
  /// C++ enum variant: <span style='color: green;'>```Leading = 0```</span>
  Leading = 0,
  /// C++ enum variant: <span style='color: green;'>```Trailing = 1```</span>
  Trailing = 1,
}

/// C++ type: <span style='color: green;'>```QTextLine```</span>
#[repr(C)]
pub struct TextLine([u8; ::type_sizes::QT_GUI_TEXT_LINE_TEXT_LINE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for TextLine {
  unsafe fn new_uninitialized() -> TextLine {
    TextLine(::std::mem::uninitialized())
  }
}

impl TextLine {
  /// C++ method: <span style='color: green;'>```double QTextLine::ascent() const```</span>
  ///
  ///
  pub fn ascent(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLine_ascent(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```QTextLine::cursorToX```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn cursor_to_x(&self, ::libc::c_int) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QTextLine::cursorToX(int cursorPos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn cursor_to_x(&self, (::libc::c_int, ::text_line::Edge)) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QTextLine::cursorToX(int cursorPos, QTextLine::Edge edge = ?) const```</span>
  ///
  ///
  pub fn cursor_to_x<'largs, Args>(&'largs self, args: Args) -> ::libc::c_double
    where Args: overloading::TextLineCursorToXArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextLine::cursorToX```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn cursor_to_x_unsafe(&self, *mut ::libc::c_int) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QTextLine::cursorToX(int* cursorPos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn cursor_to_x_unsafe(&self, (*mut ::libc::c_int, ::text_line::Edge)) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QTextLine::cursorToX(int* cursorPos, QTextLine::Edge edge = ?) const```</span>
  ///
  ///
  pub unsafe fn cursor_to_x_unsafe<'largs, Args>(&'largs self, args: Args) -> ::libc::c_double
    where Args: overloading::TextLineCursorToXUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QTextLine::descent() const```</span>
  ///
  ///
  pub fn descent(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLine_descent(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```QTextLine::draw```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw(&self, (*mut ::painter::Painter, &::qt_core::point_f::PointF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextLine::draw(QPainter* p, const QPointF& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw(&self, (*mut ::painter::Painter, &::qt_core::point_f::PointF, *const ::text_layout::FormatRange)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextLine::draw(QPainter* p, const QPointF& point, const QTextLayout::FormatRange* selection = ?) const```</span>
  ///
  ///
  pub unsafe fn draw<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::TextLineDrawArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextLine::glyphRuns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn glyph_runs(&self, ()) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QTextLine::glyphRuns() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn glyph_runs(&self, ::libc::c_int) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QTextLine::glyphRuns(int from = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn glyph_runs(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QTextLine::glyphRuns(int from = ?, int length = ?) const```</span>
  ///
  ///
  pub fn glyph_runs<'largs, Args>(&'largs self, args: Args) -> ::list::ListGlyphRun
    where Args: overloading::TextLineGlyphRunsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QTextLine::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLine_height(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```double QTextLine::horizontalAdvance() const```</span>
  ///
  ///
  pub fn horizontal_advance(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLine_horizontalAdvance(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextLine::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextLine_isValid(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```double QTextLine::leading() const```</span>
  ///
  ///
  pub fn leading(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLine_leading(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```bool QTextLine::leadingIncluded() const```</span>
  ///
  ///
  pub fn leading_included(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QTextLine_leadingIncluded(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```int QTextLine::lineNumber() const```</span>
  ///
  ///
  pub fn line_number(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextLine_lineNumber(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```QRectF QTextLine::naturalTextRect() const```</span>
  ///
  ///
  pub fn natural_text_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLine_naturalTextRect_to_output(self as *const ::text_line::TextLine, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QTextLine::naturalTextWidth() const```</span>
  ///
  ///
  pub fn natural_text_width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLine_naturalTextWidth(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QTextLine::QTextLine()```</span>
  ///
  ///
  pub fn new() -> ::text_line::TextLine {
    {
      let mut object: ::text_line::TextLine =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLine_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QTextLine::position() const```</span>
  ///
  ///
  pub fn position(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLine_position_to_output(self as *const ::text_line::TextLine, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QTextLine::rect() const```</span>
  ///
  ///
  pub fn rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QTextLine_rect_to_output(self as *const ::text_line::TextLine, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QTextLine::setLeadingIncluded(bool included)```</span>
  ///
  ///
  pub fn set_leading_included(&mut self, included: bool) {
    unsafe { ::ffi::qt_gui_c_QTextLine_setLeadingIncluded(self as *mut ::text_line::TextLine, included) }
  }

  /// C++ method: <span style='color: green;'>```void QTextLine::setLineWidth(double width)```</span>
  ///
  ///
  pub fn set_line_width(&mut self, width: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QTextLine_setLineWidth(self as *mut ::text_line::TextLine, width) }
  }

  /// C++ method: <span style='color: green;'>```QTextLine::setNumColumns```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_num_columns(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextLine::setNumColumns(int columns)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_num_columns(&mut self, (::libc::c_int, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QTextLine::setNumColumns(int columns, double alignmentWidth)```</span>
  ///
  ///
  pub fn set_num_columns<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::TextLineSetNumColumnsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QTextLine::setPosition(const QPointF& pos)```</span>
  ///
  ///
  pub fn set_position(&mut self, pos: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QTextLine_setPosition(self as *mut ::text_line::TextLine,
                                            pos as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```int QTextLine::textLength() const```</span>
  ///
  ///
  pub fn text_length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextLine_textLength(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```int QTextLine::textStart() const```</span>
  ///
  ///
  pub fn text_start(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QTextLine_textStart(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```double QTextLine::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLine_width(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```double QTextLine::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLine_x(self as *const ::text_line::TextLine) }
  }

  /// C++ method: <span style='color: green;'>```QTextLine::xToCursor```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn x_to_cursor(&self, ::libc::c_double) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTextLine::xToCursor(double x) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn x_to_cursor(&self, (::libc::c_double, ::text_line::CursorPosition)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QTextLine::xToCursor(double x, QTextLine::CursorPosition arg2 = ?) const```</span>
  ///
  ///
  pub fn x_to_cursor<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::TextLineXToCursorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QTextLine::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QTextLine_y(self as *const ::text_line::TextLine) }
  }
}

impl Drop for ::text_line::TextLine {
  /// C++ method: <span style='color: green;'>```[destructor] void QTextLine::~QTextLine()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QTextLine_destructor(self as *mut ::text_line::TextLine) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [TextLine::cursor_to_x](../struct.TextLine.html#method.cursor_to_x) method.
  pub trait TextLineCursorToXArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::libc::c_double;
  }
  impl<'largs> TextLineCursorToXArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::libc::c_double {
      let cursor_pos = self;
      unsafe { ::ffi::qt_gui_c_QTextLine_cursorToX_int(original_self as *const ::text_line::TextLine, cursor_pos) }
    }
  }
  impl<'largs> TextLineCursorToXArgs<'largs> for (::libc::c_int, ::text_line::Edge) {
    fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::libc::c_double {
      let cursor_pos = self.0;
      let edge = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextLine_cursorToX_int_QTextLine_Edge(original_self as *const ::text_line::TextLine,
                                                               cursor_pos,
                                                               edge)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextLine::cursor_to_x_unsafe](../struct.TextLine.html#method.cursor_to_x_unsafe) method.
  pub trait TextLineCursorToXUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::libc::c_double;
  }
  impl<'largs> TextLineCursorToXUnsafeArgs<'largs> for *mut ::libc::c_int {
    unsafe fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::libc::c_double {
      let cursor_pos = self;
      ::ffi::qt_gui_c_QTextLine_cursorToX_int_ptr(original_self as *const ::text_line::TextLine, cursor_pos)
    }
  }
  impl<'largs> TextLineCursorToXUnsafeArgs<'largs> for (*mut ::libc::c_int, ::text_line::Edge) {
    unsafe fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::libc::c_double {
      let cursor_pos = self.0;
      let edge = self.1;
      ::ffi::qt_gui_c_QTextLine_cursorToX_int_ptr_QTextLine_Edge(original_self as *const ::text_line::TextLine,
                                                                 cursor_pos,
                                                                 edge)
    }
  }
  /// This trait represents a set of arguments accepted by [TextLine::draw](../struct.TextLine.html#method.draw) method.
  pub trait TextLineDrawArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::text_line::TextLine) -> ();
  }
  impl<'largs> TextLineDrawArgs<'largs> for (*mut ::painter::Painter, &'largs ::qt_core::point_f::PointF) {
    unsafe fn exec(self, original_self: &'largs ::text_line::TextLine) -> () {
      let p = self.0;
      let point = self.1;
      ::ffi::qt_gui_c_QTextLine_draw_p_point(original_self as *const ::text_line::TextLine,
                                             p,
                                             point as *const ::qt_core::point_f::PointF)
    }
  }
  impl<'largs> TextLineDrawArgs<'largs>
    for (*mut ::painter::Painter, &'largs ::qt_core::point_f::PointF, *const ::text_layout::FormatRange) {
    unsafe fn exec(self, original_self: &'largs ::text_line::TextLine) -> () {
      let p = self.0;
      let point = self.1;
      let selection = self.2;
      ::ffi::qt_gui_c_QTextLine_draw_p_point_selection(original_self as *const ::text_line::TextLine,
                                                       p,
                                                       point as *const ::qt_core::point_f::PointF,
                                                       selection)
    }
  }
  /// This trait represents a set of arguments accepted by [TextLine::glyph_runs](../struct.TextLine.html#method.glyph_runs) method.
  pub trait TextLineGlyphRunsArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::list::ListGlyphRun;
  }
  impl<'largs> TextLineGlyphRunsArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::list::ListGlyphRun {
      let from = self;
      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextLine_glyphRuns_to_output_from(original_self as *const ::text_line::TextLine,
                                                             from,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextLineGlyphRunsArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::list::ListGlyphRun {
      let from = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextLine_glyphRuns_to_output_from_length(original_self as *const ::text_line::TextLine,
                                                                    from,
                                                                    length,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> TextLineGlyphRunsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::list::ListGlyphRun {

      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QTextLine_glyphRuns_to_output_no_args(original_self as *const ::text_line::TextLine,
                                                                &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextLine::set_num_columns](../struct.TextLine.html#method.set_num_columns) method.
  pub trait TextLineSetNumColumnsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::text_line::TextLine) -> ();
  }
  impl<'largs> TextLineSetNumColumnsArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::text_line::TextLine) -> () {
      let columns = self;
      unsafe { ::ffi::qt_gui_c_QTextLine_setNumColumns_columns(original_self as *mut ::text_line::TextLine, columns) }
    }
  }
  impl<'largs> TextLineSetNumColumnsArgs<'largs> for (::libc::c_int, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::text_line::TextLine) -> () {
      let columns = self.0;
      let alignment_width = self.1;
      unsafe {
        ::ffi::qt_gui_c_QTextLine_setNumColumns_columns_alignmentWidth(original_self as *mut ::text_line::TextLine,
                                                                       columns,
                                                                       alignment_width)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [TextLine::x_to_cursor](../struct.TextLine.html#method.x_to_cursor) method.
  pub trait TextLineXToCursorArgs<'largs> {
    fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::libc::c_int;
  }
  impl<'largs> TextLineXToCursorArgs<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::libc::c_int {
      let x = self;
      unsafe { ::ffi::qt_gui_c_QTextLine_xToCursor_x(original_self as *const ::text_line::TextLine, x) }
    }
  }
  impl<'largs> TextLineXToCursorArgs<'largs> for (::libc::c_double, ::text_line::CursorPosition) {
    fn exec(self, original_self: &'largs ::text_line::TextLine) -> ::libc::c_int {
      let x = self.0;
      let arg2 = self.1;
      unsafe { ::ffi::qt_gui_c_QTextLine_xToCursor_x_arg2(original_self as *const ::text_line::TextLine, x, arg2) }
    }
  }
}
