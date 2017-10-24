/// C++ type: <span style='color: green;'>```QPointF```</span>
#[repr(C)]
pub struct PointF([u8; ::type_sizes::QT_CORE_POINT_F_POINT_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for PointF {
  unsafe fn new_uninitialized() -> PointF {
    PointF(::std::mem::uninitialized())
  }
}

impl PointF {
  /// C++ method: <span style='color: green;'>```static double QPointF::dotProduct(const QPointF& p1, const QPointF& p2)```</span>
  ///
  ///
  pub fn dot_product(p1: &::point_f::PointF, p2: &::point_f::PointF) -> ::libc::c_double {
    unsafe {
      ::ffi::qt_core_c_QPointF_dotProduct(p1 as *const ::point_f::PointF,
                                          p2 as *const ::point_f::PointF)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QPointF::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QPointF_isNull(self as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```double QPointF::manhattanLength() const```</span>
  ///
  ///
  pub fn manhattan_length(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QPointF_manhattanLength(self as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```QPointF::QPointF```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPointF::QPointF()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::point::Point) -> ::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPointF::QPointF(const QPoint& p)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_double, ::libc::c_double)) -> ::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPointF::QPointF(double xpos, double ypos)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::point_f::PointF
    where Args: overloading::PointFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPointF& QPointF::operator+=(const QPointF& p)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, p: &'l1 ::point_f::PointF) -> &'l0 mut ::point_f::PointF {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QPointF_operator_add_assign(self as *mut ::point_f::PointF,
                                                   p as *const ::point_f::PointF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPointF& QPointF::operator/=(double c)```</span>
  ///
  ///
  pub fn op_div_assign<'l0>(&'l0 mut self, c: ::libc::c_double) -> &'l0 mut ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPointF_operator_div_assign(self as *mut ::point_f::PointF, c) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPointF& QPointF::operator*=(double c)```</span>
  ///
  ///
  pub fn op_mul_assign<'l0>(&'l0 mut self, c: ::libc::c_double) -> &'l0 mut ::point_f::PointF {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPointF_operator_mul_assign(self as *mut ::point_f::PointF, c) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPointF& QPointF::operator-=(const QPointF& p)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self, p: &'l1 ::point_f::PointF) -> &'l0 mut ::point_f::PointF {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QPointF_operator_sub_assign(self as *mut ::point_f::PointF,
                                                   p as *const ::point_f::PointF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QPointF::rx()```</span>
  ///
  ///
  pub fn rx<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPointF_rx(self as *mut ::point_f::PointF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QPointF::ry()```</span>
  ///
  ///
  pub fn ry<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPointF_ry(self as *mut ::point_f::PointF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QPointF::setX(double x)```</span>
  ///
  ///
  pub fn set_x(&mut self, x: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QPointF_setX(self as *mut ::point_f::PointF, x) }
  }

  /// C++ method: <span style='color: green;'>```void QPointF::setY(double y)```</span>
  ///
  ///
  pub fn set_y(&mut self, y: ::libc::c_double) {
    unsafe { ::ffi::qt_core_c_QPointF_setY(self as *mut ::point_f::PointF, y) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QPointF::toPoint() const```</span>
  ///
  ///
  pub fn to_point(&self) -> ::point::Point {
    {
      let mut object: ::point::Point = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QPointF_toPoint_to_output(self as *const ::point_f::PointF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```double QPointF::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QPointF_x(self as *const ::point_f::PointF) }
  }

  /// C++ method: <span style='color: green;'>```double QPointF::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_core_c_QPointF_y(self as *const ::point_f::PointF) }
  }
}

impl Drop for ::point_f::PointF {
  /// C++ method: <span style='color: green;'>```[destructor] void QPointF::~QPointF()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QPointF_destructor(self as *mut ::point_f::PointF) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [PointF::new](../struct.PointF.html#method.new) method.
  pub trait PointFNewArgs {
    fn exec(self) -> ::point_f::PointF;
  }
  impl PointFNewArgs for () {
    fn exec(self) -> ::point_f::PointF {

      {
        let mut object: ::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPointF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> PointFNewArgs for &'a ::point::Point {
    fn exec(self) -> ::point_f::PointF {
      let p = self;
      {
        let mut object: ::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPointF_constructor_p(p as *const ::point::Point, &mut object);
        }
        object
      }
    }
  }
  impl PointFNewArgs for (::libc::c_double, ::libc::c_double) {
    fn exec(self) -> ::point_f::PointF {
      let xpos = self.0;
      let ypos = self.1;
      {
        let mut object: ::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPointF_constructor_xpos_ypos(xpos, ypos, &mut object);
        }
        object
      }
    }
  }
}
