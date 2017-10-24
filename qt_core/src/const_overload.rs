/// C++ type: <span style='color: green;'>```QMacAutoReleasePool```</span>
#[repr(C)]
pub struct MacAutoReleasePool([u8; ::type_sizes::QT_CORE_CONST_OVERLOAD_MAC_AUTO_RELEASE_POOL]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MacAutoReleasePool {
  unsafe fn new_uninitialized() -> MacAutoReleasePool {
    MacAutoReleasePool(::std::mem::uninitialized())
  }
}

impl MacAutoReleasePool {
  /// C++ method: <span style='color: green;'>```[constructor] void QMacAutoReleasePool::QMacAutoReleasePool()```</span>
  ///
  ///
  pub fn new() -> ::const_overload::MacAutoReleasePool {
    {
      let mut object: ::const_overload::MacAutoReleasePool =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMacAutoReleasePool_constructor(&mut object);
      }
      object
    }
  }
}

impl Drop for ::const_overload::MacAutoReleasePool {
  /// C++ method: <span style='color: green;'>```[destructor] void QMacAutoReleasePool::~QMacAutoReleasePool()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMacAutoReleasePool_destructor(self as *mut ::const_overload::MacAutoReleasePool) }
  }
}

/// C++ method: <span style='color: green;'>```qEnvironmentVariableIntValue```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn environment_variable_int_value(*const ::libc::c_char) -> ::libc::c_int```<br>
/// C++ method: <span style='color: green;'>```int qEnvironmentVariableIntValue(const char* varName)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn environment_variable_int_value((*const ::libc::c_char, *mut bool)) -> ::libc::c_int```<br>
/// C++ method: <span style='color: green;'>```int qEnvironmentVariableIntValue(const char* varName, bool* ok = ?)```</span>
///
///
pub unsafe fn environment_variable_int_value<Args>(args: Args) -> ::libc::c_int
  where Args: overloading::EnvironmentVariableIntValueArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool qEnvironmentVariableIsEmpty(const char* varName)```</span>
///
///
pub unsafe fn environment_variable_is_empty(var_name: *const ::libc::c_char) -> bool {
  ::ffi::qt_core_c_QConstOverload_G_qEnvironmentVariableIsEmpty(var_name)
}

/// C++ method: <span style='color: green;'>```bool qEnvironmentVariableIsSet(const char* varName)```</span>
///
///
pub unsafe fn environment_variable_is_set(var_name: *const ::libc::c_char) -> bool {
  ::ffi::qt_core_c_QConstOverload_G_qEnvironmentVariableIsSet(var_name)
}

/// C++ method: <span style='color: green;'>```void qFreeAligned(void* ptr)```</span>
///
///
pub unsafe fn free_aligned(ptr: *mut ::libc::c_void) {
  ::ffi::qt_core_c_QConstOverload_G_qFreeAligned(ptr)
}

/// C++ method: <span style='color: green;'>```bool qFuzzyCompare(double p1, double p2)```</span>
///
///
pub fn fuzzy_compare_c_double_c_double(p1: ::libc::c_double, p2: ::libc::c_double) -> bool {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qFuzzyCompare_double_double(p1, p2) }
}

/// C++ method: <span style='color: green;'>```bool qFuzzyCompare(float p1, float p2)```</span>
///
///
pub fn fuzzy_compare_c_float_c_float(p1: ::libc::c_float, p2: ::libc::c_float) -> bool {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qFuzzyCompare_float_float(p1, p2) }
}

/// C++ method: <span style='color: green;'>```bool qFuzzyIsNull(double d)```</span>
///
///
pub fn fuzzy_is_null_c_double(d: ::libc::c_double) -> bool {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qFuzzyIsNull_d(d) }
}

/// C++ method: <span style='color: green;'>```bool qFuzzyIsNull(float f)```</span>
///
///
pub fn fuzzy_is_null_c_float(f: ::libc::c_float) -> bool {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qFuzzyIsNull_f(f) }
}

/// C++ method: <span style='color: green;'>```int qIntCast(double f)```</span>
///
///
pub fn int_cast_c_double(f: ::libc::c_double) -> ::libc::c_int {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qIntCast_double(f) }
}

/// C++ method: <span style='color: green;'>```int qIntCast(float f)```</span>
///
///
pub fn int_cast_c_float(f: ::libc::c_float) -> ::libc::c_int {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qIntCast_float(f) }
}

/// C++ method: <span style='color: green;'>```bool qIsNull(double d)```</span>
///
///
pub fn is_null_c_double(d: ::libc::c_double) -> bool {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qIsNull_d(d) }
}

/// C++ method: <span style='color: green;'>```bool qIsNull(float f)```</span>
///
///
pub fn is_null_c_float(f: ::libc::c_float) -> bool {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qIsNull_f(f) }
}

/// C++ method: <span style='color: green;'>```void* qMallocAligned(unsigned long size, unsigned long alignment)```</span>
///
///
pub fn malloc_aligned(size: ::libc::c_ulong, alignment: ::libc::c_ulong) -> *mut ::libc::c_void {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qMallocAligned(size, alignment) }
}

/// C++ method: <span style='color: green;'>```QByteArray qgetenv(const char* varName)```</span>
///
///
pub unsafe fn qgetenv(var_name: *const ::libc::c_char) -> ::byte_array::ByteArray {
  {
    let mut object: ::byte_array::ByteArray = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
    ::ffi::qt_core_c_QConstOverload_G_qgetenv_to_output(var_name, &mut object);
    object
  }
}

/// C++ method: <span style='color: green;'>```bool qputenv(const char* varName, const QByteArray& value)```</span>
///
///
pub unsafe fn qputenv(var_name: *const ::libc::c_char, value: &::byte_array::ByteArray) -> bool {
  ::ffi::qt_core_c_QConstOverload_G_qputenv(var_name, value as *const ::byte_array::ByteArray)
}

/// C++ method: <span style='color: green;'>```int qrand()```</span>
///
///
pub fn qrand() -> ::libc::c_int {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qrand() }
}

/// C++ method: <span style='color: green;'>```void qsrand(unsigned int seed)```</span>
///
///
pub fn qsrand(seed: ::libc::c_uint) {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qsrand(seed) }
}

/// C++ method: <span style='color: green;'>```qt_error_string```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn qt_error_string(()) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```QString qt_error_string()```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn qt_error_string(::libc::c_int) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```QString qt_error_string(int errorCode = ?)```</span>
///
///
pub fn qt_error_string<Args>(args: Args) -> ::string::String
  where Args: overloading::QtErrorStringArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qtTrId```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn qt_tr_id(*const ::libc::c_char) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```QString qtTrId(const char* id)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn qt_tr_id((*const ::libc::c_char, ::libc::c_int)) -> ::string::String```<br>
/// C++ method: <span style='color: green;'>```QString qtTrId(const char* id, int n = ?)```</span>
///
///
pub unsafe fn qt_tr_id<Args>(args: Args) -> ::string::String
  where Args: overloading::QtTrIdArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool qunsetenv(const char* varName)```</span>
///
///
pub unsafe fn qunsetenv(var_name: *const ::libc::c_char) -> bool {
  ::ffi::qt_core_c_QConstOverload_G_qunsetenv(var_name)
}

/// C++ method: <span style='color: green;'>```void* qReallocAligned(void* ptr, unsigned long size, unsigned long oldsize, unsigned long alignment)```</span>
///
///
pub unsafe fn realloc_aligned(ptr: *mut ::libc::c_void,
                              size: ::libc::c_ulong,
                              oldsize: ::libc::c_ulong,
                              alignment: ::libc::c_ulong)
                              -> *mut ::libc::c_void {
  ::ffi::qt_core_c_QConstOverload_G_qReallocAligned(ptr, size, oldsize, alignment)
}

/// C++ method: <span style='color: green;'>```qint64 qRound64(double d)```</span>
///
///
pub fn round64_c_double(d: ::libc::c_double) -> i64 {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qRound64_double(d) }
}

/// C++ method: <span style='color: green;'>```qint64 qRound64(float d)```</span>
///
///
pub fn round64_c_float(d: ::libc::c_float) -> i64 {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qRound64_float(d) }
}

/// C++ method: <span style='color: green;'>```int qRound(double d)```</span>
///
///
pub fn round_c_double(d: ::libc::c_double) -> ::libc::c_int {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qRound_double(d) }
}

/// C++ method: <span style='color: green;'>```int qRound(float d)```</span>
///
///
pub fn round_c_float(d: ::libc::c_float) -> ::libc::c_int {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qRound_float(d) }
}

/// C++ method: <span style='color: green;'>```void qTerminate()```</span>
///
///
pub fn terminate() {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qTerminate() }
}

/// C++ method: <span style='color: green;'>```const char* qVersion()```</span>
///
///
pub fn version() -> *const ::libc::c_char {
  unsafe { ::ffi::qt_core_c_QConstOverload_G_qVersion() }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [environment_variable_int_value](../fn.environment_variable_int_value.html) method.
  pub trait EnvironmentVariableIntValueArgs {
    unsafe fn exec(self) -> ::libc::c_int;
  }
  impl EnvironmentVariableIntValueArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::libc::c_int {
      let var_name = self;
      ::ffi::qt_core_c_QConstOverload_G_qEnvironmentVariableIntValue_varName(var_name)
    }
  }
  impl EnvironmentVariableIntValueArgs for (*const ::libc::c_char, *mut bool) {
    unsafe fn exec(self) -> ::libc::c_int {
      let var_name = self.0;
      let ok = self.1;
      ::ffi::qt_core_c_QConstOverload_G_qEnvironmentVariableIntValue_varName_ok(var_name, ok)
    }
  }
  /// This trait represents a set of arguments accepted by [qt_error_string](../fn.qt_error_string.html) method.
  pub trait QtErrorStringArgs {
    fn exec(self) -> ::string::String;
  }
  impl QtErrorStringArgs for ::libc::c_int {
    fn exec(self) -> ::string::String {
      let error_code = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QConstOverload_G_qt_error_string_to_output_errorCode(error_code, &mut object);
        }
        object
      }
    }
  }
  impl QtErrorStringArgs for () {
    fn exec(self) -> ::string::String {

      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QConstOverload_G_qt_error_string_to_output_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [qt_tr_id](../fn.qt_tr_id.html) method.
  pub trait QtTrIdArgs {
    unsafe fn exec(self) -> ::string::String;
  }
  impl QtTrIdArgs for *const ::libc::c_char {
    unsafe fn exec(self) -> ::string::String {
      let id = self;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QConstOverload_G_qtTrId_to_output_id(id, &mut object);
        object
      }
    }
  }
  impl QtTrIdArgs for (*const ::libc::c_char, ::libc::c_int) {
    unsafe fn exec(self) -> ::string::String {
      let id = self.0;
      let n = self.1;
      {
        let mut object: ::string::String = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
        ::ffi::qt_core_c_QConstOverload_G_qtTrId_to_output_id_n(id, n, &mut object);
        object
      }
    }
  }
}
