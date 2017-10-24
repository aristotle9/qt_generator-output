/// C++ type: <span style='color: green;'>```QCryptographicHash::Algorithm```</span>
#[derive(Debug, PartialEq, Eq, Clone)]
#[repr(C)]
pub enum Algorithm {
  /// C++ enum variant: <span style='color: green;'>```Md4 = 0```</span>
  Md4 = 0,
  /// C++ enum variant: <span style='color: green;'>```Md5 = 1```</span>
  Md5 = 1,
  /// C++ enum variant: <span style='color: green;'>```Sha1 = 2```</span>
  Sha1 = 2,
  /// C++ enum variant: <span style='color: green;'>```Sha224 = 3```</span>
  Sha224 = 3,
  /// C++ enum variant: <span style='color: green;'>```Sha256 = 4```</span>
  Sha256 = 4,
  /// C++ enum variant: <span style='color: green;'>```Sha384 = 5```</span>
  Sha384 = 5,
  /// C++ enum variant: <span style='color: green;'>```Sha512 = 6```</span>
  Sha512 = 6,
  /// C++ enum variant: <span style='color: green;'>```Sha3_224 = 7```</span>
  Sha3224 = 7,
  /// C++ enum variant: <span style='color: green;'>```Sha3_256 = 8```</span>
  Sha3256 = 8,
  /// C++ enum variant: <span style='color: green;'>```Sha3_384 = 9```</span>
  Sha3384 = 9,
  /// C++ enum variant: <span style='color: green;'>```Sha3_512 = 10```</span>
  Sha3512 = 10,
}

/// C++ type: <span style='color: green;'>```QCryptographicHash```</span>
#[repr(C)]
pub struct CryptographicHash([u8; ::type_sizes::QT_CORE_CRYPTOGRAPHIC_HASH_CRYPTOGRAPHIC_HASH]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for CryptographicHash {
  unsafe fn new_uninitialized() -> CryptographicHash {
    CryptographicHash(::std::mem::uninitialized())
  }
}

impl CryptographicHash {
  /// C++ method: <span style='color: green;'>```void QCryptographicHash::addData(const QByteArray& data)```</span>
  ///
  ///
  pub fn add_data(&mut self, data: &::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QCryptographicHash_addData_data(self as *mut ::cryptographic_hash::CryptographicHash,
                                                       data as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QCryptographicHash::addData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_data_unsafe(&mut self, *mut ::io_device::IODevice) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QCryptographicHash::addData(QIODevice* device)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_data_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QCryptographicHash::addData(const char* data, int length)```</span>
  ///
  ///
  pub unsafe fn add_data_unsafe<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::CryptographicHashAddDataUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QByteArray QCryptographicHash::hash(const QByteArray& data, QCryptographicHash::Algorithm method)```</span>
  ///
  ///
  pub fn hash(data: &::byte_array::ByteArray, method: ::cryptographic_hash::Algorithm) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCryptographicHash_hash_to_output(data as *const ::byte_array::ByteArray, method, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QCryptographicHash::QCryptographicHash(QCryptographicHash::Algorithm method)```</span>
  ///
  ///
  pub fn new(method: ::cryptographic_hash::Algorithm) -> ::cryptographic_hash::CryptographicHash {
    {
      let mut object: ::cryptographic_hash::CryptographicHash =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCryptographicHash_constructor(method, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QCryptographicHash::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_core_c_QCryptographicHash_reset(self as *mut ::cryptographic_hash::CryptographicHash) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QCryptographicHash::result() const```</span>
  ///
  ///
  pub fn result(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QCryptographicHash_result_to_output(self as *const ::cryptographic_hash::CryptographicHash,
                                                             &mut object);
      }
      object
    }
  }
}

impl Drop for ::cryptographic_hash::CryptographicHash {
  /// C++ method: <span style='color: green;'>```[destructor] void QCryptographicHash::~QCryptographicHash()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QCryptographicHash_destructor(self as *mut ::cryptographic_hash::CryptographicHash) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [CryptographicHash::add_data_unsafe](../struct.CryptographicHash.html#method.add_data_unsafe) method.
  pub trait CryptographicHashAddDataUnsafeArgs<'largs> {
    type ReturnType;
    unsafe fn exec(self, original_self: &'largs mut ::cryptographic_hash::CryptographicHash) -> Self::ReturnType;
  }
  impl<'largs> CryptographicHashAddDataUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_int) {
    type ReturnType = ();
    unsafe fn exec(self, original_self: &'largs mut ::cryptographic_hash::CryptographicHash) -> () {
      let data = self.0;
      let length = self.1;
      ::ffi::qt_core_c_QCryptographicHash_addData_data_length(original_self as *mut ::cryptographic_hash::CryptographicHash, data, length)
    }
  }
  impl<'largs> CryptographicHashAddDataUnsafeArgs<'largs> for *mut ::io_device::IODevice {
    type ReturnType = bool;
    unsafe fn exec(self, original_self: &'largs mut ::cryptographic_hash::CryptographicHash) -> bool {
      let device = self;
      ::ffi::qt_core_c_QCryptographicHash_addData_device(original_self as *mut ::cryptographic_hash::CryptographicHash, device)
    }
  }
}
