/// C++ type: <span style='color: green;'>```QSequentialIterable::const_iterator```</span>
#[repr(C)]
pub struct ConstIterator([u8; ::type_sizes::QT_CORE_SEQUENTIAL_ITERABLE_CONST_ITERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ConstIterator {
  unsafe fn new_uninitialized() -> ConstIterator {
    ConstIterator(::std::mem::uninitialized())
  }
}

impl ConstIterator {
  /// C++ method: <span style='color: green;'>```[constructor] void QSequentialIterable::const_iterator::const_iterator(const QSequentialIterable::const_iterator& other)```</span>
  ///
  ///
  pub fn new(other: &::sequential_iterable::ConstIterator) -> ::sequential_iterable::ConstIterator {
    {
      let mut object: ::sequential_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSequentialIterable_const_iterator_constructor(other as *const ::sequential_iterable::ConstIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSequentialIterable::const_iterator QSequentialIterable::const_iterator::operator+(int j) const```</span>
  ///
  ///
  pub fn op_add(&self, j: ::libc::c_int) -> ::sequential_iterable::ConstIterator {
    {
      let mut object: ::sequential_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_add_to_output(self as *const ::sequential_iterable::ConstIterator, j, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSequentialIterable::const_iterator& QSequentialIterable::const_iterator::operator+=(int j)```</span>
  ///
  ///
  pub fn op_add_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::sequential_iterable::ConstIterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_add_assign(self as *mut ::sequential_iterable::ConstIterator, j) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSequentialIterable::const_iterator& QSequentialIterable::const_iterator::operator=(const QSequentialIterable::const_iterator& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::sequential_iterable::ConstIterator)
                             -> &'l0 mut ::sequential_iterable::ConstIterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_assign(self as *mut ::sequential_iterable::ConstIterator, other as *const ::sequential_iterable::ConstIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSequentialIterable::const_iterator& QSequentialIterable::const_iterator::operator--()```</span>
  ///
  ///
  pub fn op_dec<'l0>(&'l0 mut self) -> &'l0 mut ::sequential_iterable::ConstIterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_dec(self as *mut ::sequential_iterable::ConstIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSequentialIterable::const_iterator QSequentialIterable::const_iterator::operator--(int arg1)```</span>
  ///
  ///
  pub fn op_dec_postfix(&mut self, arg1: ::libc::c_int) -> ::sequential_iterable::ConstIterator {
    {
      let mut object: ::sequential_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_dec_postfix_to_output(self as *mut ::sequential_iterable::ConstIterator, arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSequentialIterable::const_iterator::operator==(const QSequentialIterable::const_iterator& o) const```</span>
  ///
  ///
  pub fn op_eq(&self, o: &::sequential_iterable::ConstIterator) -> bool {
    unsafe { ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_eq(self as *const ::sequential_iterable::ConstIterator, o as *const ::sequential_iterable::ConstIterator) }
  }

  /// C++ method: <span style='color: green;'>```QSequentialIterable::const_iterator& QSequentialIterable::const_iterator::operator++()```</span>
  ///
  ///
  pub fn op_inc<'l0>(&'l0 mut self) -> &'l0 mut ::sequential_iterable::ConstIterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_inc(self as *mut ::sequential_iterable::ConstIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSequentialIterable::const_iterator QSequentialIterable::const_iterator::operator++(int arg1)```</span>
  ///
  ///
  pub fn op_inc_postfix(&mut self, arg1: ::libc::c_int) -> ::sequential_iterable::ConstIterator {
    {
      let mut object: ::sequential_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_inc_postfix_to_output(self as *mut ::sequential_iterable::ConstIterator, arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QVariant QSequentialIterable::const_iterator::operator*() const```</span>
  ///
  ///
  pub fn op_indirection(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_indirection_to_output(self as *const ::sequential_iterable::ConstIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSequentialIterable::const_iterator::operator!=(const QSequentialIterable::const_iterator& o) const```</span>
  ///
  ///
  pub fn op_neq(&self, o: &::sequential_iterable::ConstIterator) -> bool {
    unsafe { ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_neq(self as *const ::sequential_iterable::ConstIterator, o as *const ::sequential_iterable::ConstIterator) }
  }

  /// C++ method: <span style='color: green;'>```QSequentialIterable::const_iterator QSequentialIterable::const_iterator::operator-(int j) const```</span>
  ///
  ///
  pub fn op_sub(&self, j: ::libc::c_int) -> ::sequential_iterable::ConstIterator {
    {
      let mut object: ::sequential_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_sub_to_output(self as *const ::sequential_iterable::ConstIterator, j, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSequentialIterable::const_iterator& QSequentialIterable::const_iterator::operator-=(int j)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::sequential_iterable::ConstIterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSequentialIterable_const_iterator_operator_sub_assign(self as *mut ::sequential_iterable::ConstIterator, j) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}

impl Drop for ::sequential_iterable::ConstIterator {
  /// C++ method: <span style='color: green;'>```[destructor] void QSequentialIterable::const_iterator::~const_iterator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QSequentialIterable_const_iterator_destructor(self as *mut ::sequential_iterable::ConstIterator)
    }
  }
}

/// C++ type: <span style='color: green;'>```QSequentialIterable```</span>
#[repr(C)]
pub struct SequentialIterable([u8; ::type_sizes::QT_CORE_SEQUENTIAL_ITERABLE_SEQUENTIAL_ITERABLE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SequentialIterable {
  unsafe fn new_uninitialized() -> SequentialIterable {
    SequentialIterable(::std::mem::uninitialized())
  }
}

impl SequentialIterable {
  /// C++ method: <span style='color: green;'>```QVariant QSequentialIterable::at(int idx) const```</span>
  ///
  ///
  pub fn at(&self, idx: ::libc::c_int) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSequentialIterable_at_to_output(self as *const ::sequential_iterable::SequentialIterable,
                                                          idx,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSequentialIterable::const_iterator QSequentialIterable::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> ::sequential_iterable::ConstIterator {
    {
      let mut object: ::sequential_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSequentialIterable_begin_to_output(self as *const ::sequential_iterable::SequentialIterable, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSequentialIterable::canReverseIterate() const```</span>
  ///
  ///
  pub fn can_reverse_iterate(&self) -> bool {
    unsafe {
      ::ffi::qt_core_c_QSequentialIterable_canReverseIterate(self as *const ::sequential_iterable::SequentialIterable)
    }
  }

  /// C++ method: <span style='color: green;'>```QSequentialIterable::const_iterator QSequentialIterable::end() const```</span>
  ///
  ///
  pub fn end(&self) -> ::sequential_iterable::ConstIterator {
    {
      let mut object: ::sequential_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSequentialIterable_end_to_output(self as *const ::sequential_iterable::SequentialIterable,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QSequentialIterable::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QSequentialIterable_size(self as *const ::sequential_iterable::SequentialIterable) }
  }
}

impl Drop for ::sequential_iterable::SequentialIterable {
  /// C++ method: <span style='color: green;'>```[destructor] void QSequentialIterable::~QSequentialIterable()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QSequentialIterable_destructor(self as *mut ::sequential_iterable::SequentialIterable) }
  }
}
