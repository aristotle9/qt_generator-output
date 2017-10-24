/// C++ type: <span style='color: green;'>```QUuid```</span>
#[repr(C)]
pub struct Uuid([u8; ::type_sizes::QT_CORE_UUID_UUID]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Uuid {
  unsafe fn new_uninitialized() -> Uuid {
    Uuid(::std::mem::uninitialized())
  }
}

impl Uuid {
  /// C++ method: <span style='color: green;'>```static QUuid QUuid::createUuid()```</span>
  ///
  ///
  pub fn create_uuid() -> ::uuid::Uuid {
    {
      let mut object: ::uuid::Uuid = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUuid_createUuid_to_output(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUuid::createUuidV3```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_uuid_v3((&::uuid::Uuid, &::byte_array::ByteArray)) -> ::uuid::Uuid```<br>
  /// C++ method: <span style='color: green;'>```static QUuid QUuid::createUuidV3(const QUuid& ns, const QByteArray& baseData)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_uuid_v3((&::uuid::Uuid, &::string::String)) -> ::uuid::Uuid```<br>
  /// C++ method: <span style='color: green;'>```static QUuid QUuid::createUuidV3(const QUuid& ns, const QString& baseData)```</span>
  ///
  ///
  pub fn create_uuid_v3<Args>(args: Args) -> ::uuid::Uuid
    where Args: overloading::UuidCreateUuidV3Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QUuid::createUuidV5```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn create_uuid_v5((&::uuid::Uuid, &::byte_array::ByteArray)) -> ::uuid::Uuid```<br>
  /// C++ method: <span style='color: green;'>```static QUuid QUuid::createUuidV5(const QUuid& ns, const QByteArray& baseData)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn create_uuid_v5((&::uuid::Uuid, &::string::String)) -> ::uuid::Uuid```<br>
  /// C++ method: <span style='color: green;'>```static QUuid QUuid::createUuidV5(const QUuid& ns, const QString& baseData)```</span>
  ///
  ///
  pub fn create_uuid_v5<Args>(args: Args) -> ::uuid::Uuid
    where Args: overloading::UuidCreateUuidV5Args
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```unsigned int QUuid::data1() const```</span>
  ///
  ///
  pub fn data1(&self) -> ::libc::c_uint {
    unsafe { ::ffi::qt_core_c_QUuid_data1(self as *const ::uuid::Uuid) }
  }

  /// C++ method: <span style='color: green;'>```unsigned short QUuid::data2() const```</span>
  ///
  ///
  pub fn data2(&self) -> ::libc::c_ushort {
    unsafe { ::ffi::qt_core_c_QUuid_data2(self as *const ::uuid::Uuid) }
  }

  /// C++ method: <span style='color: green;'>```unsigned short QUuid::data3() const```</span>
  ///
  ///
  pub fn data3(&self) -> ::libc::c_ushort {
    unsafe { ::ffi::qt_core_c_QUuid_data3(self as *const ::uuid::Uuid) }
  }

  /// C++ method: <span style='color: green;'>```static QUuid QUuid::fromRfc4122(const QByteArray& arg1)```</span>
  ///
  ///
  pub fn from_rfc4122(arg1: &::byte_array::ByteArray) -> ::uuid::Uuid {
    {
      let mut object: ::uuid::Uuid = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUuid_fromRfc4122_to_output(arg1 as *const ::byte_array::ByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QUuid::isNull() const```</span>
  ///
  ///
  pub fn is_null(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QUuid_isNull(self as *const ::uuid::Uuid) }
  }

  /// C++ method: <span style='color: green;'>```QUuid::QUuid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::uuid::Uuid```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUuid::QUuid()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::byte_array::ByteArray) -> ::uuid::Uuid```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUuid::QUuid(const QByteArray& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::string::String) -> ::uuid::Uuid```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUuid::QUuid(const QString& arg1)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_uint, ::libc::c_ushort, ::libc::c_ushort, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar, ::libc::c_uchar)) -> ::uuid::Uuid```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QUuid::QUuid(unsigned int l, unsigned short w1, unsigned short w2, unsigned char b1, unsigned char b2, unsigned char b3, unsigned char b4, unsigned char b5, unsigned char b6, unsigned char b7, unsigned char b8)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::uuid::Uuid
    where Args: overloading::UuidNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QUuid::QUuid(const char* arg1)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(arg1: *const ::libc::c_char) -> ::uuid::Uuid {
    {
      let mut object: ::uuid::Uuid = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QUuid_constructor_char(arg1, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QUuid::operator==(const QUuid& orig) const```</span>
  ///
  ///
  pub fn op_eq(&self, orig: &::uuid::Uuid) -> bool {
    unsafe { ::ffi::qt_core_c_QUuid_operator_eq(self as *const ::uuid::Uuid, orig as *const ::uuid::Uuid) }
  }

  /// C++ method: <span style='color: green;'>```bool QUuid::operator>(const QUuid& other) const```</span>
  ///
  ///
  pub fn op_gt(&self, other: &::uuid::Uuid) -> bool {
    unsafe { ::ffi::qt_core_c_QUuid_operator_gt(self as *const ::uuid::Uuid, other as *const ::uuid::Uuid) }
  }

  /// C++ method: <span style='color: green;'>```bool QUuid::operator<(const QUuid& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::uuid::Uuid) -> bool {
    unsafe { ::ffi::qt_core_c_QUuid_operator_lt(self as *const ::uuid::Uuid, other as *const ::uuid::Uuid) }
  }

  /// C++ method: <span style='color: green;'>```bool QUuid::operator!=(const QUuid& orig) const```</span>
  ///
  ///
  pub fn op_neq(&self, orig: &::uuid::Uuid) -> bool {
    unsafe { ::ffi::qt_core_c_QUuid_operator_neq(self as *const ::uuid::Uuid, orig as *const ::uuid::Uuid) }
  }

  /// C++ method: <span style='color: green;'>```void QUuid::set_data1(unsigned int value)```</span>
  ///
  ///
  pub fn set_data1(&mut self, value: ::libc::c_uint) {
    unsafe { ::ffi::qt_core_c_QUuid_set_data1(self as *mut ::uuid::Uuid, value) }
  }

  /// C++ method: <span style='color: green;'>```void QUuid::set_data2(unsigned short value)```</span>
  ///
  ///
  pub fn set_data2(&mut self, value: ::libc::c_ushort) {
    unsafe { ::ffi::qt_core_c_QUuid_set_data2(self as *mut ::uuid::Uuid, value) }
  }

  /// C++ method: <span style='color: green;'>```void QUuid::set_data3(unsigned short value)```</span>
  ///
  ///
  pub fn set_data3(&mut self, value: ::libc::c_ushort) {
    unsafe { ::ffi::qt_core_c_QUuid_set_data3(self as *mut ::uuid::Uuid, value) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QUuid::toByteArray() const```</span>
  ///
  ///
  pub fn to_byte_array(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUuid_toByteArray_to_output(self as *const ::uuid::Uuid, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QUuid::toRfc4122() const```</span>
  ///
  ///
  pub fn to_rfc4122(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUuid_toRfc4122_to_output(self as *const ::uuid::Uuid, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QUuid::toString() const```</span>
  ///
  ///
  pub fn to_string(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QUuid_toString_to_output(self as *const ::uuid::Uuid, &mut object);
      }
      object
    }
  }
}

impl Drop for ::uuid::Uuid {
  /// C++ method: <span style='color: green;'>```[destructor] void QUuid::~QUuid()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QUuid_destructor(self as *mut ::uuid::Uuid) }
  }
}

/// C++ type: <span style='color: green;'>```QUuid::Variant```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Variant {
  /// C++ enum variant: <span style='color: green;'>```VarUnknown = -1```</span>
  VarUnknown = -1,
  /// C++ enum variant: <span style='color: green;'>```NCS = 0```</span>
  NCS = 0,
  /// C++ enum variant: <span style='color: green;'>```DCE = 2```</span>
  DCE = 2,
  /// C++ enum variant: <span style='color: green;'>```Microsoft = 6```</span>
  Microsoft = 6,
  /// C++ enum variant: <span style='color: green;'>```Reserved = 7```</span>
  Reserved = 7,
}

/// C++ type: <span style='color: green;'>```QUuid::Version```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Version {
  /// C++ enum variant: <span style='color: green;'>```VerUnknown = -1```</span>
  VerUnknown = -1,
  /// C++ enum variant: <span style='color: green;'>```Time = 1```</span>
  Time = 1,
  /// C++ enum variant: <span style='color: green;'>```EmbeddedPOSIX = 2```</span>
  EmbeddedPOSIX = 2,
  /// This variant corresponds to multiple C++ enum variants with the same value:
  ///
  /// - <span style='color: green;'>```Md5 = 3```</span>
  /// - <span style='color: green;'>```Name = 3```</span>
  ///
  Md5 = 3,
  /// C++ enum variant: <span style='color: green;'>```Random = 4```</span>
  Random = 4,
  /// C++ enum variant: <span style='color: green;'>```Sha1 = 5```</span>
  Sha1 = 5,
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash(&::uuid::Uuid) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QUuid& uuid)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash((&::uuid::Uuid, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QUuid& uuid, unsigned int seed = ?)```</span>
///
///
pub fn hash<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```bool operator>=(const QUuid& lhs, const QUuid& rhs)```</span>
///
///
pub fn op_ge(lhs: &::uuid::Uuid, rhs: &::uuid::Uuid) -> bool {
  unsafe { ::ffi::qt_core_c_QUuid_G_operator_ge(lhs as *const ::uuid::Uuid, rhs as *const ::uuid::Uuid) }
}

/// C++ method: <span style='color: green;'>```bool operator<=(const QUuid& lhs, const QUuid& rhs)```</span>
///
///
pub fn op_le(lhs: &::uuid::Uuid, rhs: &::uuid::Uuid) -> bool {
  unsafe { ::ffi::qt_core_c_QUuid_G_operator_le(lhs as *const ::uuid::Uuid, rhs as *const ::uuid::Uuid) }
}

/// C++ method: <span style='color: green;'>```operator<<```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn op_shl((&'l0 mut ::data_stream::DataStream, &'l1 ::uuid::Uuid)) -> &'l0 mut ::data_stream::DataStream```<br>
/// C++ method: <span style='color: green;'>```QDataStream& operator<<(QDataStream& arg1, const QUuid& arg2)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn op_shl((&::debug::Debug, &::uuid::Uuid)) -> ::debug::Debug```<br>
/// C++ method: <span style='color: green;'>```QDebug operator<<(QDebug arg1, const QUuid& arg2)```</span>
///
///
pub fn op_shl<Args>(args: Args) -> Args::ReturnType
  where Args: overloading::OpShlArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```QDataStream& operator>>(QDataStream& arg1, QUuid& arg2)```</span>
///
///
pub fn op_shr<'l0, 'l1>(arg1: &'l0 mut ::data_stream::DataStream,
                        arg2: &'l1 mut ::uuid::Uuid)
                        -> &'l0 mut ::data_stream::DataStream {
  let ffi_result = unsafe {
    ::ffi::qt_core_c_QUuid_G_operator_shr(arg1 as *mut ::data_stream::DataStream,
                                          arg2 as *mut ::uuid::Uuid)
  };
  unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [Uuid::create_uuid_v3](../struct.Uuid.html#method.create_uuid_v3) method.
  pub trait UuidCreateUuidV3Args {
    fn exec(self) -> ::uuid::Uuid;
  }
  impl<'a> UuidCreateUuidV3Args for (&'a ::uuid::Uuid, &'a ::byte_array::ByteArray) {
    fn exec(self) -> ::uuid::Uuid {
      let ns = self.0;
      let base_data = self.1;
      {
        let mut object: ::uuid::Uuid = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUuid_createUuidV3_to_output_QUuid_QByteArray(ns as *const ::uuid::Uuid,
                                                                         base_data as *const ::byte_array::ByteArray,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'a> UuidCreateUuidV3Args for (&'a ::uuid::Uuid, &'a ::string::String) {
    fn exec(self) -> ::uuid::Uuid {
      let ns = self.0;
      let base_data = self.1;
      {
        let mut object: ::uuid::Uuid = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUuid_createUuidV3_to_output_QUuid_QString(ns as *const ::uuid::Uuid,
                                                                      base_data as *const ::string::String,
                                                                      &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Uuid::create_uuid_v5](../struct.Uuid.html#method.create_uuid_v5) method.
  pub trait UuidCreateUuidV5Args {
    fn exec(self) -> ::uuid::Uuid;
  }
  impl<'a> UuidCreateUuidV5Args for (&'a ::uuid::Uuid, &'a ::byte_array::ByteArray) {
    fn exec(self) -> ::uuid::Uuid {
      let ns = self.0;
      let base_data = self.1;
      {
        let mut object: ::uuid::Uuid = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUuid_createUuidV5_to_output_QUuid_QByteArray(ns as *const ::uuid::Uuid,
                                                                         base_data as *const ::byte_array::ByteArray,
                                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'a> UuidCreateUuidV5Args for (&'a ::uuid::Uuid, &'a ::string::String) {
    fn exec(self) -> ::uuid::Uuid {
      let ns = self.0;
      let base_data = self.1;
      {
        let mut object: ::uuid::Uuid = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUuid_createUuidV5_to_output_QUuid_QString(ns as *const ::uuid::Uuid,
                                                                      base_data as *const ::string::String,
                                                                      &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Uuid::new](../struct.Uuid.html#method.new) method.
  pub trait UuidNewArgs {
    fn exec(self) -> ::uuid::Uuid;
  }
  impl<'a> UuidNewArgs for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::uuid::Uuid {
      let arg1 = self;
      {
        let mut object: ::uuid::Uuid = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUuid_constructor_QByteArray(arg1 as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'a> UuidNewArgs for &'a ::string::String {
    fn exec(self) -> ::uuid::Uuid {
      let arg1 = self;
      {
        let mut object: ::uuid::Uuid = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUuid_constructor_QString(arg1 as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl UuidNewArgs for () {
    fn exec(self) -> ::uuid::Uuid {

      {
        let mut object: ::uuid::Uuid = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUuid_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl UuidNewArgs
    for (::libc::c_uint,
                            ::libc::c_ushort,
                            ::libc::c_ushort,
                            ::libc::c_uchar,
                            ::libc::c_uchar,
                            ::libc::c_uchar,
                            ::libc::c_uchar,
                            ::libc::c_uchar,
                            ::libc::c_uchar,
                            ::libc::c_uchar,
                            ::libc::c_uchar) {
    fn exec(self) -> ::uuid::Uuid {
      let l = self.0;
      let w1 = self.1;
      let w2 = self.2;
      let b1 = self.3;
      let b2 = self.4;
      let b3 = self.5;
      let b4 = self.6;
      let b5 = self.7;
      let b6 = self.8;
      let b7 = self.9;
      let b8 = self.10;
      {
        let mut object: ::uuid::Uuid = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUuid_constructor_unsigned_int_unsigned_short_unsigned_short_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char_unsigned_char(l, w1, w2, b1, b2, b3, b4, b5, b6, b7, b8, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [hash](../fn.hash.html) method.
  pub trait HashArgs {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> HashArgs for &'a ::uuid::Uuid {
    fn exec(self) -> ::libc::c_uint {
      let uuid = self;
      unsafe { ::ffi::qt_core_c_QUuid_G_qHash_uuid(uuid as *const ::uuid::Uuid) }
    }
  }
  impl<'a> HashArgs for (&'a ::uuid::Uuid, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let uuid = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QUuid_G_qHash_uuid_seed(uuid as *const ::uuid::Uuid, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [op_shl](../fn.op_shl.html) method.
  pub trait OpShlArgs {
    type ReturnType;
    fn exec(self) -> Self::ReturnType;
  }
  impl<'a> OpShlArgs for (&'a mut ::data_stream::DataStream, &'a ::uuid::Uuid) {
    type ReturnType = &'a mut ::data_stream::DataStream;
    fn exec(self) -> &'a mut ::data_stream::DataStream {
      let arg1 = self.0;
      let arg2 = self.1;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QUuid_G_operator_shl(arg1 as *mut ::data_stream::DataStream,
                                              arg2 as *const ::uuid::Uuid)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'a> OpShlArgs for (&'a ::debug::Debug, &'a ::uuid::Uuid) {
    type ReturnType = ::debug::Debug;
    fn exec(self) -> ::debug::Debug {
      let arg1 = self.0;
      let arg2 = self.1;
      {
        let mut object: ::debug::Debug =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QUuid_G_operator_shl_to_output(arg1 as *const ::debug::Debug,
                                                          arg2 as *const ::uuid::Uuid,
                                                          &mut object);
        }
        object
      }
    }
  }
}
