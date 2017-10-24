/// C++ type: <span style='color: green;'>```QHash<int, QByteArray>```</span>
#[repr(C)]
pub struct HashCIntByteArray([u8; ::type_sizes::QT_CORE_HASH_HASH_C_INT_BYTE_ARRAY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for HashCIntByteArray {
  unsafe fn new_uninitialized() -> HashCIntByteArray {
    HashCIntByteArray(::std::mem::uninitialized())
  }
}

impl HashCIntByteArray {
  /// C++ method: <span style='color: green;'>```int QHash<int, QByteArray>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QHash_int_QByteArray_capacity(self as *const ::hash::HashCIntByteArray) }
  }

  /// C++ method: <span style='color: green;'>```void QHash<int, QByteArray>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QHash_int_QByteArray_clear(self as *mut ::hash::HashCIntByteArray) }
  }

  /// C++ method: <span style='color: green;'>```bool QHash<int, QByteArray>::contains(const int& key) const```</span>
  ///
  ///
  pub fn contains(&self, key: &::libc::c_int) -> bool {
    unsafe {
      ::ffi::qt_core_c_QHash_int_QByteArray_contains(self as *const ::hash::HashCIntByteArray,
                                                     key as *const ::libc::c_int)
    }
  }

  /// C++ method: <span style='color: green;'>```QHash<int, QByteArray>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QHash<int, QByteArray>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QHash<int, QByteArray>::count(const int& key) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::HashCIntByteArrayCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QHash<int, QByteArray>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QHash_int_QByteArray_empty(self as *const ::hash::HashCIntByteArray) }
  }

  /// C++ method: <span style='color: green;'>```bool QHash<int, QByteArray>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QHash_int_QByteArray_isEmpty(self as *const ::hash::HashCIntByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QHash<int, QByteArray>::key```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn key(&self, &::byte_array::ByteArray) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```const int QHash<int, QByteArray>::key(const QByteArray& value) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn key(&self, (&::byte_array::ByteArray, &::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```const int QHash<int, QByteArray>::key(const QByteArray& value, const int& defaultKey) const```</span>
  ///
  ///
  pub fn key<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::HashCIntByteArrayKeyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QHash<int, QByteArray>::keys```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn keys(&self, ()) -> ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int> QHash<int, QByteArray>::keys() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn keys(&self, &::byte_array::ByteArray) -> ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int> QHash<int, QByteArray>::keys(const QByteArray& value) const```</span>
  ///
  ///
  pub fn keys<'largs, Args>(&'largs self, args: Args) -> ::list::ListCInt
    where Args: overloading::HashCIntByteArrayKeysArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QHash<int, QByteArray>::QHash()```</span>
  ///
  ///
  pub fn new() -> ::hash::HashCIntByteArray {
    {
      let mut object: ::hash::HashCIntByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QHash_int_QByteArray_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QByteArray QHash<int, QByteArray>::operator[](const int& key) const```</span>
  ///
  ///
  pub fn op_index(&self, key: &::libc::c_int) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QHash_int_QByteArray_operator_index_to_output(self as *const ::hash::HashCIntByteArray,
                                                                       key as *const ::libc::c_int,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray& QHash<int, QByteArray>::operator[](const int& key)```</span>
  ///
  ///
  pub fn op_index_mut<'l0, 'l1>(&'l0 mut self, key: &'l1 ::libc::c_int) -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QHash_int_QByteArray_operator_index(self as *mut ::hash::HashCIntByteArray,
                                                           key as *const ::libc::c_int)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QHash<int, QByteArray>::remove(const int& key)```</span>
  ///
  ///
  pub fn remove(&mut self, key: &::libc::c_int) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QHash_int_QByteArray_remove(self as *mut ::hash::HashCIntByteArray,
                                                   key as *const ::libc::c_int)
    }
  }

  /// C++ method: <span style='color: green;'>```void QHash<int, QByteArray>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QHash_int_QByteArray_reserve(self as *mut ::hash::HashCIntByteArray, size) }
  }

  /// C++ method: <span style='color: green;'>```int QHash<int, QByteArray>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QHash_int_QByteArray_size(self as *const ::hash::HashCIntByteArray) }
  }

  /// C++ method: <span style='color: green;'>```void QHash<int, QByteArray>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QHash_int_QByteArray_squeeze(self as *mut ::hash::HashCIntByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QHash<int, QByteArray>::take(const int& key)```</span>
  ///
  ///
  pub fn take(&mut self, key: &::libc::c_int) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QHash_int_QByteArray_take_to_output(self as *mut ::hash::HashCIntByteArray,
                                                             key as *const ::libc::c_int,
                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<int> QHash<int, QByteArray>::uniqueKeys() const```</span>
  ///
  ///
  pub fn unique_keys(&self) -> ::list::ListCInt {
    {
      let mut object: ::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QHash_int_QByteArray_uniqueKeys_to_output(self as *const ::hash::HashCIntByteArray,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QHash<int, QByteArray>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, &::libc::c_int) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```const QByteArray QHash<int, QByteArray>::value(const int& key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (&::libc::c_int, &::byte_array::ByteArray)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```const QByteArray QHash<int, QByteArray>::value(const int& key, const QByteArray& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::HashCIntByteArrayValueArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QHash<int, QByteArray>::values```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn values(&self, ()) -> ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```QList<QByteArray> QHash<int, QByteArray>::values() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn values(&self, &::libc::c_int) -> ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```QList<QByteArray> QHash<int, QByteArray>::values(const int& key) const```</span>
  ///
  ///
  pub fn values<'largs, Args>(&'largs self, args: Args) -> ::list::ListByteArray
    where Args: overloading::HashCIntByteArrayValuesArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::hash::HashCIntByteArray {
  /// C++ method: <span style='color: green;'>```[destructor] void QHash<int, QByteArray>::~QHash()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QHash_int_QByteArray_destructor(self as *mut ::hash::HashCIntByteArray) }
  }
}

/// C++ type: <span style='color: green;'>```QHash<QString, QVariant>```</span>
#[repr(C)]
pub struct HashStringVariant([u8; ::type_sizes::QT_CORE_HASH_HASH_STRING_VARIANT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for HashStringVariant {
  unsafe fn new_uninitialized() -> HashStringVariant {
    HashStringVariant(::std::mem::uninitialized())
  }
}

impl HashStringVariant {
  /// C++ method: <span style='color: green;'>```int QHash<QString, QVariant>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QHash_QString_QVariant_capacity(self as *const ::hash::HashStringVariant) }
  }

  /// C++ method: <span style='color: green;'>```void QHash<QString, QVariant>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QHash_QString_QVariant_clear(self as *mut ::hash::HashStringVariant) }
  }

  /// C++ method: <span style='color: green;'>```bool QHash<QString, QVariant>::contains(const QString& key) const```</span>
  ///
  ///
  pub fn contains(&self, key: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QHash_QString_QVariant_contains(self as *const ::hash::HashStringVariant,
                                                       key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QHash<QString, QVariant>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QHash<QString, QVariant>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QHash<QString, QVariant>::count(const QString& key) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::HashStringVariantCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QHash<QString, QVariant>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QHash_QString_QVariant_empty(self as *const ::hash::HashStringVariant) }
  }

  /// C++ method: <span style='color: green;'>```bool QHash<QString, QVariant>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QHash_QString_QVariant_isEmpty(self as *const ::hash::HashStringVariant) }
  }

  /// C++ method: <span style='color: green;'>```QHash<QString, QVariant>::key```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn key(&self, &::variant::Variant) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```const QString QHash<QString, QVariant>::key(const QVariant& value) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn key(&self, (&::variant::Variant, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```const QString QHash<QString, QVariant>::key(const QVariant& value, const QString& defaultKey) const```</span>
  ///
  ///
  pub fn key<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::HashStringVariantKeyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QHash<QString, QVariant>::keys```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn keys(&self, ()) -> ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```QList<QString> QHash<QString, QVariant>::keys() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn keys(&self, &::variant::Variant) -> ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```QList<QString> QHash<QString, QVariant>::keys(const QVariant& value) const```</span>
  ///
  ///
  pub fn keys<'largs, Args>(&'largs self, args: Args) -> ::list::ListString
    where Args: overloading::HashStringVariantKeysArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QHash<QString, QVariant>::QHash()```</span>
  ///
  ///
  pub fn new() -> ::hash::HashStringVariant {
    {
      let mut object: ::hash::HashStringVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QHash_QString_QVariant_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QVariant QHash<QString, QVariant>::operator[](const QString& key) const```</span>
  ///
  ///
  pub fn op_index(&self, key: &::string::String) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QHash_QString_QVariant_operator_index_to_output(self as *const ::hash::HashStringVariant,
                                                                         key as *const ::string::String,
                                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant& QHash<QString, QVariant>::operator[](const QString& key)```</span>
  ///
  ///
  pub fn op_index_mut<'l0, 'l1>(&'l0 mut self, key: &'l1 ::string::String) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QHash_QString_QVariant_operator_index(self as *mut ::hash::HashStringVariant,
                                                             key as *const ::string::String)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QHash<QString, QVariant>::remove(const QString& key)```</span>
  ///
  ///
  pub fn remove(&mut self, key: &::string::String) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QHash_QString_QVariant_remove(self as *mut ::hash::HashStringVariant,
                                                     key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QHash<QString, QVariant>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QHash_QString_QVariant_reserve(self as *mut ::hash::HashStringVariant, size) }
  }

  /// C++ method: <span style='color: green;'>```int QHash<QString, QVariant>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QHash_QString_QVariant_size(self as *const ::hash::HashStringVariant) }
  }

  /// C++ method: <span style='color: green;'>```void QHash<QString, QVariant>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QHash_QString_QVariant_squeeze(self as *mut ::hash::HashStringVariant) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QHash<QString, QVariant>::take(const QString& key)```</span>
  ///
  ///
  pub fn take(&mut self, key: &::string::String) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QHash_QString_QVariant_take_to_output(self as *mut ::hash::HashStringVariant,
                                                               key as *const ::string::String,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QString> QHash<QString, QVariant>::uniqueKeys() const```</span>
  ///
  ///
  pub fn unique_keys(&self) -> ::list::ListString {
    {
      let mut object: ::list::ListString =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QHash_QString_QVariant_uniqueKeys_to_output(self as *const ::hash::HashStringVariant,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QHash<QString, QVariant>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, &::string::String) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```const QVariant QHash<QString, QVariant>::value(const QString& key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (&::string::String, &::variant::Variant)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```const QVariant QHash<QString, QVariant>::value(const QString& key, const QVariant& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::HashStringVariantValueArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QHash<QString, QVariant>::values```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn values(&self, ()) -> ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant> QHash<QString, QVariant>::values() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn values(&self, &::string::String) -> ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant> QHash<QString, QVariant>::values(const QString& key) const```</span>
  ///
  ///
  pub fn values<'largs, Args>(&'largs self, args: Args) -> ::list::ListVariant
    where Args: overloading::HashStringVariantValuesArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::hash::HashStringVariant {
  /// C++ method: <span style='color: green;'>```[destructor] void QHash<QString, QVariant>::~QHash()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QHash_QString_QVariant_destructor(self as *mut ::hash::HashStringVariant) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [HashCIntByteArray::count](../struct.HashCIntByteArray.html#method.count) method.
  pub trait HashCIntByteArrayCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::libc::c_int;
  }
  impl<'largs> HashCIntByteArrayCountArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::libc::c_int {
      let key = self;
      unsafe {
        ::ffi::qt_core_c_QHash_int_QByteArray_count_key(original_self as *const ::hash::HashCIntByteArray,
                                                        key as *const ::libc::c_int)
      }
    }
  }
  impl<'largs> HashCIntByteArrayCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QHash_int_QByteArray_count_no_args(original_self as *const ::hash::HashCIntByteArray) }
    }
  }
  /// This trait represents a set of arguments accepted by [HashCIntByteArray::key](../struct.HashCIntByteArray.html#method.key) method.
  pub trait HashCIntByteArrayKeyArgs<'largs> {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::libc::c_int;
  }
  impl<'largs> HashCIntByteArrayKeyArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::libc::c_int {
      let value = self;
      unsafe {
        ::ffi::qt_core_c_QHash_int_QByteArray_key_value(original_self as *const ::hash::HashCIntByteArray,
                                                        value as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> HashCIntByteArrayKeyArgs<'largs> for (&'largs ::byte_array::ByteArray, &'largs ::libc::c_int) {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::libc::c_int {
      let value = self.0;
      let default_key = self.1;
      unsafe {
        ::ffi::qt_core_c_QHash_int_QByteArray_key_value_defaultKey(original_self as *const ::hash::HashCIntByteArray,
                                                                   value as *const ::byte_array::ByteArray,
                                                                   default_key as *const ::libc::c_int)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [HashCIntByteArray::keys](../struct.HashCIntByteArray.html#method.keys) method.
  pub trait HashCIntByteArrayKeysArgs<'largs> {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::list::ListCInt;
  }
  impl<'largs> HashCIntByteArrayKeysArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::list::ListCInt {

      {
        let mut object: ::list::ListCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_int_QByteArray_keys_to_output_no_args(original_self as *const ::hash::HashCIntByteArray, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> HashCIntByteArrayKeysArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::list::ListCInt {
      let value = self;
      {
        let mut object: ::list::ListCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_int_QByteArray_keys_to_output_value(original_self as *const ::hash::HashCIntByteArray, value as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [HashCIntByteArray::value](../struct.HashCIntByteArray.html#method.value) method.
  pub trait HashCIntByteArrayValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::byte_array::ByteArray;
  }
  impl<'largs> HashCIntByteArrayValueArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::byte_array::ByteArray {
      let key = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_int_QByteArray_value_to_output_key(original_self as *const ::hash::HashCIntByteArray, key as *const ::libc::c_int, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> HashCIntByteArrayValueArgs<'largs> for (&'largs ::libc::c_int, &'largs ::byte_array::ByteArray) {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::byte_array::ByteArray {
      let key = self.0;
      let default_value = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_int_QByteArray_value_to_output_key_defaultValue(original_self as *const ::hash::HashCIntByteArray, key as *const ::libc::c_int, default_value as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [HashCIntByteArray::values](../struct.HashCIntByteArray.html#method.values) method.
  pub trait HashCIntByteArrayValuesArgs<'largs> {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::list::ListByteArray;
  }
  impl<'largs> HashCIntByteArrayValuesArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::list::ListByteArray {
      let key = self;
      {
        let mut object: ::list::ListByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_int_QByteArray_values_to_output_key(original_self as *const ::hash::HashCIntByteArray, key as *const ::libc::c_int, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> HashCIntByteArrayValuesArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::hash::HashCIntByteArray) -> ::list::ListByteArray {

      {
        let mut object: ::list::ListByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_int_QByteArray_values_to_output_no_args(original_self as *const ::hash::HashCIntByteArray, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [HashStringVariant::count](../struct.HashStringVariant.html#method.count) method.
  pub trait HashStringVariantCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::libc::c_int;
  }
  impl<'largs> HashStringVariantCountArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::libc::c_int {
      let key = self;
      unsafe {
        ::ffi::qt_core_c_QHash_QString_QVariant_count_key(original_self as *const ::hash::HashStringVariant,
                                                          key as *const ::string::String)
      }
    }
  }
  impl<'largs> HashStringVariantCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::libc::c_int {

      unsafe {
        ::ffi::qt_core_c_QHash_QString_QVariant_count_no_args(original_self as *const ::hash::HashStringVariant)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [HashStringVariant::key](../struct.HashStringVariant.html#method.key) method.
  pub trait HashStringVariantKeyArgs<'largs> {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::string::String;
  }
  impl<'largs> HashStringVariantKeyArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::string::String {
      let value = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_QString_QVariant_key_to_output_value(original_self as *const ::hash::HashStringVariant, value as *const ::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> HashStringVariantKeyArgs<'largs> for (&'largs ::variant::Variant, &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::string::String {
      let value = self.0;
      let default_key = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_QString_QVariant_key_to_output_value_defaultKey(original_self as *const ::hash::HashStringVariant, value as *const ::variant::Variant, default_key as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [HashStringVariant::keys](../struct.HashStringVariant.html#method.keys) method.
  pub trait HashStringVariantKeysArgs<'largs> {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::list::ListString;
  }
  impl<'largs> HashStringVariantKeysArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::list::ListString {

      {
        let mut object: ::list::ListString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_QString_QVariant_keys_to_output_no_args(original_self as *const ::hash::HashStringVariant, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> HashStringVariantKeysArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::list::ListString {
      let value = self;
      {
        let mut object: ::list::ListString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_QString_QVariant_keys_to_output_value(original_self as *const ::hash::HashStringVariant, value as *const ::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [HashStringVariant::value](../struct.HashStringVariant.html#method.value) method.
  pub trait HashStringVariantValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::variant::Variant;
  }
  impl<'largs> HashStringVariantValueArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::variant::Variant {
      let key = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_QString_QVariant_value_to_output_key(original_self as *const ::hash::HashStringVariant, key as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> HashStringVariantValueArgs<'largs> for (&'largs ::string::String, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::variant::Variant {
      let key = self.0;
      let default_value = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_QString_QVariant_value_to_output_key_defaultValue(original_self as *const ::hash::HashStringVariant, key as *const ::string::String, default_value as *const ::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [HashStringVariant::values](../struct.HashStringVariant.html#method.values) method.
  pub trait HashStringVariantValuesArgs<'largs> {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::list::ListVariant;
  }
  impl<'largs> HashStringVariantValuesArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::list::ListVariant {
      let key = self;
      {
        let mut object: ::list::ListVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_QString_QVariant_values_to_output_key(original_self as *const ::hash::HashStringVariant, key as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> HashStringVariantValuesArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::hash::HashStringVariant) -> ::list::ListVariant {

      {
        let mut object: ::list::ListVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QHash_QString_QVariant_values_to_output_no_args(original_self as *const ::hash::HashStringVariant, &mut object);
        }
        object
      }
    }
  }
}
