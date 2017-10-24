/// C++ type: <span style='color: green;'>```QAssociativeIterable```</span>
#[repr(C)]
pub struct AssociativeIterable([u8; ::type_sizes::QT_CORE_ASSOCIATIVE_ITERABLE_ASSOCIATIVE_ITERABLE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for AssociativeIterable {
  unsafe fn new_uninitialized() -> AssociativeIterable {
    AssociativeIterable(::std::mem::uninitialized())
  }
}

impl AssociativeIterable {
  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator QAssociativeIterable::begin() const```</span>
  ///
  ///
  pub fn begin(&self) -> ::associative_iterable::ConstIterator {
    {
      let mut object: ::associative_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_begin_to_output(self as *const ::associative_iterable::AssociativeIterable, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator QAssociativeIterable::end() const```</span>
  ///
  ///
  pub fn end(&self) -> ::associative_iterable::ConstIterator {
    {
      let mut object: ::associative_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_end_to_output(self as *const ::associative_iterable::AssociativeIterable, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator QAssociativeIterable::find(const QVariant& key) const```</span>
  ///
  ///
  pub fn find(&self, key: &::variant::Variant) -> ::associative_iterable::ConstIterator {
    {
      let mut object: ::associative_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_find_to_output(self as *const ::associative_iterable::AssociativeIterable, key as *const ::variant::Variant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```int QAssociativeIterable::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QAssociativeIterable_size(self as *const ::associative_iterable::AssociativeIterable) }
  }

  /// C++ method: <span style='color: green;'>```QVariant QAssociativeIterable::value(const QVariant& key) const```</span>
  ///
  ///
  pub fn value(&self, key: &::variant::Variant) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_value_to_output(self as *const ::associative_iterable::AssociativeIterable, key as *const ::variant::Variant, &mut object);
      }
      object
    }
  }
}

impl Drop for ::associative_iterable::AssociativeIterable {
  /// C++ method: <span style='color: green;'>```[destructor] void QAssociativeIterable::~QAssociativeIterable()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QAssociativeIterable_destructor(self as *mut ::associative_iterable::AssociativeIterable)
    }
  }
}

/// C++ type: <span style='color: green;'>```QAssociativeIterable::const_iterator```</span>
#[repr(C)]
pub struct ConstIterator([u8; ::type_sizes::QT_CORE_ASSOCIATIVE_ITERABLE_CONST_ITERATOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ConstIterator {
  unsafe fn new_uninitialized() -> ConstIterator {
    ConstIterator(::std::mem::uninitialized())
  }
}

impl ConstIterator {
  /// C++ method: <span style='color: green;'>```const QVariant QAssociativeIterable::const_iterator::key() const```</span>
  ///
  ///
  pub fn key(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_const_iterator_key_to_output(self as *const ::associative_iterable::ConstIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QAssociativeIterable::const_iterator::const_iterator(const QAssociativeIterable::const_iterator& other)```</span>
  ///
  ///
  pub fn new(other: &::associative_iterable::ConstIterator) -> ::associative_iterable::ConstIterator {
    {
      let mut object: ::associative_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_const_iterator_constructor(other as *const ::associative_iterable::ConstIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator QAssociativeIterable::const_iterator::operator+(int j) const```</span>
  ///
  ///
  pub fn op_add(&self, j: ::libc::c_int) -> ::associative_iterable::ConstIterator {
    {
      let mut object: ::associative_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_add_to_output(self as *const ::associative_iterable::ConstIterator, j, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator& QAssociativeIterable::const_iterator::operator+=(int j)```</span>
  ///
  ///
  pub fn op_add_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::associative_iterable::ConstIterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_add_assign(self as *mut ::associative_iterable::ConstIterator, j) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator& QAssociativeIterable::const_iterator::operator=(const QAssociativeIterable::const_iterator& other)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::associative_iterable::ConstIterator)
                             -> &'l0 mut ::associative_iterable::ConstIterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_assign(self as *mut ::associative_iterable::ConstIterator, other as *const ::associative_iterable::ConstIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator& QAssociativeIterable::const_iterator::operator--()```</span>
  ///
  ///
  pub fn op_dec<'l0>(&'l0 mut self) -> &'l0 mut ::associative_iterable::ConstIterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_dec(self as *mut ::associative_iterable::ConstIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator QAssociativeIterable::const_iterator::operator--(int arg1)```</span>
  ///
  ///
  pub fn op_dec_postfix(&mut self, arg1: ::libc::c_int) -> ::associative_iterable::ConstIterator {
    {
      let mut object: ::associative_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_dec_postfix_to_output(self as *mut ::associative_iterable::ConstIterator, arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAssociativeIterable::const_iterator::operator==(const QAssociativeIterable::const_iterator& o) const```</span>
  ///
  ///
  pub fn op_eq(&self, o: &::associative_iterable::ConstIterator) -> bool {
    unsafe { ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_eq(self as *const ::associative_iterable::ConstIterator, o as *const ::associative_iterable::ConstIterator) }
  }

  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator& QAssociativeIterable::const_iterator::operator++()```</span>
  ///
  ///
  pub fn op_inc<'l0>(&'l0 mut self) -> &'l0 mut ::associative_iterable::ConstIterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_inc(self as *mut ::associative_iterable::ConstIterator) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator QAssociativeIterable::const_iterator::operator++(int arg1)```</span>
  ///
  ///
  pub fn op_inc_postfix(&mut self, arg1: ::libc::c_int) -> ::associative_iterable::ConstIterator {
    {
      let mut object: ::associative_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_inc_postfix_to_output(self as *mut ::associative_iterable::ConstIterator, arg1, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QVariant QAssociativeIterable::const_iterator::operator*() const```</span>
  ///
  ///
  pub fn op_indirection(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_indirection_to_output(self as *const ::associative_iterable::ConstIterator, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```bool QAssociativeIterable::const_iterator::operator!=(const QAssociativeIterable::const_iterator& o) const```</span>
  ///
  ///
  pub fn op_neq(&self, o: &::associative_iterable::ConstIterator) -> bool {
    unsafe { ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_neq(self as *const ::associative_iterable::ConstIterator, o as *const ::associative_iterable::ConstIterator) }
  }

  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator QAssociativeIterable::const_iterator::operator-(int j) const```</span>
  ///
  ///
  pub fn op_sub(&self, j: ::libc::c_int) -> ::associative_iterable::ConstIterator {
    {
      let mut object: ::associative_iterable::ConstIterator =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_sub_to_output(self as *const ::associative_iterable::ConstIterator, j, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAssociativeIterable::const_iterator& QAssociativeIterable::const_iterator::operator-=(int j)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0>(&'l0 mut self, j: ::libc::c_int) -> &'l0 mut ::associative_iterable::ConstIterator {
    let ffi_result = unsafe { ::ffi::qt_core_c_QAssociativeIterable_const_iterator_operator_sub_assign(self as *mut ::associative_iterable::ConstIterator, j) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QVariant QAssociativeIterable::const_iterator::value() const```</span>
  ///
  ///
  pub fn value(&self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QAssociativeIterable_const_iterator_value_to_output(self as *const ::associative_iterable::ConstIterator, &mut object);
      }
      object
    }
  }
}

impl Drop for ::associative_iterable::ConstIterator {
  /// C++ method: <span style='color: green;'>```[destructor] void QAssociativeIterable::const_iterator::~const_iterator()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QAssociativeIterable_const_iterator_destructor(self as *mut ::associative_iterable::ConstIterator) }
  }
}
