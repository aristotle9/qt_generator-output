/// C++ type: <span style='color: green;'>```QMap<int, QVariant>```</span>
#[repr(C)]
pub struct MapCIntVariant([u8; ::type_sizes::QT_CORE_MAP_MAP_C_INT_VARIANT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MapCIntVariant {
  unsafe fn new_uninitialized() -> MapCIntVariant {
    MapCIntVariant(::std::mem::uninitialized())
  }
}

impl MapCIntVariant {
  /// C++ method: <span style='color: green;'>```void QMap<int, QVariant>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QMap_int_QVariant_clear(self as *mut ::map::MapCIntVariant) }
  }

  /// C++ method: <span style='color: green;'>```bool QMap<int, QVariant>::contains(const int& key) const```</span>
  ///
  ///
  pub fn contains(&self, key: &::libc::c_int) -> bool {
    unsafe {
      ::ffi::qt_core_c_QMap_int_QVariant_contains(self as *const ::map::MapCIntVariant,
                                                  key as *const ::libc::c_int)
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<int, QVariant>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMap<int, QVariant>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMap<int, QVariant>::count(const int& key) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::MapCIntVariantCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QMap<int, QVariant>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMap_int_QVariant_empty(self as *const ::map::MapCIntVariant) }
  }

  /// C++ method: <span style='color: green;'>```const QVariant& QMap<int, QVariant>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMap_int_QVariant_first_const(self as *const ::map::MapCIntVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const int& QMap<int, QVariant>::firstKey() const```</span>
  ///
  ///
  pub fn first_key<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMap_int_QVariant_firstKey(self as *const ::map::MapCIntVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVariant& QMap<int, QVariant>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMap_int_QVariant_first(self as *mut ::map::MapCIntVariant) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<int, QVariant>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMap_int_QVariant_isEmpty(self as *const ::map::MapCIntVariant) }
  }

  /// C++ method: <span style='color: green;'>```QMap<int, QVariant>::key```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn key(&self, &::variant::Variant) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```const int QMap<int, QVariant>::key(const QVariant& value) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn key(&self, (&::variant::Variant, &::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```const int QMap<int, QVariant>::key(const QVariant& value, const int& defaultKey = ?) const```</span>
  ///
  ///
  pub fn key<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::MapCIntVariantKeyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMap<int, QVariant>::keys```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn keys(&self, ()) -> ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int> QMap<int, QVariant>::keys() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn keys(&self, &::variant::Variant) -> ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int> QMap<int, QVariant>::keys(const QVariant& value) const```</span>
  ///
  ///
  pub fn keys<'largs, Args>(&'largs self, args: Args) -> ::list::ListCInt
    where Args: overloading::MapCIntVariantKeysArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QVariant& QMap<int, QVariant>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMap_int_QVariant_last_const(self as *const ::map::MapCIntVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const int& QMap<int, QVariant>::lastKey() const```</span>
  ///
  ///
  pub fn last_key<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMap_int_QVariant_lastKey(self as *const ::map::MapCIntVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVariant& QMap<int, QVariant>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMap_int_QVariant_last(self as *mut ::map::MapCIntVariant) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMap<int, QVariant>::QMap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::map::MapCIntVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMap<int, QVariant>::QMap()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::map::MapCIntVariant) -> ::map::MapCIntVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMap<int, QVariant>::QMap(const QMap<int, QVariant>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::map::MapCIntVariant
    where Args: overloading::MapCIntVariantNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMap<int, QVariant>& QMap<int, QVariant>::operator=(const QMap<int, QVariant>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::map::MapCIntVariant) -> &'l0 mut ::map::MapCIntVariant {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QMap_int_QVariant_operator_assign(self as *mut ::map::MapCIntVariant,
                                                         other as *const ::map::MapCIntVariant)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<int, QVariant>::operator==(const QMap<int, QVariant>& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::map::MapCIntVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QMap_int_QVariant_operator_eq(self as *const ::map::MapCIntVariant,
                                                     other as *const ::map::MapCIntVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```const QVariant QMap<int, QVariant>::operator[](const int& key) const```</span>
  ///
  ///
  pub fn op_index(&self, key: &::libc::c_int) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMap_int_QVariant_operator_index_to_output(self as *const ::map::MapCIntVariant,
                                                                    key as *const ::libc::c_int,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant& QMap<int, QVariant>::operator[](const int& key)```</span>
  ///
  ///
  pub fn op_index_mut<'l0, 'l1>(&'l0 mut self, key: &'l1 ::libc::c_int) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QMap_int_QVariant_operator_index(self as *mut ::map::MapCIntVariant,
                                                        key as *const ::libc::c_int)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<int, QVariant>::operator!=(const QMap<int, QVariant>& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::map::MapCIntVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QMap_int_QVariant_operator_neq(self as *const ::map::MapCIntVariant,
                                                      other as *const ::map::MapCIntVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```int QMap<int, QVariant>::remove(const int& key)```</span>
  ///
  ///
  pub fn remove(&mut self, key: &::libc::c_int) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QMap_int_QVariant_remove(self as *mut ::map::MapCIntVariant,
                                                key as *const ::libc::c_int)
    }
  }

  /// C++ method: <span style='color: green;'>```int QMap<int, QVariant>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMap_int_QVariant_size(self as *const ::map::MapCIntVariant) }
  }

  /// C++ method: <span style='color: green;'>```void QMap<int, QVariant>::swap(QMap<int, QVariant>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::map::MapCIntVariant) {
    unsafe {
      ::ffi::qt_core_c_QMap_int_QVariant_swap(self as *mut ::map::MapCIntVariant,
                                              other as *mut ::map::MapCIntVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QMap<int, QVariant>::take(const int& key)```</span>
  ///
  ///
  pub fn take(&mut self, key: &::libc::c_int) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMap_int_QVariant_take_to_output(self as *mut ::map::MapCIntVariant,
                                                          key as *const ::libc::c_int,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<int> QMap<int, QVariant>::uniqueKeys() const```</span>
  ///
  ///
  pub fn unique_keys(&self) -> ::list::ListCInt {
    {
      let mut object: ::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMap_int_QVariant_uniqueKeys_to_output(self as *const ::map::MapCIntVariant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<int, QVariant>& QMap<int, QVariant>::unite(const QMap<int, QVariant>& other)```</span>
  ///
  ///
  pub fn unite<'l0, 'l1>(&'l0 mut self, other: &'l1 ::map::MapCIntVariant) -> &'l0 mut ::map::MapCIntVariant {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QMap_int_QVariant_unite(self as *mut ::map::MapCIntVariant,
                                               other as *const ::map::MapCIntVariant)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMap<int, QVariant>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, &::libc::c_int) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```const QVariant QMap<int, QVariant>::value(const int& key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (&::libc::c_int, &::variant::Variant)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```const QVariant QMap<int, QVariant>::value(const int& key, const QVariant& defaultValue = ?) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::MapCIntVariantValueArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMap<int, QVariant>::values```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn values(&self, ()) -> ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant> QMap<int, QVariant>::values() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn values(&self, &::libc::c_int) -> ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant> QMap<int, QVariant>::values(const int& key) const```</span>
  ///
  ///
  pub fn values<'largs, Args>(&'largs self, args: Args) -> ::list::ListVariant
    where Args: overloading::MapCIntVariantValuesArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::map::MapCIntVariant {
  /// C++ method: <span style='color: green;'>```[destructor] void QMap<int, QVariant>::~QMap()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMap_int_QVariant_destructor(self as *mut ::map::MapCIntVariant) }
  }
}

/// C++ type: <span style='color: green;'>```QMap<QString, QVariant>```</span>
#[repr(C)]
pub struct MapStringVariant([u8; ::type_sizes::QT_CORE_MAP_MAP_STRING_VARIANT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for MapStringVariant {
  unsafe fn new_uninitialized() -> MapStringVariant {
    MapStringVariant(::std::mem::uninitialized())
  }
}

impl MapStringVariant {
  /// C++ method: <span style='color: green;'>```void QMap<QString, QVariant>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_clear(self as *mut ::map::MapStringVariant) }
  }

  /// C++ method: <span style='color: green;'>```bool QMap<QString, QVariant>::contains(const QString& key) const```</span>
  ///
  ///
  pub fn contains(&self, key: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QMap_QString_QVariant_contains(self as *const ::map::MapStringVariant,
                                                      key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<QString, QVariant>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMap<QString, QVariant>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QMap<QString, QVariant>::count(const QString& key) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::MapStringVariantCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QMap<QString, QVariant>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_empty(self as *const ::map::MapStringVariant) }
  }

  /// C++ method: <span style='color: green;'>```const QVariant& QMap<QString, QVariant>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::variant::Variant {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_first_const(self as *const ::map::MapStringVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QString& QMap<QString, QVariant>::firstKey() const```</span>
  ///
  ///
  pub fn first_key<'l0>(&'l0 self) -> &'l0 ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_firstKey(self as *const ::map::MapStringVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVariant& QMap<QString, QVariant>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_first(self as *mut ::map::MapStringVariant) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<QString, QVariant>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_isEmpty(self as *const ::map::MapStringVariant) }
  }

  /// C++ method: <span style='color: green;'>```QMap<QString, QVariant>::key```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn key(&self, &::variant::Variant) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```const QString QMap<QString, QVariant>::key(const QVariant& value) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn key(&self, (&::variant::Variant, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```const QString QMap<QString, QVariant>::key(const QVariant& value, const QString& defaultKey = ?) const```</span>
  ///
  ///
  pub fn key<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::MapStringVariantKeyArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMap<QString, QVariant>::keys```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn keys(&self, ()) -> ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```QList<QString> QMap<QString, QVariant>::keys() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn keys(&self, &::variant::Variant) -> ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```QList<QString> QMap<QString, QVariant>::keys(const QVariant& value) const```</span>
  ///
  ///
  pub fn keys<'largs, Args>(&'largs self, args: Args) -> ::list::ListString
    where Args: overloading::MapStringVariantKeysArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QVariant& QMap<QString, QVariant>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::variant::Variant {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_last_const(self as *const ::map::MapStringVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QString& QMap<QString, QVariant>::lastKey() const```</span>
  ///
  ///
  pub fn last_key<'l0>(&'l0 self) -> &'l0 ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_lastKey(self as *const ::map::MapStringVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVariant& QMap<QString, QVariant>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_last(self as *mut ::map::MapStringVariant) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMap<QString, QVariant>::QMap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::map::MapStringVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMap<QString, QVariant>::QMap()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::map::MapStringVariant) -> ::map::MapStringVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QMap<QString, QVariant>::QMap(const QMap<QString, QVariant>& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::map::MapStringVariant
    where Args: overloading::MapStringVariantNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QMap<QString, QVariant>& QMap<QString, QVariant>::operator=(const QMap<QString, QVariant>& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::map::MapStringVariant) -> &'l0 mut ::map::MapStringVariant {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QMap_QString_QVariant_operator_assign(self as *mut ::map::MapStringVariant,
                                                             other as *const ::map::MapStringVariant)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<QString, QVariant>::operator==(const QMap<QString, QVariant>& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::map::MapStringVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QMap_QString_QVariant_operator_eq(self as *const ::map::MapStringVariant,
                                                         other as *const ::map::MapStringVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```const QVariant QMap<QString, QVariant>::operator[](const QString& key) const```</span>
  ///
  ///
  pub fn op_index(&self, key: &::string::String) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMap_QString_QVariant_operator_index_to_output(self as *const ::map::MapStringVariant,
                                                                        key as *const ::string::String,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant& QMap<QString, QVariant>::operator[](const QString& key)```</span>
  ///
  ///
  pub fn op_index_mut<'l0, 'l1>(&'l0 mut self, key: &'l1 ::string::String) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QMap_QString_QVariant_operator_index(self as *mut ::map::MapStringVariant,
                                                            key as *const ::string::String)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QMap<QString, QVariant>::operator!=(const QMap<QString, QVariant>& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::map::MapStringVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QMap_QString_QVariant_operator_neq(self as *const ::map::MapStringVariant,
                                                          other as *const ::map::MapStringVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```int QMap<QString, QVariant>::remove(const QString& key)```</span>
  ///
  ///
  pub fn remove(&mut self, key: &::string::String) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QMap_QString_QVariant_remove(self as *mut ::map::MapStringVariant,
                                                    key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```int QMap<QString, QVariant>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_size(self as *const ::map::MapStringVariant) }
  }

  /// C++ method: <span style='color: green;'>```void QMap<QString, QVariant>::swap(QMap<QString, QVariant>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::map::MapStringVariant) {
    unsafe {
      ::ffi::qt_core_c_QMap_QString_QVariant_swap(self as *mut ::map::MapStringVariant,
                                                  other as *mut ::map::MapStringVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QMap<QString, QVariant>::take(const QString& key)```</span>
  ///
  ///
  pub fn take(&mut self, key: &::string::String) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMap_QString_QVariant_take_to_output(self as *mut ::map::MapStringVariant,
                                                              key as *const ::string::String,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QString> QMap<QString, QVariant>::uniqueKeys() const```</span>
  ///
  ///
  pub fn unique_keys(&self) -> ::list::ListString {
    {
      let mut object: ::list::ListString =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QMap_QString_QVariant_uniqueKeys_to_output(self as *const ::map::MapStringVariant,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<QString, QVariant>& QMap<QString, QVariant>::unite(const QMap<QString, QVariant>& other)```</span>
  ///
  ///
  pub fn unite<'l0, 'l1>(&'l0 mut self, other: &'l1 ::map::MapStringVariant) -> &'l0 mut ::map::MapStringVariant {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QMap_QString_QVariant_unite(self as *mut ::map::MapStringVariant,
                                                   other as *const ::map::MapStringVariant)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMap<QString, QVariant>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, &::string::String) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```const QVariant QMap<QString, QVariant>::value(const QString& key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (&::string::String, &::variant::Variant)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```const QVariant QMap<QString, QVariant>::value(const QString& key, const QVariant& defaultValue = ?) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::MapStringVariantValueArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMap<QString, QVariant>::values```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn values(&self, ()) -> ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant> QMap<QString, QVariant>::values() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn values(&self, &::string::String) -> ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant> QMap<QString, QVariant>::values(const QString& key) const```</span>
  ///
  ///
  pub fn values<'largs, Args>(&'largs self, args: Args) -> ::list::ListVariant
    where Args: overloading::MapStringVariantValuesArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::map::MapStringVariant {
  /// C++ method: <span style='color: green;'>```[destructor] void QMap<QString, QVariant>::~QMap()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_destructor(self as *mut ::map::MapStringVariant) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [MapCIntVariant::count](../struct.MapCIntVariant.html#method.count) method.
  pub trait MapCIntVariantCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::libc::c_int;
  }
  impl<'largs> MapCIntVariantCountArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::libc::c_int {
      let key = self;
      unsafe {
        ::ffi::qt_core_c_QMap_int_QVariant_count_key(original_self as *const ::map::MapCIntVariant,
                                                     key as *const ::libc::c_int)
      }
    }
  }
  impl<'largs> MapCIntVariantCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QMap_int_QVariant_count_no_args(original_self as *const ::map::MapCIntVariant) }
    }
  }
  /// This trait represents a set of arguments accepted by [MapCIntVariant::key](../struct.MapCIntVariant.html#method.key) method.
  pub trait MapCIntVariantKeyArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::libc::c_int;
  }
  impl<'largs> MapCIntVariantKeyArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::libc::c_int {
      let value = self;
      unsafe {
        ::ffi::qt_core_c_QMap_int_QVariant_key_value(original_self as *const ::map::MapCIntVariant,
                                                     value as *const ::variant::Variant)
      }
    }
  }
  impl<'largs> MapCIntVariantKeyArgs<'largs> for (&'largs ::variant::Variant, &'largs ::libc::c_int) {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::libc::c_int {
      let value = self.0;
      let default_key = self.1;
      unsafe {
        ::ffi::qt_core_c_QMap_int_QVariant_key_value_defaultKey(original_self as *const ::map::MapCIntVariant,
                                                                value as *const ::variant::Variant,
                                                                default_key as *const ::libc::c_int)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapCIntVariant::keys](../struct.MapCIntVariant.html#method.keys) method.
  pub trait MapCIntVariantKeysArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::list::ListCInt;
  }
  impl<'largs> MapCIntVariantKeysArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::list::ListCInt {

      {
        let mut object: ::list::ListCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_int_QVariant_keys_to_output_no_args(original_self as *const ::map::MapCIntVariant,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MapCIntVariantKeysArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::list::ListCInt {
      let value = self;
      {
        let mut object: ::list::ListCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_int_QVariant_keys_to_output_value(original_self as *const ::map::MapCIntVariant,
                                                                  value as *const ::variant::Variant,
                                                                  &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapCIntVariant::new](../struct.MapCIntVariant.html#method.new) method.
  pub trait MapCIntVariantNewArgs {
    fn exec(self) -> ::map::MapCIntVariant;
  }
  impl MapCIntVariantNewArgs for () {
    fn exec(self) -> ::map::MapCIntVariant {

      {
        let mut object: ::map::MapCIntVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_int_QVariant_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> MapCIntVariantNewArgs for &'a ::map::MapCIntVariant {
    fn exec(self) -> ::map::MapCIntVariant {
      let other = self;
      {
        let mut object: ::map::MapCIntVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_int_QVariant_constructor_other(other as *const ::map::MapCIntVariant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapCIntVariant::value](../struct.MapCIntVariant.html#method.value) method.
  pub trait MapCIntVariantValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::variant::Variant;
  }
  impl<'largs> MapCIntVariantValueArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::variant::Variant {
      let key = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_int_QVariant_value_to_output_key(original_self as *const ::map::MapCIntVariant,
                                                                 key as *const ::libc::c_int,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MapCIntVariantValueArgs<'largs> for (&'largs ::libc::c_int, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::variant::Variant {
      let key = self.0;
      let default_value = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_int_QVariant_value_to_output_key_defaultValue(original_self as *const ::map::MapCIntVariant, key as *const ::libc::c_int, default_value as *const ::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapCIntVariant::values](../struct.MapCIntVariant.html#method.values) method.
  pub trait MapCIntVariantValuesArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::list::ListVariant;
  }
  impl<'largs> MapCIntVariantValuesArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::list::ListVariant {
      let key = self;
      {
        let mut object: ::list::ListVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_int_QVariant_values_to_output_key(original_self as *const ::map::MapCIntVariant,
                                                                  key as *const ::libc::c_int,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MapCIntVariantValuesArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::map::MapCIntVariant) -> ::list::ListVariant {

      {
        let mut object: ::list::ListVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_int_QVariant_values_to_output_no_args(original_self as *const ::map::MapCIntVariant,
                                                                      &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapStringVariant::count](../struct.MapStringVariant.html#method.count) method.
  pub trait MapStringVariantCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::libc::c_int;
  }
  impl<'largs> MapStringVariantCountArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::libc::c_int {
      let key = self;
      unsafe {
        ::ffi::qt_core_c_QMap_QString_QVariant_count_key(original_self as *const ::map::MapStringVariant,
                                                         key as *const ::string::String)
      }
    }
  }
  impl<'largs> MapStringVariantCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QMap_QString_QVariant_count_no_args(original_self as *const ::map::MapStringVariant) }
    }
  }
  /// This trait represents a set of arguments accepted by [MapStringVariant::key](../struct.MapStringVariant.html#method.key) method.
  pub trait MapStringVariantKeyArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::string::String;
  }
  impl<'largs> MapStringVariantKeyArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::string::String {
      let value = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_QString_QVariant_key_to_output_value(original_self as *const ::map::MapStringVariant,
                                                                     value as *const ::variant::Variant,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MapStringVariantKeyArgs<'largs> for (&'largs ::variant::Variant, &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::string::String {
      let value = self.0;
      let default_key = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_QString_QVariant_key_to_output_value_defaultKey(original_self as *const ::map::MapStringVariant, value as *const ::variant::Variant, default_key as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapStringVariant::keys](../struct.MapStringVariant.html#method.keys) method.
  pub trait MapStringVariantKeysArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::list::ListString;
  }
  impl<'largs> MapStringVariantKeysArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::list::ListString {

      {
        let mut object: ::list::ListString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_QString_QVariant_keys_to_output_no_args(original_self as *const ::map::MapStringVariant, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MapStringVariantKeysArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::list::ListString {
      let value = self;
      {
        let mut object: ::list::ListString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_QString_QVariant_keys_to_output_value(original_self as *const ::map::MapStringVariant, value as *const ::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapStringVariant::new](../struct.MapStringVariant.html#method.new) method.
  pub trait MapStringVariantNewArgs {
    fn exec(self) -> ::map::MapStringVariant;
  }
  impl MapStringVariantNewArgs for () {
    fn exec(self) -> ::map::MapStringVariant {

      {
        let mut object: ::map::MapStringVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_QString_QVariant_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> MapStringVariantNewArgs for &'a ::map::MapStringVariant {
    fn exec(self) -> ::map::MapStringVariant {
      let other = self;
      {
        let mut object: ::map::MapStringVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_QString_QVariant_constructor_other(other as *const ::map::MapStringVariant,
                                                                   &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapStringVariant::value](../struct.MapStringVariant.html#method.value) method.
  pub trait MapStringVariantValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::variant::Variant;
  }
  impl<'largs> MapStringVariantValueArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::variant::Variant {
      let key = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_QString_QVariant_value_to_output_key(original_self as *const ::map::MapStringVariant,
                                                                     key as *const ::string::String,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MapStringVariantValueArgs<'largs> for (&'largs ::string::String, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::variant::Variant {
      let key = self.0;
      let default_value = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_QString_QVariant_value_to_output_key_defaultValue(original_self as *const ::map::MapStringVariant, key as *const ::string::String, default_value as *const ::variant::Variant, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [MapStringVariant::values](../struct.MapStringVariant.html#method.values) method.
  pub trait MapStringVariantValuesArgs<'largs> {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::list::ListVariant;
  }
  impl<'largs> MapStringVariantValuesArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::list::ListVariant {
      let key = self;
      {
        let mut object: ::list::ListVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_QString_QVariant_values_to_output_key(original_self as *const ::map::MapStringVariant, key as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> MapStringVariantValuesArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::map::MapStringVariant) -> ::list::ListVariant {

      {
        let mut object: ::list::ListVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QMap_QString_QVariant_values_to_output_no_args(original_self as *const ::map::MapStringVariant, &mut object);
        }
        object
      }
    }
  }
}
