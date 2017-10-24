/// C++ type: <span style='color: green;'>```QJsonObject::const_iterator```</span>
#[repr(C)]
pub struct ConstIterator([u8; ::type_sizes::QT_CORE_JSON_OBJECT_CONST_ITERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ConstIterator {
  unsafe fn new_uninitialized() -> ConstIterator {
    ConstIterator(::std::mem::uninitialized())
  }
}

impl ConstIterator {
  /// C++ method: <span style='color: green;'>```QString QJsonObject::const_iterator::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_const_iterator_key_to_output(self as *const ::json_object::ConstIterator,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator::const_iterator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::json_object::ConstIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonObject::const_iterator::const_iterator()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::json_object::Iterator) -> ::json_object::ConstIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonObject::const_iterator::const_iterator(const QJsonObject::iterator& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::json_object::ConstIterator
    where Args: overloading::ConstIteratorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonObject::const_iterator::const_iterator(const QJsonObject* obj, int index)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(obj: *const ::json_object::JsonObject,
                           index: ::libc::c_int)
                           -> ::json_object::ConstIterator {
    {
      let mut object: ::json_object::ConstIterator =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QJsonObject_const_iterator_constructor_obj_index(obj, index, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::const_iterator::operator+(int j) const```</span>
  ///
  ///
  pub fn op_add(&self, j: ::libc::c_int) -> ::json_object::ConstIterator {
    {
      let mut object: ::json_object::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_const_iterator_operator_add_to_output(self as *const ::json_object::ConstIterator, j, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator& QJsonObject::const_iterator::operator+=(int j)```</span>
  ///
  ///
  pub fn op_add_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::json_object::ConstIterator {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QJsonObject_const_iterator_operator_add_assign(self as *mut ::json_object::ConstIterator, j)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator& QJsonObject::const_iterator::operator--()```</span>
  ///
  ///
  pub fn op_dec<'l0>(&'l0 mut self) -> &'l0 mut ::json_object::ConstIterator {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QJsonObject_const_iterator_operator_dec(self as *mut ::json_object::ConstIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::const_iterator::operator--(int arg1)```</span>
  ///
  ///
  pub fn op_dec_postfix(&mut self, arg1: ::libc::c_int) -> ::json_object::ConstIterator {
    {
      let mut object: ::json_object::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_const_iterator_operator_dec_postfix_to_output(self as *mut ::json_object::ConstIterator, arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator::operator==```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_eq(&self, &::json_object::ConstIterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonObject::const_iterator::operator==(const QJsonObject::const_iterator& other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_eq(&self, &::json_object::Iterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonObject::const_iterator::operator==(const QJsonObject::iterator& other) const```</span>
  ///
  ///
  pub fn op_eq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::ConstIteratorOpEqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator& QJsonObject::const_iterator::operator++()```</span>
  ///
  ///
  pub fn op_inc<'l0>(&'l0 mut self) -> &'l0 mut ::json_object::ConstIterator {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QJsonObject_const_iterator_operator_inc(self as *mut ::json_object::ConstIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::const_iterator::operator++(int arg1)```</span>
  ///
  ///
  pub fn op_inc_postfix(&mut self, arg1: ::libc::c_int) -> ::json_object::ConstIterator {
    {
      let mut object: ::json_object::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_const_iterator_operator_inc_postfix_to_output(self as *mut ::json_object::ConstIterator, arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue QJsonObject::const_iterator::operator*() const```</span>
  ///
  ///
  pub fn op_indirection(&self) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_const_iterator_operator_indirection_to_output(self as *const ::json_object::ConstIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator::operator!=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_neq(&self, &::json_object::ConstIterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonObject::const_iterator::operator!=(const QJsonObject::const_iterator& other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_neq(&self, &::json_object::Iterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonObject::const_iterator::operator!=(const QJsonObject::iterator& other) const```</span>
  ///
  ///
  pub fn op_neq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::ConstIteratorOpNeqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::const_iterator::operator-(int j) const```</span>
  ///
  ///
  pub fn op_sub(&self, j: ::libc::c_int) -> ::json_object::ConstIterator {
    {
      let mut object: ::json_object::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_const_iterator_operator_sub_to_output(self as *const ::json_object::ConstIterator, j, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator& QJsonObject::const_iterator::operator-=(int j)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::json_object::ConstIterator {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QJsonObject_const_iterator_operator_sub_assign(self as *mut ::json_object::ConstIterator, j)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonValue QJsonObject::const_iterator::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_const_iterator_value_to_output(self as *const ::json_object::ConstIterator,
                                                                    &mut object);
      }
      object
    }
  }
}

impl Drop for ::json_object::ConstIterator {
  /// C++ method: <span style='color: green;'>```[destructor] void QJsonObject::const_iterator::~QJsonObject::const_iterator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonObject_const_iterator_destructor(self as *mut ::json_object::ConstIterator) }
  }
}

/// C++ type: <span style='color: green;'>```QJsonObject::iterator```</span>
#[repr(C)]
pub struct Iterator([u8; ::type_sizes::QT_CORE_JSON_OBJECT_ITERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Iterator {
  unsafe fn new_uninitialized() -> Iterator {
    Iterator(::std::mem::uninitialized())
  }
}

impl Iterator {
  /// C++ method: <span style='color: green;'>```QString QJsonObject::iterator::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_iterator_key_to_output(self as *const ::json_object::Iterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QJsonObject::iterator::iterator()```</span>
  ///
  ///
  pub fn new() -> ::json_object::Iterator {
    {
      let mut object: ::json_object::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_iterator_constructor_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QJsonObject::iterator::iterator(QJsonObject* obj, int index)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(obj: *mut ::json_object::JsonObject, index: ::libc::c_int) -> ::json_object::Iterator {
    {
      let mut object: ::json_object::Iterator = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QJsonObject_iterator_constructor_obj_index(obj, index, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator QJsonObject::iterator::operator+(int j) const```</span>
  ///
  ///
  pub fn op_add(&self, j: ::libc::c_int) -> ::json_object::Iterator {
    {
      let mut object: ::json_object::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_iterator_operator_add_to_output(self as *const ::json_object::Iterator,
                                                                     j,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator& QJsonObject::iterator::operator+=(int j)```</span>
  ///
  ///
  pub fn op_add_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::json_object::Iterator {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QJsonObject_iterator_operator_add_assign(self as *mut ::json_object::Iterator, j) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator& QJsonObject::iterator::operator--()```</span>
  ///
  ///
  pub fn op_dec<'l0>(&'l0 mut self) -> &'l0 mut ::json_object::Iterator {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QJsonObject_iterator_operator_dec(self as *mut ::json_object::Iterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator QJsonObject::iterator::operator--(int arg1)```</span>
  ///
  ///
  pub fn op_dec_postfix(&mut self, arg1: ::libc::c_int) -> ::json_object::Iterator {
    {
      let mut object: ::json_object::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_iterator_operator_dec_postfix_to_output(self as *mut ::json_object::Iterator,
                                                                             arg1,
                                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator::operator==```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_eq(&self, &::json_object::ConstIterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonObject::iterator::operator==(const QJsonObject::const_iterator& other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_eq(&self, &::json_object::Iterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonObject::iterator::operator==(const QJsonObject::iterator& other) const```</span>
  ///
  ///
  pub fn op_eq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::IteratorOpEqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonObject::iterator& QJsonObject::iterator::operator++()```</span>
  ///
  ///
  pub fn op_inc<'l0>(&'l0 mut self) -> &'l0 mut ::json_object::Iterator {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QJsonObject_iterator_operator_inc(self as *mut ::json_object::Iterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator QJsonObject::iterator::operator++(int arg1)```</span>
  ///
  ///
  pub fn op_inc_postfix(&mut self, arg1: ::libc::c_int) -> ::json_object::Iterator {
    {
      let mut object: ::json_object::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_iterator_operator_inc_postfix_to_output(self as *mut ::json_object::Iterator,
                                                                             arg1,
                                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValueRef QJsonObject::iterator::operator*() const```</span>
  ///
  ///
  pub fn op_indirection(&self) -> ::json_value_ref::JsonValueRef {
    {
      let mut object: ::json_value_ref::JsonValueRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_iterator_operator_indirection_to_output(self as *const ::json_object::Iterator,
                                                                             &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator::operator!=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_neq(&self, &::json_object::ConstIterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonObject::iterator::operator!=(const QJsonObject::const_iterator& other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_neq(&self, &::json_object::Iterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonObject::iterator::operator!=(const QJsonObject::iterator& other) const```</span>
  ///
  ///
  pub fn op_neq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::IteratorOpNeqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonObject::iterator QJsonObject::iterator::operator-(int j) const```</span>
  ///
  ///
  pub fn op_sub(&self, j: ::libc::c_int) -> ::json_object::Iterator {
    {
      let mut object: ::json_object::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_iterator_operator_sub_to_output(self as *const ::json_object::Iterator,
                                                                     j,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator& QJsonObject::iterator::operator-=(int j)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::json_object::Iterator {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QJsonObject_iterator_operator_sub_assign(self as *mut ::json_object::Iterator, j) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonValueRef QJsonObject::iterator::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::json_value_ref::JsonValueRef {
    {
      let mut object: ::json_value_ref::JsonValueRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_iterator_value_to_output(self as *const ::json_object::Iterator, &mut object);
      }
      object
    }
  }
}

impl Drop for ::json_object::Iterator {
  /// C++ method: <span style='color: green;'>```[destructor] void QJsonObject::iterator::~QJsonObject::iterator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonObject_iterator_destructor(self as *mut ::json_object::Iterator) }
  }
}

/// C++ type: <span style='color: green;'>```QJsonObject```</span>
#[repr(C)]
pub struct JsonObject([u8; ::type_sizes::QT_CORE_JSON_OBJECT_JSON_OBJECT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for JsonObject {
  unsafe fn new_uninitialized() -> JsonObject {
    JsonObject(::std::mem::uninitialized())
  }
}

impl JsonObject {
  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> ::json_object::ConstIterator {
    {
      let mut object: ::json_object::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_begin_to_output_const(self as *const ::json_object::JsonObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator QJsonObject::begin()```</span>
  ///
  ///
  pub fn begin_mut(&mut self) -> ::json_object::Iterator {
    {
      let mut object: ::json_object::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_begin_to_output(self as *mut ::json_object::JsonObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::constBegin() const```</span>
  ///
  ///
  pub fn const_begin(&self) -> ::json_object::ConstIterator {
    {
      let mut object: ::json_object::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_constBegin_to_output(self as *const ::json_object::JsonObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::constEnd() const```</span>
  ///
  ///
  pub fn const_end(&self) -> ::json_object::ConstIterator {
    {
      let mut object: ::json_object::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_constEnd_to_output(self as *const ::json_object::JsonObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::constFind```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn const_find(&self, &::latin1_string::Latin1String) -> ::json_object::ConstIterator```<br>
  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::constFind(QLatin1String key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn const_find(&self, &::string::String) -> ::json_object::ConstIterator```<br>
  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::constFind(const QString& key) const```</span>
  ///
  ///
  pub fn const_find<'largs, Args>(&'largs self, args: Args) -> ::json_object::ConstIterator
    where Args: overloading::JsonObjectConstFindArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonObject::contains```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn contains(&self, &::latin1_string::Latin1String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonObject::contains(QLatin1String key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn contains(&self, &::string::String) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonObject::contains(const QString& key) const```</span>
  ///
  ///
  pub fn contains<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::JsonObjectContainsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QJsonObject::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QJsonObject_count(self as *const ::json_object::JsonObject) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonObject::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonObject_empty(self as *const ::json_object::JsonObject) }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::end() const```</span>
  ///
  ///
  pub fn end(&self) -> ::json_object::ConstIterator {
    {
      let mut object: ::json_object::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_end_to_output_const(self as *const ::json_object::JsonObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator QJsonObject::end()```</span>
  ///
  ///
  pub fn end_mut(&mut self) -> ::json_object::Iterator {
    {
      let mut object: ::json_object::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_end_to_output(self as *mut ::json_object::JsonObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator QJsonObject::erase(QJsonObject::iterator it)```</span>
  ///
  ///
  pub fn erase(&mut self, it: &::json_object::Iterator) -> ::json_object::Iterator {
    {
      let mut object: ::json_object::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_erase_to_output(self as *mut ::json_object::JsonObject,
                                                     it as *const ::json_object::Iterator,
                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::find```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn find(&self, &::latin1_string::Latin1String) -> ::json_object::ConstIterator```<br>
  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::find(QLatin1String key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn find(&self, &::string::String) -> ::json_object::ConstIterator```<br>
  /// C++ method: <span style='color: green;'>```QJsonObject::const_iterator QJsonObject::find(const QString& key) const```</span>
  ///
  ///
  pub fn find<'largs, Args>(&'largs self, args: Args) -> ::json_object::ConstIterator
    where Args: overloading::JsonObjectFindArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonObject::find```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn find_mut(&mut self, &::latin1_string::Latin1String) -> ::json_object::Iterator```<br>
  /// C++ method: <span style='color: green;'>```QJsonObject::iterator QJsonObject::find(QLatin1String key)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn find_mut(&mut self, &::string::String) -> ::json_object::Iterator```<br>
  /// C++ method: <span style='color: green;'>```QJsonObject::iterator QJsonObject::find(const QString& key)```</span>
  ///
  ///
  pub fn find_mut<'largs, Args>(&'largs mut self, args: Args) -> ::json_object::Iterator
    where Args: overloading::JsonObjectFindMutArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```static QJsonObject QJsonObject::fromVariantHash(const QHash<QString, QVariant>& map)```</span>
  ///
  ///
  pub fn from_variant_hash(map: &::hash::HashStringVariant) -> ::json_object::JsonObject {
    {
      let mut object: ::json_object::JsonObject =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_fromVariantHash_to_output(map as *const ::hash::HashStringVariant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QJsonObject QJsonObject::fromVariantMap(const QMap<QString, QVariant>& map)```</span>
  ///
  ///
  pub fn from_variant_map(map: &::map::MapStringVariant) -> ::json_object::JsonObject {
    {
      let mut object: ::json_object::JsonObject =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_fromVariantMap_to_output(map as *const ::map::MapStringVariant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::iterator QJsonObject::insert(const QString& key, const QJsonValue& value)```</span>
  ///
  ///
  pub fn insert(&mut self, key: &::string::String, value: &::json_value::JsonValue) -> ::json_object::Iterator {
    {
      let mut object: ::json_object::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_insert_to_output(self as *mut ::json_object::JsonObject,
                                                      key as *const ::string::String,
                                                      value as *const ::json_value::JsonValue,
                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonObject::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonObject_isEmpty(self as *const ::json_object::JsonObject) }
  }

  /// C++ method: <span style='color: green;'>```QStringList QJsonObject::keys() const```</span>
  ///
  ///
  pub fn keys(&self) -> ::string_list::StringList {
    {
      let mut object: ::string_list::StringList =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_keys_to_output(self as *const ::json_object::JsonObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QJsonObject::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QJsonObject_length(self as *const ::json_object::JsonObject) }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::QJsonObject```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::json_object::JsonObject```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonObject::QJsonObject()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::json_object::JsonObject) -> ::json_object::JsonObject```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonObject::QJsonObject(const QJsonObject& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::json_object::JsonObject
    where Args: overloading::JsonObjectNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QJsonObject& QJsonObject::operator=(const QJsonObject& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::json_object::JsonObject)
                             -> &'l0 mut ::json_object::JsonObject {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QJsonObject_operator_assign(self as *mut ::json_object::JsonObject,
                                                   other as *const ::json_object::JsonObject)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QJsonObject::operator==(const QJsonObject& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::json_object::JsonObject) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonObject_operator_eq(self as *const ::json_object::JsonObject,
                                               other as *const ::json_object::JsonObject)
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::operator[]```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_index(&self, &::latin1_string::Latin1String) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```QJsonValue QJsonObject::operator[](QLatin1String key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_index(&self, &::string::String) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```QJsonValue QJsonObject::operator[](const QString& key) const```</span>
  ///
  ///
  pub fn op_index<'largs, Args>(&'largs self, args: Args) -> ::json_value::JsonValue
    where Args: overloading::JsonObjectOpIndexArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonObject::operator[]```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_index_mut(&mut self, &::latin1_string::Latin1String) -> ::json_value_ref::JsonValueRef```<br>
  /// C++ method: <span style='color: green;'>```QJsonValueRef QJsonObject::operator[](QLatin1String key)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_index_mut(&mut self, &::string::String) -> ::json_value_ref::JsonValueRef```<br>
  /// C++ method: <span style='color: green;'>```QJsonValueRef QJsonObject::operator[](const QString& key)```</span>
  ///
  ///
  pub fn op_index_mut<'largs, Args>(&'largs mut self, args: Args) -> ::json_value_ref::JsonValueRef
    where Args: overloading::JsonObjectOpIndexMutArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QJsonObject::operator!=(const QJsonObject& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::json_object::JsonObject) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonObject_operator_neq(self as *const ::json_object::JsonObject,
                                                other as *const ::json_object::JsonObject)
    }
  }

  /// C++ method: <span style='color: green;'>```void QJsonObject::remove(const QString& key)```</span>
  ///
  ///
  pub fn remove(&mut self, key: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QJsonObject_remove(self as *mut ::json_object::JsonObject,
                                          key as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```int QJsonObject::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QJsonObject_size(self as *const ::json_object::JsonObject) }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue QJsonObject::take(const QString& key)```</span>
  ///
  ///
  pub fn take(&mut self, key: &::string::String) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_take_to_output(self as *mut ::json_object::JsonObject,
                                                    key as *const ::string::String,
                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QHash<QString, QVariant> QJsonObject::toVariantHash() const```</span>
  ///
  ///
  pub fn to_variant_hash(&self) -> ::hash::HashStringVariant {
    {
      let mut object: ::hash::HashStringVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_toVariantHash_to_output(self as *const ::json_object::JsonObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMap<QString, QVariant> QJsonObject::toVariantMap() const```</span>
  ///
  ///
  pub fn to_variant_map(&self) -> ::map::MapStringVariant {
    {
      let mut object: ::map::MapStringVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonObject_toVariantMap_to_output(self as *const ::json_object::JsonObject, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonObject::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, &::latin1_string::Latin1String) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```QJsonValue QJsonObject::value(QLatin1String key) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, &::string::String) -> ::json_value::JsonValue```<br>
  /// C++ method: <span style='color: green;'>```QJsonValue QJsonObject::value(const QString& key) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::json_value::JsonValue
    where Args: overloading::JsonObjectValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::json_object::JsonObject {
  /// C++ method: <span style='color: green;'>```[destructor] void QJsonObject::~QJsonObject()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonObject_destructor(self as *mut ::json_object::JsonObject) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ConstIterator::new](../struct.ConstIterator.html#method.new) method.
  pub trait ConstIteratorNewArgs {
    fn exec(self) -> ::json_object::ConstIterator;
  }
  impl ConstIteratorNewArgs for () {
    fn exec(self) -> ::json_object::ConstIterator {

      {
        let mut object: ::json_object::ConstIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_const_iterator_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> ConstIteratorNewArgs for &'a ::json_object::Iterator {
    fn exec(self) -> ::json_object::ConstIterator {
      let other = self;
      {
        let mut object: ::json_object::ConstIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_const_iterator_constructor_other(other as *const ::json_object::Iterator,
                                                                        &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ConstIterator::op_eq](../struct.ConstIterator.html#method.op_eq) method.
  pub trait ConstIteratorOpEqArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_object::ConstIterator) -> bool;
  }
  impl<'largs> ConstIteratorOpEqArgs<'largs> for &'largs ::json_object::ConstIterator {
    fn exec(self, original_self: &'largs ::json_object::ConstIterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonObject_const_iterator_operator_eq_QJsonObject_const_iterator(original_self as *const ::json_object::ConstIterator, other as *const ::json_object::ConstIterator) }
    }
  }
  impl<'largs> ConstIteratorOpEqArgs<'largs> for &'largs ::json_object::Iterator {
    fn exec(self, original_self: &'largs ::json_object::ConstIterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonObject_const_iterator_operator_eq_QJsonObject_iterator(original_self as *const ::json_object::ConstIterator, other as *const ::json_object::Iterator) }
    }
  }
  /// This trait represents a set of arguments accepted by [ConstIterator::op_neq](../struct.ConstIterator.html#method.op_neq) method.
  pub trait ConstIteratorOpNeqArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_object::ConstIterator) -> bool;
  }
  impl<'largs> ConstIteratorOpNeqArgs<'largs> for &'largs ::json_object::ConstIterator {
    fn exec(self, original_self: &'largs ::json_object::ConstIterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonObject_const_iterator_operator_neq_QJsonObject_const_iterator(original_self as *const ::json_object::ConstIterator, other as *const ::json_object::ConstIterator) }
    }
  }
  impl<'largs> ConstIteratorOpNeqArgs<'largs> for &'largs ::json_object::Iterator {
    fn exec(self, original_self: &'largs ::json_object::ConstIterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonObject_const_iterator_operator_neq_QJsonObject_iterator(original_self as *const ::json_object::ConstIterator, other as *const ::json_object::Iterator) }
    }
  }
  /// This trait represents a set of arguments accepted by [Iterator::op_eq](../struct.Iterator.html#method.op_eq) method.
  pub trait IteratorOpEqArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_object::Iterator) -> bool;
  }
  impl<'largs> IteratorOpEqArgs<'largs> for &'largs ::json_object::ConstIterator {
    fn exec(self, original_self: &'largs ::json_object::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonObject_iterator_operator_eq_QJsonObject_const_iterator(original_self as *const ::json_object::Iterator, other as *const ::json_object::ConstIterator) }
    }
  }
  impl<'largs> IteratorOpEqArgs<'largs> for &'largs ::json_object::Iterator {
    fn exec(self, original_self: &'largs ::json_object::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonObject_iterator_operator_eq_QJsonObject_iterator(original_self as *const ::json_object::Iterator, other as *const ::json_object::Iterator) }
    }
  }
  /// This trait represents a set of arguments accepted by [Iterator::op_neq](../struct.Iterator.html#method.op_neq) method.
  pub trait IteratorOpNeqArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_object::Iterator) -> bool;
  }
  impl<'largs> IteratorOpNeqArgs<'largs> for &'largs ::json_object::ConstIterator {
    fn exec(self, original_self: &'largs ::json_object::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonObject_iterator_operator_neq_QJsonObject_const_iterator(original_self as *const ::json_object::Iterator, other as *const ::json_object::ConstIterator) }
    }
  }
  impl<'largs> IteratorOpNeqArgs<'largs> for &'largs ::json_object::Iterator {
    fn exec(self, original_self: &'largs ::json_object::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonObject_iterator_operator_neq_QJsonObject_iterator(original_self as *const ::json_object::Iterator, other as *const ::json_object::Iterator) }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonObject::const_find](../struct.JsonObject.html#method.const_find) method.
  pub trait JsonObjectConstFindArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_object::ConstIterator;
  }
  impl<'largs> JsonObjectConstFindArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_object::ConstIterator {
      let key = self;
      {
        let mut object: ::json_object::ConstIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_constFind_to_output_QLatin1String(original_self as *const ::json_object::JsonObject, key as *const ::latin1_string::Latin1String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> JsonObjectConstFindArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_object::ConstIterator {
      let key = self;
      {
        let mut object: ::json_object::ConstIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_constFind_to_output_QString(original_self as *const ::json_object::JsonObject,
                                                                   key as *const ::string::String,
                                                                   &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonObject::contains](../struct.JsonObject.html#method.contains) method.
  pub trait JsonObjectContainsArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> bool;
  }
  impl<'largs> JsonObjectContainsArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> bool {
      let key = self;
      unsafe {
        ::ffi::qt_core_c_QJsonObject_contains_QLatin1String(original_self as *const ::json_object::JsonObject,
                                                            key as *const ::latin1_string::Latin1String)
      }
    }
  }
  impl<'largs> JsonObjectContainsArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> bool {
      let key = self;
      unsafe {
        ::ffi::qt_core_c_QJsonObject_contains_QString(original_self as *const ::json_object::JsonObject,
                                                      key as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonObject::find](../struct.JsonObject.html#method.find) method.
  pub trait JsonObjectFindArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_object::ConstIterator;
  }
  impl<'largs> JsonObjectFindArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_object::ConstIterator {
      let key = self;
      {
        let mut object: ::json_object::ConstIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_find_to_output_const_QLatin1String(original_self as *const ::json_object::JsonObject, key as *const ::latin1_string::Latin1String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> JsonObjectFindArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_object::ConstIterator {
      let key = self;
      {
        let mut object: ::json_object::ConstIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_find_to_output_const_QString(original_self as *const ::json_object::JsonObject, key as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonObject::find_mut](../struct.JsonObject.html#method.find_mut) method.
  pub trait JsonObjectFindMutArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::json_object::JsonObject) -> ::json_object::Iterator;
  }
  impl<'largs> JsonObjectFindMutArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs mut ::json_object::JsonObject) -> ::json_object::Iterator {
      let key = self;
      {
        let mut object: ::json_object::Iterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_find_to_output_QLatin1String(original_self as *mut ::json_object::JsonObject,
                                                                    key as *const ::latin1_string::Latin1String,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl<'largs> JsonObjectFindMutArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::json_object::JsonObject) -> ::json_object::Iterator {
      let key = self;
      {
        let mut object: ::json_object::Iterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_find_to_output_QString(original_self as *mut ::json_object::JsonObject,
                                                              key as *const ::string::String,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonObject::new](../struct.JsonObject.html#method.new) method.
  pub trait JsonObjectNewArgs {
    fn exec(self) -> ::json_object::JsonObject;
  }
  impl JsonObjectNewArgs for () {
    fn exec(self) -> ::json_object::JsonObject {

      {
        let mut object: ::json_object::JsonObject =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> JsonObjectNewArgs for &'a ::json_object::JsonObject {
    fn exec(self) -> ::json_object::JsonObject {
      let other = self;
      {
        let mut object: ::json_object::JsonObject =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_constructor_other(other as *const ::json_object::JsonObject, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonObject::op_index](../struct.JsonObject.html#method.op_index) method.
  pub trait JsonObjectOpIndexArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_value::JsonValue;
  }
  impl<'largs> JsonObjectOpIndexArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_value::JsonValue {
      let key = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_operator_index_to_output_const_QLatin1String(original_self as *const ::json_object::JsonObject, key as *const ::latin1_string::Latin1String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> JsonObjectOpIndexArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_value::JsonValue {
      let key = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_operator_index_to_output_const_QString(original_self as *const ::json_object::JsonObject, key as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonObject::op_index_mut](../struct.JsonObject.html#method.op_index_mut) method.
  pub trait JsonObjectOpIndexMutArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::json_object::JsonObject) -> ::json_value_ref::JsonValueRef;
  }
  impl<'largs> JsonObjectOpIndexMutArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs mut ::json_object::JsonObject) -> ::json_value_ref::JsonValueRef {
      let key = self;
      {
        let mut object: ::json_value_ref::JsonValueRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_operator_index_to_output_QLatin1String(original_self as *mut ::json_object::JsonObject, key as *const ::latin1_string::Latin1String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> JsonObjectOpIndexMutArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::json_object::JsonObject) -> ::json_value_ref::JsonValueRef {
      let key = self;
      {
        let mut object: ::json_value_ref::JsonValueRef =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_operator_index_to_output_QString(original_self as *mut ::json_object::JsonObject, key as *const ::string::String, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonObject::value](../struct.JsonObject.html#method.value) method.
  pub trait JsonObjectValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_value::JsonValue;
  }
  impl<'largs> JsonObjectValueArgs<'largs> for &'largs ::latin1_string::Latin1String {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_value::JsonValue {
      let key = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_value_to_output_QLatin1String(original_self as *const ::json_object::JsonObject, key as *const ::latin1_string::Latin1String, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> JsonObjectValueArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::json_object::JsonObject) -> ::json_value::JsonValue {
      let key = self;
      {
        let mut object: ::json_value::JsonValue =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonObject_value_to_output_QString(original_self as *const ::json_object::JsonObject,
                                                               key as *const ::string::String,
                                                               &mut object);
        }
        object
      }
    }
  }
}
