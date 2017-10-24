/// C++ type: <span style='color: green;'>```QPoint```</span>
#[repr(C)]
pub struct Point([u8; ::type_sizes::QT_CORE_POINT_POINT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Point {
  unsafe fn new_uninitialized() -> Point {
    Point(::std::mem::uninitialized())
  }
}

impl Point {
  /// C++ method: <span style='color: green;'>```static int QPoint::dotProduct(const QPoint& p1, const QPoint& p2)```</span>
  ///
  ///
  pub fn dot_product(p1: &::point::Point, p2: &::point::Point) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QPoint_dotProduct(p1 as *const ::point::Point, p2 as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```bool QPoint::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QPoint_isNull(self as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```int QPoint::manhattanLength() const```</span>
  ///
  ///
  pub fn manhattan_length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QPoint_manhattanLength(self as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```QPoint::QPoint```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::point::Point```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPoint::QPoint()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::point::Point```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QPoint::QPoint(int xpos, int ypos)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::point::Point
    where Args: overloading::PointNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QPoint& QPoint::operator+=(const QPoint& p)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, p: &'l1 ::point::Point) -> &'l0 mut ::point::Point {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QPoint_operator_add_assign(self as *mut ::point::Point, p as *const ::point::Point) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint& QPoint::operator/=(double divisor)```</span>
  ///
  ///
  pub fn op_div_assign<'l0>(&'l0 mut self, divisor: ::libc::c_double) -> &'l0 mut ::point::Point {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPoint_operator_div_assign(self as *mut ::point::Point, divisor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint::operator*=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_mul_assign0(&mut self, ::libc::c_double) -> &'l0 mut ::point::Point```<br>
  /// C++ method: <span style='color: green;'>```QPoint& QPoint::operator*=(double factor)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_mul_assign0(&mut self, ::libc::c_int) -> &'l0 mut ::point::Point```<br>
  /// C++ method: <span style='color: green;'>```QPoint& QPoint::operator*=(int factor)```</span>
  ///
  ///
  pub fn op_mul_assign0<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::point::Point
    where Args: overloading::PointOpMulAssign0Args<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPoint& QPoint::operator*=(float factor)```</span>
  ///
  ///
  pub fn op_mul_assign1<'l0>(&'l0 mut self, factor: ::libc::c_float) -> &'l0 mut ::point::Point {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPoint_operator_mul_assign_float(self as *mut ::point::Point, factor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPoint& QPoint::operator-=(const QPoint& p)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self, p: &'l1 ::point::Point) -> &'l0 mut ::point::Point {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QPoint_operator_sub_assign(self as *mut ::point::Point, p as *const ::point::Point) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QPoint::rx()```</span>
  ///
  ///
  pub fn rx<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPoint_rx(self as *mut ::point::Point) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QPoint::ry()```</span>
  ///
  ///
  pub fn ry<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QPoint_ry(self as *mut ::point::Point) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QPoint::setX(int x)```</span>
  ///
  ///
  pub fn set_x(&mut self, x: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QPoint_setX(self as *mut ::point::Point, x) }
  }

  /// C++ method: <span style='color: green;'>```void QPoint::setY(int y)```</span>
  ///
  ///
  pub fn set_y(&mut self, y: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QPoint_setY(self as *mut ::point::Point, y) }
  }

  /// C++ method: <span style='color: green;'>```int QPoint::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QPoint_x(self as *const ::point::Point) }
  }

  /// C++ method: <span style='color: green;'>```int QPoint::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QPoint_y(self as *const ::point::Point) }
  }
}

impl Drop for ::point::Point {
  /// C++ method: <span style='color: green;'>```[destructor] void QPoint::~QPoint()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QPoint_destructor(self as *mut ::point::Point) }
  }
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::point::Point)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QPoint& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::point_f::PointF)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QPointF& arg2)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::point::Point)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QPoint& arg2)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::point_f::PointF)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug d, const QPointF& p)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```operator>>```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::point::Point)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QPoint& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shr((&'l0 mut ::data_stream::DataStream, &'l1 mut ::point_f::PointF)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QPointF& arg2)```</span>
///
///
pub fn op_shr<'largs, Args>(args: Args) -> &'largs mut ::data_stream::DataStream
  where Args: overloading::OpShrArgs<'largs>
{
  args.exec()
}
/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Point::new](../struct.Point.html#method.new) method.
  pub trait PointNewArgs {
    fn exec(self) -> ::point::Point;
  }
  impl PointNewArgs for () {
    fn exec(self) -> ::point::Point {

      {
        let mut object: ::point::Point =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPoint_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl PointNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::point::Point {
      let xpos = self.0;
      let ypos = self.1;
      {
        let mut object: ::point::Point =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPoint_constructor_xpos_ypos(xpos, ypos, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Point::op_mul_assign0](../struct.Point.html#method.op_mul_assign0) method.
  pub trait PointOpMulAssign0Args<'largs> {
    fn exec(self, original_self: &'largs mut ::point::Point) -> &'largs mut ::point::Point;
  }
  impl<'largs> PointOpMulAssign0Args<'largs> for ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::point::Point) -> &'largs mut ::point::Point {
      let factor = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QPoint_operator_mul_assign_double(original_self as *mut ::point::Point, factor) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> PointOpMulAssign0Args<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::point::Point) -> &'largs mut ::point::Point {
      let factor = self;
      let ffi_result =
        unsafe { ::ffi::qt_core_c_QPoint_operator_mul_assign_int(original_self as *mut ::point::Point, factor) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::point::Point) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QPoint_G_operator_shl_QDataStream_QPoint(arg1 as *mut ::data_stream::DataStream,
                                                                    arg2 as *const ::point::Point)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::point_f::PointF) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QPoint_G_operator_shl_QDataStream_QPointF(arg1 as *mut ::data_stream::DataStream,
                                                                     arg2 as *const ::point_f::PointF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::point::Point) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPoint_G_operator_shl_to_output_arg1_arg2(arg1 as *const ::debug::Debug,
                                                                     arg2 as *const ::point::Point,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::point_f::PointF) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let d = self.0;
      let p = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QPoint_G_operator_shl_to_output_d_p(d as *const ::debug::Debug,
                                                               p as *const ::point_f::PointF,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shr](../fn.op_shr.html) method.
  pub trait OpShrArgs<'largs> {
    fn exec(self) -> &'largs mut ::data_stream::DataStream;
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::point::Point) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QPoint_G_operator_shr_QDataStream_QPoint(arg1 as *mut ::data_stream::DataStream,
                                                                    arg2 as *mut ::point::Point)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> OpShrArgs<'largs> for (&'largs mut ::data_stream::DataStream, &'largs mut ::point_f::PointF) {
    fn exec(self) -> &'largs mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QPoint_G_operator_shr_QDataStream_QPointF(arg1 as *mut ::data_stream::DataStream,
                                                                     arg2 as *mut ::point_f::PointF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
