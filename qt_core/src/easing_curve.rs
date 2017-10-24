/// C++ type: <span style='color: green;'>```QEasingCurve```</span>
#[repr(C)]
pub struct EasingCurve([u8; ::type_sizes::QT_CORE_EASING_CURVE_EASING_CURVE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for EasingCurve {
  unsafe fn new_uninitialized() -> EasingCurve {
    EasingCurve(::std::mem::uninitialized())
  }
}

impl EasingCurve {
  /// C++ method: <span style='color: green;'>```void QEasingCurve::addCubicBezierSegment(const QPointF& c1, const QPointF& c2, const QPointF& endPoint)```</span>
  ///
  ///
  pub fn add_cubic_bezier_segment(&mut self,
                                  c1: &::point_f::PointF,
                                  c2: &::point_f::PointF,
                                  end_point: &::point_f::PointF) {
    unsafe {
      ::ffi::qt_core_c_QEasingCurve_addCubicBezierSegment(self as *mut ::easing_curve::EasingCurve,
                                                          c1 as *const ::point_f::PointF,
                                                          c2 as *const ::point_f::PointF,
                                                          end_point as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QEasingCurve::addTCBSegment(const QPointF& nextPoint, double t, double c, double b)```</span>
  ///
  ///
  pub fn add_t_c_b_segment(&mut self,
                           next_point: &::point_f::PointF,
                           t: ::libc::c_double,
                           c: ::libc::c_double,
                           b: ::libc::c_double) {
    unsafe {
      ::ffi::qt_core_c_QEasingCurve_addTCBSegment(self as *mut ::easing_curve::EasingCurve,
                                                  next_point as *const ::point_f::PointF,
                                                  t,
                                                  c,
                                                  b)
    }
  }

  /// C++ method: <span style='color: green;'>```double QEasingCurve::amplitude() const```</span>
  ///
  ///
  pub fn amplitude(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QEasingCurve_amplitude(self as *const ::easing_curve::EasingCurve) }
  }

  /// C++ method: <span style='color: green;'>```double (*FN_PTR)(double) QEasingCurve::customType() const```</span>
  ///
  ///
  pub fn custom_type(&self) -> extern "C" fn(::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QEasingCurve_customType(self as *const ::easing_curve::EasingCurve) }
  }

  /// C++ method: <span style='color: green;'>```QEasingCurve::QEasingCurve```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::easing_curve::EasingCurve```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QEasingCurve::QEasingCurve()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(::easing_curve::Type) -> ::easing_curve::EasingCurve```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QEasingCurve::QEasingCurve(QEasingCurve::Type type = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::easing_curve::EasingCurve) -> ::easing_curve::EasingCurve```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QEasingCurve::QEasingCurve(const QEasingCurve& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::easing_curve::EasingCurve
    where Args: overloading::EasingCurveNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QEasingCurve& QEasingCurve::operator=(const QEasingCurve& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::easing_curve::EasingCurve)
                             -> &'l0 mut ::easing_curve::EasingCurve {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QEasingCurve_operator_assign(self as *mut ::easing_curve::EasingCurve,
                                                    other as *const ::easing_curve::EasingCurve)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QEasingCurve::operator==(const QEasingCurve& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::easing_curve::EasingCurve) -> bool {
    unsafe {
      ::ffi::qt_core_c_QEasingCurve_operator_eq(self as *const ::easing_curve::EasingCurve,
                                                other as *const ::easing_curve::EasingCurve)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QEasingCurve::operator!=(const QEasingCurve& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::easing_curve::EasingCurve) -> bool {
    unsafe {
      ::ffi::qt_core_c_QEasingCurve_operator_neq(self as *const ::easing_curve::EasingCurve,
                                                 other as *const ::easing_curve::EasingCurve)
    }
  }

  /// C++ method: <span style='color: green;'>```double QEasingCurve::overshoot() const```</span>
  ///
  ///
  pub fn overshoot(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QEasingCurve_overshoot(self as *const ::easing_curve::EasingCurve) }
  }

  /// C++ method: <span style='color: green;'>```double QEasingCurve::period() const```</span>
  ///
  ///
  pub fn period(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QEasingCurve_period(self as *const ::easing_curve::EasingCurve) }
  }

  /// C++ method: <span style='color: green;'>```void QEasingCurve::setAmplitude(double amplitude)```</span>
  ///
  ///
  pub fn set_amplitude(&mut self, amplitude: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QEasingCurve_setAmplitude(self as *mut ::easing_curve::EasingCurve, amplitude) }
  }

  /// C++ method: <span style='color: green;'>```void QEasingCurve::setCustomType(double (*FN_PTR)(double) func)```</span>
  ///
  ///
  pub unsafe fn set_custom_type(&mut self, func: extern "C" fn(::libc::c_double) -> ::libc::c_double) {
    ::ffi::qt_core_c_QEasingCurve_setCustomType(self as *mut ::easing_curve::EasingCurve, func)
  }

  /// C++ method: <span style='color: green;'>```void QEasingCurve::setOvershoot(double overshoot)```</span>
  ///
  ///
  pub fn set_overshoot(&mut self, overshoot: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QEasingCurve_setOvershoot(self as *mut ::easing_curve::EasingCurve, overshoot) }
  }

  /// C++ method: <span style='color: green;'>```void QEasingCurve::setPeriod(double period)```</span>
  ///
  ///
  pub fn set_period(&mut self, period: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QEasingCurve_setPeriod(self as *mut ::easing_curve::EasingCurve, period) }
  }

  /// C++ method: <span style='color: green;'>```void QEasingCurve::setType(QEasingCurve::Type type)```</span>
  ///
  ///
  pub fn set_type(&mut self, type_: ::easing_curve::Type) {
    unsafe { ::ffi::qt_core_c_QEasingCurve_setType(self as *mut ::easing_curve::EasingCurve, type_) }
  }

  /// C++ method: <span style='color: green;'>```void QEasingCurve::swap(QEasingCurve& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::easing_curve::EasingCurve) {
    unsafe {
      ::ffi::qt_core_c_QEasingCurve_swap(self as *mut ::easing_curve::EasingCurve,
                                         other as *mut ::easing_curve::EasingCurve)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QPointF> QEasingCurve::toCubicSpline() const```</span>
  ///
  ///
  pub fn to_cubic_spline(&self) -> ::vector::VectorPointF {
    {
      let mut object: ::vector::VectorPointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QEasingCurve_toCubicSpline_to_output(self as *const ::easing_curve::EasingCurve, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QEasingCurve::Type QEasingCurve::type() const```</span>
  ///
  ///
  pub fn type_(&self) -> ::easing_curve::Type {
    unsafe { ::ffi::qt_core_c_QEasingCurve_type(self as *const ::easing_curve::EasingCurve) }
  }

  /// C++ method: <span style='color: green;'>```double QEasingCurve::valueForProgress(double progress) const```</span>
  ///
  ///
  pub fn value_for_progress(&self, progress: ::libc::c_double) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QEasingCurve_valueForProgress(self as *const ::easing_curve::EasingCurve, progress) }
  }
}

impl Drop for ::easing_curve::EasingCurve {
  /// C++ method: <span style='color: green;'>```[destructor] void QEasingCurve::~QEasingCurve()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QEasingCurve_destructor(self as *mut ::easing_curve::EasingCurve) }
  }
}

/// C++ type: <span style='color: green;'>```QEasingCurve::Type```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Type {
  /// C++ enum variant: <span style='color: green;'>```Linear = 0```</span>
  Linear = 0,
  /// C++ enum variant: <span style='color: green;'>```InQuad = 1```</span>
  InQuad = 1,
  /// C++ enum variant: <span style='color: green;'>```OutQuad = 2```</span>
  OutQuad = 2,
  /// C++ enum variant: <span style='color: green;'>```InOutQuad = 3```</span>
  InOutQuad = 3,
  /// C++ enum variant: <span style='color: green;'>```OutInQuad = 4```</span>
  OutInQuad = 4,
  /// C++ enum variant: <span style='color: green;'>```InCubic = 5```</span>
  InCubic = 5,
  /// C++ enum variant: <span style='color: green;'>```OutCubic = 6```</span>
  OutCubic = 6,
  /// C++ enum variant: <span style='color: green;'>```InOutCubic = 7```</span>
  InOutCubic = 7,
  /// C++ enum variant: <span style='color: green;'>```OutInCubic = 8```</span>
  OutInCubic = 8,
  /// C++ enum variant: <span style='color: green;'>```InQuart = 9```</span>
  InQuart = 9,
  /// C++ enum variant: <span style='color: green;'>```OutQuart = 10```</span>
  OutQuart = 10,
  /// C++ enum variant: <span style='color: green;'>```InOutQuart = 11```</span>
  InOutQuart = 11,
  /// C++ enum variant: <span style='color: green;'>```OutInQuart = 12```</span>
  OutInQuart = 12,
  /// C++ enum variant: <span style='color: green;'>```InQuint = 13```</span>
  InQuint = 13,
  /// C++ enum variant: <span style='color: green;'>```OutQuint = 14```</span>
  OutQuint = 14,
  /// C++ enum variant: <span style='color: green;'>```InOutQuint = 15```</span>
  InOutQuint = 15,
  /// C++ enum variant: <span style='color: green;'>```OutInQuint = 16```</span>
  OutInQuint = 16,
  /// C++ enum variant: <span style='color: green;'>```InSine = 17```</span>
  InSine = 17,
  /// C++ enum variant: <span style='color: green;'>```OutSine = 18```</span>
  OutSine = 18,
  /// C++ enum variant: <span style='color: green;'>```InOutSine = 19```</span>
  InOutSine = 19,
  /// C++ enum variant: <span style='color: green;'>```OutInSine = 20```</span>
  OutInSine = 20,
  /// C++ enum variant: <span style='color: green;'>```InExpo = 21```</span>
  InExpo = 21,
  /// C++ enum variant: <span style='color: green;'>```OutExpo = 22```</span>
  OutExpo = 22,
  /// C++ enum variant: <span style='color: green;'>```InOutExpo = 23```</span>
  InOutExpo = 23,
  /// C++ enum variant: <span style='color: green;'>```OutInExpo = 24```</span>
  OutInExpo = 24,
  /// C++ enum variant: <span style='color: green;'>```InCirc = 25```</span>
  InCirc = 25,
  /// C++ enum variant: <span style='color: green;'>```OutCirc = 26```</span>
  OutCirc = 26,
  /// C++ enum variant: <span style='color: green;'>```InOutCirc = 27```</span>
  InOutCirc = 27,
  /// C++ enum variant: <span style='color: green;'>```OutInCirc = 28```</span>
  OutInCirc = 28,
  /// C++ enum variant: <span style='color: green;'>```InElastic = 29```</span>
  InElastic = 29,
  /// C++ enum variant: <span style='color: green;'>```OutElastic = 30```</span>
  OutElastic = 30,
  /// C++ enum variant: <span style='color: green;'>```InOutElastic = 31```</span>
  InOutElastic = 31,
  /// C++ enum variant: <span style='color: green;'>```OutInElastic = 32```</span>
  OutInElastic = 32,
  /// C++ enum variant: <span style='color: green;'>```InBack = 33```</span>
  InBack = 33,
  /// C++ enum variant: <span style='color: green;'>```OutBack = 34```</span>
  OutBack = 34,
  /// C++ enum variant: <span style='color: green;'>```InOutBack = 35```</span>
  InOutBack = 35,
  /// C++ enum variant: <span style='color: green;'>```OutInBack = 36```</span>
  OutInBack = 36,
  /// C++ enum variant: <span style='color: green;'>```InBounce = 37```</span>
  InBounce = 37,
  /// C++ enum variant: <span style='color: green;'>```OutBounce = 38```</span>
  OutBounce = 38,
  /// C++ enum variant: <span style='color: green;'>```InOutBounce = 39```</span>
  InOutBounce = 39,
  /// C++ enum variant: <span style='color: green;'>```OutInBounce = 40```</span>
  OutInBounce = 40,
  /// C++ enum variant: <span style='color: green;'>```InCurve = 41```</span>
  InCurve = 41,
  /// C++ enum variant: <span style='color: green;'>```OutCurve = 42```</span>
  OutCurve = 42,
  /// C++ enum variant: <span style='color: green;'>```SineCurve = 43```</span>
  SineCurve = 43,
  /// C++ enum variant: <span style='color: green;'>```CosineCurve = 44```</span>
  CosineCurve = 44,
  /// C++ enum variant: <span style='color: green;'>```BezierSpline = 45```</span>
  BezierSpline = 45,
  /// C++ enum variant: <span style='color: green;'>```TCBSpline = 46```</span>
  TCBSpline = 46,
  /// C++ enum variant: <span style='color: green;'>```Custom = 47```</span>
  Custom = 47,
  /// C++ enum variant: <span style='color: green;'>```NCurveTypes = 48```</span>
  NCurveTypes = 48,
}

/// C++ method: <span style='color: green;'>```void swap(QEasingCurve& value1, QEasingCurve& value2)```</span>
///
///
pub fn swap(value1: &mut ::easing_curve::EasingCurve, value2: &mut ::easing_curve::EasingCurve) {
  unsafe {
    ::ffi::qt_core_c_QEasingCurve_G_swap(value1 as *mut ::easing_curve::EasingCurve,
                                         value2 as *mut ::easing_curve::EasingCurve)
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [EasingCurve::new](../struct.EasingCurve.html#method.new) method.
  pub trait EasingCurveNewArgs {
    fn exec(self) -> ::easing_curve::EasingCurve;
  }
  impl EasingCurveNewArgs for () {
    fn exec(self) -> ::easing_curve::EasingCurve {

      {
        let mut object: ::easing_curve::EasingCurve =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QEasingCurve_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> EasingCurveNewArgs for &'a ::easing_curve::EasingCurve {
    fn exec(self) -> ::easing_curve::EasingCurve {
      let other = self;
      {
        let mut object: ::easing_curve::EasingCurve =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QEasingCurve_constructor_other(other as *const ::easing_curve::EasingCurve, &mut object);
        }
        object
      }
    }
  }
  impl EasingCurveNewArgs for ::easing_curve::Type {
    fn exec(self) -> ::easing_curve::EasingCurve {
      let type_ = self;
      {
        let mut object: ::easing_curve::EasingCurve =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QEasingCurve_constructor_type(type_, &mut object);
        }
        object
      }
    }
  }
}
