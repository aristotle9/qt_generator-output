/// C++ type: <span style='color: green;'>```QMatrix4x4```</span>
#[repr(C)]
pub struct Matrix4X4(u8);

impl Matrix4X4 {
  /// C++ method: <span style='color: green;'>```QVariant QMatrix4x4::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMatrix4x4_convert_to_QVariant_to_output(self as *const ::matrix_4x4::Matrix4X4, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector4D QMatrix4x4::column(int index) const```</span>
  ///
  ///
  pub fn column(&self, index: ::libc::c_int) -> ::cpp_utils::CppBox<::vector_4d::Vector4D> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMatrix4x4_column_as_ptr(self as *const ::matrix_4x4::Matrix4X4, index) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```const float* QMatrix4x4::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QMatrix4x4_constData(self as *const ::matrix_4x4::Matrix4X4) }
  }

  /// C++ method: <span style='color: green;'>```void QMatrix4x4::copyDataTo(float* values) const```</span>
  ///
  ///
  pub unsafe fn copy_data_to(&self, values: *mut ::libc::c_float) {
    ::ffi::qt_gui_c_QMatrix4x4_copyDataTo(self as *const ::matrix_4x4::Matrix4X4, values)
  }

  /// C++ method: <span style='color: green;'>```const float* QMatrix4x4::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QMatrix4x4_data_const(self as *const ::matrix_4x4::Matrix4X4) }
  }

  /// C++ method: <span style='color: green;'>```float* QMatrix4x4::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QMatrix4x4_data(self as *mut ::matrix_4x4::Matrix4X4) }
  }

  /// C++ method: <span style='color: green;'>```double QMatrix4x4::determinant() const```</span>
  ///
  ///
  pub fn determinant(&self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QMatrix4x4_determinant(self as *const ::matrix_4x4::Matrix4X4) }
  }

  /// C++ method: <span style='color: green;'>```void QMatrix4x4::fill(float value)```</span>
  ///
  ///
  pub fn fill(&mut self, value: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QMatrix4x4_fill(self as *mut ::matrix_4x4::Matrix4X4, value) }
  }

  /// C++ method: <span style='color: green;'>```void QMatrix4x4::flipCoordinates()```</span>
  ///
  ///
  pub fn flip_coordinates(&mut self) {
    unsafe { ::ffi::qt_gui_c_QMatrix4x4_flipCoordinates(self as *mut ::matrix_4x4::Matrix4X4) }
  }

  /// C++ method: <span style='color: green;'>```void QMatrix4x4::frustum(float left, float right, float bottom, float top, float nearPlane, float farPlane)```</span>
  ///
  ///
  pub fn frustum(&mut self,
                 left: ::libc::c_float,
                 right: ::libc::c_float,
                 bottom: ::libc::c_float,
                 top: ::libc::c_float,
                 near_plane: ::libc::c_float,
                 far_plane: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QMatrix4x4_frustum(self as *mut ::matrix_4x4::Matrix4X4,
                                         left,
                                         right,
                                         bottom,
                                         top,
                                         near_plane,
                                         far_plane)
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4 QMatrix4x4::inverted() const```</span>
  ///
  ///
  pub fn inverted(&self) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_inverted_as_ptr_no_args(self as *const ::matrix_4x4::Matrix4X4) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4 QMatrix4x4::inverted(bool* invertible = ?) const```</span>
  ///
  ///
  pub unsafe fn inverted_unsafe(&self, invertible: *mut bool) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4> {
    let ffi_result = ::ffi::qt_gui_c_QMatrix4x4_inverted_as_ptr_invertible(self as *const ::matrix_4x4::Matrix4X4,
                                                                           invertible);
    ::cpp_utils::CppBox::new(ffi_result)
  }

  /// C++ method: <span style='color: green;'>```bool QMatrix4x4::isAffine() const```</span>
  ///
  ///
  pub fn is_affine(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QMatrix4x4_isAffine(self as *const ::matrix_4x4::Matrix4X4) }
  }

  /// C++ method: <span style='color: green;'>```bool QMatrix4x4::isIdentity() const```</span>
  ///
  ///
  pub fn is_identity(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QMatrix4x4_isIdentity(self as *const ::matrix_4x4::Matrix4X4) }
  }

  /// C++ method: <span style='color: green;'>```void QMatrix4x4::lookAt(const QVector3D& eye, const QVector3D& center, const QVector3D& up)```</span>
  ///
  ///
  pub fn look_at(&mut self, eye: &::vector_3d::Vector3D, center: &::vector_3d::Vector3D, up: &::vector_3d::Vector3D) {
    unsafe {
      ::ffi::qt_gui_c_QMatrix4x4_lookAt(self as *mut ::matrix_4x4::Matrix4X4,
                                        eye as *const ::vector_3d::Vector3D,
                                        center as *const ::vector_3d::Vector3D,
                                        up as *const ::vector_3d::Vector3D)
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4::map```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map(&self, &::qt_core::point::Point) -> ::qt_core::point::Point```<br>
  /// C++ method: <span style='color: green;'>```QPoint QMatrix4x4::map(const QPoint& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map(&self, &::qt_core::point_f::PointF) -> ::qt_core::point_f::PointF```<br>
  /// C++ method: <span style='color: green;'>```QPointF QMatrix4x4::map(const QPointF& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn map(&self, &::vector_3d::Vector3D) -> ::cpp_utils::CppBox<::vector_3d::Vector3D>```<br>
  /// C++ method: <span style='color: green;'>```QVector3D QMatrix4x4::map(const QVector3D& point) const```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn map(&self, &::vector_4d::Vector4D) -> ::cpp_utils::CppBox<::vector_4d::Vector4D>```<br>
  /// C++ method: <span style='color: green;'>```QVector4D QMatrix4x4::map(const QVector4D& point) const```</span>
  ///
  ///
  pub fn map<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::Matrix4X4MapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMatrix4x4::mapRect```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn map_rect(&self, &::qt_core::rect::Rect) -> ::qt_core::rect::Rect```<br>
  /// C++ method: <span style='color: green;'>```QRect QMatrix4x4::mapRect(const QRect& rect) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn map_rect(&self, &::qt_core::rect_f::RectF) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QMatrix4x4::mapRect(const QRectF& rect) const```</span>
  ///
  ///
  pub fn map_rect<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::Matrix4X4MapRectArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector3D QMatrix4x4::mapVector(const QVector3D& vector) const```</span>
  ///
  ///
  pub fn map_vector(&self, vector: &::vector_3d::Vector3D) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QMatrix4x4_mapVector_as_ptr(self as *const ::matrix_4x4::Matrix4X4,
                                                  vector as *const ::vector_3d::Vector3D)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4::QMatrix4x4```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMatrix4x4::QMatrix4x4()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::matrix::Matrix) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMatrix4x4::QMatrix4x4(const QMatrix& matrix)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::transform::Transform) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMatrix4x4::QMatrix4x4(const QTransform& transform)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMatrix4x4::QMatrix4x4(float m11, float m12, float m13, float m14, float m21, float m22, float m23, float m24, float m31, float m32, float m33, float m34, float m41, float m42, float m43, float m44)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4>
    where Args: overloading::Matrix4X4NewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMatrix4x4::QMatrix4x4```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new_unsafe(*const ::libc::c_float) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMatrix4x4::QMatrix4x4(const float* values)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new_unsafe((*const ::libc::c_float, ::libc::c_int, ::libc::c_int)) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4>```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMatrix4x4::QMatrix4x4(const float* values, int cols, int rows)```</span>
  ///
  ///
  pub unsafe fn new_unsafe<Args>(args: Args) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4>
    where Args: overloading::Matrix4X4NewUnsafeArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMatrix4x4& QMatrix4x4::operator+=(const QMatrix4x4& other)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 other: &'l1 ::matrix_4x4::Matrix4X4)
                                 -> &'l0 mut ::matrix_4x4::Matrix4X4 {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QMatrix4x4_operator_add_assign(self as *mut ::matrix_4x4::Matrix4X4,
                                                     other as *const ::matrix_4x4::Matrix4X4)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const float& QMatrix4x4::operator()(int row, int column) const```</span>
  ///
  ///
  pub fn op_call<'l0>(&'l0 self, row: ::libc::c_int, column: ::libc::c_int) -> &'l0 ::libc::c_float {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_operator_call_const(self as *const ::matrix_4x4::Matrix4X4, row, column) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```float& QMatrix4x4::operator()(int row, int column)```</span>
  ///
  ///
  pub fn op_call_mut<'l0>(&'l0 mut self, row: ::libc::c_int, column: ::libc::c_int) -> &'l0 mut ::libc::c_float {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_operator_call(self as *mut ::matrix_4x4::Matrix4X4, row, column) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4& QMatrix4x4::operator/=(float divisor)```</span>
  ///
  ///
  pub fn op_div_assign<'l0>(&'l0 mut self, divisor: ::libc::c_float) -> &'l0 mut ::matrix_4x4::Matrix4X4 {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_operator_div_assign(self as *mut ::matrix_4x4::Matrix4X4, divisor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMatrix4x4::operator==(const QMatrix4x4& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::matrix_4x4::Matrix4X4) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QMatrix4x4_operator_eq(self as *const ::matrix_4x4::Matrix4X4,
                                             other as *const ::matrix_4x4::Matrix4X4)
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4::operator*=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, &'l1 ::matrix_4x4::Matrix4X4) -> &'l0 mut ::matrix_4x4::Matrix4X4```<br>
  /// C++ method: <span style='color: green;'>```QMatrix4x4& QMatrix4x4::operator*=(const QMatrix4x4& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, ::libc::c_float) -> &'l0 mut ::matrix_4x4::Matrix4X4```<br>
  /// C++ method: <span style='color: green;'>```QMatrix4x4& QMatrix4x4::operator*=(float factor)```</span>
  ///
  ///
  pub fn op_mul_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::matrix_4x4::Matrix4X4
    where Args: overloading::Matrix4X4OpMulAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QMatrix4x4::operator!=(const QMatrix4x4& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::matrix_4x4::Matrix4X4) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QMatrix4x4_operator_neq(self as *const ::matrix_4x4::Matrix4X4,
                                              other as *const ::matrix_4x4::Matrix4X4)
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4& QMatrix4x4::operator-=(const QMatrix4x4& other)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self,
                                 other: &'l1 ::matrix_4x4::Matrix4X4)
                                 -> &'l0 mut ::matrix_4x4::Matrix4X4 {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QMatrix4x4_operator_sub_assign(self as *mut ::matrix_4x4::Matrix4X4,
                                                     other as *const ::matrix_4x4::Matrix4X4)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QMatrix4x4::optimize()```</span>
  ///
  ///
  pub fn optimize(&mut self) {
    unsafe { ::ffi::qt_gui_c_QMatrix4x4_optimize(self as *mut ::matrix_4x4::Matrix4X4) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4::ortho```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn ortho(&mut self, &::qt_core::rect::Rect) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::ortho(const QRect& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn ortho(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::ortho(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn ortho(&mut self, (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::ortho(float left, float right, float bottom, float top, float nearPlane, float farPlane)```</span>
  ///
  ///
  pub fn ortho<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::Matrix4X4OrthoArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::perspective(float verticalAngle, float aspectRatio, float nearPlane, float farPlane)```</span>
  ///
  ///
  pub fn perspective(&mut self,
                     vertical_angle: ::libc::c_float,
                     aspect_ratio: ::libc::c_float,
                     near_plane: ::libc::c_float,
                     far_plane: ::libc::c_float) {
    unsafe {
      ::ffi::qt_gui_c_QMatrix4x4_perspective(self as *mut ::matrix_4x4::Matrix4X4,
                                             vertical_angle,
                                             aspect_ratio,
                                             near_plane,
                                             far_plane)
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4::rotate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn rotate(&mut self, &::quaternion::Quaternion) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::rotate(const QQuaternion& quaternion)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn rotate(&mut self, (::libc::c_float, &::vector_3d::Vector3D)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::rotate(float angle, const QVector3D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn rotate(&mut self, (::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::rotate(float angle, float x, float y)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn rotate(&mut self, (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::rotate(float angle, float x, float y, float z = ?)```</span>
  ///
  ///
  pub fn rotate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::Matrix4X4RotateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector4D QMatrix4x4::row(int index) const```</span>
  ///
  ///
  pub fn row(&self, index: ::libc::c_int) -> ::cpp_utils::CppBox<::vector_4d::Vector4D> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMatrix4x4_row_as_ptr(self as *const ::matrix_4x4::Matrix4X4, index) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4::scale```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn scale(&mut self, &::vector_3d::Vector3D) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::scale(const QVector3D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn scale(&mut self, ::libc::c_float) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::scale(float factor)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn scale(&mut self, (::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::scale(float x, float y)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn scale(&mut self, (::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::scale(float x, float y, float z)```</span>
  ///
  ///
  pub fn scale<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::Matrix4X4ScaleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::setColumn(int index, const QVector4D& value)```</span>
  ///
  ///
  pub fn set_column(&mut self, index: ::libc::c_int, value: &::vector_4d::Vector4D) {
    unsafe {
      ::ffi::qt_gui_c_QMatrix4x4_setColumn(self as *mut ::matrix_4x4::Matrix4X4,
                                           index,
                                           value as *const ::vector_4d::Vector4D)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMatrix4x4::setRow(int index, const QVector4D& value)```</span>
  ///
  ///
  pub fn set_row(&mut self, index: ::libc::c_int, value: &::vector_4d::Vector4D) {
    unsafe {
      ::ffi::qt_gui_c_QMatrix4x4_setRow(self as *mut ::matrix_4x4::Matrix4X4,
                                        index,
                                        value as *const ::vector_4d::Vector4D)
    }
  }

  /// C++ method: <span style='color: green;'>```void QMatrix4x4::setToIdentity()```</span>
  ///
  ///
  pub fn set_to_identity(&mut self) {
    unsafe { ::ffi::qt_gui_c_QMatrix4x4_setToIdentity(self as *mut ::matrix_4x4::Matrix4X4) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix QMatrix4x4::toAffine() const```</span>
  ///
  ///
  pub fn to_affine(&self) -> ::matrix::Matrix {
    {
      let mut object: ::matrix::Matrix =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QMatrix4x4_toAffine_to_output(self as *const ::matrix_4x4::Matrix4X4, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4::toTransform```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn to_transform(&self, ()) -> ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```QTransform QMatrix4x4::toTransform() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn to_transform(&self, ::libc::c_float) -> ::transform::Transform```<br>
  /// C++ method: <span style='color: green;'>```QTransform QMatrix4x4::toTransform(float distanceToPlane) const```</span>
  ///
  ///
  pub fn to_transform<'largs, Args>(&'largs self, args: Args) -> ::transform::Transform
    where Args: overloading::Matrix4X4ToTransformArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMatrix4x4::translate```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn translate(&mut self, &::vector_3d::Vector3D) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::translate(const QVector3D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn translate(&mut self, (::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::translate(float x, float y)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn translate(&mut self, (::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::translate(float x, float y, float z)```</span>
  ///
  ///
  pub fn translate<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::Matrix4X4TranslateArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMatrix4x4 QMatrix4x4::transposed() const```</span>
  ///
  ///
  pub fn transposed(&self) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QMatrix4x4_transposed_as_ptr(self as *const ::matrix_4x4::Matrix4X4) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QMatrix4x4::viewport```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn viewport(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::viewport(const QRectF& rect)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn viewport(&mut self, (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::viewport(float left, float bottom, float width, float height)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn viewport(&mut self, (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::viewport(float left, float bottom, float width, float height, float nearPlane = ?)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn viewport(&mut self, (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMatrix4x4::viewport(float left, float bottom, float width, float height, float nearPlane = ?, float farPlane = ?)```</span>
  ///
  ///
  pub fn viewport<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::Matrix4X4ViewportArgs<'largs>
  {
    args.exec(self)
  }
}

impl ::cpp_utils::CppDeletable for ::matrix_4x4::Matrix4X4 {
  fn deleter() -> ::cpp_utils::Deleter<Self> {
    ::ffi::qt_gui_c_QMatrix4x4_delete
  }
}

/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QMatrix4x4& arg2)```</span>
///
///
pub fn op_shl<'l0, 'l1>(arg1: &'l0 mut ::qt_core::data_stream::DataStream,
                        arg2: &'l1 ::matrix_4x4::Matrix4X4)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QMatrix4x4_G_operator_shl(arg1 as *mut ::qt_core::data_stream::DataStream,
                                              arg2 as *const ::matrix_4x4::Matrix4X4)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QMatrix4x4& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::qt_core::data_stream::DataStream,
                        arg2: &'l1 mut ::matrix_4x4::Matrix4X4)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QMatrix4x4_G_operator_shr(arg1 as *mut ::qt_core::data_stream::DataStream,
                                              arg2 as *mut ::matrix_4x4::Matrix4X4)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Matrix4X4::map](../struct.Matrix4X4.html#method.map) method.
  pub trait Matrix4X4MapArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::matrix_4x4::Matrix4X4) -> Self::ReturnType;
  }
  impl<'largs> Matrix4X4MapArgs<'largs> for &'largs ::vector_3d::Vector3D {
    type ReturnType = ::cpp_utils::CppBox<::vector_3d::Vector3D>;
    fn exec(self, original_self: &'largs ::matrix_4x4::Matrix4X4) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
      let point = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QMatrix4x4_map_as_ptr_QVector3D(original_self as *const ::matrix_4x4::Matrix4X4,
                                                          point as *const ::vector_3d::Vector3D)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> Matrix4X4MapArgs<'largs> for &'largs ::vector_4d::Vector4D {
    type ReturnType = ::cpp_utils::CppBox<::vector_4d::Vector4D>;
    fn exec(self, original_self: &'largs ::matrix_4x4::Matrix4X4) -> ::cpp_utils::CppBox<::vector_4d::Vector4D> {
      let point = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QMatrix4x4_map_as_ptr_QVector4D(original_self as *const ::matrix_4x4::Matrix4X4,
                                                          point as *const ::vector_4d::Vector4D)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'largs> Matrix4X4MapArgs<'largs> for &'largs ::qt_core::point::Point {
    type ReturnType = ::qt_core::point::Point;
    fn exec(self, original_self: &'largs ::matrix_4x4::Matrix4X4) -> ::qt_core::point::Point {
      let point = self;
      {
        let mut object: ::qt_core::point::Point =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix4x4_map_to_output_QPoint(original_self as *const ::matrix_4x4::Matrix4X4,
                                                          point as *const ::qt_core::point::Point,
                                                          &mut object);
        }
        object
      }
    }
  }
  impl<'largs> Matrix4X4MapArgs<'largs> for &'largs ::qt_core::point_f::PointF {
    type ReturnType = ::qt_core::point_f::PointF;
    fn exec(self, original_self: &'largs ::matrix_4x4::Matrix4X4) -> ::qt_core::point_f::PointF {
      let point = self;
      {
        let mut object: ::qt_core::point_f::PointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix4x4_map_to_output_QPointF(original_self as *const ::matrix_4x4::Matrix4X4,
                                                           point as *const ::qt_core::point_f::PointF,
                                                           &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix4X4::map_rect](../struct.Matrix4X4.html#method.map_rect) method.
  pub trait Matrix4X4MapRectArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::matrix_4x4::Matrix4X4) -> Self::ReturnType;
  }
  impl<'largs> Matrix4X4MapRectArgs<'largs> for &'largs ::qt_core::rect::Rect {
    type ReturnType = ::qt_core::rect::Rect;
    fn exec(self, original_self: &'largs ::matrix_4x4::Matrix4X4) -> ::qt_core::rect::Rect {
      let rect = self;
      {
        let mut object: ::qt_core::rect::Rect =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix4x4_mapRect_to_output_QRect(original_self as *const ::matrix_4x4::Matrix4X4,
                                                             rect as *const ::qt_core::rect::Rect,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> Matrix4X4MapRectArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    type ReturnType = ::qt_core::rect_f::RectF;
    fn exec(self, original_self: &'largs ::matrix_4x4::Matrix4X4) -> ::qt_core::rect_f::RectF {
      let rect = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix4x4_mapRect_to_output_QRectF(original_self as *const ::matrix_4x4::Matrix4X4,
                                                              rect as *const ::qt_core::rect_f::RectF,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix4X4::new](../struct.Matrix4X4.html#method.new) method.
  pub trait Matrix4X4NewArgs {
    fn exec(self) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4>;
  }
  impl Matrix4X4NewArgs
    for (::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float,
                                 ::libc::c_float) {
    fn exec(self) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4> {
      let m11 = self.0;
      let m12 = self.1;
      let m13 = self.2;
      let m14 = self.3;
      let m21 = self.4;
      let m22 = self.5;
      let m23 = self.6;
      let m24 = self.7;
      let m31 = self.8;
      let m32 = self.9;
      let m33 = self.10;
      let m34 = self.11;
      let m41 = self.12;
      let m42 = self.13;
      let m43 = self.14;
      let m44 = self.15;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QMatrix4x4_new_m11_m12_m13_m14_m21_m22_m23_m24_m31_m32_m33_m34_m41_m42_m43_m44(m11,
                                                                                                         m12,
                                                                                                         m13,
                                                                                                         m14,
                                                                                                         m21,
                                                                                                         m22,
                                                                                                         m23,
                                                                                                         m24,
                                                                                                         m31,
                                                                                                         m32,
                                                                                                         m33,
                                                                                                         m34,
                                                                                                         m41,
                                                                                                         m42,
                                                                                                         m43,
                                                                                                         m44)
        };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> Matrix4X4NewArgs for &'a ::matrix::Matrix {
    fn exec(self) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4> {
      let matrix = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QMatrix4x4_new_matrix(matrix as *const ::matrix::Matrix) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl Matrix4X4NewArgs for () {
    fn exec(self) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4> {

      let ffi_result = unsafe { ::ffi::qt_gui_c_QMatrix4x4_new_no_args() };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  impl<'a> Matrix4X4NewArgs for &'a ::transform::Transform {
    fn exec(self) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4> {
      let transform = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QMatrix4x4_new_transform(transform as *const ::transform::Transform) };
      unsafe { ::cpp_utils::CppBox::new(ffi_result) }
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix4X4::new_unsafe](../struct.Matrix4X4.html#method.new_unsafe) method.
  pub trait Matrix4X4NewUnsafeArgs {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4>;
  }
  impl Matrix4X4NewUnsafeArgs for *const ::libc::c_float {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4> {
      let values = self;
      let ffi_result = ::ffi::qt_gui_c_QMatrix4x4_new_values(values);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  impl Matrix4X4NewUnsafeArgs for (*const ::libc::c_float, ::libc::c_int, ::libc::c_int) {
    unsafe fn exec(self) -> ::cpp_utils::CppBox<::matrix_4x4::Matrix4X4> {
      let values = self.0;
      let cols = self.1;
      let rows = self.2;
      let ffi_result = ::ffi::qt_gui_c_QMatrix4x4_new_values_cols_rows(values, cols, rows);
      ::cpp_utils::CppBox::new(ffi_result)
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix4X4::op_mul_assign](../struct.Matrix4X4.html#method.op_mul_assign) method.
  pub trait Matrix4X4OpMulAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> &'largs mut ::matrix_4x4::Matrix4X4;
  }
  impl<'largs> Matrix4X4OpMulAssignArgs<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> &'largs mut ::matrix_4x4::Matrix4X4 {
      let factor = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QMatrix4x4_operator_mul_assign_factor(original_self as *mut ::matrix_4x4::Matrix4X4, factor)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> Matrix4X4OpMulAssignArgs<'largs> for &'largs ::matrix_4x4::Matrix4X4 {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> &'largs mut ::matrix_4x4::Matrix4X4 {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QMatrix4x4_operator_mul_assign_other(original_self as *mut ::matrix_4x4::Matrix4X4,
                                                               other as *const ::matrix_4x4::Matrix4X4)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix4X4::ortho](../struct.Matrix4X4.html#method.ortho) method.
  pub trait Matrix4X4OrthoArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> ();
  }
  impl<'largs> Matrix4X4OrthoArgs<'largs> for &'largs ::qt_core::rect::Rect {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_gui_c_QMatrix4x4_ortho_QRect(original_self as *mut ::matrix_4x4::Matrix4X4,
                                               rect as *const ::qt_core::rect::Rect)
      }
    }
  }
  impl<'largs> Matrix4X4OrthoArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_gui_c_QMatrix4x4_ortho_QRectF(original_self as *mut ::matrix_4x4::Matrix4X4,
                                                rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> Matrix4X4OrthoArgs<'largs>
    for (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let left = self.0;
      let right = self.1;
      let bottom = self.2;
      let top = self.3;
      let near_plane = self.4;
      let far_plane = self.5;
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_ortho_float_float_float_float_float_float(original_self as *mut ::matrix_4x4::Matrix4X4, left, right, bottom, top, near_plane, far_plane) }
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix4X4::rotate](../struct.Matrix4X4.html#method.rotate) method.
  pub trait Matrix4X4RotateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> ();
  }
  impl<'largs> Matrix4X4RotateArgs<'largs> for (::libc::c_float, &'largs ::vector_3d::Vector3D) {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let angle = self.0;
      let vector = self.1;
      unsafe {
        ::ffi::qt_gui_c_QMatrix4x4_rotate_angle_vector(original_self as *mut ::matrix_4x4::Matrix4X4,
                                                       angle,
                                                       vector as *const ::vector_3d::Vector3D)
      }
    }
  }
  impl<'largs> Matrix4X4RotateArgs<'largs> for (::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let angle = self.0;
      let x = self.1;
      let y = self.2;
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_rotate_angle_x_y(original_self as *mut ::matrix_4x4::Matrix4X4, angle, x, y) }
    }
  }
  impl<'largs> Matrix4X4RotateArgs<'largs> for (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let angle = self.0;
      let x = self.1;
      let y = self.2;
      let z = self.3;
      unsafe {
        ::ffi::qt_gui_c_QMatrix4x4_rotate_angle_x_y_z(original_self as *mut ::matrix_4x4::Matrix4X4,
                                                      angle,
                                                      x,
                                                      y,
                                                      z)
      }
    }
  }
  impl<'largs> Matrix4X4RotateArgs<'largs> for &'largs ::quaternion::Quaternion {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let quaternion = self;
      unsafe {
        ::ffi::qt_gui_c_QMatrix4x4_rotate_quaternion(original_self as *mut ::matrix_4x4::Matrix4X4,
                                                     quaternion as *const ::quaternion::Quaternion)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix4X4::scale](../struct.Matrix4X4.html#method.scale) method.
  pub trait Matrix4X4ScaleArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> ();
  }
  impl<'largs> Matrix4X4ScaleArgs<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let factor = self;
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_scale_factor(original_self as *mut ::matrix_4x4::Matrix4X4, factor) }
    }
  }
  impl<'largs> Matrix4X4ScaleArgs<'largs> for &'largs ::vector_3d::Vector3D {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let vector = self;
      unsafe {
        ::ffi::qt_gui_c_QMatrix4x4_scale_vector(original_self as *mut ::matrix_4x4::Matrix4X4,
                                                vector as *const ::vector_3d::Vector3D)
      }
    }
  }
  impl<'largs> Matrix4X4ScaleArgs<'largs> for (::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_scale_x_y(original_self as *mut ::matrix_4x4::Matrix4X4, x, y) }
    }
  }
  impl<'largs> Matrix4X4ScaleArgs<'largs> for (::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let x = self.0;
      let y = self.1;
      let z = self.2;
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_scale_x_y_z(original_self as *mut ::matrix_4x4::Matrix4X4, x, y, z) }
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix4X4::to_transform](../struct.Matrix4X4.html#method.to_transform) method.
  pub trait Matrix4X4ToTransformArgs<'largs> {
    fn exec(self, original_self: &'largs ::matrix_4x4::Matrix4X4) -> ::transform::Transform;
  }
  impl<'largs> Matrix4X4ToTransformArgs<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs ::matrix_4x4::Matrix4X4) -> ::transform::Transform {
      let distance_to_plane = self;
      {
        let mut object: ::transform::Transform =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix4x4_toTransform_to_output_distanceToPlane(original_self as *const ::matrix_4x4::Matrix4X4, distance_to_plane, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> Matrix4X4ToTransformArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::matrix_4x4::Matrix4X4) -> ::transform::Transform {

      {
        let mut object: ::transform::Transform =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QMatrix4x4_toTransform_to_output_no_args(original_self as *const ::matrix_4x4::Matrix4X4,
                                                                   &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix4X4::translate](../struct.Matrix4X4.html#method.translate) method.
  pub trait Matrix4X4TranslateArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> ();
  }
  impl<'largs> Matrix4X4TranslateArgs<'largs> for &'largs ::vector_3d::Vector3D {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let vector = self;
      unsafe {
        ::ffi::qt_gui_c_QMatrix4x4_translate_vector(original_self as *mut ::matrix_4x4::Matrix4X4,
                                                    vector as *const ::vector_3d::Vector3D)
      }
    }
  }
  impl<'largs> Matrix4X4TranslateArgs<'largs> for (::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let x = self.0;
      let y = self.1;
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_translate_x_y(original_self as *mut ::matrix_4x4::Matrix4X4, x, y) }
    }
  }
  impl<'largs> Matrix4X4TranslateArgs<'largs> for (::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let x = self.0;
      let y = self.1;
      let z = self.2;
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_translate_x_y_z(original_self as *mut ::matrix_4x4::Matrix4X4, x, y, z) }
    }
  }
  /// This trait represents a set of arguments accepted by [Matrix4X4::viewport](../struct.Matrix4X4.html#method.viewport) method.
  pub trait Matrix4X4ViewportArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> ();
  }
  impl<'largs> Matrix4X4ViewportArgs<'largs> for (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let left = self.0;
      let bottom = self.1;
      let width = self.2;
      let height = self.3;
      unsafe {
        ::ffi::qt_gui_c_QMatrix4x4_viewport_left_bottom_width_height(original_self as *mut ::matrix_4x4::Matrix4X4,
                                                                     left,
                                                                     bottom,
                                                                     width,
                                                                     height)
      }
    }
  }
  impl<'largs> Matrix4X4ViewportArgs<'largs>
    for (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let left = self.0;
      let bottom = self.1;
      let width = self.2;
      let height = self.3;
      let near_plane = self.4;
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_viewport_left_bottom_width_height_nearPlane(original_self as *mut ::matrix_4x4::Matrix4X4, left, bottom, width, height, near_plane) }
    }
  }
  impl<'largs> Matrix4X4ViewportArgs<'largs>
    for (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let left = self.0;
      let bottom = self.1;
      let width = self.2;
      let height = self.3;
      let near_plane = self.4;
      let far_plane = self.5;
      unsafe { ::ffi::qt_gui_c_QMatrix4x4_viewport_left_bottom_width_height_nearPlane_farPlane(original_self as *mut ::matrix_4x4::Matrix4X4, left, bottom, width, height, near_plane, far_plane) }
    }
  }
  impl<'largs> Matrix4X4ViewportArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::matrix_4x4::Matrix4X4) -> () {
      let rect = self;
      unsafe {
        ::ffi::qt_gui_c_QMatrix4x4_viewport_rect(original_self as *mut ::matrix_4x4::Matrix4X4,
                                                 rect as *const ::qt_core::rect_f::RectF)
      }
    }
  }
}
