/// C++ type: <span style='color: green;'>```QJsonArray::const_iterator```</span>
#[repr(C)]
pub struct ConstIterator([u8; ::type_sizes::QT_CORE_JSON_ARRAY_CONST_ITERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ConstIterator {
  unsafe fn new_uninitialized() -> ConstIterator {
    ConstIterator(::std::mem::uninitialized())
  }
}

impl ConstIterator {
  /// C++ method: <span style='color: green;'>```const QJsonArray* QJsonArray::const_iterator::a() const```</span>
  ///
  ///
  pub fn a(&self) -> *const ::json_array::JsonArray {
    unsafe { ::ffi::qt_core_c_QJsonArray_const_iterator_a(self as *const ::json_array::ConstIterator) }
  }

  /// C++ method: <span style='color: green;'>```int QJsonArray::const_iterator::i() const```</span>
  ///
  ///
  pub fn i(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QJsonArray_const_iterator_i(self as *const ::json_array::ConstIterator) }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator::const_iterator```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::json_array::ConstIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonArray::const_iterator::const_iterator()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::json_array::ConstIterator) -> ::json_array::ConstIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonArray::const_iterator::const_iterator(const QJsonArray::const_iterator& o)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(&::json_array::Iterator) -> ::json_array::ConstIterator```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonArray::const_iterator::const_iterator(const QJsonArray::iterator& o)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::json_array::ConstIterator
    where Args: overloading::ConstIteratorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonArray::const_iterator::const_iterator(const QJsonArray* array, int index)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(array: *const ::json_array::JsonArray, index: ::libc::c_int) -> ::json_array::ConstIterator {
    {
      let mut object: ::json_array::ConstIterator =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QJsonArray_const_iterator_constructor_QJsonArray_int(array, index, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator QJsonArray::const_iterator::operator+(int j) const```</span>
  ///
  ///
  pub fn op_add(&self, j: ::libc::c_int) -> ::json_array::ConstIterator {
    {
      let mut object: ::json_array::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_const_iterator_operator_add_to_output(self as *const ::json_array::ConstIterator,
                                                                          j,
                                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator& QJsonArray::const_iterator::operator+=(int j)```</span>
  ///
  ///
  pub fn op_add_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::json_array::ConstIterator {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QJsonArray_const_iterator_operator_add_assign(self as *mut ::json_array::ConstIterator, j)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator& QJsonArray::const_iterator::operator--()```</span>
  ///
  ///
  pub fn op_dec<'l0>(&'l0 mut self) -> &'l0 mut ::json_array::ConstIterator {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QJsonArray_const_iterator_operator_dec(self as *mut ::json_array::ConstIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator QJsonArray::const_iterator::operator--(int arg1)```</span>
  ///
  ///
  pub fn op_dec_postfix(&mut self, arg1: ::libc::c_int) -> ::json_array::ConstIterator {
    {
      let mut object: ::json_array::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_const_iterator_operator_dec_postfix_to_output(self as *mut ::json_array::ConstIterator, arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonArray::const_iterator::operator==(const QJsonArray::const_iterator& o) const```</span>
  ///
  ///
  pub fn op_eq(&self, o: &::json_array::ConstIterator) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_const_iterator_operator_eq(self as *const ::json_array::ConstIterator,
                                                             o as *const ::json_array::ConstIterator)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonArray::const_iterator::operator>=(const QJsonArray::const_iterator& other) const```</span>
  ///
  ///
  pub fn op_ge(&self, other: &::json_array::ConstIterator) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_const_iterator_operator_ge(self as *const ::json_array::ConstIterator,
                                                             other as *const ::json_array::ConstIterator)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonArray::const_iterator::operator>(const QJsonArray::const_iterator& other) const```</span>
  ///
  ///
  pub fn op_gt(&self, other: &::json_array::ConstIterator) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_const_iterator_operator_gt(self as *const ::json_array::ConstIterator,
                                                             other as *const ::json_array::ConstIterator)
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator& QJsonArray::const_iterator::operator++()```</span>
  ///
  ///
  pub fn op_inc<'l0>(&'l0 mut self) -> &'l0 mut ::json_array::ConstIterator {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QJsonArray_const_iterator_operator_inc(self as *mut ::json_array::ConstIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator QJsonArray::const_iterator::operator++(int arg1)```</span>
  ///
  ///
  pub fn op_inc_postfix(&mut self, arg1: ::libc::c_int) -> ::json_array::ConstIterator {
    {
      let mut object: ::json_array::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_const_iterator_operator_inc_postfix_to_output(self as *mut ::json_array::ConstIterator, arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue QJsonArray::const_iterator::operator[](int j) const```</span>
  ///
  ///
  pub fn op_index(&self, j: ::libc::c_int) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_const_iterator_operator_index_to_output(self as *const ::json_array::ConstIterator, j, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue QJsonArray::const_iterator::operator*() const```</span>
  ///
  ///
  pub fn op_indirection(&self) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_const_iterator_operator_indirection_to_output(self as *const ::json_array::ConstIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonArray::const_iterator::operator<=(const QJsonArray::const_iterator& other) const```</span>
  ///
  ///
  pub fn op_le(&self, other: &::json_array::ConstIterator) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_const_iterator_operator_le(self as *const ::json_array::ConstIterator,
                                                             other as *const ::json_array::ConstIterator)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonArray::const_iterator::operator<(const QJsonArray::const_iterator& other) const```</span>
  ///
  ///
  pub fn op_lt(&self, other: &::json_array::ConstIterator) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_const_iterator_operator_lt(self as *const ::json_array::ConstIterator,
                                                             other as *const ::json_array::ConstIterator)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonArray::const_iterator::operator!=(const QJsonArray::const_iterator& o) const```</span>
  ///
  ///
  pub fn op_neq(&self, o: &::json_array::ConstIterator) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_const_iterator_operator_neq(self as *const ::json_array::ConstIterator,
                                                              o as *const ::json_array::ConstIterator)
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator::operator-```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_sub(&self, ::libc::c_int) -> ::json_array::ConstIterator```<br>
  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator QJsonArray::const_iterator::operator-(int j) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_sub(&self, &::json_array::ConstIterator) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QJsonArray::const_iterator::operator-(QJsonArray::const_iterator j) const```</span>
  ///
  ///
  pub fn op_sub<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::ConstIteratorOpSubArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator& QJsonArray::const_iterator::operator-=(int j)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::json_array::ConstIterator {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QJsonArray_const_iterator_operator_sub_assign(self as *mut ::json_array::ConstIterator, j)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::const_iterator::set_a(const QJsonArray* value)```</span>
  ///
  ///
  pub unsafe fn set_a(&mut self, value: *const ::json_array::JsonArray) {
    ::ffi::qt_core_c_QJsonArray_const_iterator_set_a(self as *mut ::json_array::ConstIterator, value)
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::const_iterator::set_i(int value)```</span>
  ///
  ///
  pub fn set_i(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QJsonArray_const_iterator_set_i(self as *mut ::json_array::ConstIterator, value) }
  }
}

impl Drop for ::json_array::ConstIterator {
  /// C++ method: <span style='color: green;'>```[destructor] void QJsonArray::const_iterator::~QJsonArray::const_iterator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonArray_const_iterator_destructor(self as *mut ::json_array::ConstIterator) }
  }
}

/// C++ type: <span style='color: green;'>```QJsonArray::iterator```</span>
#[repr(C)]
pub struct Iterator([u8; ::type_sizes::QT_CORE_JSON_ARRAY_ITERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for Iterator {
  unsafe fn new_uninitialized() -> Iterator {
    Iterator(::std::mem::uninitialized())
  }
}

impl Iterator {
  /// C++ method: <span style='color: green;'>```QJsonArray* QJsonArray::iterator::a() const```</span>
  ///
  ///
  pub fn a(&self) -> *mut ::json_array::JsonArray {
    unsafe { ::ffi::qt_core_c_QJsonArray_iterator_a(self as *const ::json_array::Iterator) }
  }

  /// C++ method: <span style='color: green;'>```int QJsonArray::iterator::i() const```</span>
  ///
  ///
  pub fn i(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QJsonArray_iterator_i(self as *const ::json_array::Iterator) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QJsonArray::iterator::iterator()```</span>
  ///
  ///
  pub fn new() -> ::json_array::Iterator {
    {
      let mut object: ::json_array::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_iterator_constructor_no_args(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QJsonArray::iterator::iterator(QJsonArray* array, int index)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(array: *mut ::json_array::JsonArray, index: ::libc::c_int) -> ::json_array::Iterator {
    {
      let mut object: ::json_array::Iterator = ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_core_c_QJsonArray_iterator_constructor_array_index(array, index, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::iterator QJsonArray::iterator::operator+(int j) const```</span>
  ///
  ///
  pub fn op_add(&self, j: ::libc::c_int) -> ::json_array::Iterator {
    {
      let mut object: ::json_array::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_iterator_operator_add_to_output(self as *const ::json_array::Iterator,
                                                                    j,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::iterator& QJsonArray::iterator::operator+=(int j)```</span>
  ///
  ///
  pub fn op_add_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::json_array::Iterator {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_add_assign(self as *mut ::json_array::Iterator, j) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::iterator& QJsonArray::iterator::operator--()```</span>
  ///
  ///
  pub fn op_dec<'l0>(&'l0 mut self) -> &'l0 mut ::json_array::Iterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_dec(self as *mut ::json_array::Iterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::iterator QJsonArray::iterator::operator--(int arg1)```</span>
  ///
  ///
  pub fn op_dec_postfix(&mut self, arg1: ::libc::c_int) -> ::json_array::Iterator {
    {
      let mut object: ::json_array::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_iterator_operator_dec_postfix_to_output(self as *mut ::json_array::Iterator,
                                                                            arg1,
                                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::iterator::operator==```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_eq(&self, &::json_array::ConstIterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator==(const QJsonArray::const_iterator& o) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_eq(&self, &::json_array::Iterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator==(const QJsonArray::iterator& o) const```</span>
  ///
  ///
  pub fn op_eq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::IteratorOpEqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonArray::iterator::operator>=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_ge(&self, &::json_array::ConstIterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator>=(const QJsonArray::const_iterator& other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_ge(&self, &::json_array::Iterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator>=(const QJsonArray::iterator& other) const```</span>
  ///
  ///
  pub fn op_ge<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::IteratorOpGeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonArray::iterator::operator>```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_gt(&self, &::json_array::ConstIterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator>(const QJsonArray::const_iterator& other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_gt(&self, &::json_array::Iterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator>(const QJsonArray::iterator& other) const```</span>
  ///
  ///
  pub fn op_gt<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::IteratorOpGtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonArray::iterator& QJsonArray::iterator::operator++()```</span>
  ///
  ///
  pub fn op_inc<'l0>(&'l0 mut self) -> &'l0 mut ::json_array::Iterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_inc(self as *mut ::json_array::Iterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::iterator QJsonArray::iterator::operator++(int arg1)```</span>
  ///
  ///
  pub fn op_inc_postfix(&mut self, arg1: ::libc::c_int) -> ::json_array::Iterator {
    {
      let mut object: ::json_array::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_iterator_operator_inc_postfix_to_output(self as *mut ::json_array::Iterator,
                                                                            arg1,
                                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValueRef QJsonArray::iterator::operator[](int j) const```</span>
  ///
  ///
  pub fn op_index(&self, j: ::libc::c_int) -> ::json_value_ref::JsonValueRef {
    {
      let mut object: ::json_value_ref::JsonValueRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_iterator_operator_index_to_output(self as *const ::json_array::Iterator,
                                                                      j,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValueRef QJsonArray::iterator::operator*() const```</span>
  ///
  ///
  pub fn op_indirection(&self) -> ::json_value_ref::JsonValueRef {
    {
      let mut object: ::json_value_ref::JsonValueRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_iterator_operator_indirection_to_output(self as *const ::json_array::Iterator,
                                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::iterator::operator<=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_le(&self, &::json_array::ConstIterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator<=(const QJsonArray::const_iterator& other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_le(&self, &::json_array::Iterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator<=(const QJsonArray::iterator& other) const```</span>
  ///
  ///
  pub fn op_le<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::IteratorOpLeArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonArray::iterator::operator<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_lt(&self, &::json_array::ConstIterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator<(const QJsonArray::const_iterator& other) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_lt(&self, &::json_array::Iterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator<(const QJsonArray::iterator& other) const```</span>
  ///
  ///
  pub fn op_lt<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::IteratorOpLtArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonArray::iterator::operator!=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_neq(&self, &::json_array::ConstIterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator!=(const QJsonArray::const_iterator& o) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_neq(&self, &::json_array::Iterator) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QJsonArray::iterator::operator!=(const QJsonArray::iterator& o) const```</span>
  ///
  ///
  pub fn op_neq<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::IteratorOpNeqArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonArray::iterator::operator-```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_sub(&self, ::libc::c_int) -> ::json_array::Iterator```<br>
  /// C++ method: <span style='color: green;'>```QJsonArray::iterator QJsonArray::iterator::operator-(int j) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_sub(&self, &::json_array::Iterator) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QJsonArray::iterator::operator-(QJsonArray::iterator j) const```</span>
  ///
  ///
  pub fn op_sub<'largs, Args>(&'largs self, args: Args) -> Args::ReturnType
    where Args: overloading::IteratorOpSubArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QJsonArray::iterator& QJsonArray::iterator::operator-=(int j)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::json_array::Iterator {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_sub_assign(self as *mut ::json_array::Iterator, j) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::iterator::set_a(QJsonArray* value)```</span>
  ///
  ///
  pub unsafe fn set_a(&mut self, value: *mut ::json_array::JsonArray) {
    ::ffi::qt_core_c_QJsonArray_iterator_set_a(self as *mut ::json_array::Iterator, value)
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::iterator::set_i(int value)```</span>
  ///
  ///
  pub fn set_i(&mut self, value: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QJsonArray_iterator_set_i(self as *mut ::json_array::Iterator, value) }
  }
}

impl Drop for ::json_array::Iterator {
  /// C++ method: <span style='color: green;'>```[destructor] void QJsonArray::iterator::~QJsonArray::iterator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonArray_iterator_destructor(self as *mut ::json_array::Iterator) }
  }
}

/// C++ type: <span style='color: green;'>```QJsonArray```</span>
#[repr(C)]
pub struct JsonArray([u8; ::type_sizes::QT_CORE_JSON_ARRAY_JSON_ARRAY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for JsonArray {
  unsafe fn new_uninitialized() -> JsonArray {
    JsonArray(::std::mem::uninitialized())
  }
}

impl JsonArray {
  /// C++ method: <span style='color: green;'>```void QJsonArray::append(const QJsonValue& value)```</span>
  ///
  ///
  pub fn append(&mut self, value: &::json_value::JsonValue) {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_append(self as *mut ::json_array::JsonArray,
                                         value as *const ::json_value::JsonValue)
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue QJsonArray::at(int i) const```</span>
  ///
  ///
  pub fn at(&self, i: ::libc::c_int) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_at_to_output(self as *const ::json_array::JsonArray, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator QJsonArray::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> ::json_array::ConstIterator {
    {
      let mut object: ::json_array::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_begin_to_output_const(self as *const ::json_array::JsonArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::iterator QJsonArray::begin()```</span>
  ///
  ///
  pub fn begin_mut(&mut self) -> ::json_array::Iterator {
    {
      let mut object: ::json_array::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_begin_to_output(self as *mut ::json_array::JsonArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator QJsonArray::constBegin() const```</span>
  ///
  ///
  pub fn const_begin(&self) -> ::json_array::ConstIterator {
    {
      let mut object: ::json_array::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_constBegin_to_output(self as *const ::json_array::JsonArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator QJsonArray::constEnd() const```</span>
  ///
  ///
  pub fn const_end(&self) -> ::json_array::ConstIterator {
    {
      let mut object: ::json_array::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_constEnd_to_output(self as *const ::json_array::JsonArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonArray::contains(const QJsonValue& element) const```</span>
  ///
  ///
  pub fn contains(&self, element: &::json_value::JsonValue) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_contains(self as *const ::json_array::JsonArray,
                                           element as *const ::json_value::JsonValue)
    }
  }

  /// C++ method: <span style='color: green;'>```int QJsonArray::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QJsonArray_count(self as *const ::json_array::JsonArray) }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonArray::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonArray_empty(self as *const ::json_array::JsonArray) }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::const_iterator QJsonArray::end() const```</span>
  ///
  ///
  pub fn end(&self) -> ::json_array::ConstIterator {
    {
      let mut object: ::json_array::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_end_to_output_const(self as *const ::json_array::JsonArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::iterator QJsonArray::end()```</span>
  ///
  ///
  pub fn end_mut(&mut self) -> ::json_array::Iterator {
    {
      let mut object: ::json_array::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_end_to_output(self as *mut ::json_array::JsonArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::iterator QJsonArray::erase(QJsonArray::iterator it)```</span>
  ///
  ///
  pub fn erase(&mut self, it: &::json_array::Iterator) -> ::json_array::Iterator {
    {
      let mut object: ::json_array::Iterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_erase_to_output(self as *mut ::json_array::JsonArray,
                                                    it as *const ::json_array::Iterator,
                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue QJsonArray::first() const```</span>
  ///
  ///
  pub fn first(&self) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_first_to_output(self as *const ::json_array::JsonArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QJsonArray QJsonArray::fromStringList(const QStringList& list)```</span>
  ///
  ///
  pub fn from_string_list(list: &::string_list::StringList) -> ::json_array::JsonArray {
    {
      let mut object: ::json_array::JsonArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_fromStringList_to_output(list as *const ::string_list::StringList, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```static QJsonArray QJsonArray::fromVariantList(const QList<QVariant>& list)```</span>
  ///
  ///
  pub fn from_variant_list(list: &::list::ListVariant) -> ::json_array::JsonArray {
    {
      let mut object: ::json_array::JsonArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_fromVariantList_to_output(list as *const ::list::ListVariant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (&::json_array::Iterator, &::json_value::JsonValue)) -> ::json_array::Iterator```<br>
  /// C++ method: <span style='color: green;'>```QJsonArray::iterator QJsonArray::insert(QJsonArray::iterator before, const QJsonValue& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::json_value::JsonValue)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QJsonArray::insert(int i, const QJsonValue& value)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> Args::ReturnType
    where Args: overloading::JsonArrayInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QJsonArray::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QJsonArray_isEmpty(self as *const ::json_array::JsonArray) }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue QJsonArray::last() const```</span>
  ///
  ///
  pub fn last(&self) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_last_to_output(self as *const ::json_array::JsonArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray::QJsonArray```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::json_array::JsonArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonArray::QJsonArray()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::json_array::JsonArray) -> ::json_array::JsonArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QJsonArray::QJsonArray(const QJsonArray& other)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::json_array::JsonArray
    where Args: overloading::JsonArrayNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QJsonArray QJsonArray::operator+(const QJsonValue& v) const```</span>
  ///
  ///
  pub fn op_add(&self, v: &::json_value::JsonValue) -> ::json_array::JsonArray {
    {
      let mut object: ::json_array::JsonArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_operator_add_to_output(self as *const ::json_array::JsonArray,
                                                           v as *const ::json_value::JsonValue,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray& QJsonArray::operator+=(const QJsonValue& v)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::json_value::JsonValue) -> &'l0 mut ::json_array::JsonArray {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QJsonArray_operator_add_assign(self as *mut ::json_array::JsonArray,
                                                      v as *const ::json_value::JsonValue)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QJsonArray& QJsonArray::operator=(const QJsonArray& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, other: &'l1 ::json_array::JsonArray) -> &'l0 mut ::json_array::JsonArray {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QJsonArray_operator_assign(self as *mut ::json_array::JsonArray,
                                                  other as *const ::json_array::JsonArray)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QJsonArray::operator==(const QJsonArray& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::json_array::JsonArray) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_operator_eq(self as *const ::json_array::JsonArray,
                                              other as *const ::json_array::JsonArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue QJsonArray::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index(&self, i: ::libc::c_int) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_operator_index_to_output_const(self as *const ::json_array::JsonArray,
                                                                   i,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonValueRef QJsonArray::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut(&mut self, i: ::libc::c_int) -> ::json_value_ref::JsonValueRef {
    {
      let mut object: ::json_value_ref::JsonValueRef =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_operator_index_to_output(self as *mut ::json_array::JsonArray, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QJsonArray::operator!=(const QJsonArray& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::json_array::JsonArray) -> bool {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_operator_neq(self as *const ::json_array::JsonArray,
                                               other as *const ::json_array::JsonArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QJsonArray& QJsonArray::operator<<(const QJsonValue& v)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, v: &'l1 ::json_value::JsonValue) -> &'l0 mut ::json_array::JsonArray {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QJsonArray_operator_shl(self as *mut ::json_array::JsonArray,
                                               v as *const ::json_value::JsonValue)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonArray_pop_back(self as *mut ::json_array::JsonArray) }
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonArray_pop_front(self as *mut ::json_array::JsonArray) }
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::prepend(const QJsonValue& value)```</span>
  ///
  ///
  pub fn prepend(&mut self, value: &::json_value::JsonValue) {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_prepend(self as *mut ::json_array::JsonArray,
                                          value as *const ::json_value::JsonValue)
    }
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::push_back(const QJsonValue& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::json_value::JsonValue) {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_push_back(self as *mut ::json_array::JsonArray,
                                            t as *const ::json_value::JsonValue)
    }
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::push_front(const QJsonValue& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::json_value::JsonValue) {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_push_front(self as *mut ::json_array::JsonArray,
                                             t as *const ::json_value::JsonValue)
    }
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QJsonArray_removeAt(self as *mut ::json_array::JsonArray, i) }
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonArray_removeFirst(self as *mut ::json_array::JsonArray) }
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonArray_removeLast(self as *mut ::json_array::JsonArray) }
  }

  /// C++ method: <span style='color: green;'>```void QJsonArray::replace(int i, const QJsonValue& value)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, value: &::json_value::JsonValue) {
    unsafe {
      ::ffi::qt_core_c_QJsonArray_replace(self as *mut ::json_array::JsonArray,
                                          i,
                                          value as *const ::json_value::JsonValue)
    }
  }

  /// C++ method: <span style='color: green;'>```int QJsonArray::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QJsonArray_size(self as *const ::json_array::JsonArray) }
  }

  /// C++ method: <span style='color: green;'>```QJsonValue QJsonArray::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::json_value::JsonValue {
    {
      let mut object: ::json_value::JsonValue =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_takeAt_to_output(self as *mut ::json_array::JsonArray, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QVariant> QJsonArray::toVariantList() const```</span>
  ///
  ///
  pub fn to_variant_list(&self) -> ::list::ListVariant {
    {
      let mut object: ::list::ListVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QJsonArray_toVariantList_to_output(self as *const ::json_array::JsonArray, &mut object);
      }
      object
    }
  }
}

impl Drop for ::json_array::JsonArray {
  /// C++ method: <span style='color: green;'>```[destructor] void QJsonArray::~QJsonArray()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QJsonArray_destructor(self as *mut ::json_array::JsonArray) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ConstIterator::new](../struct.ConstIterator.html#method.new) method.
  pub trait ConstIteratorNewArgs {
    fn exec(self) -> ::json_array::ConstIterator;
  }
  impl<'a> ConstIteratorNewArgs for &'a ::json_array::ConstIterator {
    fn exec(self) -> ::json_array::ConstIterator {
      let o = self;
      {
        let mut object: ::json_array::ConstIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonArray_const_iterator_constructor_QJsonArray_const_iterator(o as *const ::json_array::ConstIterator, &mut object);
        }
        object
      }
    }
  }
  impl<'a> ConstIteratorNewArgs for &'a ::json_array::Iterator {
    fn exec(self) -> ::json_array::ConstIterator {
      let o = self;
      {
        let mut object: ::json_array::ConstIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonArray_const_iterator_constructor_QJsonArray_iterator(o as *const ::json_array::Iterator, &mut object);
        }
        object
      }
    }
  }
  impl ConstIteratorNewArgs for () {
    fn exec(self) -> ::json_array::ConstIterator {

      {
        let mut object: ::json_array::ConstIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonArray_const_iterator_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ConstIterator::op_sub](../struct.ConstIterator.html#method.op_sub) method.
  pub trait ConstIteratorOpSubArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::json_array::ConstIterator) -> Self::ReturnType;
  }
  impl<'largs> ConstIteratorOpSubArgs<'largs> for &'largs ::json_array::ConstIterator {
    type ReturnType = ::libc::c_int;
    fn exec(self, original_self: &'largs ::json_array::ConstIterator) -> ::libc::c_int {
      let j = self;
      unsafe {
        ::ffi::qt_core_c_QJsonArray_const_iterator_operator_sub(original_self as *const ::json_array::ConstIterator,
                                                                j as *const ::json_array::ConstIterator)
      }
    }
  }
  impl<'largs> ConstIteratorOpSubArgs<'largs> for ::libc::c_int {
    type ReturnType = ::json_array::ConstIterator;
    fn exec(self, original_self: &'largs ::json_array::ConstIterator) -> ::json_array::ConstIterator {
      let j = self;
      {
        let mut object: ::json_array::ConstIterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonArray_const_iterator_operator_sub_to_output(original_self as *const ::json_array::ConstIterator, j, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [Iterator::op_eq](../struct.Iterator.html#method.op_eq) method.
  pub trait IteratorOpEqArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool;
  }
  impl<'largs> IteratorOpEqArgs<'largs> for &'largs ::json_array::ConstIterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let o = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_eq_QJsonArray_const_iterator(original_self as *const ::json_array::Iterator, o as *const ::json_array::ConstIterator) }
    }
  }
  impl<'largs> IteratorOpEqArgs<'largs> for &'largs ::json_array::Iterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let o = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_eq_QJsonArray_iterator(original_self as *const ::json_array::Iterator, o as *const ::json_array::Iterator) }
    }
  }
  /// This trait represents a set of arguments accepted by [Iterator::op_ge](../struct.Iterator.html#method.op_ge) method.
  pub trait IteratorOpGeArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool;
  }
  impl<'largs> IteratorOpGeArgs<'largs> for &'largs ::json_array::ConstIterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_ge_QJsonArray_const_iterator(original_self as *const ::json_array::Iterator, other as *const ::json_array::ConstIterator) }
    }
  }
  impl<'largs> IteratorOpGeArgs<'largs> for &'largs ::json_array::Iterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_ge_QJsonArray_iterator(original_self as *const ::json_array::Iterator, other as *const ::json_array::Iterator) }
    }
  }
  /// This trait represents a set of arguments accepted by [Iterator::op_gt](../struct.Iterator.html#method.op_gt) method.
  pub trait IteratorOpGtArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool;
  }
  impl<'largs> IteratorOpGtArgs<'largs> for &'largs ::json_array::ConstIterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_gt_QJsonArray_const_iterator(original_self as *const ::json_array::Iterator, other as *const ::json_array::ConstIterator) }
    }
  }
  impl<'largs> IteratorOpGtArgs<'largs> for &'largs ::json_array::Iterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_gt_QJsonArray_iterator(original_self as *const ::json_array::Iterator, other as *const ::json_array::Iterator) }
    }
  }
  /// This trait represents a set of arguments accepted by [Iterator::op_le](../struct.Iterator.html#method.op_le) method.
  pub trait IteratorOpLeArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool;
  }
  impl<'largs> IteratorOpLeArgs<'largs> for &'largs ::json_array::ConstIterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_le_QJsonArray_const_iterator(original_self as *const ::json_array::Iterator, other as *const ::json_array::ConstIterator) }
    }
  }
  impl<'largs> IteratorOpLeArgs<'largs> for &'largs ::json_array::Iterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_le_QJsonArray_iterator(original_self as *const ::json_array::Iterator, other as *const ::json_array::Iterator) }
    }
  }
  /// This trait represents a set of arguments accepted by [Iterator::op_lt](../struct.Iterator.html#method.op_lt) method.
  pub trait IteratorOpLtArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool;
  }
  impl<'largs> IteratorOpLtArgs<'largs> for &'largs ::json_array::ConstIterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_lt_QJsonArray_const_iterator(original_self as *const ::json_array::Iterator, other as *const ::json_array::ConstIterator) }
    }
  }
  impl<'largs> IteratorOpLtArgs<'largs> for &'largs ::json_array::Iterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let other = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_lt_QJsonArray_iterator(original_self as *const ::json_array::Iterator, other as *const ::json_array::Iterator) }
    }
  }
  /// This trait represents a set of arguments accepted by [Iterator::op_neq](../struct.Iterator.html#method.op_neq) method.
  pub trait IteratorOpNeqArgs<'largs> {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool;
  }
  impl<'largs> IteratorOpNeqArgs<'largs> for &'largs ::json_array::ConstIterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let o = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_neq_QJsonArray_const_iterator(original_self as *const ::json_array::Iterator, o as *const ::json_array::ConstIterator) }
    }
  }
  impl<'largs> IteratorOpNeqArgs<'largs> for &'largs ::json_array::Iterator {
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> bool {
      let o = self;
      unsafe { ::ffi::qt_core_c_QJsonArray_iterator_operator_neq_QJsonArray_iterator(original_self as *const ::json_array::Iterator, o as *const ::json_array::Iterator) }
    }
  }
  /// This trait represents a set of arguments accepted by [Iterator::op_sub](../struct.Iterator.html#method.op_sub) method.
  pub trait IteratorOpSubArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> Self::ReturnType;
  }
  impl<'largs> IteratorOpSubArgs<'largs> for &'largs ::json_array::Iterator {
    type ReturnType = ::libc::c_int;
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> ::libc::c_int {
      let j = self;
      unsafe {
        ::ffi::qt_core_c_QJsonArray_iterator_operator_sub(original_self as *const ::json_array::Iterator,
                                                          j as *const ::json_array::Iterator)
      }
    }
  }
  impl<'largs> IteratorOpSubArgs<'largs> for ::libc::c_int {
    type ReturnType = ::json_array::Iterator;
    fn exec(self, original_self: &'largs ::json_array::Iterator) -> ::json_array::Iterator {
      let j = self;
      {
        let mut object: ::json_array::Iterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonArray_iterator_operator_sub_to_output(original_self as *const ::json_array::Iterator,
                                                                      j,
                                                                      &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonArray::insert](../struct.JsonArray.html#method.insert) method.
  pub trait JsonArrayInsertArgs<'largs> {
    type ReturnType;
    fn exec(self, original_self: &'largs mut ::json_array::JsonArray) -> Self::ReturnType;
  }
  impl<'largs> JsonArrayInsertArgs<'largs> for (::libc::c_int, &'largs ::json_value::JsonValue) {
    type ReturnType = ();
    fn exec(self, original_self: &'largs mut ::json_array::JsonArray) -> () {
      let i = self.0;
      let value = self.1;
      unsafe {
        ::ffi::qt_core_c_QJsonArray_insert(original_self as *mut ::json_array::JsonArray,
                                           i,
                                           value as *const ::json_value::JsonValue)
      }
    }
  }
  impl<'largs> JsonArrayInsertArgs<'largs> for (&'largs ::json_array::Iterator, &'largs ::json_value::JsonValue) {
    type ReturnType = ::json_array::Iterator;
    fn exec(self, original_self: &'largs mut ::json_array::JsonArray) -> ::json_array::Iterator {
      let before = self.0;
      let value = self.1;
      {
        let mut object: ::json_array::Iterator =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonArray_insert_to_output(original_self as *mut ::json_array::JsonArray,
                                                       before as *const ::json_array::Iterator,
                                                       value as *const ::json_value::JsonValue,
                                                       &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [JsonArray::new](../struct.JsonArray.html#method.new) method.
  pub trait JsonArrayNewArgs {
    fn exec(self) -> ::json_array::JsonArray;
  }
  impl JsonArrayNewArgs for () {
    fn exec(self) -> ::json_array::JsonArray {

      {
        let mut object: ::json_array::JsonArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonArray_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl<'a> JsonArrayNewArgs for &'a ::json_array::JsonArray {
    fn exec(self) -> ::json_array::JsonArray {
      let other = self;
      {
        let mut object: ::json_array::JsonArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QJsonArray_constructor_other(other as *const ::json_array::JsonArray, &mut object);
        }
        object
      }
    }
  }
}
