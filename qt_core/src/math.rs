/// C++ method: <span style='color: green;'>```double qAcos(double v)```</span>
///
///
pub fn acos(v: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qAcos(v) }
}

/// C++ method: <span style='color: green;'>```double qAsin(double v)```</span>
///
///
pub fn asin(v: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qAsin(v) }
}

/// C++ method: <span style='color: green;'>```double qAtan(double v)```</span>
///
///
pub fn atan(v: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qAtan(v) }
}

/// C++ method: <span style='color: green;'>```double qAtan2(double y, double x)```</span>
///
///
pub fn atan2(y: ::libc::c_double, x: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qAtan2(y, x) }
}

/// C++ method: <span style='color: green;'>```int qCeil(double v)```</span>
///
///
pub fn ceil(v: ::libc::c_double) -> ::libc::c_int {
  unsafe { ::ffi::qt_core_c_QtMath_G_qCeil(v) }
}

/// C++ method: <span style='color: green;'>```double qCos(double v)```</span>
///
///
pub fn cos(v: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qCos(v) }
}

/// C++ method: <span style='color: green;'>```double qDegreesToRadians(double degrees)```</span>
///
///
pub fn degrees_to_radians_c_double(degrees: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qDegreesToRadians_double(degrees) }
}

/// C++ method: <span style='color: green;'>```float qDegreesToRadians(float degrees)```</span>
///
///
pub fn degrees_to_radians_c_float(degrees: ::libc::c_float) -> ::libc::c_float {
  unsafe { ::ffi::qt_core_c_QtMath_G_qDegreesToRadians_float(degrees) }
}

/// C++ method: <span style='color: green;'>```double qExp(double v)```</span>
///
///
pub fn exp(v: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qExp(v) }
}

/// C++ method: <span style='color: green;'>```double qFabs(double v)```</span>
///
///
pub fn fabs(v: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qFabs(v) }
}

/// C++ method: <span style='color: green;'>```double qFastCos(double x)```</span>
///
///
pub fn fast_cos(x: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qFastCos(x) }
}

/// C++ method: <span style='color: green;'>```double qFastSin(double x)```</span>
///
///
pub fn fast_sin(x: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qFastSin(x) }
}

/// C++ method: <span style='color: green;'>```int qFloor(double v)```</span>
///
///
pub fn floor(v: ::libc::c_double) -> ::libc::c_int {
  unsafe { ::ffi::qt_core_c_QtMath_G_qFloor(v) }
}

/// C++ method: <span style='color: green;'>```double qLn(double v)```</span>
///
///
pub fn ln(v: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qLn(v) }
}

/// C++ method: <span style='color: green;'>```qNextPowerOfTwo```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn next_power_of_two(i32) -> u32```<br>
/// C++ method: <span style='color: green;'>```quint32 qNextPowerOfTwo(qint32 v)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn next_power_of_two(u32) -> u32```<br>
/// C++ method: <span style='color: green;'>```quint32 qNextPowerOfTwo(quint32 v)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn next_power_of_two(i64) -> u64```<br>
/// C++ method: <span style='color: green;'>```quint64 qNextPowerOfTwo(qint64 v)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn next_power_of_two(u64) -> u64```<br>
/// C++ method: <span style='color: green;'>```quint64 qNextPowerOfTwo(quint64 v)```</span>
///
///
pub fn next_power_of_two<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::NextPowerOfTwoArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```double qPow(double x, double y)```</span>
///
///
pub fn pow(x: ::libc::c_double, y: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qPow(x, y) }
}

/// C++ method: <span style='color: green;'>```double qRadiansToDegrees(double radians)```</span>
///
///
pub fn radians_to_degrees_c_double(radians: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qRadiansToDegrees_double(radians) }
}

/// C++ method: <span style='color: green;'>```float qRadiansToDegrees(float radians)```</span>
///
///
pub fn radians_to_degrees_c_float(radians: ::libc::c_float) -> ::libc::c_float {
  unsafe { ::ffi::qt_core_c_QtMath_G_qRadiansToDegrees_float(radians) }
}

/// C++ method: <span style='color: green;'>```double qSin(double v)```</span>
///
///
pub fn sin(v: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qSin(v) }
}

/// C++ method: <span style='color: green;'>```double qSqrt(double v)```</span>
///
///
pub fn sqrt(v: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qSqrt(v) }
}

/// C++ method: <span style='color: green;'>```double qTan(double v)```</span>
///
///
pub fn tan(v: ::libc::c_double) -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtMath_G_qTan(v) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [next_power_of_two](../fn.next_power_of_two.html) method.
  pub trait NextPowerOfTwoArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl NextPowerOfTwoArgs for i32 {
    type ReturnType = u32;
    fn exec(self) -> u32 {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtMath_G_qNextPowerOfTwo_qint32(v) }
    }
  }
  impl NextPowerOfTwoArgs for i64 {
    type ReturnType = u64;
    fn exec(self) -> u64 {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtMath_G_qNextPowerOfTwo_qint64(v) }
    }
  }
  impl NextPowerOfTwoArgs for u32 {
    type ReturnType = u32;
    fn exec(self) -> u32 {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtMath_G_qNextPowerOfTwo_quint32(v) }
    }
  }
  impl NextPowerOfTwoArgs for u64 {
    type ReturnType = u64;
    fn exec(self) -> u64 {
      let v = self;
      unsafe { ::ffi::qt_core_c_QtMath_G_qNextPowerOfTwo_quint64(v) }
    }
  }
}
