/// C++ type: <span style='color: green;'>```QLineF::IntersectType```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum IntersectType {
  /// C++ enum variant: <span style='color: green;'>```NoIntersection = 0```</span>
  No = 0,
  /// C++ enum variant: <span style='color: green;'>```BoundedIntersection = 1```</span>
  Bounded = 1,
  /// C++ enum variant: <span style='color: green;'>```UnboundedIntersection = 2```</span>
  Unbounded = 2,
}

/// C++ type: <span style='color: green;'>```QLineF```</span>
#[repr(C)]
pub struct LineF([u8; ::type_sizes::QT_CORE_LINE_F_LINE_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for LineF {
  unsafe fn new_uninitialized() -> LineF {
    LineF(::std::mem::uninitialized())
  }
}

impl LineF {
  /// C++ method: <span style='color: green;'>```QLineF::angle```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn angle(&self, ()) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QLineF::angle() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn angle(&self, &::line_f::LineF) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QLineF::angle(const QLineF& l) const```</span>
  ///
  ///
  pub fn angle<'largs, Args>(&'largs self, args: Args) -> ::libc::c_double
    where Args: overloading::LineFAngleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QLineF::angleTo(const QLineF& l) const```</span>
  ///
  ///
  pub fn angle_to(&self, l: &::line_f::LineF) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QLineF_angleTo(self as *const ::line_f::LineF, l as *const ::line_f::LineF) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QLineF::center() const```</span>
  ///
  ///
  pub fn center(&self) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLineF_center_to_output(self as *const ::line_f::LineF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QLineF::dx() const```</span>
  ///
  ///
  pub fn dx(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QLineF_dx(self as *const ::line_f::LineF) }
  }

  /// C++ method: <span style='color: green;'>```double QLineF::dy() const```</span>
  ///
  ///
  pub fn dy(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QLineF_dy(self as *const ::line_f::LineF) }
  }

  /// C++ method: <span style='color: green;'>```static QLineF QLineF::fromPolar(double length, double angle)```</span>
  ///
  ///
  pub fn from_polar(length: ::libc::c_double, angle: ::libc::c_double) -> ::line_f::LineF {
    {
      let mut object: ::line_f::LineF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLineF_fromPolar_to_output(length, angle, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLineF::IntersectType QLineF::intersect(const QLineF& l, QPointF* intersectionPoint) const```</span>
  ///
  ///
  pub unsafe fn intersect(&self,
                          l: &::line_f::LineF,
                          intersection_point: *mut ::point_f::PointF)
                          -> ::line_f::IntersectType {
    ::ffi::qt_core_c_QLineF_intersect(self as *const ::line_f::LineF,
                                      l as *const ::line_f::LineF,
                                      intersection_point)
  }

  /// C++ method: <span style='color: green;'>```bool QLineF::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QLineF_isNull(self as *const ::line_f::LineF) }
  }

  /// C++ method: <span style='color: green;'>```double QLineF::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QLineF_length(self as *const ::line_f::LineF) }
  }

  /// C++ method: <span style='color: green;'>```QLineF::QLineF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::line_f::LineF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLineF::QLineF()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::line::Line) -> ::line_f::LineF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLineF::QLineF(const QLine& line)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((&::point_f::PointF, &::point_f::PointF)) -> ::line_f::LineF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLineF::QLineF(const QPointF& pt1, const QPointF& pt2)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double)) -> ::line_f::LineF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QLineF::QLineF(double x1, double y1, double x2, double y2)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::line_f::LineF
    where Args: overloading::LineFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QLineF QLineF::normalVector() const```</span>
  ///
  ///
  pub fn normal_vector(&self) -> ::line_f::LineF {
    {
      let mut object: ::line_f::LineF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLineF_normalVector_to_output(self as *const ::line_f::LineF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QLineF::operator==(const QLineF& d) const```</span>
  ///
  ///
  pub fn op_eq(&self, d: &::line_f::LineF) -> bool {
    unsafe { ::ffi::qt_core_c_QLineF_operator_eq(self as *const ::line_f::LineF, d as *const ::line_f::LineF) }
  }

  /// C++ method: <span style='color: green;'>```bool QLineF::operator!=(const QLineF& d) const```</span>
  ///
  ///
  pub fn op_neq(&self, d: &::line_f::LineF) -> bool {
    unsafe { ::ffi::qt_core_c_QLineF_operator_neq(self as *const ::line_f::LineF, d as *const ::line_f::LineF) }
  }

  /// C++ method: <span style='color: green;'>```QPointF QLineF::p1() const```</span>
  ///
  ///
  pub fn p1(&self) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLineF_p1_to_output(self as *const ::line_f::LineF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QLineF::p2() const```</span>
  ///
  ///
  pub fn p2(&self) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLineF_p2_to_output(self as *const ::line_f::LineF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QLineF::pointAt(double t) const```</span>
  ///
  ///
  pub fn point_at(&self, t: ::libc::c_double) -> ::point_f::PointF {
    {
      let mut object: ::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLineF_pointAt_to_output(self as *const ::line_f::LineF, t, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QLineF::setAngle(double angle)```</span>
  ///
  ///
  pub fn set_angle(&mut self, angle: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QLineF_setAngle(self as *mut ::line_f::LineF, angle) }
  }

  /// C++ method: <span style='color: green;'>```void QLineF::setLength(double len)```</span>
  ///
  ///
  pub fn set_length(&mut self, len: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QLineF_setLength(self as *mut ::line_f::LineF, len) }
  }

  /// C++ method: <span style='color: green;'>```void QLineF::setLine(double x1, double y1, double x2, double y2)```</span>
  ///
  ///
  pub fn set_line(&mut self, x1: ::libc::c_double, y1: ::libc::c_double, x2: ::libc::c_double, y2: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QLineF_setLine(self as *mut ::line_f::LineF, x1, y1, x2, y2) }
  }

  /// C++ method: <span style='color: green;'>```void QLineF::setP1(const QPointF& p1)```</span>
  ///
  ///
  pub fn set_p1(&mut self, p1: &::point_f::PointF) {
    unsafe { ::ffi::qt_core_c_QLineF_setP1(self as *mut ::line_f::LineF, p1 as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QLineF::setP2(const QPointF& p2)```</span>
  ///
  ///
  pub fn set_p2(&mut self, p2: &::point_f::PointF) {
    unsafe { ::ffi::qt_core_c_QLineF_setP2(self as *mut ::line_f::LineF, p2 as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```void QLineF::setPoints(const QPointF& p1, const QPointF& p2)```</span>
  ///
  ///
  pub fn set_points(&mut self, p1: &::point_f::PointF, p2: &::point_f::PointF) {
    unsafe {
      ::ffi::qt_core_c_QLineF_setPoints(self as *mut ::line_f::LineF,
                                        p1 as *const ::point_f::PointF,
                                        p2 as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```QLine QLineF::toLine() const```</span>
  ///
  ///
  pub fn to_line(&self) -> ::line::Line {
    {
      let mut object: ::line::Line = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLineF_toLine_to_output(self as *const ::line_f::LineF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLineF::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&mut self, &::point_f::PointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLineF::translate(const QPointF& p)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&mut self, (::libc::c_double, ::libc::c_double)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QLineF::translate(double dx, double dy)```</span>
  ///
  ///
  pub fn translate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::LineFTranslateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLineF::translated```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translated(&self, &::point_f::PointF) -> ::line_f::LineF```<br>
  /// C++ method: <span style='color: green;'>```QLineF QLineF::translated(const QPointF& p) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translated(&self, (::libc::c_double, ::libc::c_double)) -> ::line_f::LineF```<br>
  /// C++ method: <span style='color: green;'>```QLineF QLineF::translated(double dx, double dy) const```</span>
  ///
  ///
  pub fn translated<'largs, Args>(&'largs self, args: Args) -> ::line_f::LineF
    where Args: overloading::LineFTranslatedArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLineF QLineF::unitVector() const```</span>
  ///
  ///
  pub fn unit_vector(&self) -> ::line_f::LineF {
    {
      let mut object: ::line_f::LineF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QLineF_unitVector_to_output(self as *const ::line_f::LineF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QLineF::x1() const```</span>
  ///
  ///
  pub fn x1(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QLineF_x1(self as *const ::line_f::LineF) }
  }

  /// C++ method: <span style='color: green;'>```double QLineF::x2() const```</span>
  ///
  ///
  pub fn x2(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QLineF_x2(self as *const ::line_f::LineF) }
  }

  /// C++ method: <span style='color: green;'>```double QLineF::y1() const```</span>
  ///
  ///
  pub fn y1(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QLineF_y1(self as *const ::line_f::LineF) }
  }

  /// C++ method: <span style='color: green;'>```double QLineF::y2() const```</span>
  ///
  ///
  pub fn y2(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QLineF_y2(self as *const ::line_f::LineF) }
  }
}

impl Drop for ::line_f::LineF {
  /// C++ method: <span style='color: green;'>```[destructor] void QLineF::~QLineF()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QLineF_destructor(self as *mut ::line_f::LineF) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [LineF::angle](../struct.LineF.html#method.angle) method.
  pub trait LineFAngleArgs<'largs> {
    fn exec(self, original_self: &'largs ::line_f::LineF) -> ::libc::c_double;
  }
  impl<'largs> LineFAngleArgs<'largs> for &'largs ::line_f::LineF {
    fn exec(self, original_self: &'largs ::line_f::LineF) -> ::libc::c_double {
      let l = self;
      unsafe {
        ::ffi::qt_core_c_QLineF_angle_l(original_self as *const ::line_f::LineF,
                                        l as *const ::line_f::LineF)
      }
    }
  }
  impl<'largs> LineFAngleArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::line_f::LineF) -> ::libc::c_double {

      unsafe { ::ffi::qt_core_c_QLineF_angle_no_args(original_self as *const ::line_f::LineF) }
    }
  }
  /// This trait represents a set of arguments accepted by [LineF::new](../struct.LineF.html#method.new) method.
  pub trait LineFNewArgs {
    fn exec(self) -> ::line_f::LineF;
  }
  impl<'a> LineFNewArgs for &'a ::line::Line {
    fn exec(self) -> ::line_f::LineF {
      let line = self;
      {
        let mut object: ::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLineF_constructor_line(line as *const ::line::Line, &mut object);
        }
        object
      }
    }
  }
  impl LineFNewArgs for () {
    fn exec(self) -> ::line_f::LineF {

      {
        let mut object: ::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLineF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> LineFNewArgs for (&'a ::point_f::PointF, &'a ::point_f::PointF) {
    fn exec(self) -> ::line_f::LineF {
      let pt1 = self.0;
      let pt2 = self.1;
      {
        let mut object: ::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLineF_constructor_pt1_pt2(pt1 as *const ::point_f::PointF,
                                                      pt2 as *const ::point_f::PointF,
                                                      &mut object);
        }
        object
      }
    }
  }
  impl LineFNewArgs for (::libc::c_double, ::libc::c_double, ::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::line_f::LineF {
      let x1 = self.0;
      let y1 = self.1;
      let x2 = self.2;
      let y2 = self.3;
      {
        let mut object: ::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLineF_constructor_x1_y1_x2_y2(x1, y1, x2, y2, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [LineF::translate](../struct.LineF.html#method.translate) method.
  pub trait LineFTranslateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::line_f::LineF) -> ();
  }
  impl<'largs> LineFTranslateArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs mut ::line_f::LineF) -> () {
      let dx = self.0;
      let dy = self.1;
      unsafe { ::ffi::qt_core_c_QLineF_translate_dx_dy(original_self as *mut ::line_f::LineF, dx, dy) }
    }
  }
  impl<'largs> LineFTranslateArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs mut ::line_f::LineF) -> () {
      let p = self;
      unsafe {
        ::ffi::qt_core_c_QLineF_translate_p(original_self as *mut ::line_f::LineF,
                                            p as *const ::point_f::PointF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [LineF::translated](../struct.LineF.html#method.translated) method.
  pub trait LineFTranslatedArgs<'largs> {
    fn exec(self, original_self: &'largs ::line_f::LineF) -> ::line_f::LineF;
  }
  impl<'largs> LineFTranslatedArgs<'largs> for (::libc::c_double, ::libc::c_double) {
    fn exec(self, original_self: &'largs ::line_f::LineF) -> ::line_f::LineF {
      let dx = self.0;
      let dy = self.1;
      {
        let mut object: ::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLineF_translated_to_output_dx_dy(original_self as *const ::line_f::LineF,
                                                             dx,
                                                             dy,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> LineFTranslatedArgs<'largs> for &'largs ::point_f::PointF {
    fn exec(self, original_self: &'largs ::line_f::LineF) -> ::line_f::LineF {
      let p = self;
      {
        let mut object: ::line_f::LineF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QLineF_translated_to_output_p(original_self as *const ::line_f::LineF,
                                                         p as *const ::point_f::PointF,
                                                         &mut object);
        }
        object
      }
    }
  }
}
