/// C++ type: <span style='color: green;'>```QVector3D```</span>
#[repr(C)]
pub struct Vector3D(u8);

impl Vector3D {
  /// C++ method: <span style='color: green;'>```QVariant QVector3D::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector3D_convert_to_QVariant_to_output(self as *const ::vector_3d::Vector3D, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QVector3D QVector3D::crossProduct(const QVector3D& v1, const QVector3D& v2)```</span>
  ///
  ///
  pub fn cross_product(v1: &::vector_3d::Vector3D,
                       v2: &::vector_3d::Vector3D)
                       -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector3D_crossProduct_as_ptr(v1 as *const ::vector_3d::Vector3D,
                                                    v2 as *const ::vector_3d::Vector3D)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```float QVector3D::distanceToLine(const QVector3D& point, const QVector3D& direction) const```</span>
  ///
  ///
  pub fn distance_to_line(&self, point: &::vector_3d::Vector3D, direction: &::vector_3d::Vector3D) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_gui_c_QVector3D_distanceToLine(self as *const ::vector_3d::Vector3D,
                                               point as *const ::vector_3d::Vector3D,
                                               direction as *const ::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector3D::distanceToPlane```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn distance_to_plane(&self, (&::vector_3d::Vector3D, &::vector_3d::Vector3D)) -> ::libc::c_float```<br>
  /// C++ method: <span style='color: green;'>```float QVector3D::distanceToPlane(const QVector3D& plane, const QVector3D& normal) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn distance_to_plane(&self, (&::vector_3d::Vector3D, &::vector_3d::Vector3D, &::vector_3d::Vector3D)) -> ::libc::c_float```<br>
  /// C++ method: <span style='color: green;'>```float QVector3D::distanceToPlane(const QVector3D& plane1, const QVector3D& plane2, const QVector3D& plane3) const```</span>
  ///
  ///
  pub fn distance_to_plane<'largs, Args>(&'largs self, args: Args) -> ::libc::c_float
    where Args: overloading::Vector3DDistanceToPlaneArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```float QVector3D::distanceToPoint(const QVector3D& point) const```</span>
  ///
  ///
  pub fn distance_to_point(&self, point: &::vector_3d::Vector3D) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_gui_c_QVector3D_distanceToPoint(self as *const ::vector_3d::Vector3D,
                                                point as *const ::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```static float QVector3D::dotProduct(const QVector3D& v1, const QVector3D& v2)```</span>
  ///
  ///
  pub fn dot_product(v1: &::vector_3d::Vector3D, v2: &::vector_3d::Vector3D) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_gui_c_QVector3D_dotProduct(v1 as *const ::vector_3d::Vector3D,
                                           v2 as *const ::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector3D::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QVector3D_isNull(self as *const ::vector_3d::Vector3D) }
  }

  /// C++ method: <span style='color: green;'>```float QVector3D::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector3D_length(self as *const ::vector_3d::Vector3D) }
  }

  /// C++ method: <span style='color: green;'>```float QVector3D::lengthSquared() const```</span>
  ///
  ///
  pub fn length_squared(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector3D_lengthSquared(self as *const ::vector_3d::Vector3D) }
  }

  /// C++ method: <span style='color: green;'>```QVector3D::QVector3D```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector3D::QVector3D()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::qt_core::point::Point) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector3D::QVector3D(const QPoint& point)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::qt_core::point_f::PointF) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector3D::QVector3D(const QPointF& point)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new(&::vector_2d::Vector2D) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector3D::QVector3D(const QVector2D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((&::vector_2d::Vector2D, ::libc::c_float)) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector3D::QVector3D(const QVector2D& vector, float zpos)```</span>
  ///
  ///
  ///
  /// ## Variant 6
  ///
  /// Rust arguments: ```fn new(&::vector_4d::Vector4D) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector3D::QVector3D(const QVector4D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 7
  ///
  /// Rust arguments: ```fn new((::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector3D::QVector3D(float xpos, float ypos, float zpos)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>
    where Args: overloading::Vector3DNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector3D::normal```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn normal((&::vector_3d::Vector3D, &::vector_3d::Vector3D)) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>```<br>
  /// C++ method: <span style='color: green;'>```static QVector3D QVector3D::normal(const QVector3D& v1, const QVector3D& v2)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn normal((&::vector_3d::Vector3D, &::vector_3d::Vector3D, &::vector_3d::Vector3D)) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>```<br>
  /// C++ method: <span style='color: green;'>```static QVector3D QVector3D::normal(const QVector3D& v1, const QVector3D& v2, const QVector3D& v3)```</span>
  ///
  ///
  pub fn normal<Args>(args: Args) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>
    where Args: overloading::Vector3DNormalArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QVector3D::normalize()```</span>
  ///
  ///
  pub fn normalize(&mut self) {
    unsafe { ::ffi::qt_gui_c_QVector3D_normalize(self as *mut ::vector_3d::Vector3D) }
  }

  /// C++ method: <span style='color: green;'>```QVector3D QVector3D::normalized() const```</span>
  ///
  ///
  pub fn normalized(&self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector3D_normalized_as_ptr(self as *const ::vector_3d::Vector3D) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVector3D& QVector3D::operator+=(const QVector3D& vector)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, vector: &'l1 ::vector_3d::Vector3D) -> &'l0 mut ::vector_3d::Vector3D {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector3D_operator_add_assign(self as *mut ::vector_3d::Vector3D,
                                                    vector as *const ::vector_3d::Vector3D)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector3D::operator/=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_div_assign(&mut self, &'l1 ::vector_3d::Vector3D) -> &'l0 mut ::vector_3d::Vector3D```<br>
  /// C++ method: <span style='color: green;'>```QVector3D& QVector3D::operator/=(const QVector3D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_div_assign(&mut self, ::libc::c_float) -> &'l0 mut ::vector_3d::Vector3D```<br>
  /// C++ method: <span style='color: green;'>```QVector3D& QVector3D::operator/=(float divisor)```</span>
  ///
  ///
  pub fn op_div_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector_3d::Vector3D
    where Args: overloading::Vector3DOpDivAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```float QVector3D::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index(&self, i: ::libc::c_int) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector3D_operator_index_const(self as *const ::vector_3d::Vector3D, i) }
  }

  /// C++ method: <span style='color: green;'>```float& QVector3D::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::libc::c_float {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector3D_operator_index(self as *mut ::vector_3d::Vector3D, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector3D::operator*=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, &'l1 ::vector_3d::Vector3D) -> &'l0 mut ::vector_3d::Vector3D```<br>
  /// C++ method: <span style='color: green;'>```QVector3D& QVector3D::operator*=(const QVector3D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, ::libc::c_float) -> &'l0 mut ::vector_3d::Vector3D```<br>
  /// C++ method: <span style='color: green;'>```QVector3D& QVector3D::operator*=(float factor)```</span>
  ///
  ///
  pub fn op_mul_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector_3d::Vector3D
    where Args: overloading::Vector3DOpMulAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector3D& QVector3D::operator-=(const QVector3D& vector)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self, vector: &'l1 ::vector_3d::Vector3D) -> &'l0 mut ::vector_3d::Vector3D {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector3D_operator_sub_assign(self as *mut ::vector_3d::Vector3D,
                                                    vector as *const ::vector_3d::Vector3D)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector3D QVector3D::project(const QMatrix4x4& modelView, const QMatrix4x4& projection, const QRect& viewport) const```</span>
  ///
  ///
  pub fn project(&self,
                 model_view: &::matrix_4x4::Matrix4X4,
                 projection: &::matrix_4x4::Matrix4X4,
                 viewport: &::qt_core::rect::Rect)
                 -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector3D_project_as_ptr(self as *const ::vector_3d::Vector3D,
                                               model_view as *const ::matrix_4x4::Matrix4X4,
                                               projection as *const ::matrix_4x4::Matrix4X4,
                                               viewport as *const ::qt_core::rect::Rect)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```void QVector3D::setX(float x)```</span>
  ///
  ///
  pub fn set_x(&mut self, x: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QVector3D_setX(self as *mut ::vector_3d::Vector3D, x) }
  }

  /// C++ method: <span style='color: green;'>```void QVector3D::setY(float y)```</span>
  ///
  ///
  pub fn set_y(&mut self, y: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QVector3D_setY(self as *mut ::vector_3d::Vector3D, y) }
  }

  /// C++ method: <span style='color: green;'>```void QVector3D::setZ(float z)```</span>
  ///
  ///
  pub fn set_z(&mut self, z: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QVector3D_setZ(self as *mut ::vector_3d::Vector3D, z) }
  }

  /// C++ method: <span style='color: green;'>```QPoint QVector3D::toPoint() const```</span>
  ///
  ///
  pub fn to_point(&self) -> ::qt_core::point::Point {
    {
      let mut object: ::qt_core::point::Point =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector3D_toPoint_to_output(self as *const ::vector_3d::Vector3D, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPointF QVector3D::toPointF() const```</span>
  ///
  ///
  pub fn to_point_f(&self) -> ::qt_core::point_f::PointF {
    {
      let mut object: ::qt_core::point_f::PointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QVector3D_toPointF_to_output(self as *const ::vector_3d::Vector3D, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector2D QVector3D::toVector2D() const```</span>
  ///
  ///
  pub fn to_vector_2d(&self) -> ::cpp_utils::CppBox<::vector_2d::Vector2D> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector3D_toVector2D_as_ptr(self as *const ::vector_3d::Vector3D) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVector4D QVector3D::toVector4D() const```</span>
  ///
  ///
  pub fn to_vector_4d(&self) -> ::cpp_utils::CppBox<::vector_4d::Vector4D> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QVector3D_toVector4D_as_ptr(self as *const ::vector_3d::Vector3D) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVector3D QVector3D::unproject(const QMatrix4x4& modelView, const QMatrix4x4& projection, const QRect& viewport) const```</span>
  ///
  ///
  pub fn unproject(&self,
                   model_view: &::matrix_4x4::Matrix4X4,
                   projection: &::matrix_4x4::Matrix4X4,
                   viewport: &::qt_core::rect::Rect)
                   -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QVector3D_unproject_as_ptr(self as *const ::vector_3d::Vector3D,
                                                 model_view as *const ::matrix_4x4::Matrix4X4,
                                                 projection as *const ::matrix_4x4::Matrix4X4,
                                                 viewport as *const ::qt_core::rect::Rect)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```float QVector3D::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector3D_x(self as *const ::vector_3d::Vector3D) }
  }

  /// C++ method: <span style='color: green;'>```float QVector3D::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector3D_y(self as *const ::vector_3d::Vector3D) }
  }

  /// C++ method: <span style='color: green;'>```float QVector3D::z() const```</span>
  ///
  ///
  pub fn z(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QVector3D_z(self as *const ::vector_3d::Vector3D) }
  }
}

impl ::cpp_utils::CppDeletable for ::vector_3d::Vector3D {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QVector3D_delete
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
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::vector_3d::Vector3D)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QVector3D& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::vector_3d::Vector3D)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QVector3D& vector)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QVector3D& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::qt_core::data_stream::DataStream,
                        arg2: &'l1 mut ::vector_3d::Vector3D)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QVector3D_G_operator_shr(arg1 as *mut ::qt_core::data_stream::DataStream,
                                             arg2 as *mut ::vector_3d::Vector3D)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Vector3D::distance_to_plane](../struct.Vector3D.html#method.distance_to_plane) method.
  pub trait Vector3DDistanceToPlaneArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector_3d::Vector3D) -> ::libc::c_float;
  }
  impl<'largs> Vector3DDistanceToPlaneArgs<'largs>
    for (&'largs ::vector_3d::Vector3D, &'largs ::vector_3d::Vector3D, &'largs ::vector_3d::Vector3D) {
    fn exec(self, original_self: &'largs ::vector_3d::Vector3D) -> ::libc::c_float {
      let plane1 = self.0;
      let plane2 = self.1;
      let plane3 = self.2;
      unsafe {
        ::ffi::qt_gui_c_QVector3D_distanceToPlane_plane1_plane2_plane3(original_self as *const ::vector_3d::Vector3D,
                                                                       plane1 as *const ::vector_3d::Vector3D,
                                                                       plane2 as *const ::vector_3d::Vector3D,
                                                                       plane3 as *const ::vector_3d::Vector3D)
      }
    }
  }
  impl<'largs> Vector3DDistanceToPlaneArgs<'largs> for (&'largs ::vector_3d::Vector3D, &'largs ::vector_3d::Vector3D) {
    fn exec(self, original_self: &'largs ::vector_3d::Vector3D) -> ::libc::c_float {
      let plane = self.0;
      let normal = self.1;
      unsafe {
        ::ffi::qt_gui_c_QVector3D_distanceToPlane_plane_normal(original_self as *const ::vector_3d::Vector3D,
                                                               plane as *const ::vector_3d::Vector3D,
                                                               normal as *const ::vector_3d::Vector3D)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Vector3D::new](../struct.Vector3D.html#method.new) method.
  pub trait Vector3DNewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>;
  }
  impl<'a> Vector3DNewArgs for &'a ::qt_core::point::Point {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
      let point = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector3D_new_QPoint(point as *const ::qt_core::point::Point) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> Vector3DNewArgs for &'a ::qt_core::point_f::PointF {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
      let point = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector3D_new_QPointF(point as *const ::qt_core::point_f::PointF) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> Vector3DNewArgs for &'a ::vector_2d::Vector2D {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
      let vector = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector3D_new_QVector2D(vector as *const ::vector_2d::Vector2D) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> Vector3DNewArgs for (&'a ::vector_2d::Vector2D, ::libc::c_float) {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
      let vector = self.0;
      let zpos = self.1;
      let ffi_result =
        unsafe { ::ffi::qt_gui_c_QVector3D_new_QVector2D_float(vector as *const ::vector_2d::Vector2D, zpos) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> Vector3DNewArgs for &'a ::vector_4d::Vector4D {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
      let vector = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector3D_new_QVector4D(vector as *const ::vector_4d::Vector4D) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl Vector3DNewArgs for (::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
      let xpos = self.0;
      let ypos = self.1;
      let zpos = self.2;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector3D_new_float_float_float(xpos, ypos, zpos) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl Vector3DNewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QVector3D_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Vector3D::normal](../struct.Vector3D.html#method.normal) method.
  pub trait Vector3DNormalArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>;
  }
  impl<'a> Vector3DNormalArgs for (&'a ::vector_3d::Vector3D, &'a ::vector_3d::Vector3D) {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
      let v1 = self.0;
      let v2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector3D_normal_as_ptr_v1_v2(v1 as *const ::vector_3d::Vector3D,
                                                      v2 as *const ::vector_3d::Vector3D)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> Vector3DNormalArgs for (&'a ::vector_3d::Vector3D, &'a ::vector_3d::Vector3D, &'a ::vector_3d::Vector3D) {
    fn exec(self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
      let v1 = self.0;
      let v2 = self.1;
      let v3 = self.2;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector3D_normal_as_ptr_v1_v2_v3(v1 as *const ::vector_3d::Vector3D,
                                                         v2 as *const ::vector_3d::Vector3D,
                                                         v3 as *const ::vector_3d::Vector3D)
      };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Vector3D::op_div_assign](../struct.Vector3D.html#method.op_div_assign) method.
  pub trait Vector3DOpDivAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector_3d::Vector3D) -> &'largs mut ::vector_3d::Vector3D;
  }
  impl<'largs> Vector3DOpDivAssignArgs<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::vector_3d::Vector3D) -> &'largs mut ::vector_3d::Vector3D {
      let divisor = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector3D_operator_div_assign_divisor(original_self as *mut ::vector_3d::Vector3D, divisor)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> Vector3DOpDivAssignArgs<'largs> for &'largs ::vector_3d::Vector3D {
    fn exec(self, original_self: &'largs mut ::vector_3d::Vector3D) -> &'largs mut ::vector_3d::Vector3D {
      let vector = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector3D_operator_div_assign_vector(original_self as *mut ::vector_3d::Vector3D,
                                                               vector as *const ::vector_3d::Vector3D)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Vector3D::op_mul_assign](../struct.Vector3D.html#method.op_mul_assign) method.
  pub trait Vector3DOpMulAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector_3d::Vector3D) -> &'largs mut ::vector_3d::Vector3D;
  }
  impl<'largs> Vector3DOpMulAssignArgs<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::vector_3d::Vector3D) -> &'largs mut ::vector_3d::Vector3D {
      let factor = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector3D_operator_mul_assign_factor(original_self as *mut ::vector_3d::Vector3D, factor)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> Vector3DOpMulAssignArgs<'largs> for &'largs ::vector_3d::Vector3D {
    fn exec(self, original_self: &'largs mut ::vector_3d::Vector3D) -> &'largs mut ::vector_3d::Vector3D {
      let vector = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QVector3D_operator_mul_assign_vector(original_self as *mut ::vector_3d::Vector3D,
                                                               vector as *const ::vector_3d::Vector3D)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::vector_3d::Vector3D) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QVector3D_G_operator_shl(arg1 as *mut ::qt_core::data_stream::DataStream,
                                                 arg2 as *const ::vector_3d::Vector3D)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::vector_3d::Vector3D) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let dbg = self.0;
      let vector = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QVector3D_G_operator_shl_to_output(dbg as *const ::qt_core::debug::Debug,
                                                             vector as *const ::vector_3d::Vector3D,
                                                             &mut object);
        }
        object
      }
    }
  }
}
