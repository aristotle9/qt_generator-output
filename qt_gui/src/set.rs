/// C++ type: <span style='color: green;'>```QSet<QByteArray>```</span>
#[repr(C)]
pub struct SetQtCoreByteArray([u8; ::type_sizes::QT_GUI_SET_SET_QT_CORE_BYTE_ARRAY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SetQtCoreByteArray {
  unsafe fn new_uninitialized() -> SetQtCoreByteArray {
    SetQtCoreByteArray(::std::mem::uninitialized())
  }
}

impl SetQtCoreByteArray {
  /// C++ method: <span style='color: green;'>```int QSet<QByteArray>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSet_QByteArray_capacity(self as *const ::set::SetQtCoreByteArray) }
  }

  /// C++ method: <span style='color: green;'>```void QSet<QByteArray>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QSet_QByteArray_clear(self as *mut ::set::SetQtCoreByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray>::contains```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn contains(&self, &::qt_core::byte_array::ByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QSet<QByteArray>::contains(const QByteArray& value) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn contains(&self, &::set::SetQtCoreByteArray) -> bool```<br>
  /// C++ method: <span style='color: green;'>```bool QSet<QByteArray>::contains(const QSet<QByteArray>& set) const```</span>
  ///
  ///
  pub fn contains<'largs, Args>(&'largs self, args: Args) -> bool
    where Args: overloading::SetQtCoreByteArrayContainsArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QSet<QByteArray>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSet_QByteArray_count(self as *const ::set::SetQtCoreByteArray) }
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QByteArray>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QSet_QByteArray_empty(self as *const ::set::SetQtCoreByteArray) }
  }

  /// C++ method: <span style='color: green;'>```static QSet<QByteArray> QSet<QByteArray>::fromList(const QList<QByteArray>& list)```</span>
  ///
  ///
  pub fn from_list(list: &::qt_core::list::ListByteArray) -> ::set::SetQtCoreByteArray {
    {
      let mut object: ::set::SetQtCoreByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSet_QByteArray_fromList_to_output(list as *const ::qt_core::list::ListByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::intersect(const QSet<QByteArray>& other)```</span>
  ///
  ///
  pub fn intersect<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::set::SetQtCoreByteArray)
                             -> &'l0 mut ::set::SetQtCoreByteArray {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QSet_QByteArray_intersect(self as *mut ::set::SetQtCoreByteArray,
                                                other as *const ::set::SetQtCoreByteArray)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QByteArray>::intersects(const QSet<QByteArray>& other) const```</span>
  ///
  ///
  pub fn intersects(&self, other: &::set::SetQtCoreByteArray) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QSet_QByteArray_intersects(self as *const ::set::SetQtCoreByteArray,
                                                 other as *const ::set::SetQtCoreByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QByteArray>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QSet_QByteArray_isEmpty(self as *const ::set::SetQtCoreByteArray) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSet<QByteArray>::QSet()```</span>
  ///
  ///
  pub fn new() -> ::set::SetQtCoreByteArray {
    {
      let mut object: ::set::SetQtCoreByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSet_QByteArray_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray> QSet<QByteArray>::operator+(const QSet<QByteArray>& other) const```</span>
  ///
  ///
  pub fn op_add(&self, other: &::set::SetQtCoreByteArray) -> ::set::SetQtCoreByteArray {
    {
      let mut object: ::set::SetQtCoreByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSet_QByteArray_operator_add_to_output(self as *const ::set::SetQtCoreByteArray,
                                                               other as *const ::set::SetQtCoreByteArray,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt_core::byte_array::ByteArray) -> &'l0 mut ::set::SetQtCoreByteArray```<br>
  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::operator+=(const QByteArray& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::set::SetQtCoreByteArray) -> &'l0 mut ::set::SetQtCoreByteArray```<br>
  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::operator+=(const QSet<QByteArray>& other)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::set::SetQtCoreByteArray
    where Args: overloading::SetQtCoreByteArrayOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSet<QByteArray> QSet<QByteArray>::operator&(const QSet<QByteArray>& other) const```</span>
  ///
  ///
  pub fn op_bit_and(&self, other: &::set::SetQtCoreByteArray) -> ::set::SetQtCoreByteArray {
    {
      let mut object: ::set::SetQtCoreByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSet_QByteArray_operator_bit_and_to_output(self as *const ::set::SetQtCoreByteArray,
                                                                   other as *const ::set::SetQtCoreByteArray,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray>::operator&=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_bit_and_assign(&mut self, &'l1 ::qt_core::byte_array::ByteArray) -> &'l0 mut ::set::SetQtCoreByteArray```<br>
  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::operator&=(const QByteArray& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_bit_and_assign(&mut self, &'l1 ::set::SetQtCoreByteArray) -> &'l0 mut ::set::SetQtCoreByteArray```<br>
  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::operator&=(const QSet<QByteArray>& other)```</span>
  ///
  ///
  pub fn op_bit_and_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::set::SetQtCoreByteArray
    where Args: overloading::SetQtCoreByteArrayOpBitAndAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSet<QByteArray> QSet<QByteArray>::operator|(const QSet<QByteArray>& other) const```</span>
  ///
  ///
  pub fn op_bit_or(&self, other: &::set::SetQtCoreByteArray) -> ::set::SetQtCoreByteArray {
    {
      let mut object: ::set::SetQtCoreByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSet_QByteArray_operator_bit_or_to_output(self as *const ::set::SetQtCoreByteArray,
                                                                  other as *const ::set::SetQtCoreByteArray,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray>::operator|=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_bit_or_assign(&mut self, &'l1 ::qt_core::byte_array::ByteArray) -> &'l0 mut ::set::SetQtCoreByteArray```<br>
  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::operator|=(const QByteArray& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_bit_or_assign(&mut self, &'l1 ::set::SetQtCoreByteArray) -> &'l0 mut ::set::SetQtCoreByteArray```<br>
  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::operator|=(const QSet<QByteArray>& other)```</span>
  ///
  ///
  pub fn op_bit_or_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::set::SetQtCoreByteArray
    where Args: overloading::SetQtCoreByteArrayOpBitOrAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QSet<QByteArray>::operator==(const QSet<QByteArray>& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::set::SetQtCoreByteArray) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QSet_QByteArray_operator_eq(self as *const ::set::SetQtCoreByteArray,
                                                  other as *const ::set::SetQtCoreByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QByteArray>::operator!=(const QSet<QByteArray>& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::set::SetQtCoreByteArray) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QSet_QByteArray_operator_neq(self as *const ::set::SetQtCoreByteArray,
                                                   other as *const ::set::SetQtCoreByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::operator<<(const QByteArray& value)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          value: &'l1 ::qt_core::byte_array::ByteArray)
                          -> &'l0 mut ::set::SetQtCoreByteArray {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QSet_QByteArray_operator_shl(self as *mut ::set::SetQtCoreByteArray,
                                                   value as *const ::qt_core::byte_array::ByteArray)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray> QSet<QByteArray>::operator-(const QSet<QByteArray>& other) const```</span>
  ///
  ///
  pub fn op_sub(&self, other: &::set::SetQtCoreByteArray) -> ::set::SetQtCoreByteArray {
    {
      let mut object: ::set::SetQtCoreByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSet_QByteArray_operator_sub_to_output(self as *const ::set::SetQtCoreByteArray,
                                                               other as *const ::set::SetQtCoreByteArray,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray>::operator-=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_sub_assign(&mut self, &'l1 ::qt_core::byte_array::ByteArray) -> &'l0 mut ::set::SetQtCoreByteArray```<br>
  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::operator-=(const QByteArray& value)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_sub_assign(&mut self, &'l1 ::set::SetQtCoreByteArray) -> &'l0 mut ::set::SetQtCoreByteArray```<br>
  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::operator-=(const QSet<QByteArray>& other)```</span>
  ///
  ///
  pub fn op_sub_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::set::SetQtCoreByteArray
    where Args: overloading::SetQtCoreByteArrayOpSubAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QSet<QByteArray>::remove(const QByteArray& value)```</span>
  ///
  ///
  pub fn remove(&mut self, value: &::qt_core::byte_array::ByteArray) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QSet_QByteArray_remove(self as *mut ::set::SetQtCoreByteArray,
                                             value as *const ::qt_core::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QSet<QByteArray>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QSet_QByteArray_reserve(self as *mut ::set::SetQtCoreByteArray, size) }
  }

  /// C++ method: <span style='color: green;'>```int QSet<QByteArray>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QSet_QByteArray_size(self as *const ::set::SetQtCoreByteArray) }
  }

  /// C++ method: <span style='color: green;'>```void QSet<QByteArray>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_gui_c_QSet_QByteArray_squeeze(self as *mut ::set::SetQtCoreByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::subtract(const QSet<QByteArray>& other)```</span>
  ///
  ///
  pub fn subtract<'l0, 'l1>(&'l0 mut self,
                            other: &'l1 ::set::SetQtCoreByteArray)
                            -> &'l0 mut ::set::SetQtCoreByteArray {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QSet_QByteArray_subtract(self as *mut ::set::SetQtCoreByteArray,
                                               other as *const ::set::SetQtCoreByteArray)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QSet<QByteArray>::swap(QSet<QByteArray>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::set::SetQtCoreByteArray) {
    unsafe {
      ::ffi::qt_gui_c_QSet_QByteArray_swap(self as *mut ::set::SetQtCoreByteArray,
                                           other as *mut ::set::SetQtCoreByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray> QSet<QByteArray>::toList() const```</span>
  ///
  ///
  pub fn to_list(&self) -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSet_QByteArray_toList_to_output(self as *const ::set::SetQtCoreByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QByteArray>& QSet<QByteArray>::unite(const QSet<QByteArray>& other)```</span>
  ///
  ///
  pub fn unite<'l0, 'l1>(&'l0 mut self, other: &'l1 ::set::SetQtCoreByteArray) -> &'l0 mut ::set::SetQtCoreByteArray {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QSet_QByteArray_unite(self as *mut ::set::SetQtCoreByteArray,
                                            other as *const ::set::SetQtCoreByteArray)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray> QSet<QByteArray>::values() const```</span>
  ///
  ///
  pub fn values(&self) -> ::qt_core::list::ListByteArray {
    {
      let mut object: ::qt_core::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QSet_QByteArray_values_to_output(self as *const ::set::SetQtCoreByteArray, &mut object);
      }
      object
    }
  }
}

impl Drop for ::set::SetQtCoreByteArray {
  /// C++ method: <span style='color: green;'>```[destructor] void QSet<QByteArray>::~QSet()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QSet_QByteArray_destructor(self as *mut ::set::SetQtCoreByteArray) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [SetQtCoreByteArray::contains](../struct.SetQtCoreByteArray.html#method.contains) method.
  pub trait SetQtCoreByteArrayContainsArgs<'largs> {
    fn exec(self, original_self: &'largs ::set::SetQtCoreByteArray) -> bool;
  }
  impl<'largs> SetQtCoreByteArrayContainsArgs<'largs> for &'largs ::set::SetQtCoreByteArray {
    fn exec(self, original_self: &'largs ::set::SetQtCoreByteArray) -> bool {
      let set = self;
      unsafe {
        ::ffi::qt_gui_c_QSet_QByteArray_contains_set(original_self as *const ::set::SetQtCoreByteArray,
                                                     set as *const ::set::SetQtCoreByteArray)
      }
    }
  }
  impl<'largs> SetQtCoreByteArrayContainsArgs<'largs> for &'largs ::qt_core::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::set::SetQtCoreByteArray) -> bool {
      let value = self;
      unsafe {
        ::ffi::qt_gui_c_QSet_QByteArray_contains_value(original_self as *const ::set::SetQtCoreByteArray,
                                                       value as *const ::qt_core::byte_array::ByteArray)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [SetQtCoreByteArray::op_add_assign](../struct.SetQtCoreByteArray.html#method.op_add_assign) method.
  pub trait SetQtCoreByteArrayOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray;
  }
  impl<'largs> SetQtCoreByteArrayOpAddAssignArgs<'largs> for &'largs ::set::SetQtCoreByteArray {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QSet_QByteArray_operator_add_assign_other(original_self as *mut ::set::SetQtCoreByteArray,
                                                                    other as *const ::set::SetQtCoreByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> SetQtCoreByteArrayOpAddAssignArgs<'largs> for &'largs ::qt_core::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray {
      let value = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QSet_QByteArray_operator_add_assign_value(original_self as *mut ::set::SetQtCoreByteArray,
                                                                    value as *const ::qt_core::byte_array::ByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [SetQtCoreByteArray::op_bit_and_assign](../struct.SetQtCoreByteArray.html#method.op_bit_and_assign) method.
  pub trait SetQtCoreByteArrayOpBitAndAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray;
  }
  impl<'largs> SetQtCoreByteArrayOpBitAndAssignArgs<'largs> for &'largs ::set::SetQtCoreByteArray {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QSet_QByteArray_operator_bit_and_assign_other(original_self as *mut ::set::SetQtCoreByteArray, other as *const ::set::SetQtCoreByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> SetQtCoreByteArrayOpBitAndAssignArgs<'largs> for &'largs ::qt_core::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray {
      let value = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QSet_QByteArray_operator_bit_and_assign_value(original_self as *mut ::set::SetQtCoreByteArray, value as *const ::qt_core::byte_array::ByteArray) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [SetQtCoreByteArray::op_bit_or_assign](../struct.SetQtCoreByteArray.html#method.op_bit_or_assign) method.
  pub trait SetQtCoreByteArrayOpBitOrAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray;
  }
  impl<'largs> SetQtCoreByteArrayOpBitOrAssignArgs<'largs> for &'largs ::set::SetQtCoreByteArray {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QSet_QByteArray_operator_bit_or_assign_other(original_self as *mut ::set::SetQtCoreByteArray, other as *const ::set::SetQtCoreByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> SetQtCoreByteArrayOpBitOrAssignArgs<'largs> for &'largs ::qt_core::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray {
      let value = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QSet_QByteArray_operator_bit_or_assign_value(original_self as *mut ::set::SetQtCoreByteArray, value as *const ::qt_core::byte_array::ByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [SetQtCoreByteArray::op_sub_assign](../struct.SetQtCoreByteArray.html#method.op_sub_assign) method.
  pub trait SetQtCoreByteArrayOpSubAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray;
  }
  impl<'largs> SetQtCoreByteArrayOpSubAssignArgs<'largs> for &'largs ::set::SetQtCoreByteArray {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray {
      let other = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QSet_QByteArray_operator_sub_assign_other(original_self as *mut ::set::SetQtCoreByteArray,
                                                                    other as *const ::set::SetQtCoreByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> SetQtCoreByteArrayOpSubAssignArgs<'largs> for &'largs ::qt_core::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::set::SetQtCoreByteArray) -> &'largs mut ::set::SetQtCoreByteArray {
      let value = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QSet_QByteArray_operator_sub_assign_value(original_self as *mut ::set::SetQtCoreByteArray,
                                                                    value as *const ::qt_core::byte_array::ByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
}
