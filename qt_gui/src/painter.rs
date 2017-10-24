/// C++ type: <span style='color: green;'>```QPainter::CompositionMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum CompositionMode {
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_SourceOver = 0```</span>
  CompositionModeSourceOver = 0,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_DestinationOver = 1```</span>
  CompositionModeDestinationOver = 1,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Clear = 2```</span>
  CompositionModeClear = 2,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Source = 3```</span>
  CompositionModeSource = 3,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Destination = 4```</span>
  CompositionModeDestination = 4,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_SourceIn = 5```</span>
  CompositionModeSourceIn = 5,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_DestinationIn = 6```</span>
  CompositionModeDestinationIn = 6,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_SourceOut = 7```</span>
  CompositionModeSourceOut = 7,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_DestinationOut = 8```</span>
  CompositionModeDestinationOut = 8,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_SourceAtop = 9```</span>
  CompositionModeSourceAtop = 9,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_DestinationAtop = 10```</span>
  CompositionModeDestinationAtop = 10,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Xor = 11```</span>
  CompositionModeXor = 11,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Plus = 12```</span>
  CompositionModePlus = 12,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Multiply = 13```</span>
  CompositionModeMultiply = 13,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Screen = 14```</span>
  CompositionModeScreen = 14,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Overlay = 15```</span>
  CompositionModeOverlay = 15,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Darken = 16```</span>
  CompositionModeDarken = 16,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Lighten = 17```</span>
  CompositionModeLighten = 17,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_ColorDodge = 18```</span>
  CompositionModeColorDodge = 18,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_ColorBurn = 19```</span>
  CompositionModeColorBurn = 19,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_HardLight = 20```</span>
  CompositionModeHardLight = 20,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_SoftLight = 21```</span>
  CompositionModeSoftLight = 21,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Difference = 22```</span>
  CompositionModeDifference = 22,
  /// C++ enum variant: <span style='color: green;'>```CompositionMode_Exclusion = 23```</span>
  CompositionModeExclusion = 23,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_SourceOrDestination = 24```</span>
  RasterOpSourceOrDestination = 24,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_SourceAndDestination = 25```</span>
  RasterOpSourceAndDestination = 25,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_SourceXorDestination = 26```</span>
  RasterOpSourceXorDestination = 26,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_NotSourceAndNotDestination = 27```</span>
  RasterOpNotSourceAndNotDestination = 27,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_NotSourceOrNotDestination = 28```</span>
  RasterOpNotSourceOrNotDestination = 28,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_NotSourceXorDestination = 29```</span>
  RasterOpNotSourceXorDestination = 29,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_NotSource = 30```</span>
  RasterOpNotSource = 30,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_NotSourceAndDestination = 31```</span>
  RasterOpNotSourceAndDestination = 31,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_SourceAndNotDestination = 32```</span>
  RasterOpSourceAndNotDestination = 32,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_NotSourceOrDestination = 33```</span>
  RasterOpNotSourceOrDestination = 33,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_SourceOrNotDestination = 34```</span>
  RasterOpSourceOrNotDestination = 34,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_ClearDestination = 35```</span>
  RasterOpClearDestination = 35,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_SetDestination = 36```</span>
  RasterOpSetDestination = 36,
  /// C++ enum variant: <span style='color: green;'>```RasterOp_NotDestination = 37```</span>
  RasterOpNotDestination = 37,
}

/// C++ type: <span style='color: green;'>```QPainter```</span>
#[repr(C)]
pub struct Painter(u8);

impl Painter {
  /// C++ method: <span style='color: green;'>```const QBrush& QPainter::background() const```</span>
  ///
  ///
  pub fn background<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_background(self as *const ::painter::Painter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QPainter::begin(QPaintDevice* arg1)```</span>
  ///
  ///
  pub unsafe fn begin(&mut self, arg1: *mut ::paint_device::PaintDevice) -> bool {
    ::ffi::qt_gui_c_QPainter_begin(self as *mut ::painter::Painter, arg1)
  }

  /// C++ method: <span style='color: green;'>```void QPainter::beginNativePainting()```</span>
  ///
  ///
  pub fn begin_native_painting(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPainter_beginNativePainting(self as *mut ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```QPainter::boundingRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn bounding_rect(&mut self, (&::qt_core::rect::Rect, ::libc::c_int, &::qt_core::string::String)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QPainter::boundingRect(const QRect& rect, int flags, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn bounding_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_core::string::String)) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QPainter::boundingRect(int x, int y, int w, int h, int flags, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn bounding_rect(&mut self, (&::qt_core::rect_f::RectF, &::qt_core::string::String)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QPainter::boundingRect(const QRectF& rect, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn bounding_rect(&mut self, (&::qt_core::rect_f::RectF, &::qt_core::string::String, &::text_option::TextOption)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QPainter::boundingRect(const QRectF& rect, const QString& text, const QTextOption& o = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn bounding_rect(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int, &::qt_core::string::String)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QPainter::boundingRect(const QRectF& rect, int flags, const QString& text)```</span>
  ///
  ///
  pub fn bounding_rect<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::PainterBoundingRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QBrush& QPainter::brush() const```</span>
  ///
  ///
  pub fn brush<'l0>(&'l0 self) -> &'l0 ::brush::Brush {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_brush(self as *const ::painter::Painter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint QPainter::brushOrigin() const```</span>
  ///
  ///
  pub fn brush_origin(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainter_brushOrigin_to_output(self as *const ::painter::Painter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QPainter::clipBoundingRect() const```</span>
  ///
  ///
  pub fn clip_bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainter_clipBoundingRect_to_output(self as *const ::painter::Painter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QPainter::clipPath() const```</span>
  ///
  ///
  pub fn clip_path(&self) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainter_clipPath_to_output(self as *const ::painter::Painter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRegion QPainter::clipRegion() const```</span>
  ///
  ///
  pub fn clip_region(&self) -> ::cpp_utils::CppBox<::region::Region> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_clipRegion_as_ptr(self as *const ::painter::Painter) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix QPainter::combinedMatrix() const```</span>
  ///
  ///
  pub fn combined_matrix(&self) -> ::matrix::Matrix {
    {
      let mut object: ::matrix::Matrix =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainter_combinedMatrix_to_output(self as *const ::painter::Painter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTransform QPainter::combinedTransform() const```</span>
  ///
  ///
  pub fn combined_transform(&self) -> ::transform::Transform {
    {
      let mut object: ::transform::Transform =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainter_combinedTransform_to_output(self as *const ::painter::Painter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainter::CompositionMode QPainter::compositionMode() const```</span>
  ///
  ///
  pub fn composition_mode(&self) -> ::painter::CompositionMode {
    unsafe { ::ffi::qt_gui_c_QPainter_compositionMode(self as *const ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```QPaintDevice* QPainter::device() const```</span>
  ///
  ///
  pub fn device(&self) -> *mut ::paint_device::PaintDevice {
    unsafe { ::ffi::qt_gui_c_QPainter_device(self as *const ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```const QMatrix& QPainter::deviceMatrix() const```</span>
  ///
  ///
  pub fn device_matrix<'l0>(&'l0 self) -> &'l0 ::matrix::Matrix {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_deviceMatrix(self as *const ::painter::Painter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTransform& QPainter::deviceTransform() const```</span>
  ///
  ///
  pub fn device_transform<'l0>(&'l0 self) -> &'l0 ::transform::Transform {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_deviceTransform(self as *const ::painter::Painter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPainter::drawArc```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_arc(&mut self, (&::qt_core::rect::Rect, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawArc(const QRect& arg1, int a, int alen)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_arc(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawArc(const QRectF& rect, int a, int alen)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_arc(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawArc(int x, int y, int w, int h, int a, int alen)```</span>
  ///
  ///
  pub fn draw_arc<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawArcArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawChord```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_chord(&mut self, (&::qt_core::rect::Rect, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawChord(const QRect& arg1, int a, int alen)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_chord(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawChord(const QRectF& rect, int a, int alen)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_chord(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawChord(int x, int y, int w, int h, int a, int alen)```</span>
  ///
  ///
  pub fn draw_chord<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawChordArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawConvexPolygon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_convex_polygon(&mut self, &::polygon::Polygon) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawConvexPolygon(const QPolygon& polygon)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_convex_polygon(&mut self, &::polygon_f::PolygonF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawConvexPolygon(const QPolygonF& polygon)```</span>
  ///
  ///
  pub fn draw_convex_polygon<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawConvexPolygonArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawConvexPolygon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_convex_polygon_unsafe(&mut self, (*const ::qt_core::point::Point, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawConvexPolygon(const QPoint* points, int pointCount)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_convex_polygon_unsafe(&mut self, (*const ::qt_core::point_f::PointF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawConvexPolygon(const QPointF* points, int pointCount)```</span>
  ///
  ///
  pub unsafe fn draw_convex_polygon_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawConvexPolygonUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawEllipse```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_ellipse(&mut self, (&::qt_core::point::Point, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawEllipse(const QPoint& center, int rx, int ry)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_ellipse(&mut self, (&::qt_core::point_f::PointF, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawEllipse(const QPointF& center, double rx, double ry)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_ellipse(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawEllipse(const QRect& r)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_ellipse(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawEllipse(const QRectF& r)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn draw_ellipse(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawEllipse(int x, int y, int w, int h)```</span>
  ///
  ///
  pub fn draw_ellipse<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawEllipseArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainter::drawGlyphRun(const QPointF& position, const QGlyphRun& glyphRun)```</span>
  ///
  ///
  pub fn draw_glyph_run(&mut self, position: &::qt_core::point_f::PointF, glyph_run: &::glyph_run::GlyphRun) {
    unsafe {
      ::ffi::qt_gui_c_QPainter_drawGlyphRun(self as *mut ::painter::Painter,
                                            position as *const ::qt_core::point_f::PointF,
                                            glyph_run as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainter::drawImage```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_image(&mut self, (&::qt_core::point::Point, &::image::Image)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawImage(const QPoint& p, const QImage& image)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_image(&mut self, (&::qt_core::point_f::PointF, &::image::Image)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawImage(const QPointF& p, const QImage& image)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_image(&mut self, (&::qt_core::rect::Rect, &::image::Image)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawImage(const QRect& r, const QImage& image)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_image(&mut self, (&::qt_core::rect_f::RectF, &::image::Image)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawImage(const QRectF& r, const QImage& image)```</span>
  ///
  ///
  pub fn draw_image<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawImageArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawLine```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_line(&mut self, &::qt_core::line::Line) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLine(const QLine& line)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_line(&mut self, &::qt_core::line_f::LineF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLine(const QLineF& line)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_line(&mut self, (&::qt_core::point::Point, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLine(const QPoint& p1, const QPoint& p2)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_line(&mut self, (&::qt_core::point_f::PointF, &::qt_core::point_f::PointF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLine(const QPointF& p1, const QPointF& p2)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn draw_line(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLine(int x1, int y1, int x2, int y2)```</span>
  ///
  ///
  pub fn draw_line<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawLineArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawLines```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_lines(&mut self, &::vector::VectorQtCoreLine) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLines(const QVector<QLine>& lines)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_lines(&mut self, &::vector::VectorQtCoreLineF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLines(const QVector<QLineF>& lines)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_lines(&mut self, &::vector::VectorQtCorePoint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLines(const QVector<QPoint>& pointPairs)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_lines(&mut self, &::qt_core::vector::VectorPointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLines(const QVector<QPointF>& pointPairs)```</span>
  ///
  ///
  pub fn draw_lines<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawLinesArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawLines```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_lines_unsafe(&mut self, (*const ::qt_core::line::Line, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLines(const QLine* lines, int lineCount)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_lines_unsafe(&mut self, (*const ::qt_core::line_f::LineF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLines(const QLineF* lines, int lineCount)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_lines_unsafe(&mut self, (*const ::qt_core::point::Point, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLines(const QPoint* pointPairs, int lineCount)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_lines_unsafe(&mut self, (*const ::qt_core::point_f::PointF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawLines(const QPointF* pointPairs, int lineCount)```</span>
  ///
  ///
  pub unsafe fn draw_lines_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawLinesUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainter::drawPath(const QPainterPath& path)```</span>
  ///
  ///
  pub fn draw_path(&mut self, path: &::painter_path::PainterPath) {
    unsafe {
      ::ffi::qt_gui_c_QPainter_drawPath(self as *mut ::painter::Painter,
                                        path as *const ::painter_path::PainterPath)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainter::drawPicture```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_picture(&mut self, (&::qt_core::point::Point, &::picture::Picture)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPicture(const QPoint& p, const QPicture& picture)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_picture(&mut self, (&::qt_core::point_f::PointF, &::picture::Picture)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPicture(const QPointF& p, const QPicture& picture)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_picture(&mut self, (::libc::c_int, ::libc::c_int, &::picture::Picture)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPicture(int x, int y, const QPicture& picture)```</span>
  ///
  ///
  pub fn draw_picture<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawPictureArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawPie```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_pie(&mut self, (&::qt_core::rect::Rect, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPie(const QRect& arg1, int a, int alen)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_pie(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPie(const QRectF& rect, int a, int alen)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_pie(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPie(int x, int y, int w, int h, int a, int alen)```</span>
  ///
  ///
  pub fn draw_pie<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawPieArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawPixmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_pixmap(&mut self, (&::qt_core::point::Point, &::pixmap::Pixmap)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmap(const QPoint& p, const QPixmap& pm)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_pixmap(&mut self, (&::qt_core::point::Point, &::pixmap::Pixmap, &::qt_core::rect::Rect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmap(const QPoint& p, const QPixmap& pm, const QRect& sr)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_pixmap(&mut self, (&::qt_core::point_f::PointF, &::pixmap::Pixmap)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmap(const QPointF& p, const QPixmap& pm)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_pixmap(&mut self, (&::qt_core::point_f::PointF, &::pixmap::Pixmap, &::qt_core::rect_f::RectF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmap(const QPointF& p, const QPixmap& pm, const QRectF& sr)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn draw_pixmap(&mut self, (&::qt_core::rect::Rect, &::pixmap::Pixmap)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmap(const QRect& r, const QPixmap& pm)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn draw_pixmap(&mut self, (&::qt_core::rect::Rect, &::pixmap::Pixmap, &::qt_core::rect::Rect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmap(const QRect& targetRect, const QPixmap& pixmap, const QRect& sourceRect)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn draw_pixmap(&mut self, (&::qt_core::rect_f::RectF, &::pixmap::Pixmap, &::qt_core::rect_f::RectF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmap(const QRectF& targetRect, const QPixmap& pixmap, const QRectF& sourceRect)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn draw_pixmap(&mut self, (::libc::c_int, ::libc::c_int, &::pixmap::Pixmap)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmap(int x, int y, const QPixmap& pm)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn draw_pixmap(&mut self, (::libc::c_int, ::libc::c_int, &::pixmap::Pixmap, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmap(int x, int y, const QPixmap& pm, int sx, int sy, int sw, int sh)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn draw_pixmap(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::pixmap::Pixmap)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap& pm)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn draw_pixmap(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::pixmap::Pixmap, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmap(int x, int y, int w, int h, const QPixmap& pm, int sx, int sy, int sw, int sh)```</span>
  ///
  ///
  pub fn draw_pixmap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawPixmapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawPixmapFragments```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_pixmap_fragments(&mut self, (*const ::painter::PixmapFragment, ::libc::c_int, &::pixmap::Pixmap)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmapFragments(const QPainter::PixmapFragment* fragments, int fragmentCount, const QPixmap& pixmap)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_pixmap_fragments(&mut self, (*const ::painter::PixmapFragment, ::libc::c_int, &::pixmap::Pixmap, ::qt_core::flags::Flags<::painter::PixmapFragmentHint>)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPixmapFragments(const QPainter::PixmapFragment* fragments, int fragmentCount, const QPixmap& pixmap, QFlags<QPainter::PixmapFragmentHint> hints = ?)```</span>
  ///
  ///
  pub unsafe fn draw_pixmap_fragments<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawPixmapFragmentsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawPoint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_point(&mut self, &::qt_core::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPoint(const QPoint& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_point(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPoint(const QPointF& pt)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_point(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPoint(int x, int y)```</span>
  ///
  ///
  pub fn draw_point<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawPointArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawPoints```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_points(&mut self, &::polygon::Polygon) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPoints(const QPolygon& points)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_points(&mut self, &::polygon_f::PolygonF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPoints(const QPolygonF& points)```</span>
  ///
  ///
  pub fn draw_points<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawPointsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawPoints```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_points_unsafe(&mut self, (*const ::qt_core::point::Point, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPoints(const QPoint* points, int pointCount)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_points_unsafe(&mut self, (*const ::qt_core::point_f::PointF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPoints(const QPointF* points, int pointCount)```</span>
  ///
  ///
  pub unsafe fn draw_points_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawPointsUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawPolygon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_polygon(&mut self, &::polygon::Polygon) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolygon(const QPolygon& polygon)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_polygon(&mut self, (&::polygon::Polygon, &::qt_core::qt::FillRule)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolygon(const QPolygon& polygon, Qt::FillRule fillRule = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_polygon(&mut self, &::polygon_f::PolygonF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolygon(const QPolygonF& polygon)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_polygon(&mut self, (&::polygon_f::PolygonF, &::qt_core::qt::FillRule)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolygon(const QPolygonF& polygon, Qt::FillRule fillRule = ?)```</span>
  ///
  ///
  pub fn draw_polygon<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawPolygonArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawPolygon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_polygon_unsafe(&mut self, (*const ::qt_core::point::Point, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolygon(const QPoint* points, int pointCount)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_polygon_unsafe(&mut self, (*const ::qt_core::point::Point, ::libc::c_int, &::qt_core::qt::FillRule)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolygon(const QPoint* points, int pointCount, Qt::FillRule fillRule = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_polygon_unsafe(&mut self, (*const ::qt_core::point_f::PointF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolygon(const QPointF* points, int pointCount)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_polygon_unsafe(&mut self, (*const ::qt_core::point_f::PointF, ::libc::c_int, &::qt_core::qt::FillRule)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolygon(const QPointF* points, int pointCount, Qt::FillRule fillRule = ?)```</span>
  ///
  ///
  pub unsafe fn draw_polygon_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawPolygonUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawPolyline```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_polyline(&mut self, &::polygon::Polygon) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolyline(const QPolygon& polygon)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_polyline(&mut self, &::polygon_f::PolygonF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolyline(const QPolygonF& polyline)```</span>
  ///
  ///
  pub fn draw_polyline<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawPolylineArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawPolyline```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_polyline_unsafe(&mut self, (*const ::qt_core::point::Point, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolyline(const QPoint* points, int pointCount)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_polyline_unsafe(&mut self, (*const ::qt_core::point_f::PointF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawPolyline(const QPointF* points, int pointCount)```</span>
  ///
  ///
  pub unsafe fn draw_polyline_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawPolylineUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_rect(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRect(const QRect& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_rect(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRect(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRect(int x1, int y1, int w, int h)```</span>
  ///
  ///
  pub fn draw_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawRects```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_rects(&mut self, &::vector::VectorQtCoreRect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRects(const QVector<QRect>& rectangles)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_rects(&mut self, &::vector::VectorQtCoreRectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRects(const QVector<QRectF>& rectangles)```</span>
  ///
  ///
  pub fn draw_rects<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawRectsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawRects```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_rects_unsafe(&mut self, (*const ::qt_core::rect::Rect, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRects(const QRect* rects, int rectCount)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_rects_unsafe(&mut self, (*const ::qt_core::rect_f::RectF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRects(const QRectF* rects, int rectCount)```</span>
  ///
  ///
  pub unsafe fn draw_rects_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawRectsUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawRoundRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_round_rect(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundRect(const QRect& r)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_round_rect(&mut self, (&::qt_core::rect::Rect, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundRect(const QRect& r, int xround = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_round_rect(&mut self, (&::qt_core::rect::Rect, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundRect(const QRect& r, int xround = ?, int yround = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_round_rect(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundRect(const QRectF& r)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn draw_round_rect(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundRect(const QRectF& r, int xround = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn draw_round_rect(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundRect(const QRectF& r, int xround = ?, int yround = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn draw_round_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundRect(int x, int y, int w, int h)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn draw_round_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundRect(int x, int y, int w, int h, int arg5 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn draw_round_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundRect(int x, int y, int w, int h, int arg5 = ?, int arg6 = ?)```</span>
  ///
  ///
  pub fn draw_round_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawRoundRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawRoundedRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_rounded_rect(&mut self, (&::qt_core::rect::Rect, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundedRect(const QRect& rect, double xRadius, double yRadius)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_rounded_rect(&mut self, (&::qt_core::rect::Rect, ::libc::c_double, ::libc::c_double, &::qt_core::qt::SizeMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundedRect(const QRect& rect, double xRadius, double yRadius, Qt::SizeMode mode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_rounded_rect(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundedRect(const QRectF& rect, double xRadius, double yRadius)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_rounded_rect(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double, &::qt_core::qt::SizeMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundedRect(const QRectF& rect, double xRadius, double yRadius, Qt::SizeMode mode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn draw_rounded_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundedRect(int x, int y, int w, int h, double xRadius, double yRadius)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn draw_rounded_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_double, ::libc::c_double, &::qt_core::qt::SizeMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawRoundedRect(int x, int y, int w, int h, double xRadius, double yRadius, Qt::SizeMode mode = ?)```</span>
  ///
  ///
  pub fn draw_rounded_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawRoundedRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawStaticText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_static_text(&mut self, (&::qt_core::point::Point, &::static_text::StaticText)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawStaticText(const QPoint& topLeftPosition, const QStaticText& staticText)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_static_text(&mut self, (&::qt_core::point_f::PointF, &::static_text::StaticText)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawStaticText(const QPointF& topLeftPosition, const QStaticText& staticText)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_static_text(&mut self, (::libc::c_int, ::libc::c_int, &::static_text::StaticText)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawStaticText(int left, int top, const QStaticText& staticText)```</span>
  ///
  ///
  pub fn draw_static_text<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawStaticTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_text(&mut self, (&::qt_core::point::Point, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(const QPoint& p, const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_text(&mut self, (&::qt_core::point_f::PointF, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(const QPointF& p, const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_text(&mut self, (&::qt_core::point_f::PointF, &::qt_core::string::String, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(const QPointF& p, const QString& str, int tf, int justificationPadding)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_text(&mut self, (&::qt_core::rect::Rect, ::libc::c_int, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(const QRect& r, int flags, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn draw_text(&mut self, (&::qt_core::rect_f::RectF, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(const QRectF& r, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn draw_text(&mut self, (&::qt_core::rect_f::RectF, &::qt_core::string::String, &::text_option::TextOption)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(const QRectF& r, const QString& text, const QTextOption& o = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn draw_text(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(const QRectF& r, int flags, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn draw_text(&mut self, (::libc::c_int, ::libc::c_int, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(int x, int y, const QString& s)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn draw_text(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(int x, int y, int w, int h, int flags, const QString& text)```</span>
  ///
  ///
  pub fn draw_text<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawTextItem```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_text_item(&mut self, (&::qt_core::point::Point, &::text_item::TextItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawTextItem(const QPoint& p, const QTextItem& ti)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_text_item(&mut self, (&::qt_core::point_f::PointF, &::text_item::TextItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawTextItem(const QPointF& p, const QTextItem& ti)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_text_item(&mut self, (::libc::c_int, ::libc::c_int, &::text_item::TextItem)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawTextItem(int x, int y, const QTextItem& ti)```</span>
  ///
  ///
  pub fn draw_text_item<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawTextItemArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_text_unsafe(&mut self, (&::qt_core::rect::Rect, ::libc::c_int, &::qt_core::string::String, *mut ::qt_core::rect::Rect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(const QRect& r, int flags, const QString& text, QRect* br = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_text_unsafe(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int, &::qt_core::string::String, *mut ::qt_core::rect_f::RectF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(const QRectF& r, int flags, const QString& text, QRectF* br = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_text_unsafe(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_core::string::String, *mut ::qt_core::rect::Rect)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawText(int x, int y, int w, int h, int flags, const QString& text, QRect* br = ?)```</span>
  ///
  ///
  pub unsafe fn draw_text_unsafe<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawTextUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::drawTiledPixmap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_tiled_pixmap(&mut self, (&::qt_core::rect::Rect, &::pixmap::Pixmap)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawTiledPixmap(const QRect& arg1, const QPixmap& arg2)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_tiled_pixmap(&mut self, (&::qt_core::rect::Rect, &::pixmap::Pixmap, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawTiledPixmap(const QRect& arg1, const QPixmap& arg2, const QPoint& arg3 = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn draw_tiled_pixmap(&mut self, (&::qt_core::rect_f::RectF, &::pixmap::Pixmap)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawTiledPixmap(const QRectF& rect, const QPixmap& pm)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn draw_tiled_pixmap(&mut self, (&::qt_core::rect_f::RectF, &::pixmap::Pixmap, &::qt_core::point_f::PointF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawTiledPixmap(const QRectF& rect, const QPixmap& pm, const QPointF& offset = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn draw_tiled_pixmap(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::pixmap::Pixmap)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawTiledPixmap(int x, int y, int w, int h, const QPixmap& arg5)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn draw_tiled_pixmap(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::pixmap::Pixmap, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawTiledPixmap(int x, int y, int w, int h, const QPixmap& arg5, int sx = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn draw_tiled_pixmap(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::pixmap::Pixmap, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::drawTiledPixmap(int x, int y, int w, int h, const QPixmap& arg5, int sx = ?, int sy = ?)```</span>
  ///
  ///
  pub fn draw_tiled_pixmap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterDrawTiledPixmapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QPainter::end()```</span>
  ///
  ///
  pub fn end(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPainter_end(self as *mut ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::endNativePainting()```</span>
  ///
  ///
  pub fn end_native_painting(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPainter_endNativePainting(self as *mut ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```QPainter::eraseRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn erase_rect(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::eraseRect(const QRect& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn erase_rect(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::eraseRect(const QRectF& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn erase_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::eraseRect(int x, int y, int w, int h)```</span>
  ///
  ///
  pub fn erase_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterEraseRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainter::fillPath(const QPainterPath& path, const QBrush& brush)```</span>
  ///
  ///
  pub fn fill_path(&mut self, path: &::painter_path::PainterPath, brush: &::brush::Brush) {
    unsafe {
      ::ffi::qt_gui_c_QPainter_fillPath(self as *mut ::painter::Painter,
                                        path as *const ::painter_path::PainterPath,
                                        brush as *const ::brush::Brush)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainter::fillRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (&::qt_core::rect::Rect, &::brush::Brush)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(const QRect& arg1, const QBrush& arg2)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (&::qt_core::rect::Rect, &::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(const QRect& arg1, const QColor& color)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (&::qt_core::rect::Rect, &::qt_core::qt::BrushStyle)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(const QRect& r, Qt::BrushStyle style)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (&::qt_core::rect::Rect, &::qt_core::qt::GlobalColor)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(const QRect& r, Qt::GlobalColor c)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (&::qt_core::rect_f::RectF, &::brush::Brush)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(const QRectF& arg1, const QBrush& arg2)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (&::qt_core::rect_f::RectF, &::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(const QRectF& arg1, const QColor& color)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (&::qt_core::rect_f::RectF, &::qt_core::qt::BrushStyle)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(const QRectF& r, Qt::BrushStyle style)```</span>
  ///
  ///
  ///
  /// ## Variant 8
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (&::qt_core::rect_f::RectF, &::qt_core::qt::GlobalColor)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(const QRectF& r, Qt::GlobalColor c)```</span>
  ///
  ///
  ///
  /// ## Variant 9
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_core::qt::BrushStyle)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(int x, int y, int w, int h, Qt::BrushStyle style)```</span>
  ///
  ///
  ///
  /// ## Variant 10
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_core::qt::GlobalColor)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(int x, int y, int w, int h, Qt::GlobalColor c)```</span>
  ///
  ///
  ///
  /// ## Variant 11
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::brush::Brush)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(int x, int y, int w, int h, const QBrush& arg5)```</span>
  ///
  ///
  ///
  /// ## Variant 12
  ///
  /// Rust arguments: ```fn fill_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::fillRect(int x, int y, int w, int h, const QColor& color)```</span>
  ///
  ///
  pub fn fill_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterFillRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QFont& QPainter::font() const```</span>
  ///
  ///
  pub fn font<'l0>(&'l0 self) -> &'l0 ::font::Font {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_font(self as *const ::painter::Painter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFontInfo QPainter::fontInfo() const```</span>
  ///
  ///
  pub fn font_info(&self) -> ::font_info::FontInfo {
    {
      let mut object: ::font_info::FontInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainter_fontInfo_to_output(self as *const ::painter::Painter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFontMetrics QPainter::fontMetrics() const```</span>
  ///
  ///
  pub fn font_metrics(&self) -> ::font_metrics::FontMetrics {
    {
      let mut object: ::font_metrics::FontMetrics =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainter_fontMetrics_to_output(self as *const ::painter::Painter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPainter::hasClipping() const```</span>
  ///
  ///
  pub fn has_clipping(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPainter_hasClipping(self as *const ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::initFrom(const QPaintDevice* device)```</span>
  ///
  ///
  pub unsafe fn init_from(&mut self, device: *const ::paint_device::PaintDevice) {
    ::ffi::qt_gui_c_QPainter_initFrom(self as *mut ::painter::Painter, device)
  }

  /// C++ method: <span style='color: green;'>```bool QPainter::isActive() const```</span>
  ///
  ///
  pub fn is_active(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPainter_isActive(self as *const ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```const QMatrix& QPainter::matrix() const```</span>
  ///
  ///
  pub fn matrix<'l0>(&'l0 self) -> &'l0 ::matrix::Matrix {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_matrix(self as *const ::painter::Painter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QPainter::matrixEnabled() const```</span>
  ///
  ///
  pub fn matrix_enabled(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPainter_matrixEnabled(self as *const ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPainter::QPainter()```</span>
  ///
  ///
  pub fn new() -> ::cpp_utils::CppBox<::painter::Painter> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_new_no_args() };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QPainter::QPainter(QPaintDevice* arg1)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(arg1: *mut ::paint_device::PaintDevice) -> ::cpp_utils::CppBox<::painter::Painter> {
    let ffi_result = ::ffi::qt_gui_c_QPainter_new_arg1(arg1);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```double QPainter::opacity() const```</span>
  ///
  ///
  pub fn opacity(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainter_opacity(self as *const ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```QPaintEngine* QPainter::paintEngine() const```</span>
  ///
  ///
  pub fn paint_engine(&self) -> *mut ::paint_engine::PaintEngine {
    unsafe { ::ffi::qt_gui_c_QPainter_paintEngine(self as *const ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```const QPen& QPainter::pen() const```</span>
  ///
  ///
  pub fn pen<'l0>(&'l0 self) -> &'l0 ::pen::Pen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_pen(self as *const ::painter::Painter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPainter::redirected```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn redirected(*const ::paint_device::PaintDevice) -> *mut ::paint_device::PaintDevice```<br>
  /// C++ method: <span style='color: green;'>```static QPaintDevice* QPainter::redirected(const QPaintDevice* device)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn redirected((*const ::paint_device::PaintDevice, *mut ::qt_core::point::Point)) -> *mut ::paint_device::PaintDevice```<br>
  /// C++ method: <span style='color: green;'>```static QPaintDevice* QPainter::redirected(const QPaintDevice* device, QPoint* offset = ?)```</span>
  ///
  ///
  pub unsafe fn redirected<Args>(args: Args) -> *mut ::paint_device::PaintDevice
    where Args: overloading::PainterRedirectedArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QFlags<QPainter::RenderHint> QPainter::renderHints() const```</span>
  ///
  ///
  pub fn render_hints(&self) -> ::qt_core::flags::Flags<::painter::RenderHint> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_renderHints(self as *const ::painter::Painter) };
    ::qt_core::flags::Flags::from_int(ffi_result as i32)
  }

  /// C++ method: <span style='color: green;'>```void QPainter::resetMatrix()```</span>
  ///
  ///
  pub fn reset_matrix(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPainter_resetMatrix(self as *mut ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::resetTransform()```</span>
  ///
  ///
  pub fn reset_transform(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPainter_resetTransform(self as *mut ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::restore()```</span>
  ///
  ///
  pub fn restore(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPainter_restore(self as *mut ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```static void QPainter::restoreRedirected(const QPaintDevice* device)```</span>
  ///
  ///
  pub unsafe fn restore_redirected(device: *const ::paint_device::PaintDevice) {
    ::ffi::qt_gui_c_QPainter_restoreRedirected(device)
  }

  /// C++ method: <span style='color: green;'>```void QPainter::rotate(double a)```</span>
  ///
  ///
  pub fn rotate(&mut self, a: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_rotate(self as *mut ::painter::Painter, a) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::save()```</span>
  ///
  ///
  pub fn save(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPainter_save(self as *mut ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::scale(double sx, double sy)```</span>
  ///
  ///
  pub fn scale(&mut self, sx: ::libc::c_double, sy: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_scale(self as *mut ::painter::Painter, sx, sy) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::setBackground(const QBrush& bg)```</span>
  ///
  ///
  pub fn set_background(&mut self, bg: &::brush::Brush) {
    unsafe { ::ffi::qt_gui_c_QPainter_setBackground(self as *mut ::painter::Painter, bg as *const ::brush::Brush) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::setBackgroundMode(Qt::BGMode mode)```</span>
  ///
  ///
  pub fn set_background_mode(&mut self, mode: &::qt_core::qt::BGMode) {
    unsafe {
      ::ffi::qt_gui_c_QPainter_setBackgroundMode(self as *mut ::painter::Painter,
                                                 mode as *const ::qt_core::qt::BGMode)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainter::setBrush```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_brush(&mut self, &::qt_core::qt::BrushStyle) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setBrush(Qt::BrushStyle style)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_brush(&mut self, &::brush::Brush) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setBrush(const QBrush& brush)```</span>
  ///
  ///
  pub fn set_brush<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetBrushArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::setBrushOrigin```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_brush_origin(&mut self, &::qt_core::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setBrushOrigin(const QPoint& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_brush_origin(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setBrushOrigin(const QPointF& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_brush_origin(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setBrushOrigin(int x, int y)```</span>
  ///
  ///
  pub fn set_brush_origin<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetBrushOriginArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::setClipPath```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_clip_path(&mut self, &::painter_path::PainterPath) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setClipPath(const QPainterPath& path)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_clip_path(&mut self, (&::painter_path::PainterPath, &::qt_core::qt::ClipOperation)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setClipPath(const QPainterPath& path, Qt::ClipOperation op = ?)```</span>
  ///
  ///
  pub fn set_clip_path<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetClipPathArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::setClipRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_clip_rect(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setClipRect(const QRect& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_clip_rect(&mut self, (&::qt_core::rect::Rect, &::qt_core::qt::ClipOperation)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setClipRect(const QRect& arg1, Qt::ClipOperation op = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_clip_rect(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setClipRect(const QRectF& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn set_clip_rect(&mut self, (&::qt_core::rect_f::RectF, &::qt_core::qt::ClipOperation)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setClipRect(const QRectF& arg1, Qt::ClipOperation op = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn set_clip_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setClipRect(int x, int y, int w, int h)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn set_clip_rect(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &::qt_core::qt::ClipOperation)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setClipRect(int x, int y, int w, int h, Qt::ClipOperation op = ?)```</span>
  ///
  ///
  pub fn set_clip_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetClipRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::setClipRegion```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_clip_region(&mut self, &::region::Region) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setClipRegion(const QRegion& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_clip_region(&mut self, (&::region::Region, &::qt_core::qt::ClipOperation)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setClipRegion(const QRegion& arg1, Qt::ClipOperation op = ?)```</span>
  ///
  ///
  pub fn set_clip_region<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetClipRegionArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainter::setClipping(bool enable)```</span>
  ///
  ///
  pub fn set_clipping(&mut self, enable: bool) {
    unsafe { ::ffi::qt_gui_c_QPainter_setClipping(self as *mut ::painter::Painter, enable) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::setCompositionMode(QPainter::CompositionMode mode)```</span>
  ///
  ///
  pub fn set_composition_mode(&mut self, mode: ::painter::CompositionMode) {
    unsafe { ::ffi::qt_gui_c_QPainter_setCompositionMode(self as *mut ::painter::Painter, mode) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::setFont(const QFont& f)```</span>
  ///
  ///
  pub fn set_font(&mut self, f: &::font::Font) {
    unsafe { ::ffi::qt_gui_c_QPainter_setFont(self as *mut ::painter::Painter, f as *const ::font::Font) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::setLayoutDirection(Qt::LayoutDirection direction)```</span>
  ///
  ///
  pub fn set_layout_direction(&mut self, direction: &::qt_core::qt::LayoutDirection) {
    unsafe {
      ::ffi::qt_gui_c_QPainter_setLayoutDirection(self as *mut ::painter::Painter,
                                                  direction as *const ::qt_core::qt::LayoutDirection)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainter::setMatrix```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_matrix(&mut self, &::matrix::Matrix) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setMatrix(const QMatrix& matrix)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_matrix(&mut self, (&::matrix::Matrix, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setMatrix(const QMatrix& matrix, bool combine = ?)```</span>
  ///
  ///
  pub fn set_matrix<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetMatrixArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainter::setMatrixEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_matrix_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_gui_c_QPainter_setMatrixEnabled(self as *mut ::painter::Painter, enabled) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::setOpacity(double opacity)```</span>
  ///
  ///
  pub fn set_opacity(&mut self, opacity: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_setOpacity(self as *mut ::painter::Painter, opacity) }
  }

  /// C++ method: <span style='color: green;'>```QPainter::setPen```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_pen(&mut self, &::qt_core::qt::PenStyle) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setPen(Qt::PenStyle style)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_pen(&mut self, &::color::Color) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setPen(const QColor& color)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn set_pen(&mut self, &::pen::Pen) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setPen(const QPen& pen)```</span>
  ///
  ///
  pub fn set_pen<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetPenArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::setRedirected```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_redirected((*const ::paint_device::PaintDevice, *mut ::paint_device::PaintDevice)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QPainter::setRedirected(const QPaintDevice* device, QPaintDevice* replacement)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_redirected((*const ::paint_device::PaintDevice, *mut ::paint_device::PaintDevice, &::qt_core::point::Point)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```static void QPainter::setRedirected(const QPaintDevice* device, QPaintDevice* replacement, const QPoint& offset = ?)```</span>
  ///
  ///
  pub unsafe fn set_redirected<Args>(args: Args) -> ()
    where Args: overloading::PainterSetRedirectedArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPainter::setRenderHint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_render_hint(&mut self, ::painter::RenderHint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setRenderHint(QPainter::RenderHint hint)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_render_hint(&mut self, (::painter::RenderHint, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setRenderHint(QPainter::RenderHint hint, bool on = ?)```</span>
  ///
  ///
  pub fn set_render_hint<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetRenderHintArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::setRenderHints```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_render_hints(&mut self, ::qt_core::flags::Flags<::painter::RenderHint>) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setRenderHints(QFlags<QPainter::RenderHint> hints)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_render_hints(&mut self, (::qt_core::flags::Flags<::painter::RenderHint>, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setRenderHints(QFlags<QPainter::RenderHint> hints, bool on = ?)```</span>
  ///
  ///
  pub fn set_render_hints<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetRenderHintsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::setTransform```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_transform(&mut self, &::transform::Transform) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setTransform(const QTransform& transform)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_transform(&mut self, (&::transform::Transform, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setTransform(const QTransform& transform, bool combine = ?)```</span>
  ///
  ///
  pub fn set_transform<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetTransformArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainter::setViewTransformEnabled(bool enable)```</span>
  ///
  ///
  pub fn set_view_transform_enabled(&mut self, enable: bool) {
    unsafe { ::ffi::qt_gui_c_QPainter_setViewTransformEnabled(self as *mut ::painter::Painter, enable) }
  }

  /// C++ method: <span style='color: green;'>```QPainter::setViewport```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_viewport(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setViewport(const QRect& viewport)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_viewport(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setViewport(int x, int y, int w, int h)```</span>
  ///
  ///
  pub fn set_viewport<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetViewportArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::setWindow```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_window(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setWindow(const QRect& window)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_window(&mut self, (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setWindow(int x, int y, int w, int h)```</span>
  ///
  ///
  pub fn set_window<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetWindowArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainter::setWorldMatrix```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_world_matrix(&mut self, &::matrix::Matrix) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setWorldMatrix(const QMatrix& matrix)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_world_matrix(&mut self, (&::matrix::Matrix, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setWorldMatrix(const QMatrix& matrix, bool combine = ?)```</span>
  ///
  ///
  pub fn set_world_matrix<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetWorldMatrixArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainter::setWorldMatrixEnabled(bool enabled)```</span>
  ///
  ///
  pub fn set_world_matrix_enabled(&mut self, enabled: bool) {
    unsafe { ::ffi::qt_gui_c_QPainter_setWorldMatrixEnabled(self as *mut ::painter::Painter, enabled) }
  }

  /// C++ method: <span style='color: green;'>```QPainter::setWorldTransform```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_world_transform(&mut self, &::transform::Transform) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setWorldTransform(const QTransform& matrix)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_world_transform(&mut self, (&::transform::Transform, bool)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::setWorldTransform(const QTransform& matrix, bool combine = ?)```</span>
  ///
  ///
  pub fn set_world_transform<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterSetWorldTransformArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainter::shear(double sh, double sv)```</span>
  ///
  ///
  pub fn shear(&mut self, sh: ::libc::c_double, sv: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_shear(self as *mut ::painter::Painter, sh, sv) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::strokePath(const QPainterPath& path, const QPen& pen)```</span>
  ///
  ///
  pub fn stroke_path(&mut self, path: &::painter_path::PainterPath, pen: &::pen::Pen) {
    unsafe {
      ::ffi::qt_gui_c_QPainter_strokePath(self as *mut ::painter::Painter,
                                          path as *const ::painter_path::PainterPath,
                                          pen as *const ::pen::Pen)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPainter::testRenderHint(QPainter::RenderHint hint) const```</span>
  ///
  ///
  pub fn test_render_hint(&self, hint: ::painter::RenderHint) -> bool {
    unsafe { ::ffi::qt_gui_c_QPainter_testRenderHint(self as *const ::painter::Painter, hint) }
  }

  /// C++ method: <span style='color: green;'>```const QTransform& QPainter::transform() const```</span>
  ///
  ///
  pub fn transform<'l0>(&'l0 self) -> &'l0 ::transform::Transform {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_transform(self as *const ::painter::Painter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPainter::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&mut self, &::qt_core::point::Point) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::translate(const QPoint& offset)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::translate(const QPointF& offset)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn translate(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainter::translate(double dx, double dy)```</span>
  ///
  ///
  pub fn translate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterTranslateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QPainter::viewTransformEnabled() const```</span>
  ///
  ///
  pub fn view_transform_enabled(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPainter_viewTransformEnabled(self as *const ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```QRect QPainter::viewport() const```</span>
  ///
  ///
  pub fn viewport(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainter_viewport_to_output(self as *const ::painter::Painter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRect QPainter::window() const```</span>
  ///
  ///
  pub fn window(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainter_window_to_output(self as *const ::painter::Painter, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QMatrix& QPainter::worldMatrix() const```</span>
  ///
  ///
  pub fn world_matrix<'l0>(&'l0 self) -> &'l0 ::matrix::Matrix {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_worldMatrix(self as *const ::painter::Painter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QPainter::worldMatrixEnabled() const```</span>
  ///
  ///
  pub fn world_matrix_enabled(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPainter_worldMatrixEnabled(self as *const ::painter::Painter) }
  }

  /// C++ method: <span style='color: green;'>```const QTransform& QPainter::worldTransform() const```</span>
  ///
  ///
  pub fn world_transform<'l0>(&'l0 self) -> &'l0 ::transform::Transform {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_worldTransform(self as *const ::painter::Painter) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }
}

impl ::cpp_utils::CppDeletable for ::painter::Painter {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPainter_delete
  }
}

/// C++ type: <span style='color: green;'>```QPainter::PixmapFragment```</span>
#[repr(C)]
pub struct PixmapFragment(u8);

impl PixmapFragment {
  /// C++ method: <span style='color: green;'>```QPainter::PixmapFragment::create```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create((&::qt_core::point_f::PointF, &::qt_core::rect_f::RectF)) -> ::cpp_utils::CppBox<::painter::PixmapFragment>```<br>
  /// C++ method: <span style='color: green;'>```static QPainter::PixmapFragment QPainter::PixmapFragment::create(const QPointF& pos, const QRectF& sourceRect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create((&::qt_core::point_f::PointF, &::qt_core::rect_f::RectF, ::libc::c_double)) -> ::cpp_utils::CppBox<::painter::PixmapFragment>```<br>
  /// C++ method: <span style='color: green;'>```static QPainter::PixmapFragment QPainter::PixmapFragment::create(const QPointF& pos, const QRectF& sourceRect, double scaleX = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn create((&::qt_core::point_f::PointF, &::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::painter::PixmapFragment>```<br>
  /// C++ method: <span style='color: green;'>```static QPainter::PixmapFragment QPainter::PixmapFragment::create(const QPointF& pos, const QRectF& sourceRect, double scaleX = ?, double scaleY = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn create((&::qt_core::point_f::PointF, &::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::painter::PixmapFragment>```<br>
  /// C++ method: <span style='color: green;'>```static QPainter::PixmapFragment QPainter::PixmapFragment::create(const QPointF& pos, const QRectF& sourceRect, double scaleX = ?, double scaleY = ?, double rotation = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn create((&::qt_core::point_f::PointF, &::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::cpp_utils::CppBox<::painter::PixmapFragment>```<br>
  /// C++ method: <span style='color: green;'>```static QPainter::PixmapFragment QPainter::PixmapFragment::create(const QPointF& pos, const QRectF& sourceRect, double scaleX = ?, double scaleY = ?, double rotation = ?, double opacity = ?)```</span>
  ///
  ///
  pub fn create<Args>(args: Args) -> ::cpp_utils::CppBox<::painter::PixmapFragment>
    where Args: overloading::PixmapFragmentCreateArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```double QPainter::PixmapFragment::height() const```</span>
  ///
  ///
  pub fn height(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_height(self as *const ::painter::PixmapFragment) }
  }

  /// C++ method: <span style='color: green;'>```double QPainter::PixmapFragment::opacity() const```</span>
  ///
  ///
  pub fn opacity(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_opacity(self as *const ::painter::PixmapFragment) }
  }

  /// C++ method: <span style='color: green;'>```double QPainter::PixmapFragment::rotation() const```</span>
  ///
  ///
  pub fn rotation(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_rotation(self as *const ::painter::PixmapFragment) }
  }

  /// C++ method: <span style='color: green;'>```double QPainter::PixmapFragment::scaleX() const```</span>
  ///
  ///
  pub fn scale_x(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_scaleX(self as *const ::painter::PixmapFragment) }
  }

  /// C++ method: <span style='color: green;'>```double QPainter::PixmapFragment::scaleY() const```</span>
  ///
  ///
  pub fn scale_y(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_scaleY(self as *const ::painter::PixmapFragment) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::PixmapFragment::set_height(double value)```</span>
  ///
  ///
  pub fn set_height(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_set_height(self as *mut ::painter::PixmapFragment, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::PixmapFragment::set_opacity(double value)```</span>
  ///
  ///
  pub fn set_opacity(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_set_opacity(self as *mut ::painter::PixmapFragment, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::PixmapFragment::set_rotation(double value)```</span>
  ///
  ///
  pub fn set_rotation(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_set_rotation(self as *mut ::painter::PixmapFragment, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::PixmapFragment::set_scaleX(double value)```</span>
  ///
  ///
  pub fn set_scale_x(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_set_scaleX(self as *mut ::painter::PixmapFragment, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::PixmapFragment::set_scaleY(double value)```</span>
  ///
  ///
  pub fn set_scale_y(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_set_scaleY(self as *mut ::painter::PixmapFragment, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::PixmapFragment::set_sourceLeft(double value)```</span>
  ///
  ///
  pub fn set_source_left(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_set_sourceLeft(self as *mut ::painter::PixmapFragment, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::PixmapFragment::set_sourceTop(double value)```</span>
  ///
  ///
  pub fn set_source_top(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_set_sourceTop(self as *mut ::painter::PixmapFragment, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::PixmapFragment::set_width(double value)```</span>
  ///
  ///
  pub fn set_width(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_set_width(self as *mut ::painter::PixmapFragment, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::PixmapFragment::set_x(double value)```</span>
  ///
  ///
  pub fn set_x(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_set_x(self as *mut ::painter::PixmapFragment, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPainter::PixmapFragment::set_y(double value)```</span>
  ///
  ///
  pub fn set_y(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_set_y(self as *mut ::painter::PixmapFragment, value) }
  }

  /// C++ method: <span style='color: green;'>```double QPainter::PixmapFragment::sourceLeft() const```</span>
  ///
  ///
  pub fn source_left(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_sourceLeft(self as *const ::painter::PixmapFragment) }
  }

  /// C++ method: <span style='color: green;'>```double QPainter::PixmapFragment::sourceTop() const```</span>
  ///
  ///
  pub fn source_top(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_sourceTop(self as *const ::painter::PixmapFragment) }
  }

  /// C++ method: <span style='color: green;'>```double QPainter::PixmapFragment::width() const```</span>
  ///
  ///
  pub fn width(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_width(self as *const ::painter::PixmapFragment) }
  }

  /// C++ method: <span style='color: green;'>```double QPainter::PixmapFragment::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_x(self as *const ::painter::PixmapFragment) }
  }

  /// C++ method: <span style='color: green;'>```double QPainter::PixmapFragment::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_y(self as *const ::painter::PixmapFragment) }
  }
}

impl ::cpp_utils::CppDeletable for ::painter::PixmapFragment {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPainter_PixmapFragment_delete
  }
}

/// C++ type: <span style='color: green;'>```QPainter::PixmapFragmentHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PixmapFragmentHint {
  /// This variant is added in Rust because enums with one variant and C representation are not supported.
  _Invalid = 0,
  /// C++ enum variant: <span style='color: green;'>```OpaqueHint = 1```</span>
  OpaqueHint = 1,
}

impl ::qt_core::flags::FlaggableEnum for PixmapFragmentHint {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "PixmapFragmentHint"
  }
}

/// C++ type: <span style='color: green;'>```QPainter::RenderHint```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum RenderHint {
  /// C++ enum variant: <span style='color: green;'>```Antialiasing = 1```</span>
  Antialiasing = 1,
  /// C++ enum variant: <span style='color: green;'>```TextAntialiasing = 2```</span>
  TextAntialiasing = 2,
  /// C++ enum variant: <span style='color: green;'>```SmoothPixmapTransform = 4```</span>
  SmoothPixmapTransform = 4,
  /// C++ enum variant: <span style='color: green;'>```HighQualityAntialiasing = 8```</span>
  HighQualityAntialiasing = 8,
  /// C++ enum variant: <span style='color: green;'>```NonCosmeticDefaultPen = 16```</span>
  NonCosmeticDefaultPen = 16,
  /// C++ enum variant: <span style='color: green;'>```Qt4CompatiblePainting = 32```</span>
  Qt4CompatiblePainting = 32,
}

impl ::qt_core::flags::FlaggableEnum for RenderHint {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "RenderHint"
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Painter::bounding_rect](../struct.Painter.html#method.bounding_rect) method.
  pub trait PainterBoundingRectArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> Self::ReturnType;
  }
  impl<'largs> PainterBoundingRectArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::string::String) {
    type ReturnType = ::qt_core::rect_f::RectF;
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ::qt_core::rect_f::RectF {
      let rect = self.0;
      let text = self.1;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainter_boundingRect_to_output_QRectF_QString(original_self as *mut ::painter::Painter,
                                                                         rect as *const ::qt_core::rect_f::RectF,
                                                                         text as *const ::qt_core::string::String,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PainterBoundingRectArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::string::String, &'largs ::text_option::TextOption) {
    type ReturnType = ::qt_core::rect_f::RectF;
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ::qt_core::rect_f::RectF {
      let rect = self.0;
      let text = self.1;
      let o = self.2;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainter_boundingRect_to_output_QRectF_QString_QTextOption(original_self as *mut ::painter::Painter, rect as *const ::qt_core::rect_f::RectF, text as *const ::qt_core::string::String, o as *const ::text_option::TextOption, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PainterBoundingRectArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int, &'largs ::qt_core::string::String) {
    type ReturnType = ::qt_core::rect_f::RectF;
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ::qt_core::rect_f::RectF {
      let rect = self.0;
      let flags = self.1;
      let text = self.2;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainter_boundingRect_to_output_QRectF_int_QString(original_self as *mut ::painter::Painter, rect as *const ::qt_core::rect_f::RectF, flags, text as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PainterBoundingRectArgs<'largs>
    for (&'largs ::qt_core::rect::Rect, ::libc::c_int, &'largs ::qt_core::string::String) {
    type ReturnType = ::qt_core::rect::Rect;
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ::qt_core::rect::Rect {
      let rect = self.0;
      let flags = self.1;
      let text = self.2;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainter_boundingRect_to_output_QRect_int_QString(original_self as *mut ::painter::Painter,
                                                                            rect as *const ::qt_core::rect::Rect,
                                                                            flags,
                                                                            text as *const ::qt_core::string::String,
                                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PainterBoundingRectArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::qt_core::string::String) {
    type ReturnType = ::qt_core::rect::Rect;
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ::qt_core::rect::Rect {
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
          ::ffi::qt_gui_c_QPainter_boundingRect_to_output_int_int_int_int_int_QString(original_self as *mut ::painter::Painter, x, y, w, h, flags, text as *const ::qt_core::string::String, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_arc](../struct.Painter.html#method.draw_arc) method.
  pub trait PainterDrawArcArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawArcArgs<'largs> for (&'largs ::qt_core::rect::Rect, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let a = self.1;
      let alen = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawArc_arg1_a_alen(original_self as *mut ::painter::Painter,
                                                     arg1 as *const ::qt_core::rect::Rect,
                                                     a,
                                                     alen)
      }
    }
  }
  impl<'largs> PainterDrawArcArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rect = self.0;
      let a = self.1;
      let alen = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawArc_rect_a_alen(original_self as *mut ::painter::Painter,
                                                     rect as *const ::qt_core::rect_f::RectF,
                                                     a,
                                                     alen)
      }
    }
  }
  impl<'largs> PainterDrawArcArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let a = self.4;
      let alen = self.5;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawArc_x_y_w_h_a_alen(original_self as *mut ::painter::Painter,
                                                        x,
                                                        y,
                                                        w,
                                                        h,
                                                        a,
                                                        alen)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_chord](../struct.Painter.html#method.draw_chord) method.
  pub trait PainterDrawChordArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawChordArgs<'largs> for (&'largs ::qt_core::rect::Rect, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let a = self.1;
      let alen = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawChord_arg1_a_alen(original_self as *mut ::painter::Painter,
                                                       arg1 as *const ::qt_core::rect::Rect,
                                                       a,
                                                       alen)
      }
    }
  }
  impl<'largs> PainterDrawChordArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rect = self.0;
      let a = self.1;
      let alen = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawChord_rect_a_alen(original_self as *mut ::painter::Painter,
                                                       rect as *const ::qt_core::rect_f::RectF,
                                                       a,
                                                       alen)
      }
    }
  }
  impl<'largs> PainterDrawChordArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let a = self.4;
      let alen = self.5;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawChord_x_y_w_h_a_alen(original_self as *mut ::painter::Painter,
                                                          x,
                                                          y,
                                                          w,
                                                          h,
                                                          a,
                                                          alen)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_convex_polygon](../struct.Painter.html#method.draw_convex_polygon) method.
  pub trait PainterDrawConvexPolygonArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawConvexPolygonArgs<'largs> for &'largs ::polygon::Polygon {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let polygon = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawConvexPolygon_QPolygon(original_self as *mut ::painter::Painter,
                                                            polygon as *const ::polygon::Polygon)
      }
    }
  }
  impl<'largs> PainterDrawConvexPolygonArgs<'largs> for &'largs ::polygon_f::PolygonF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let polygon = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawConvexPolygon_QPolygonF(original_self as *mut ::painter::Painter,
                                                             polygon as *const ::polygon_f::PolygonF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_convex_polygon_unsafe](../struct.Painter.html#method.draw_convex_polygon_unsafe) method.
  pub trait PainterDrawConvexPolygonUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawConvexPolygonUnsafeArgs<'largs> for (*const ::qt_core::point_f::PointF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self.0;
      let point_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawConvexPolygon_QPointF_int(original_self as *mut ::painter::Painter,
                                                             points,
                                                             point_count)
    }
  }
  impl<'largs> PainterDrawConvexPolygonUnsafeArgs<'largs> for (*const ::qt_core::point::Point, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self.0;
      let point_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawConvexPolygon_QPoint_int(original_self as *mut ::painter::Painter,
                                                            points,
                                                            point_count)
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_ellipse](../struct.Painter.html#method.draw_ellipse) method.
  pub trait PainterDrawEllipseArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawEllipseArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let center = self.0;
      let rx = self.1;
      let ry = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawEllipse_QPointF_double_double(original_self as *mut ::painter::Painter,
                                                                   center as *const ::qt_core::point_f::PointF,
                                                                   rx,
                                                                   ry)
      }
    }
  }
  impl<'largs> PainterDrawEllipseArgs<'largs> for (&'largs ::qt_core::point::Point, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let center = self.0;
      let rx = self.1;
      let ry = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawEllipse_QPoint_int_int(original_self as *mut ::painter::Painter,
                                                            center as *const ::qt_core::point::Point,
                                                            rx,
                                                            ry)
      }
    }
  }
  impl<'largs> PainterDrawEllipseArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawEllipse_QRect(original_self as *mut ::painter::Painter,
                                                   r as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PainterDrawEllipseArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawEllipse_QRectF(original_self as *mut ::painter::Painter,
                                                    r as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> PainterDrawEllipseArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawEllipse_int_int_int_int(original_self as *mut ::painter::Painter, x, y, w, h)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_image](../struct.Painter.html#method.draw_image) method.
  pub trait PainterDrawImageArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawImageArgs<'largs> for (&'largs ::qt_core::point_f::PointF, &'largs ::image::Image) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let image = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawImage_QPointF_QImage(original_self as *mut ::painter::Painter,
                                                          p as *const ::qt_core::point_f::PointF,
                                                          image as *const ::image::Image)
      }
    }
  }
  impl<'largs> PainterDrawImageArgs<'largs> for (&'largs ::qt_core::point::Point, &'largs ::image::Image) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let image = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawImage_QPoint_QImage(original_self as *mut ::painter::Painter,
                                                         p as *const ::qt_core::point::Point,
                                                         image as *const ::image::Image)
      }
    }
  }
  impl<'largs> PainterDrawImageArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, &'largs ::image::Image) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let image = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawImage_QRectF_QImage(original_self as *mut ::painter::Painter,
                                                         r as *const ::qt_core::rect_f::RectF,
                                                         image as *const ::image::Image)
      }
    }
  }
  impl<'largs> PainterDrawImageArgs<'largs> for (&'largs ::qt_core::rect::Rect, &'largs ::image::Image) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let image = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawImage_QRect_QImage(original_self as *mut ::painter::Painter,
                                                        r as *const ::qt_core::rect::Rect,
                                                        image as *const ::image::Image)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_line](../struct.Painter.html#method.draw_line) method.
  pub trait PainterDrawLineArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawLineArgs<'largs> for &'largs ::qt_core::line::Line {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let line = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawLine_QLine(original_self as *mut ::painter::Painter,
                                                line as *const ::qt_core::line::Line)
      }
    }
  }
  impl<'largs> PainterDrawLineArgs<'largs> for &'largs ::qt_core::line_f::LineF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let line = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawLine_QLineF(original_self as *mut ::painter::Painter,
                                                 line as *const ::qt_core::line_f::LineF)
      }
    }
  }
  impl<'largs> PainterDrawLineArgs<'largs> for (&'largs ::qt_core::point_f::PointF, &'largs ::qt_core::point_f::PointF) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p1 = self.0;
      let p2 = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawLine_QPointF_QPointF(original_self as *mut ::painter::Painter,
                                                          p1 as *const ::qt_core::point_f::PointF,
                                                          p2 as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> PainterDrawLineArgs<'largs> for (&'largs ::qt_core::point::Point, &'largs ::qt_core::point::Point) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p1 = self.0;
      let p2 = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawLine_QPoint_QPoint(original_self as *mut ::painter::Painter,
                                                        p1 as *const ::qt_core::point::Point,
                                                        p2 as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> PainterDrawLineArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x1 = self.0;
      let y1 = self.1;
      let x2 = self.2;
      let y2 = self.3;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawLine_int_int_int_int(original_self as *mut ::painter::Painter, x1, y1, x2, y2)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_lines](../struct.Painter.html#method.draw_lines) method.
  pub trait PainterDrawLinesArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawLinesArgs<'largs> for &'largs ::vector::VectorQtCoreLine {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let lines = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawLines_QVector_QLine(original_self as *mut ::painter::Painter,
                                                         lines as *const ::vector::VectorQtCoreLine)
      }
    }
  }
  impl<'largs> PainterDrawLinesArgs<'largs> for &'largs ::vector::VectorQtCoreLineF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let lines = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawLines_QVector_QLineF(original_self as *mut ::painter::Painter,
                                                          lines as *const ::vector::VectorQtCoreLineF)
      }
    }
  }
  impl<'largs> PainterDrawLinesArgs<'largs> for &'largs ::vector::VectorQtCorePoint {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let point_pairs = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawLines_QVector_QPoint(original_self as *mut ::painter::Painter,
                                                          point_pairs as *const ::vector::VectorQtCorePoint)
      }
    }
  }
  impl<'largs> PainterDrawLinesArgs<'largs> for &'largs ::qt_core::vector::VectorPointF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let point_pairs = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawLines_QVector_QPointF(original_self as *mut ::painter::Painter,
                                                           point_pairs as *const ::qt_core::vector::VectorPointF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_lines_unsafe](../struct.Painter.html#method.draw_lines_unsafe) method.
  pub trait PainterDrawLinesUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawLinesUnsafeArgs<'largs> for (*const ::qt_core::line_f::LineF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let lines = self.0;
      let line_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawLines_QLineF_int(original_self as *mut ::painter::Painter, lines, line_count)
    }
  }
  impl<'largs> PainterDrawLinesUnsafeArgs<'largs> for (*const ::qt_core::line::Line, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let lines = self.0;
      let line_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawLines_QLine_int(original_self as *mut ::painter::Painter, lines, line_count)
    }
  }
  impl<'largs> PainterDrawLinesUnsafeArgs<'largs> for (*const ::qt_core::point_f::PointF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let point_pairs = self.0;
      let line_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawLines_QPointF_int(original_self as *mut ::painter::Painter,
                                                     point_pairs,
                                                     line_count)
    }
  }
  impl<'largs> PainterDrawLinesUnsafeArgs<'largs> for (*const ::qt_core::point::Point, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let point_pairs = self.0;
      let line_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawLines_QPoint_int(original_self as *mut ::painter::Painter,
                                                    point_pairs,
                                                    line_count)
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_picture](../struct.Painter.html#method.draw_picture) method.
  pub trait PainterDrawPictureArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawPictureArgs<'largs> for (&'largs ::qt_core::point_f::PointF, &'largs ::picture::Picture) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let picture = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPicture_QPointF_QPicture(original_self as *mut ::painter::Painter,
                                                              p as *const ::qt_core::point_f::PointF,
                                                              picture as *const ::picture::Picture)
      }
    }
  }
  impl<'largs> PainterDrawPictureArgs<'largs> for (&'largs ::qt_core::point::Point, &'largs ::picture::Picture) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let picture = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPicture_QPoint_QPicture(original_self as *mut ::painter::Painter,
                                                             p as *const ::qt_core::point::Point,
                                                             picture as *const ::picture::Picture)
      }
    }
  }
  impl<'largs> PainterDrawPictureArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::picture::Picture) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let picture = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPicture_int_int_QPicture(original_self as *mut ::painter::Painter,
                                                              x,
                                                              y,
                                                              picture as *const ::picture::Picture)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_pie](../struct.Painter.html#method.draw_pie) method.
  pub trait PainterDrawPieArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawPieArgs<'largs> for (&'largs ::qt_core::rect::Rect, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let a = self.1;
      let alen = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPie_arg1_a_alen(original_self as *mut ::painter::Painter,
                                                     arg1 as *const ::qt_core::rect::Rect,
                                                     a,
                                                     alen)
      }
    }
  }
  impl<'largs> PainterDrawPieArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rect = self.0;
      let a = self.1;
      let alen = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPie_rect_a_alen(original_self as *mut ::painter::Painter,
                                                     rect as *const ::qt_core::rect_f::RectF,
                                                     a,
                                                     alen)
      }
    }
  }
  impl<'largs> PainterDrawPieArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let a = self.4;
      let alen = self.5;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPie_x_y_w_h_a_alen(original_self as *mut ::painter::Painter,
                                                        x,
                                                        y,
                                                        w,
                                                        h,
                                                        a,
                                                        alen)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_pixmap](../struct.Painter.html#method.draw_pixmap) method.
  pub trait PainterDrawPixmapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawPixmapArgs<'largs> for (&'largs ::qt_core::point_f::PointF, &'largs ::pixmap::Pixmap) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let pm = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPixmap_QPointF_QPixmap(original_self as *mut ::painter::Painter,
                                                            p as *const ::qt_core::point_f::PointF,
                                                            pm as *const ::pixmap::Pixmap)
      }
    }
  }
  impl<'largs> PainterDrawPixmapArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF, &'largs ::pixmap::Pixmap, &'largs ::qt_core::rect_f::RectF) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let pm = self.1;
      let sr = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPixmap_QPointF_QPixmap_QRectF(original_self as *mut ::painter::Painter,
                                                                   p as *const ::qt_core::point_f::PointF,
                                                                   pm as *const ::pixmap::Pixmap,
                                                                   sr as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> PainterDrawPixmapArgs<'largs> for (&'largs ::qt_core::point::Point, &'largs ::pixmap::Pixmap) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let pm = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPixmap_QPoint_QPixmap(original_self as *mut ::painter::Painter,
                                                           p as *const ::qt_core::point::Point,
                                                           pm as *const ::pixmap::Pixmap)
      }
    }
  }
  impl<'largs> PainterDrawPixmapArgs<'largs>
    for (&'largs ::qt_core::point::Point, &'largs ::pixmap::Pixmap, &'largs ::qt_core::rect::Rect) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let pm = self.1;
      let sr = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPixmap_QPoint_QPixmap_QRect(original_self as *mut ::painter::Painter,
                                                                 p as *const ::qt_core::point::Point,
                                                                 pm as *const ::pixmap::Pixmap,
                                                                 sr as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PainterDrawPixmapArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, &'largs ::pixmap::Pixmap, &'largs ::qt_core::rect_f::RectF) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let target_rect = self.0;
      let pixmap = self.1;
      let source_rect = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPixmap_QRectF_QPixmap_QRectF(original_self as *mut ::painter::Painter,
                                                                  target_rect as *const ::qt_core::rect_f::RectF,
                                                                  pixmap as *const ::pixmap::Pixmap,
                                                                  source_rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> PainterDrawPixmapArgs<'largs> for (&'largs ::qt_core::rect::Rect, &'largs ::pixmap::Pixmap) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let pm = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPixmap_QRect_QPixmap(original_self as *mut ::painter::Painter,
                                                          r as *const ::qt_core::rect::Rect,
                                                          pm as *const ::pixmap::Pixmap)
      }
    }
  }
  impl<'largs> PainterDrawPixmapArgs<'largs>
    for (&'largs ::qt_core::rect::Rect, &'largs ::pixmap::Pixmap, &'largs ::qt_core::rect::Rect) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let target_rect = self.0;
      let pixmap = self.1;
      let source_rect = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPixmap_QRect_QPixmap_QRect(original_self as *mut ::painter::Painter,
                                                                target_rect as *const ::qt_core::rect::Rect,
                                                                pixmap as *const ::pixmap::Pixmap,
                                                                source_rect as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PainterDrawPixmapArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::pixmap::Pixmap) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let pm = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPixmap_int_int_QPixmap(original_self as *mut ::painter::Painter,
                                                            x,
                                                            y,
                                                            pm as *const ::pixmap::Pixmap)
      }
    }
  }
  impl<'largs> PainterDrawPixmapArgs<'largs>
    for (::libc::c_int,
                                                      ::libc::c_int,
                                                      &'largs ::pixmap::Pixmap,
                                                      ::libc::c_int,
                                                      ::libc::c_int,
                                                      ::libc::c_int,
                                                      ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let pm = self.2;
      let sx = self.3;
      let sy = self.4;
      let sw = self.5;
      let sh = self.6;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPixmap_int_int_QPixmap_int_int_int_int(original_self as *mut ::painter::Painter,
                                                                            x,
                                                                            y,
                                                                            pm as *const ::pixmap::Pixmap,
                                                                            sx,
                                                                            sy,
                                                                            sw,
                                                                            sh)
      }
    }
  }
  impl<'largs> PainterDrawPixmapArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::pixmap::Pixmap) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let pm = self.4;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPixmap_int_int_int_int_QPixmap(original_self as *mut ::painter::Painter,
                                                                    x,
                                                                    y,
                                                                    w,
                                                                    h,
                                                                    pm as *const ::pixmap::Pixmap)
      }
    }
  }
  impl<'largs> PainterDrawPixmapArgs<'largs>
    for (::libc::c_int,
                                                      ::libc::c_int,
                                                      ::libc::c_int,
                                                      ::libc::c_int,
                                                      &'largs ::pixmap::Pixmap,
                                                      ::libc::c_int,
                                                      ::libc::c_int,
                                                      ::libc::c_int,
                                                      ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let pm = self.4;
      let sx = self.5;
      let sy = self.6;
      let sw = self.7;
      let sh = self.8;
      unsafe { ::ffi::qt_gui_c_QPainter_drawPixmap_int_int_int_int_QPixmap_int_int_int_int(original_self as *mut ::painter::Painter, x, y, w, h, pm as *const ::pixmap::Pixmap, sx, sy, sw, sh) }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_pixmap_fragments](../struct.Painter.html#method.draw_pixmap_fragments) method.
  pub trait PainterDrawPixmapFragmentsArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawPixmapFragmentsArgs<'largs>
    for (*const ::painter::PixmapFragment, ::libc::c_int, &'largs ::pixmap::Pixmap) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let fragments = self.0;
      let fragment_count = self.1;
      let pixmap = self.2;
      ::ffi::qt_gui_c_QPainter_drawPixmapFragments_fragments_fragmentCount_pixmap(original_self as *mut ::painter::Painter, fragments, fragment_count, pixmap as *const ::pixmap::Pixmap)
    }
  }
  impl<'largs> PainterDrawPixmapFragmentsArgs<'largs>
    for (*const ::painter::PixmapFragment,
                                                               ::libc::c_int,
                                                               &'largs ::pixmap::Pixmap,
                                                               ::qt_core::flags::Flags<::painter::PixmapFragmentHint>) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let fragments = self.0;
      let fragment_count = self.1;
      let pixmap = self.2;
      let hints = self.3;
      ::ffi::qt_gui_c_QPainter_drawPixmapFragments_fragments_fragmentCount_pixmap_hints(original_self as *mut ::painter::Painter, fragments, fragment_count, pixmap as *const ::pixmap::Pixmap, hints.to_int() as ::libc::c_uint)
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_point](../struct.Painter.html#method.draw_point) method.
  pub trait PainterDrawPointArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawPointArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPoint_p(original_self as *mut ::painter::Painter,
                                             p as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> PainterDrawPointArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let pt = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPoint_pt(original_self as *mut ::painter::Painter,
                                              pt as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> PainterDrawPointArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_gui_c_QPainter_drawPoint_x_y(original_self as *mut ::painter::Painter, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_points](../struct.Painter.html#method.draw_points) method.
  pub trait PainterDrawPointsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawPointsArgs<'largs> for &'largs ::polygon::Polygon {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPoints_QPolygon(original_self as *mut ::painter::Painter,
                                                     points as *const ::polygon::Polygon)
      }
    }
  }
  impl<'largs> PainterDrawPointsArgs<'largs> for &'largs ::polygon_f::PolygonF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPoints_QPolygonF(original_self as *mut ::painter::Painter,
                                                      points as *const ::polygon_f::PolygonF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_points_unsafe](../struct.Painter.html#method.draw_points_unsafe) method.
  pub trait PainterDrawPointsUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawPointsUnsafeArgs<'largs> for (*const ::qt_core::point_f::PointF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self.0;
      let point_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawPoints_QPointF_int(original_self as *mut ::painter::Painter,
                                                      points,
                                                      point_count)
    }
  }
  impl<'largs> PainterDrawPointsUnsafeArgs<'largs> for (*const ::qt_core::point::Point, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self.0;
      let point_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawPoints_QPoint_int(original_self as *mut ::painter::Painter,
                                                     points,
                                                     point_count)
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_polygon](../struct.Painter.html#method.draw_polygon) method.
  pub trait PainterDrawPolygonArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawPolygonArgs<'largs> for &'largs ::polygon::Polygon {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let polygon = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPolygon_QPolygon(original_self as *mut ::painter::Painter,
                                                      polygon as *const ::polygon::Polygon)
      }
    }
  }
  impl<'largs> PainterDrawPolygonArgs<'largs> for &'largs ::polygon_f::PolygonF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let polygon = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPolygon_QPolygonF(original_self as *mut ::painter::Painter,
                                                       polygon as *const ::polygon_f::PolygonF)
      }
    }
  }
  impl<'largs> PainterDrawPolygonArgs<'largs> for (&'largs ::polygon_f::PolygonF, &'largs ::qt_core::qt::FillRule) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let polygon = self.0;
      let fill_rule = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPolygon_QPolygonF_Qt_FillRule(original_self as *mut ::painter::Painter,
                                                                   polygon as *const ::polygon_f::PolygonF,
                                                                   fill_rule as *const ::qt_core::qt::FillRule)
      }
    }
  }
  impl<'largs> PainterDrawPolygonArgs<'largs> for (&'largs ::polygon::Polygon, &'largs ::qt_core::qt::FillRule) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let polygon = self.0;
      let fill_rule = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPolygon_QPolygon_Qt_FillRule(original_self as *mut ::painter::Painter,
                                                                  polygon as *const ::polygon::Polygon,
                                                                  fill_rule as *const ::qt_core::qt::FillRule)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_polygon_unsafe](../struct.Painter.html#method.draw_polygon_unsafe) method.
  pub trait PainterDrawPolygonUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawPolygonUnsafeArgs<'largs> for (*const ::qt_core::point_f::PointF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self.0;
      let point_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawPolygon_QPointF_int(original_self as *mut ::painter::Painter,
                                                       points,
                                                       point_count)
    }
  }
  impl<'largs> PainterDrawPolygonUnsafeArgs<'largs>
    for (*const ::qt_core::point_f::PointF, ::libc::c_int, &'largs ::qt_core::qt::FillRule) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self.0;
      let point_count = self.1;
      let fill_rule = self.2;
      ::ffi::qt_gui_c_QPainter_drawPolygon_QPointF_int_Qt_FillRule(original_self as *mut ::painter::Painter,
                                                                   points,
                                                                   point_count,
                                                                   fill_rule as *const ::qt_core::qt::FillRule)
    }
  }
  impl<'largs> PainterDrawPolygonUnsafeArgs<'largs> for (*const ::qt_core::point::Point, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self.0;
      let point_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawPolygon_QPoint_int(original_self as *mut ::painter::Painter,
                                                      points,
                                                      point_count)
    }
  }
  impl<'largs> PainterDrawPolygonUnsafeArgs<'largs>
    for (*const ::qt_core::point::Point, ::libc::c_int, &'largs ::qt_core::qt::FillRule) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self.0;
      let point_count = self.1;
      let fill_rule = self.2;
      ::ffi::qt_gui_c_QPainter_drawPolygon_QPoint_int_Qt_FillRule(original_self as *mut ::painter::Painter,
                                                                  points,
                                                                  point_count,
                                                                  fill_rule as *const ::qt_core::qt::FillRule)
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_polyline](../struct.Painter.html#method.draw_polyline) method.
  pub trait PainterDrawPolylineArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawPolylineArgs<'largs> for &'largs ::polygon::Polygon {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let polygon = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPolyline_QPolygon(original_self as *mut ::painter::Painter,
                                                       polygon as *const ::polygon::Polygon)
      }
    }
  }
  impl<'largs> PainterDrawPolylineArgs<'largs> for &'largs ::polygon_f::PolygonF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let polyline = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawPolyline_QPolygonF(original_self as *mut ::painter::Painter,
                                                        polyline as *const ::polygon_f::PolygonF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_polyline_unsafe](../struct.Painter.html#method.draw_polyline_unsafe) method.
  pub trait PainterDrawPolylineUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawPolylineUnsafeArgs<'largs> for (*const ::qt_core::point_f::PointF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self.0;
      let point_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawPolyline_QPointF_int(original_self as *mut ::painter::Painter,
                                                        points,
                                                        point_count)
    }
  }
  impl<'largs> PainterDrawPolylineUnsafeArgs<'largs> for (*const ::qt_core::point::Point, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let points = self.0;
      let point_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawPolyline_QPoint_int(original_self as *mut ::painter::Painter,
                                                       points,
                                                       point_count)
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_rect](../struct.Painter.html#method.draw_rect) method.
  pub trait PainterDrawRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawRectArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRect_QRect(original_self as *mut ::painter::Painter,
                                                rect as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PainterDrawRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRect_QRectF(original_self as *mut ::painter::Painter,
                                                 rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> PainterDrawRectArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x1 = self.0;
      let y1 = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRect_int_int_int_int(original_self as *mut ::painter::Painter, x1, y1, w, h)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_rects](../struct.Painter.html#method.draw_rects) method.
  pub trait PainterDrawRectsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawRectsArgs<'largs> for &'largs ::vector::VectorQtCoreRect {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rectangles = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRects_QVector_QRect(original_self as *mut ::painter::Painter,
                                                         rectangles as *const ::vector::VectorQtCoreRect)
      }
    }
  }
  impl<'largs> PainterDrawRectsArgs<'largs> for &'largs ::vector::VectorQtCoreRectF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rectangles = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRects_QVector_QRectF(original_self as *mut ::painter::Painter,
                                                          rectangles as *const ::vector::VectorQtCoreRectF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_rects_unsafe](../struct.Painter.html#method.draw_rects_unsafe) method.
  pub trait PainterDrawRectsUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawRectsUnsafeArgs<'largs> for (*const ::qt_core::rect_f::RectF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rects = self.0;
      let rect_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawRects_QRectF_int(original_self as *mut ::painter::Painter, rects, rect_count)
    }
  }
  impl<'largs> PainterDrawRectsUnsafeArgs<'largs> for (*const ::qt_core::rect::Rect, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rects = self.0;
      let rect_count = self.1;
      ::ffi::qt_gui_c_QPainter_drawRects_QRect_int(original_self as *mut ::painter::Painter, rects, rect_count)
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_round_rect](../struct.Painter.html#method.draw_round_rect) method.
  pub trait PainterDrawRoundRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawRoundRectArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundRect_QRect(original_self as *mut ::painter::Painter,
                                                     r as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PainterDrawRoundRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundRect_QRectF(original_self as *mut ::painter::Painter,
                                                      r as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> PainterDrawRoundRectArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let xround = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundRect_QRectF_int(original_self as *mut ::painter::Painter,
                                                          r as *const ::qt_core::rect_f::RectF,
                                                          xround)
      }
    }
  }
  impl<'largs> PainterDrawRoundRectArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let xround = self.1;
      let yround = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundRect_QRectF_int_int(original_self as *mut ::painter::Painter,
                                                              r as *const ::qt_core::rect_f::RectF,
                                                              xround,
                                                              yround)
      }
    }
  }
  impl<'largs> PainterDrawRoundRectArgs<'largs> for (&'largs ::qt_core::rect::Rect, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let xround = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundRect_QRect_int(original_self as *mut ::painter::Painter,
                                                         r as *const ::qt_core::rect::Rect,
                                                         xround)
      }
    }
  }
  impl<'largs> PainterDrawRoundRectArgs<'largs> for (&'largs ::qt_core::rect::Rect, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let xround = self.1;
      let yround = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundRect_QRect_int_int(original_self as *mut ::painter::Painter,
                                                             r as *const ::qt_core::rect::Rect,
                                                             xround,
                                                             yround)
      }
    }
  }
  impl<'largs> PainterDrawRoundRectArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundRect_int_int_int_int(original_self as *mut ::painter::Painter, x, y, w, h)
      }
    }
  }
  impl<'largs> PainterDrawRoundRectArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let arg5 = self.4;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundRect_int_int_int_int_int(original_self as *mut ::painter::Painter,
                                                                   x,
                                                                   y,
                                                                   w,
                                                                   h,
                                                                   arg5)
      }
    }
  }
  impl<'largs> PainterDrawRoundRectArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let arg5 = self.4;
      let arg6 = self.5;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundRect_int_int_int_int_int_int(original_self as *mut ::painter::Painter,
                                                                       x,
                                                                       y,
                                                                       w,
                                                                       h,
                                                                       arg5,
                                                                       arg6)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_rounded_rect](../struct.Painter.html#method.draw_rounded_rect) method.
  pub trait PainterDrawRoundedRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawRoundedRectArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rect = self.0;
      let x_radius = self.1;
      let y_radius = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundedRect_QRectF_double_double(original_self as *mut ::painter::Painter,
                                                                      rect as *const ::qt_core::rect_f::RectF,
                                                                      x_radius,
                                                                      y_radius)
      }
    }
  }
  impl<'largs> PainterDrawRoundedRectArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double, &'largs ::qt_core::qt::SizeMode) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rect = self.0;
      let x_radius = self.1;
      let y_radius = self.2;
      let mode = self.3;
      unsafe { ::ffi::qt_gui_c_QPainter_drawRoundedRect_QRectF_double_double_Qt_SizeMode(original_self as *mut ::painter::Painter, rect as *const ::qt_core::rect_f::RectF, x_radius, y_radius, mode as *const ::qt_core::qt::SizeMode) }
    }
  }
  impl<'largs> PainterDrawRoundedRectArgs<'largs>
    for (&'largs ::qt_core::rect::Rect, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rect = self.0;
      let x_radius = self.1;
      let y_radius = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundedRect_QRect_double_double(original_self as *mut ::painter::Painter,
                                                                     rect as *const ::qt_core::rect::Rect,
                                                                     x_radius,
                                                                     y_radius)
      }
    }
  }
  impl<'largs> PainterDrawRoundedRectArgs<'largs>
    for (&'largs ::qt_core::rect::Rect, ::libc::c_double, ::libc::c_double, &'largs ::qt_core::qt::SizeMode) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rect = self.0;
      let x_radius = self.1;
      let y_radius = self.2;
      let mode = self.3;
      unsafe { ::ffi::qt_gui_c_QPainter_drawRoundedRect_QRect_double_double_Qt_SizeMode(original_self as *mut ::painter::Painter, rect as *const ::qt_core::rect::Rect, x_radius, y_radius, mode as *const ::qt_core::qt::SizeMode) }
    }
  }
  impl<'largs> PainterDrawRoundedRectArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let x_radius = self.4;
      let y_radius = self.5;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawRoundedRect_int_int_int_int_double_double(original_self as *mut ::painter::Painter, x, y, w, h, x_radius, y_radius)
      }
    }
  }
  impl<'largs> PainterDrawRoundedRectArgs<'largs>
    for (::libc::c_int,
                                                           ::libc::c_int,
                                                           ::libc::c_int,
                                                           ::libc::c_int,
                                                           ::libc::c_double,
                                                           ::libc::c_double,
                                                           &'largs ::qt_core::qt::SizeMode) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let x_radius = self.4;
      let y_radius = self.5;
      let mode = self.6;
      unsafe { ::ffi::qt_gui_c_QPainter_drawRoundedRect_int_int_int_int_double_double_Qt_SizeMode(original_self as *mut ::painter::Painter, x, y, w, h, x_radius, y_radius, mode as *const ::qt_core::qt::SizeMode) }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_static_text](../struct.Painter.html#method.draw_static_text) method.
  pub trait PainterDrawStaticTextArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawStaticTextArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF, &'largs ::static_text::StaticText) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let top_left_position = self.0;
      let static_text = self.1;
      unsafe { ::ffi::qt_gui_c_QPainter_drawStaticText_QPointF_QStaticText(original_self as *mut ::painter::Painter, top_left_position as *const ::qt_core::point_f::PointF, static_text as *const ::static_text::StaticText) }
    }
  }
  impl<'largs> PainterDrawStaticTextArgs<'largs>
    for (&'largs ::qt_core::point::Point, &'largs ::static_text::StaticText) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let top_left_position = self.0;
      let static_text = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawStaticText_QPoint_QStaticText(original_self as *mut ::painter::Painter, top_left_position as *const ::qt_core::point::Point, static_text as *const ::static_text::StaticText)
      }
    }
  }
  impl<'largs> PainterDrawStaticTextArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::static_text::StaticText) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let left = self.0;
      let top = self.1;
      let static_text = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawStaticText_int_int_QStaticText(original_self as *mut ::painter::Painter,
                                                                    left,
                                                                    top,
                                                                    static_text as *const ::static_text::StaticText)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_text](../struct.Painter.html#method.draw_text) method.
  pub trait PainterDrawTextArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawTextArgs<'largs> for (&'largs ::qt_core::point_f::PointF, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let s = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawText_QPointF_QString(original_self as *mut ::painter::Painter,
                                                          p as *const ::qt_core::point_f::PointF,
                                                          s as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> PainterDrawTextArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF, &'largs ::qt_core::string::String, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let str = self.1;
      let tf = self.2;
      let justification_padding = self.3;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawText_QPointF_QString_int_int(original_self as *mut ::painter::Painter,
                                                                  p as *const ::qt_core::point_f::PointF,
                                                                  str as *const ::qt_core::string::String,
                                                                  tf,
                                                                  justification_padding)
      }
    }
  }
  impl<'largs> PainterDrawTextArgs<'largs> for (&'largs ::qt_core::point::Point, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let s = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawText_QPoint_QString(original_self as *mut ::painter::Painter,
                                                         p as *const ::qt_core::point::Point,
                                                         s as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> PainterDrawTextArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let text = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawText_QRectF_QString(original_self as *mut ::painter::Painter,
                                                         r as *const ::qt_core::rect_f::RectF,
                                                         text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> PainterDrawTextArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::string::String, &'largs ::text_option::TextOption) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let text = self.1;
      let o = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawText_QRectF_QString_QTextOption(original_self as *mut ::painter::Painter,
                                                                     r as *const ::qt_core::rect_f::RectF,
                                                                     text as *const ::qt_core::string::String,
                                                                     o as *const ::text_option::TextOption)
      }
    }
  }
  impl<'largs> PainterDrawTextArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let flags = self.1;
      let text = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawText_QRectF_int_QString(original_self as *mut ::painter::Painter,
                                                             r as *const ::qt_core::rect_f::RectF,
                                                             flags,
                                                             text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> PainterDrawTextArgs<'largs>
    for (&'largs ::qt_core::rect::Rect, ::libc::c_int, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let flags = self.1;
      let text = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawText_QRect_int_QString(original_self as *mut ::painter::Painter,
                                                            r as *const ::qt_core::rect::Rect,
                                                            flags,
                                                            text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> PainterDrawTextArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let s = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawText_int_int_QString(original_self as *mut ::painter::Painter,
                                                          x,
                                                          y,
                                                          s as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> PainterDrawTextArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let flags = self.4;
      let text = self.5;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawText_int_int_int_int_int_QString(original_self as *mut ::painter::Painter,
                                                                      x,
                                                                      y,
                                                                      w,
                                                                      h,
                                                                      flags,
                                                                      text as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_text_item](../struct.Painter.html#method.draw_text_item) method.
  pub trait PainterDrawTextItemArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawTextItemArgs<'largs> for (&'largs ::qt_core::point_f::PointF, &'largs ::text_item::TextItem) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let ti = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawTextItem_QPointF_QTextItem(original_self as *mut ::painter::Painter,
                                                                p as *const ::qt_core::point_f::PointF,
                                                                ti as *const ::text_item::TextItem)
      }
    }
  }
  impl<'largs> PainterDrawTextItemArgs<'largs> for (&'largs ::qt_core::point::Point, &'largs ::text_item::TextItem) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let p = self.0;
      let ti = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawTextItem_QPoint_QTextItem(original_self as *mut ::painter::Painter,
                                                               p as *const ::qt_core::point::Point,
                                                               ti as *const ::text_item::TextItem)
      }
    }
  }
  impl<'largs> PainterDrawTextItemArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::text_item::TextItem) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let ti = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawTextItem_int_int_QTextItem(original_self as *mut ::painter::Painter,
                                                                x,
                                                                y,
                                                                ti as *const ::text_item::TextItem)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_text_unsafe](../struct.Painter.html#method.draw_text_unsafe) method.
  pub trait PainterDrawTextUnsafeArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawTextUnsafeArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF,
                                                          ::libc::c_int,
                                                          &'largs ::qt_core::string::String,
                                                          *mut ::qt_core::rect_f::RectF) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let flags = self.1;
      let text = self.2;
      let br = self.3;
      ::ffi::qt_gui_c_QPainter_drawText_QRectF_int_QString_QRectF(original_self as *mut ::painter::Painter,
                                                                  r as *const ::qt_core::rect_f::RectF,
                                                                  flags,
                                                                  text as *const ::qt_core::string::String,
                                                                  br)
    }
  }
  impl<'largs> PainterDrawTextUnsafeArgs<'largs>
    for (&'largs ::qt_core::rect::Rect, ::libc::c_int, &'largs ::qt_core::string::String, *mut ::qt_core::rect::Rect) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let flags = self.1;
      let text = self.2;
      let br = self.3;
      ::ffi::qt_gui_c_QPainter_drawText_QRect_int_QString_QRect(original_self as *mut ::painter::Painter,
                                                                r as *const ::qt_core::rect::Rect,
                                                                flags,
                                                                text as *const ::qt_core::string::String,
                                                                br)
    }
  }
  impl<'largs> PainterDrawTextUnsafeArgs<'largs>
    for (::libc::c_int,
                                                          ::libc::c_int,
                                                          ::libc::c_int,
                                                          ::libc::c_int,
                                                          ::libc::c_int,
                                                          &'largs ::qt_core::string::String,
                                                          *mut ::qt_core::rect::Rect) {
    unsafe fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let flags = self.4;
      let text = self.5;
      let br = self.6;
      ::ffi::qt_gui_c_QPainter_drawText_int_int_int_int_int_QString_QRect(original_self as *mut ::painter::Painter,
                                                                          x,
                                                                          y,
                                                                          w,
                                                                          h,
                                                                          flags,
                                                                          text as *const ::qt_core::string::String,
                                                                          br)
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::draw_tiled_pixmap](../struct.Painter.html#method.draw_tiled_pixmap) method.
  pub trait PainterDrawTiledPixmapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterDrawTiledPixmapArgs<'largs> for (&'largs ::qt_core::rect::Rect, &'largs ::pixmap::Pixmap) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let arg2 = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawTiledPixmap_arg1_arg2(original_self as *mut ::painter::Painter,
                                                           arg1 as *const ::qt_core::rect::Rect,
                                                           arg2 as *const ::pixmap::Pixmap)
      }
    }
  }
  impl<'largs> PainterDrawTiledPixmapArgs<'largs>
    for (&'largs ::qt_core::rect::Rect, &'largs ::pixmap::Pixmap, &'largs ::qt_core::point::Point) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let arg2 = self.1;
      let arg3 = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawTiledPixmap_arg1_arg2_arg3(original_self as *mut ::painter::Painter,
                                                                arg1 as *const ::qt_core::rect::Rect,
                                                                arg2 as *const ::pixmap::Pixmap,
                                                                arg3 as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> PainterDrawTiledPixmapArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, &'largs ::pixmap::Pixmap) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rect = self.0;
      let pm = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawTiledPixmap_rect_pm(original_self as *mut ::painter::Painter,
                                                         rect as *const ::qt_core::rect_f::RectF,
                                                         pm as *const ::pixmap::Pixmap)
      }
    }
  }
  impl<'largs> PainterDrawTiledPixmapArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, &'largs ::pixmap::Pixmap, &'largs ::qt_core::point_f::PointF) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let rect = self.0;
      let pm = self.1;
      let offset = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawTiledPixmap_rect_pm_offset(original_self as *mut ::painter::Painter,
                                                                rect as *const ::qt_core::rect_f::RectF,
                                                                pm as *const ::pixmap::Pixmap,
                                                                offset as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> PainterDrawTiledPixmapArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::pixmap::Pixmap) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let arg5 = self.4;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawTiledPixmap_x_y_w_h_arg5(original_self as *mut ::painter::Painter,
                                                              x,
                                                              y,
                                                              w,
                                                              h,
                                                              arg5 as *const ::pixmap::Pixmap)
      }
    }
  }
  impl<'largs> PainterDrawTiledPixmapArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::pixmap::Pixmap, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let arg5 = self.4;
      let sx = self.5;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawTiledPixmap_x_y_w_h_arg5_sx(original_self as *mut ::painter::Painter,
                                                                 x,
                                                                 y,
                                                                 w,
                                                                 h,
                                                                 arg5 as *const ::pixmap::Pixmap,
                                                                 sx)
      }
    }
  }
  impl<'largs> PainterDrawTiledPixmapArgs<'largs>
    for (::libc::c_int,
                                                           ::libc::c_int,
                                                           ::libc::c_int,
                                                           ::libc::c_int,
                                                           &'largs ::pixmap::Pixmap,
                                                           ::libc::c_int,
                                                           ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let arg5 = self.4;
      let sx = self.5;
      let sy = self.6;
      unsafe {
        ::ffi::qt_gui_c_QPainter_drawTiledPixmap_x_y_w_h_arg5_sx_sy(original_self as *mut ::painter::Painter,
                                                                    x,
                                                                    y,
                                                                    w,
                                                                    h,
                                                                    arg5 as *const ::pixmap::Pixmap,
                                                                    sx,
                                                                    sy)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::erase_rect](../struct.Painter.html#method.erase_rect) method.
  pub trait PainterEraseRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterEraseRectArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_eraseRect_QRect(original_self as *mut ::painter::Painter,
                                                 arg1 as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PainterEraseRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_eraseRect_QRectF(original_self as *mut ::painter::Painter,
                                                  arg1 as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> PainterEraseRectArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_gui_c_QPainter_eraseRect_int_int_int_int(original_self as *mut ::painter::Painter, x, y, w, h)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::fill_rect](../struct.Painter.html#method.fill_rect) method.
  pub trait PainterFillRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterFillRectArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, &'largs ::brush::Brush) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let arg2 = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_QRectF_QBrush(original_self as *mut ::painter::Painter,
                                                        arg1 as *const ::qt_core::rect_f::RectF,
                                                        arg2 as *const ::brush::Brush)
      }
    }
  }
  impl<'largs> PainterFillRectArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, &'largs ::color::Color) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let color = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_QRectF_QColor(original_self as *mut ::painter::Painter,
                                                        arg1 as *const ::qt_core::rect_f::RectF,
                                                        color as *const ::color::Color)
      }
    }
  }
  impl<'largs> PainterFillRectArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::qt::BrushStyle) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let style = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_QRectF_Qt_BrushStyle(original_self as *mut ::painter::Painter,
                                                               r as *const ::qt_core::rect_f::RectF,
                                                               style as *const ::qt_core::qt::BrushStyle)
      }
    }
  }
  impl<'largs> PainterFillRectArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::qt::GlobalColor) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let c = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_QRectF_Qt_GlobalColor(original_self as *mut ::painter::Painter,
                                                                r as *const ::qt_core::rect_f::RectF,
                                                                c as *const ::qt_core::qt::GlobalColor)
      }
    }
  }
  impl<'largs> PainterFillRectArgs<'largs> for (&'largs ::qt_core::rect::Rect, &'largs ::brush::Brush) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let arg2 = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_QRect_QBrush(original_self as *mut ::painter::Painter,
                                                       arg1 as *const ::qt_core::rect::Rect,
                                                       arg2 as *const ::brush::Brush)
      }
    }
  }
  impl<'largs> PainterFillRectArgs<'largs> for (&'largs ::qt_core::rect::Rect, &'largs ::color::Color) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let color = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_QRect_QColor(original_self as *mut ::painter::Painter,
                                                       arg1 as *const ::qt_core::rect::Rect,
                                                       color as *const ::color::Color)
      }
    }
  }
  impl<'largs> PainterFillRectArgs<'largs> for (&'largs ::qt_core::rect::Rect, &'largs ::qt_core::qt::BrushStyle) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let style = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_QRect_Qt_BrushStyle(original_self as *mut ::painter::Painter,
                                                              r as *const ::qt_core::rect::Rect,
                                                              style as *const ::qt_core::qt::BrushStyle)
      }
    }
  }
  impl<'largs> PainterFillRectArgs<'largs> for (&'largs ::qt_core::rect::Rect, &'largs ::qt_core::qt::GlobalColor) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let r = self.0;
      let c = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_QRect_Qt_GlobalColor(original_self as *mut ::painter::Painter,
                                                               r as *const ::qt_core::rect::Rect,
                                                               c as *const ::qt_core::qt::GlobalColor)
      }
    }
  }
  impl<'largs> PainterFillRectArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::brush::Brush) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let arg5 = self.4;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_int_int_int_int_QBrush(original_self as *mut ::painter::Painter,
                                                                 x,
                                                                 y,
                                                                 w,
                                                                 h,
                                                                 arg5 as *const ::brush::Brush)
      }
    }
  }
  impl<'largs> PainterFillRectArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::color::Color) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let color = self.4;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_int_int_int_int_QColor(original_self as *mut ::painter::Painter,
                                                                 x,
                                                                 y,
                                                                 w,
                                                                 h,
                                                                 color as *const ::color::Color)
      }
    }
  }
  impl<'largs> PainterFillRectArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::qt_core::qt::BrushStyle) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let style = self.4;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_int_int_int_int_Qt_BrushStyle(original_self as *mut ::painter::Painter,
                                                                        x,
                                                                        y,
                                                                        w,
                                                                        h,
                                                                        style as *const ::qt_core::qt::BrushStyle)
      }
    }
  }
  impl<'largs> PainterFillRectArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::qt_core::qt::GlobalColor) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let c = self.4;
      unsafe {
        ::ffi::qt_gui_c_QPainter_fillRect_int_int_int_int_Qt_GlobalColor(original_self as *mut ::painter::Painter,
                                                                         x,
                                                                         y,
                                                                         w,
                                                                         h,
                                                                         c as *const ::qt_core::qt::GlobalColor)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::redirected](../struct.Painter.html#method.redirected) method.
  pub trait PainterRedirectedArgs {
    unsafe fn exec(self) -> *mut ::paint_device::PaintDevice;
  }
  impl PainterRedirectedArgs for *const ::paint_device::PaintDevice {
    unsafe fn exec(self) -> *mut ::paint_device::PaintDevice {
      let device = self;
      ::ffi::qt_gui_c_QPainter_redirected_device(device)
    }
  }
  impl PainterRedirectedArgs for (*const ::paint_device::PaintDevice, *mut ::qt_core::point::Point) {
    unsafe fn exec(self) -> *mut ::paint_device::PaintDevice {
      let device = self.0;
      let offset = self.1;
      ::ffi::qt_gui_c_QPainter_redirected_device_offset(device, offset)
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_brush](../struct.Painter.html#method.set_brush) method.
  pub trait PainterSetBrushArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetBrushArgs<'largs> for &'largs ::brush::Brush {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let brush = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setBrush_brush(original_self as *mut ::painter::Painter,
                                                brush as *const ::brush::Brush)
      }
    }
  }
  impl<'largs> PainterSetBrushArgs<'largs> for &'largs ::qt_core::qt::BrushStyle {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let style = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setBrush_style(original_self as *mut ::painter::Painter,
                                                style as *const ::qt_core::qt::BrushStyle)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_brush_origin](../struct.Painter.html#method.set_brush_origin) method.
  pub trait PainterSetBrushOriginArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetBrushOriginArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setBrushOrigin_QPoint(original_self as *mut ::painter::Painter,
                                                       arg1 as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> PainterSetBrushOriginArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setBrushOrigin_QPointF(original_self as *mut ::painter::Painter,
                                                        arg1 as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> PainterSetBrushOriginArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_gui_c_QPainter_setBrushOrigin_int_int(original_self as *mut ::painter::Painter, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_clip_path](../struct.Painter.html#method.set_clip_path) method.
  pub trait PainterSetClipPathArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetClipPathArgs<'largs> for &'largs ::painter_path::PainterPath {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let path = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setClipPath_path(original_self as *mut ::painter::Painter,
                                                  path as *const ::painter_path::PainterPath)
      }
    }
  }
  impl<'largs> PainterSetClipPathArgs<'largs>
    for (&'largs ::painter_path::PainterPath, &'largs ::qt_core::qt::ClipOperation) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let path = self.0;
      let op = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setClipPath_path_op(original_self as *mut ::painter::Painter,
                                                     path as *const ::painter_path::PainterPath,
                                                     op as *const ::qt_core::qt::ClipOperation)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_clip_rect](../struct.Painter.html#method.set_clip_rect) method.
  pub trait PainterSetClipRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetClipRectArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setClipRect_QRect(original_self as *mut ::painter::Painter,
                                                   arg1 as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PainterSetClipRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setClipRect_QRectF(original_self as *mut ::painter::Painter,
                                                    arg1 as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> PainterSetClipRectArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, &'largs ::qt_core::qt::ClipOperation) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let op = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setClipRect_QRectF_Qt_ClipOperation(original_self as *mut ::painter::Painter,
                                                                     arg1 as *const ::qt_core::rect_f::RectF,
                                                                     op as *const ::qt_core::qt::ClipOperation)
      }
    }
  }
  impl<'largs> PainterSetClipRectArgs<'largs> for (&'largs ::qt_core::rect::Rect, &'largs ::qt_core::qt::ClipOperation) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let op = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setClipRect_QRect_Qt_ClipOperation(original_self as *mut ::painter::Painter,
                                                                    arg1 as *const ::qt_core::rect::Rect,
                                                                    op as *const ::qt_core::qt::ClipOperation)
      }
    }
  }
  impl<'largs> PainterSetClipRectArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setClipRect_int_int_int_int(original_self as *mut ::painter::Painter, x, y, w, h)
      }
    }
  }
  impl<'largs> PainterSetClipRectArgs<'largs>
    for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int, &'largs ::qt_core::qt::ClipOperation) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let op = self.4;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setClipRect_int_int_int_int_Qt_ClipOperation(original_self as *mut ::painter::Painter, x, y, w, h, op as *const ::qt_core::qt::ClipOperation)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_clip_region](../struct.Painter.html#method.set_clip_region) method.
  pub trait PainterSetClipRegionArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetClipRegionArgs<'largs> for &'largs ::region::Region {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setClipRegion_arg1(original_self as *mut ::painter::Painter,
                                                    arg1 as *const ::region::Region)
      }
    }
  }
  impl<'largs> PainterSetClipRegionArgs<'largs> for (&'largs ::region::Region, &'largs ::qt_core::qt::ClipOperation) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let arg1 = self.0;
      let op = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setClipRegion_arg1_op(original_self as *mut ::painter::Painter,
                                                       arg1 as *const ::region::Region,
                                                       op as *const ::qt_core::qt::ClipOperation)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_matrix](../struct.Painter.html#method.set_matrix) method.
  pub trait PainterSetMatrixArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetMatrixArgs<'largs> for &'largs ::matrix::Matrix {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let matrix = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setMatrix_matrix(original_self as *mut ::painter::Painter,
                                                  matrix as *const ::matrix::Matrix)
      }
    }
  }
  impl<'largs> PainterSetMatrixArgs<'largs> for (&'largs ::matrix::Matrix, bool) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let matrix = self.0;
      let combine = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setMatrix_matrix_combine(original_self as *mut ::painter::Painter,
                                                          matrix as *const ::matrix::Matrix,
                                                          combine)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_pen](../struct.Painter.html#method.set_pen) method.
  pub trait PainterSetPenArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetPenArgs<'largs> for &'largs ::color::Color {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let color = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setPen_color(original_self as *mut ::painter::Painter,
                                              color as *const ::color::Color)
      }
    }
  }
  impl<'largs> PainterSetPenArgs<'largs> for &'largs ::pen::Pen {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let pen = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setPen_pen(original_self as *mut ::painter::Painter,
                                            pen as *const ::pen::Pen)
      }
    }
  }
  impl<'largs> PainterSetPenArgs<'largs> for &'largs ::qt_core::qt::PenStyle {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let style = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setPen_style(original_self as *mut ::painter::Painter,
                                              style as *const ::qt_core::qt::PenStyle)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_redirected](../struct.Painter.html#method.set_redirected) method.
  pub trait PainterSetRedirectedArgs {
    unsafe fn exec(self) -> ();
  }
  impl PainterSetRedirectedArgs for (*const ::paint_device::PaintDevice, *mut ::paint_device::PaintDevice) {
    unsafe fn exec(self) -> () {
      let device = self.0;
      let replacement = self.1;
      ::ffi::qt_gui_c_QPainter_setRedirected_device_replacement(device, replacement)
    }
  }
  impl<'a> PainterSetRedirectedArgs
    for (*const ::paint_device::PaintDevice, *mut ::paint_device::PaintDevice, &'a ::qt_core::point::Point) {
    unsafe fn exec(self) -> () {
      let device = self.0;
      let replacement = self.1;
      let offset = self.2;
      ::ffi::qt_gui_c_QPainter_setRedirected_device_replacement_offset(device,
                                                                       replacement,
                                                                       offset as *const ::qt_core::point::Point)
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_render_hint](../struct.Painter.html#method.set_render_hint) method.
  pub trait PainterSetRenderHintArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetRenderHintArgs<'largs> for ::painter::RenderHint {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let hint = self;
      unsafe { ::ffi::qt_gui_c_QPainter_setRenderHint_hint(original_self as *mut ::painter::Painter, hint) }
    }
  }
  impl<'largs> PainterSetRenderHintArgs<'largs> for (::painter::RenderHint, bool) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let hint = self.0;
      let on = self.1;
      unsafe { ::ffi::qt_gui_c_QPainter_setRenderHint_hint_on(original_self as *mut ::painter::Painter, hint, on) }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_render_hints](../struct.Painter.html#method.set_render_hints) method.
  pub trait PainterSetRenderHintsArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetRenderHintsArgs<'largs> for ::qt_core::flags::Flags<::painter::RenderHint> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let hints = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setRenderHints_hints(original_self as *mut ::painter::Painter,
                                                      hints.to_int() as ::libc::c_uint)
      }
    }
  }
  impl<'largs> PainterSetRenderHintsArgs<'largs> for (::qt_core::flags::Flags<::painter::RenderHint>, bool) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let hints = self.0;
      let on = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setRenderHints_hints_on(original_self as *mut ::painter::Painter,
                                                         hints.to_int() as ::libc::c_uint,
                                                         on)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_transform](../struct.Painter.html#method.set_transform) method.
  pub trait PainterSetTransformArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetTransformArgs<'largs> for &'largs ::transform::Transform {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let transform = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setTransform_transform(original_self as *mut ::painter::Painter,
                                                        transform as *const ::transform::Transform)
      }
    }
  }
  impl<'largs> PainterSetTransformArgs<'largs> for (&'largs ::transform::Transform, bool) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let transform = self.0;
      let combine = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setTransform_transform_combine(original_self as *mut ::painter::Painter,
                                                                transform as *const ::transform::Transform,
                                                                combine)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_viewport](../struct.Painter.html#method.set_viewport) method.
  pub trait PainterSetViewportArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetViewportArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let viewport = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setViewport_viewport(original_self as *mut ::painter::Painter,
                                                      viewport as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PainterSetViewportArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe { ::ffi::qt_gui_c_QPainter_setViewport_x_y_w_h(original_self as *mut ::painter::Painter, x, y, w, h) }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_window](../struct.Painter.html#method.set_window) method.
  pub trait PainterSetWindowArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetWindowArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let window = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setWindow_window(original_self as *mut ::painter::Painter,
                                                  window as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PainterSetWindowArgs<'largs> for (::libc::c_int, ::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe { ::ffi::qt_gui_c_QPainter_setWindow_x_y_w_h(original_self as *mut ::painter::Painter, x, y, w, h) }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_world_matrix](../struct.Painter.html#method.set_world_matrix) method.
  pub trait PainterSetWorldMatrixArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetWorldMatrixArgs<'largs> for &'largs ::matrix::Matrix {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let matrix = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setWorldMatrix_matrix(original_self as *mut ::painter::Painter,
                                                       matrix as *const ::matrix::Matrix)
      }
    }
  }
  impl<'largs> PainterSetWorldMatrixArgs<'largs> for (&'largs ::matrix::Matrix, bool) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let matrix = self.0;
      let combine = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setWorldMatrix_matrix_combine(original_self as *mut ::painter::Painter,
                                                               matrix as *const ::matrix::Matrix,
                                                               combine)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::set_world_transform](../struct.Painter.html#method.set_world_transform) method.
  pub trait PainterSetWorldTransformArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterSetWorldTransformArgs<'largs> for &'largs ::transform::Transform {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let matrix = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setWorldTransform_matrix(original_self as *mut ::painter::Painter,
                                                          matrix as *const ::transform::Transform)
      }
    }
  }
  impl<'largs> PainterSetWorldTransformArgs<'largs> for (&'largs ::transform::Transform, bool) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let matrix = self.0;
      let combine = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainter_setWorldTransform_matrix_combine(original_self as *mut ::painter::Painter,
                                                                  matrix as *const ::transform::Transform,
                                                                  combine)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Painter::translate](../struct.Painter.html#method.translate) method.
  pub trait PainterTranslateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> ();
  }
  impl<'largs> PainterTranslateArgs<'largs> for &'largs ::qt_core::point::Point {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let offset = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_translate_QPoint(original_self as *mut ::painter::Painter,
                                                  offset as *const ::qt_core::point::Point)
      }
    }
  }
  impl<'largs> PainterTranslateArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let offset = self;
      unsafe {
        ::ffi::qt_gui_c_QPainter_translate_QPointF(original_self as *mut ::painter::Painter,
                                                   offset as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> PainterTranslateArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter::Painter) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe { ::ffi::qt_gui_c_QPainter_translate_double_double(original_self as *mut ::painter::Painter, dx, dy) }
    }
  }
  /// This trait represents a set of arguments accepted by [PixmapFragment::create](../struct.PixmapFragment.html#method.create) method.
  pub trait PixmapFragmentCreateArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::painter::PixmapFragment>;
  }
  impl<'a> PixmapFragmentCreateArgs for (&'a ::qt_core::point_f::PointF, &'a ::qt_core::rect_f::RectF) {
    fn exec(self) -> ::cpp_utils::CppBox<::painter::PixmapFragment> {
      let pos = self.0;
      let source_rect = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect(pos as *const ::qt_core::point_f::PointF, source_rect as *const ::qt_core::rect_f::RectF) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PixmapFragmentCreateArgs for (&'a ::qt_core::point_f::PointF, &'a ::qt_core::rect_f::RectF, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::painter::PixmapFragment> {
      let pos = self.0;
      let source_rect = self.1;
      let scale_x = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX(pos as *const ::qt_core::point_f::PointF, source_rect as *const ::qt_core::rect_f::RectF, scale_x) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PixmapFragmentCreateArgs
    for (&'a ::qt_core::point_f::PointF, &'a ::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::painter::PixmapFragment> {
      let pos = self.0;
      let source_rect = self.1;
      let scale_x = self.2;
      let scale_y = self.3;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX_scaleY(pos as *const ::qt_core::point_f::PointF, source_rect as *const ::qt_core::rect_f::RectF, scale_x, scale_y) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PixmapFragmentCreateArgs
    for (&'a ::qt_core::point_f::PointF,
                                             &'a ::qt_core::rect_f::RectF,
                                             ::libc::c_double,
                                             ::libc::c_double,
                                             ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::painter::PixmapFragment> {
      let pos = self.0;
      let source_rect = self.1;
      let scale_x = self.2;
      let scale_y = self.3;
      let rotation = self.4;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX_scaleY_rotation(pos as *const ::qt_core::point_f::PointF, source_rect as *const ::qt_core::rect_f::RectF, scale_x, scale_y, rotation) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> PixmapFragmentCreateArgs
    for (&'a ::qt_core::point_f::PointF,
                                             &'a ::qt_core::rect_f::RectF,
                                             ::libc::c_double,
                                             ::libc::c_double,
                                             ::libc::c_double,
                                             ::libc::c_double) {
    fn exec(self) -> ::cpp_utils::CppBox<::painter::PixmapFragment> {
      let pos = self.0;
      let source_rect = self.1;
      let scale_x = self.2;
      let scale_y = self.3;
      let rotation = self.4;
      let opacity = self.5;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QPainter_PixmapFragment_create_as_ptr_pos_sourceRect_scaleX_scaleY_rotation_opacity(pos as *const ::qt_core::point_f::PointF, source_rect as *const ::qt_core::rect_f::RectF, scale_x, scale_y, rotation, opacity) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
}
