/// C++ type: <span style='color: green;'>```QSet<QAbstractState*>```</span>
#[repr(C)]
pub struct SetAbstractStateMutPtr([u8; ::type_sizes::QT_CORE_SET_SET_ABSTRACT_STATE_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for SetAbstractStateMutPtr {
  unsafe fn new_uninitialized() -> SetAbstractStateMutPtr {
    SetAbstractStateMutPtr(::std::mem::uninitialized())
  }
}

impl SetAbstractStateMutPtr {
  /// C++ method: <span style='color: green;'>```int QSet<QAbstractState*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_capacity(self as *const ::set::SetAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QSet<QAbstractState*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_clear(self as *mut ::set::SetAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QAbstractState*>::contains(const QSet<QAbstractState*>& set) const```</span>
  ///
  ///
  pub fn contains(&self, set: &::set::SetAbstractStateMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QSet_QAbstractState_ptr_contains_set(self as *const ::set::SetAbstractStateMutPtr,
                                                            set as *const ::set::SetAbstractStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QAbstractState*>::contains(QAbstractState* const & value) const```</span>
  ///
  ///
  pub unsafe fn contains_unsafe(&self, value: &*mut ::abstract_state::AbstractState) -> bool {
    ::ffi::qt_core_c_QSet_QAbstractState_ptr_contains_value(self as *const ::set::SetAbstractStateMutPtr,
                                                            value as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```int QSet<QAbstractState*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_count(self as *const ::set::SetAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QAbstractState*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_empty(self as *const ::set::SetAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```static QSet<QAbstractState*> QSet<QAbstractState*>::fromList(const QList<QAbstractState*>& list)```</span>
  ///
  ///
  pub fn from_list(list: &::list::ListAbstractStateMutPtr) -> ::set::SetAbstractStateMutPtr {
    {
      let mut object: ::set::SetAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSet_QAbstractState_ptr_fromList_to_output(list as *const ::list::ListAbstractStateMutPtr,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::intersect(const QSet<QAbstractState*>& other)```</span>
  ///
  ///
  pub fn intersect<'l0, 'l1>(&'l0 mut self,
                             other: &'l1 ::set::SetAbstractStateMutPtr)
                             -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QSet_QAbstractState_ptr_intersect(self as *mut ::set::SetAbstractStateMutPtr,
                                                         other as *const ::set::SetAbstractStateMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QAbstractState*>::intersects(const QSet<QAbstractState*>& other) const```</span>
  ///
  ///
  pub fn intersects(&self, other: &::set::SetAbstractStateMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QSet_QAbstractState_ptr_intersects(self as *const ::set::SetAbstractStateMutPtr,
                                                          other as *const ::set::SetAbstractStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QAbstractState*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_isEmpty(self as *const ::set::SetAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```[constructor] void QSet<QAbstractState*>::QSet()```</span>
  ///
  ///
  pub fn new() -> ::set::SetAbstractStateMutPtr {
    {
      let mut object: ::set::SetAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSet_QAbstractState_ptr_constructor(&mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*> QSet<QAbstractState*>::operator+(const QSet<QAbstractState*>& other) const```</span>
  ///
  ///
  pub fn op_add(&self, other: &::set::SetAbstractStateMutPtr) -> ::set::SetAbstractStateMutPtr {
    {
      let mut object: ::set::SetAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_add_to_output(self as *const ::set::SetAbstractStateMutPtr, other as *const ::set::SetAbstractStateMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::operator+=(const QSet<QAbstractState*>& other)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 other: &'l1 ::set::SetAbstractStateMutPtr)
                                 -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_add_assign_other(self as *mut ::set::SetAbstractStateMutPtr, other as *const ::set::SetAbstractStateMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::operator+=(QAbstractState* const & value)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               value: &'l1 *mut ::abstract_state::AbstractState)
                                               -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result = ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_add_assign_value(self as *mut ::set::SetAbstractStateMutPtr, value as *const *mut ::abstract_state::AbstractState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*> QSet<QAbstractState*>::operator&(const QSet<QAbstractState*>& other) const```</span>
  ///
  ///
  pub fn op_bit_and(&self, other: &::set::SetAbstractStateMutPtr) -> ::set::SetAbstractStateMutPtr {
    {
      let mut object: ::set::SetAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_bit_and_to_output(self as *const ::set::SetAbstractStateMutPtr, other as *const ::set::SetAbstractStateMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::operator&=(const QSet<QAbstractState*>& other)```</span>
  ///
  ///
  pub fn op_bit_and_assign<'l0, 'l1>(&'l0 mut self,
                                     other: &'l1 ::set::SetAbstractStateMutPtr)
                                     -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_bit_and_assign_other(self as *mut ::set::SetAbstractStateMutPtr, other as *const ::set::SetAbstractStateMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::operator&=(QAbstractState* const & value)```</span>
  ///
  ///
  pub unsafe fn op_bit_and_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                                   value: &'l1 *mut ::abstract_state::AbstractState)
                                                   -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result = ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_bit_and_assign_value(self as *mut ::set::SetAbstractStateMutPtr, value as *const *mut ::abstract_state::AbstractState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*> QSet<QAbstractState*>::operator|(const QSet<QAbstractState*>& other) const```</span>
  ///
  ///
  pub fn op_bit_or(&self, other: &::set::SetAbstractStateMutPtr) -> ::set::SetAbstractStateMutPtr {
    {
      let mut object: ::set::SetAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_bit_or_to_output(self as *const ::set::SetAbstractStateMutPtr, other as *const ::set::SetAbstractStateMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::operator|=(const QSet<QAbstractState*>& other)```</span>
  ///
  ///
  pub fn op_bit_or_assign<'l0, 'l1>(&'l0 mut self,
                                    other: &'l1 ::set::SetAbstractStateMutPtr)
                                    -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_bit_or_assign_other(self as *mut ::set::SetAbstractStateMutPtr, other as *const ::set::SetAbstractStateMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::operator|=(QAbstractState* const & value)```</span>
  ///
  ///
  pub unsafe fn op_bit_or_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                                  value: &'l1 *mut ::abstract_state::AbstractState)
                                                  -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result = ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_bit_or_assign_value(self as *mut ::set::SetAbstractStateMutPtr, value as *const *mut ::abstract_state::AbstractState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QAbstractState*>::operator==(const QSet<QAbstractState*>& other) const```</span>
  ///
  ///
  pub fn op_eq(&self, other: &::set::SetAbstractStateMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_eq(self as *const ::set::SetAbstractStateMutPtr,
                                                           other as *const ::set::SetAbstractStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QAbstractState*>::operator!=(const QSet<QAbstractState*>& other) const```</span>
  ///
  ///
  pub fn op_neq(&self, other: &::set::SetAbstractStateMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_neq(self as *const ::set::SetAbstractStateMutPtr,
                                                            other as *const ::set::SetAbstractStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::operator<<(QAbstractState* const & value)```</span>
  ///
  ///
  pub unsafe fn op_shl<'l0, 'l1>(&'l0 mut self,
                                 value: &'l1 *mut ::abstract_state::AbstractState)
                                 -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result =
      ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_shl(self as *mut ::set::SetAbstractStateMutPtr,
                                                            value as *const *mut ::abstract_state::AbstractState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*> QSet<QAbstractState*>::operator-(const QSet<QAbstractState*>& other) const```</span>
  ///
  ///
  pub fn op_sub(&self, other: &::set::SetAbstractStateMutPtr) -> ::set::SetAbstractStateMutPtr {
    {
      let mut object: ::set::SetAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_sub_to_output(self as *const ::set::SetAbstractStateMutPtr, other as *const ::set::SetAbstractStateMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::operator-=(const QSet<QAbstractState*>& other)```</span>
  ///
  ///
  pub fn op_sub_assign<'l0, 'l1>(&'l0 mut self,
                                 other: &'l1 ::set::SetAbstractStateMutPtr)
                                 -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result = unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_sub_assign_other(self as *mut ::set::SetAbstractStateMutPtr, other as *const ::set::SetAbstractStateMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::operator-=(QAbstractState* const & value)```</span>
  ///
  ///
  pub unsafe fn op_sub_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               value: &'l1 *mut ::abstract_state::AbstractState)
                                               -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result = ::ffi::qt_core_c_QSet_QAbstractState_ptr_operator_sub_assign_value(self as *mut ::set::SetAbstractStateMutPtr, value as *const *mut ::abstract_state::AbstractState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QSet<QAbstractState*>::remove(QAbstractState* const & value)```</span>
  ///
  ///
  pub unsafe fn remove(&mut self, value: &*mut ::abstract_state::AbstractState) -> bool {
    ::ffi::qt_core_c_QSet_QAbstractState_ptr_remove(self as *mut ::set::SetAbstractStateMutPtr,
                                                    value as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```void QSet<QAbstractState*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_reserve(self as *mut ::set::SetAbstractStateMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QSet<QAbstractState*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_size(self as *const ::set::SetAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QSet<QAbstractState*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_squeeze(self as *mut ::set::SetAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::subtract(const QSet<QAbstractState*>& other)```</span>
  ///
  ///
  pub fn subtract<'l0, 'l1>(&'l0 mut self,
                            other: &'l1 ::set::SetAbstractStateMutPtr)
                            -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QSet_QAbstractState_ptr_subtract(self as *mut ::set::SetAbstractStateMutPtr,
                                                        other as *const ::set::SetAbstractStateMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QSet<QAbstractState*>::swap(QSet<QAbstractState*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::set::SetAbstractStateMutPtr) {
    unsafe {
      ::ffi::qt_core_c_QSet_QAbstractState_ptr_swap(self as *mut ::set::SetAbstractStateMutPtr,
                                                    other as *mut ::set::SetAbstractStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*> QSet<QAbstractState*>::toList() const```</span>
  ///
  ///
  pub fn to_list(&self) -> ::list::ListAbstractStateMutPtr {
    {
      let mut object: ::list::ListAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSet_QAbstractState_ptr_toList_to_output(self as *const ::set::SetAbstractStateMutPtr,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*>& QSet<QAbstractState*>::unite(const QSet<QAbstractState*>& other)```</span>
  ///
  ///
  pub fn unite<'l0, 'l1>(&'l0 mut self,
                         other: &'l1 ::set::SetAbstractStateMutPtr)
                         -> &'l0 mut ::set::SetAbstractStateMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QSet_QAbstractState_ptr_unite(self as *mut ::set::SetAbstractStateMutPtr,
                                                     other as *const ::set::SetAbstractStateMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*> QSet<QAbstractState*>::values() const```</span>
  ///
  ///
  pub fn values(&self) -> ::list::ListAbstractStateMutPtr {
    {
      let mut object: ::list::ListAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QSet_QAbstractState_ptr_values_to_output(self as *const ::set::SetAbstractStateMutPtr,
                                                                  &mut object);
      }
      object
    }
  }
}

impl Drop for ::set::SetAbstractStateMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QSet<QAbstractState*>::~QSet()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QSet_QAbstractState_ptr_destructor(self as *mut ::set::SetAbstractStateMutPtr) }
  }
}
