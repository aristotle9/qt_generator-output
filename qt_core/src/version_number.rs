/// C++ type: <span style='color: green;'>```QVersionNumber```</span>
#[repr(C)]
pub struct VersionNumber([u8; ::type_sizes::QT_CORE_VERSION_NUMBER_VERSION_NUMBER]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VersionNumber {
  unsafe fn new_uninitialized() -> VersionNumber {
    VersionNumber(::std::mem::uninitialized())
  }
}

impl VersionNumber {
  /// C++ method: <span style='color: green;'>```static QVersionNumber QVersionNumber::commonPrefix(const QVersionNumber& v1, const QVersionNumber& v2)```</span>
  ///
  ///
  pub fn common_prefix(v1: &::version_number::VersionNumber,
                       v2: &::version_number::VersionNumber)
                       -> ::version_number::VersionNumber {
    {
      let mut object: ::version_number::VersionNumber =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVersionNumber_commonPrefix_to_output(v1 as *const ::version_number::VersionNumber,
                                                               v2 as *const ::version_number::VersionNumber,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static int QVersionNumber::compare(const QVersionNumber& v1, const QVersionNumber& v2)```</span>
  ///
  ///
  pub fn compare(v1: &::version_number::VersionNumber, v2: &::version_number::VersionNumber) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QVersionNumber_compare(v1 as *const ::version_number::VersionNumber,
                                              v2 as *const ::version_number::VersionNumber)
    }
  }

  /// C++ method: <span style='color: green;'>```static QVersionNumber QVersionNumber::fromString(const QString& string)```</span>
  ///
  ///
  pub fn from_string(string: &::string::String) -> ::version_number::VersionNumber {
    {
      let mut object: ::version_number::VersionNumber =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVersionNumber_fromString_to_output_string(string as *const ::string::String, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QVersionNumber QVersionNumber::fromString(const QString& string, int* suffixIndex = ?)```</span>
  ///
  ///
  pub unsafe fn from_string_unsafe(string: &::string::String,
                                   suffix_index: *mut ::libc::c_int)
                                   -> ::version_number::VersionNumber {
    {
      let mut object: ::version_number::VersionNumber =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QVersionNumber_fromString_to_output_string_suffixIndex(string as *const ::string::String,
                                                                              suffix_index,
                                                                              &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVersionNumber::isNormalized() const```</span>
  ///
  ///
  pub fn is_normalized(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVersionNumber_isNormalized(self as *const ::version_number::VersionNumber) }
  }

  /// C++ method: <span style='color: green;'>```bool QVersionNumber::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QVersionNumber_isNull(self as *const ::version_number::VersionNumber) }
  }

  /// C++ method: <span style='color: green;'>```bool QVersionNumber::isPrefixOf(const QVersionNumber& other) const```</span>
  ///
  ///
  pub fn is_prefix_of(&self, other: &::version_number::VersionNumber) -> bool {
    unsafe {
      ::ffi::qt_core_c_QVersionNumber_isPrefixOf(self as *const ::version_number::VersionNumber,
                                                 other as *const ::version_number::VersionNumber)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVersionNumber::majorVersion() const```</span>
  ///
  ///
  pub fn major_version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVersionNumber_majorVersion(self as *const ::version_number::VersionNumber) }
  }

  /// C++ method: <span style='color: green;'>```int QVersionNumber::microVersion() const```</span>
  ///
  ///
  pub fn micro_version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVersionNumber_microVersion(self as *const ::version_number::VersionNumber) }
  }

  /// C++ method: <span style='color: green;'>```int QVersionNumber::minorVersion() const```</span>
  ///
  ///
  pub fn minor_version(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVersionNumber_minorVersion(self as *const ::version_number::VersionNumber) }
  }

  /// C++ method: <span style='color: green;'>```QVersionNumber::QVersionNumber```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::version_number::VersionNumber```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVersionNumber::QVersionNumber()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorCInt) -> ::version_number::VersionNumber```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVersionNumber::QVersionNumber(const QVector<int>& seg)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::version_number::VersionNumber```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVersionNumber::QVersionNumber(int maj)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int)) -> ::version_number::VersionNumber```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVersionNumber::QVersionNumber(int maj, int min)```</span>
  ///
  ///
  ///
  /// ## Variant 5
  ///
  /// Rust arguments: ```fn new((::libc::c_int, ::libc::c_int, ::libc::c_int)) -> ::version_number::VersionNumber```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVersionNumber::QVersionNumber(int maj, int min, int mic)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::version_number::VersionNumber
    where Args: overloading::VersionNumberNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVersionNumber QVersionNumber::normalized() const```</span>
  ///
  ///
  pub fn normalized(&self) -> ::version_number::VersionNumber {
    {
      let mut object: ::version_number::VersionNumber =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVersionNumber_normalized_to_output(self as *const ::version_number::VersionNumber,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QVersionNumber::segmentAt(int index) const```</span>
  ///
  ///
  pub fn segment_at(&self, index: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVersionNumber_segmentAt(self as *const ::version_number::VersionNumber, index) }
  }

  /// C++ method: <span style='color: green;'>```int QVersionNumber::segmentCount() const```</span>
  ///
  ///
  pub fn segment_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QVersionNumber_segmentCount(self as *const ::version_number::VersionNumber) }
  }

  /// C++ method: <span style='color: green;'>```QVector<int> QVersionNumber::segments() const```</span>
  ///
  ///
  pub fn segments(&self) -> ::vector::VectorCInt {
    {
      let mut object: ::vector::VectorCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVersionNumber_segments_to_output(self as *const ::version_number::VersionNumber, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QVersionNumber::toString() const```</span>
  ///
  ///
  pub fn to_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QVersionNumber_toString_to_output(self as *const ::version_number::VersionNumber, &mut object);
      }
      object
    }
  }
}

impl Drop for ::version_number::VersionNumber {
  /// C++ method: <span style='color: green;'>```[destructor] void QVersionNumber::~QVersionNumber()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QVersionNumber_destructor(self as *mut ::version_number::VersionNumber) }
  }
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::version_number::VersionNumber) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QVersionNumber& key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::version_number::VersionNumber, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QVersionNumber& key, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator==(const QVersionNumber& lhs, const QVersionNumber& rhs)```</span>
///
///
pub fn op_eq(lhs: &::version_number::VersionNumber, rhs: &::version_number::VersionNumber) -> bool {
  unsafe {
    ::ffi::qt_core_c_QVersionNumber_G_operator_eq(lhs as *const ::version_number::VersionNumber,
                                                  rhs as *const ::version_number::VersionNumber)
  }
}

/// C++ method: <span style='color: green;'>```bool operator>=(const QVersionNumber& lhs, const QVersionNumber& rhs)```</span>
///
///
pub fn op_ge(lhs: &::version_number::VersionNumber, rhs: &::version_number::VersionNumber) -> bool {
  unsafe {
    ::ffi::qt_core_c_QVersionNumber_G_operator_ge(lhs as *const ::version_number::VersionNumber,
                                                  rhs as *const ::version_number::VersionNumber)
  }
}

/// C++ method: <span style='color: green;'>```bool operator>(const QVersionNumber& lhs, const QVersionNumber& rhs)```</span>
///
///
pub fn op_gt(lhs: &::version_number::VersionNumber, rhs: &::version_number::VersionNumber) -> bool {
  unsafe {
    ::ffi::qt_core_c_QVersionNumber_G_operator_gt(lhs as *const ::version_number::VersionNumber,
                                                  rhs as *const ::version_number::VersionNumber)
  }
}

/// C++ method: <span style='color: green;'>```bool operator<=(const QVersionNumber& lhs, const QVersionNumber& rhs)```</span>
///
///
pub fn op_le(lhs: &::version_number::VersionNumber, rhs: &::version_number::VersionNumber) -> bool {
  unsafe {
    ::ffi::qt_core_c_QVersionNumber_G_operator_le(lhs as *const ::version_number::VersionNumber,
                                                  rhs as *const ::version_number::VersionNumber)
  }
}

/// C++ method: <span style='color: green;'>```bool operator<(const QVersionNumber& lhs, const QVersionNumber& rhs)```</span>
///
///
pub fn op_lt(lhs: &::version_number::VersionNumber, rhs: &::version_number::VersionNumber) -> bool {
  unsafe {
    ::ffi::qt_core_c_QVersionNumber_G_operator_lt(lhs as *const ::version_number::VersionNumber,
                                                  rhs as *const ::version_number::VersionNumber)
  }
}

/// C++ method: <span style='color: green;'>```bool operator!=(const QVersionNumber& lhs, const QVersionNumber& rhs)```</span>
///
///
pub fn op_neq(lhs: &::version_number::VersionNumber, rhs: &::version_number::VersionNumber) -> bool {
  unsafe {
    ::ffi::qt_core_c_QVersionNumber_G_operator_neq(lhs as *const ::version_number::VersionNumber,
                                                   rhs as *const ::version_number::VersionNumber)
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
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::version_number::VersionNumber)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& out, const QVersionNumber& version)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::version_number::VersionNumber)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QVersionNumber& version)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& in, QVersionNumber& version)```</span>
///
///
pub fn op_shr<'l0, 'l1>(in_: &'l0 mut ::data_stream::DataStream,
                        version: &'l1 mut ::version_number::VersionNumber)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_core_c_QVersionNumber_G_operator_shr(in_ as *mut ::data_stream::DataStream,
                                                   version as *mut ::version_number::VersionNumber)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [VersionNumber::new](../struct.VersionNumber.html#method.new) method.
  pub trait VersionNumberNewArgs {
    fn exec(self) -> ::version_number::VersionNumber;
  }
  impl VersionNumberNewArgs for ::libc::c_int {
    fn exec(self) -> ::version_number::VersionNumber {
      let maj = self;
      {
        let mut object: ::version_number::VersionNumber =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVersionNumber_constructor_maj(maj, &mut object);
        }
        object
      }
    }
  }
  impl VersionNumberNewArgs for (::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::version_number::VersionNumber {
      let maj = self.0;
      let min = self.1;
      {
        let mut object: ::version_number::VersionNumber =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVersionNumber_constructor_maj_min(maj, min, &mut object);
        }
        object
      }
    }
  }
  impl VersionNumberNewArgs for (::libc::c_int, ::libc::c_int, ::libc::c_int) {
    fn exec(self) -> ::version_number::VersionNumber {
      let maj = self.0;
      let min = self.1;
      let mic = self.2;
      {
        let mut object: ::version_number::VersionNumber =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVersionNumber_constructor_maj_min_mic(maj, min, mic, &mut object);
        }
        object
      }
    }
  }
  impl VersionNumberNewArgs for () {
    fn exec(self) -> ::version_number::VersionNumber {

      {
        let mut object: ::version_number::VersionNumber =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVersionNumber_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> VersionNumberNewArgs for &'a ::vector::VectorCInt {
    fn exec(self) -> ::version_number::VersionNumber {
      let seg = self;
      {
        let mut object: ::version_number::VersionNumber =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVersionNumber_constructor_seg(seg as *const ::vector::VectorCInt, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::version_number::VersionNumber {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QVersionNumber_G_qHash_key(key as *const ::version_number::VersionNumber) }
    }
  }
  impl<'a> HashArgs for (&'a ::version_number::VersionNumber, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QVersionNumber_G_qHash_key_seed(key as *const ::version_number::VersionNumber, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::version_number::VersionNumber) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let out = self.0;
      let version = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QVersionNumber_G_operator_shl(out as *mut ::data_stream::DataStream,
                                                         version as *const ::version_number::VersionNumber)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::version_number::VersionNumber) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let version = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QVersionNumber_G_operator_shl_to_output(arg1 as *const ::debug::Debug,
                                                                   version as *const ::version_number::VersionNumber,
                                                                   &mut object);
        }
        object
      }
    }
  }
}
