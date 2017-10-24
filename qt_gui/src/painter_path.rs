/// C++ type: <span style='color: green;'>```QPainterPath::Element```</span>
#[repr(C)]
pub struct Element([u8; ::type_sizes::QT_GUI_PAINTER_PATH_ELEMENT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Element {
  unsafe fn new_uninitialized() -> Element {
    Element(::std::mem::uninitialized())
  }
}

impl Element {
  /// C++ method: <span style='color: green;'>```QPointF QPainterPath::Element::operator QPointF() const```</span>
  ///
  ///
  pub fn as_q_point_f(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_Element_convert_to_QPointF_to_output(self as *const ::painter_path::Element,
                                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPainterPath::Element::isCurveTo() const```</span>
  ///
  ///
  pub fn is_curve_to(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPainterPath_Element_isCurveTo(self as *const ::painter_path::Element) }
  }

  /// C++ method: <span style='color: green;'>```bool QPainterPath::Element::isLineTo() const```</span>
  ///
  ///
  pub fn is_line_to(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPainterPath_Element_isLineTo(self as *const ::painter_path::Element) }
  }

  /// C++ method: <span style='color: green;'>```bool QPainterPath::Element::isMoveTo() const```</span>
  ///
  ///
  pub fn is_move_to(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPainterPath_Element_isMoveTo(self as *const ::painter_path::Element) }
  }

  /// C++ method: <span style='color: green;'>```bool QPainterPath::Element::operator==(const QPainterPath::Element& e) const```</span>
  ///
  ///
  pub fn op_eq(&self, e: &::painter_path::Element) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPainterPath_Element_operator_eq(self as *const ::painter_path::Element,
                                                       e as *const ::painter_path::Element)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPainterPath::Element::operator!=(const QPainterPath::Element& e) const```</span>
  ///
  ///
  pub fn op_neq(&self, e: &::painter_path::Element) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPainterPath_Element_operator_neq(self as *const ::painter_path::Element,
                                                        e as *const ::painter_path::Element)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPath::Element::set_type(QPainterPath::ElementType value)```</span>
  ///
  ///
  pub fn set_type(&mut self, value: ::painter_path::ElementType) {
    unsafe { ::ffi::qt_gui_c_QPainterPath_Element_set_type(self as *mut ::painter_path::Element, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPath::Element::set_x(double value)```</span>
  ///
  ///
  pub fn set_x(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainterPath_Element_set_x(self as *mut ::painter_path::Element, value) }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPath::Element::set_y(double value)```</span>
  ///
  ///
  pub fn set_y(&mut self, value: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainterPath_Element_set_y(self as *mut ::painter_path::Element, value) }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::ElementType QPainterPath::Element::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::painter_path::ElementType {
    unsafe { ::ffi::qt_gui_c_QPainterPath_Element_type(self as *const ::painter_path::Element) }
  }

  /// C++ method: <span style='color: green;'>```double QPainterPath::Element::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainterPath_Element_x(self as *const ::painter_path::Element) }
  }

  /// C++ method: <span style='color: green;'>```double QPainterPath::Element::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainterPath_Element_y(self as *const ::painter_path::Element) }
  }
}

impl Drop for ::painter_path::Element {
  /// C++ method: <span style='color: green;'>```[destructor] void QPainterPath::Element::~QPainterPath::Element()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPainterPath_Element_destructor(self as *mut ::painter_path::Element) }
  }
}

/// C++ type: <span style='color: green;'>```QPainterPath::ElementType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum ElementType {
  /// C++ enum variant: <span style='color: green;'>```MoveToElement = 0```</span>
  MoveTo = 0,
  /// C++ enum variant: <span style='color: green;'>```LineToElement = 1```</span>
  LineTo = 1,
  /// C++ enum variant: <span style='color: green;'>```CurveToElement = 2```</span>
  CurveTo = 2,
  /// C++ enum variant: <span style='color: green;'>```CurveToDataElement = 3```</span>
  CurveToData = 3,
}

/// C++ type: <span style='color: green;'>```QPainterPath```</span>
#[repr(C)]
pub struct PainterPath([u8; ::type_sizes::QT_GUI_PAINTER_PATH_PAINTER_PATH]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PainterPath {
  unsafe fn new_uninitialized() -> PainterPath {
    PainterPath(::std::mem::uninitialized())
  }
}

impl PainterPath {
  /// C++ method: <span style='color: green;'>```QPainterPath::addEllipse```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_ellipse(&mut self, (&::qt_core::point_f::PointF, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addEllipse(const QPointF& center, double rx, double ry)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_ellipse(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addEllipse(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_ellipse(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addEllipse(double x, double y, double w, double h)```</span>
  ///
  ///
  pub fn add_ellipse<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathAddEllipseArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainterPath::addPath(const QPainterPath& path)```</span>
  ///
  ///
  pub fn add_path(&mut self, path: &::painter_path::PainterPath) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPath_addPath(self as *mut ::painter_path::PainterPath,
                                           path as *const ::painter_path::PainterPath)
    }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPath::addPolygon(const QPolygonF& polygon)```</span>
  ///
  ///
  pub fn add_polygon(&mut self, polygon: &::polygon_f::PolygonF) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPath_addPolygon(self as *mut ::painter_path::PainterPath,
                                              polygon as *const ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::addRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_rect(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addRect(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addRect(double x, double y, double w, double h)```</span>
  ///
  ///
  pub fn add_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathAddRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainterPath::addRegion(const QRegion& region)```</span>
  ///
  ///
  pub fn add_region(&mut self, region: &::region::Region) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPath_addRegion(self as *mut ::painter_path::PainterPath,
                                             region as *const ::region::Region)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::addRoundRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_round_rect(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addRoundRect(const QRectF& rect, int roundness)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_round_rect(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addRoundRect(const QRectF& rect, int xRnd, int yRnd)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_round_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addRoundRect(double x, double y, double w, double h, int roundness)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn add_round_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addRoundRect(double x, double y, double w, double h, int xRnd, int yRnd)```</span>
  ///
  ///
  pub fn add_round_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathAddRoundRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainterPath::addRoundedRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_rounded_rect(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addRoundedRect(const QRectF& rect, double xRadius, double yRadius)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_rounded_rect(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double, &::qt_core::qt::SizeMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addRoundedRect(const QRectF& rect, double xRadius, double yRadius, Qt::SizeMode mode = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn add_rounded_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addRoundedRect(double x, double y, double w, double h, double xRadius, double yRadius)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn add_rounded_rect(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, &::qt_core::qt::SizeMode)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addRoundedRect(double x, double y, double w, double h, double xRadius, double yRadius, Qt::SizeMode mode = ?)```</span>
  ///
  ///
  pub fn add_rounded_rect<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathAddRoundedRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainterPath::addText```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_text(&mut self, (&::qt_core::point_f::PointF, &::font::Font, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addText(const QPointF& point, const QFont& f, const QString& text)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_text(&mut self, (::libc::c_double, ::libc::c_double, &::font::Font, &::qt_core::string::String)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::addText(double x, double y, const QFont& f, const QString& text)```</span>
  ///
  ///
  pub fn add_text<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathAddTextArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QPainterPath::angleAtPercent(double t) const```</span>
  ///
  ///
  pub fn angle_at_percent(&self, t: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainterPath_angleAtPercent(self as *const ::painter_path::PainterPath, t) }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::arcMoveTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn arc_move_to(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::arcMoveTo(const QRectF& rect, double angle)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn arc_move_to(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::arcMoveTo(double x, double y, double w, double h, double angle)```</span>
  ///
  ///
  pub fn arc_move_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathArcMoveToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainterPath::arcTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn arc_to(&mut self, (&::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::arcTo(const QRectF& rect, double startAngle, double arcLength)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn arc_to(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::arcTo(double x, double y, double w, double h, double startAngle, double arcLength)```</span>
  ///
  ///
  pub fn arc_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathArcToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF QPainterPath::boundingRect() const```</span>
  ///
  ///
  pub fn bounding_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_boundingRect_to_output(self as *const ::painter_path::PainterPath, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPath::closeSubpath()```</span>
  ///
  ///
  pub fn close_subpath(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPainterPath_closeSubpath(self as *mut ::painter_path::PainterPath) }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPath::connectPath(const QPainterPath& path)```</span>
  ///
  ///
  pub fn connect_path(&mut self, path: &::painter_path::PainterPath) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPath_connectPath(self as *mut ::painter_path::PainterPath,
                                               path as *const ::painter_path::PainterPath)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::contains```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn contains(&self, &::painter_path::PainterPath) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPainterPath::contains(const QPainterPath& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn contains(&self, &::qt_core::point_f::PointF) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPainterPath::contains(const QPointF& pt) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn contains(&self, &::qt_core::rect_f::RectF) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPainterPath::contains(const QRectF& rect) const```</span>
  ///
  ///
  pub fn contains<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::PainterPathContainsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF QPainterPath::controlPointRect() const```</span>
  ///
  ///
  pub fn control_point_rect(&self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_controlPointRect_to_output(self as *const ::painter_path::PainterPath,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::cubicTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn cubic_to(&mut self, (&::qt_core::point_f::PointF, &::qt_core::point_f::PointF, &::qt_core::point_f::PointF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::cubicTo(const QPointF& ctrlPt1, const QPointF& ctrlPt2, const QPointF& endPt)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn cubic_to(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::cubicTo(double ctrlPt1x, double ctrlPt1y, double ctrlPt2x, double ctrlPt2y, double endPtx, double endPty)```</span>
  ///
  ///
  pub fn cubic_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathCubicToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPointF QPainterPath::currentPosition() const```</span>
  ///
  ///
  pub fn current_position(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_currentPosition_to_output(self as *const ::painter_path::PainterPath, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::Element QPainterPath::elementAt(int i) const```</span>
  ///
  ///
  pub fn element_at(&self, i: ::libc::c_int) -> ::painter_path::Element {
    {
      let mut object: ::painter_path::Element =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_elementAt_to_output(self as *const ::painter_path::PainterPath, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QPainterPath::elementCount() const```</span>
  ///
  ///
  pub fn element_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QPainterPath_elementCount(self as *const ::painter_path::PainterPath) }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::intersected(const QPainterPath& r) const```</span>
  ///
  ///
  pub fn intersected(&self, r: &::painter_path::PainterPath) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_intersected_to_output(self as *const ::painter_path::PainterPath,
                                                           r as *const ::painter_path::PainterPath,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::intersects```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn intersects(&self, &::painter_path::PainterPath) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPainterPath::intersects(const QPainterPath& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn intersects(&self, &::qt_core::rect_f::RectF) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QPainterPath::intersects(const QRectF& rect) const```</span>
  ///
  ///
  pub fn intersects<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::PainterPathIntersectsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QPainterPath::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QPainterPath_isEmpty(self as *const ::painter_path::PainterPath) }
  }

  /// C++ method: <span style='color: green;'>```double QPainterPath::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainterPath_length(self as *const ::painter_path::PainterPath) }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::lineTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn line_to(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::lineTo(const QPointF& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn line_to(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::lineTo(double x, double y)```</span>
  ///
  ///
  pub fn line_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathLineToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainterPath::moveTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn move_to(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::moveTo(const QPointF& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn move_to(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::moveTo(double x, double y)```</span>
  ///
  ///
  pub fn move_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathMoveToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainterPath::QPainterPath```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPainterPath::QPainterPath()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::painter_path::PainterPath) -> ::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPainterPath::QPainterPath(const QPainterPath& other)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::point_f::PointF) -> ::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPainterPath::QPainterPath(const QPointF& startPoint)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::painter_path::PainterPath
    where Args: overloading::PainterPathNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::operator+(const QPainterPath& other) const```</span>
  ///
  ///
  pub fn op_add(&self, other: &::painter_path::PainterPath) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_operator_add_to_output(self as *const ::painter_path::PainterPath,
                                                            other as *const ::painter_path::PainterPath,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath& QPainterPath::operator+=(const QPainterPath& other)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 other: &'l1 ::painter_path::PainterPath)
                                 -> &'l0 mut ::painter_path::PainterPath {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPainterPath_operator_add_assign(self as *mut ::painter_path::PainterPath,
                                                       other as *const ::painter_path::PainterPath)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPainterPath& QPainterPath::operator=(const QPainterPath& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::painter_path::PainterPath)
                             -> &'l0 mut ::painter_path::PainterPath {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPainterPath_operator_assign(self as *mut ::painter_path::PainterPath,
                                                   other as *const ::painter_path::PainterPath)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::operator&(const QPainterPath& other) const```</span>
  ///
  ///
  pub fn op_bit_and(&self, other: &::painter_path::PainterPath) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_operator_bit_and_to_output(self as *const ::painter_path::PainterPath,
                                                                other as *const ::painter_path::PainterPath,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath& QPainterPath::operator&=(const QPainterPath& other)```</span>
  ///
  ///
  pub fn op_bit_and_assign<'l0, 'l1>(&'l0 mut self,
                                     other: &'l1 ::painter_path::PainterPath)
                                     -> &'l0 mut ::painter_path::PainterPath {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPainterPath_operator_bit_and_assign(self as *mut ::painter_path::PainterPath,
                                                           other as *const ::painter_path::PainterPath)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::operator|(const QPainterPath& other) const```</span>
  ///
  ///
  pub fn op_bit_or(&self, other: &::painter_path::PainterPath) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_operator_bit_or_to_output(self as *const ::painter_path::PainterPath,
                                                               other as *const ::painter_path::PainterPath,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath& QPainterPath::operator|=(const QPainterPath& other)```</span>
  ///
  ///
  pub fn op_bit_or_assign<'l0, 'l1>(&'l0 mut self,
                                    other: &'l1 ::painter_path::PainterPath)
                                    -> &'l0 mut ::painter_path::PainterPath {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPainterPath_operator_bit_or_assign(self as *mut ::painter_path::PainterPath,
                                                          other as *const ::painter_path::PainterPath)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QPainterPath::operator==(const QPainterPath& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::painter_path::PainterPath) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPainterPath_operator_eq(self as *const ::painter_path::PainterPath,
                                               other as *const ::painter_path::PainterPath)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPainterPath::operator!=(const QPainterPath& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::painter_path::PainterPath) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QPainterPath_operator_neq(self as *const ::painter_path::PainterPath,
                                                other as *const ::painter_path::PainterPath)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::operator-(const QPainterPath& other) const```</span>
  ///
  ///
  pub fn op_sub(&self, other: &::painter_path::PainterPath) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_operator_sub_to_output(self as *const ::painter_path::PainterPath,
                                                            other as *const ::painter_path::PainterPath,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath& QPainterPath::operator-=(const QPainterPath& other)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self,
                                 other: &'l1 ::painter_path::PainterPath)
                                 -> &'l0 mut ::painter_path::PainterPath {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QPainterPath_operator_sub_assign(self as *mut ::painter_path::PainterPath,
                                                       other as *const ::painter_path::PainterPath)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double QPainterPath::percentAtLength(double t) const```</span>
  ///
  ///
  pub fn percent_at_length(&self, t: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainterPath_percentAtLength(self as *const ::painter_path::PainterPath, t) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QPainterPath::pointAtPercent(double t) const```</span>
  ///
  ///
  pub fn point_at_percent(&self, t: ::libc::c_double) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_pointAtPercent_to_output(self as *const ::painter_path::PainterPath,
                                                              t,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::quadTo```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn quad_to(&mut self, (&::qt_core::point_f::PointF, &::qt_core::point_f::PointF)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::quadTo(const QPointF& ctrlPt, const QPointF& endPt)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn quad_to(&mut self, (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::quadTo(double ctrlPtx, double ctrlPty, double endPtx, double endPty)```</span>
  ///
  ///
  pub fn quad_to<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathQuadToArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QPainterPath::setElementPositionAt(int i, double x, double y)```</span>
  ///
  ///
  pub fn set_element_position_at(&mut self, i: ::libc::c_int, x: ::libc::c_double, y: ::libc::c_double) {
    unsafe { ::ffi::qt_gui_c_QPainterPath_setElementPositionAt(self as *mut ::painter_path::PainterPath, i, x, y) }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPath::setFillRule(Qt::FillRule fillRule)```</span>
  ///
  ///
  pub fn set_fill_rule(&mut self, fill_rule: &::qt_core::qt::FillRule) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPath_setFillRule(self as *mut ::painter_path::PainterPath,
                                               fill_rule as *const ::qt_core::qt::FillRule)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::simplified() const```</span>
  ///
  ///
  pub fn simplified(&self) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_simplified_to_output(self as *const ::painter_path::PainterPath, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QPainterPath::slopeAtPercent(double t) const```</span>
  ///
  ///
  pub fn slope_at_percent(&self, t: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QPainterPath_slopeAtPercent(self as *const ::painter_path::PainterPath, t) }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::subtracted(const QPainterPath& r) const```</span>
  ///
  ///
  pub fn subtracted(&self, r: &::painter_path::PainterPath) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_subtracted_to_output(self as *const ::painter_path::PainterPath,
                                                          r as *const ::painter_path::PainterPath,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::subtractedInverted(const QPainterPath& r) const```</span>
  ///
  ///
  pub fn subtracted_inverted(&self, r: &::painter_path::PainterPath) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_subtractedInverted_to_output(self as *const ::painter_path::PainterPath,
                                                                  r as *const ::painter_path::PainterPath,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QPainterPath::swap(QPainterPath& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::painter_path::PainterPath) {
    unsafe {
      ::ffi::qt_gui_c_QPainterPath_swap(self as *mut ::painter_path::PainterPath,
                                        other as *mut ::painter_path::PainterPath)
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::toFillPolygon```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_fill_polygon(&self, ()) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QPainterPath::toFillPolygon() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_fill_polygon(&self, &::matrix::Matrix) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QPainterPath::toFillPolygon(const QMatrix& matrix = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn to_fill_polygon(&self, &::transform::Transform) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QPainterPath::toFillPolygon(const QTransform& matrix) const```</span>
  ///
  ///
  pub fn to_fill_polygon<'largs, Args>(&'largs self, args: Args) -> ::polygon_f::PolygonF
    where Args: overloading::PainterPathToFillPolygonArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainterPath::toFillPolygons```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_fill_polygons(&self, ()) -> ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF> QPainterPath::toFillPolygons() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_fill_polygons(&self, &::matrix::Matrix) -> ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF> QPainterPath::toFillPolygons(const QMatrix& matrix = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn to_fill_polygons(&self, &::transform::Transform) -> ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF> QPainterPath::toFillPolygons(const QTransform& matrix) const```</span>
  ///
  ///
  pub fn to_fill_polygons<'largs, Args>(&'largs self, args: Args) -> ::list::ListPolygonF
    where Args: overloading::PainterPathToFillPolygonsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::toReversed() const```</span>
  ///
  ///
  pub fn to_reversed(&self) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_toReversed_to_output(self as *const ::painter_path::PainterPath, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPainterPath::toSubpathPolygons```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_subpath_polygons(&self, ()) -> ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF> QPainterPath::toSubpathPolygons() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_subpath_polygons(&self, &::matrix::Matrix) -> ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF> QPainterPath::toSubpathPolygons(const QMatrix& matrix = ?) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn to_subpath_polygons(&self, &::transform::Transform) -> ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF> QPainterPath::toSubpathPolygons(const QTransform& matrix) const```</span>
  ///
  ///
  pub fn to_subpath_polygons<'largs, Args>(&'largs self, args: Args) -> ::list::ListPolygonF
    where Args: overloading::PainterPathToSubpathPolygonsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainterPath::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&mut self, &::qt_core::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::translate(const QPointF& offset)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QPainterPath::translate(double dx, double dy)```</span>
  ///
  ///
  pub fn translate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::PainterPathTranslateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainterPath::translated```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translated(&self, &::qt_core::point_f::PointF) -> ::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::translated(const QPointF& offset) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translated(&self, (::libc::c_double, ::libc::c_double)) -> ::painter_path::PainterPath```<br>
  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::translated(double dx, double dy) const```</span>
  ///
  ///
  pub fn translated<'largs, Args>(&'largs self, args: Args) -> ::painter_path::PainterPath
    where Args: overloading::PainterPathTranslatedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPainterPath QPainterPath::united(const QPainterPath& r) const```</span>
  ///
  ///
  pub fn united(&self, r: &::painter_path::PainterPath) -> ::painter_path::PainterPath {
    {
      let mut object: ::painter_path::PainterPath =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_united_to_output(self as *const ::painter_path::PainterPath,
                                                      r as *const ::painter_path::PainterPath,
                                                      &mut object);
      }
      object
    }
  }
}

impl Drop for ::painter_path::PainterPath {
  /// C++ method: <span style='color: green;'>```[destructor] void QPainterPath::~QPainterPath()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QPainterPath_destructor(self as *mut ::painter_path::PainterPath) }
  }
}

/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPainterPath& arg2)```</span>
///
///
pub fn op_shl(arg1: &::qt_core::debug::Debug, arg2: &::painter_path::PainterPath) -> ::qt_core::debug::Debug {
  {
    let mut object: ::qt_core::debug::Debug =
      unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
    unsafe {
      ::ffi::qt_gui_c_QPainterPath_G_operator_shl_to_output(arg1 as *const ::qt_core::debug::Debug,
                                                            arg2 as *const ::painter_path::PainterPath,
                                                            &mut object);
    }
    object
  }
}

/// C++ method: <span style='color: green;'>```void swap(QPainterPath& value1, QPainterPath& value2)```</span>
///
///
pub fn swap(value1: &mut ::painter_path::PainterPath, value2: &mut ::painter_path::PainterPath) {
  unsafe {
    ::ffi::qt_gui_c_QPainterPath_G_swap(value1 as *mut ::painter_path::PainterPath,
                                        value2 as *mut ::painter_path::PainterPath)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PainterPath::add_ellipse](../struct.PainterPath.html#method.add_ellipse) method.
  pub trait PainterPathAddEllipseArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathAddEllipseArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let center = self.0;
      let rx = self.1;
      let ry = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_addEllipse_center_rx_ry(original_self as *mut ::painter_path::PainterPath,
                                                             center as *const ::qt_core::point_f::PointF,
                                                             rx,
                                                             ry)
      }
    }
  }
  impl<'largs> PainterPathAddEllipseArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_addEllipse_rect(original_self as *mut ::painter_path::PainterPath,
                                                     rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> PainterPathAddEllipseArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_addEllipse_x_y_w_h(original_self as *mut ::painter_path::PainterPath,
                                                        x,
                                                        y,
                                                        w,
                                                        h)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::add_rect](../struct.PainterPath.html#method.add_rect) method.
  pub trait PainterPathAddRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathAddRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_addRect_rect(original_self as *mut ::painter_path::PainterPath,
                                                  rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> PainterPathAddRectArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_addRect_x_y_w_h(original_self as *mut ::painter_path::PainterPath,
                                                     x,
                                                     y,
                                                     w,
                                                     h)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::add_round_rect](../struct.PainterPath.html#method.add_round_rect) method.
  pub trait PainterPathAddRoundRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathAddRoundRectArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let rect = self.0;
      let roundness = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_addRoundRect_rect_roundness(original_self as *mut ::painter_path::PainterPath,
                                                                 rect as *const ::qt_core::rect_f::RectF,
                                                                 roundness)
      }
    }
  }
  impl<'largs> PainterPathAddRoundRectArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let rect = self.0;
      let x_rnd = self.1;
      let y_rnd = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_addRoundRect_rect_xRnd_yRnd(original_self as *mut ::painter_path::PainterPath,
                                                                 rect as *const ::qt_core::rect_f::RectF,
                                                                 x_rnd,
                                                                 y_rnd)
      }
    }
  }
  impl<'largs> PainterPathAddRoundRectArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let roundness = self.4;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_addRoundRect_x_y_w_h_roundness(original_self as *mut ::painter_path::PainterPath,
                                                                    x,
                                                                    y,
                                                                    w,
                                                                    h,
                                                                    roundness)
      }
    }
  }
  impl<'largs> PainterPathAddRoundRectArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let x_rnd = self.4;
      let y_rnd = self.5;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_addRoundRect_x_y_w_h_xRnd_yRnd(original_self as *mut ::painter_path::PainterPath,
                                                                    x,
                                                                    y,
                                                                    w,
                                                                    h,
                                                                    x_rnd,
                                                                    y_rnd)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::add_rounded_rect](../struct.PainterPath.html#method.add_rounded_rect) method.
  pub trait PainterPathAddRoundedRectArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathAddRoundedRectArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let rect = self.0;
      let x_radius = self.1;
      let y_radius = self.2;
      unsafe { ::ffi::qt_gui_c_QPainterPath_addRoundedRect_rect_xRadius_yRadius(original_self as *mut ::painter_path::PainterPath, rect as *const ::qt_core::rect_f::RectF, x_radius, y_radius) }
    }
  }
  impl<'largs> PainterPathAddRoundedRectArgs<'largs>
    for (&'largs ::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double, &'largs ::qt_core::qt::SizeMode) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let rect = self.0;
      let x_radius = self.1;
      let y_radius = self.2;
      let mode = self.3;
      unsafe { ::ffi::qt_gui_c_QPainterPath_addRoundedRect_rect_xRadius_yRadius_mode(original_self as *mut ::painter_path::PainterPath, rect as *const ::qt_core::rect_f::RectF, x_radius, y_radius, mode as *const ::qt_core::qt::SizeMode) }
    }
  }
  impl<'largs> PainterPathAddRoundedRectArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let x_radius = self.4;
      let y_radius = self.5;
      unsafe { ::ffi::qt_gui_c_QPainterPath_addRoundedRect_x_y_w_h_xRadius_yRadius(original_self as *mut ::painter_path::PainterPath, x, y, w, h, x_radius, y_radius) }
    }
  }
  impl<'largs> PainterPathAddRoundedRectArgs<'largs>
    for (::libc::c_double,
                                                              ::libc::c_double,
                                                              ::libc::c_double,
                                                              ::libc::c_double,
                                                              ::libc::c_double,
                                                              ::libc::c_double,
                                                              &'largs ::qt_core::qt::SizeMode) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let x_radius = self.4;
      let y_radius = self.5;
      let mode = self.6;
      unsafe { ::ffi::qt_gui_c_QPainterPath_addRoundedRect_x_y_w_h_xRadius_yRadius_mode(original_self as *mut ::painter_path::PainterPath, x, y, w, h, x_radius, y_radius, mode as *const ::qt_core::qt::SizeMode) }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::add_text](../struct.PainterPath.html#method.add_text) method.
  pub trait PainterPathAddTextArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathAddTextArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF, &'largs ::font::Font, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let point = self.0;
      let f = self.1;
      let text = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_addText_point_f_text(original_self as *mut ::painter_path::PainterPath,
                                                          point as *const ::qt_core::point_f::PointF,
                                                          f as *const ::font::Font,
                                                          text as *const ::qt_core::string::String)
      }
    }
  }
  impl<'largs> PainterPathAddTextArgs<'largs>
    for (::libc::c_double, ::libc::c_double, &'largs ::font::Font, &'largs ::qt_core::string::String) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let x = self.0;
      let y = self.1;
      let f = self.2;
      let text = self.3;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_addText_x_y_f_text(original_self as *mut ::painter_path::PainterPath,
                                                        x,
                                                        y,
                                                        f as *const ::font::Font,
                                                        text as *const ::qt_core::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::arc_move_to](../struct.PainterPath.html#method.arc_move_to) method.
  pub trait PainterPathArcMoveToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathArcMoveToArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let rect = self.0;
      let angle = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_arcMoveTo_rect_angle(original_self as *mut ::painter_path::PainterPath,
                                                          rect as *const ::qt_core::rect_f::RectF,
                                                          angle)
      }
    }
  }
  impl<'largs> PainterPathArcMoveToArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let angle = self.4;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_arcMoveTo_x_y_w_h_angle(original_self as *mut ::painter_path::PainterPath,
                                                             x,
                                                             y,
                                                             w,
                                                             h,
                                                             angle)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::arc_to](../struct.PainterPath.html#method.arc_to) method.
  pub trait PainterPathArcToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathArcToArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let rect = self.0;
      let start_angle = self.1;
      let arc_length = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_arcTo_rect_startAngle_arcLength(original_self as *mut ::painter_path::PainterPath, rect as *const ::qt_core::rect_f::RectF, start_angle, arc_length)
      }
    }
  }
  impl<'largs> PainterPathArcToArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let x = self.0;
      let y = self.1;
      let w = self.2;
      let h = self.3;
      let start_angle = self.4;
      let arc_length = self.5;
      unsafe { ::ffi::qt_gui_c_QPainterPath_arcTo_x_y_w_h_startAngle_arcLength(original_self as *mut ::painter_path::PainterPath, x, y, w, h, start_angle, arc_length) }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::contains](../struct.PainterPath.html#method.contains) method.
  pub trait PainterPathContainsArgs<'largs> {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> bool;
  }
  impl<'largs> PainterPathContainsArgs<'largs> for &'largs ::painter_path::PainterPath {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> bool {
      let p = self;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_contains_p(original_self as *const ::painter_path::PainterPath,
                                                p as *const ::painter_path::PainterPath)
      }
    }
  }
  impl<'largs> PainterPathContainsArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> bool {
      let pt = self;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_contains_pt(original_self as *const ::painter_path::PainterPath,
                                                 pt as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> PainterPathContainsArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> bool {
      let rect = self;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_contains_rect(original_self as *const ::painter_path::PainterPath,
                                                   rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::cubic_to](../struct.PainterPath.html#method.cubic_to) method.
  pub trait PainterPathCubicToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathCubicToArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF, &'largs ::qt_core::point_f::PointF, &'largs ::qt_core::point_f::PointF) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let ctrl_pt1 = self.0;
      let ctrl_pt2 = self.1;
      let end_pt = self.2;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_cubicTo_ctrlPt1_ctrlPt2_endPt(original_self as *mut ::painter_path::PainterPath,
                                                                   ctrl_pt1 as *const ::qt_core::point_f::PointF,
                                                                   ctrl_pt2 as *const ::qt_core::point_f::PointF,
                                                                   end_pt as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> PainterPathCubicToArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let ctrl_pt_1x = self.0;
      let ctrl_pt_1y = self.1;
      let ctrl_pt_2x = self.2;
      let ctrl_pt_2y = self.3;
      let end_ptx = self.4;
      let end_pty = self.5;
      unsafe { ::ffi::qt_gui_c_QPainterPath_cubicTo_ctrlPt1x_ctrlPt1y_ctrlPt2x_ctrlPt2y_endPtx_endPty(original_self as *mut ::painter_path::PainterPath, ctrl_pt_1x, ctrl_pt_1y, ctrl_pt_2x, ctrl_pt_2y, end_ptx, end_pty) }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::intersects](../struct.PainterPath.html#method.intersects) method.
  pub trait PainterPathIntersectsArgs<'largs> {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> bool;
  }
  impl<'largs> PainterPathIntersectsArgs<'largs> for &'largs ::painter_path::PainterPath {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> bool {
      let p = self;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_intersects_p(original_self as *const ::painter_path::PainterPath,
                                                  p as *const ::painter_path::PainterPath)
      }
    }
  }
  impl<'largs> PainterPathIntersectsArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> bool {
      let rect = self;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_intersects_rect(original_self as *const ::painter_path::PainterPath,
                                                     rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::line_to](../struct.PainterPath.html#method.line_to) method.
  pub trait PainterPathLineToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathLineToArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let p = self;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_lineTo_p(original_self as *mut ::painter_path::PainterPath,
                                              p as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> PainterPathLineToArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_gui_c_QPainterPath_lineTo_x_y(original_self as *mut ::painter_path::PainterPath, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::move_to](../struct.PainterPath.html#method.move_to) method.
  pub trait PainterPathMoveToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathMoveToArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let p = self;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_moveTo_p(original_self as *mut ::painter_path::PainterPath,
                                              p as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> PainterPathMoveToArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_gui_c_QPainterPath_moveTo_x_y(original_self as *mut ::painter_path::PainterPath, x, y) }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::new](../struct.PainterPath.html#method.new) method.
  pub trait PainterPathNewArgs {
    fn exec(self) -> ::painter_path::PainterPath;
  }
  impl PainterPathNewArgs for () {
    fn exec(self) -> ::painter_path::PainterPath {

      {
        let mut object: ::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PainterPathNewArgs for &'a ::painter_path::PainterPath {
    fn exec(self) -> ::painter_path::PainterPath {
      let other = self;
      {
        let mut object: ::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_constructor_other(other as *const ::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  impl<'a> PainterPathNewArgs for &'a ::qt_core::point_f::PointF {
    fn exec(self) -> ::painter_path::PainterPath {
      let start_point = self;
      {
        let mut object: ::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_constructor_startPoint(start_point as *const ::qt_core::point_f::PointF,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::quad_to](../struct.PainterPath.html#method.quad_to) method.
  pub trait PainterPathQuadToArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathQuadToArgs<'largs>
    for (&'largs ::qt_core::point_f::PointF, &'largs ::qt_core::point_f::PointF) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let ctrl_pt = self.0;
      let end_pt = self.1;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_quadTo_ctrlPt_endPt(original_self as *mut ::painter_path::PainterPath,
                                                         ctrl_pt as *const ::qt_core::point_f::PointF,
                                                         end_pt as *const ::qt_core::point_f::PointF)
      }
    }
  }
  impl<'largs> PainterPathQuadToArgs<'largs>
    for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let ctrl_ptx = self.0;
      let ctrl_pty = self.1;
      let end_ptx = self.2;
      let end_pty = self.3;
      unsafe { ::ffi::qt_gui_c_QPainterPath_quadTo_ctrlPtx_ctrlPty_endPtx_endPty(original_self as *mut ::painter_path::PainterPath, ctrl_ptx, ctrl_pty, end_ptx, end_pty) }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::to_fill_polygon](../struct.PainterPath.html#method.to_fill_polygon) method.
  pub trait PainterPathToFillPolygonArgs<'largs> {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::polygon_f::PolygonF;
  }
  impl<'largs> PainterPathToFillPolygonArgs<'largs> for &'largs ::matrix::Matrix {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::polygon_f::PolygonF {
      let matrix = self;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_toFillPolygon_to_output_QMatrix(original_self as *const ::painter_path::PainterPath, matrix as *const ::matrix::Matrix, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PainterPathToFillPolygonArgs<'largs> for &'largs ::transform::Transform {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::polygon_f::PolygonF {
      let matrix = self;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_toFillPolygon_to_output_QTransform(original_self as *const ::painter_path::PainterPath, matrix as *const ::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PainterPathToFillPolygonArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::polygon_f::PolygonF {

      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_toFillPolygon_to_output_no_args(original_self as *const ::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::to_fill_polygons](../struct.PainterPath.html#method.to_fill_polygons) method.
  pub trait PainterPathToFillPolygonsArgs<'largs> {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::list::ListPolygonF;
  }
  impl<'largs> PainterPathToFillPolygonsArgs<'largs> for &'largs ::matrix::Matrix {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::list::ListPolygonF {
      let matrix = self;
      {
        let mut object: ::list::ListPolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_toFillPolygons_to_output_QMatrix(original_self as *const ::painter_path::PainterPath, matrix as *const ::matrix::Matrix, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PainterPathToFillPolygonsArgs<'largs> for &'largs ::transform::Transform {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::list::ListPolygonF {
      let matrix = self;
      {
        let mut object: ::list::ListPolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_toFillPolygons_to_output_QTransform(original_self as *const ::painter_path::PainterPath, matrix as *const ::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PainterPathToFillPolygonsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::list::ListPolygonF {

      {
        let mut object: ::list::ListPolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_toFillPolygons_to_output_no_args(original_self as *const ::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::to_subpath_polygons](../struct.PainterPath.html#method.to_subpath_polygons) method.
  pub trait PainterPathToSubpathPolygonsArgs<'largs> {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::list::ListPolygonF;
  }
  impl<'largs> PainterPathToSubpathPolygonsArgs<'largs> for &'largs ::matrix::Matrix {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::list::ListPolygonF {
      let matrix = self;
      {
        let mut object: ::list::ListPolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_toSubpathPolygons_to_output_QMatrix(original_self as *const ::painter_path::PainterPath, matrix as *const ::matrix::Matrix, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PainterPathToSubpathPolygonsArgs<'largs> for &'largs ::transform::Transform {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::list::ListPolygonF {
      let matrix = self;
      {
        let mut object: ::list::ListPolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_toSubpathPolygons_to_output_QTransform(original_self as *const ::painter_path::PainterPath, matrix as *const ::transform::Transform, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PainterPathToSubpathPolygonsArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::list::ListPolygonF {

      {
        let mut object: ::list::ListPolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_toSubpathPolygons_to_output_no_args(original_self as *const ::painter_path::PainterPath, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::translate](../struct.PainterPath.html#method.translate) method.
  pub trait PainterPathTranslateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> ();
  }
  impl<'largs> PainterPathTranslateArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe { ::ffi::qt_gui_c_QPainterPath_translate_dx_dy(original_self as *mut ::painter_path::PainterPath, dx, dy) }
    }
  }
  impl<'largs> PainterPathTranslateArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::painter_path::PainterPath) -> () {
      let offset = self;
      unsafe {
        ::ffi::qt_gui_c_QPainterPath_translate_offset(original_self as *mut ::painter_path::PainterPath,
                                                      offset as *const ::qt_core::point_f::PointF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [PainterPath::translated](../struct.PainterPath.html#method.translated) method.
  pub trait PainterPathTranslatedArgs<'largs> {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::painter_path::PainterPath;
  }
  impl<'largs> PainterPathTranslatedArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::painter_path::PainterPath {
      let dx = self.0;
      let dy = self.1;
      {
        let mut object: ::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_translated_to_output_dx_dy(original_self as *const ::painter_path::PainterPath, dx, dy, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> PainterPathTranslatedArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    fn exec(self, original_self: &'largs ::painter_path::PainterPath) -> ::painter_path::PainterPath {
      let offset = self;
      {
        let mut object: ::painter_path::PainterPath =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QPainterPath_translated_to_output_offset(original_self as *const ::painter_path::PainterPath, offset as *const ::qt_core::point_f::PointF, &mut object);
        }
        object
      }
    }
  }
}
