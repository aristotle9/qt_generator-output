/// C++ type: <span style='color: green;'>```QVector2D```</span>
#[repr(C)]
pub struct Vector2D(u8);

impl Vector2D {
  /// C++ method: <span style='color: green;'>```QVariant QVector2D::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector2D_convert_to_QVariant_to_output(self as *const ::vector_2d::Vector2D, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float QVector2D::distanceToLine(const QVector2D& point, const QVector2D& direction) const```</span>
  ///
  ///
  pub fn distance_to_line(&self, point: &::vector_2d::Vector2D, direction: &::vector_2d::Vector2D) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_gui_c_QVector2D_distanceToLine(self as *const ::vector_2d::Vector2D,
                                               point as *const ::vector_2d::Vector2D,
                                               direction as *const ::vector_2d::Vector2D)
    }
  }

  /// C++ method: <span style='color: green;'>```float QVector2D::distanceToPoint(const QVector2D& point) const```</span>
  ///
  ///
  pub fn distance_to_point(&self, point: &::vector_2d::Vector2D) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_gui_c_QVector2D_distanceToPoint(self as *const ::vector_2d::Vector2D,
                                                point as *const ::vector_2d::Vector2D)
    }
  }

  /// C++ method: <span style='color: green;'>```static float QVector2D::dotProduct(const QVector2D& v1, const QVector2D& v2)```</span>
  ///
  ///
  pub fn dot_product(v1: &::vector_2d::Vector2D, v2: &::vector_2d::Vector2D) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_gui_c_QVector2D_dotProduct(v1 as *const ::vector_2d::Vector2D,
                                           v2 as *const ::vector_2d::Vector2D)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector2D::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector2D_isNull(self as *const ::vector_2d::Vector2D) }
  }

  /// C++ method: <span style='color: green;'>```float QVector2D::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector2D_length(self as *const ::vector_2d::Vector2D) }
  }

  /// C++ method: <span style='color: green;'>```float QVector2D::lengthSquared() const```</span>
  ///
  ///
  pub fn length_squared(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector2D_lengthSquared(self as *const ::vector_2d::Vector2D) }
  }

  /// C++ method: <span style='color: green;'>```QVector2D::QVector2D```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::vector_2d::Vector2D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector2D::QVector2D()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::point::Point) -> ::cpp_utils::CppBox<::vector_2d::Vector2D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector2D::QVector2D(const QPoint& point)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::point_f::PointF) -> ::cpp_utils::CppBox<::vector_2d::Vector2D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector2D::QVector2D(const QPointF& point)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::vector_3d::Vector3D) -> ::cpp_utils::CppBox<::vector_2d::Vector2D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector2D::QVector2D(const QVector3D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new(&::vector_4d::Vector4D) -> ::cpp_utils::CppBox<::vector_2d::Vector2D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector2D::QVector2D(const QVector4D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new((::libc::c_float, ::libc::c_float)) -> ::cpp_utils::CppBox<::vector_2d::Vector2D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector2D::QVector2D(float xpos, float ypos)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::vector_2d::Vector2D>
    where Args: overloading::Vector2DNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QVector2D::normalize()```</span>
  ///
  ///
  pub fn normalize(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector2D_normalize(self as *mut ::vector_2d::Vector2D) }
  }

  /// C++ method: <span style='color: green;'>```QVector2D QVector2D::normalized() const```</span>
  ///
  ///
  pub fn normalized(&self) -> ::cpp_utils::CppBox<::vector_2d::Vector2D> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector2D_normalized_as_ptr(self as *const ::vector_2d::Vector2D) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVector2D& QVector2D::operator+=(const QVector2D& vector)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, vector: &'l1 ::vector_2d::Vector2D) -> &'l0 mut ::vector_2d::Vector2D {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector2D_operator_add_assign(self as *mut ::vector_2d::Vector2D,
                                                    vector as *const ::vector_2d::Vector2D)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector2D::operator/=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_div_assign(&mut self, &'l1 ::vector_2d::Vector2D) -> &'l0 mut ::vector_2d::Vector2D```<br>
  /// C++ method: <span style='color: green;'>```QVector2D& QVector2D::operator/=(const QVector2D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_div_assign(&mut self, ::libc::c_float) -> &'l0 mut ::vector_2d::Vector2D```<br>
  /// C++ method: <span style='color: green;'>```QVector2D& QVector2D::operator/=(float divisor)```</span>
  ///
  ///
  pub fn op_div_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector_2d::Vector2D
    where Args: overloading::Vector2DOpDivAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```float QVector2D::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index(&self, i: ::libc::c_int) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector2D_operator_index_const(self as *const ::vector_2d::Vector2D, i) }
  }

  /// C++ method: <span style='color: green;'>```float& QVector2D::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector2D_operator_index(self as *mut ::vector_2d::Vector2D, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector2D::operator*=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, &'l1 ::vector_2d::Vector2D) -> &'l0 mut ::vector_2d::Vector2D```<br>
  /// C++ method: <span style='color: green;'>```QVector2D& QVector2D::operator*=(const QVector2D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, ::libc::c_float) -> &'l0 mut ::vector_2d::Vector2D```<br>
  /// C++ method: <span style='color: green;'>```QVector2D& QVector2D::operator*=(float factor)```</span>
  ///
  ///
  pub fn op_mul_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector_2d::Vector2D
    where Args: overloading::Vector2DOpMulAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector2D& QVector2D::operator-=(const QVector2D& vector)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self, vector: &'l1 ::vector_2d::Vector2D) -> &'l0 mut ::vector_2d::Vector2D {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector2D_operator_sub_assign(self as *mut ::vector_2d::Vector2D,
                                                    vector as *const ::vector_2d::Vector2D)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector2D::setX(float x)```</span>
  ///
  ///
  pub fn set_x(&mut self, x: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QVector2D_setX(self as *mut ::vector_2d::Vector2D, x) }
  }

  /// C++ method: <span style='color: green;'>```void QVector2D::setY(float y)```</span>
  ///
  ///
  pub fn set_y(&mut self, y: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QVector2D_setY(self as *mut ::vector_2d::Vector2D, y) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QVector2D::toPoint() const```</span>
  ///
  ///
  pub fn to_point(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector2D_toPoint_to_output(self as *const ::vector_2d::Vector2D, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QVector2D::toPointF() const```</span>
  ///
  ///
  pub fn to_point_f(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector2D_toPointF_to_output(self as *const ::vector_2d::Vector2D, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector3D QVector2D::toVector3D() const```</span>
  ///
  ///
  pub fn to_vector_3d(&self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector2D_toVector3D_as_ptr(self as *const ::vector_2d::Vector2D) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVector4D QVector2D::toVector4D() const```</span>
  ///
  ///
  pub fn to_vector_4d(&self) -> ::cpp_utils::CppBox<::vector_4d::Vector4D> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector2D_toVector4D_as_ptr(self as *const ::vector_2d::Vector2D) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```float QVector2D::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector2D_x(self as *const ::vector_2d::Vector2D) }
  }

  /// C++ method: <span style='color: green;'>```float QVector2D::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector2D_y(self as *const ::vector_2d::Vector2D) }
  }
}

impl ::cpp_utils::CppDeletable for ::vector_2d::Vector2D {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QVector2D_delete
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
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::vector_2d::Vector2D)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QVector2D& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::vector_2d::Vector2D)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QVector2D& vector)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QVector2D& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::qt_core::data_stream::DataStream,
                        arg2: &'l1 mut ::vector_2d::Vector2D)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QVector2D_G_operator_shr(arg1 as *mut ::qt_core::data_stream::DataStream,
                                             arg2 as *mut ::vector_2d::Vector2D)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Vector2D::new](../struct.Vector2D.html#method.new) method.
  pub trait Vector2DNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_2d::Vector2D>;
  }
  impl<'a> Vector2DNewArgs for &'a ::qt_core::point::Point {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_2d::Vector2D> {
      let point = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector2D_new_QPoint(point as *const ::qt_core::point::Point) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> Vector2DNewArgs for &'a ::qt_core::point_f::PointF {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_2d::Vector2D> {
      let point = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector2D_new_QPointF(point as *const ::qt_core::point_f::PointF) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> Vector2DNewArgs for &'a ::vector_3d::Vector3D {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_2d::Vector2D> {
      let vector = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector2D_new_QVector3D(vector as *const ::vector_3d::Vector3D) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> Vector2DNewArgs for &'a ::vector_4d::Vector4D {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_2d::Vector2D> {
      let vector = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector2D_new_QVector4D(vector as *const ::vector_4d::Vector4D) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl Vector2DNewArgs for (::libc::c_float, ::libc::c_float) {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_2d::Vector2D> {
      let xpos = self.0;
      let ypos = self.1;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector2D_new_float_float(xpos, ypos) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl Vector2DNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_2d::Vector2D> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector2D_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Vector2D::op_div_assign](../struct.Vector2D.html#method.op_div_assign) method.
  pub trait Vector2DOpDivAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector_2d::Vector2D) -> &'largs mut ::vector_2d::Vector2D;
  }
  impl<'largs> Vector2DOpDivAssignArgs<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::vector_2d::Vector2D) -> &'largs mut ::vector_2d::Vector2D {
      let divisor = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector2D_operator_div_assign_divisor(original_self as *mut ::vector_2d::Vector2D, divisor)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> Vector2DOpDivAssignArgs<'largs> for &'largs ::vector_2d::Vector2D {
    fn exec(self, original_self: &'largs mut ::vector_2d::Vector2D) -> &'largs mut ::vector_2d::Vector2D {
      let vector = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector2D_operator_div_assign_vector(original_self as *mut ::vector_2d::Vector2D,
                                                               vector as *const ::vector_2d::Vector2D)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Vector2D::op_mul_assign](../struct.Vector2D.html#method.op_mul_assign) method.
  pub trait Vector2DOpMulAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector_2d::Vector2D) -> &'largs mut ::vector_2d::Vector2D;
  }
  impl<'largs> Vector2DOpMulAssignArgs<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::vector_2d::Vector2D) -> &'largs mut ::vector_2d::Vector2D {
      let factor = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector2D_operator_mul_assign_factor(original_self as *mut ::vector_2d::Vector2D, factor)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> Vector2DOpMulAssignArgs<'largs> for &'largs ::vector_2d::Vector2D {
    fn exec(self, original_self: &'largs mut ::vector_2d::Vector2D) -> &'largs mut ::vector_2d::Vector2D {
      let vector = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector2D_operator_mul_assign_vector(original_self as *mut ::vector_2d::Vector2D,
                                                               vector as *const ::vector_2d::Vector2D)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::vector_2d::Vector2D) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector2D_G_operator_shl(arg1 as *mut ::qt_core::data_stream::DataStream,
                                                 arg2 as *const ::vector_2d::Vector2D)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::vector_2d::Vector2D) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let dbg = self.0;
      let vector = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector2D_G_operator_shl_to_output(dbg as *const ::qt_core::debug::Debug,
                                                             vector as *const ::vector_2d::Vector2D,
                                                             &mut object);
        }
        object
      }
    }
  }
}
