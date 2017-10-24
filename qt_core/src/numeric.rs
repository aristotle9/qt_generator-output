/// C++ method: <span style='color: green;'>```quint64 qFloatDistance(double a, double b)```</span>
///
///
pub fn float_distance_c_double_c_double(a: ::libc::c_double, b: ::libc::c_double) -> u64 {
  unsafe { ::ffi::qt_core_c_QtNumeric_G_qFloatDistance_double_double(a, b) }
}

/// C++ method: <span style='color: green;'>```quint32 qFloatDistance(float a, float b)```</span>
///
///
pub fn float_distance_c_float_c_float(a: ::libc::c_float, b: ::libc::c_float) -> u32 {
  unsafe { ::ffi::qt_core_c_QtNumeric_G_qFloatDistance_float_float(a, b) }
}

/// C++ method: <span style='color: green;'>```double qInf()```</span>
///
///
pub fn inf() -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtNumeric_G_qInf() }
}

/// C++ method: <span style='color: green;'>```bool qIsFinite(double d)```</span>
///
///
pub fn is_finite_c_double(d: ::libc::c_double) -> bool {
  unsafe { ::ffi::qt_core_c_QtNumeric_G_qIsFinite_d(d) }
}

/// C++ method: <span style='color: green;'>```bool qIsFinite(float f)```</span>
///
///
pub fn is_finite_c_float(f: ::libc::c_float) -> bool {
  unsafe { ::ffi::qt_core_c_QtNumeric_G_qIsFinite_f(f) }
}

/// C++ method: <span style='color: green;'>```bool qIsInf(double d)```</span>
///
///
pub fn is_inf_c_double(d: ::libc::c_double) -> bool {
  unsafe { ::ffi::qt_core_c_QtNumeric_G_qIsInf_d(d) }
}

/// C++ method: <span style='color: green;'>```bool qIsInf(float f)```</span>
///
///
pub fn is_inf_c_float(f: ::libc::c_float) -> bool {
  unsafe { ::ffi::qt_core_c_QtNumeric_G_qIsInf_f(f) }
}

/// C++ method: <span style='color: green;'>```bool qIsNaN(double d)```</span>
///
///
pub fn is_nan_c_double(d: ::libc::c_double) -> bool {
  unsafe { ::ffi::qt_core_c_QtNumeric_G_qIsNaN_d(d) }
}

/// C++ method: <span style='color: green;'>```bool qIsNaN(float f)```</span>
///
///
pub fn is_nan_c_float(f: ::libc::c_float) -> bool {
  unsafe { ::ffi::qt_core_c_QtNumeric_G_qIsNaN_f(f) }
}

/// C++ method: <span style='color: green;'>```double qQNaN()```</span>
///
///
pub fn q_nan() -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtNumeric_G_qQNaN() }
}

/// C++ method: <span style='color: green;'>```double qSNaN()```</span>
///
///
pub fn s_nan() -> ::libc::c_double {
  unsafe { ::ffi::qt_core_c_QtNumeric_G_qSNaN() }
}
