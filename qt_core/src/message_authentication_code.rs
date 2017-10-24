/// C++ type: <span style='color: green;'>```QMessageAuthenticationCode```</span>
#[repr(C)]
pub struct MessageAuthenticationCode([u8; ::type_sizes::QT_CORE_MESSAGE_AUTHENTICATION_CODE_MESSAGE_AUTHENTICATION_CODE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MessageAuthenticationCode {
  unsafe fn new_uninitialized() -> MessageAuthenticationCode {
    MessageAuthenticationCode(::std::mem::uninitialized())
  }
}

impl MessageAuthenticationCode {
  /// C++ method: <span style='color: green;'>```void QMessageAuthenticationCode::addData(const QByteArray& data)```</span>
  ///
  ///
  pub fn add_data(&mut self, data: &::byte_array::ByteArray) {
    unsafe { ::ffi::qt_core_c_QMessageAuthenticationCode_addData_data(self as *mut ::message_authentication_code::MessageAuthenticationCode, data as *const ::byte_array::ByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QMessageAuthenticationCode::addData```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn add_data_unsafe(&mut self, *mut ::io_device::IODevice) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QMessageAuthenticationCode::addData(QIODevice* device)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn add_data_unsafe(&mut self, (*const ::libc::c_char, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QMessageAuthenticationCode::addData(const char* data, int length)```</span>
  ///
  ///
  pub unsafe fn add_data_unsafe<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::MessageAuthenticationCodeAddDataUnsafeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QByteArray QMessageAuthenticationCode::hash(const QByteArray& message, const QByteArray& key, QCryptographicHash::Algorithm method)```</span>
  ///
  ///
  pub fn hash(message: &::byte_array::ByteArray,
              key: &::byte_array::ByteArray,
              method: &::cryptographic_hash::Algorithm)
              -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMessageAuthenticationCode_hash_to_output(message as *const ::byte_array::ByteArray,
                                                                   key as *const ::byte_array::ByteArray,
                                                                   method as *const ::cryptographic_hash::Algorithm,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMessageAuthenticationCode::QMessageAuthenticationCode```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(&::cryptographic_hash::Algorithm) -> ::message_authentication_code::MessageAuthenticationCode```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMessageAuthenticationCode::QMessageAuthenticationCode(QCryptographicHash::Algorithm method)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new((&::cryptographic_hash::Algorithm, &::byte_array::ByteArray)) -> ::message_authentication_code::MessageAuthenticationCode```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMessageAuthenticationCode::QMessageAuthenticationCode(QCryptographicHash::Algorithm method, const QByteArray& key = ?)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::message_authentication_code::MessageAuthenticationCode
    where Args: overloading::MessageAuthenticationCodeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```void QMessageAuthenticationCode::reset()```</span>
  ///
  ///
  pub fn reset(&mut self) {
    unsafe { ::ffi::qt_core_c_QMessageAuthenticationCode_reset(self as *mut ::message_authentication_code::MessageAuthenticationCode) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QMessageAuthenticationCode::result() const```</span>
  ///
  ///
  pub fn result(&self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMessageAuthenticationCode_result_to_output(self as *const ::message_authentication_code::MessageAuthenticationCode, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```void QMessageAuthenticationCode::setKey(const QByteArray& key)```</span>
  ///
  ///
  pub fn set_key(&mut self, key: &::byte_array::ByteArray) {
    unsafe { ::ffi::qt_core_c_QMessageAuthenticationCode_setKey(self as *mut ::message_authentication_code::MessageAuthenticationCode, key as *const ::byte_array::ByteArray) }
  }
}

impl Drop for ::message_authentication_code::MessageAuthenticationCode {
  /// C++ method: <span style='color: green;'>```[destructor] void QMessageAuthenticationCode::~QMessageAuthenticationCode()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMessageAuthenticationCode_destructor(self as *mut ::message_authentication_code::MessageAuthenticationCode) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MessageAuthenticationCode::add_data_unsafe](../struct.MessageAuthenticationCode.html#method.add_data_unsafe) method.
  pub trait MessageAuthenticationCodeAddDataUnsafeArgs<'largs> {
    type ReturnType;
    unsafe fn exec(self,
                   original_self: &'largs mut ::message_authentication_code::MessageAuthenticationCode)
                   -> Self::ReturnType;
  }
  impl<'largs> MessageAuthenticationCodeAddDataUnsafeArgs<'largs> for (*const ::libc::c_char, ::libc::c_int) {
    type ReturnType = ();
    unsafe fn exec(self, original_self: &'largs mut ::message_authentication_code::MessageAuthenticationCode) -> () {
      let data = self.0;
      let length = self.1;
      ::ffi::qt_core_c_QMessageAuthenticationCode_addData_data_length(original_self as *mut ::message_authentication_code::MessageAuthenticationCode, data, length)
    }
  }
  impl<'largs> MessageAuthenticationCodeAddDataUnsafeArgs<'largs> for *mut ::io_device::IODevice {
    type ReturnType = bool;
    unsafe fn exec(self, original_self: &'largs mut ::message_authentication_code::MessageAuthenticationCode) -> bool {
      let device = self;
      ::ffi::qt_core_c_QMessageAuthenticationCode_addData_device(original_self as *mut ::message_authentication_code::MessageAuthenticationCode, device)
    }
  }
  /// This trait represents a set of arguments accepted by [MessageAuthenticationCode::new](../struct.MessageAuthenticationCode.html#method.new) method.
  pub trait MessageAuthenticationCodeNewArgs {
    fn exec(self) -> ::message_authentication_code::MessageAuthenticationCode;
  }
  impl<'a> MessageAuthenticationCodeNewArgs for &'a ::cryptographic_hash::Algorithm {
    fn exec(self) -> ::message_authentication_code::MessageAuthenticationCode {
      let method = self;
      {
        let mut object: ::message_authentication_code::MessageAuthenticationCode =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMessageAuthenticationCode_constructor_method(method as *const ::cryptographic_hash::Algorithm, &mut object);
        }
        object
      }
    }
  }
  impl<'a> MessageAuthenticationCodeNewArgs for (&'a ::cryptographic_hash::Algorithm, &'a ::byte_array::ByteArray) {
    fn exec(self) -> ::message_authentication_code::MessageAuthenticationCode {
      let method = self.0;
      let key = self.1;
      {
        let mut object: ::message_authentication_code::MessageAuthenticationCode =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMessageAuthenticationCode_constructor_method_key(method as *const ::cryptographic_hash::Algorithm, key as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
}
