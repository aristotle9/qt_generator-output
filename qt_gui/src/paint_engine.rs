/// C++ type: <span style='color: green;'>```QPaintEngine::DirtyFlag```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum DirtyFlag {
  /// C++ enum variant: <span style='color: green;'>```DirtyPen = 1```</span>
  DirtyPen = 1,
  /// C++ enum variant: <span style='color: green;'>```DirtyBrush = 2```</span>
  DirtyBrush = 2,
  /// C++ enum variant: <span style='color: green;'>```DirtyBrushOrigin = 4```</span>
  DirtyBrushOrigin = 4,
  /// C++ enum variant: <span style='color: green;'>```DirtyFont = 8```</span>
  DirtyFont = 8,
  /// C++ enum variant: <span style='color: green;'>```DirtyBackground = 16```</span>
  DirtyBackground = 16,
  /// C++ enum variant: <span style='color: green;'>```DirtyBackgroundMode = 32```</span>
  DirtyBackgroundMode = 32,
  /// C++ enum variant: <span style='color: green;'>```DirtyTransform = 64```</span>
  DirtyTransform = 64,
  /// C++ enum variant: <span style='color: green;'>```DirtyClipRegion = 128```</span>
  DirtyClipRegion = 128,
  /// C++ enum variant: <span style='color: green;'>```DirtyClipPath = 256```</span>
  DirtyClipPath = 256,
  /// C++ enum variant: <span style='color: green;'>```DirtyHints = 512```</span>
  DirtyHints = 512,
  /// C++ enum variant: <span style='color: green;'>```DirtyCompositionMode = 1024```</span>
  DirtyCompositionMode = 1024,
  /// C++ enum variant: <span style='color: green;'>```DirtyClipEnabled = 2048```</span>
  DirtyClipEnabled = 2048,
  /// C++ enum variant: <span style='color: green;'>```DirtyOpacity = 4096```</span>
  DirtyOpacity = 4096,
  /// C++ enum variant: <span style='color: green;'>```AllDirty = 65535```</span>
  AllDirty = 65535,
}

impl ::qt_core::flags::FlaggableEnum for DirtyFlag {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "DirtyFlag"
  }
}

/// C++ type: <span style='color: green;'>```QPaintEngine```</span>
#[repr(C)]
pub struct PaintEngine(u8);

impl PaintEngine {
  /// C++ method: <span style='color: green;'>```pure virtual bool QPaintEngine::begin(QPaintDevice* pdev)```</span>
  ///
  ///
  pub unsafe fn begin(&mut self, pdev: *mut ::paint_device::PaintDevice) -> bool {
    ::ffi::qt_gui_c_QPaintEngine_begin(self as *mut ::paint_engine::PaintEngine, pdev)
  }

  /// C++ method: <span style='color: green;'>```void QPaintEngine::clearDirty(QFlags<QPaintEngine::DirtyFlag> df)```</span>
  ///
  ///
  pub fn clear_dirty(&mut self, df: ::qt_core::flags::Flags<::paint_engine::DirtyFlag>) {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngine_clearDirty(self as *mut ::paint_engine::PaintEngine,
                                              df.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual QPoint QPaintEngine::coordinateOffset() const```</span>
  ///
  ///
  pub fn coordinate_offset(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPaintEngine_coordinateOffset_to_output(self as *const ::paint_engine::PaintEngine,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPaintEngine::drawEllipse```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_ellipse(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawEllipse(const QRect& r)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_ellipse(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawEllipse(const QRectF& r)```</span>
  ///
  ///
  pub fn draw_ellipse<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PaintEngineDrawEllipseArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPaintEngine::drawLines```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_lines(&mut self, (*const ::qt_core::line::Line, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawLines(const QLine* lines, int lineCount)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_lines(&mut self, (*const ::qt_core::line_f::LineF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawLines(const QLineF* lines, int lineCount)```</span>
  ///
  ///
  pub unsafe fn draw_lines<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PaintEngineDrawLinesArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawPath(const QPainterPath& path)```</span>
  ///
  ///
  pub fn draw_path(&mut self, path: &::painter_path::PainterPath) {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngine_drawPath(self as *mut ::paint_engine::PaintEngine,
                                            path as *const ::painter_path::PainterPath)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QPaintEngine::drawPixmap(const QRectF& r, const QPixmap& pm, const QRectF& sr)```</span>
  ///
  ///
  pub fn draw_pixmap(&mut self, r: &::qt_core::rect_f::RectF, pm: &::pixmap::Pixmap, sr: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngine_drawPixmap(self as *mut ::paint_engine::PaintEngine,
                                              r as *const ::qt_core::rect_f::RectF,
                                              pm as *const ::pixmap::Pixmap,
                                              sr as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```QPaintEngine::drawPoints```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_points(&mut self, (*const ::qt_core::point::Point, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawPoints(const QPoint* points, int pointCount)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_points(&mut self, (*const ::qt_core::point_f::PointF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawPoints(const QPointF* points, int pointCount)```</span>
  ///
  ///
  pub unsafe fn draw_points<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PaintEngineDrawPointsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPaintEngine::drawPolygon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_polygon(&mut self, (*const ::qt_core::point::Point, ::libc::c_int, ::paint_engine::PolygonDrawMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawPolygon(const QPoint* points, int pointCount, QPaintEngine::PolygonDrawMode mode)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_polygon(&mut self, (*const ::qt_core::point_f::PointF, ::libc::c_int, ::paint_engine::PolygonDrawMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawPolygon(const QPointF* points, int pointCount, QPaintEngine::PolygonDrawMode mode)```</span>
  ///
  ///
  pub unsafe fn draw_polygon<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PaintEngineDrawPolygonArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPaintEngine::drawRects```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn draw_rects(&mut self, (*const ::qt_core::rect::Rect, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawRects(const QRect* rects, int rectCount)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn draw_rects(&mut self, (*const ::qt_core::rect_f::RectF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawRects(const QRectF* rects, int rectCount)```</span>
  ///
  ///
  pub unsafe fn draw_rects<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PaintEngineDrawRectsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawTextItem(const QPointF& p, const QTextItem& textItem)```</span>
  ///
  ///
  pub fn draw_text_item(&mut self, p: &::qt_core::point_f::PointF, text_item: &::text_item::TextItem) {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngine_drawTextItem(self as *mut ::paint_engine::PaintEngine,
                                                p as *const ::qt_core::point_f::PointF,
                                                text_item as *const ::text_item::TextItem)
    }
  }

  /// C++ method: <span style='color: green;'>```virtual void QPaintEngine::drawTiledPixmap(const QRectF& r, const QPixmap& pixmap, const QPointF& s)```</span>
  ///
  ///
  pub fn draw_tiled_pixmap(&mut self,
                           r: &::qt_core::rect_f::RectF,
                           pixmap: &::pixmap::Pixmap,
                           s: &::qt_core::point_f::PointF) {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngine_drawTiledPixmap(self as *mut ::paint_engine::PaintEngine,
                                                   r as *const ::qt_core::rect_f::RectF,
                                                   pixmap as *const ::pixmap::Pixmap,
                                                   s as *const ::qt_core::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual bool QPaintEngine::end()```</span>
  ///
  ///
  pub fn end(&mut self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPaintEngine_end(self as *mut ::paint_engine::PaintEngine) }
  }

  /// C++ method: <span style='color: green;'>```void QPaintEngine::fix_neg_rect(int* x, int* y, int* w, int* h)```</span>
  ///
  ///
  pub unsafe fn fix_neg_rect(&mut self,
                             x: *mut ::libc::c_int,
                             y: *mut ::libc::c_int,
                             w: *mut ::libc::c_int,
                             h: *mut ::libc::c_int) {
    ::ffi::qt_gui_c_QPaintEngine_fix_neg_rect(self as *mut ::paint_engine::PaintEngine, x, y, w, h)
  }

  /// C++ method: <span style='color: green;'>```bool QPaintEngine::hasFeature(QFlags<QPaintEngine::PaintEngineFeature> feature) const```</span>
  ///
  ///
  pub fn has_feature(&self, feature: ::qt_core::flags::Flags<::paint_engine::PaintEngineFeature>) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngine_hasFeature(self as *const ::paint_engine::PaintEngine,
                                              feature.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPaintEngine::isActive() const```</span>
  ///
  ///
  pub fn is_active(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPaintEngine_isActive(self as *const ::paint_engine::PaintEngine) }
  }

  /// C++ method: <span style='color: green;'>```bool QPaintEngine::isExtended() const```</span>
  ///
  ///
  pub fn is_extended(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPaintEngine_isExtended(self as *const ::paint_engine::PaintEngine) }
  }

  /// C++ method: <span style='color: green;'>```QPaintDevice* QPaintEngine::paintDevice() const```</span>
  ///
  ///
  pub fn paint_device(&self) -> *mut ::paint_device::PaintDevice {
    unsafe { ::ffi::qt_gui_c_QPaintEngine_paintDevice(self as *const ::paint_engine::PaintEngine) }
  }

  /// C++ method: <span style='color: green;'>```QPainter* QPaintEngine::painter() const```</span>
  ///
  ///
  pub fn painter(&self) -> *mut ::painter::Painter {
    unsafe { ::ffi::qt_gui_c_QPaintEngine_painter(self as *const ::paint_engine::PaintEngine) }
  }

  /// C++ method: <span style='color: green;'>```void QPaintEngine::setActive(bool newState)```</span>
  ///
  ///
  pub fn set_active(&mut self, new_state: bool) {
    unsafe { ::ffi::qt_gui_c_QPaintEngine_setActive(self as *mut ::paint_engine::PaintEngine, new_state) }
  }

  /// C++ method: <span style='color: green;'>```void QPaintEngine::setDirty(QFlags<QPaintEngine::DirtyFlag> df)```</span>
  ///
  ///
  pub fn set_dirty(&mut self, df: ::qt_core::flags::Flags<::paint_engine::DirtyFlag>) {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngine_setDirty(self as *mut ::paint_engine::PaintEngine,
                                            df.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPaintEngine::setPaintDevice(QPaintDevice* device)```</span>
  ///
  ///
  pub unsafe fn set_paint_device(&mut self, device: *mut ::paint_device::PaintDevice) {
    ::ffi::qt_gui_c_QPaintEngine_setPaintDevice(self as *mut ::paint_engine::PaintEngine, device)
  }

  /// C++ method: <span style='color: green;'>```void QPaintEngine::setSystemClip(const QRegion& baseClip)```</span>
  ///
  ///
  pub fn set_system_clip(&mut self, base_clip: &::region::Region) {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngine_setSystemClip(self as *mut ::paint_engine::PaintEngine,
                                                 base_clip as *const ::region::Region)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPaintEngine::setSystemRect(const QRect& rect)```</span>
  ///
  ///
  pub fn set_system_rect(&mut self, rect: &::qt_core::rect::Rect) {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngine_setSystemRect(self as *mut ::paint_engine::PaintEngine,
                                                 rect as *const ::qt_core::rect::Rect)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPaintEngine::syncState()```</span>
  ///
  ///
  pub fn sync_state(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPaintEngine_syncState(self as *mut ::paint_engine::PaintEngine) }
  }

  /// C++ method: <span style='color: green;'>```QRegion QPaintEngine::systemClip() const```</span>
  ///
  ///
  pub fn system_clip(&self) -> ::cpp_utils::CppBox<::region::Region> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QPaintEngine_systemClip_as_ptr(self as *const ::paint_engine::PaintEngine) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QRect QPaintEngine::systemRect() const```</span>
  ///
  ///
  pub fn system_rect(&self) -> ::qt_core::rect::Rect {
    {
      let mut object: ::qt_core::rect::Rect =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPaintEngine_systemRect_to_output(self as *const ::paint_engine::PaintEngine, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPaintEngine::testDirty(QFlags<QPaintEngine::DirtyFlag> df)```</span>
  ///
  ///
  pub fn test_dirty(&mut self, df: ::qt_core::flags::Flags<::paint_engine::DirtyFlag>) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngine_testDirty(self as *mut ::paint_engine::PaintEngine,
                                             df.to_int() as ::libc::c_uint)
    }
  }

  /// C++ method: <span style='color: green;'>```pure virtual QPaintEngine::Type QPaintEngine::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::paint_engine::Type {
    unsafe { ::ffi::qt_gui_c_QPaintEngine_type(self as *const ::paint_engine::PaintEngine) }
  }

  /// C++ method: <span style='color: green;'>```pure virtual void QPaintEngine::updateState(const QPaintEngineState& state)```</span>
  ///
  ///
  pub fn update_state(&mut self, state: &::paint_engine_state::PaintEngineState) {
    unsafe {
      ::ffi::qt_gui_c_QPaintEngine_updateState(self as *mut ::paint_engine::PaintEngine,
                                               state as *const ::paint_engine_state::PaintEngineState)
    }
  }
}

impl ::cpp_utils::CppDeletable for ::paint_engine::PaintEngine {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QPaintEngine_delete
  }
}

/// C++ type: <span style='color: green;'>```QPaintEngine::PaintEngineFeature```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PaintEngineFeature {
  /// C++ enum variant: <span style='color: green;'>```AllFeatures = -1```</span>
  AllFeatures = -1,
  /// C++ enum variant: <span style='color: green;'>```PrimitiveTransform = 1```</span>
  PrimitiveTransform = 1,
  /// C++ enum variant: <span style='color: green;'>```PatternTransform = 2```</span>
  PatternTransform = 2,
  /// C++ enum variant: <span style='color: green;'>```PixmapTransform = 4```</span>
  PixmapTransform = 4,
  /// C++ enum variant: <span style='color: green;'>```PatternBrush = 8```</span>
  PatternBrush = 8,
  /// C++ enum variant: <span style='color: green;'>```LinearGradientFill = 16```</span>
  LinearGradientFill = 16,
  /// C++ enum variant: <span style='color: green;'>```RadialGradientFill = 32```</span>
  RadialGradientFill = 32,
  /// C++ enum variant: <span style='color: green;'>```ConicalGradientFill = 64```</span>
  ConicalGradientFill = 64,
  /// C++ enum variant: <span style='color: green;'>```AlphaBlend = 128```</span>
  AlphaBlend = 128,
  /// C++ enum variant: <span style='color: green;'>```PorterDuff = 256```</span>
  PorterDuff = 256,
  /// C++ enum variant: <span style='color: green;'>```PainterPaths = 512```</span>
  PainterPaths = 512,
  /// C++ enum variant: <span style='color: green;'>```Antialiasing = 1024```</span>
  Antialiasing = 1024,
  /// C++ enum variant: <span style='color: green;'>```BrushStroke = 2048```</span>
  BrushStroke = 2048,
  /// C++ enum variant: <span style='color: green;'>```ConstantOpacity = 4096```</span>
  ConstantOpacity = 4096,
  /// C++ enum variant: <span style='color: green;'>```MaskedBrush = 8192```</span>
  MaskedBrush = 8192,
  /// C++ enum variant: <span style='color: green;'>```PerspectiveTransform = 16384```</span>
  PerspectiveTransform = 16384,
  /// C++ enum variant: <span style='color: green;'>```BlendModes = 32768```</span>
  BlendModes = 32768,
  /// C++ enum variant: <span style='color: green;'>```ObjectBoundingModeGradients = 65536```</span>
  ObjectBoundingModeGradients = 65536,
  /// C++ enum variant: <span style='color: green;'>```RasterOpModes = 131072```</span>
  RasterOpModes = 131072,
  /// C++ enum variant: <span style='color: green;'>```PaintOutsidePaintEvent = 536870912```</span>
  PaintOutsidePaintEvent = 536870912,
}

impl ::qt_core::flags::FlaggableEnum for PaintEngineFeature {
  fn to_flag_value(self) -> ::libc::c_int {
    self as ::libc::c_int
  }
  fn enum_name() -> &'static str {
    "PaintEngineFeature"
  }
}

/// C++ type: <span style='color: green;'>```QPaintEngine::PolygonDrawMode```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum PolygonDrawMode {
  /// C++ enum variant: <span style='color: green;'>```OddEvenMode = 0```</span>
  OddEven = 0,
  /// C++ enum variant: <span style='color: green;'>```WindingMode = 1```</span>
  Winding = 1,
  /// C++ enum variant: <span style='color: green;'>```ConvexMode = 2```</span>
  Convex = 2,
  /// C++ enum variant: <span style='color: green;'>```PolylineMode = 3```</span>
  Polyline = 3,
}

/// C++ type: <span style='color: green;'>```QPaintEngine::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```X11 = 0```</span>
  X11 = 0,
  /// C++ enum variant: <span style='color: green;'>```Windows = 1```</span>
  Windows = 1,
  /// C++ enum variant: <span style='color: green;'>```QuickDraw = 2```</span>
  QuickDraw = 2,
  /// C++ enum variant: <span style='color: green;'>```CoreGraphics = 3```</span>
  CoreGraphics = 3,
  /// C++ enum variant: <span style='color: green;'>```MacPrinter = 4```</span>
  MacPrinter = 4,
  /// C++ enum variant: <span style='color: green;'>```QWindowSystem = 5```</span>
  QWindowSystem = 5,
  /// C++ enum variant: <span style='color: green;'>```PostScript = 6```</span>
  PostScript = 6,
  /// C++ enum variant: <span style='color: green;'>```OpenGL = 7```</span>
  OpenGL = 7,
  /// C++ enum variant: <span style='color: green;'>```Picture = 8```</span>
  Picture = 8,
  /// C++ enum variant: <span style='color: green;'>```SVG = 9```</span>
  SVG = 9,
  /// C++ enum variant: <span style='color: green;'>```Raster = 10```</span>
  Raster = 10,
  /// C++ enum variant: <span style='color: green;'>```Direct3D = 11```</span>
  Direct3D = 11,
  /// C++ enum variant: <span style='color: green;'>```Pdf = 12```</span>
  Pdf = 12,
  /// C++ enum variant: <span style='color: green;'>```OpenVG = 13```</span>
  OpenVG = 13,
  /// C++ enum variant: <span style='color: green;'>```OpenGL2 = 14```</span>
  OpenGL2 = 14,
  /// C++ enum variant: <span style='color: green;'>```PaintBuffer = 15```</span>
  PaintBuffer = 15,
  /// C++ enum variant: <span style='color: green;'>```Blitter = 16```</span>
  Blitter = 16,
  /// C++ enum variant: <span style='color: green;'>```Direct2D = 17```</span>
  Direct2D = 17,
  /// C++ enum variant: <span style='color: green;'>```User = 50```</span>
  User = 50,
  /// C++ enum variant: <span style='color: green;'>```MaxUser = 100```</span>
  MaxUser = 100,
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PaintEngine::draw_ellipse](../struct.PaintEngine.html#method.draw_ellipse) method.
  pub trait PaintEngineDrawEllipseArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> ();
  }
  impl<'largs> PaintEngineDrawEllipseArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> () {
      let r = self;
      unsafe {
        ::ffi::qt_gui_c_QPaintEngine_drawEllipse_QRect(original_self as *mut ::paint_engine::PaintEngine,
                                                       r as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> PaintEngineDrawEllipseArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> () {
      let r = self;
      unsafe {
        ::ffi::qt_gui_c_QPaintEngine_drawEllipse_QRectF(original_self as *mut ::paint_engine::PaintEngine,
                                                        r as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PaintEngine::draw_lines](../struct.PaintEngine.html#method.draw_lines) method.
  pub trait PaintEngineDrawLinesArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> ();
  }
  impl<'largs> PaintEngineDrawLinesArgs<'largs> for (*const ::qt_core::line_f::LineF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> () {
      let lines = self.0;
      let line_count = self.1;
      ::ffi::qt_gui_c_QPaintEngine_drawLines_QLineF_int(original_self as *mut ::paint_engine::PaintEngine,
                                                        lines,
                                                        line_count)
    }
  }
  impl<'largs> PaintEngineDrawLinesArgs<'largs> for (*const ::qt_core::line::Line, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> () {
      let lines = self.0;
      let line_count = self.1;
      ::ffi::qt_gui_c_QPaintEngine_drawLines_QLine_int(original_self as *mut ::paint_engine::PaintEngine,
                                                       lines,
                                                       line_count)
    }
  }
  /// This trait represents a set of arguments accepted by [PaintEngine::draw_points](../struct.PaintEngine.html#method.draw_points) method.
  pub trait PaintEngineDrawPointsArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> ();
  }
  impl<'largs> PaintEngineDrawPointsArgs<'largs> for (*const ::qt_core::point_f::PointF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> () {
      let points = self.0;
      let point_count = self.1;
      ::ffi::qt_gui_c_QPaintEngine_drawPoints_QPointF_int(original_self as *mut ::paint_engine::PaintEngine,
                                                          points,
                                                          point_count)
    }
  }
  impl<'largs> PaintEngineDrawPointsArgs<'largs> for (*const ::qt_core::point::Point, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> () {
      let points = self.0;
      let point_count = self.1;
      ::ffi::qt_gui_c_QPaintEngine_drawPoints_QPoint_int(original_self as *mut ::paint_engine::PaintEngine,
                                                         points,
                                                         point_count)
    }
  }
  /// This trait represents a set of arguments accepted by [PaintEngine::draw_polygon](../struct.PaintEngine.html#method.draw_polygon) method.
  pub trait PaintEngineDrawPolygonArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> ();
  }
  impl<'largs> PaintEngineDrawPolygonArgs<'largs>
    for (*const ::qt_core::point_f::PointF, ::libc::c_int, ::paint_engine::PolygonDrawMode) {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> () {
      let points = self.0;
      let point_count = self.1;
      let mode = self.2;
      ::ffi::qt_gui_c_QPaintEngine_drawPolygon_QPointF_int_QPaintEngine_PolygonDrawMode(original_self as *mut ::paint_engine::PaintEngine, points, point_count, mode)
    }
  }
  impl<'largs> PaintEngineDrawPolygonArgs<'largs>
    for (*const ::qt_core::point::Point, ::libc::c_int, ::paint_engine::PolygonDrawMode) {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> () {
      let points = self.0;
      let point_count = self.1;
      let mode = self.2;
      ::ffi::qt_gui_c_QPaintEngine_drawPolygon_QPoint_int_QPaintEngine_PolygonDrawMode(original_self as *mut ::paint_engine::PaintEngine, points, point_count, mode)
    }
  }
  /// This trait represents a set of arguments accepted by [PaintEngine::draw_rects](../struct.PaintEngine.html#method.draw_rects) method.
  pub trait PaintEngineDrawRectsArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> ();
  }
  impl<'largs> PaintEngineDrawRectsArgs<'largs> for (*const ::qt_core::rect_f::RectF, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> () {
      let rects = self.0;
      let rect_count = self.1;
      ::ffi::qt_gui_c_QPaintEngine_drawRects_QRectF_int(original_self as *mut ::paint_engine::PaintEngine,
                                                        rects,
                                                        rect_count)
    }
  }
  impl<'largs> PaintEngineDrawRectsArgs<'largs> for (*const ::qt_core::rect::Rect, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs mut ::paint_engine::PaintEngine) -> () {
      let rects = self.0;
      let rect_count = self.1;
      ::ffi::qt_gui_c_QPaintEngine_drawRects_QRect_int(original_self as *mut ::paint_engine::PaintEngine,
                                                       rects,
                                                       rect_count)
    }
  }
}
