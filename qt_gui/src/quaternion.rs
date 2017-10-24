/// C++ type: <span style='color: green;'>```QQuaternion```</span>
#[repr(C)]
pub struct Quaternion([u8; ::type_sizes::QT_GUI_QUATERNION_QUATERNION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Quaternion {
  unsafe fn new_uninitialized() -> Quaternion {
    Quaternion(::std::mem::uninitialized())
  }
}

impl Quaternion {
  /// C++ method: <span style='color: green;'>```QVariant QQuaternion::operator QVariant() const```</span>
  ///
  ///
  pub fn as_q_variant(&self) -> ::qt_core::variant::Variant {
    {
      let mut object: ::qt_core::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QQuaternion_convert_to_QVariant_to_output(self as *const ::quaternion::Quaternion, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion QQuaternion::conjugate() const```</span>
  ///
  ///
  pub fn conjugate(&self) -> ::quaternion::Quaternion {
    {
      let mut object: ::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QQuaternion_conjugate_to_output(self as *const ::quaternion::Quaternion, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion QQuaternion::conjugated() const```</span>
  ///
  ///
  pub fn conjugated(&self) -> ::quaternion::Quaternion {
    {
      let mut object: ::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QQuaternion_conjugated_to_output(self as *const ::quaternion::Quaternion, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static float QQuaternion::dotProduct(const QQuaternion& q1, const QQuaternion& q2)```</span>
  ///
  ///
  pub fn dot_product(q1: &::quaternion::Quaternion, q2: &::quaternion::Quaternion) -> ::libc::c_float {
    unsafe {
      ::ffi::qt_gui_c_QQuaternion_dotProduct(q1 as *const ::quaternion::Quaternion,
                                             q2 as *const ::quaternion::Quaternion)
    }
  }

  /// C++ method: <span style='color: green;'>```static QQuaternion QQuaternion::fromAxes(const QVector3D& xAxis, const QVector3D& yAxis, const QVector3D& zAxis)```</span>
  ///
  ///
  pub fn from_axes(x_axis: &::vector_3d::Vector3D,
                   y_axis: &::vector_3d::Vector3D,
                   z_axis: &::vector_3d::Vector3D)
                   -> ::quaternion::Quaternion {
    {
      let mut object: ::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QQuaternion_fromAxes_to_output(x_axis as *const ::vector_3d::Vector3D,
                                                       y_axis as *const ::vector_3d::Vector3D,
                                                       z_axis as *const ::vector_3d::Vector3D,
                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion::fromAxisAndAngle```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_axis_and_angle((&::vector_3d::Vector3D, ::libc::c_float)) -> ::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```static QQuaternion QQuaternion::fromAxisAndAngle(const QVector3D& axis, float angle)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_axis_and_angle((::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```static QQuaternion QQuaternion::fromAxisAndAngle(float x, float y, float z, float angle)```</span>
  ///
  ///
  pub fn from_axis_and_angle<Args>(args: Args) -> ::quaternion::Quaternion
    where Args: overloading::QuaternionFromAxisAndAngleArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QQuaternion QQuaternion::fromDirection(const QVector3D& direction, const QVector3D& up)```</span>
  ///
  ///
  pub fn from_direction(direction: &::vector_3d::Vector3D, up: &::vector_3d::Vector3D) -> ::quaternion::Quaternion {
    {
      let mut object: ::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QQuaternion_fromDirection_to_output(direction as *const ::vector_3d::Vector3D,
                                                            up as *const ::vector_3d::Vector3D,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion::fromEulerAngles```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn from_euler_angles(&::vector_3d::Vector3D) -> ::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```static QQuaternion QQuaternion::fromEulerAngles(const QVector3D& eulerAngles)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn from_euler_angles((::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```static QQuaternion QQuaternion::fromEulerAngles(float pitch, float yaw, float roll)```</span>
  ///
  ///
  pub fn from_euler_angles<Args>(args: Args) -> ::quaternion::Quaternion
    where Args: overloading::QuaternionFromEulerAnglesArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QQuaternion::getAxes(QVector3D* xAxis, QVector3D* yAxis, QVector3D* zAxis) const```</span>
  ///
  ///
  pub unsafe fn get_axes(&self,
                         x_axis: *mut ::vector_3d::Vector3D,
                         y_axis: *mut ::vector_3d::Vector3D,
                         z_axis: *mut ::vector_3d::Vector3D) {
    ::ffi::qt_gui_c_QQuaternion_getAxes(self as *const ::quaternion::Quaternion,
                                        x_axis,
                                        y_axis,
                                        z_axis)
  }

  /// C++ method: <span style='color: green;'>```QQuaternion::getAxisAndAngle```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn get_axis_and_angle(&self, (*mut ::vector_3d::Vector3D, *mut ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QQuaternion::getAxisAndAngle(QVector3D* axis, float* angle) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn get_axis_and_angle(&self, (*mut ::libc::c_float, *mut ::libc::c_float, *mut ::libc::c_float, *mut ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QQuaternion::getAxisAndAngle(float* x, float* y, float* z, float* angle) const```</span>
  ///
  ///
  pub unsafe fn get_axis_and_angle<'largs, Args>(&'largs self, args: Args) -> ()
    where Args: overloading::QuaternionGetAxisAndAngleArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QQuaternion::getEulerAngles(float* pitch, float* yaw, float* roll) const```</span>
  ///
  ///
  pub unsafe fn get_euler_angles(&self,
                                 pitch: *mut ::libc::c_float,
                                 yaw: *mut ::libc::c_float,
                                 roll: *mut ::libc::c_float) {
    ::ffi::qt_gui_c_QQuaternion_getEulerAngles(self as *const ::quaternion::Quaternion, pitch, yaw, roll)
  }

  /// C++ method: <span style='color: green;'>```QQuaternion QQuaternion::inverted() const```</span>
  ///
  ///
  pub fn inverted(&self) -> ::quaternion::Quaternion {
    {
      let mut object: ::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QQuaternion_inverted_to_output(self as *const ::quaternion::Quaternion, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QQuaternion::isIdentity() const```</span>
  ///
  ///
  pub fn is_identity(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QQuaternion_isIdentity(self as *const ::quaternion::Quaternion) }
  }

  /// C++ method: <span style='color: green;'>```bool QQuaternion::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QQuaternion_isNull(self as *const ::quaternion::Quaternion) }
  }

  /// C++ method: <span style='color: green;'>```float QQuaternion::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QQuaternion_length(self as *const ::quaternion::Quaternion) }
  }

  /// C++ method: <span style='color: green;'>```float QQuaternion::lengthSquared() const```</span>
  ///
  ///
  pub fn length_squared(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QQuaternion_lengthSquared(self as *const ::quaternion::Quaternion) }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion::QQuaternion```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QQuaternion::QQuaternion()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector_4d::Vector4D) -> ::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QQuaternion::QQuaternion(const QVector4D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new((::libc::c_float, &::vector_3d::Vector3D)) -> ::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QQuaternion::QQuaternion(float scalar, const QVector3D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QQuaternion::QQuaternion(float scalar, float xpos, float ypos, float zpos)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::quaternion::Quaternion
    where Args: overloading::QuaternionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```static QQuaternion QQuaternion::nlerp(const QQuaternion& q1, const QQuaternion& q2, float t)```</span>
  ///
  ///
  pub fn nlerp(q1: &::quaternion::Quaternion,
               q2: &::quaternion::Quaternion,
               t: ::libc::c_float)
               -> ::quaternion::Quaternion {
    {
      let mut object: ::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QQuaternion_nlerp_to_output(q1 as *const ::quaternion::Quaternion,
                                                    q2 as *const ::quaternion::Quaternion,
                                                    t,
                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QQuaternion::normalize()```</span>
  ///
  ///
  pub fn normalize(&mut self) {
    unsafe { ::ffi::qt_gui_c_QQuaternion_normalize(self as *mut ::quaternion::Quaternion) }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion QQuaternion::normalized() const```</span>
  ///
  ///
  pub fn normalized(&self) -> ::quaternion::Quaternion {
    {
      let mut object: ::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QQuaternion_normalized_to_output(self as *const ::quaternion::Quaternion, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion& QQuaternion::operator+=(const QQuaternion& quaternion)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 quaternion: &'l1 ::quaternion::Quaternion)
                                 -> &'l0 mut ::quaternion::Quaternion {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QQuaternion_operator_add_assign(self as *mut ::quaternion::Quaternion,
                                                      quaternion as *const ::quaternion::Quaternion)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QQuaternion& QQuaternion::operator/=(float divisor)```</span>
  ///
  ///
  pub fn op_div_assign<'l0>(&'l0 mut self, divisor: ::libc::c_float) -> &'l0 mut ::quaternion::Quaternion {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QQuaternion_operator_div_assign(self as *mut ::quaternion::Quaternion, divisor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QQuaternion::operator*=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, &'l1 ::quaternion::Quaternion) -> &'l0 mut ::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```QQuaternion& QQuaternion::operator*=(const QQuaternion& quaternion)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_mul_assign(&mut self, ::libc::c_float) -> &'l0 mut ::quaternion::Quaternion```<br>
  /// C++ method: <span style='color: green;'>```QQuaternion& QQuaternion::operator*=(float factor)```</span>
  ///
  ///
  pub fn op_mul_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::quaternion::Quaternion
    where Args: overloading::QuaternionOpMulAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QQuaternion& QQuaternion::operator-=(const QQuaternion& quaternion)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self,
                                 quaternion: &'l1 ::quaternion::Quaternion)
                                 -> &'l0 mut ::quaternion::Quaternion {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QQuaternion_operator_sub_assign(self as *mut ::quaternion::Quaternion,
                                                      quaternion as *const ::quaternion::Quaternion)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector3D QQuaternion::rotatedVector(const QVector3D& vector) const```</span>
  ///
  ///
  pub fn rotated_vector(&self, vector: &::vector_3d::Vector3D) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QQuaternion_rotatedVector_as_ptr(self as *const ::quaternion::Quaternion,
                                                       vector as *const ::vector_3d::Vector3D)
    };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```static QQuaternion QQuaternion::rotationTo(const QVector3D& from, const QVector3D& to)```</span>
  ///
  ///
  pub fn rotation_to(from: &::vector_3d::Vector3D, to: &::vector_3d::Vector3D) -> ::quaternion::Quaternion {
    {
      let mut object: ::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QQuaternion_rotationTo_to_output(from as *const ::vector_3d::Vector3D,
                                                         to as *const ::vector_3d::Vector3D,
                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```float QQuaternion::scalar() const```</span>
  ///
  ///
  pub fn scalar(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QQuaternion_scalar(self as *const ::quaternion::Quaternion) }
  }

  /// C++ method: <span style='color: green;'>```void QQuaternion::setScalar(float scalar)```</span>
  ///
  ///
  pub fn set_scalar(&mut self, scalar: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QQuaternion_setScalar(self as *mut ::quaternion::Quaternion, scalar) }
  }

  /// C++ method: <span style='color: green;'>```QQuaternion::setVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn set_vector(&mut self, &::vector_3d::Vector3D) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QQuaternion::setVector(const QVector3D& vector)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn set_vector(&mut self, (::libc::c_float, ::libc::c_float, ::libc::c_float)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QQuaternion::setVector(float x, float y, float z)```</span>
  ///
  ///
  pub fn set_vector<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::QuaternionSetVectorArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QQuaternion::setX(float x)```</span>
  ///
  ///
  pub fn set_x(&mut self, x: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QQuaternion_setX(self as *mut ::quaternion::Quaternion, x) }
  }

  /// C++ method: <span style='color: green;'>```void QQuaternion::setY(float y)```</span>
  ///
  ///
  pub fn set_y(&mut self, y: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QQuaternion_setY(self as *mut ::quaternion::Quaternion, y) }
  }

  /// C++ method: <span style='color: green;'>```void QQuaternion::setZ(float z)```</span>
  ///
  ///
  pub fn set_z(&mut self, z: ::libc::c_float) {
    unsafe { ::ffi::qt_gui_c_QQuaternion_setZ(self as *mut ::quaternion::Quaternion, z) }
  }

  /// C++ method: <span style='color: green;'>```static QQuaternion QQuaternion::slerp(const QQuaternion& q1, const QQuaternion& q2, float t)```</span>
  ///
  ///
  pub fn slerp(q1: &::quaternion::Quaternion,
               q2: &::quaternion::Quaternion,
               t: ::libc::c_float)
               -> ::quaternion::Quaternion {
    {
      let mut object: ::quaternion::Quaternion =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QQuaternion_slerp_to_output(q1 as *const ::quaternion::Quaternion,
                                                    q2 as *const ::quaternion::Quaternion,
                                                    t,
                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector3D QQuaternion::toEulerAngles() const```</span>
  ///
  ///
  pub fn to_euler_angles(&self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QQuaternion_toEulerAngles_as_ptr(self as *const ::quaternion::Quaternion) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVector4D QQuaternion::toVector4D() const```</span>
  ///
  ///
  pub fn to_vector_4d(&self) -> ::cpp_utils::CppBox<::vector_4d::Vector4D> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QQuaternion_toVector4D_as_ptr(self as *const ::quaternion::Quaternion) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QVector3D QQuaternion::vector() const```</span>
  ///
  ///
  pub fn vector(&self) -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QQuaternion_vector_as_ptr(self as *const ::quaternion::Quaternion) };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```float QQuaternion::x() const```</span>
  ///
  ///
  pub fn x(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QQuaternion_x(self as *const ::quaternion::Quaternion) }
  }

  /// C++ method: <span style='color: green;'>```float QQuaternion::y() const```</span>
  ///
  ///
  pub fn y(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QQuaternion_y(self as *const ::quaternion::Quaternion) }
  }

  /// C++ method: <span style='color: green;'>```float QQuaternion::z() const```</span>
  ///
  ///
  pub fn z(&self) -> ::libc::c_float {
    unsafe { ::ffi::qt_gui_c_QQuaternion_z(self as *const ::quaternion::Quaternion) }
  }
}

impl Drop for ::quaternion::Quaternion {
  /// C++ method: <span style='color: green;'>```[destructor] void QQuaternion::~QQuaternion()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QQuaternion_destructor(self as *mut ::quaternion::Quaternion) }
  }
}

/// C++ method: <span style='color: green;'>```QVector3D operator*(const QQuaternion& quaternion, const QVector3D& vec)```</span>
///
///
pub fn op_mul(quaternion: &::quaternion::Quaternion,
              vec: &::vector_3d::Vector3D)
              -> ::cpp_utils::CppBox<::vector_3d::Vector3D> {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QQuaternion_G_operator_mul_as_ptr(quaternion as *const ::quaternion::Quaternion,
                                                      vec as *const ::vector_3d::Vector3D)
  };
  unsafe { ::cpp_utils::CppBox::new(ffi_result) }
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::qt_core::data_stream::DataStream, &'l1 ::quaternion::Quaternion)) -> &'l0 mut ::qt_core::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QQuaternion& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::qt_core::debug::Debug, &::quaternion::Quaternion)) -> ::qt_core::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug dbg, const QQuaternion& q)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QQuaternion& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::qt_core::data_stream::DataStream,
                        arg2: &'l1 mut ::quaternion::Quaternion)
                        -> &'l0 mut ::qt_core::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_gui_c_QQuaternion_G_operator_shr(arg1 as *mut ::qt_core::data_stream::DataStream,
                                               arg2 as *mut ::quaternion::Quaternion)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Quaternion::from_axis_and_angle](../struct.Quaternion.html#method.from_axis_and_angle) method.
  pub trait QuaternionFromAxisAndAngleArgs {
    fn exec(self) -> ::quaternion::Quaternion;
  }
  impl<'a> QuaternionFromAxisAndAngleArgs for (&'a ::vector_3d::Vector3D, ::libc::c_float) {
    fn exec(self) -> ::quaternion::Quaternion {
      let axis = self.0;
      let angle = self.1;
      {
        let mut object: ::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QQuaternion_fromAxisAndAngle_to_output_axis_angle(axis as *const ::vector_3d::Vector3D,
                                                                            angle,
                                                                            &mut object);
        }
        object
      }
    }
  }
  impl QuaternionFromAxisAndAngleArgs for (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self) -> ::quaternion::Quaternion {
      let x = self.0;
      let y = self.1;
      let z = self.2;
      let angle = self.3;
      {
        let mut object: ::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QQuaternion_fromAxisAndAngle_to_output_x_y_z_angle(x, y, z, angle, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Quaternion::from_euler_angles](../struct.Quaternion.html#method.from_euler_angles) method.
  pub trait QuaternionFromEulerAnglesArgs {
    fn exec(self) -> ::quaternion::Quaternion;
  }
  impl<'a> QuaternionFromEulerAnglesArgs for &'a ::vector_3d::Vector3D {
    fn exec(self) -> ::quaternion::Quaternion {
      let euler_angles = self;
      {
        let mut object: ::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QQuaternion_fromEulerAngles_to_output_eulerAngles(euler_angles as *const ::vector_3d::Vector3D, &mut object);
        }
        object
      }
    }
  }
  impl QuaternionFromEulerAnglesArgs for (::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self) -> ::quaternion::Quaternion {
      let pitch = self.0;
      let yaw = self.1;
      let roll = self.2;
      {
        let mut object: ::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QQuaternion_fromEulerAngles_to_output_pitch_yaw_roll(pitch, yaw, roll, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Quaternion::get_axis_and_angle](../struct.Quaternion.html#method.get_axis_and_angle) method.
  pub trait QuaternionGetAxisAndAngleArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::quaternion::Quaternion) -> ();
  }
  impl<'largs> QuaternionGetAxisAndAngleArgs<'largs> for (*mut ::vector_3d::Vector3D, *mut ::libc::c_float) {
    unsafe fn exec(self, original_self: &'largs ::quaternion::Quaternion) -> () {
      let axis = self.0;
      let angle = self.1;
      ::ffi::qt_gui_c_QQuaternion_getAxisAndAngle_axis_angle(original_self as *const ::quaternion::Quaternion,
                                                             axis,
                                                             angle)
    }
  }
  impl<'largs> QuaternionGetAxisAndAngleArgs<'largs>
    for (*mut ::libc::c_float, *mut ::libc::c_float, *mut ::libc::c_float, *mut ::libc::c_float) {
    unsafe fn exec(self, original_self: &'largs ::quaternion::Quaternion) -> () {
      let x = self.0;
      let y = self.1;
      let z = self.2;
      let angle = self.3;
      ::ffi::qt_gui_c_QQuaternion_getAxisAndAngle_x_y_z_angle(original_self as *const ::quaternion::Quaternion,
                                                              x,
                                                              y,
                                                              z,
                                                              angle)
    }
  }
  /// This trait represents a set of arguments accepted by [Quaternion::new](../struct.Quaternion.html#method.new) method.
  pub trait QuaternionNewArgs {
    fn exec(self) -> ::quaternion::Quaternion;
  }
  impl QuaternionNewArgs for () {
    fn exec(self) -> ::quaternion::Quaternion {

      {
        let mut object: ::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QQuaternion_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> QuaternionNewArgs for (::libc::c_float, &'a ::vector_3d::Vector3D) {
    fn exec(self) -> ::quaternion::Quaternion {
      let scalar = self.0;
      let vector = self.1;
      {
        let mut object: ::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QQuaternion_constructor_scalar_vector(scalar,
                                                                vector as *const ::vector_3d::Vector3D,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl QuaternionNewArgs for (::libc::c_float, ::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self) -> ::quaternion::Quaternion {
      let scalar = self.0;
      let xpos = self.1;
      let ypos = self.2;
      let zpos = self.3;
      {
        let mut object: ::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QQuaternion_constructor_scalar_xpos_ypos_zpos(scalar, xpos, ypos, zpos, &mut object);
        }
        object
      }
    }
  }
  impl<'a> QuaternionNewArgs for &'a ::vector_4d::Vector4D {
    fn exec(self) -> ::quaternion::Quaternion {
      let vector = self;
      {
        let mut object: ::quaternion::Quaternion =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QQuaternion_constructor_vector(vector as *const ::vector_4d::Vector4D, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Quaternion::op_mul_assign](../struct.Quaternion.html#method.op_mul_assign) method.
  pub trait QuaternionOpMulAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::quaternion::Quaternion) -> &'largs mut ::quaternion::Quaternion;
  }
  impl<'largs> QuaternionOpMulAssignArgs<'largs> for ::libc::c_float {
    fn exec(self, original_self: &'largs mut ::quaternion::Quaternion) -> &'largs mut ::quaternion::Quaternion {
      let factor = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QQuaternion_operator_mul_assign_factor(original_self as *mut ::quaternion::Quaternion, factor)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> QuaternionOpMulAssignArgs<'largs> for &'largs ::quaternion::Quaternion {
    fn exec(self, original_self: &'largs mut ::quaternion::Quaternion) -> &'largs mut ::quaternion::Quaternion {
      let quaternion = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QQuaternion_operator_mul_assign_quaternion(original_self as *mut ::quaternion::Quaternion,
                                                                     quaternion as *const ::quaternion::Quaternion)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [Quaternion::set_vector](../struct.Quaternion.html#method.set_vector) method.
  pub trait QuaternionSetVectorArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::quaternion::Quaternion) -> ();
  }
  impl<'largs> QuaternionSetVectorArgs<'largs> for &'largs ::vector_3d::Vector3D {
    fn exec(self, original_self: &'largs mut ::quaternion::Quaternion) -> () {
      let vector = self;
      unsafe {
        ::ffi::qt_gui_c_QQuaternion_setVector_vector(original_self as *mut ::quaternion::Quaternion,
                                                     vector as *const ::vector_3d::Vector3D)
      }
    }
  }
  impl<'largs> QuaternionSetVectorArgs<'largs> for (::libc::c_float, ::libc::c_float, ::libc::c_float) {
    fn exec(self, original_self: &'largs mut ::quaternion::Quaternion) -> () {
      let x = self.0;
      let y = self.1;
      let z = self.2;
      unsafe { ::ffi::qt_gui_c_QQuaternion_setVector_x_y_z(original_self as *mut ::quaternion::Quaternion, x, y, z) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::qt_core::data_stream::DataStream, &'a ::quaternion::Quaternion) {
    type ReturnType = &'a mut ::qt_core::data_stream::DataStream;
    fn exec(self) -> &'a mut ::qt_core::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QQuaternion_G_operator_shl(arg1 as *mut ::qt_core::data_stream::DataStream,
                                                   arg2 as *const ::quaternion::Quaternion)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::qt_core::debug::Debug, &'a ::quaternion::Quaternion) {
    type ReturnType = ::qt_core::debug::Debug;
    fn exec(self) -> ::qt_core::debug::Debug {
      let dbg = self.0;
      let q = self.1;
      {
        let mut object: ::qt_core::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QQuaternion_G_operator_shl_to_output(dbg as *const ::qt_core::debug::Debug,
                                                               q as *const ::quaternion::Quaternion,
                                                               &mut object);
        }
        object
      }
    }
  }
}
