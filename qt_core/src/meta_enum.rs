/// C++ type: <span style='color: green;'>```QMetaEnum```</span>
#[repr(C)]
pub struct MetaEnum([u8; ::type_sizes::QT_CORE_META_ENUM_META_ENUM]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MetaEnum {
  unsafe fn new_uninitialized() -> MetaEnum {
    MetaEnum(::std::mem::uninitialized())
  }
}

impl MetaEnum {
  /// C++ method: <span style='color: green;'>```const QMetaObject* QMetaEnum::enclosingMetaObject() const```</span>
  ///
  ///
  pub fn enclosing_meta_object(&self) -> *const ::meta_object::MetaObject {
    unsafe { ::ffi::qt_core_c_QMetaEnum_enclosingMetaObject(self as *const ::meta_enum::MetaEnum) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaEnum::isFlag() const```</span>
  ///
  ///
  pub fn is_flag(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaEnum_isFlag(self as *const ::meta_enum::MetaEnum) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaEnum::isScoped() const```</span>
  ///
  ///
  pub fn is_scoped(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaEnum_isScoped(self as *const ::meta_enum::MetaEnum) }
  }

  /// C++ method: <span style='color: green;'>```bool QMetaEnum::isValid() const```</span>
  ///
  ///
  pub fn is_valid(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMetaEnum_isValid(self as *const ::meta_enum::MetaEnum) }
  }

  /// C++ method: <span style='color: green;'>```const char* QMetaEnum::key(int index) const```</span>
  ///
  ///
  pub fn key(&self, index: ::libc::c_int) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaEnum_key(self as *const ::meta_enum::MetaEnum, index) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaEnum::keyCount() const```</span>
  ///
  ///
  pub fn key_count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaEnum_keyCount(self as *const ::meta_enum::MetaEnum) }
  }

  /// C++ method: <span style='color: green;'>```QMetaEnum::keyToValue```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn key_to_value(&self, *const ::libc::c_char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMetaEnum::keyToValue(const char* key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn key_to_value(&self, (*const ::libc::c_char, *mut bool)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMetaEnum::keyToValue(const char* key, bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn key_to_value<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::MetaEnumKeyToValueArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMetaEnum::keysToValue```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn keys_to_value(&self, *const ::libc::c_char) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMetaEnum::keysToValue(const char* keys) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn keys_to_value(&self, (*const ::libc::c_char, *mut bool)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMetaEnum::keysToValue(const char* keys, bool* ok = ?) const```</span>
  ///
  ///
  pub unsafe fn keys_to_value<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::MetaEnumKeysToValueArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const char* QMetaEnum::name() const```</span>
  ///
  ///
  pub fn name(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaEnum_name(self as *const ::meta_enum::MetaEnum) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QMetaEnum::QMetaEnum()```</span>
  ///
  ///
  pub fn new() -> ::meta_enum::MetaEnum {
    {
      let mut object: ::meta_enum::MetaEnum =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaEnum_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const char* QMetaEnum::scope() const```</span>
  ///
  ///
  pub fn scope(&self) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaEnum_scope(self as *const ::meta_enum::MetaEnum) }
  }

  /// C++ method: <span style='color: green;'>```int QMetaEnum::value(int index) const```</span>
  ///
  ///
  pub fn value(&self, index: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMetaEnum_value(self as *const ::meta_enum::MetaEnum, index) }
  }

  /// C++ method: <span style='color: green;'>```const char* QMetaEnum::valueToKey(int value) const```</span>
  ///
  ///
  pub fn value_to_key(&self, value: ::libc::c_int) -> *const ::libc::c_char {
    unsafe { ::ffi::qt_core_c_QMetaEnum_valueToKey(self as *const ::meta_enum::MetaEnum, value) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QMetaEnum::valueToKeys(int value) const```</span>
  ///
  ///
  pub fn value_to_keys(&self, value: ::libc::c_int) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMetaEnum_valueToKeys_to_output(self as *const ::meta_enum::MetaEnum, value, &mut object);
      }
      object
    }
  }
}

impl Drop for ::meta_enum::MetaEnum {
  /// C++ method: <span style='color: green;'>```[destructor] void QMetaEnum::~QMetaEnum()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMetaEnum_destructor(self as *mut ::meta_enum::MetaEnum) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MetaEnum::key_to_value](../struct.MetaEnum.html#method.key_to_value) method.
  pub trait MetaEnumKeyToValueArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::meta_enum::MetaEnum) -> ::libc::c_int;
  }
  impl<'largs> MetaEnumKeyToValueArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs ::meta_enum::MetaEnum) -> ::libc::c_int {
      let key = self;
      ::ffi::qt_core_c_QMetaEnum_keyToValue_key(original_self as *const ::meta_enum::MetaEnum, key)
    }
  }
  impl<'largs> MetaEnumKeyToValueArgs<'largs> for (*const ::libc::c_char, *mut bool) {
    unsafe fn exec(self, original_self: &'largs ::meta_enum::MetaEnum) -> ::libc::c_int {
      let key = self.0;
      let ok = self.1;
      ::ffi::qt_core_c_QMetaEnum_keyToValue_key_ok(original_self as *const ::meta_enum::MetaEnum, key, ok)
    }
  }
  /// This trait represents a set of arguments accepted by [MetaEnum::keys_to_value](../struct.MetaEnum.html#method.keys_to_value) method.
  pub trait MetaEnumKeysToValueArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::meta_enum::MetaEnum) -> ::libc::c_int;
  }
  impl<'largs> MetaEnumKeysToValueArgs<'largs> for *const ::libc::c_char {
    unsafe fn exec(self, original_self: &'largs ::meta_enum::MetaEnum) -> ::libc::c_int {
      let keys = self;
      ::ffi::qt_core_c_QMetaEnum_keysToValue_keys(original_self as *const ::meta_enum::MetaEnum, keys)
    }
  }
  impl<'largs> MetaEnumKeysToValueArgs<'largs> for (*const ::libc::c_char, *mut bool) {
    unsafe fn exec(self, original_self: &'largs ::meta_enum::MetaEnum) -> ::libc::c_int {
      let keys = self.0;
      let ok = self.1;
      ::ffi::qt_core_c_QMetaEnum_keysToValue_keys_ok(original_self as *const ::meta_enum::MetaEnum, keys, ok)
    }
  }
}
