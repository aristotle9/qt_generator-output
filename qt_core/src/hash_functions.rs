/// C++ method: <span style='color: green;'>```int qGlobalQHashSeed()```</span>
///
///
pub fn global_q_hash_seed() -> ::libc::c_int {
  unsafe { ::ffi::qt_core_c_QHashFunctions_G_qGlobalQHashSeed() }
}

/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash0(&::latin1_string::Latin1String) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(QLatin1String key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash0((&::latin1_string::Latin1String, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(QLatin1String key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn hash0(::libc::c_char) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(char key)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn hash0((::libc::c_char, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(char key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn hash0(&::bit_array::BitArray) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QBitArray& key)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn hash0((&::bit_array::BitArray, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QBitArray& key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 7
///
/// Rust arguments: ```fn hash0(&::byte_array::ByteArray) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QByteArray& key)```</span>
///
///
///
/// ## Variant 8
///
/// Rust arguments: ```fn hash0((&::byte_array::ByteArray, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QByteArray& key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 9
///
/// Rust arguments: ```fn hash0(&::char::Char) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QChar key)```</span>
///
///
///
/// ## Variant 10
///
/// Rust arguments: ```fn hash0((&::char::Char, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QChar key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 11
///
/// Rust arguments: ```fn hash0(&::string::String) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QString& key)```</span>
///
///
///
/// ## Variant 12
///
/// Rust arguments: ```fn hash0((&::string::String, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QString& key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 13
///
/// Rust arguments: ```fn hash0(&::string_ref::StringRef) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QStringRef& key)```</span>
///
///
///
/// ## Variant 14
///
/// Rust arguments: ```fn hash0((&::string_ref::StringRef, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(const QStringRef& key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 15
///
/// Rust arguments: ```fn hash0(::libc::c_double) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(double key)```</span>
///
///
///
/// ## Variant 16
///
/// Rust arguments: ```fn hash0((::libc::c_double, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(double key, unsigned int seed = ?)```</span>
///
///
pub fn hash0<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::Hash0Args
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash1(::libc::c_float) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(float key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash1((::libc::c_float, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(float key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn hash1(::libc::c_int) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(int key)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn hash1((::libc::c_int, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(int key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 5
///
/// Rust arguments: ```fn hash1(u64) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(quint64 key)```</span>
///
///
///
/// ## Variant 6
///
/// Rust arguments: ```fn hash1((u64, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(quint64 key, unsigned int seed = ?)```</span>
///
///
pub fn hash1<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::Hash1Args
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash2(::libc::c_long) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(long key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash2((::libc::c_long, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(long key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn hash2(::libc::c_uchar) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(unsigned char key)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn hash2((::libc::c_uchar, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(unsigned char key, unsigned int seed = ?)```</span>
///
///
pub fn hash2<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::Hash2Args
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash3(i64) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(qint64 key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash3((i64, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(qint64 key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn hash3(::libc::c_uint) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(unsigned int key)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn hash3((::libc::c_uint, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(unsigned int key, unsigned int seed = ?)```</span>
///
///
pub fn hash3<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::Hash3Args
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash4(::libc::c_short) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(short key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash4((::libc::c_short, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(short key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn hash4(::libc::c_ulong) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(unsigned long key)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn hash4((::libc::c_ulong, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(unsigned long key, unsigned int seed = ?)```</span>
///
///
pub fn hash4<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::Hash4Args
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qHash```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash5(::libc::c_schar) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(signed char key)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash5((::libc::c_schar, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(signed char key, unsigned int seed = ?)```</span>
///
///
///
/// ## Variant 3
///
/// Rust arguments: ```fn hash5(::libc::c_ushort) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(unsigned short key)```</span>
///
///
///
/// ## Variant 4
///
/// Rust arguments: ```fn hash5((::libc::c_ushort, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHash(unsigned short key, unsigned int seed = ?)```</span>
///
///
pub fn hash5<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::Hash5Args
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```qHashBits```</span>
///
/// This is an overloaded function. Available variants:
///
///
///
/// ## Variant 1
///
/// Rust arguments: ```fn hash_bits((*const ::libc::c_void, ::libc::c_ulong)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHashBits(const void* p, unsigned long size)```</span>
///
///
///
/// ## Variant 2
///
/// Rust arguments: ```fn hash_bits((*const ::libc::c_void, ::libc::c_ulong, ::libc::c_uint)) -> ::libc::c_uint```<br>
/// C++ method: <span style='color: green;'>```unsigned int qHashBits(const void* p, unsigned long size, unsigned int seed = ?)```</span>
///
///
pub unsafe fn hash_bits<Args>(args: Args) -> ::libc::c_uint
  where Args: overloading::HashBitsArgs
{
  args.exec()
}
/// C++ method: <span style='color: green;'>```void qSetGlobalQHashSeed(int newSeed)```</span>
///
///
pub fn set_global_q_hash_seed(new_seed: ::libc::c_int) {
  unsafe { ::ffi::qt_core_c_QHashFunctions_G_qSetGlobalQHashSeed(new_seed) }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [hash0](../fn.hash0.html) method.
  pub trait Hash0Args {
    fn exec(self) -> ::libc::c_uint;
  }
  impl<'a> Hash0Args for &'a ::bit_array::BitArray {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_QBitArray(key as *const ::bit_array::BitArray) }
    }
  }
  impl<'a> Hash0Args for (&'a ::bit_array::BitArray, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe {
        ::ffi::qt_core_c_QHashFunctions_G_qHash_QBitArray_unsigned_int(key as *const ::bit_array::BitArray, seed)
      }
    }
  }
  impl<'a> Hash0Args for &'a ::byte_array::ByteArray {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_QByteArray(key as *const ::byte_array::ByteArray) }
    }
  }
  impl<'a> Hash0Args for (&'a ::byte_array::ByteArray, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe {
        ::ffi::qt_core_c_QHashFunctions_G_qHash_QByteArray_unsigned_int(key as *const ::byte_array::ByteArray, seed)
      }
    }
  }
  impl<'a> Hash0Args for &'a ::char::Char {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_QChar(key as *const ::char::Char) }
    }
  }
  impl<'a> Hash0Args for (&'a ::char::Char, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_QChar_unsigned_int(key as *const ::char::Char, seed) }
    }
  }
  impl<'a> Hash0Args for &'a ::latin1_string::Latin1String {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_QLatin1String(key as *const ::latin1_string::Latin1String) }
    }
  }
  impl<'a> Hash0Args for (&'a ::latin1_string::Latin1String, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe {
        ::ffi::qt_core_c_QHashFunctions_G_qHash_QLatin1String_unsigned_int(key as *const ::latin1_string::Latin1String, seed)
      }
    }
  }
  impl<'a> Hash0Args for &'a ::string::String {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_QString(key as *const ::string::String) }
    }
  }
  impl<'a> Hash0Args for &'a ::string_ref::StringRef {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_QStringRef(key as *const ::string_ref::StringRef) }
    }
  }
  impl<'a> Hash0Args for (&'a ::string_ref::StringRef, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe {
        ::ffi::qt_core_c_QHashFunctions_G_qHash_QStringRef_unsigned_int(key as *const ::string_ref::StringRef, seed)
      }
    }
  }
  impl<'a> Hash0Args for (&'a ::string::String, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_QString_unsigned_int(key as *const ::string::String, seed) }
    }
  }
  impl Hash0Args for ::libc::c_char {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_char(key) }
    }
  }
  impl Hash0Args for (::libc::c_char, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_char_unsigned_int(key, seed) }
    }
  }
  impl Hash0Args for ::libc::c_double {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_double(key) }
    }
  }
  impl Hash0Args for (::libc::c_double, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_double_unsigned_int(key, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [hash1](../fn.hash1.html) method.
  pub trait Hash1Args {
    fn exec(self) -> ::libc::c_uint;
  }
  impl Hash1Args for ::libc::c_float {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_float(key) }
    }
  }
  impl Hash1Args for (::libc::c_float, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_float_unsigned_int(key, seed) }
    }
  }
  impl Hash1Args for ::libc::c_int {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_int(key) }
    }
  }
  impl Hash1Args for (::libc::c_int, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_int_unsigned_int(key, seed) }
    }
  }
  impl Hash1Args for u64 {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_quint64(key) }
    }
  }
  impl Hash1Args for (u64, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_quint64_unsigned_int(key, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [hash2](../fn.hash2.html) method.
  pub trait Hash2Args {
    fn exec(self) -> ::libc::c_uint;
  }
  impl Hash2Args for ::libc::c_long {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_long(key) }
    }
  }
  impl Hash2Args for (::libc::c_long, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_long_unsigned_int(key, seed) }
    }
  }
  impl Hash2Args for ::libc::c_uchar {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_unsigned_char(key) }
    }
  }
  impl Hash2Args for (::libc::c_uchar, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_unsigned_char_unsigned_int(key, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [hash3](../fn.hash3.html) method.
  pub trait Hash3Args {
    fn exec(self) -> ::libc::c_uint;
  }
  impl Hash3Args for i64 {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_qint64(key) }
    }
  }
  impl Hash3Args for (i64, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_qint64_unsigned_int(key, seed) }
    }
  }
  impl Hash3Args for ::libc::c_uint {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_unsigned_int(key) }
    }
  }
  impl Hash3Args for (::libc::c_uint, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_unsigned_int_unsigned_int(key, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [hash4](../fn.hash4.html) method.
  pub trait Hash4Args {
    fn exec(self) -> ::libc::c_uint;
  }
  impl Hash4Args for ::libc::c_short {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_short(key) }
    }
  }
  impl Hash4Args for (::libc::c_short, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_short_unsigned_int(key, seed) }
    }
  }
  impl Hash4Args for ::libc::c_ulong {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_unsigned_long(key) }
    }
  }
  impl Hash4Args for (::libc::c_ulong, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_unsigned_long_unsigned_int(key, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [hash5](../fn.hash5.html) method.
  pub trait Hash5Args {
    fn exec(self) -> ::libc::c_uint;
  }
  impl Hash5Args for ::libc::c_schar {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_signed_char(key) }
    }
  }
  impl Hash5Args for (::libc::c_schar, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_signed_char_unsigned_int(key, seed) }
    }
  }
  impl Hash5Args for ::libc::c_ushort {
    fn exec(self) -> ::libc::c_uint {
      let key = self;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_unsigned_short(key) }
    }
  }
  impl Hash5Args for (::libc::c_ushort, ::libc::c_uint) {
    fn exec(self) -> ::libc::c_uint {
      let key = self.0;
      let seed = self.1;
      unsafe { ::ffi::qt_core_c_QHashFunctions_G_qHash_unsigned_short_unsigned_int(key, seed) }
    }
  }
  /// This trait represents a set of arguments accepted by [hash_bits](../fn.hash_bits.html) method.
  pub trait HashBitsArgs {
    unsafe fn exec(self) -> ::libc::c_uint;
  }
  impl HashBitsArgs for (*const ::libc::c_void, ::libc::c_ulong) {
    unsafe fn exec(self) -> ::libc::c_uint {
      let p = self.0;
      let size = self.1;
      ::ffi::qt_core_c_QHashFunctions_G_qHashBits_p_size(p, size)
    }
  }
  impl HashBitsArgs for (*const ::libc::c_void, ::libc::c_ulong, ::libc::c_uint) {
    unsafe fn exec(self) -> ::libc::c_uint {
      let p = self.0;
      let size = self.1;
      let seed = self.2;
      ::ffi::qt_core_c_QHashFunctions_G_qHashBits_p_size_seed(p, size, seed)
    }
  }
}
