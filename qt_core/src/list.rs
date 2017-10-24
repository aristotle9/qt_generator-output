/// C++ type: <span style='color: green;'>```QList<QAbstractAnimation*>```</span>
#[repr(C)]
pub struct ListAbstractAnimationMutPtr([u8; ::type_sizes::QT_CORE_LIST_LIST_ABSTRACT_ANIMATION_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListAbstractAnimationMutPtr {
  unsafe fn new_uninitialized() -> ListAbstractAnimationMutPtr {
    ListAbstractAnimationMutPtr(::std::mem::uninitialized())
  }
}

impl ListAbstractAnimationMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::append(const QList<QAbstractAnimation*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListAbstractAnimationMutPtr) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_append_QList_QAbstractAnimation_ptr(self as *mut ::list::ListAbstractAnimationMutPtr, t as *const ::list::ListAbstractAnimationMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::append(QAbstractAnimation* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_append_QAbstractAnimation(self as *mut ::list::ListAbstractAnimationMutPtr, t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* const & QList<QAbstractAnimation*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_at(self as *const ::list::ListAbstractAnimationMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* const & QList<QAbstractAnimation*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_back_const(self as *const ::list::ListAbstractAnimationMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation*& QList<QAbstractAnimation*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_back(self as *mut ::list::ListAbstractAnimationMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_clear(self as *mut ::list::ListAbstractAnimationMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* const & QList<QAbstractAnimation*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_constFirst(self as *const ::list::ListAbstractAnimationMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* const & QList<QAbstractAnimation*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_constLast(self as *const ::list::ListAbstractAnimationMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractAnimation*>::contains(QAbstractAnimation* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::abstract_animation::AbstractAnimation) -> bool {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_contains(self as *const ::list::ListAbstractAnimationMutPtr,
                                                           t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractAnimation*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_count_no_args(self as *const ::list::ListAbstractAnimationMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractAnimation*>::count(QAbstractAnimation* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::abstract_animation::AbstractAnimation) -> ::libc::c_int {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_count_t(self as *const ::list::ListAbstractAnimationMutPtr,
                                                          t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractAnimation*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_empty(self as *const ::list::ListAbstractAnimationMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractAnimation*>::endsWith(QAbstractAnimation* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::abstract_animation::AbstractAnimation) -> bool {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_endsWith(self as *const ::list::ListAbstractAnimationMutPtr,
                                                           t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* const & QList<QAbstractAnimation*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_first_const(self as *const ::list::ListAbstractAnimationMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation*& QList<QAbstractAnimation*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_first(self as *mut ::list::ListAbstractAnimationMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* const & QList<QAbstractAnimation*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_front_const(self as *const ::list::ListAbstractAnimationMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation*& QList<QAbstractAnimation*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_front(self as *mut ::list::ListAbstractAnimationMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::abstract_animation::AbstractAnimation) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractAnimation*>::indexOf(QAbstractAnimation* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::abstract_animation::AbstractAnimation, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractAnimation*>::indexOf(QAbstractAnimation* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListAbstractAnimationMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::insert(int i, QAbstractAnimation* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_insert(self as *mut ::list::ListAbstractAnimationMutPtr,
                                                         i,
                                                         t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractAnimation*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_isEmpty(self as *const ::list::ListAbstractAnimationMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* const & QList<QAbstractAnimation*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_last_const(self as *const ::list::ListAbstractAnimationMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::abstract_animation::AbstractAnimation) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractAnimation*>::lastIndexOf(QAbstractAnimation* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::abstract_animation::AbstractAnimation, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractAnimation*>::lastIndexOf(QAbstractAnimation* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListAbstractAnimationMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractAnimation*& QList<QAbstractAnimation*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_last(self as *mut ::list::ListAbstractAnimationMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractAnimation*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_length(self as *const ::list::ListAbstractAnimationMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListAbstractAnimationMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*> QList<QAbstractAnimation*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListAbstractAnimationMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*> QList<QAbstractAnimation*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListAbstractAnimationMutPtr
    where Args: overloading::ListAbstractAnimationMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_move(self as *mut ::list::ListAbstractAnimationMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListAbstractAnimationMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAbstractAnimation*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListAbstractAnimationMutPtr) -> ::list::ListAbstractAnimationMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAbstractAnimation*>::QList(const QList<QAbstractAnimation*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListAbstractAnimationMutPtr
    where Args: overloading::ListAbstractAnimationMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*> QList<QAbstractAnimation*>::operator+(const QList<QAbstractAnimation*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListAbstractAnimationMutPtr) -> ::list::ListAbstractAnimationMutPtr {
    {
      let mut object: ::list::ListAbstractAnimationMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_operator_add_to_output(self as *const ::list::ListAbstractAnimationMutPtr, l as *const ::list::ListAbstractAnimationMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*>& QList<QAbstractAnimation*>::operator+=(const QList<QAbstractAnimation*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListAbstractAnimationMutPtr)
                                 -> &'l0 mut ::list::ListAbstractAnimationMutPtr {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_operator_add_assign_l(self as *mut ::list::ListAbstractAnimationMutPtr, l as *const ::list::ListAbstractAnimationMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*>& QList<QAbstractAnimation*>::operator+=(QAbstractAnimation* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::abstract_animation::AbstractAnimation)
                                               -> &'l0 mut ::list::ListAbstractAnimationMutPtr {
    let ffi_result = ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_operator_add_assign_t(self as *mut ::list::ListAbstractAnimationMutPtr, t as *const *mut ::abstract_animation::AbstractAnimation);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*>& QList<QAbstractAnimation*>::operator=(const QList<QAbstractAnimation*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListAbstractAnimationMutPtr)
                             -> &'l0 mut ::list::ListAbstractAnimationMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_operator_assign(self as *mut ::list::ListAbstractAnimationMutPtr, l as *const ::list::ListAbstractAnimationMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractAnimation*>::operator==(const QList<QAbstractAnimation*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListAbstractAnimationMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_operator_eq(self as *const ::list::ListAbstractAnimationMutPtr,
                                                                l as *const ::list::ListAbstractAnimationMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* const & QList<QAbstractAnimation*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_animation::AbstractAnimation {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_operator_index_const(self as *const ::list::ListAbstractAnimationMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation*& QList<QAbstractAnimation*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::abstract_animation::AbstractAnimation {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_operator_index(self as *mut ::list::ListAbstractAnimationMutPtr,
                                                                     i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractAnimation*>::operator!=(const QList<QAbstractAnimation*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListAbstractAnimationMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_operator_neq(self as *const ::list::ListAbstractAnimationMutPtr,
                                                                 l as *const ::list::ListAbstractAnimationMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*>& QList<QAbstractAnimation*>::operator<<(const QList<QAbstractAnimation*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListAbstractAnimationMutPtr)
                          -> &'l0 mut ::list::ListAbstractAnimationMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_operator_shl_l(self as *mut ::list::ListAbstractAnimationMutPtr,
                                                                     l as *const ::list::ListAbstractAnimationMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*>& QList<QAbstractAnimation*>::operator<<(QAbstractAnimation* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::abstract_animation::AbstractAnimation)
                                        -> &'l0 mut ::list::ListAbstractAnimationMutPtr {
    let ffi_result = ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_operator_shl_t(self as *mut ::list::ListAbstractAnimationMutPtr, t as *const *mut ::abstract_animation::AbstractAnimation);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_pop_back(self as *mut ::list::ListAbstractAnimationMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_pop_front(self as *mut ::list::ListAbstractAnimationMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::prepend(QAbstractAnimation* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_prepend(self as *mut ::list::ListAbstractAnimationMutPtr,
                                                          t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::push_back(QAbstractAnimation* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_push_back(self as *mut ::list::ListAbstractAnimationMutPtr,
                                                            t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::push_front(QAbstractAnimation* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_push_front(self as *mut ::list::ListAbstractAnimationMutPtr,
                                                             t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractAnimation*>::removeAll(QAbstractAnimation* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::abstract_animation::AbstractAnimation) -> ::libc::c_int {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_removeAll(self as *mut ::list::ListAbstractAnimationMutPtr,
                                                            t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_removeAt(self as *mut ::list::ListAbstractAnimationMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_removeFirst(self as *mut ::list::ListAbstractAnimationMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_removeLast(self as *mut ::list::ListAbstractAnimationMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractAnimation*>::removeOne(QAbstractAnimation* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::abstract_animation::AbstractAnimation) -> bool {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_removeOne(self as *mut ::list::ListAbstractAnimationMutPtr,
                                                            t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::replace(int i, QAbstractAnimation* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::abstract_animation::AbstractAnimation) {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_replace(self as *mut ::list::ListAbstractAnimationMutPtr,
                                                          i,
                                                          t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_reserve(self as *mut ::list::ListAbstractAnimationMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractAnimation*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_size(self as *const ::list::ListAbstractAnimationMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractAnimation*>::startsWith(QAbstractAnimation* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::abstract_animation::AbstractAnimation) -> bool {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_startsWith(self as *const ::list::ListAbstractAnimationMutPtr,
                                                             t as *const *mut ::abstract_animation::AbstractAnimation)
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractAnimation*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListAbstractAnimationMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::swap(QList<QAbstractAnimation*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractAnimation*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListAbstractAnimationMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractAnimation* QList<QAbstractAnimation*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::abstract_animation::AbstractAnimation {
    unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_takeAt(self as *mut ::list::ListAbstractAnimationMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* QList<QAbstractAnimation*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::abstract_animation::AbstractAnimation {
    unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_takeFirst(self as *mut ::list::ListAbstractAnimationMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* QList<QAbstractAnimation*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::abstract_animation::AbstractAnimation {
    unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_takeLast(self as *mut ::list::ListAbstractAnimationMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* QList<QAbstractAnimation*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::abstract_animation::AbstractAnimation {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_value_i(self as *const ::list::ListAbstractAnimationMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractAnimation* QList<QAbstractAnimation*>::value(int i, QAbstractAnimation* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::abstract_animation::AbstractAnimation)
                             -> *mut ::abstract_animation::AbstractAnimation {
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_value_i_defaultValue(self as *const ::list::ListAbstractAnimationMutPtr, i, default_value as *const *mut ::abstract_animation::AbstractAnimation)
  }
}

impl Drop for ::list::ListAbstractAnimationMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QAbstractAnimation*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_destructor(self as *mut ::list::ListAbstractAnimationMutPtr)
    }
  }
}

/// C++ type: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>```</span>
#[repr(C)]
pub struct ListAbstractEventDispatcherTimerInfo([u8; ::type_sizes::QT_CORE_LIST_LIST_ABSTRACT_EVENT_DISPATCHER_TIMER_INFO]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListAbstractEventDispatcherTimerInfo {
  unsafe fn new_uninitialized() -> ListAbstractEventDispatcherTimerInfo {
    ListAbstractEventDispatcherTimerInfo(::std::mem::uninitialized())
  }
}

impl ListAbstractEventDispatcherTimerInfo {
  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::abstract_event_dispatcher::TimerInfo) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::append(const QAbstractEventDispatcher::TimerInfo& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListAbstractEventDispatcherTimerInfo) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::append(const QList<QAbstractEventDispatcher::TimerInfo>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListAbstractEventDispatcherTimerInfoAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_at(self as *const ::list::ListAbstractEventDispatcherTimerInfo, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_back_const(self as *const ::list::ListAbstractEventDispatcherTimerInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_back(self as *mut ::list::ListAbstractEventDispatcherTimerInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_clear(self as *mut ::list::ListAbstractEventDispatcherTimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```const QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_constFirst(self as *const ::list::ListAbstractEventDispatcherTimerInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_constLast(self as *const ::list::ListAbstractEventDispatcherTimerInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractEventDispatcher::TimerInfo>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_count(self as *const ::list::ListAbstractEventDispatcherTimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractEventDispatcher::TimerInfo>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_empty(self as *const ::list::ListAbstractEventDispatcherTimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```const QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_first_const(self as *const ::list::ListAbstractEventDispatcherTimerInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_first(self as *mut ::list::ListAbstractEventDispatcherTimerInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_front_const(self as *const ::list::ListAbstractEventDispatcherTimerInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_front(self as *mut ::list::ListAbstractEventDispatcherTimerInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::insert(int i, const QAbstractEventDispatcher::TimerInfo& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::abstract_event_dispatcher::TimerInfo) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_insert(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, i, t as *const ::abstract_event_dispatcher::TimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractEventDispatcher::TimerInfo>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_isEmpty(self as *const ::list::ListAbstractEventDispatcherTimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```const QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_last_const(self as *const ::list::ListAbstractEventDispatcherTimerInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_last(self as *mut ::list::ListAbstractEventDispatcherTimerInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractEventDispatcher::TimerInfo>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_length(self as *const ::list::ListAbstractEventDispatcherTimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListAbstractEventDispatcherTimerInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo> QList<QAbstractEventDispatcher::TimerInfo>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListAbstractEventDispatcherTimerInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo> QList<QAbstractEventDispatcher::TimerInfo>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListAbstractEventDispatcherTimerInfo
    where Args: overloading::ListAbstractEventDispatcherTimerInfoMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_move(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListAbstractEventDispatcherTimerInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAbstractEventDispatcher::TimerInfo>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListAbstractEventDispatcherTimerInfo) -> ::list::ListAbstractEventDispatcherTimerInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAbstractEventDispatcher::TimerInfo>::QList(const QList<QAbstractEventDispatcher::TimerInfo>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListAbstractEventDispatcherTimerInfo
    where Args: overloading::ListAbstractEventDispatcherTimerInfoNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo> QList<QAbstractEventDispatcher::TimerInfo>::operator+(const QList<QAbstractEventDispatcher::TimerInfo>& l) const```</span>
  ///
  ///
  pub fn op_add(&self,
                l: &::list::ListAbstractEventDispatcherTimerInfo)
                -> ::list::ListAbstractEventDispatcherTimerInfo {
    {
      let mut object: ::list::ListAbstractEventDispatcherTimerInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_add_to_output(self as *const ::list::ListAbstractEventDispatcherTimerInfo, l as *const ::list::ListAbstractEventDispatcherTimerInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::abstract_event_dispatcher::TimerInfo) -> &'l0 mut ::list::ListAbstractEventDispatcherTimerInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>& QList<QAbstractEventDispatcher::TimerInfo>::operator+=(const QAbstractEventDispatcher::TimerInfo& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListAbstractEventDispatcherTimerInfo) -> &'l0 mut ::list::ListAbstractEventDispatcherTimerInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>& QList<QAbstractEventDispatcher::TimerInfo>::operator+=(const QList<QAbstractEventDispatcher::TimerInfo>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self,
                                     args: Args)
                                     -> &'largs mut ::list::ListAbstractEventDispatcherTimerInfo
    where Args: overloading::ListAbstractEventDispatcherTimerInfoOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>& QList<QAbstractEventDispatcher::TimerInfo>::operator=(const QList<QAbstractEventDispatcher::TimerInfo>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListAbstractEventDispatcherTimerInfo)
                             -> &'l0 mut ::list::ListAbstractEventDispatcherTimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_assign(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, l as *const ::list::ListAbstractEventDispatcherTimerInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_index_const(self as *const ::list::ListAbstractEventDispatcherTimerInfo, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractEventDispatcher::TimerInfo& QList<QAbstractEventDispatcher::TimerInfo>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::abstract_event_dispatcher::TimerInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_index(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::abstract_event_dispatcher::TimerInfo) -> &'l0 mut ::list::ListAbstractEventDispatcherTimerInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>& QList<QAbstractEventDispatcher::TimerInfo>::operator<<(const QAbstractEventDispatcher::TimerInfo& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListAbstractEventDispatcherTimerInfo) -> &'l0 mut ::list::ListAbstractEventDispatcherTimerInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>& QList<QAbstractEventDispatcher::TimerInfo>::operator<<(const QList<QAbstractEventDispatcher::TimerInfo>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListAbstractEventDispatcherTimerInfo
    where Args: overloading::ListAbstractEventDispatcherTimerInfoOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_pop_back(self as *mut ::list::ListAbstractEventDispatcherTimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_pop_front(self as *mut ::list::ListAbstractEventDispatcherTimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::prepend(const QAbstractEventDispatcher::TimerInfo& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::abstract_event_dispatcher::TimerInfo) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_prepend(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, t as *const ::abstract_event_dispatcher::TimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::push_back(const QAbstractEventDispatcher::TimerInfo& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::abstract_event_dispatcher::TimerInfo) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_push_back(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, t as *const ::abstract_event_dispatcher::TimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::push_front(const QAbstractEventDispatcher::TimerInfo& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::abstract_event_dispatcher::TimerInfo) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_push_front(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, t as *const ::abstract_event_dispatcher::TimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_removeAt(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_removeFirst(self as *mut ::list::ListAbstractEventDispatcherTimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_removeLast(self as *mut ::list::ListAbstractEventDispatcherTimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::replace(int i, const QAbstractEventDispatcher::TimerInfo& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::abstract_event_dispatcher::TimerInfo) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_replace(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, i, t as *const ::abstract_event_dispatcher::TimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_reserve(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractEventDispatcher::TimerInfo>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_size(self as *const ::list::ListAbstractEventDispatcherTimerInfo) }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractEventDispatcher::TimerInfo>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListAbstractEventDispatcherTimerInfo) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::swap(QList<QAbstractEventDispatcher::TimerInfo>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractEventDispatcher::TimerInfo>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListAbstractEventDispatcherTimerInfoSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractEventDispatcher::TimerInfo QList<QAbstractEventDispatcher::TimerInfo>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::abstract_event_dispatcher::TimerInfo {
    {
      let mut object: ::abstract_event_dispatcher::TimerInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_takeAt_to_output(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractEventDispatcher::TimerInfo QList<QAbstractEventDispatcher::TimerInfo>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::abstract_event_dispatcher::TimerInfo {
    {
      let mut object: ::abstract_event_dispatcher::TimerInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_takeFirst_to_output(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractEventDispatcher::TimerInfo QList<QAbstractEventDispatcher::TimerInfo>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::abstract_event_dispatcher::TimerInfo {
    {
      let mut object: ::abstract_event_dispatcher::TimerInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_takeLast_to_output(self as *mut ::list::ListAbstractEventDispatcherTimerInfo, &mut object);
      }
      object
    }
  }
}

impl Drop for ::list::ListAbstractEventDispatcherTimerInfo {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QAbstractEventDispatcher::TimerInfo>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_destructor(self as *mut ::list::ListAbstractEventDispatcherTimerInfo) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QAbstractState*>```</span>
#[repr(C)]
pub struct ListAbstractStateMutPtr([u8; ::type_sizes::QT_CORE_LIST_LIST_ABSTRACT_STATE_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListAbstractStateMutPtr {
  unsafe fn new_uninitialized() -> ListAbstractStateMutPtr {
    ListAbstractStateMutPtr(::std::mem::uninitialized())
  }
}

impl ListAbstractStateMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::append(const QList<QAbstractState*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListAbstractStateMutPtr) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_append_QList_QAbstractState_ptr(self as *mut ::list::ListAbstractStateMutPtr, t as *const ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::append(QAbstractState* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_append_QAbstractState(self as *mut ::list::ListAbstractStateMutPtr,
                                                                    t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* const & QList<QAbstractState*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_at(self as *const ::list::ListAbstractStateMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* const & QList<QAbstractState*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_back_const(self as *const ::list::ListAbstractStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractState*& QList<QAbstractState*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_back(self as *mut ::list::ListAbstractStateMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_clear(self as *mut ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* const & QList<QAbstractState*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_constFirst(self as *const ::list::ListAbstractStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* const & QList<QAbstractState*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_constLast(self as *const ::list::ListAbstractStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractState*>::contains(QAbstractState* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::abstract_state::AbstractState) -> bool {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_contains(self as *const ::list::ListAbstractStateMutPtr,
                                                       t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractState*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_count_no_args(self as *const ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractState*>::count(QAbstractState* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::abstract_state::AbstractState) -> ::libc::c_int {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_count_t(self as *const ::list::ListAbstractStateMutPtr,
                                                      t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractState*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_empty(self as *const ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractState*>::endsWith(QAbstractState* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::abstract_state::AbstractState) -> bool {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_endsWith(self as *const ::list::ListAbstractStateMutPtr,
                                                       t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* const & QList<QAbstractState*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_first_const(self as *const ::list::ListAbstractStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractState*& QList<QAbstractState*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_first(self as *mut ::list::ListAbstractStateMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QList<QAbstractState*> QList<QAbstractState*>::fromSet(const QSet<QAbstractState*>& set)```</span>
  ///
  ///
  pub fn from_set(set: &::set::SetAbstractStateMutPtr) -> ::list::ListAbstractStateMutPtr {
    {
      let mut object: ::list::ListAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractState_ptr_fromSet_to_output(set as *const ::set::SetAbstractStateMutPtr,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* const & QList<QAbstractState*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_front_const(self as *const ::list::ListAbstractStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractState*& QList<QAbstractState*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_front(self as *mut ::list::ListAbstractStateMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::abstract_state::AbstractState) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractState*>::indexOf(QAbstractState* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::abstract_state::AbstractState, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractState*>::indexOf(QAbstractState* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListAbstractStateMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::insert(int i, QAbstractState* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_insert(self as *mut ::list::ListAbstractStateMutPtr,
                                                     i,
                                                     t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractState*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_isEmpty(self as *const ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* const & QList<QAbstractState*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_last_const(self as *const ::list::ListAbstractStateMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::abstract_state::AbstractState) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractState*>::lastIndexOf(QAbstractState* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::abstract_state::AbstractState, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractState*>::lastIndexOf(QAbstractState* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListAbstractStateMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractState*& QList<QAbstractState*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_last(self as *mut ::list::ListAbstractStateMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractState*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_length(self as *const ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListAbstractStateMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractState*> QList<QAbstractState*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListAbstractStateMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractState*> QList<QAbstractState*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListAbstractStateMutPtr
    where Args: overloading::ListAbstractStateMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_move(self as *mut ::list::ListAbstractStateMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListAbstractStateMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAbstractState*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListAbstractStateMutPtr) -> ::list::ListAbstractStateMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAbstractState*>::QList(const QList<QAbstractState*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListAbstractStateMutPtr
    where Args: overloading::ListAbstractStateMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QAbstractState*> QList<QAbstractState*>::operator+(const QList<QAbstractState*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListAbstractStateMutPtr) -> ::list::ListAbstractStateMutPtr {
    {
      let mut object: ::list::ListAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractState_ptr_operator_add_to_output(self as *const ::list::ListAbstractStateMutPtr, l as *const ::list::ListAbstractStateMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*>& QList<QAbstractState*>::operator+=(const QList<QAbstractState*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListAbstractStateMutPtr)
                                 -> &'l0 mut ::list::ListAbstractStateMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractState_ptr_operator_add_assign_l(self as *mut ::list::ListAbstractStateMutPtr,
                                                                        l as *const ::list::ListAbstractStateMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*>& QList<QAbstractState*>::operator+=(QAbstractState* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::abstract_state::AbstractState)
                                               -> &'l0 mut ::list::ListAbstractStateMutPtr {
    let ffi_result = ::ffi::qt_core_c_QList_QAbstractState_ptr_operator_add_assign_t(self as *mut ::list::ListAbstractStateMutPtr, t as *const *mut ::abstract_state::AbstractState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*>& QList<QAbstractState*>::operator=(const QList<QAbstractState*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListAbstractStateMutPtr)
                             -> &'l0 mut ::list::ListAbstractStateMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractState_ptr_operator_assign(self as *mut ::list::ListAbstractStateMutPtr,
                                                                  l as *const ::list::ListAbstractStateMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractState*>::operator==(const QList<QAbstractState*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListAbstractStateMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractState_ptr_operator_eq(self as *const ::list::ListAbstractStateMutPtr,
                                                            l as *const ::list::ListAbstractStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* const & QList<QAbstractState*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractState_ptr_operator_index_const(self as *const ::list::ListAbstractStateMutPtr,
                                                                       i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractState*& QList<QAbstractState*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::abstract_state::AbstractState {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractState_ptr_operator_index(self as *mut ::list::ListAbstractStateMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractState*>::operator!=(const QList<QAbstractState*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListAbstractStateMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractState_ptr_operator_neq(self as *const ::list::ListAbstractStateMutPtr,
                                                             l as *const ::list::ListAbstractStateMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*>& QList<QAbstractState*>::operator<<(const QList<QAbstractState*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListAbstractStateMutPtr)
                          -> &'l0 mut ::list::ListAbstractStateMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractState_ptr_operator_shl_l(self as *mut ::list::ListAbstractStateMutPtr,
                                                                 l as *const ::list::ListAbstractStateMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*>& QList<QAbstractState*>::operator<<(QAbstractState* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::abstract_state::AbstractState)
                                        -> &'l0 mut ::list::ListAbstractStateMutPtr {
    let ffi_result =
      ::ffi::qt_core_c_QList_QAbstractState_ptr_operator_shl_t(self as *mut ::list::ListAbstractStateMutPtr,
                                                               t as *const *mut ::abstract_state::AbstractState);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_pop_back(self as *mut ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_pop_front(self as *mut ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::prepend(QAbstractState* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_prepend(self as *mut ::list::ListAbstractStateMutPtr,
                                                      t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::push_back(QAbstractState* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_push_back(self as *mut ::list::ListAbstractStateMutPtr,
                                                        t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::push_front(QAbstractState* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_push_front(self as *mut ::list::ListAbstractStateMutPtr,
                                                         t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractState*>::removeAll(QAbstractState* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::abstract_state::AbstractState) -> ::libc::c_int {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_removeAll(self as *mut ::list::ListAbstractStateMutPtr,
                                                        t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_removeAt(self as *mut ::list::ListAbstractStateMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_removeFirst(self as *mut ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_removeLast(self as *mut ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractState*>::removeOne(QAbstractState* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::abstract_state::AbstractState) -> bool {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_removeOne(self as *mut ::list::ListAbstractStateMutPtr,
                                                        t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::replace(int i, QAbstractState* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::abstract_state::AbstractState) {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_replace(self as *mut ::list::ListAbstractStateMutPtr,
                                                      i,
                                                      t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_reserve(self as *mut ::list::ListAbstractStateMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractState*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_size(self as *const ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractState*>::startsWith(QAbstractState* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::abstract_state::AbstractState) -> bool {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_startsWith(self as *const ::list::ListAbstractStateMutPtr,
                                                         t as *const *mut ::abstract_state::AbstractState)
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractState*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListAbstractStateMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::swap(QList<QAbstractState*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractState*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListAbstractStateMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractState* QList<QAbstractState*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::abstract_state::AbstractState {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_takeAt(self as *mut ::list::ListAbstractStateMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* QList<QAbstractState*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::abstract_state::AbstractState {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_takeFirst(self as *mut ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* QList<QAbstractState*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::abstract_state::AbstractState {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_takeLast(self as *mut ::list::ListAbstractStateMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QSet<QAbstractState*> QList<QAbstractState*>::toSet() const```</span>
  ///
  ///
  pub fn to_set(&self) -> ::set::SetAbstractStateMutPtr {
    {
      let mut object: ::set::SetAbstractStateMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractState_ptr_toSet_to_output(self as *const ::list::ListAbstractStateMutPtr,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* QList<QAbstractState*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::abstract_state::AbstractState {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_value_i(self as *const ::list::ListAbstractStateMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractState* QList<QAbstractState*>::value(int i, QAbstractState* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::abstract_state::AbstractState)
                             -> *mut ::abstract_state::AbstractState {
    ::ffi::qt_core_c_QList_QAbstractState_ptr_value_i_defaultValue(self as *const ::list::ListAbstractStateMutPtr, i, default_value as *const *mut ::abstract_state::AbstractState)
  }
}

impl Drop for ::list::ListAbstractStateMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QAbstractState*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractState_ptr_destructor(self as *mut ::list::ListAbstractStateMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QAbstractTransition*>```</span>
#[repr(C)]
pub struct ListAbstractTransitionMutPtr([u8; ::type_sizes::QT_CORE_LIST_LIST_ABSTRACT_TRANSITION_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListAbstractTransitionMutPtr {
  unsafe fn new_uninitialized() -> ListAbstractTransitionMutPtr {
    ListAbstractTransitionMutPtr(::std::mem::uninitialized())
  }
}

impl ListAbstractTransitionMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::append(const QList<QAbstractTransition*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListAbstractTransitionMutPtr) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractTransition_ptr_append_QList_QAbstractTransition_ptr(self as *mut ::list::ListAbstractTransitionMutPtr, t as *const ::list::ListAbstractTransitionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::append(QAbstractTransition* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::abstract_transition::AbstractTransition) {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_append_QAbstractTransition(self as *mut ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* const & QList<QAbstractTransition*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_at(self as *const ::list::ListAbstractTransitionMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* const & QList<QAbstractTransition*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_back_const(self as *const ::list::ListAbstractTransitionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition*& QList<QAbstractTransition*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractTransition_ptr_back(self as *mut ::list::ListAbstractTransitionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QAbstractTransition_ptr_clear(self as *mut ::list::ListAbstractTransitionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* const & QList<QAbstractTransition*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_constFirst(self as *const ::list::ListAbstractTransitionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* const & QList<QAbstractTransition*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_constLast(self as *const ::list::ListAbstractTransitionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractTransition*>::contains(QAbstractTransition* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::abstract_transition::AbstractTransition) -> bool {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_contains(self as *const ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractTransition*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_count_no_args(self as *const ::list::ListAbstractTransitionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractTransition*>::count(QAbstractTransition* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::abstract_transition::AbstractTransition) -> ::libc::c_int {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_count_t(self as *const ::list::ListAbstractTransitionMutPtr,
                                                           t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractTransition*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QAbstractTransition_ptr_empty(self as *const ::list::ListAbstractTransitionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractTransition*>::endsWith(QAbstractTransition* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::abstract_transition::AbstractTransition) -> bool {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_endsWith(self as *const ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* const & QList<QAbstractTransition*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_first_const(self as *const ::list::ListAbstractTransitionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition*& QList<QAbstractTransition*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_first(self as *mut ::list::ListAbstractTransitionMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* const & QList<QAbstractTransition*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_front_const(self as *const ::list::ListAbstractTransitionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition*& QList<QAbstractTransition*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_front(self as *mut ::list::ListAbstractTransitionMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::abstract_transition::AbstractTransition) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractTransition*>::indexOf(QAbstractTransition* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::abstract_transition::AbstractTransition, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractTransition*>::indexOf(QAbstractTransition* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListAbstractTransitionMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::insert(int i, QAbstractTransition* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::abstract_transition::AbstractTransition) {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_insert(self as *mut ::list::ListAbstractTransitionMutPtr,
                                                          i,
                                                          t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractTransition*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_isEmpty(self as *const ::list::ListAbstractTransitionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* const & QList<QAbstractTransition*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_last_const(self as *const ::list::ListAbstractTransitionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::abstract_transition::AbstractTransition) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractTransition*>::lastIndexOf(QAbstractTransition* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::abstract_transition::AbstractTransition, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractTransition*>::lastIndexOf(QAbstractTransition* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListAbstractTransitionMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractTransition*& QList<QAbstractTransition*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QAbstractTransition_ptr_last(self as *mut ::list::ListAbstractTransitionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractTransition*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_length(self as *const ::list::ListAbstractTransitionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListAbstractTransitionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*> QList<QAbstractTransition*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListAbstractTransitionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*> QList<QAbstractTransition*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListAbstractTransitionMutPtr
    where Args: overloading::ListAbstractTransitionMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_move(self as *mut ::list::ListAbstractTransitionMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListAbstractTransitionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAbstractTransition*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListAbstractTransitionMutPtr) -> ::list::ListAbstractTransitionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAbstractTransition*>::QList(const QList<QAbstractTransition*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListAbstractTransitionMutPtr
    where Args: overloading::ListAbstractTransitionMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*> QList<QAbstractTransition*>::operator+(const QList<QAbstractTransition*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListAbstractTransitionMutPtr) -> ::list::ListAbstractTransitionMutPtr {
    {
      let mut object: ::list::ListAbstractTransitionMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_operator_add_to_output(self as *const ::list::ListAbstractTransitionMutPtr, l as *const ::list::ListAbstractTransitionMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*>& QList<QAbstractTransition*>::operator+=(const QList<QAbstractTransition*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListAbstractTransitionMutPtr)
                                 -> &'l0 mut ::list::ListAbstractTransitionMutPtr {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractTransition_ptr_operator_add_assign_l(self as *mut ::list::ListAbstractTransitionMutPtr, l as *const ::list::ListAbstractTransitionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*>& QList<QAbstractTransition*>::operator+=(QAbstractTransition* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::abstract_transition::AbstractTransition)
                                               -> &'l0 mut ::list::ListAbstractTransitionMutPtr {
    let ffi_result = ::ffi::qt_core_c_QList_QAbstractTransition_ptr_operator_add_assign_t(self as *mut ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*>& QList<QAbstractTransition*>::operator=(const QList<QAbstractTransition*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListAbstractTransitionMutPtr)
                             -> &'l0 mut ::list::ListAbstractTransitionMutPtr {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractTransition_ptr_operator_assign(self as *mut ::list::ListAbstractTransitionMutPtr, l as *const ::list::ListAbstractTransitionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractTransition*>::operator==(const QList<QAbstractTransition*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListAbstractTransitionMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_operator_eq(self as *const ::list::ListAbstractTransitionMutPtr,
                                                                 l as *const ::list::ListAbstractTransitionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* const & QList<QAbstractTransition*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_transition::AbstractTransition {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractTransition_ptr_operator_index_const(self as *const ::list::ListAbstractTransitionMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition*& QList<QAbstractTransition*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::abstract_transition::AbstractTransition {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_operator_index(self as *mut ::list::ListAbstractTransitionMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractTransition*>::operator!=(const QList<QAbstractTransition*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListAbstractTransitionMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_operator_neq(self as *const ::list::ListAbstractTransitionMutPtr,
                                                                  l as *const ::list::ListAbstractTransitionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*>& QList<QAbstractTransition*>::operator<<(const QList<QAbstractTransition*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListAbstractTransitionMutPtr)
                          -> &'l0 mut ::list::ListAbstractTransitionMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractTransition_ptr_operator_shl_l(self as *mut ::list::ListAbstractTransitionMutPtr, l as *const ::list::ListAbstractTransitionMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*>& QList<QAbstractTransition*>::operator<<(QAbstractTransition* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::abstract_transition::AbstractTransition)
                                        -> &'l0 mut ::list::ListAbstractTransitionMutPtr {
    let ffi_result = ::ffi::qt_core_c_QList_QAbstractTransition_ptr_operator_shl_t(self as *mut ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_pop_back(self as *mut ::list::ListAbstractTransitionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_pop_front(self as *mut ::list::ListAbstractTransitionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::prepend(QAbstractTransition* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::abstract_transition::AbstractTransition) {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_prepend(self as *mut ::list::ListAbstractTransitionMutPtr,
                                                           t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::push_back(QAbstractTransition* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::abstract_transition::AbstractTransition) {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_push_back(self as *mut ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::push_front(QAbstractTransition* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::abstract_transition::AbstractTransition) {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_push_front(self as *mut ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractTransition*>::removeAll(QAbstractTransition* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::abstract_transition::AbstractTransition) -> ::libc::c_int {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_removeAll(self as *mut ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_removeAt(self as *mut ::list::ListAbstractTransitionMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_removeFirst(self as *mut ::list::ListAbstractTransitionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_removeLast(self as *mut ::list::ListAbstractTransitionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractTransition*>::removeOne(QAbstractTransition* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::abstract_transition::AbstractTransition) -> bool {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_removeOne(self as *mut ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::replace(int i, QAbstractTransition* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::abstract_transition::AbstractTransition) {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_replace(self as *mut ::list::ListAbstractTransitionMutPtr,
                                                           i,
                                                           t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_reserve(self as *mut ::list::ListAbstractTransitionMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractTransition*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QAbstractTransition_ptr_size(self as *const ::list::ListAbstractTransitionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractTransition*>::startsWith(QAbstractTransition* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::abstract_transition::AbstractTransition) -> bool {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_startsWith(self as *const ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition)
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractTransition*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListAbstractTransitionMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::swap(QList<QAbstractTransition*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractTransition*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListAbstractTransitionMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractTransition* QList<QAbstractTransition*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::abstract_transition::AbstractTransition {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_takeAt(self as *mut ::list::ListAbstractTransitionMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* QList<QAbstractTransition*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::abstract_transition::AbstractTransition {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_takeFirst(self as *mut ::list::ListAbstractTransitionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* QList<QAbstractTransition*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::abstract_transition::AbstractTransition {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_takeLast(self as *mut ::list::ListAbstractTransitionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* QList<QAbstractTransition*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::abstract_transition::AbstractTransition {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_value_i(self as *const ::list::ListAbstractTransitionMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractTransition* QList<QAbstractTransition*>::value(int i, QAbstractTransition* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::abstract_transition::AbstractTransition)
                             -> *mut ::abstract_transition::AbstractTransition {
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_value_i_defaultValue(self as *const ::list::ListAbstractTransitionMutPtr, i, default_value as *const *mut ::abstract_transition::AbstractTransition)
  }
}

impl Drop for ::list::ListAbstractTransitionMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QAbstractTransition*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_core_c_QList_QAbstractTransition_ptr_destructor(self as *mut ::list::ListAbstractTransitionMutPtr)
    }
  }
}

/// C++ type: <span style='color: green;'>```QList<QByteArray>```</span>
#[repr(C)]
pub struct ListByteArray([u8; ::type_sizes::QT_CORE_LIST_LIST_BYTE_ARRAY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListByteArray {
  unsafe fn new_uninitialized() -> ListByteArray {
    ListByteArray(::std::mem::uninitialized())
  }
}

impl ListByteArray {
  /// C++ method: <span style='color: green;'>```QList<QByteArray>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::byte_array::ByteArray) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::append(const QByteArray& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListByteArray) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::append(const QList<QByteArray>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListByteArrayAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QByteArray& QList<QByteArray>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_at(self as *const ::list::ListByteArray, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QByteArray& QList<QByteArray>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_back_const(self as *const ::list::ListByteArray) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QByteArray& QList<QByteArray>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_back(self as *mut ::list::ListByteArray) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_clear(self as *mut ::list::ListByteArray) }
  }

  /// C++ method: <span style='color: green;'>```const QByteArray& QList<QByteArray>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_constFirst(self as *const ::list::ListByteArray) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QByteArray& QList<QByteArray>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_constLast(self as *const ::list::ListByteArray) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QByteArray>::contains(const QByteArray& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::byte_array::ByteArray) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_contains(self as *const ::list::ListByteArray,
                                                 t as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QByteArray>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::byte_array::ByteArray) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QByteArray>::count(const QByteArray& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListByteArrayCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QByteArray>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_empty(self as *const ::list::ListByteArray) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QByteArray>::endsWith(const QByteArray& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::byte_array::ByteArray) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_endsWith(self as *const ::list::ListByteArray,
                                                 t as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```const QByteArray& QList<QByteArray>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_first_const(self as *const ::list::ListByteArray) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QByteArray& QList<QByteArray>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_first(self as *mut ::list::ListByteArray) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QByteArray& QList<QByteArray>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_front_const(self as *const ::list::ListByteArray) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QByteArray& QList<QByteArray>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_front(self as *mut ::list::ListByteArray) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::byte_array::ByteArray) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QByteArray>::indexOf(const QByteArray& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::byte_array::ByteArray, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QByteArray>::indexOf(const QByteArray& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListByteArrayIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::insert(int i, const QByteArray& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_insert(self as *mut ::list::ListByteArray,
                                               i,
                                               t as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QByteArray>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_isEmpty(self as *const ::list::ListByteArray) }
  }

  /// C++ method: <span style='color: green;'>```const QByteArray& QList<QByteArray>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_last_const(self as *const ::list::ListByteArray) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::byte_array::ByteArray) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QByteArray>::lastIndexOf(const QByteArray& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::byte_array::ByteArray, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QByteArray>::lastIndexOf(const QByteArray& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListByteArrayLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray& QList<QByteArray>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_last(self as *mut ::list::ListByteArray) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QByteArray>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_length(self as *const ::list::ListByteArray) }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```QList<QByteArray> QList<QByteArray>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```QList<QByteArray> QList<QByteArray>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListByteArray
    where Args: overloading::ListByteArrayMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_move(self as *mut ::list::ListByteArray, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QByteArray>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListByteArray) -> ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QByteArray>::QList(const QList<QByteArray>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListByteArray
    where Args: overloading::ListByteArrayNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QByteArray> QList<QByteArray>::operator+(const QList<QByteArray>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListByteArray) -> ::list::ListByteArray {
    {
      let mut object: ::list::ListByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_operator_add_to_output(self as *const ::list::ListByteArray,
                                                                 l as *const ::list::ListByteArray,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::byte_array::ByteArray) -> &'l0 mut ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```QList<QByteArray>& QList<QByteArray>::operator+=(const QByteArray& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListByteArray) -> &'l0 mut ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```QList<QByteArray>& QList<QByteArray>::operator+=(const QList<QByteArray>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListByteArray
    where Args: overloading::ListByteArrayOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QByteArray>& QList<QByteArray>::operator=(const QList<QByteArray>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListByteArray) -> &'l0 mut ::list::ListByteArray {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QByteArray_operator_assign(self as *mut ::list::ListByteArray,
                                                        l as *const ::list::ListByteArray)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QByteArray>::operator==(const QList<QByteArray>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListByteArray) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_operator_eq(self as *const ::list::ListByteArray,
                                                    l as *const ::list::ListByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```const QByteArray& QList<QByteArray>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::byte_array::ByteArray {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QByteArray_operator_index_const(self as *const ::list::ListByteArray, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QByteArray& QList<QByteArray>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::byte_array::ByteArray {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QByteArray_operator_index(self as *mut ::list::ListByteArray, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QByteArray>::operator!=(const QList<QByteArray>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListByteArray) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_operator_neq(self as *const ::list::ListByteArray,
                                                     l as *const ::list::ListByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::byte_array::ByteArray) -> &'l0 mut ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```QList<QByteArray>& QList<QByteArray>::operator<<(const QByteArray& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListByteArray) -> &'l0 mut ::list::ListByteArray```<br>
  /// C++ method: <span style='color: green;'>```QList<QByteArray>& QList<QByteArray>::operator<<(const QList<QByteArray>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListByteArray
    where Args: overloading::ListByteArrayOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_pop_back(self as *mut ::list::ListByteArray) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_pop_front(self as *mut ::list::ListByteArray) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::prepend(const QByteArray& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_prepend(self as *mut ::list::ListByteArray,
                                                t as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::push_back(const QByteArray& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_push_back(self as *mut ::list::ListByteArray,
                                                  t as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::push_front(const QByteArray& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_push_front(self as *mut ::list::ListByteArray,
                                                   t as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QByteArray>::removeAll(const QByteArray& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::byte_array::ByteArray) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_removeAll(self as *mut ::list::ListByteArray,
                                                  t as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_removeAt(self as *mut ::list::ListByteArray, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_removeFirst(self as *mut ::list::ListByteArray) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_removeLast(self as *mut ::list::ListByteArray) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QByteArray>::removeOne(const QByteArray& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::byte_array::ByteArray) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_removeOne(self as *mut ::list::ListByteArray,
                                                  t as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::replace(int i, const QByteArray& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::byte_array::ByteArray) {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_replace(self as *mut ::list::ListByteArray,
                                                i,
                                                t as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_reserve(self as *mut ::list::ListByteArray, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QByteArray>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_size(self as *const ::list::ListByteArray) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QByteArray>::startsWith(const QByteArray& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::byte_array::ByteArray) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QByteArray_startsWith(self as *const ::list::ListByteArray,
                                                   t as *const ::byte_array::ByteArray)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListByteArray) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::swap(QList<QByteArray>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QByteArray>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListByteArraySwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QByteArray QList<QByteArray>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_takeAt_to_output(self as *mut ::list::ListByteArray, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QList<QByteArray>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_takeFirst_to_output(self as *mut ::list::ListByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QByteArray QList<QByteArray>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::byte_array::ByteArray {
    {
      let mut object: ::byte_array::ByteArray =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_takeLast_to_output(self as *mut ::list::ListByteArray, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QByteArray>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QList<QByteArray>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::byte_array::ByteArray)) -> ::byte_array::ByteArray```<br>
  /// C++ method: <span style='color: green;'>```QByteArray QList<QByteArray>::value(int i, const QByteArray& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::byte_array::ByteArray
    where Args: overloading::ListByteArrayValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListByteArray {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QByteArray>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QByteArray_destructor(self as *mut ::list::ListByteArray) }
  }
}

/// C++ type: <span style='color: green;'>```QList<int>```</span>
#[repr(C)]
pub struct ListCInt([u8; ::type_sizes::QT_CORE_LIST_LIST_C_INT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListCInt {
  unsafe fn new_uninitialized() -> ListCInt {
    ListCInt(::std::mem::uninitialized())
  }
}

impl ListCInt {
  /// C++ method: <span style='color: green;'>```QList<int>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListCInt) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<int>::append(const QList<int>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<int>::append(const int& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListCIntAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const int& QList<int>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_at(self as *const ::list::ListCInt, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const int& QList<int>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_back_const(self as *const ::list::ListCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QList<int>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_back(self as *mut ::list::ListCInt) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<int>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_int_clear(self as *mut ::list::ListCInt) }
  }

  /// C++ method: <span style='color: green;'>```const int& QList<int>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_constFirst(self as *const ::list::ListCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const int& QList<int>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_constLast(self as *const ::list::ListCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<int>::contains(const int& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QList_int_contains(self as *const ::list::ListCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```QList<int>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<int>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<int>::count(const int& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListCIntCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<int>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_int_empty(self as *const ::list::ListCInt) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<int>::endsWith(const int& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QList_int_endsWith(self as *const ::list::ListCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```const int& QList<int>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_first_const(self as *const ::list::ListCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QList<int>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_first(self as *mut ::list::ListCInt) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QList<int> QList<int>::fromVector(const QVector<int>& vector)```</span>
  ///
  ///
  pub fn from_vector(vector: &::vector::VectorCInt) -> ::list::ListCInt {
    {
      let mut object: ::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_int_fromVector_to_output(vector as *const ::vector::VectorCInt, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const int& QList<int>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_front_const(self as *const ::list::ListCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QList<int>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_front(self as *mut ::list::ListCInt) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<int>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<int>::indexOf(const int& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<int>::indexOf(const int& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListCIntIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<int>::insert(int i, const int& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_int_insert(self as *mut ::list::ListCInt, i, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<int>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_int_isEmpty(self as *const ::list::ListCInt) }
  }

  /// C++ method: <span style='color: green;'>```const int& QList<int>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_last_const(self as *const ::list::ListCInt) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<int>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<int>::lastIndexOf(const int& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::libc::c_int, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<int>::lastIndexOf(const int& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListCIntLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int& QList<int>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_last(self as *mut ::list::ListCInt) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<int>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_int_length(self as *const ::list::ListCInt) }
  }

  /// C++ method: <span style='color: green;'>```QList<int>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int> QList<int>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int> QList<int>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListCInt
    where Args: overloading::ListCIntMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<int>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_int_move(self as *mut ::list::ListCInt, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<int>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<int>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListCInt) -> ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<int>::QList(const QList<int>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListCInt
    where Args: overloading::ListCIntNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<int> QList<int>::operator+(const QList<int>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListCInt) -> ::list::ListCInt {
    {
      let mut object: ::list::ListCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_int_operator_add_to_output(self as *const ::list::ListCInt,
                                                          l as *const ::list::ListCInt,
                                                          &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<int>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListCInt) -> &'l0 mut ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int>& QList<int>::operator+=(const QList<int>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::libc::c_int) -> &'l0 mut ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int>& QList<int>::operator+=(const int& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListCInt
    where Args: overloading::ListCIntOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<int>& QList<int>::operator=(const QList<int>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListCInt) -> &'l0 mut ::list::ListCInt {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_int_operator_assign(self as *mut ::list::ListCInt, l as *const ::list::ListCInt)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<int>::operator==(const QList<int>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListCInt) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_int_operator_eq(self as *const ::list::ListCInt,
                                             l as *const ::list::ListCInt)
    }
  }

  /// C++ method: <span style='color: green;'>```const int& QList<int>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_operator_index_const(self as *const ::list::ListCInt, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int& QList<int>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::libc::c_int {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_int_operator_index(self as *mut ::list::ListCInt, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<int>::operator!=(const QList<int>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListCInt) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_int_operator_neq(self as *const ::list::ListCInt,
                                              l as *const ::list::ListCInt)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<int>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListCInt) -> &'l0 mut ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int>& QList<int>::operator<<(const QList<int>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::libc::c_int) -> &'l0 mut ::list::ListCInt```<br>
  /// C++ method: <span style='color: green;'>```QList<int>& QList<int>::operator<<(const int& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListCInt
    where Args: overloading::ListCIntOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<int>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_int_pop_back(self as *mut ::list::ListCInt) }
  }

  /// C++ method: <span style='color: green;'>```void QList<int>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_int_pop_front(self as *mut ::list::ListCInt) }
  }

  /// C++ method: <span style='color: green;'>```void QList<int>::prepend(const int& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_int_prepend(self as *mut ::list::ListCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```void QList<int>::push_back(const int& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_int_push_back(self as *mut ::list::ListCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```void QList<int>::push_front(const int& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_int_push_front(self as *mut ::list::ListCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```int QList<int>::removeAll(const int& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_int_removeAll(self as *mut ::list::ListCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```void QList<int>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_int_removeAt(self as *mut ::list::ListCInt, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<int>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_int_removeFirst(self as *mut ::list::ListCInt) }
  }

  /// C++ method: <span style='color: green;'>```void QList<int>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_int_removeLast(self as *mut ::list::ListCInt) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<int>::removeOne(const int& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QList_int_removeOne(self as *mut ::list::ListCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```void QList<int>::replace(int i, const int& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_int_replace(self as *mut ::list::ListCInt, i, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```void QList<int>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_int_reserve(self as *mut ::list::ListCInt, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<int>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_int_size(self as *const ::list::ListCInt) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<int>::startsWith(const int& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::libc::c_int) -> bool {
    unsafe { ::ffi::qt_core_c_QList_int_startsWith(self as *const ::list::ListCInt, t as *const ::libc::c_int) }
  }

  /// C++ method: <span style='color: green;'>```QList<int>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListCInt) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<int>::swap(QList<int>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<int>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListCIntSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QList<int>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_int_takeAt(self as *mut ::list::ListCInt, i) }
  }

  /// C++ method: <span style='color: green;'>```int QList<int>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_int_takeFirst(self as *mut ::list::ListCInt) }
  }

  /// C++ method: <span style='color: green;'>```int QList<int>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_int_takeLast(self as *mut ::list::ListCInt) }
  }

  /// C++ method: <span style='color: green;'>```QVector<int> QList<int>::toVector() const```</span>
  ///
  ///
  pub fn to_vector(&self) -> ::vector::VectorCInt {
    {
      let mut object: ::vector::VectorCInt =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_int_toVector_to_output(self as *const ::list::ListCInt, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<int>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<int>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<int>::value(int i, const int& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListCIntValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListCInt {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<int>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_int_destructor(self as *mut ::list::ListCInt) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QCommandLineOption>```</span>
#[repr(C)]
pub struct ListCommandLineOption([u8; ::type_sizes::QT_CORE_LIST_LIST_COMMAND_LINE_OPTION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListCommandLineOption {
  unsafe fn new_uninitialized() -> ListCommandLineOption {
    ListCommandLineOption(::std::mem::uninitialized())
  }
}

impl ListCommandLineOption {
  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::command_line_option::CommandLineOption) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::append(const QCommandLineOption& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListCommandLineOption) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::append(const QList<QCommandLineOption>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListCommandLineOptionAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QCommandLineOption& QList<QCommandLineOption>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_at(self as *const ::list::ListCommandLineOption, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QCommandLineOption& QList<QCommandLineOption>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_back_const(self as *const ::list::ListCommandLineOption) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QCommandLineOption& QList<QCommandLineOption>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_back(self as *mut ::list::ListCommandLineOption) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_clear(self as *mut ::list::ListCommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```const QCommandLineOption& QList<QCommandLineOption>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_constFirst(self as *const ::list::ListCommandLineOption) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QCommandLineOption& QList<QCommandLineOption>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_constLast(self as *const ::list::ListCommandLineOption) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QCommandLineOption>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_count(self as *const ::list::ListCommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QCommandLineOption>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_empty(self as *const ::list::ListCommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```const QCommandLineOption& QList<QCommandLineOption>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_first_const(self as *const ::list::ListCommandLineOption) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QCommandLineOption& QList<QCommandLineOption>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_first(self as *mut ::list::ListCommandLineOption) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QCommandLineOption& QList<QCommandLineOption>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_front_const(self as *const ::list::ListCommandLineOption) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QCommandLineOption& QList<QCommandLineOption>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_front(self as *mut ::list::ListCommandLineOption) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::insert(int i, const QCommandLineOption& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::command_line_option::CommandLineOption) {
    unsafe {
      ::ffi::qt_core_c_QList_QCommandLineOption_insert(self as *mut ::list::ListCommandLineOption,
                                                       i,
                                                       t as *const ::command_line_option::CommandLineOption)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QCommandLineOption>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_isEmpty(self as *const ::list::ListCommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```const QCommandLineOption& QList<QCommandLineOption>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_last_const(self as *const ::list::ListCommandLineOption) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QCommandLineOption& QList<QCommandLineOption>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_last(self as *mut ::list::ListCommandLineOption) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QCommandLineOption>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_length(self as *const ::list::ListCommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListCommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption> QList<QCommandLineOption>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListCommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption> QList<QCommandLineOption>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListCommandLineOption
    where Args: overloading::ListCommandLineOptionMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_move(self as *mut ::list::ListCommandLineOption, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListCommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QCommandLineOption>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListCommandLineOption) -> ::list::ListCommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QCommandLineOption>::QList(const QList<QCommandLineOption>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListCommandLineOption
    where Args: overloading::ListCommandLineOptionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption> QList<QCommandLineOption>::operator+(const QList<QCommandLineOption>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListCommandLineOption) -> ::list::ListCommandLineOption {
    {
      let mut object: ::list::ListCommandLineOption =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QCommandLineOption_operator_add_to_output(self as *const ::list::ListCommandLineOption, l as *const ::list::ListCommandLineOption, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::command_line_option::CommandLineOption) -> &'l0 mut ::list::ListCommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption>& QList<QCommandLineOption>::operator+=(const QCommandLineOption& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListCommandLineOption) -> &'l0 mut ::list::ListCommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption>& QList<QCommandLineOption>::operator+=(const QList<QCommandLineOption>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListCommandLineOption
    where Args: overloading::ListCommandLineOptionOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption>& QList<QCommandLineOption>::operator=(const QList<QCommandLineOption>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListCommandLineOption)
                             -> &'l0 mut ::list::ListCommandLineOption {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QCommandLineOption_operator_assign(self as *mut ::list::ListCommandLineOption,
                                                                  l as *const ::list::ListCommandLineOption)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QCommandLineOption& QList<QCommandLineOption>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QCommandLineOption_operator_index_const(self as *const ::list::ListCommandLineOption, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QCommandLineOption& QList<QCommandLineOption>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::command_line_option::CommandLineOption {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QCommandLineOption_operator_index(self as *mut ::list::ListCommandLineOption, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::command_line_option::CommandLineOption) -> &'l0 mut ::list::ListCommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption>& QList<QCommandLineOption>::operator<<(const QCommandLineOption& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListCommandLineOption) -> &'l0 mut ::list::ListCommandLineOption```<br>
  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption>& QList<QCommandLineOption>::operator<<(const QList<QCommandLineOption>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListCommandLineOption
    where Args: overloading::ListCommandLineOptionOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_pop_back(self as *mut ::list::ListCommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_pop_front(self as *mut ::list::ListCommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::prepend(const QCommandLineOption& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::command_line_option::CommandLineOption) {
    unsafe {
      ::ffi::qt_core_c_QList_QCommandLineOption_prepend(self as *mut ::list::ListCommandLineOption,
                                                        t as *const ::command_line_option::CommandLineOption)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::push_back(const QCommandLineOption& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::command_line_option::CommandLineOption) {
    unsafe {
      ::ffi::qt_core_c_QList_QCommandLineOption_push_back(self as *mut ::list::ListCommandLineOption,
                                                          t as *const ::command_line_option::CommandLineOption)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::push_front(const QCommandLineOption& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::command_line_option::CommandLineOption) {
    unsafe {
      ::ffi::qt_core_c_QList_QCommandLineOption_push_front(self as *mut ::list::ListCommandLineOption,
                                                           t as *const ::command_line_option::CommandLineOption)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_removeAt(self as *mut ::list::ListCommandLineOption, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_removeFirst(self as *mut ::list::ListCommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_removeLast(self as *mut ::list::ListCommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::replace(int i, const QCommandLineOption& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::command_line_option::CommandLineOption) {
    unsafe {
      ::ffi::qt_core_c_QList_QCommandLineOption_replace(self as *mut ::list::ListCommandLineOption,
                                                        i,
                                                        t as *const ::command_line_option::CommandLineOption)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_reserve(self as *mut ::list::ListCommandLineOption, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QCommandLineOption>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_size(self as *const ::list::ListCommandLineOption) }
  }

  /// C++ method: <span style='color: green;'>```QList<QCommandLineOption>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListCommandLineOption) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::swap(QList<QCommandLineOption>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QCommandLineOption>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListCommandLineOptionSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QCommandLineOption QList<QCommandLineOption>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::command_line_option::CommandLineOption {
    {
      let mut object: ::command_line_option::CommandLineOption =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QCommandLineOption_takeAt_to_output(self as *mut ::list::ListCommandLineOption,
                                                                   i,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCommandLineOption QList<QCommandLineOption>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::command_line_option::CommandLineOption {
    {
      let mut object: ::command_line_option::CommandLineOption =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QCommandLineOption_takeFirst_to_output(self as *mut ::list::ListCommandLineOption,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QCommandLineOption QList<QCommandLineOption>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::command_line_option::CommandLineOption {
    {
      let mut object: ::command_line_option::CommandLineOption =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QCommandLineOption_takeLast_to_output(self as *mut ::list::ListCommandLineOption,
                                                                     &mut object);
      }
      object
    }
  }
}

impl Drop for ::list::ListCommandLineOption {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QCommandLineOption>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_destructor(self as *mut ::list::ListCommandLineOption) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QFileInfo>```</span>
#[repr(C)]
pub struct ListFileInfo([u8; ::type_sizes::QT_CORE_LIST_LIST_FILE_INFO]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListFileInfo {
  unsafe fn new_uninitialized() -> ListFileInfo {
    ListFileInfo(::std::mem::uninitialized())
  }
}

impl ListFileInfo {
  /// C++ method: <span style='color: green;'>```QList<QFileInfo>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::file_info::FileInfo) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::append(const QFileInfo& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListFileInfo) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::append(const QList<QFileInfo>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListFileInfoAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QFileInfo& QList<QFileInfo>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_at(self as *const ::list::ListFileInfo, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QFileInfo& QList<QFileInfo>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_back_const(self as *const ::list::ListFileInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFileInfo& QList<QFileInfo>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_back(self as *mut ::list::ListFileInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_clear(self as *mut ::list::ListFileInfo) }
  }

  /// C++ method: <span style='color: green;'>```const QFileInfo& QList<QFileInfo>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_constFirst(self as *const ::list::ListFileInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QFileInfo& QList<QFileInfo>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_constLast(self as *const ::list::ListFileInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFileInfo>::contains(const QFileInfo& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::file_info::FileInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_contains(self as *const ::list::ListFileInfo,
                                                t as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFileInfo>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFileInfo>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::file_info::FileInfo) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFileInfo>::count(const QFileInfo& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListFileInfoCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QFileInfo>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_empty(self as *const ::list::ListFileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFileInfo>::endsWith(const QFileInfo& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::file_info::FileInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_endsWith(self as *const ::list::ListFileInfo,
                                                t as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```const QFileInfo& QList<QFileInfo>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_first_const(self as *const ::list::ListFileInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFileInfo& QList<QFileInfo>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_first(self as *mut ::list::ListFileInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QFileInfo& QList<QFileInfo>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_front_const(self as *const ::list::ListFileInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFileInfo& QList<QFileInfo>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_front(self as *mut ::list::ListFileInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QFileInfo>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::file_info::FileInfo) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFileInfo>::indexOf(const QFileInfo& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::file_info::FileInfo, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFileInfo>::indexOf(const QFileInfo& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListFileInfoIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::insert(int i, const QFileInfo& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::file_info::FileInfo) {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_insert(self as *mut ::list::ListFileInfo,
                                              i,
                                              t as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFileInfo>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_isEmpty(self as *const ::list::ListFileInfo) }
  }

  /// C++ method: <span style='color: green;'>```const QFileInfo& QList<QFileInfo>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_last_const(self as *const ::list::ListFileInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QFileInfo>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::file_info::FileInfo) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFileInfo>::lastIndexOf(const QFileInfo& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::file_info::FileInfo, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFileInfo>::lastIndexOf(const QFileInfo& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListFileInfoLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFileInfo& QList<QFileInfo>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_last(self as *mut ::list::ListFileInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QFileInfo>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_length(self as *const ::list::ListFileInfo) }
  }

  /// C++ method: <span style='color: green;'>```QList<QFileInfo>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo> QList<QFileInfo>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo> QList<QFileInfo>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListFileInfo
    where Args: overloading::ListFileInfoMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_move(self as *mut ::list::ListFileInfo, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QFileInfo>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QFileInfo>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListFileInfo) -> ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QFileInfo>::QList(const QList<QFileInfo>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListFileInfo
    where Args: overloading::ListFileInfoNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QFileInfo> QList<QFileInfo>::operator+(const QList<QFileInfo>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListFileInfo) -> ::list::ListFileInfo {
    {
      let mut object: ::list::ListFileInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_operator_add_to_output(self as *const ::list::ListFileInfo,
                                                                l as *const ::list::ListFileInfo,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFileInfo>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::file_info::FileInfo) -> &'l0 mut ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo>& QList<QFileInfo>::operator+=(const QFileInfo& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListFileInfo) -> &'l0 mut ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo>& QList<QFileInfo>::operator+=(const QList<QFileInfo>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListFileInfo
    where Args: overloading::ListFileInfoOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QFileInfo>& QList<QFileInfo>::operator=(const QList<QFileInfo>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListFileInfo) -> &'l0 mut ::list::ListFileInfo {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_operator_assign(self as *mut ::list::ListFileInfo,
                                                       l as *const ::list::ListFileInfo)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFileInfo>::operator==(const QList<QFileInfo>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListFileInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_operator_eq(self as *const ::list::ListFileInfo,
                                                   l as *const ::list::ListFileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```const QFileInfo& QList<QFileInfo>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::file_info::FileInfo {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QFileInfo_operator_index_const(self as *const ::list::ListFileInfo, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFileInfo& QList<QFileInfo>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::file_info::FileInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QFileInfo_operator_index(self as *mut ::list::ListFileInfo, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFileInfo>::operator!=(const QList<QFileInfo>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListFileInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_operator_neq(self as *const ::list::ListFileInfo,
                                                    l as *const ::list::ListFileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFileInfo>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::file_info::FileInfo) -> &'l0 mut ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo>& QList<QFileInfo>::operator<<(const QFileInfo& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListFileInfo) -> &'l0 mut ::list::ListFileInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QFileInfo>& QList<QFileInfo>::operator<<(const QList<QFileInfo>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListFileInfo
    where Args: overloading::ListFileInfoOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_pop_back(self as *mut ::list::ListFileInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_pop_front(self as *mut ::list::ListFileInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::prepend(const QFileInfo& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::file_info::FileInfo) {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_prepend(self as *mut ::list::ListFileInfo,
                                               t as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::push_back(const QFileInfo& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::file_info::FileInfo) {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_push_back(self as *mut ::list::ListFileInfo,
                                                 t as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::push_front(const QFileInfo& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::file_info::FileInfo) {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_push_front(self as *mut ::list::ListFileInfo,
                                                  t as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QFileInfo>::removeAll(const QFileInfo& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::file_info::FileInfo) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_removeAll(self as *mut ::list::ListFileInfo,
                                                 t as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_removeAt(self as *mut ::list::ListFileInfo, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_removeFirst(self as *mut ::list::ListFileInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_removeLast(self as *mut ::list::ListFileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFileInfo>::removeOne(const QFileInfo& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::file_info::FileInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_removeOne(self as *mut ::list::ListFileInfo,
                                                 t as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::replace(int i, const QFileInfo& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::file_info::FileInfo) {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_replace(self as *mut ::list::ListFileInfo,
                                               i,
                                               t as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_reserve(self as *mut ::list::ListFileInfo, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QFileInfo>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_size(self as *const ::list::ListFileInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFileInfo>::startsWith(const QFileInfo& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::file_info::FileInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QFileInfo_startsWith(self as *const ::list::ListFileInfo,
                                                  t as *const ::file_info::FileInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFileInfo>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListFileInfo) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::swap(QList<QFileInfo>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QFileInfo>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListFileInfoSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFileInfo QList<QFileInfo>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::file_info::FileInfo {
    {
      let mut object: ::file_info::FileInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_takeAt_to_output(self as *mut ::list::ListFileInfo, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileInfo QList<QFileInfo>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::file_info::FileInfo {
    {
      let mut object: ::file_info::FileInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_takeFirst_to_output(self as *mut ::list::ListFileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QFileInfo QList<QFileInfo>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::file_info::FileInfo {
    {
      let mut object: ::file_info::FileInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_takeLast_to_output(self as *mut ::list::ListFileInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFileInfo>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::file_info::FileInfo```<br>
  /// C++ method: <span style='color: green;'>```QFileInfo QList<QFileInfo>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::file_info::FileInfo)) -> ::file_info::FileInfo```<br>
  /// C++ method: <span style='color: green;'>```QFileInfo QList<QFileInfo>::value(int i, const QFileInfo& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::file_info::FileInfo
    where Args: overloading::ListFileInfoValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListFileInfo {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QFileInfo>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QFileInfo_destructor(self as *mut ::list::ListFileInfo) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QItemSelectionRange>```</span>
#[repr(C)]
pub struct ListItemSelectionRange([u8; ::type_sizes::QT_CORE_LIST_LIST_ITEM_SELECTION_RANGE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListItemSelectionRange {
  unsafe fn new_uninitialized() -> ListItemSelectionRange {
    ListItemSelectionRange(::std::mem::uninitialized())
  }
}

impl ListItemSelectionRange {
  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::item_selection_range::ItemSelectionRange) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::append(const QItemSelectionRange& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListItemSelectionRange) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::append(const QList<QItemSelectionRange>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListItemSelectionRangeAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QItemSelectionRange& QList<QItemSelectionRange>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_at(self as *const ::list::ListItemSelectionRange, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QItemSelectionRange& QList<QItemSelectionRange>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_back_const(self as *const ::list::ListItemSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionRange& QList<QItemSelectionRange>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_back(self as *mut ::list::ListItemSelectionRange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_clear(self as *mut ::list::ListItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```const QItemSelectionRange& QList<QItemSelectionRange>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_constFirst(self as *const ::list::ListItemSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QItemSelectionRange& QList<QItemSelectionRange>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_constLast(self as *const ::list::ListItemSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QItemSelectionRange>::contains(const QItemSelectionRange& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::item_selection_range::ItemSelectionRange) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_contains(self as *const ::list::ListItemSelectionRange,
                                                          t as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QItemSelectionRange>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::item_selection_range::ItemSelectionRange) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QItemSelectionRange>::count(const QItemSelectionRange& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListItemSelectionRangeCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QItemSelectionRange>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_empty(self as *const ::list::ListItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QItemSelectionRange>::endsWith(const QItemSelectionRange& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::item_selection_range::ItemSelectionRange) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_endsWith(self as *const ::list::ListItemSelectionRange,
                                                          t as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```const QItemSelectionRange& QList<QItemSelectionRange>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_first_const(self as *const ::list::ListItemSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionRange& QList<QItemSelectionRange>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_first(self as *mut ::list::ListItemSelectionRange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QItemSelectionRange& QList<QItemSelectionRange>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_front_const(self as *const ::list::ListItemSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionRange& QList<QItemSelectionRange>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_front(self as *mut ::list::ListItemSelectionRange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::item_selection_range::ItemSelectionRange) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QItemSelectionRange>::indexOf(const QItemSelectionRange& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::item_selection_range::ItemSelectionRange, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QItemSelectionRange>::indexOf(const QItemSelectionRange& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListItemSelectionRangeIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::insert(int i, const QItemSelectionRange& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::item_selection_range::ItemSelectionRange) {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_insert(self as *mut ::list::ListItemSelectionRange,
                                                        i,
                                                        t as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QItemSelectionRange>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_isEmpty(self as *const ::list::ListItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```const QItemSelectionRange& QList<QItemSelectionRange>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_last_const(self as *const ::list::ListItemSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::item_selection_range::ItemSelectionRange) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QItemSelectionRange>::lastIndexOf(const QItemSelectionRange& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::item_selection_range::ItemSelectionRange, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QItemSelectionRange>::lastIndexOf(const QItemSelectionRange& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListItemSelectionRangeLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QItemSelectionRange& QList<QItemSelectionRange>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_last(self as *mut ::list::ListItemSelectionRange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QItemSelectionRange>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_length(self as *const ::list::ListItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange> QList<QItemSelectionRange>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange> QList<QItemSelectionRange>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListItemSelectionRange
    where Args: overloading::ListItemSelectionRangeMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_move(self as *mut ::list::ListItemSelectionRange, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QItemSelectionRange>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListItemSelectionRange) -> ::list::ListItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QItemSelectionRange>::QList(const QList<QItemSelectionRange>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListItemSelectionRange
    where Args: overloading::ListItemSelectionRangeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange> QList<QItemSelectionRange>::operator+(const QList<QItemSelectionRange>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListItemSelectionRange) -> ::list::ListItemSelectionRange {
    {
      let mut object: ::list::ListItemSelectionRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_operator_add_to_output(self as *const ::list::ListItemSelectionRange, l as *const ::list::ListItemSelectionRange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::item_selection_range::ItemSelectionRange) -> &'l0 mut ::list::ListItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>& QList<QItemSelectionRange>::operator+=(const QItemSelectionRange& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListItemSelectionRange) -> &'l0 mut ::list::ListItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>& QList<QItemSelectionRange>::operator+=(const QList<QItemSelectionRange>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListItemSelectionRange
    where Args: overloading::ListItemSelectionRangeOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>& QList<QItemSelectionRange>::operator=(const QList<QItemSelectionRange>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListItemSelectionRange)
                             -> &'l0 mut ::list::ListItemSelectionRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_operator_assign(self as *mut ::list::ListItemSelectionRange,
                                                                   l as *const ::list::ListItemSelectionRange)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QItemSelectionRange>::operator==(const QList<QItemSelectionRange>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListItemSelectionRange) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_operator_eq(self as *const ::list::ListItemSelectionRange,
                                                             l as *const ::list::ListItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```const QItemSelectionRange& QList<QItemSelectionRange>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_operator_index_const(self as *const ::list::ListItemSelectionRange,
                                                                        i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionRange& QList<QItemSelectionRange>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::item_selection_range::ItemSelectionRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_operator_index(self as *mut ::list::ListItemSelectionRange, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QItemSelectionRange>::operator!=(const QList<QItemSelectionRange>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListItemSelectionRange) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_operator_neq(self as *const ::list::ListItemSelectionRange,
                                                              l as *const ::list::ListItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::item_selection_range::ItemSelectionRange) -> &'l0 mut ::list::ListItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>& QList<QItemSelectionRange>::operator<<(const QItemSelectionRange& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListItemSelectionRange) -> &'l0 mut ::list::ListItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>& QList<QItemSelectionRange>::operator<<(const QList<QItemSelectionRange>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListItemSelectionRange
    where Args: overloading::ListItemSelectionRangeOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_pop_back(self as *mut ::list::ListItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_pop_front(self as *mut ::list::ListItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::prepend(const QItemSelectionRange& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::item_selection_range::ItemSelectionRange) {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_prepend(self as *mut ::list::ListItemSelectionRange,
                                                         t as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::push_back(const QItemSelectionRange& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::item_selection_range::ItemSelectionRange) {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_push_back(self as *mut ::list::ListItemSelectionRange,
                                                           t as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::push_front(const QItemSelectionRange& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::item_selection_range::ItemSelectionRange) {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_push_front(self as *mut ::list::ListItemSelectionRange,
                                                            t as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QItemSelectionRange>::removeAll(const QItemSelectionRange& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::item_selection_range::ItemSelectionRange) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_removeAll(self as *mut ::list::ListItemSelectionRange,
                                                           t as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_removeAt(self as *mut ::list::ListItemSelectionRange, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_removeFirst(self as *mut ::list::ListItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_removeLast(self as *mut ::list::ListItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QItemSelectionRange>::removeOne(const QItemSelectionRange& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::item_selection_range::ItemSelectionRange) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_removeOne(self as *mut ::list::ListItemSelectionRange,
                                                           t as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::replace(int i, const QItemSelectionRange& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::item_selection_range::ItemSelectionRange) {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_replace(self as *mut ::list::ListItemSelectionRange,
                                                         i,
                                                         t as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_reserve(self as *mut ::list::ListItemSelectionRange, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QItemSelectionRange>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_size(self as *const ::list::ListItemSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QItemSelectionRange>::startsWith(const QItemSelectionRange& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::item_selection_range::ItemSelectionRange) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QItemSelectionRange_startsWith(self as *const ::list::ListItemSelectionRange,
                                                            t as *const ::item_selection_range::ItemSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListItemSelectionRange) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::swap(QList<QItemSelectionRange>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QItemSelectionRange>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListItemSelectionRangeSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QItemSelectionRange QList<QItemSelectionRange>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::item_selection_range::ItemSelectionRange {
    {
      let mut object: ::item_selection_range::ItemSelectionRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_takeAt_to_output(self as *mut ::list::ListItemSelectionRange,
                                                                    i,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionRange QList<QItemSelectionRange>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::item_selection_range::ItemSelectionRange {
    {
      let mut object: ::item_selection_range::ItemSelectionRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_takeFirst_to_output(self as *mut ::list::ListItemSelectionRange,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QItemSelectionRange QList<QItemSelectionRange>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::item_selection_range::ItemSelectionRange {
    {
      let mut object: ::item_selection_range::ItemSelectionRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_takeLast_to_output(self as *mut ::list::ListItemSelectionRange,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QItemSelectionRange>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::item_selection_range::ItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QItemSelectionRange QList<QItemSelectionRange>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::item_selection_range::ItemSelectionRange)) -> ::item_selection_range::ItemSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QItemSelectionRange QList<QItemSelectionRange>::value(int i, const QItemSelectionRange& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::item_selection_range::ItemSelectionRange
    where Args: overloading::ListItemSelectionRangeValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListItemSelectionRange {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QItemSelectionRange>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_destructor(self as *mut ::list::ListItemSelectionRange) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QLocale>```</span>
#[repr(C)]
pub struct ListLocale([u8; ::type_sizes::QT_CORE_LIST_LIST_LOCALE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListLocale {
  unsafe fn new_uninitialized() -> ListLocale {
    ListLocale(::std::mem::uninitialized())
  }
}

impl ListLocale {
  /// C++ method: <span style='color: green;'>```QList<QLocale>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListLocale) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QLocale>::append(const QList<QLocale>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::locale::Locale) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QLocale>::append(const QLocale& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListLocaleAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QLocale& QList<QLocale>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_at(self as *const ::list::ListLocale, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLocale& QList<QLocale>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_back_const(self as *const ::list::ListLocale) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLocale& QList<QLocale>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_back(self as *mut ::list::ListLocale) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_clear(self as *mut ::list::ListLocale) }
  }

  /// C++ method: <span style='color: green;'>```const QLocale& QList<QLocale>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_constFirst(self as *const ::list::ListLocale) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLocale& QList<QLocale>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_constLast(self as *const ::list::ListLocale) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale>::contains(const QLocale& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::locale::Locale) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_contains(self as *const ::list::ListLocale,
                                              t as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::locale::Locale) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale>::count(const QLocale& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListLocaleCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QLocale>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QLocale_empty(self as *const ::list::ListLocale) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale>::endsWith(const QLocale& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::locale::Locale) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_endsWith(self as *const ::list::ListLocale,
                                              t as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```const QLocale& QList<QLocale>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_first_const(self as *const ::list::ListLocale) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLocale& QList<QLocale>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_first(self as *mut ::list::ListLocale) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLocale& QList<QLocale>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_front_const(self as *const ::list::ListLocale) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLocale& QList<QLocale>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_front(self as *mut ::list::ListLocale) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::locale::Locale) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale>::indexOf(const QLocale& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::locale::Locale, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale>::indexOf(const QLocale& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListLocaleIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QLocale>::insert(int i, const QLocale& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::locale::Locale) {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_insert(self as *mut ::list::ListLocale,
                                            i,
                                            t as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QLocale_isEmpty(self as *const ::list::ListLocale) }
  }

  /// C++ method: <span style='color: green;'>```const QLocale& QList<QLocale>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_last_const(self as *const ::list::ListLocale) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::locale::Locale) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale>::lastIndexOf(const QLocale& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::locale::Locale, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale>::lastIndexOf(const QLocale& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListLocaleLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLocale& QList<QLocale>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_last(self as *mut ::list::ListLocale) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QLocale>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QLocale_length(self as *const ::list::ListLocale) }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListLocale```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale> QList<QLocale>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListLocale```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale> QList<QLocale>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListLocale
    where Args: overloading::ListLocaleMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QLocale>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_move(self as *mut ::list::ListLocale, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListLocale```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QLocale>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListLocale) -> ::list::ListLocale```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QLocale>::QList(const QList<QLocale>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListLocale
    where Args: overloading::ListLocaleNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QLocale> QList<QLocale>::operator+(const QList<QLocale>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListLocale) -> ::list::ListLocale {
    {
      let mut object: ::list::ListLocale =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_operator_add_to_output(self as *const ::list::ListLocale,
                                                              l as *const ::list::ListLocale,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListLocale) -> &'l0 mut ::list::ListLocale```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale>& QList<QLocale>::operator+=(const QList<QLocale>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::locale::Locale) -> &'l0 mut ::list::ListLocale```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale>& QList<QLocale>::operator+=(const QLocale& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListLocale
    where Args: overloading::ListLocaleOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QLocale>& QList<QLocale>::operator=(const QList<QLocale>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListLocale) -> &'l0 mut ::list::ListLocale {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QLocale_operator_assign(self as *mut ::list::ListLocale,
                                                     l as *const ::list::ListLocale)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale>::operator==(const QList<QLocale>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListLocale) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_operator_eq(self as *const ::list::ListLocale,
                                                 l as *const ::list::ListLocale)
    }
  }

  /// C++ method: <span style='color: green;'>```const QLocale& QList<QLocale>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::locale::Locale {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QLocale_operator_index_const(self as *const ::list::ListLocale, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLocale& QList<QLocale>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::locale::Locale {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_operator_index(self as *mut ::list::ListLocale, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale>::operator!=(const QList<QLocale>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListLocale) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_operator_neq(self as *const ::list::ListLocale,
                                                  l as *const ::list::ListLocale)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListLocale) -> &'l0 mut ::list::ListLocale```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale>& QList<QLocale>::operator<<(const QList<QLocale>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::locale::Locale) -> &'l0 mut ::list::ListLocale```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale>& QList<QLocale>::operator<<(const QLocale& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListLocale
    where Args: overloading::ListLocaleOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QLocale>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_pop_back(self as *mut ::list::ListLocale) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_pop_front(self as *mut ::list::ListLocale) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale>::prepend(const QLocale& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::locale::Locale) {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_prepend(self as *mut ::list::ListLocale,
                                             t as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale>::push_back(const QLocale& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::locale::Locale) {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_push_back(self as *mut ::list::ListLocale,
                                               t as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale>::push_front(const QLocale& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::locale::Locale) {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_push_front(self as *mut ::list::ListLocale,
                                                t as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QLocale>::removeAll(const QLocale& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::locale::Locale) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_removeAll(self as *mut ::list::ListLocale,
                                               t as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_removeAt(self as *mut ::list::ListLocale, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_removeFirst(self as *mut ::list::ListLocale) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_removeLast(self as *mut ::list::ListLocale) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale>::removeOne(const QLocale& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::locale::Locale) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_removeOne(self as *mut ::list::ListLocale,
                                               t as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale>::replace(int i, const QLocale& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::locale::Locale) {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_replace(self as *mut ::list::ListLocale,
                                             i,
                                             t as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_reserve(self as *mut ::list::ListLocale, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QLocale>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QLocale_size(self as *const ::list::ListLocale) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale>::startsWith(const QLocale& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::locale::Locale) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_startsWith(self as *const ::list::ListLocale,
                                                t as *const ::locale::Locale)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListLocale) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QLocale>::swap(QList<QLocale>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QLocale>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListLocaleSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLocale QList<QLocale>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::locale::Locale {
    {
      let mut object: ::locale::Locale =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_takeAt_to_output(self as *mut ::list::ListLocale, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLocale QList<QLocale>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::locale::Locale {
    {
      let mut object: ::locale::Locale =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_takeFirst_to_output(self as *mut ::list::ListLocale, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QLocale QList<QLocale>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::locale::Locale {
    {
      let mut object: ::locale::Locale =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_takeLast_to_output(self as *mut ::list::ListLocale, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::locale::Locale```<br>
  /// C++ method: <span style='color: green;'>```QLocale QList<QLocale>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::locale::Locale)) -> ::locale::Locale```<br>
  /// C++ method: <span style='color: green;'>```QLocale QList<QLocale>::value(int i, const QLocale& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::locale::Locale
    where Args: overloading::ListLocaleValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListLocale {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QLocale>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_destructor(self as *mut ::list::ListLocale) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QLocale::Country>```</span>
#[repr(C)]
pub struct ListLocaleCountry([u8; ::type_sizes::QT_CORE_LIST_LIST_LOCALE_COUNTRY]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListLocaleCountry {
  unsafe fn new_uninitialized() -> ListLocaleCountry {
    ListLocaleCountry(::std::mem::uninitialized())
  }
}

impl ListLocaleCountry {
  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListLocaleCountry) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::append(const QList<QLocale::Country>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::locale::Country) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::append(const QLocale::Country& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListLocaleCountryAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QLocale::Country& QList<QLocale::Country>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::locale::Country {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_Country_at(self as *const ::list::ListLocaleCountry, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLocale::Country& QList<QLocale::Country>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::locale::Country {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QLocale_Country_back_const(self as *const ::list::ListLocaleCountry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLocale::Country& QList<QLocale::Country>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::locale::Country {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_Country_back(self as *mut ::list::ListLocaleCountry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_clear(self as *mut ::list::ListLocaleCountry) }
  }

  /// C++ method: <span style='color: green;'>```const QLocale::Country& QList<QLocale::Country>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::locale::Country {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QLocale_Country_constFirst(self as *const ::list::ListLocaleCountry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLocale::Country& QList<QLocale::Country>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::locale::Country {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QLocale_Country_constLast(self as *const ::list::ListLocaleCountry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale::Country>::contains(const QLocale::Country& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::locale::Country) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_contains(self as *const ::list::ListLocaleCountry,
                                                      t as *const ::locale::Country)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale::Country>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::locale::Country) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale::Country>::count(const QLocale::Country& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListLocaleCountryCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QLocale::Country>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_empty(self as *const ::list::ListLocaleCountry) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale::Country>::endsWith(const QLocale::Country& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::locale::Country) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_endsWith(self as *const ::list::ListLocaleCountry,
                                                      t as *const ::locale::Country)
    }
  }

  /// C++ method: <span style='color: green;'>```const QLocale::Country& QList<QLocale::Country>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::locale::Country {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QLocale_Country_first_const(self as *const ::list::ListLocaleCountry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLocale::Country& QList<QLocale::Country>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::locale::Country {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_Country_first(self as *mut ::list::ListLocaleCountry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QLocale::Country& QList<QLocale::Country>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::locale::Country {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QLocale_Country_front_const(self as *const ::list::ListLocaleCountry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLocale::Country& QList<QLocale::Country>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::locale::Country {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_Country_front(self as *mut ::list::ListLocaleCountry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::locale::Country) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale::Country>::indexOf(const QLocale::Country& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::locale::Country, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale::Country>::indexOf(const QLocale::Country& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListLocaleCountryIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::insert(int i, const QLocale::Country& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::locale::Country) {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_insert(self as *mut ::list::ListLocaleCountry,
                                                    i,
                                                    t as *const ::locale::Country)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale::Country>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_isEmpty(self as *const ::list::ListLocaleCountry) }
  }

  /// C++ method: <span style='color: green;'>```const QLocale::Country& QList<QLocale::Country>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::locale::Country {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QLocale_Country_last_const(self as *const ::list::ListLocaleCountry) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::locale::Country) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale::Country>::lastIndexOf(const QLocale::Country& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::locale::Country, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QLocale::Country>::lastIndexOf(const QLocale::Country& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListLocaleCountryLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLocale::Country& QList<QLocale::Country>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::locale::Country {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QLocale_Country_last(self as *mut ::list::ListLocaleCountry) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QLocale::Country>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_length(self as *const ::list::ListLocaleCountry) }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListLocaleCountry```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale::Country> QList<QLocale::Country>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListLocaleCountry```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale::Country> QList<QLocale::Country>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListLocaleCountry
    where Args: overloading::ListLocaleCountryMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_move(self as *mut ::list::ListLocaleCountry, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListLocaleCountry```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QLocale::Country>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListLocaleCountry) -> ::list::ListLocaleCountry```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QLocale::Country>::QList(const QList<QLocale::Country>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListLocaleCountry
    where Args: overloading::ListLocaleCountryNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QLocale::Country> QList<QLocale::Country>::operator+(const QList<QLocale::Country>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListLocaleCountry) -> ::list::ListLocaleCountry {
    {
      let mut object: ::list::ListLocaleCountry =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_Country_operator_add_to_output(self as *const ::list::ListLocaleCountry,
                                                                      l as *const ::list::ListLocaleCountry,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListLocaleCountry) -> &'l0 mut ::list::ListLocaleCountry```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>& QList<QLocale::Country>::operator+=(const QList<QLocale::Country>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::locale::Country) -> &'l0 mut ::list::ListLocaleCountry```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>& QList<QLocale::Country>::operator+=(const QLocale::Country& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListLocaleCountry
    where Args: overloading::ListLocaleCountryOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>& QList<QLocale::Country>::operator=(const QList<QLocale::Country>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListLocaleCountry) -> &'l0 mut ::list::ListLocaleCountry {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_operator_assign(self as *mut ::list::ListLocaleCountry,
                                                             l as *const ::list::ListLocaleCountry)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale::Country>::operator==(const QList<QLocale::Country>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListLocaleCountry) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_operator_eq(self as *const ::list::ListLocaleCountry,
                                                         l as *const ::list::ListLocaleCountry)
    }
  }

  /// C++ method: <span style='color: green;'>```const QLocale::Country& QList<QLocale::Country>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::locale::Country {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_Country_operator_index_const(self as *const ::list::ListLocaleCountry, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QLocale::Country& QList<QLocale::Country>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::locale::Country {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QLocale_Country_operator_index(self as *mut ::list::ListLocaleCountry, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale::Country>::operator!=(const QList<QLocale::Country>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListLocaleCountry) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_operator_neq(self as *const ::list::ListLocaleCountry,
                                                          l as *const ::list::ListLocaleCountry)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListLocaleCountry) -> &'l0 mut ::list::ListLocaleCountry```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>& QList<QLocale::Country>::operator<<(const QList<QLocale::Country>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::locale::Country) -> &'l0 mut ::list::ListLocaleCountry```<br>
  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>& QList<QLocale::Country>::operator<<(const QLocale::Country& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListLocaleCountry
    where Args: overloading::ListLocaleCountryOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_pop_back(self as *mut ::list::ListLocaleCountry) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_pop_front(self as *mut ::list::ListLocaleCountry) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::prepend(const QLocale::Country& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::locale::Country) {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_prepend(self as *mut ::list::ListLocaleCountry,
                                                     t as *const ::locale::Country)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::push_back(const QLocale::Country& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::locale::Country) {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_push_back(self as *mut ::list::ListLocaleCountry,
                                                       t as *const ::locale::Country)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::push_front(const QLocale::Country& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::locale::Country) {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_push_front(self as *mut ::list::ListLocaleCountry,
                                                        t as *const ::locale::Country)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QLocale::Country>::removeAll(const QLocale::Country& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::locale::Country) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_removeAll(self as *mut ::list::ListLocaleCountry,
                                                       t as *const ::locale::Country)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_removeAt(self as *mut ::list::ListLocaleCountry, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_removeFirst(self as *mut ::list::ListLocaleCountry) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_removeLast(self as *mut ::list::ListLocaleCountry) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale::Country>::removeOne(const QLocale::Country& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::locale::Country) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_removeOne(self as *mut ::list::ListLocaleCountry,
                                                       t as *const ::locale::Country)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::replace(int i, const QLocale::Country& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::locale::Country) {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_replace(self as *mut ::list::ListLocaleCountry,
                                                     i,
                                                     t as *const ::locale::Country)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_reserve(self as *mut ::list::ListLocaleCountry, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QLocale::Country>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_size(self as *const ::list::ListLocaleCountry) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QLocale::Country>::startsWith(const QLocale::Country& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::locale::Country) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QLocale_Country_startsWith(self as *const ::list::ListLocaleCountry,
                                                        t as *const ::locale::Country)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListLocaleCountry) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::swap(QList<QLocale::Country>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QLocale::Country>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListLocaleCountrySwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QLocale::Country QList<QLocale::Country>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::locale::Country {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_takeAt(self as *mut ::list::ListLocaleCountry, i) }
  }

  /// C++ method: <span style='color: green;'>```QLocale::Country QList<QLocale::Country>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::locale::Country {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_takeFirst(self as *mut ::list::ListLocaleCountry) }
  }

  /// C++ method: <span style='color: green;'>```QLocale::Country QList<QLocale::Country>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::locale::Country {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_takeLast(self as *mut ::list::ListLocaleCountry) }
  }

  /// C++ method: <span style='color: green;'>```QList<QLocale::Country>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::locale::Country```<br>
  /// C++ method: <span style='color: green;'>```QLocale::Country QList<QLocale::Country>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::locale::Country)) -> ::locale::Country```<br>
  /// C++ method: <span style='color: green;'>```QLocale::Country QList<QLocale::Country>::value(int i, const QLocale::Country& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::locale::Country
    where Args: overloading::ListLocaleCountryValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListLocaleCountry {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QLocale::Country>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QLocale_Country_destructor(self as *mut ::list::ListLocaleCountry) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QMimeType>```</span>
#[repr(C)]
pub struct ListMimeType([u8; ::type_sizes::QT_CORE_LIST_LIST_MIME_TYPE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListMimeType {
  unsafe fn new_uninitialized() -> ListMimeType {
    ListMimeType(::std::mem::uninitialized())
  }
}

impl ListMimeType {
  /// C++ method: <span style='color: green;'>```QList<QMimeType>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListMimeType) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::append(const QList<QMimeType>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::mime_type::MimeType) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::append(const QMimeType& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListMimeTypeAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QMimeType& QList<QMimeType>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_at(self as *const ::list::ListMimeType, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QMimeType& QList<QMimeType>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_back_const(self as *const ::list::ListMimeType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMimeType& QList<QMimeType>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_back(self as *mut ::list::ListMimeType) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_clear(self as *mut ::list::ListMimeType) }
  }

  /// C++ method: <span style='color: green;'>```const QMimeType& QList<QMimeType>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_constFirst(self as *const ::list::ListMimeType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QMimeType& QList<QMimeType>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_constLast(self as *const ::list::ListMimeType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMimeType>::contains(const QMimeType& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::mime_type::MimeType) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_contains(self as *const ::list::ListMimeType,
                                                t as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QMimeType>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QMimeType>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::mime_type::MimeType) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QMimeType>::count(const QMimeType& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListMimeTypeCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QMimeType>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_empty(self as *const ::list::ListMimeType) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMimeType>::endsWith(const QMimeType& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::mime_type::MimeType) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_endsWith(self as *const ::list::ListMimeType,
                                                t as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```const QMimeType& QList<QMimeType>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_first_const(self as *const ::list::ListMimeType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMimeType& QList<QMimeType>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_first(self as *mut ::list::ListMimeType) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QMimeType& QList<QMimeType>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_front_const(self as *const ::list::ListMimeType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMimeType& QList<QMimeType>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_front(self as *mut ::list::ListMimeType) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QMimeType>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::mime_type::MimeType) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QMimeType>::indexOf(const QMimeType& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::mime_type::MimeType, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QMimeType>::indexOf(const QMimeType& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListMimeTypeIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::insert(int i, const QMimeType& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::mime_type::MimeType) {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_insert(self as *mut ::list::ListMimeType,
                                              i,
                                              t as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMimeType>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_isEmpty(self as *const ::list::ListMimeType) }
  }

  /// C++ method: <span style='color: green;'>```const QMimeType& QList<QMimeType>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_last_const(self as *const ::list::ListMimeType) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QMimeType>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::mime_type::MimeType) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QMimeType>::lastIndexOf(const QMimeType& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::mime_type::MimeType, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QMimeType>::lastIndexOf(const QMimeType& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListMimeTypeLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMimeType& QList<QMimeType>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_last(self as *mut ::list::ListMimeType) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QMimeType>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_length(self as *const ::list::ListMimeType) }
  }

  /// C++ method: <span style='color: green;'>```QList<QMimeType>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListMimeType```<br>
  /// C++ method: <span style='color: green;'>```QList<QMimeType> QList<QMimeType>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListMimeType```<br>
  /// C++ method: <span style='color: green;'>```QList<QMimeType> QList<QMimeType>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListMimeType
    where Args: overloading::ListMimeTypeMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_move(self as *mut ::list::ListMimeType, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QMimeType>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListMimeType```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QMimeType>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListMimeType) -> ::list::ListMimeType```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QMimeType>::QList(const QList<QMimeType>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListMimeType
    where Args: overloading::ListMimeTypeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QMimeType> QList<QMimeType>::operator+(const QList<QMimeType>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListMimeType) -> ::list::ListMimeType {
    {
      let mut object: ::list::ListMimeType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_operator_add_to_output(self as *const ::list::ListMimeType,
                                                                l as *const ::list::ListMimeType,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QMimeType>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListMimeType) -> &'l0 mut ::list::ListMimeType```<br>
  /// C++ method: <span style='color: green;'>```QList<QMimeType>& QList<QMimeType>::operator+=(const QList<QMimeType>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::mime_type::MimeType) -> &'l0 mut ::list::ListMimeType```<br>
  /// C++ method: <span style='color: green;'>```QList<QMimeType>& QList<QMimeType>::operator+=(const QMimeType& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListMimeType
    where Args: overloading::ListMimeTypeOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QMimeType>& QList<QMimeType>::operator=(const QList<QMimeType>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListMimeType) -> &'l0 mut ::list::ListMimeType {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QMimeType_operator_assign(self as *mut ::list::ListMimeType,
                                                       l as *const ::list::ListMimeType)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMimeType>::operator==(const QList<QMimeType>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListMimeType) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_operator_eq(self as *const ::list::ListMimeType,
                                                   l as *const ::list::ListMimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```const QMimeType& QList<QMimeType>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::mime_type::MimeType {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QMimeType_operator_index_const(self as *const ::list::ListMimeType, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMimeType& QList<QMimeType>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::mime_type::MimeType {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QMimeType_operator_index(self as *mut ::list::ListMimeType, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMimeType>::operator!=(const QList<QMimeType>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListMimeType) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_operator_neq(self as *const ::list::ListMimeType,
                                                    l as *const ::list::ListMimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QMimeType>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListMimeType) -> &'l0 mut ::list::ListMimeType```<br>
  /// C++ method: <span style='color: green;'>```QList<QMimeType>& QList<QMimeType>::operator<<(const QList<QMimeType>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::mime_type::MimeType) -> &'l0 mut ::list::ListMimeType```<br>
  /// C++ method: <span style='color: green;'>```QList<QMimeType>& QList<QMimeType>::operator<<(const QMimeType& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListMimeType
    where Args: overloading::ListMimeTypeOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_pop_back(self as *mut ::list::ListMimeType) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_pop_front(self as *mut ::list::ListMimeType) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::prepend(const QMimeType& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::mime_type::MimeType) {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_prepend(self as *mut ::list::ListMimeType,
                                               t as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::push_back(const QMimeType& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::mime_type::MimeType) {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_push_back(self as *mut ::list::ListMimeType,
                                                 t as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::push_front(const QMimeType& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::mime_type::MimeType) {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_push_front(self as *mut ::list::ListMimeType,
                                                  t as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QMimeType>::removeAll(const QMimeType& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::mime_type::MimeType) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_removeAll(self as *mut ::list::ListMimeType,
                                                 t as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_removeAt(self as *mut ::list::ListMimeType, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_removeFirst(self as *mut ::list::ListMimeType) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_removeLast(self as *mut ::list::ListMimeType) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMimeType>::removeOne(const QMimeType& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::mime_type::MimeType) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_removeOne(self as *mut ::list::ListMimeType,
                                                 t as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::replace(int i, const QMimeType& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::mime_type::MimeType) {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_replace(self as *mut ::list::ListMimeType,
                                               i,
                                               t as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_reserve(self as *mut ::list::ListMimeType, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QMimeType>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_size(self as *const ::list::ListMimeType) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMimeType>::startsWith(const QMimeType& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::mime_type::MimeType) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QMimeType_startsWith(self as *const ::list::ListMimeType,
                                                  t as *const ::mime_type::MimeType)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QMimeType>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListMimeType) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::swap(QList<QMimeType>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QMimeType>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListMimeTypeSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMimeType QList<QMimeType>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::mime_type::MimeType {
    {
      let mut object: ::mime_type::MimeType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_takeAt_to_output(self as *mut ::list::ListMimeType, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMimeType QList<QMimeType>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::mime_type::MimeType {
    {
      let mut object: ::mime_type::MimeType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_takeFirst_to_output(self as *mut ::list::ListMimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QMimeType QList<QMimeType>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::mime_type::MimeType {
    {
      let mut object: ::mime_type::MimeType =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_takeLast_to_output(self as *mut ::list::ListMimeType, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QMimeType>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::mime_type::MimeType```<br>
  /// C++ method: <span style='color: green;'>```QMimeType QList<QMimeType>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::mime_type::MimeType)) -> ::mime_type::MimeType```<br>
  /// C++ method: <span style='color: green;'>```QMimeType QList<QMimeType>::value(int i, const QMimeType& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::mime_type::MimeType
    where Args: overloading::ListMimeTypeValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListMimeType {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QMimeType>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QMimeType_destructor(self as *mut ::list::ListMimeType) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QModelIndex>```</span>
#[repr(C)]
pub struct ListModelIndex([u8; ::type_sizes::QT_CORE_LIST_LIST_MODEL_INDEX]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListModelIndex {
  unsafe fn new_uninitialized() -> ListModelIndex {
    ListModelIndex(::std::mem::uninitialized())
  }
}

impl ListModelIndex {
  /// C++ method: <span style='color: green;'>```QList<QModelIndex>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::append(const QList<QModelIndex>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::model_index::ModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::append(const QModelIndex& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListModelIndexAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QModelIndex& QList<QModelIndex>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QModelIndex_at(self as *const ::list::ListModelIndex, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QModelIndex& QList<QModelIndex>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QModelIndex_back_const(self as *const ::list::ListModelIndex) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QModelIndex& QList<QModelIndex>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QModelIndex_back(self as *mut ::list::ListModelIndex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_clear(self as *mut ::list::ListModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```const QModelIndex& QList<QModelIndex>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QModelIndex_constFirst(self as *const ::list::ListModelIndex) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QModelIndex& QList<QModelIndex>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QModelIndex_constLast(self as *const ::list::ListModelIndex) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QModelIndex>::contains(const QModelIndex& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_contains(self as *const ::list::ListModelIndex,
                                                  t as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QModelIndex>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QModelIndex>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QModelIndex>::count(const QModelIndex& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListModelIndexCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QModelIndex>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_empty(self as *const ::list::ListModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QModelIndex>::endsWith(const QModelIndex& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_endsWith(self as *const ::list::ListModelIndex,
                                                  t as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```const QModelIndex& QList<QModelIndex>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QModelIndex_first_const(self as *const ::list::ListModelIndex) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QModelIndex& QList<QModelIndex>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QModelIndex_first(self as *mut ::list::ListModelIndex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QModelIndex& QList<QModelIndex>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QModelIndex_front_const(self as *const ::list::ListModelIndex) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QModelIndex& QList<QModelIndex>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QModelIndex_front(self as *mut ::list::ListModelIndex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QModelIndex>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QModelIndex>::indexOf(const QModelIndex& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::model_index::ModelIndex, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QModelIndex>::indexOf(const QModelIndex& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListModelIndexIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::insert(int i, const QModelIndex& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_insert(self as *mut ::list::ListModelIndex,
                                                i,
                                                t as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QModelIndex>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_isEmpty(self as *const ::list::ListModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```const QModelIndex& QList<QModelIndex>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QModelIndex_last_const(self as *const ::list::ListModelIndex) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QModelIndex>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::model_index::ModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QModelIndex>::lastIndexOf(const QModelIndex& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::model_index::ModelIndex, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QModelIndex>::lastIndexOf(const QModelIndex& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListModelIndexLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QModelIndex& QList<QModelIndex>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::model_index::ModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QModelIndex_last(self as *mut ::list::ListModelIndex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QModelIndex>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_length(self as *const ::list::ListModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```QList<QModelIndex>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QModelIndex> QList<QModelIndex>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QModelIndex> QList<QModelIndex>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListModelIndex
    where Args: overloading::ListModelIndexMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_move(self as *mut ::list::ListModelIndex, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QModelIndex>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QModelIndex>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListModelIndex) -> ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QModelIndex>::QList(const QList<QModelIndex>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListModelIndex
    where Args: overloading::ListModelIndexNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QModelIndex> QList<QModelIndex>::operator+(const QList<QModelIndex>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListModelIndex) -> ::list::ListModelIndex {
    {
      let mut object: ::list::ListModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_operator_add_to_output(self as *const ::list::ListModelIndex,
                                                                  l as *const ::list::ListModelIndex,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QModelIndex>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListModelIndex) -> &'l0 mut ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QModelIndex>& QList<QModelIndex>::operator+=(const QList<QModelIndex>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::model_index::ModelIndex) -> &'l0 mut ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QModelIndex>& QList<QModelIndex>::operator+=(const QModelIndex& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListModelIndex
    where Args: overloading::ListModelIndexOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QModelIndex>& QList<QModelIndex>::operator=(const QList<QModelIndex>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListModelIndex) -> &'l0 mut ::list::ListModelIndex {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_operator_assign(self as *mut ::list::ListModelIndex,
                                                         l as *const ::list::ListModelIndex)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QModelIndex>::operator==(const QList<QModelIndex>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_operator_eq(self as *const ::list::ListModelIndex,
                                                     l as *const ::list::ListModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```const QModelIndex& QList<QModelIndex>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::model_index::ModelIndex {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QModelIndex_operator_index_const(self as *const ::list::ListModelIndex, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QModelIndex& QList<QModelIndex>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::model_index::ModelIndex {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QModelIndex_operator_index(self as *mut ::list::ListModelIndex, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QModelIndex>::operator!=(const QList<QModelIndex>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_operator_neq(self as *const ::list::ListModelIndex,
                                                      l as *const ::list::ListModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QModelIndex>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListModelIndex) -> &'l0 mut ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QModelIndex>& QList<QModelIndex>::operator<<(const QList<QModelIndex>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::model_index::ModelIndex) -> &'l0 mut ::list::ListModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QModelIndex>& QList<QModelIndex>::operator<<(const QModelIndex& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListModelIndex
    where Args: overloading::ListModelIndexOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_pop_back(self as *mut ::list::ListModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_pop_front(self as *mut ::list::ListModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::prepend(const QModelIndex& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_prepend(self as *mut ::list::ListModelIndex,
                                                 t as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::push_back(const QModelIndex& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_push_back(self as *mut ::list::ListModelIndex,
                                                   t as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::push_front(const QModelIndex& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_push_front(self as *mut ::list::ListModelIndex,
                                                    t as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QModelIndex>::removeAll(const QModelIndex& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::model_index::ModelIndex) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_removeAll(self as *mut ::list::ListModelIndex,
                                                   t as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_removeAt(self as *mut ::list::ListModelIndex, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_removeFirst(self as *mut ::list::ListModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_removeLast(self as *mut ::list::ListModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QModelIndex>::removeOne(const QModelIndex& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_removeOne(self as *mut ::list::ListModelIndex,
                                                   t as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::replace(int i, const QModelIndex& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::model_index::ModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_replace(self as *mut ::list::ListModelIndex,
                                                 i,
                                                 t as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_reserve(self as *mut ::list::ListModelIndex, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QModelIndex>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_size(self as *const ::list::ListModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QModelIndex>::startsWith(const QModelIndex& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::model_index::ModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QModelIndex_startsWith(self as *const ::list::ListModelIndex,
                                                    t as *const ::model_index::ModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QModelIndex>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::swap(QList<QModelIndex>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QModelIndex>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListModelIndexSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QModelIndex QList<QModelIndex>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_takeAt_to_output(self as *mut ::list::ListModelIndex, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QList<QModelIndex>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_takeFirst_to_output(self as *mut ::list::ListModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QModelIndex QList<QModelIndex>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::model_index::ModelIndex {
    {
      let mut object: ::model_index::ModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_takeLast_to_output(self as *mut ::list::ListModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QModelIndex>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QModelIndex QList<QModelIndex>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::model_index::ModelIndex)) -> ::model_index::ModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QModelIndex QList<QModelIndex>::value(int i, const QModelIndex& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::model_index::ModelIndex
    where Args: overloading::ListModelIndexValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListModelIndex {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QModelIndex>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QModelIndex_destructor(self as *mut ::list::ListModelIndex) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QObject*>```</span>
#[repr(C)]
pub struct ListObjectMutPtr([u8; ::type_sizes::QT_CORE_LIST_LIST_OBJECT_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListObjectMutPtr {
  unsafe fn new_uninitialized() -> ListObjectMutPtr {
    ListObjectMutPtr(::std::mem::uninitialized())
  }
}

impl ListObjectMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QObject*>::append(const QList<QObject*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListObjectMutPtr) {
    unsafe {
      ::ffi::qt_core_c_QList_QObject_ptr_append_QList_QObject_ptr(self as *mut ::list::ListObjectMutPtr,
                                                                  t as *const ::list::ListObjectMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::append(QObject* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::object::Object) {
    ::ffi::qt_core_c_QList_QObject_ptr_append_QObject(self as *mut ::list::ListObjectMutPtr,
                                                      t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```QObject* const & QList<QObject*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QObject_ptr_at(self as *const ::list::ListObjectMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QObject* const & QList<QObject*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QObject_ptr_back_const(self as *const ::list::ListObjectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QObject*& QList<QObject*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QObject_ptr_back(self as *mut ::list::ListObjectMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_clear(self as *mut ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QObject* const & QList<QObject*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QObject_ptr_constFirst(self as *const ::list::ListObjectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QObject* const & QList<QObject*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QObject_ptr_constLast(self as *const ::list::ListObjectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QObject*>::contains(QObject* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::object::Object) -> bool {
    ::ffi::qt_core_c_QList_QObject_ptr_contains(self as *const ::list::ListObjectMutPtr,
                                                t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```int QList<QObject*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_count_no_args(self as *const ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QObject*>::count(QObject* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::object::Object) -> ::libc::c_int {
    ::ffi::qt_core_c_QList_QObject_ptr_count_t(self as *const ::list::ListObjectMutPtr,
                                               t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QObject*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_empty(self as *const ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QObject*>::endsWith(QObject* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::object::Object) -> bool {
    ::ffi::qt_core_c_QList_QObject_ptr_endsWith(self as *const ::list::ListObjectMutPtr,
                                                t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```QObject* const & QList<QObject*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QObject_ptr_first_const(self as *const ::list::ListObjectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QObject*& QList<QObject*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QObject_ptr_first(self as *mut ::list::ListObjectMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QObject* const & QList<QObject*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QObject_ptr_front_const(self as *const ::list::ListObjectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QObject*& QList<QObject*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QObject_ptr_front(self as *mut ::list::ListObjectMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QObject*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::object::Object) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QObject*>::indexOf(QObject* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::object::Object, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QObject*>::indexOf(QObject* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListObjectMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QObject*>::insert(int i, QObject* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::object::Object) {
    ::ffi::qt_core_c_QList_QObject_ptr_insert(self as *mut ::list::ListObjectMutPtr,
                                              i,
                                              t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QObject*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_isEmpty(self as *const ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QObject* const & QList<QObject*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QObject_ptr_last_const(self as *const ::list::ListObjectMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QObject*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::object::Object) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QObject*>::lastIndexOf(QObject* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::object::Object, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QObject*>::lastIndexOf(QObject* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListObjectMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QObject*& QList<QObject*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::object::Object {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QObject_ptr_last(self as *mut ::list::ListObjectMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QObject*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_length(self as *const ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QObject*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListObjectMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QObject*> QList<QObject*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListObjectMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QObject*> QList<QObject*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListObjectMutPtr
    where Args: overloading::ListObjectMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QObject*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_move(self as *mut ::list::ListObjectMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QObject*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListObjectMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QObject*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListObjectMutPtr) -> ::list::ListObjectMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QObject*>::QList(const QList<QObject*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListObjectMutPtr
    where Args: overloading::ListObjectMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QObject*> QList<QObject*>::operator+(const QList<QObject*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListObjectMutPtr) -> ::list::ListObjectMutPtr {
    {
      let mut object: ::list::ListObjectMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QObject_ptr_operator_add_to_output(self as *const ::list::ListObjectMutPtr,
                                                                  l as *const ::list::ListObjectMutPtr,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QObject*>& QList<QObject*>::operator+=(const QList<QObject*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListObjectMutPtr) -> &'l0 mut ::list::ListObjectMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QObject_ptr_operator_add_assign_l(self as *mut ::list::ListObjectMutPtr,
                                                               l as *const ::list::ListObjectMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QObject*>& QList<QObject*>::operator+=(QObject* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::object::Object)
                                               -> &'l0 mut ::list::ListObjectMutPtr {
    let ffi_result = ::ffi::qt_core_c_QList_QObject_ptr_operator_add_assign_t(self as *mut ::list::ListObjectMutPtr,
                                                                              t as *const *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QObject*>& QList<QObject*>::operator=(const QList<QObject*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListObjectMutPtr) -> &'l0 mut ::list::ListObjectMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QObject_ptr_operator_assign(self as *mut ::list::ListObjectMutPtr,
                                                         l as *const ::list::ListObjectMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QObject*>::operator==(const QList<QObject*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListObjectMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QObject_ptr_operator_eq(self as *const ::list::ListObjectMutPtr,
                                                     l as *const ::list::ListObjectMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QObject* const & QList<QObject*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QObject_ptr_operator_index_const(self as *const ::list::ListObjectMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QObject*& QList<QObject*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::object::Object {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QObject_ptr_operator_index(self as *mut ::list::ListObjectMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QObject*>::operator!=(const QList<QObject*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListObjectMutPtr) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QObject_ptr_operator_neq(self as *const ::list::ListObjectMutPtr,
                                                      l as *const ::list::ListObjectMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QObject*>& QList<QObject*>::operator<<(const QList<QObject*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListObjectMutPtr) -> &'l0 mut ::list::ListObjectMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QObject_ptr_operator_shl_l(self as *mut ::list::ListObjectMutPtr,
                                                        l as *const ::list::ListObjectMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QObject*>& QList<QObject*>::operator<<(QObject* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::object::Object)
                                        -> &'l0 mut ::list::ListObjectMutPtr {
    let ffi_result = ::ffi::qt_core_c_QList_QObject_ptr_operator_shl_t(self as *mut ::list::ListObjectMutPtr,
                                                                       t as *const *mut ::object::Object);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_pop_back(self as *mut ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_pop_front(self as *mut ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::prepend(QObject* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::object::Object) {
    ::ffi::qt_core_c_QList_QObject_ptr_prepend(self as *mut ::list::ListObjectMutPtr,
                                               t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::push_back(QObject* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::object::Object) {
    ::ffi::qt_core_c_QList_QObject_ptr_push_back(self as *mut ::list::ListObjectMutPtr,
                                                 t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::push_front(QObject* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::object::Object) {
    ::ffi::qt_core_c_QList_QObject_ptr_push_front(self as *mut ::list::ListObjectMutPtr,
                                                  t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```int QList<QObject*>::removeAll(QObject* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::object::Object) -> ::libc::c_int {
    ::ffi::qt_core_c_QList_QObject_ptr_removeAll(self as *mut ::list::ListObjectMutPtr,
                                                 t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_removeAt(self as *mut ::list::ListObjectMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_removeFirst(self as *mut ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_removeLast(self as *mut ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QObject*>::removeOne(QObject* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::object::Object) -> bool {
    ::ffi::qt_core_c_QList_QObject_ptr_removeOne(self as *mut ::list::ListObjectMutPtr,
                                                 t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::replace(int i, QObject* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::object::Object) {
    ::ffi::qt_core_c_QList_QObject_ptr_replace(self as *mut ::list::ListObjectMutPtr,
                                               i,
                                               t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```void QList<QObject*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_reserve(self as *mut ::list::ListObjectMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QObject*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_size(self as *const ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QObject*>::startsWith(QObject* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::object::Object) -> bool {
    ::ffi::qt_core_c_QList_QObject_ptr_startsWith(self as *const ::list::ListObjectMutPtr,
                                                  t as *const *mut ::object::Object)
  }

  /// C++ method: <span style='color: green;'>```QList<QObject*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListObjectMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QObject*>::swap(QList<QObject*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QObject*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListObjectMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QObject* QList<QObject*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_takeAt(self as *mut ::list::ListObjectMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QObject* QList<QObject*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_takeFirst(self as *mut ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QObject* QList<QObject*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_takeLast(self as *mut ::list::ListObjectMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QObject* QList<QObject*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::object::Object {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_value_i(self as *const ::list::ListObjectMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QObject* QList<QObject*>::value(int i, QObject* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self, i: ::libc::c_int, default_value: &*mut ::object::Object) -> *mut ::object::Object {
    ::ffi::qt_core_c_QList_QObject_ptr_value_i_defaultValue(self as *const ::list::ListObjectMutPtr,
                                                            i,
                                                            default_value as *const *mut ::object::Object)
  }
}

impl Drop for ::list::ListObjectMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QObject*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QObject_ptr_destructor(self as *mut ::list::ListObjectMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QPair<QString, QString>>```</span>
#[repr(C)]
pub struct ListPairPairStringString([u8; ::type_sizes::QT_CORE_LIST_LIST_PAIR_PAIR_STRING_STRING]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListPairPairStringString {
  unsafe fn new_uninitialized() -> ListPairPairStringString {
    ListPairPairStringString(::std::mem::uninitialized())
  }
}

impl ListPairPairStringString {
  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListPairPairStringString) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::append(const QList<QPair<QString, QString>>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::pair::PairStringString) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::append(const QPair<QString, QString>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListPairPairStringStringAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPair<QString, QString>& QList<QPair<QString, QString>>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::pair::PairStringString {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_at(self as *const ::list::ListPairPairStringString, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<QString, QString>& QList<QPair<QString, QString>>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::pair::PairStringString {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_back_const(self as *const ::list::ListPairPairStringString)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<QString, QString>& QList<QPair<QString, QString>>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairStringString {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_back(self as *mut ::list::ListPairPairStringString) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_clear(self as *mut ::list::ListPairPairStringString) }
  }

  /// C++ method: <span style='color: green;'>```const QPair<QString, QString>& QList<QPair<QString, QString>>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::pair::PairStringString {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_constFirst(self as *const ::list::ListPairPairStringString)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<QString, QString>& QList<QPair<QString, QString>>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::pair::PairStringString {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_constLast(self as *const ::list::ListPairPairStringString)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<QString, QString>>::contains(const QPair<QString, QString>& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::pair::PairStringString) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_contains(self as *const ::list::ListPairPairStringString,
                                                            t as *const ::pair::PairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<QString, QString>>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::pair::PairStringString) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<QString, QString>>::count(const QPair<QString, QString>& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPairPairStringStringCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QPair<QString, QString>>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_empty(self as *const ::list::ListPairPairStringString) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<QString, QString>>::endsWith(const QPair<QString, QString>& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::pair::PairStringString) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_endsWith(self as *const ::list::ListPairPairStringString,
                                                            t as *const ::pair::PairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<QString, QString>& QList<QPair<QString, QString>>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::pair::PairStringString {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_first_const(self as *const ::list::ListPairPairStringString)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<QString, QString>& QList<QPair<QString, QString>>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairStringString {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_first(self as *mut ::list::ListPairPairStringString) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<QString, QString>& QList<QPair<QString, QString>>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::pair::PairStringString {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_front_const(self as *const ::list::ListPairPairStringString)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<QString, QString>& QList<QPair<QString, QString>>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairStringString {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_front(self as *mut ::list::ListPairPairStringString) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::pair::PairStringString) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<QString, QString>>::indexOf(const QPair<QString, QString>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::pair::PairStringString, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<QString, QString>>::indexOf(const QPair<QString, QString>& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPairPairStringStringIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::insert(int i, const QPair<QString, QString>& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::pair::PairStringString) {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_insert(self as *mut ::list::ListPairPairStringString,
                                                          i,
                                                          t as *const ::pair::PairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<QString, QString>>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_isEmpty(self as *const ::list::ListPairPairStringString) }
  }

  /// C++ method: <span style='color: green;'>```const QPair<QString, QString>& QList<QPair<QString, QString>>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::pair::PairStringString {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_last_const(self as *const ::list::ListPairPairStringString)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::pair::PairStringString) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<QString, QString>>::lastIndexOf(const QPair<QString, QString>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::pair::PairStringString, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<QString, QString>>::lastIndexOf(const QPair<QString, QString>& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPairPairStringStringLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPair<QString, QString>& QList<QPair<QString, QString>>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairStringString {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_last(self as *mut ::list::ListPairPairStringString) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QPair<QString, QString>>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_length(self as *const ::list::ListPairPairStringString) }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListPairPairStringString```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>> QList<QPair<QString, QString>>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListPairPairStringString```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>> QList<QPair<QString, QString>>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListPairPairStringString
    where Args: overloading::ListPairPairStringStringMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_move(self as *mut ::list::ListPairPairStringString, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListPairPairStringString```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QPair<QString, QString>>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListPairPairStringString) -> ::list::ListPairPairStringString```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QPair<QString, QString>>::QList(const QList<QPair<QString, QString>>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListPairPairStringString
    where Args: overloading::ListPairPairStringStringNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>> QList<QPair<QString, QString>>::operator+(const QList<QPair<QString, QString>>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListPairPairStringString) -> ::list::ListPairPairStringString {
    {
      let mut object: ::list::ListPairPairStringString =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_operator_add_to_output(self as *const ::list::ListPairPairStringString, l as *const ::list::ListPairPairStringString, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListPairPairStringString) -> &'l0 mut ::list::ListPairPairStringString```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>& QList<QPair<QString, QString>>::operator+=(const QList<QPair<QString, QString>>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::pair::PairStringString) -> &'l0 mut ::list::ListPairPairStringString```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>& QList<QPair<QString, QString>>::operator+=(const QPair<QString, QString>& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListPairPairStringString
    where Args: overloading::ListPairPairStringStringOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>& QList<QPair<QString, QString>>::operator=(const QList<QPair<QString, QString>>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListPairPairStringString)
                             -> &'l0 mut ::list::ListPairPairStringString {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_operator_assign(self as *mut ::list::ListPairPairStringString,
                                                                     l as *const ::list::ListPairPairStringString)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<QString, QString>>::operator==(const QList<QPair<QString, QString>>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListPairPairStringString) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_operator_eq(self as *const ::list::ListPairPairStringString,
                                                               l as *const ::list::ListPairPairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<QString, QString>& QList<QPair<QString, QString>>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::pair::PairStringString {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_operator_index_const(self as *const ::list::ListPairPairStringString, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<QString, QString>& QList<QPair<QString, QString>>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::pair::PairStringString {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_operator_index(self as *mut ::list::ListPairPairStringString, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<QString, QString>>::operator!=(const QList<QPair<QString, QString>>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListPairPairStringString) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_operator_neq(self as *const ::list::ListPairPairStringString,
                                                                l as *const ::list::ListPairPairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListPairPairStringString) -> &'l0 mut ::list::ListPairPairStringString```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>& QList<QPair<QString, QString>>::operator<<(const QList<QPair<QString, QString>>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::pair::PairStringString) -> &'l0 mut ::list::ListPairPairStringString```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>& QList<QPair<QString, QString>>::operator<<(const QPair<QString, QString>& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListPairPairStringString
    where Args: overloading::ListPairPairStringStringOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_pop_back(self as *mut ::list::ListPairPairStringString) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_pop_front(self as *mut ::list::ListPairPairStringString) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::prepend(const QPair<QString, QString>& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::pair::PairStringString) {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_prepend(self as *mut ::list::ListPairPairStringString,
                                                           t as *const ::pair::PairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::push_back(const QPair<QString, QString>& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::pair::PairStringString) {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_push_back(self as *mut ::list::ListPairPairStringString,
                                                             t as *const ::pair::PairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::push_front(const QPair<QString, QString>& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::pair::PairStringString) {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_push_front(self as *mut ::list::ListPairPairStringString,
                                                              t as *const ::pair::PairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QPair<QString, QString>>::removeAll(const QPair<QString, QString>& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::pair::PairStringString) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_removeAll(self as *mut ::list::ListPairPairStringString,
                                                             t as *const ::pair::PairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_removeAt(self as *mut ::list::ListPairPairStringString, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_removeFirst(self as *mut ::list::ListPairPairStringString) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_removeLast(self as *mut ::list::ListPairPairStringString) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<QString, QString>>::removeOne(const QPair<QString, QString>& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::pair::PairStringString) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_removeOne(self as *mut ::list::ListPairPairStringString,
                                                             t as *const ::pair::PairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::replace(int i, const QPair<QString, QString>& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::pair::PairStringString) {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_replace(self as *mut ::list::ListPairPairStringString,
                                                           i,
                                                           t as *const ::pair::PairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_reserve(self as *mut ::list::ListPairPairStringString, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QPair<QString, QString>>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_size(self as *const ::list::ListPairPairStringString) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<QString, QString>>::startsWith(const QPair<QString, QString>& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::pair::PairStringString) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QPair_QString_QString_startsWith(self as *const ::list::ListPairPairStringString,
                                                              t as *const ::pair::PairStringString)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListPairPairStringString) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::swap(QList<QPair<QString, QString>>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<QString, QString>>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListPairPairStringStringSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPair<QString, QString> QList<QPair<QString, QString>>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::pair::PairStringString {
    {
      let mut object: ::pair::PairStringString =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_takeAt_to_output(self as *mut ::list::ListPairPairStringString,
                                                                      i,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<QString, QString> QList<QPair<QString, QString>>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::pair::PairStringString {
    {
      let mut object: ::pair::PairStringString =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_takeFirst_to_output(self as *mut ::list::ListPairPairStringString, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<QString, QString> QList<QPair<QString, QString>>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::pair::PairStringString {
    {
      let mut object: ::pair::PairStringString =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_takeLast_to_output(self as *mut ::list::ListPairPairStringString, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<QString, QString>>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::pair::PairStringString```<br>
  /// C++ method: <span style='color: green;'>```QPair<QString, QString> QList<QPair<QString, QString>>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::pair::PairStringString)) -> ::pair::PairStringString```<br>
  /// C++ method: <span style='color: green;'>```QPair<QString, QString> QList<QPair<QString, QString>>::value(int i, const QPair<QString, QString>& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::pair::PairStringString
    where Args: overloading::ListPairPairStringStringValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListPairPairStringString {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QPair<QString, QString>>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_destructor(self as *mut ::list::ListPairPairStringString) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QPersistentModelIndex>```</span>
#[repr(C)]
pub struct ListPersistentModelIndex([u8; ::type_sizes::QT_CORE_LIST_LIST_PERSISTENT_MODEL_INDEX]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListPersistentModelIndex {
  unsafe fn new_uninitialized() -> ListPersistentModelIndex {
    ListPersistentModelIndex(::std::mem::uninitialized())
  }
}

impl ListPersistentModelIndex {
  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListPersistentModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::append(const QList<QPersistentModelIndex>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::persistent_model_index::PersistentModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::append(const QPersistentModelIndex& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListPersistentModelIndexAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPersistentModelIndex& QList<QPersistentModelIndex>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_at(self as *const ::list::ListPersistentModelIndex, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPersistentModelIndex& QList<QPersistentModelIndex>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_back_const(self as *const ::list::ListPersistentModelIndex)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPersistentModelIndex& QList<QPersistentModelIndex>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_back(self as *mut ::list::ListPersistentModelIndex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_clear(self as *mut ::list::ListPersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```const QPersistentModelIndex& QList<QPersistentModelIndex>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_constFirst(self as *const ::list::ListPersistentModelIndex)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPersistentModelIndex& QList<QPersistentModelIndex>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_constLast(self as *const ::list::ListPersistentModelIndex)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPersistentModelIndex>::contains(const QPersistentModelIndex& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::persistent_model_index::PersistentModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QPersistentModelIndex_contains(self as *const ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPersistentModelIndex>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::persistent_model_index::PersistentModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPersistentModelIndex>::count(const QPersistentModelIndex& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPersistentModelIndexCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QPersistentModelIndex>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_empty(self as *const ::list::ListPersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPersistentModelIndex>::endsWith(const QPersistentModelIndex& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::persistent_model_index::PersistentModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QPersistentModelIndex_endsWith(self as *const ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPersistentModelIndex& QList<QPersistentModelIndex>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_first_const(self as *const ::list::ListPersistentModelIndex)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPersistentModelIndex& QList<QPersistentModelIndex>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_first(self as *mut ::list::ListPersistentModelIndex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPersistentModelIndex& QList<QPersistentModelIndex>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_front_const(self as *const ::list::ListPersistentModelIndex)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPersistentModelIndex& QList<QPersistentModelIndex>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_front(self as *mut ::list::ListPersistentModelIndex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::persistent_model_index::PersistentModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPersistentModelIndex>::indexOf(const QPersistentModelIndex& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::persistent_model_index::PersistentModelIndex, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPersistentModelIndex>::indexOf(const QPersistentModelIndex& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPersistentModelIndexIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::insert(int i, const QPersistentModelIndex& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::persistent_model_index::PersistentModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QList_QPersistentModelIndex_insert(self as *mut ::list::ListPersistentModelIndex,
                                                          i,
                                                          t as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPersistentModelIndex>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_isEmpty(self as *const ::list::ListPersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```const QPersistentModelIndex& QList<QPersistentModelIndex>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_last_const(self as *const ::list::ListPersistentModelIndex)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::persistent_model_index::PersistentModelIndex) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPersistentModelIndex>::lastIndexOf(const QPersistentModelIndex& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::persistent_model_index::PersistentModelIndex, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPersistentModelIndex>::lastIndexOf(const QPersistentModelIndex& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPersistentModelIndexLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPersistentModelIndex& QList<QPersistentModelIndex>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_last(self as *mut ::list::ListPersistentModelIndex) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QPersistentModelIndex>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_length(self as *const ::list::ListPersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListPersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex> QList<QPersistentModelIndex>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListPersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex> QList<QPersistentModelIndex>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListPersistentModelIndex
    where Args: overloading::ListPersistentModelIndexMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_core_c_QList_QPersistentModelIndex_move(self as *mut ::list::ListPersistentModelIndex, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListPersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QPersistentModelIndex>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListPersistentModelIndex) -> ::list::ListPersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QPersistentModelIndex>::QList(const QList<QPersistentModelIndex>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListPersistentModelIndex
    where Args: overloading::ListPersistentModelIndexNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex> QList<QPersistentModelIndex>::operator+(const QList<QPersistentModelIndex>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListPersistentModelIndex) -> ::list::ListPersistentModelIndex {
    {
      let mut object: ::list::ListPersistentModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_operator_add_to_output(self as *const ::list::ListPersistentModelIndex, l as *const ::list::ListPersistentModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListPersistentModelIndex) -> &'l0 mut ::list::ListPersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>& QList<QPersistentModelIndex>::operator+=(const QList<QPersistentModelIndex>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::persistent_model_index::PersistentModelIndex) -> &'l0 mut ::list::ListPersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>& QList<QPersistentModelIndex>::operator+=(const QPersistentModelIndex& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListPersistentModelIndex
    where Args: overloading::ListPersistentModelIndexOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>& QList<QPersistentModelIndex>::operator=(const QList<QPersistentModelIndex>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListPersistentModelIndex)
                             -> &'l0 mut ::list::ListPersistentModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_operator_assign(self as *mut ::list::ListPersistentModelIndex,
                                                                     l as *const ::list::ListPersistentModelIndex)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPersistentModelIndex>::operator==(const QList<QPersistentModelIndex>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListPersistentModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QPersistentModelIndex_operator_eq(self as *const ::list::ListPersistentModelIndex,
                                                               l as *const ::list::ListPersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPersistentModelIndex& QList<QPersistentModelIndex>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::persistent_model_index::PersistentModelIndex {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_operator_index_const(self as *const ::list::ListPersistentModelIndex, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPersistentModelIndex& QList<QPersistentModelIndex>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::persistent_model_index::PersistentModelIndex {
    let ffi_result =
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_operator_index(self as *mut ::list::ListPersistentModelIndex, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPersistentModelIndex>::operator!=(const QList<QPersistentModelIndex>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListPersistentModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QPersistentModelIndex_operator_neq(self as *const ::list::ListPersistentModelIndex,
                                                                l as *const ::list::ListPersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListPersistentModelIndex) -> &'l0 mut ::list::ListPersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>& QList<QPersistentModelIndex>::operator<<(const QList<QPersistentModelIndex>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::persistent_model_index::PersistentModelIndex) -> &'l0 mut ::list::ListPersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>& QList<QPersistentModelIndex>::operator<<(const QPersistentModelIndex& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListPersistentModelIndex
    where Args: overloading::ListPersistentModelIndexOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_pop_back(self as *mut ::list::ListPersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_pop_front(self as *mut ::list::ListPersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::prepend(const QPersistentModelIndex& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::persistent_model_index::PersistentModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QList_QPersistentModelIndex_prepend(self as *mut ::list::ListPersistentModelIndex,
                                                           t as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::push_back(const QPersistentModelIndex& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::persistent_model_index::PersistentModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QList_QPersistentModelIndex_push_back(self as *mut ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::push_front(const QPersistentModelIndex& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::persistent_model_index::PersistentModelIndex) {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_push_front(self as *mut ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QPersistentModelIndex>::removeAll(const QPersistentModelIndex& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::persistent_model_index::PersistentModelIndex) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QPersistentModelIndex_removeAll(self as *mut ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_removeAt(self as *mut ::list::ListPersistentModelIndex, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_removeFirst(self as *mut ::list::ListPersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_removeLast(self as *mut ::list::ListPersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPersistentModelIndex>::removeOne(const QPersistentModelIndex& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::persistent_model_index::PersistentModelIndex) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QPersistentModelIndex_removeOne(self as *mut ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::replace(int i, const QPersistentModelIndex& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::persistent_model_index::PersistentModelIndex) {
    unsafe {
      ::ffi::qt_core_c_QList_QPersistentModelIndex_replace(self as *mut ::list::ListPersistentModelIndex,
                                                           i,
                                                           t as *const ::persistent_model_index::PersistentModelIndex)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_reserve(self as *mut ::list::ListPersistentModelIndex, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QPersistentModelIndex>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_size(self as *const ::list::ListPersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPersistentModelIndex>::startsWith(const QPersistentModelIndex& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::persistent_model_index::PersistentModelIndex) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_startsWith(self as *const ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex) }
  }

  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListPersistentModelIndex) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::swap(QList<QPersistentModelIndex>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPersistentModelIndex>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListPersistentModelIndexSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPersistentModelIndex QList<QPersistentModelIndex>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::persistent_model_index::PersistentModelIndex {
    {
      let mut object: ::persistent_model_index::PersistentModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_takeAt_to_output(self as *mut ::list::ListPersistentModelIndex,
                                                                      i,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPersistentModelIndex QList<QPersistentModelIndex>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::persistent_model_index::PersistentModelIndex {
    {
      let mut object: ::persistent_model_index::PersistentModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_takeFirst_to_output(self as *mut ::list::ListPersistentModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPersistentModelIndex QList<QPersistentModelIndex>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::persistent_model_index::PersistentModelIndex {
    {
      let mut object: ::persistent_model_index::PersistentModelIndex =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_takeLast_to_output(self as *mut ::list::ListPersistentModelIndex, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPersistentModelIndex>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::persistent_model_index::PersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QPersistentModelIndex QList<QPersistentModelIndex>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::persistent_model_index::PersistentModelIndex)) -> ::persistent_model_index::PersistentModelIndex```<br>
  /// C++ method: <span style='color: green;'>```QPersistentModelIndex QList<QPersistentModelIndex>::value(int i, const QPersistentModelIndex& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::persistent_model_index::PersistentModelIndex
    where Args: overloading::ListPersistentModelIndexValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListPersistentModelIndex {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QPersistentModelIndex>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_destructor(self as *mut ::list::ListPersistentModelIndex) }
  }
}

/// C++ type: <span style='color: green;'>```QList<Qt::DayOfWeek>```</span>
#[repr(C)]
pub struct ListQtDayOfWeek([u8; ::type_sizes::QT_CORE_LIST_LIST_QT_DAY_OF_WEEK]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListQtDayOfWeek {
  unsafe fn new_uninitialized() -> ListQtDayOfWeek {
    ListQtDayOfWeek(::std::mem::uninitialized())
  }
}

impl ListQtDayOfWeek {
  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListQtDayOfWeek) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::append(const QList<Qt::DayOfWeek>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::qt::DayOfWeek) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::append(const Qt::DayOfWeek& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListQtDayOfWeekAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const Qt::DayOfWeek& QList<Qt::DayOfWeek>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt::DayOfWeek {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_at(self as *const ::list::ListQtDayOfWeek, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt::DayOfWeek& QList<Qt::DayOfWeek>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::qt::DayOfWeek {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_back_const(self as *const ::list::ListQtDayOfWeek) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::DayOfWeek& QList<Qt::DayOfWeek>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt::DayOfWeek {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_back(self as *mut ::list::ListQtDayOfWeek) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_clear(self as *mut ::list::ListQtDayOfWeek) }
  }

  /// C++ method: <span style='color: green;'>```const Qt::DayOfWeek& QList<Qt::DayOfWeek>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::qt::DayOfWeek {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_constFirst(self as *const ::list::ListQtDayOfWeek) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt::DayOfWeek& QList<Qt::DayOfWeek>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::qt::DayOfWeek {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_constLast(self as *const ::list::ListQtDayOfWeek) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<Qt::DayOfWeek>::contains(const Qt::DayOfWeek& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::qt::DayOfWeek) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_contains(self as *const ::list::ListQtDayOfWeek,
                                                   t as *const ::qt::DayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<Qt::DayOfWeek>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt::DayOfWeek) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<Qt::DayOfWeek>::count(const Qt::DayOfWeek& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListQtDayOfWeekCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<Qt::DayOfWeek>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_empty(self as *const ::list::ListQtDayOfWeek) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<Qt::DayOfWeek>::endsWith(const Qt::DayOfWeek& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::qt::DayOfWeek) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_endsWith(self as *const ::list::ListQtDayOfWeek,
                                                   t as *const ::qt::DayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```const Qt::DayOfWeek& QList<Qt::DayOfWeek>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::qt::DayOfWeek {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_first_const(self as *const ::list::ListQtDayOfWeek) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::DayOfWeek& QList<Qt::DayOfWeek>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt::DayOfWeek {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_first(self as *mut ::list::ListQtDayOfWeek) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const Qt::DayOfWeek& QList<Qt::DayOfWeek>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::qt::DayOfWeek {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_front_const(self as *const ::list::ListQtDayOfWeek) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::DayOfWeek& QList<Qt::DayOfWeek>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt::DayOfWeek {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_front(self as *mut ::list::ListQtDayOfWeek) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::qt::DayOfWeek) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<Qt::DayOfWeek>::indexOf(const Qt::DayOfWeek& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::qt::DayOfWeek, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<Qt::DayOfWeek>::indexOf(const Qt::DayOfWeek& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListQtDayOfWeekIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::insert(int i, const Qt::DayOfWeek& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::qt::DayOfWeek) {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_insert(self as *mut ::list::ListQtDayOfWeek,
                                                 i,
                                                 t as *const ::qt::DayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<Qt::DayOfWeek>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_isEmpty(self as *const ::list::ListQtDayOfWeek) }
  }

  /// C++ method: <span style='color: green;'>```const Qt::DayOfWeek& QList<Qt::DayOfWeek>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::qt::DayOfWeek {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_last_const(self as *const ::list::ListQtDayOfWeek) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::qt::DayOfWeek) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<Qt::DayOfWeek>::lastIndexOf(const Qt::DayOfWeek& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::qt::DayOfWeek, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<Qt::DayOfWeek>::lastIndexOf(const Qt::DayOfWeek& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListQtDayOfWeekLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt::DayOfWeek& QList<Qt::DayOfWeek>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt::DayOfWeek {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_last(self as *mut ::list::ListQtDayOfWeek) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<Qt::DayOfWeek>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_length(self as *const ::list::ListQtDayOfWeek) }
  }

  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListQtDayOfWeek```<br>
  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek> QList<Qt::DayOfWeek>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListQtDayOfWeek```<br>
  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek> QList<Qt::DayOfWeek>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListQtDayOfWeek
    where Args: overloading::ListQtDayOfWeekMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_move(self as *mut ::list::ListQtDayOfWeek, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListQtDayOfWeek```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<Qt::DayOfWeek>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListQtDayOfWeek) -> ::list::ListQtDayOfWeek```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<Qt::DayOfWeek>::QList(const QList<Qt::DayOfWeek>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListQtDayOfWeek
    where Args: overloading::ListQtDayOfWeekNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek> QList<Qt::DayOfWeek>::operator+(const QList<Qt::DayOfWeek>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListQtDayOfWeek) -> ::list::ListQtDayOfWeek {
    {
      let mut object: ::list::ListQtDayOfWeek =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_Qt_DayOfWeek_operator_add_to_output(self as *const ::list::ListQtDayOfWeek,
                                                                   l as *const ::list::ListQtDayOfWeek,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListQtDayOfWeek) -> &'l0 mut ::list::ListQtDayOfWeek```<br>
  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>& QList<Qt::DayOfWeek>::operator+=(const QList<Qt::DayOfWeek>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt::DayOfWeek) -> &'l0 mut ::list::ListQtDayOfWeek```<br>
  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>& QList<Qt::DayOfWeek>::operator+=(const Qt::DayOfWeek& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListQtDayOfWeek
    where Args: overloading::ListQtDayOfWeekOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>& QList<Qt::DayOfWeek>::operator=(const QList<Qt::DayOfWeek>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListQtDayOfWeek) -> &'l0 mut ::list::ListQtDayOfWeek {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_operator_assign(self as *mut ::list::ListQtDayOfWeek,
                                                          l as *const ::list::ListQtDayOfWeek)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<Qt::DayOfWeek>::operator==(const QList<Qt::DayOfWeek>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListQtDayOfWeek) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_operator_eq(self as *const ::list::ListQtDayOfWeek,
                                                      l as *const ::list::ListQtDayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```const Qt::DayOfWeek& QList<Qt::DayOfWeek>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt::DayOfWeek {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_operator_index_const(self as *const ::list::ListQtDayOfWeek, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt::DayOfWeek& QList<Qt::DayOfWeek>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::qt::DayOfWeek {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_operator_index(self as *mut ::list::ListQtDayOfWeek, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<Qt::DayOfWeek>::operator!=(const QList<Qt::DayOfWeek>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListQtDayOfWeek) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_operator_neq(self as *const ::list::ListQtDayOfWeek,
                                                       l as *const ::list::ListQtDayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListQtDayOfWeek) -> &'l0 mut ::list::ListQtDayOfWeek```<br>
  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>& QList<Qt::DayOfWeek>::operator<<(const QList<Qt::DayOfWeek>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::qt::DayOfWeek) -> &'l0 mut ::list::ListQtDayOfWeek```<br>
  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>& QList<Qt::DayOfWeek>::operator<<(const Qt::DayOfWeek& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListQtDayOfWeek
    where Args: overloading::ListQtDayOfWeekOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_pop_back(self as *mut ::list::ListQtDayOfWeek) }
  }

  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_pop_front(self as *mut ::list::ListQtDayOfWeek) }
  }

  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::prepend(const Qt::DayOfWeek& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::qt::DayOfWeek) {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_prepend(self as *mut ::list::ListQtDayOfWeek,
                                                  t as *const ::qt::DayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::push_back(const Qt::DayOfWeek& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::qt::DayOfWeek) {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_push_back(self as *mut ::list::ListQtDayOfWeek,
                                                    t as *const ::qt::DayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::push_front(const Qt::DayOfWeek& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::qt::DayOfWeek) {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_push_front(self as *mut ::list::ListQtDayOfWeek,
                                                     t as *const ::qt::DayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<Qt::DayOfWeek>::removeAll(const Qt::DayOfWeek& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::qt::DayOfWeek) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_removeAll(self as *mut ::list::ListQtDayOfWeek,
                                                    t as *const ::qt::DayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_removeAt(self as *mut ::list::ListQtDayOfWeek, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_removeFirst(self as *mut ::list::ListQtDayOfWeek) }
  }

  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_removeLast(self as *mut ::list::ListQtDayOfWeek) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<Qt::DayOfWeek>::removeOne(const Qt::DayOfWeek& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::qt::DayOfWeek) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_removeOne(self as *mut ::list::ListQtDayOfWeek,
                                                    t as *const ::qt::DayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::replace(int i, const Qt::DayOfWeek& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::qt::DayOfWeek) {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_replace(self as *mut ::list::ListQtDayOfWeek,
                                                  i,
                                                  t as *const ::qt::DayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_reserve(self as *mut ::list::ListQtDayOfWeek, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<Qt::DayOfWeek>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_size(self as *const ::list::ListQtDayOfWeek) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<Qt::DayOfWeek>::startsWith(const Qt::DayOfWeek& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::qt::DayOfWeek) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_Qt_DayOfWeek_startsWith(self as *const ::list::ListQtDayOfWeek,
                                                     t as *const ::qt::DayOfWeek)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListQtDayOfWeek) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::swap(QList<Qt::DayOfWeek>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<Qt::DayOfWeek>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListQtDayOfWeekSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt::DayOfWeek QList<Qt::DayOfWeek>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::qt::DayOfWeek {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_takeAt(self as *mut ::list::ListQtDayOfWeek, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt::DayOfWeek QList<Qt::DayOfWeek>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::qt::DayOfWeek {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_takeFirst(self as *mut ::list::ListQtDayOfWeek) }
  }

  /// C++ method: <span style='color: green;'>```Qt::DayOfWeek QList<Qt::DayOfWeek>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::qt::DayOfWeek {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_takeLast(self as *mut ::list::ListQtDayOfWeek) }
  }

  /// C++ method: <span style='color: green;'>```QList<Qt::DayOfWeek>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::qt::DayOfWeek```<br>
  /// C++ method: <span style='color: green;'>```Qt::DayOfWeek QList<Qt::DayOfWeek>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::qt::DayOfWeek)) -> ::qt::DayOfWeek```<br>
  /// C++ method: <span style='color: green;'>```Qt::DayOfWeek QList<Qt::DayOfWeek>::value(int i, const Qt::DayOfWeek& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::qt::DayOfWeek
    where Args: overloading::ListQtDayOfWeekValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListQtDayOfWeek {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<Qt::DayOfWeek>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_destructor(self as *mut ::list::ListQtDayOfWeek) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QStorageInfo>```</span>
#[repr(C)]
pub struct ListStorageInfo([u8; ::type_sizes::QT_CORE_LIST_LIST_STORAGE_INFO]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListStorageInfo {
  unsafe fn new_uninitialized() -> ListStorageInfo {
    ListStorageInfo(::std::mem::uninitialized())
  }
}

impl ListStorageInfo {
  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListStorageInfo) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::append(const QList<QStorageInfo>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::storage_info::StorageInfo) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::append(const QStorageInfo& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListStorageInfoAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QStorageInfo& QList<QStorageInfo>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::storage_info::StorageInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QStorageInfo_at(self as *const ::list::ListStorageInfo, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QStorageInfo& QList<QStorageInfo>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::storage_info::StorageInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QStorageInfo_back_const(self as *const ::list::ListStorageInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStorageInfo& QList<QStorageInfo>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::storage_info::StorageInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QStorageInfo_back(self as *mut ::list::ListStorageInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_clear(self as *mut ::list::ListStorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```const QStorageInfo& QList<QStorageInfo>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::storage_info::StorageInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QStorageInfo_constFirst(self as *const ::list::ListStorageInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QStorageInfo& QList<QStorageInfo>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::storage_info::StorageInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QStorageInfo_constLast(self as *const ::list::ListStorageInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStorageInfo>::contains(const QStorageInfo& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::storage_info::StorageInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_contains(self as *const ::list::ListStorageInfo,
                                                   t as *const ::storage_info::StorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QStorageInfo>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::storage_info::StorageInfo) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QStorageInfo>::count(const QStorageInfo& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListStorageInfoCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QStorageInfo>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_empty(self as *const ::list::ListStorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStorageInfo>::endsWith(const QStorageInfo& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::storage_info::StorageInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_endsWith(self as *const ::list::ListStorageInfo,
                                                   t as *const ::storage_info::StorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```const QStorageInfo& QList<QStorageInfo>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::storage_info::StorageInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QStorageInfo_first_const(self as *const ::list::ListStorageInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStorageInfo& QList<QStorageInfo>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::storage_info::StorageInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QStorageInfo_first(self as *mut ::list::ListStorageInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QStorageInfo& QList<QStorageInfo>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::storage_info::StorageInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QStorageInfo_front_const(self as *const ::list::ListStorageInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStorageInfo& QList<QStorageInfo>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::storage_info::StorageInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QStorageInfo_front(self as *mut ::list::ListStorageInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::storage_info::StorageInfo) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QStorageInfo>::indexOf(const QStorageInfo& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::storage_info::StorageInfo, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QStorageInfo>::indexOf(const QStorageInfo& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListStorageInfoIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::insert(int i, const QStorageInfo& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::storage_info::StorageInfo) {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_insert(self as *mut ::list::ListStorageInfo,
                                                 i,
                                                 t as *const ::storage_info::StorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStorageInfo>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_isEmpty(self as *const ::list::ListStorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```const QStorageInfo& QList<QStorageInfo>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::storage_info::StorageInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QStorageInfo_last_const(self as *const ::list::ListStorageInfo) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::storage_info::StorageInfo) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QStorageInfo>::lastIndexOf(const QStorageInfo& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::storage_info::StorageInfo, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QStorageInfo>::lastIndexOf(const QStorageInfo& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListStorageInfoLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStorageInfo& QList<QStorageInfo>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::storage_info::StorageInfo {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QStorageInfo_last(self as *mut ::list::ListStorageInfo) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QStorageInfo>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_length(self as *const ::list::ListStorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListStorageInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QStorageInfo> QList<QStorageInfo>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListStorageInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QStorageInfo> QList<QStorageInfo>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListStorageInfo
    where Args: overloading::ListStorageInfoMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_move(self as *mut ::list::ListStorageInfo, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListStorageInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QStorageInfo>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListStorageInfo) -> ::list::ListStorageInfo```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QStorageInfo>::QList(const QList<QStorageInfo>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListStorageInfo
    where Args: overloading::ListStorageInfoNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QStorageInfo> QList<QStorageInfo>::operator+(const QList<QStorageInfo>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListStorageInfo) -> ::list::ListStorageInfo {
    {
      let mut object: ::list::ListStorageInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_operator_add_to_output(self as *const ::list::ListStorageInfo,
                                                                   l as *const ::list::ListStorageInfo,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListStorageInfo) -> &'l0 mut ::list::ListStorageInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>& QList<QStorageInfo>::operator+=(const QList<QStorageInfo>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::storage_info::StorageInfo) -> &'l0 mut ::list::ListStorageInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>& QList<QStorageInfo>::operator+=(const QStorageInfo& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListStorageInfo
    where Args: overloading::ListStorageInfoOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>& QList<QStorageInfo>::operator=(const QList<QStorageInfo>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListStorageInfo) -> &'l0 mut ::list::ListStorageInfo {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_operator_assign(self as *mut ::list::ListStorageInfo,
                                                          l as *const ::list::ListStorageInfo)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStorageInfo>::operator==(const QList<QStorageInfo>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListStorageInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_operator_eq(self as *const ::list::ListStorageInfo,
                                                      l as *const ::list::ListStorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```const QStorageInfo& QList<QStorageInfo>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::storage_info::StorageInfo {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QStorageInfo_operator_index_const(self as *const ::list::ListStorageInfo, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStorageInfo& QList<QStorageInfo>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::storage_info::StorageInfo {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QStorageInfo_operator_index(self as *mut ::list::ListStorageInfo, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStorageInfo>::operator!=(const QList<QStorageInfo>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListStorageInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_operator_neq(self as *const ::list::ListStorageInfo,
                                                       l as *const ::list::ListStorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListStorageInfo) -> &'l0 mut ::list::ListStorageInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>& QList<QStorageInfo>::operator<<(const QList<QStorageInfo>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::storage_info::StorageInfo) -> &'l0 mut ::list::ListStorageInfo```<br>
  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>& QList<QStorageInfo>::operator<<(const QStorageInfo& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListStorageInfo
    where Args: overloading::ListStorageInfoOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_pop_back(self as *mut ::list::ListStorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_pop_front(self as *mut ::list::ListStorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::prepend(const QStorageInfo& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::storage_info::StorageInfo) {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_prepend(self as *mut ::list::ListStorageInfo,
                                                  t as *const ::storage_info::StorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::push_back(const QStorageInfo& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::storage_info::StorageInfo) {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_push_back(self as *mut ::list::ListStorageInfo,
                                                    t as *const ::storage_info::StorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::push_front(const QStorageInfo& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::storage_info::StorageInfo) {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_push_front(self as *mut ::list::ListStorageInfo,
                                                     t as *const ::storage_info::StorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QStorageInfo>::removeAll(const QStorageInfo& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::storage_info::StorageInfo) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_removeAll(self as *mut ::list::ListStorageInfo,
                                                    t as *const ::storage_info::StorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_removeAt(self as *mut ::list::ListStorageInfo, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_removeFirst(self as *mut ::list::ListStorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_removeLast(self as *mut ::list::ListStorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStorageInfo>::removeOne(const QStorageInfo& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::storage_info::StorageInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_removeOne(self as *mut ::list::ListStorageInfo,
                                                    t as *const ::storage_info::StorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::replace(int i, const QStorageInfo& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::storage_info::StorageInfo) {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_replace(self as *mut ::list::ListStorageInfo,
                                                  i,
                                                  t as *const ::storage_info::StorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_reserve(self as *mut ::list::ListStorageInfo, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QStorageInfo>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_size(self as *const ::list::ListStorageInfo) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStorageInfo>::startsWith(const QStorageInfo& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::storage_info::StorageInfo) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QStorageInfo_startsWith(self as *const ::list::ListStorageInfo,
                                                     t as *const ::storage_info::StorageInfo)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListStorageInfo) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::swap(QList<QStorageInfo>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QStorageInfo>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListStorageInfoSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStorageInfo QList<QStorageInfo>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::storage_info::StorageInfo {
    {
      let mut object: ::storage_info::StorageInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_takeAt_to_output(self as *mut ::list::ListStorageInfo, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStorageInfo QList<QStorageInfo>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::storage_info::StorageInfo {
    {
      let mut object: ::storage_info::StorageInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_takeFirst_to_output(self as *mut ::list::ListStorageInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QStorageInfo QList<QStorageInfo>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::storage_info::StorageInfo {
    {
      let mut object: ::storage_info::StorageInfo =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_takeLast_to_output(self as *mut ::list::ListStorageInfo, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QStorageInfo>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::storage_info::StorageInfo```<br>
  /// C++ method: <span style='color: green;'>```QStorageInfo QList<QStorageInfo>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::storage_info::StorageInfo)) -> ::storage_info::StorageInfo```<br>
  /// C++ method: <span style='color: green;'>```QStorageInfo QList<QStorageInfo>::value(int i, const QStorageInfo& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::storage_info::StorageInfo
    where Args: overloading::ListStorageInfoValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListStorageInfo {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QStorageInfo>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QStorageInfo_destructor(self as *mut ::list::ListStorageInfo) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QString>```</span>
#[repr(C)]
pub struct ListString([u8; ::type_sizes::QT_CORE_LIST_LIST_STRING]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListString {
  unsafe fn new_uninitialized() -> ListString {
    ListString(::std::mem::uninitialized())
  }
}

impl ListString {
  /// C++ method: <span style='color: green;'>```QList<QString>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListString) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QString>::append(const QList<QString>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::string::String) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QString>::append(const QString& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListStringAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QString& QList<QString>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_at(self as *const ::list::ListString, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QString& QList<QString>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_back_const(self as *const ::list::ListString) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QList<QString>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_back(self as *mut ::list::ListString) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QString>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QString_clear(self as *mut ::list::ListString) }
  }

  /// C++ method: <span style='color: green;'>```const QString& QList<QString>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_constFirst(self as *const ::list::ListString) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QString& QList<QString>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_constLast(self as *const ::list::ListString) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QString>::contains(const QString& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QString_contains(self as *const ::list::ListString,
                                              t as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QString>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QString>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QString>::count(const QString& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListStringCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QString>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QString_empty(self as *const ::list::ListString) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QString>::endsWith(const QString& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QString_endsWith(self as *const ::list::ListString,
                                              t as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```const QString& QList<QString>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_first_const(self as *const ::list::ListString) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QList<QString>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_first(self as *mut ::list::ListString) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QString& QList<QString>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_front_const(self as *const ::list::ListString) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QList<QString>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_front(self as *mut ::list::ListString) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QString>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QString>::indexOf(const QString& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QString>::indexOf(const QString& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListStringIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QString>::insert(int i, const QString& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QList_QString_insert(self as *mut ::list::ListString,
                                            i,
                                            t as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QString>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QString_isEmpty(self as *const ::list::ListString) }
  }

  /// C++ method: <span style='color: green;'>```const QString& QList<QString>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_last_const(self as *const ::list::ListString) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QString>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::string::String) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QString>::lastIndexOf(const QString& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::string::String, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QString>::lastIndexOf(const QString& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListStringLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString& QList<QString>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_last(self as *mut ::list::ListString) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QString>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QString_length(self as *const ::list::ListString) }
  }

  /// C++ method: <span style='color: green;'>```QList<QString>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```QList<QString> QList<QString>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```QList<QString> QList<QString>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListString
    where Args: overloading::ListStringMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QString>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QString_move(self as *mut ::list::ListString, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QString>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QString>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListString) -> ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QString>::QList(const QList<QString>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListString
    where Args: overloading::ListStringNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QString> QList<QString>::operator+(const QList<QString>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListString) -> ::list::ListString {
    {
      let mut object: ::list::ListString =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QString_operator_add_to_output(self as *const ::list::ListString,
                                                              l as *const ::list::ListString,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QString>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListString) -> &'l0 mut ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```QList<QString>& QList<QString>::operator+=(const QList<QString>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::string::String) -> &'l0 mut ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```QList<QString>& QList<QString>::operator+=(const QString& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListString
    where Args: overloading::ListStringOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QString>& QList<QString>::operator=(const QList<QString>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListString) -> &'l0 mut ::list::ListString {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QString_operator_assign(self as *mut ::list::ListString,
                                                     l as *const ::list::ListString)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QString>::operator==(const QList<QString>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListString) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QString_operator_eq(self as *const ::list::ListString,
                                                 l as *const ::list::ListString)
    }
  }

  /// C++ method: <span style='color: green;'>```const QString& QList<QString>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::string::String {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QString_operator_index_const(self as *const ::list::ListString, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QString& QList<QString>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::string::String {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QString_operator_index(self as *mut ::list::ListString, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QString>::operator!=(const QList<QString>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListString) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QString_operator_neq(self as *const ::list::ListString,
                                                  l as *const ::list::ListString)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QString>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListString) -> &'l0 mut ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```QList<QString>& QList<QString>::operator<<(const QList<QString>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::string::String) -> &'l0 mut ::list::ListString```<br>
  /// C++ method: <span style='color: green;'>```QList<QString>& QList<QString>::operator<<(const QString& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListString
    where Args: overloading::ListStringOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QString>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QString_pop_back(self as *mut ::list::ListString) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QString>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QString_pop_front(self as *mut ::list::ListString) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QString>::prepend(const QString& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QList_QString_prepend(self as *mut ::list::ListString,
                                             t as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QString>::push_back(const QString& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QList_QString_push_back(self as *mut ::list::ListString,
                                               t as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QString>::push_front(const QString& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QList_QString_push_front(self as *mut ::list::ListString,
                                                t as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QString>::removeAll(const QString& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::string::String) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QString_removeAll(self as *mut ::list::ListString,
                                               t as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QString>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QString_removeAt(self as *mut ::list::ListString, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QString>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QString_removeFirst(self as *mut ::list::ListString) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QString>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QString_removeLast(self as *mut ::list::ListString) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QString>::removeOne(const QString& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QString_removeOne(self as *mut ::list::ListString,
                                               t as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QString>::replace(int i, const QString& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::string::String) {
    unsafe {
      ::ffi::qt_core_c_QList_QString_replace(self as *mut ::list::ListString,
                                             i,
                                             t as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QString>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QString_reserve(self as *mut ::list::ListString, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QString>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QString_size(self as *const ::list::ListString) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QString>::startsWith(const QString& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::string::String) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QString_startsWith(self as *const ::list::ListString,
                                                t as *const ::string::String)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QString>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListString) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QString>::swap(QList<QString>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QString>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListStringSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QString QList<QString>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QString_takeAt_to_output(self as *mut ::list::ListString, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QList<QString>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QString_takeFirst_to_output(self as *mut ::list::ListString, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QString QList<QString>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::string::String {
    {
      let mut object: ::string::String =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QString_takeLast_to_output(self as *mut ::list::ListString, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QString>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QList<QString>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::string::String)) -> ::string::String```<br>
  /// C++ method: <span style='color: green;'>```QString QList<QString>::value(int i, const QString& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::string::String
    where Args: overloading::ListStringValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListString {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QString>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QString_destructor(self as *mut ::list::ListString) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QUrl>```</span>
#[repr(C)]
pub struct ListUrl([u8; ::type_sizes::QT_CORE_LIST_LIST_URL]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListUrl {
  unsafe fn new_uninitialized() -> ListUrl {
    ListUrl(::std::mem::uninitialized())
  }
}

impl ListUrl {
  /// C++ method: <span style='color: green;'>```QList<QUrl>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListUrl) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QUrl>::append(const QList<QUrl>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::url::Url) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QUrl>::append(const QUrl& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListUrlAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QUrl& QList<QUrl>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_at(self as *const ::list::ListUrl, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QUrl& QList<QUrl>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_back_const(self as *const ::list::ListUrl) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QUrl& QList<QUrl>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_back(self as *mut ::list::ListUrl) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QUrl>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_clear(self as *mut ::list::ListUrl) }
  }

  /// C++ method: <span style='color: green;'>```const QUrl& QList<QUrl>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_constFirst(self as *const ::list::ListUrl) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QUrl& QList<QUrl>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_constLast(self as *const ::list::ListUrl) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUrl>::contains(const QUrl& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::url::Url) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QUrl_contains(self as *const ::list::ListUrl, t as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QUrl>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::url::Url) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QUrl>::count(const QUrl& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListUrlCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QUrl>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QUrl_empty(self as *const ::list::ListUrl) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUrl>::endsWith(const QUrl& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::url::Url) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QUrl_endsWith(self as *const ::list::ListUrl, t as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```const QUrl& QList<QUrl>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_first_const(self as *const ::list::ListUrl) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QUrl& QList<QUrl>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_first(self as *mut ::list::ListUrl) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QUrl& QList<QUrl>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_front_const(self as *const ::list::ListUrl) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QUrl& QList<QUrl>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_front(self as *mut ::list::ListUrl) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::url::Url) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QUrl>::indexOf(const QUrl& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::url::Url, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QUrl>::indexOf(const QUrl& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListUrlIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QUrl>::insert(int i, const QUrl& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::url::Url) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_insert(self as *mut ::list::ListUrl, i, t as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUrl>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QUrl_isEmpty(self as *const ::list::ListUrl) }
  }

  /// C++ method: <span style='color: green;'>```const QUrl& QList<QUrl>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_last_const(self as *const ::list::ListUrl) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::url::Url) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QUrl>::lastIndexOf(const QUrl& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::url::Url, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QUrl>::lastIndexOf(const QUrl& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListUrlLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl& QList<QUrl>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_last(self as *mut ::list::ListUrl) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QUrl>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QUrl_length(self as *const ::list::ListUrl) }
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```QList<QUrl> QList<QUrl>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```QList<QUrl> QList<QUrl>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListUrl
    where Args: overloading::ListUrlMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QUrl>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_move(self as *mut ::list::ListUrl, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QUrl>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListUrl) -> ::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QUrl>::QList(const QList<QUrl>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListUrl
    where Args: overloading::ListUrlNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QUrl> QList<QUrl>::operator+(const QList<QUrl>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListUrl) -> ::list::ListUrl {
    {
      let mut object: ::list::ListUrl =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_operator_add_to_output(self as *const ::list::ListUrl,
                                                           l as *const ::list::ListUrl,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListUrl) -> &'l0 mut ::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```QList<QUrl>& QList<QUrl>::operator+=(const QList<QUrl>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::url::Url) -> &'l0 mut ::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```QList<QUrl>& QList<QUrl>::operator+=(const QUrl& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListUrl
    where Args: overloading::ListUrlOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QUrl>& QList<QUrl>::operator=(const QList<QUrl>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListUrl) -> &'l0 mut ::list::ListUrl {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QUrl_operator_assign(self as *mut ::list::ListUrl, l as *const ::list::ListUrl) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUrl>::operator==(const QList<QUrl>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListUrl) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QUrl_operator_eq(self as *const ::list::ListUrl, l as *const ::list::ListUrl) }
  }

  /// C++ method: <span style='color: green;'>```const QUrl& QList<QUrl>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_operator_index_const(self as *const ::list::ListUrl, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QUrl& QList<QUrl>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::url::Url {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QUrl_operator_index(self as *mut ::list::ListUrl, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUrl>::operator!=(const QList<QUrl>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListUrl) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QUrl_operator_neq(self as *const ::list::ListUrl, l as *const ::list::ListUrl) }
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListUrl) -> &'l0 mut ::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```QList<QUrl>& QList<QUrl>::operator<<(const QList<QUrl>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::url::Url) -> &'l0 mut ::list::ListUrl```<br>
  /// C++ method: <span style='color: green;'>```QList<QUrl>& QList<QUrl>::operator<<(const QUrl& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListUrl
    where Args: overloading::ListUrlOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QUrl>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_pop_back(self as *mut ::list::ListUrl) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUrl>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_pop_front(self as *mut ::list::ListUrl) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUrl>::prepend(const QUrl& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::url::Url) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_prepend(self as *mut ::list::ListUrl, t as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUrl>::push_back(const QUrl& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::url::Url) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_push_back(self as *mut ::list::ListUrl, t as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUrl>::push_front(const QUrl& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::url::Url) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_push_front(self as *mut ::list::ListUrl, t as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QUrl>::removeAll(const QUrl& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::url::Url) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QUrl_removeAll(self as *mut ::list::ListUrl, t as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUrl>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_removeAt(self as *mut ::list::ListUrl, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUrl>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_removeFirst(self as *mut ::list::ListUrl) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUrl>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_removeLast(self as *mut ::list::ListUrl) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUrl>::removeOne(const QUrl& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::url::Url) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QUrl_removeOne(self as *mut ::list::ListUrl, t as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUrl>::replace(int i, const QUrl& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::url::Url) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_replace(self as *mut ::list::ListUrl, i, t as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUrl>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_reserve(self as *mut ::list::ListUrl, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QUrl>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QUrl_size(self as *const ::list::ListUrl) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUrl>::startsWith(const QUrl& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::url::Url) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QUrl_startsWith(self as *const ::list::ListUrl, t as *const ::url::Url) }
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListUrl) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QUrl>::swap(QList<QUrl>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QUrl>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListUrlSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUrl QList<QUrl>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::url::Url {
    {
      let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_takeAt_to_output(self as *mut ::list::ListUrl, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl QList<QUrl>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::url::Url {
    {
      let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_takeFirst_to_output(self as *mut ::list::ListUrl, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QUrl QList<QUrl>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::url::Url {
    {
      let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_takeLast_to_output(self as *mut ::list::ListUrl, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QUrl>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```QUrl QList<QUrl>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::url::Url)) -> ::url::Url```<br>
  /// C++ method: <span style='color: green;'>```QUrl QList<QUrl>::value(int i, const QUrl& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::url::Url
    where Args: overloading::ListUrlValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListUrl {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QUrl>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QUrl_destructor(self as *mut ::list::ListUrl) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QVariant>```</span>
#[repr(C)]
pub struct ListVariant([u8; ::type_sizes::QT_CORE_LIST_LIST_VARIANT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListVariant {
  unsafe fn new_uninitialized() -> ListVariant {
    ListVariant(::std::mem::uninitialized())
  }
}

impl ListVariant {
  /// C++ method: <span style='color: green;'>```QList<QVariant>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListVariant) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QVariant>::append(const QList<QVariant>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::variant::Variant) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QVariant>::append(const QVariant& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListVariantAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QVariant& QList<QVariant>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_at(self as *const ::list::ListVariant, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QVariant& QList<QVariant>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_back_const(self as *const ::list::ListVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVariant& QList<QVariant>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_back(self as *mut ::list::ListVariant) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QVariant>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QVariant_clear(self as *mut ::list::ListVariant) }
  }

  /// C++ method: <span style='color: green;'>```const QVariant& QList<QVariant>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_constFirst(self as *const ::list::ListVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QVariant& QList<QVariant>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_constLast(self as *const ::list::ListVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QVariant>::contains(const QVariant& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::variant::Variant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_contains(self as *const ::list::ListVariant,
                                               t as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QVariant>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QVariant>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::variant::Variant) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QVariant>::count(const QVariant& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListVariantCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QVariant>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QVariant_empty(self as *const ::list::ListVariant) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QVariant>::endsWith(const QVariant& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::variant::Variant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_endsWith(self as *const ::list::ListVariant,
                                               t as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```const QVariant& QList<QVariant>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_first_const(self as *const ::list::ListVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVariant& QList<QVariant>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_first(self as *mut ::list::ListVariant) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QVariant& QList<QVariant>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_front_const(self as *const ::list::ListVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVariant& QList<QVariant>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_front(self as *mut ::list::ListVariant) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QVariant>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::variant::Variant) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QVariant>::indexOf(const QVariant& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::variant::Variant, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QVariant>::indexOf(const QVariant& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListVariantIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QVariant>::insert(int i, const QVariant& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_insert(self as *mut ::list::ListVariant,
                                             i,
                                             t as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QVariant>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_core_c_QList_QVariant_isEmpty(self as *const ::list::ListVariant) }
  }

  /// C++ method: <span style='color: green;'>```const QVariant& QList<QVariant>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_last_const(self as *const ::list::ListVariant) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QVariant>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::variant::Variant) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QVariant>::lastIndexOf(const QVariant& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::variant::Variant, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QVariant>::lastIndexOf(const QVariant& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListVariantLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVariant& QList<QVariant>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_last(self as *mut ::list::ListVariant) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QVariant>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QVariant_length(self as *const ::list::ListVariant) }
  }

  /// C++ method: <span style='color: green;'>```QList<QVariant>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant> QList<QVariant>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant> QList<QVariant>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListVariant
    where Args: overloading::ListVariantMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QVariant>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QVariant_move(self as *mut ::list::ListVariant, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QVariant>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QVariant>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListVariant) -> ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QVariant>::QList(const QList<QVariant>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListVariant
    where Args: overloading::ListVariantNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QVariant> QList<QVariant>::operator+(const QList<QVariant>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListVariant) -> ::list::ListVariant {
    {
      let mut object: ::list::ListVariant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_operator_add_to_output(self as *const ::list::ListVariant,
                                                               l as *const ::list::ListVariant,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QVariant>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListVariant) -> &'l0 mut ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant>& QList<QVariant>::operator+=(const QList<QVariant>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::variant::Variant) -> &'l0 mut ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant>& QList<QVariant>::operator+=(const QVariant& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListVariant
    where Args: overloading::ListVariantOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QVariant>& QList<QVariant>::operator=(const QList<QVariant>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListVariant) -> &'l0 mut ::list::ListVariant {
    let ffi_result = unsafe {
      ::ffi::qt_core_c_QList_QVariant_operator_assign(self as *mut ::list::ListVariant,
                                                      l as *const ::list::ListVariant)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QVariant>::operator==(const QList<QVariant>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_operator_eq(self as *const ::list::ListVariant,
                                                  l as *const ::list::ListVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```const QVariant& QList<QVariant>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::variant::Variant {
    let ffi_result =
      unsafe { ::ffi::qt_core_c_QList_QVariant_operator_index_const(self as *const ::list::ListVariant, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVariant& QList<QVariant>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::variant::Variant {
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QVariant_operator_index(self as *mut ::list::ListVariant, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QVariant>::operator!=(const QList<QVariant>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListVariant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_operator_neq(self as *const ::list::ListVariant,
                                                   l as *const ::list::ListVariant)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QVariant>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListVariant) -> &'l0 mut ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant>& QList<QVariant>::operator<<(const QList<QVariant>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::variant::Variant) -> &'l0 mut ::list::ListVariant```<br>
  /// C++ method: <span style='color: green;'>```QList<QVariant>& QList<QVariant>::operator<<(const QVariant& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListVariant
    where Args: overloading::ListVariantOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QVariant>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QVariant_pop_back(self as *mut ::list::ListVariant) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QVariant>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QVariant_pop_front(self as *mut ::list::ListVariant) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QVariant>::prepend(const QVariant& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_prepend(self as *mut ::list::ListVariant,
                                              t as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QVariant>::push_back(const QVariant& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_push_back(self as *mut ::list::ListVariant,
                                                t as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QVariant>::push_front(const QVariant& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_push_front(self as *mut ::list::ListVariant,
                                                 t as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QVariant>::removeAll(const QVariant& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::variant::Variant) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_removeAll(self as *mut ::list::ListVariant,
                                                t as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QVariant>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QVariant_removeAt(self as *mut ::list::ListVariant, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QVariant>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QVariant_removeFirst(self as *mut ::list::ListVariant) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QVariant>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QVariant_removeLast(self as *mut ::list::ListVariant) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QVariant>::removeOne(const QVariant& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::variant::Variant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_removeOne(self as *mut ::list::ListVariant,
                                                t as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QVariant>::replace(int i, const QVariant& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::variant::Variant) {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_replace(self as *mut ::list::ListVariant,
                                              i,
                                              t as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QVariant>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_core_c_QList_QVariant_reserve(self as *mut ::list::ListVariant, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QVariant>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_core_c_QList_QVariant_size(self as *const ::list::ListVariant) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QVariant>::startsWith(const QVariant& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::variant::Variant) -> bool {
    unsafe {
      ::ffi::qt_core_c_QList_QVariant_startsWith(self as *const ::list::ListVariant,
                                                 t as *const ::variant::Variant)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QVariant>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListVariant) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QVariant>::swap(QList<QVariant>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QVariant>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListVariantSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVariant QList<QVariant>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_takeAt_to_output(self as *mut ::list::ListVariant, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QList<QVariant>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_takeFirst_to_output(self as *mut ::list::ListVariant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVariant QList<QVariant>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::variant::Variant {
    {
      let mut object: ::variant::Variant =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_takeLast_to_output(self as *mut ::list::ListVariant, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QVariant>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QList<QVariant>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::variant::Variant)) -> ::variant::Variant```<br>
  /// C++ method: <span style='color: green;'>```QVariant QList<QVariant>::value(int i, const QVariant& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::variant::Variant
    where Args: overloading::ListVariantValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListVariant {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QVariant>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_core_c_QList_QVariant_destructor(self as *mut ::list::ListVariant) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ListAbstractAnimationMutPtr::index_of](../struct.ListAbstractAnimationMutPtr.html#method.index_of) method.
  pub trait ListAbstractAnimationMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractAnimationMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListAbstractAnimationMutPtrIndexOfArgs<'largs> for &'largs *mut ::abstract_animation::AbstractAnimation {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractAnimationMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_indexOf_t(original_self as *const ::list::ListAbstractAnimationMutPtr, t as *const *mut ::abstract_animation::AbstractAnimation)
    }
  }
  impl<'largs> ListAbstractAnimationMutPtrIndexOfArgs<'largs> for (&'largs *mut ::abstract_animation::AbstractAnimation,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::list::ListAbstractAnimationMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_indexOf_t_from(original_self as *const ::list::ListAbstractAnimationMutPtr, t as *const *mut ::abstract_animation::AbstractAnimation, from)
  }
}
  /// This trait represents a set of arguments accepted by [ListAbstractAnimationMutPtr::last_index_of](../struct.ListAbstractAnimationMutPtr.html#method.last_index_of) method.
  pub trait ListAbstractAnimationMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractAnimationMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListAbstractAnimationMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::abstract_animation::AbstractAnimation {

  unsafe fn exec(self, original_self: &'largs ::list::ListAbstractAnimationMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_lastIndexOf_t(original_self as *const ::list::ListAbstractAnimationMutPtr, t as *const *mut ::abstract_animation::AbstractAnimation)
  }
}
  impl<'largs> ListAbstractAnimationMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::abstract_animation::AbstractAnimation,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::list::ListAbstractAnimationMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_lastIndexOf_t_from(original_self as *const ::list::ListAbstractAnimationMutPtr, t as *const *mut ::abstract_animation::AbstractAnimation, from)
  }
}
  /// This trait represents a set of arguments accepted by [ListAbstractAnimationMutPtr::mid](../struct.ListAbstractAnimationMutPtr.html#method.mid) method.
  pub trait ListAbstractAnimationMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListAbstractAnimationMutPtr) -> ::list::ListAbstractAnimationMutPtr;
  }
  impl<'largs> ListAbstractAnimationMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListAbstractAnimationMutPtr) -> ::list::ListAbstractAnimationMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListAbstractAnimationMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_mid_to_output_pos(original_self as *const ::list::ListAbstractAnimationMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListAbstractAnimationMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListAbstractAnimationMutPtr) -> ::list::ListAbstractAnimationMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListAbstractAnimationMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_mid_to_output_pos_length(original_self as *const ::list::ListAbstractAnimationMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractAnimationMutPtr::new](../struct.ListAbstractAnimationMutPtr.html#method.new) method.
  pub trait ListAbstractAnimationMutPtrNewArgs {
    fn exec(self) -> ::list::ListAbstractAnimationMutPtr;
  }
  impl<'a> ListAbstractAnimationMutPtrNewArgs for &'a ::list::ListAbstractAnimationMutPtr {
    fn exec(self) -> ::list::ListAbstractAnimationMutPtr {
      let l = self;
      {
        let mut object: ::list::ListAbstractAnimationMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_constructor_l(l as *const ::list::ListAbstractAnimationMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListAbstractAnimationMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListAbstractAnimationMutPtr {

      {
        let mut object: ::list::ListAbstractAnimationMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractAnimationMutPtr::swap](../struct.ListAbstractAnimationMutPtr.html#method.swap) method.
  pub trait ListAbstractAnimationMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractAnimationMutPtr) -> ();
  }
  impl<'largs> ListAbstractAnimationMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractAnimationMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_swap_i_j(original_self as *mut ::list::ListAbstractAnimationMutPtr, i, j) }
    }
  }
  impl<'largs> ListAbstractAnimationMutPtrSwapArgs<'largs> for &'largs mut ::list::ListAbstractAnimationMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractAnimationMutPtr) -> () {
      let other = self;
      unsafe { ::ffi::qt_core_c_QList_QAbstractAnimation_ptr_swap_other(original_self as *mut ::list::ListAbstractAnimationMutPtr, other as *mut ::list::ListAbstractAnimationMutPtr) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractEventDispatcherTimerInfo::append](../struct.ListAbstractEventDispatcherTimerInfo.html#method.append) method.
  pub trait ListAbstractEventDispatcherTimerInfoAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo) -> ();
  }
  impl<'largs> ListAbstractEventDispatcherTimerInfoAppendArgs<'largs> for &'largs ::abstract_event_dispatcher::TimerInfo {

  fn exec(self, original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo) -> () {
    let t = self;
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_append_QAbstractEventDispatcher_TimerInfo(original_self as *mut ::list::ListAbstractEventDispatcherTimerInfo, t as *const ::abstract_event_dispatcher::TimerInfo) }
  }
}
  impl<'largs> ListAbstractEventDispatcherTimerInfoAppendArgs<'largs> for &'largs ::list::ListAbstractEventDispatcherTimerInfo {

  fn exec(self, original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo) -> () {
    let t = self;
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_append_QList_QAbstractEventDispatcher_TimerInfo(original_self as *mut ::list::ListAbstractEventDispatcherTimerInfo, t as *const ::list::ListAbstractEventDispatcherTimerInfo) }
  }
}
  /// This trait represents a set of arguments accepted by [ListAbstractEventDispatcherTimerInfo::mid](../struct.ListAbstractEventDispatcherTimerInfo.html#method.mid) method.
  pub trait ListAbstractEventDispatcherTimerInfoMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::list::ListAbstractEventDispatcherTimerInfo)
            -> ::list::ListAbstractEventDispatcherTimerInfo;
  }
  impl<'largs> ListAbstractEventDispatcherTimerInfoMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::list::ListAbstractEventDispatcherTimerInfo)
            -> ::list::ListAbstractEventDispatcherTimerInfo {
      let pos = self;
      {
        let mut object: ::list::ListAbstractEventDispatcherTimerInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_mid_to_output_pos(original_self as *const ::list::ListAbstractEventDispatcherTimerInfo, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListAbstractEventDispatcherTimerInfoMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::list::ListAbstractEventDispatcherTimerInfo)
            -> ::list::ListAbstractEventDispatcherTimerInfo {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListAbstractEventDispatcherTimerInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_mid_to_output_pos_length(original_self as *const ::list::ListAbstractEventDispatcherTimerInfo, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractEventDispatcherTimerInfo::new](../struct.ListAbstractEventDispatcherTimerInfo.html#method.new) method.
  pub trait ListAbstractEventDispatcherTimerInfoNewArgs {
    fn exec(self) -> ::list::ListAbstractEventDispatcherTimerInfo;
  }
  impl<'a> ListAbstractEventDispatcherTimerInfoNewArgs for &'a ::list::ListAbstractEventDispatcherTimerInfo {
    fn exec(self) -> ::list::ListAbstractEventDispatcherTimerInfo {
      let l = self;
      {
        let mut object: ::list::ListAbstractEventDispatcherTimerInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_constructor_l(l as *const ::list::ListAbstractEventDispatcherTimerInfo, &mut object);
        }
        object
      }
    }
  }
  impl ListAbstractEventDispatcherTimerInfoNewArgs for () {
    fn exec(self) -> ::list::ListAbstractEventDispatcherTimerInfo {

      {
        let mut object: ::list::ListAbstractEventDispatcherTimerInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractEventDispatcherTimerInfo::op_add_assign](../struct.ListAbstractEventDispatcherTimerInfo.html#method.op_add_assign) method.
  pub trait ListAbstractEventDispatcherTimerInfoOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo)
            -> &'largs mut ::list::ListAbstractEventDispatcherTimerInfo;
  }
  impl<'largs> ListAbstractEventDispatcherTimerInfoOpAddAssignArgs<'largs> for &'largs ::list::ListAbstractEventDispatcherTimerInfo {

  fn exec(self, original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo) -> &'largs mut ::list::ListAbstractEventDispatcherTimerInfo {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_add_assign_l(original_self as *mut ::list::ListAbstractEventDispatcherTimerInfo, l as *const ::list::ListAbstractEventDispatcherTimerInfo) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> ListAbstractEventDispatcherTimerInfoOpAddAssignArgs<'largs> for &'largs ::abstract_event_dispatcher::TimerInfo {

  fn exec(self, original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo) -> &'largs mut ::list::ListAbstractEventDispatcherTimerInfo {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_add_assign_t(original_self as *mut ::list::ListAbstractEventDispatcherTimerInfo, t as *const ::abstract_event_dispatcher::TimerInfo) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [ListAbstractEventDispatcherTimerInfo::op_shl](../struct.ListAbstractEventDispatcherTimerInfo.html#method.op_shl) method.
  pub trait ListAbstractEventDispatcherTimerInfoOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo)
            -> &'largs mut ::list::ListAbstractEventDispatcherTimerInfo;
  }
  impl<'largs> ListAbstractEventDispatcherTimerInfoOpShlArgs<'largs> for &'largs ::list::ListAbstractEventDispatcherTimerInfo {

  fn exec(self, original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo) -> &'largs mut ::list::ListAbstractEventDispatcherTimerInfo {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_shl_l(original_self as *mut ::list::ListAbstractEventDispatcherTimerInfo, l as *const ::list::ListAbstractEventDispatcherTimerInfo) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> ListAbstractEventDispatcherTimerInfoOpShlArgs<'largs> for &'largs ::abstract_event_dispatcher::TimerInfo {

  fn exec(self, original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo) -> &'largs mut ::list::ListAbstractEventDispatcherTimerInfo {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_operator_shl_t(original_self as *mut ::list::ListAbstractEventDispatcherTimerInfo, t as *const ::abstract_event_dispatcher::TimerInfo) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [ListAbstractEventDispatcherTimerInfo::swap](../struct.ListAbstractEventDispatcherTimerInfo.html#method.swap) method.
  pub trait ListAbstractEventDispatcherTimerInfoSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo) -> ();
  }
  impl<'largs> ListAbstractEventDispatcherTimerInfoSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_swap_i_j(original_self as *mut ::list::ListAbstractEventDispatcherTimerInfo, i, j) }
    }
  }
  impl<'largs> ListAbstractEventDispatcherTimerInfoSwapArgs<'largs> for &'largs mut ::list::ListAbstractEventDispatcherTimerInfo {

  fn exec(self, original_self: &'largs mut ::list::ListAbstractEventDispatcherTimerInfo) -> () {
    let other = self;
    unsafe { ::ffi::qt_core_c_QList_QAbstractEventDispatcher_TimerInfo_swap_other(original_self as *mut ::list::ListAbstractEventDispatcherTimerInfo, other as *mut ::list::ListAbstractEventDispatcherTimerInfo) }
  }
}
  /// This trait represents a set of arguments accepted by [ListAbstractStateMutPtr::index_of](../struct.ListAbstractStateMutPtr.html#method.index_of) method.
  pub trait ListAbstractStateMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractStateMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListAbstractStateMutPtrIndexOfArgs<'largs> for &'largs *mut ::abstract_state::AbstractState {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractStateMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_core_c_QList_QAbstractState_ptr_indexOf_t(original_self as *const ::list::ListAbstractStateMutPtr,
                                                          t as *const *mut ::abstract_state::AbstractState)
    }
  }
  impl<'largs> ListAbstractStateMutPtrIndexOfArgs<'largs>
    for (&'largs *mut ::abstract_state::AbstractState, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractStateMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_core_c_QList_QAbstractState_ptr_indexOf_t_from(original_self as *const ::list::ListAbstractStateMutPtr, t as *const *mut ::abstract_state::AbstractState, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractStateMutPtr::last_index_of](../struct.ListAbstractStateMutPtr.html#method.last_index_of) method.
  pub trait ListAbstractStateMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractStateMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListAbstractStateMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::abstract_state::AbstractState {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractStateMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_core_c_QList_QAbstractState_ptr_lastIndexOf_t(original_self as *const ::list::ListAbstractStateMutPtr,
                                                              t as *const *mut ::abstract_state::AbstractState)
    }
  }
  impl<'largs> ListAbstractStateMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::abstract_state::AbstractState, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractStateMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_core_c_QList_QAbstractState_ptr_lastIndexOf_t_from(original_self as *const ::list::ListAbstractStateMutPtr, t as *const *mut ::abstract_state::AbstractState, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractStateMutPtr::mid](../struct.ListAbstractStateMutPtr.html#method.mid) method.
  pub trait ListAbstractStateMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListAbstractStateMutPtr) -> ::list::ListAbstractStateMutPtr;
  }
  impl<'largs> ListAbstractStateMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListAbstractStateMutPtr) -> ::list::ListAbstractStateMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListAbstractStateMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractState_ptr_mid_to_output_pos(original_self as *const ::list::ListAbstractStateMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListAbstractStateMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListAbstractStateMutPtr) -> ::list::ListAbstractStateMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListAbstractStateMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractState_ptr_mid_to_output_pos_length(original_self as *const ::list::ListAbstractStateMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractStateMutPtr::new](../struct.ListAbstractStateMutPtr.html#method.new) method.
  pub trait ListAbstractStateMutPtrNewArgs {
    fn exec(self) -> ::list::ListAbstractStateMutPtr;
  }
  impl<'a> ListAbstractStateMutPtrNewArgs for &'a ::list::ListAbstractStateMutPtr {
    fn exec(self) -> ::list::ListAbstractStateMutPtr {
      let l = self;
      {
        let mut object: ::list::ListAbstractStateMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractState_ptr_constructor_l(l as *const ::list::ListAbstractStateMutPtr,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl ListAbstractStateMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListAbstractStateMutPtr {

      {
        let mut object: ::list::ListAbstractStateMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractState_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractStateMutPtr::swap](../struct.ListAbstractStateMutPtr.html#method.swap) method.
  pub trait ListAbstractStateMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractStateMutPtr) -> ();
  }
  impl<'largs> ListAbstractStateMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractStateMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractState_ptr_swap_i_j(original_self as *mut ::list::ListAbstractStateMutPtr, i, j)
      }
    }
  }
  impl<'largs> ListAbstractStateMutPtrSwapArgs<'largs> for &'largs mut ::list::ListAbstractStateMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractStateMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QAbstractState_ptr_swap_other(original_self as *mut ::list::ListAbstractStateMutPtr,
                                                             other as *mut ::list::ListAbstractStateMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractTransitionMutPtr::index_of](../struct.ListAbstractTransitionMutPtr.html#method.index_of) method.
  pub trait ListAbstractTransitionMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractTransitionMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListAbstractTransitionMutPtrIndexOfArgs<'largs> for &'largs *mut ::abstract_transition::AbstractTransition {

  unsafe fn exec(self, original_self: &'largs ::list::ListAbstractTransitionMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_indexOf_t(original_self as *const ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition)
  }
}
  impl<'largs> ListAbstractTransitionMutPtrIndexOfArgs<'largs> for (&'largs *mut ::abstract_transition::AbstractTransition,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::list::ListAbstractTransitionMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_indexOf_t_from(original_self as *const ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition, from)
  }
}
  /// This trait represents a set of arguments accepted by [ListAbstractTransitionMutPtr::last_index_of](../struct.ListAbstractTransitionMutPtr.html#method.last_index_of) method.
  pub trait ListAbstractTransitionMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractTransitionMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListAbstractTransitionMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::abstract_transition::AbstractTransition {

  unsafe fn exec(self, original_self: &'largs ::list::ListAbstractTransitionMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_lastIndexOf_t(original_self as *const ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition)
  }
}
  impl<'largs> ListAbstractTransitionMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::abstract_transition::AbstractTransition,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::list::ListAbstractTransitionMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_core_c_QList_QAbstractTransition_ptr_lastIndexOf_t_from(original_self as *const ::list::ListAbstractTransitionMutPtr, t as *const *mut ::abstract_transition::AbstractTransition, from)
  }
}
  /// This trait represents a set of arguments accepted by [ListAbstractTransitionMutPtr::mid](../struct.ListAbstractTransitionMutPtr.html#method.mid) method.
  pub trait ListAbstractTransitionMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListAbstractTransitionMutPtr) -> ::list::ListAbstractTransitionMutPtr;
  }
  impl<'largs> ListAbstractTransitionMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListAbstractTransitionMutPtr) -> ::list::ListAbstractTransitionMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListAbstractTransitionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractTransition_ptr_mid_to_output_pos(original_self as *const ::list::ListAbstractTransitionMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListAbstractTransitionMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListAbstractTransitionMutPtr) -> ::list::ListAbstractTransitionMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListAbstractTransitionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractTransition_ptr_mid_to_output_pos_length(original_self as *const ::list::ListAbstractTransitionMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractTransitionMutPtr::new](../struct.ListAbstractTransitionMutPtr.html#method.new) method.
  pub trait ListAbstractTransitionMutPtrNewArgs {
    fn exec(self) -> ::list::ListAbstractTransitionMutPtr;
  }
  impl<'a> ListAbstractTransitionMutPtrNewArgs for &'a ::list::ListAbstractTransitionMutPtr {
    fn exec(self) -> ::list::ListAbstractTransitionMutPtr {
      let l = self;
      {
        let mut object: ::list::ListAbstractTransitionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractTransition_ptr_constructor_l(l as *const ::list::ListAbstractTransitionMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListAbstractTransitionMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListAbstractTransitionMutPtr {

      {
        let mut object: ::list::ListAbstractTransitionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QAbstractTransition_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractTransitionMutPtr::swap](../struct.ListAbstractTransitionMutPtr.html#method.swap) method.
  pub trait ListAbstractTransitionMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractTransitionMutPtr) -> ();
  }
  impl<'largs> ListAbstractTransitionMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractTransitionMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QAbstractTransition_ptr_swap_i_j(original_self as *mut ::list::ListAbstractTransitionMutPtr, i, j) }
    }
  }
  impl<'largs> ListAbstractTransitionMutPtrSwapArgs<'largs> for &'largs mut ::list::ListAbstractTransitionMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractTransitionMutPtr) -> () {
      let other = self;
      unsafe { ::ffi::qt_core_c_QList_QAbstractTransition_ptr_swap_other(original_self as *mut ::list::ListAbstractTransitionMutPtr, other as *mut ::list::ListAbstractTransitionMutPtr) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListByteArray::append](../struct.ListByteArray.html#method.append) method.
  pub trait ListByteArrayAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> ();
  }
  impl<'largs> ListByteArrayAppendArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_append_QByteArray(original_self as *mut ::list::ListByteArray,
                                                            t as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> ListByteArrayAppendArgs<'largs> for &'largs ::list::ListByteArray {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_append_QList_QByteArray(original_self as *mut ::list::ListByteArray,
                                                                  t as *const ::list::ListByteArray)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListByteArray::count](../struct.ListByteArray.html#method.count) method.
  pub trait ListByteArrayCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::libc::c_int;
  }
  impl<'largs> ListByteArrayCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QByteArray_count_no_args(original_self as *const ::list::ListByteArray) }
    }
  }
  impl<'largs> ListByteArrayCountArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_count_t(original_self as *const ::list::ListByteArray,
                                                  t as *const ::byte_array::ByteArray)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListByteArray::index_of](../struct.ListByteArray.html#method.index_of) method.
  pub trait ListByteArrayIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::libc::c_int;
  }
  impl<'largs> ListByteArrayIndexOfArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_indexOf_t(original_self as *const ::list::ListByteArray,
                                                    t as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> ListByteArrayIndexOfArgs<'largs> for (&'largs ::byte_array::ByteArray, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_indexOf_t_from(original_self as *const ::list::ListByteArray,
                                                         t as *const ::byte_array::ByteArray,
                                                         from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListByteArray::last_index_of](../struct.ListByteArray.html#method.last_index_of) method.
  pub trait ListByteArrayLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::libc::c_int;
  }
  impl<'largs> ListByteArrayLastIndexOfArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_lastIndexOf_t(original_self as *const ::list::ListByteArray,
                                                        t as *const ::byte_array::ByteArray)
      }
    }
  }
  impl<'largs> ListByteArrayLastIndexOfArgs<'largs> for (&'largs ::byte_array::ByteArray, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_lastIndexOf_t_from(original_self as *const ::list::ListByteArray,
                                                             t as *const ::byte_array::ByteArray,
                                                             from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListByteArray::mid](../struct.ListByteArray.html#method.mid) method.
  pub trait ListByteArrayMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::list::ListByteArray;
  }
  impl<'largs> ListByteArrayMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::list::ListByteArray {
      let pos = self;
      {
        let mut object: ::list::ListByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QByteArray_mid_to_output_pos(original_self as *const ::list::ListByteArray,
                                                              pos,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListByteArrayMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::list::ListByteArray {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QByteArray_mid_to_output_pos_length(original_self as *const ::list::ListByteArray,
                                                                     pos,
                                                                     length,
                                                                     &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListByteArray::new](../struct.ListByteArray.html#method.new) method.
  pub trait ListByteArrayNewArgs {
    fn exec(self) -> ::list::ListByteArray;
  }
  impl<'a> ListByteArrayNewArgs for &'a ::list::ListByteArray {
    fn exec(self) -> ::list::ListByteArray {
      let l = self;
      {
        let mut object: ::list::ListByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QByteArray_constructor_l(l as *const ::list::ListByteArray, &mut object);
        }
        object
      }
    }
  }
  impl ListByteArrayNewArgs for () {
    fn exec(self) -> ::list::ListByteArray {

      {
        let mut object: ::list::ListByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QByteArray_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListByteArray::op_add_assign](../struct.ListByteArray.html#method.op_add_assign) method.
  pub trait ListByteArrayOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> &'largs mut ::list::ListByteArray;
  }
  impl<'largs> ListByteArrayOpAddAssignArgs<'largs> for &'largs ::list::ListByteArray {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> &'largs mut ::list::ListByteArray {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QByteArray_operator_add_assign_l(original_self as *mut ::list::ListByteArray,
                                                                  l as *const ::list::ListByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListByteArrayOpAddAssignArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> &'largs mut ::list::ListByteArray {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QByteArray_operator_add_assign_t(original_self as *mut ::list::ListByteArray,
                                                                  t as *const ::byte_array::ByteArray)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListByteArray::op_shl](../struct.ListByteArray.html#method.op_shl) method.
  pub trait ListByteArrayOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> &'largs mut ::list::ListByteArray;
  }
  impl<'largs> ListByteArrayOpShlArgs<'largs> for &'largs ::list::ListByteArray {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> &'largs mut ::list::ListByteArray {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QByteArray_operator_shl_l(original_self as *mut ::list::ListByteArray,
                                                         l as *const ::list::ListByteArray)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListByteArrayOpShlArgs<'largs> for &'largs ::byte_array::ByteArray {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> &'largs mut ::list::ListByteArray {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QByteArray_operator_shl_t(original_self as *mut ::list::ListByteArray,
                                                         t as *const ::byte_array::ByteArray)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListByteArray::swap](../struct.ListByteArray.html#method.swap) method.
  pub trait ListByteArraySwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> ();
  }
  impl<'largs> ListByteArraySwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QByteArray_swap_i_j(original_self as *mut ::list::ListByteArray, i, j) }
    }
  }
  impl<'largs> ListByteArraySwapArgs<'largs> for &'largs mut ::list::ListByteArray {
    fn exec(self, original_self: &'largs mut ::list::ListByteArray) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QByteArray_swap_other(original_self as *mut ::list::ListByteArray,
                                                     other as *mut ::list::ListByteArray)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListByteArray::value](../struct.ListByteArray.html#method.value) method.
  pub trait ListByteArrayValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::byte_array::ByteArray;
  }
  impl<'largs> ListByteArrayValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::byte_array::ByteArray {
      let i = self;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QByteArray_value_to_output_i(original_self as *const ::list::ListByteArray,
                                                              i,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListByteArrayValueArgs<'largs> for (::libc::c_int, &'largs ::byte_array::ByteArray) {
    fn exec(self, original_self: &'largs ::list::ListByteArray) -> ::byte_array::ByteArray {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::byte_array::ByteArray =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QByteArray_value_to_output_i_defaultValue(original_self as *const ::list::ListByteArray, i, default_value as *const ::byte_array::ByteArray, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCInt::append](../struct.ListCInt.html#method.append) method.
  pub trait ListCIntAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> ();
  }
  impl<'largs> ListCIntAppendArgs<'largs> for &'largs ::list::ListCInt {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_int_append_QList_int(original_self as *mut ::list::ListCInt,
                                                    t as *const ::list::ListCInt)
      }
    }
  }
  impl<'largs> ListCIntAppendArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_int_append_int(original_self as *mut ::list::ListCInt,
                                              t as *const ::libc::c_int)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCInt::count](../struct.ListCInt.html#method.count) method.
  pub trait ListCIntCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int;
  }
  impl<'largs> ListCIntCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_int_count_no_args(original_self as *const ::list::ListCInt) }
    }
  }
  impl<'largs> ListCIntCountArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_int_count_t(original_self as *const ::list::ListCInt,
                                           t as *const ::libc::c_int)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCInt::index_of](../struct.ListCInt.html#method.index_of) method.
  pub trait ListCIntIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int;
  }
  impl<'largs> ListCIntIndexOfArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_int_indexOf_t(original_self as *const ::list::ListCInt,
                                             t as *const ::libc::c_int)
      }
    }
  }
  impl<'largs> ListCIntIndexOfArgs<'largs> for (&'largs ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_int_indexOf_t_from(original_self as *const ::list::ListCInt,
                                                  t as *const ::libc::c_int,
                                                  from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCInt::last_index_of](../struct.ListCInt.html#method.last_index_of) method.
  pub trait ListCIntLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int;
  }
  impl<'largs> ListCIntLastIndexOfArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_int_lastIndexOf_t(original_self as *const ::list::ListCInt,
                                                 t as *const ::libc::c_int)
      }
    }
  }
  impl<'largs> ListCIntLastIndexOfArgs<'largs> for (&'largs ::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_int_lastIndexOf_t_from(original_self as *const ::list::ListCInt,
                                                      t as *const ::libc::c_int,
                                                      from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCInt::mid](../struct.ListCInt.html#method.mid) method.
  pub trait ListCIntMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::list::ListCInt;
  }
  impl<'largs> ListCIntMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::list::ListCInt {
      let pos = self;
      {
        let mut object: ::list::ListCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_int_mid_to_output_pos(original_self as *const ::list::ListCInt, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListCIntMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::list::ListCInt {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_int_mid_to_output_pos_length(original_self as *const ::list::ListCInt,
                                                              pos,
                                                              length,
                                                              &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCInt::new](../struct.ListCInt.html#method.new) method.
  pub trait ListCIntNewArgs {
    fn exec(self) -> ::list::ListCInt;
  }
  impl<'a> ListCIntNewArgs for &'a ::list::ListCInt {
    fn exec(self) -> ::list::ListCInt {
      let l = self;
      {
        let mut object: ::list::ListCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_int_constructor_l(l as *const ::list::ListCInt, &mut object);
        }
        object
      }
    }
  }
  impl ListCIntNewArgs for () {
    fn exec(self) -> ::list::ListCInt {

      {
        let mut object: ::list::ListCInt =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_int_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCInt::op_add_assign](../struct.ListCInt.html#method.op_add_assign) method.
  pub trait ListCIntOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> &'largs mut ::list::ListCInt;
  }
  impl<'largs> ListCIntOpAddAssignArgs<'largs> for &'largs ::list::ListCInt {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> &'largs mut ::list::ListCInt {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_int_operator_add_assign_l(original_self as *mut ::list::ListCInt,
                                                         l as *const ::list::ListCInt)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListCIntOpAddAssignArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> &'largs mut ::list::ListCInt {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_int_operator_add_assign_t(original_self as *mut ::list::ListCInt,
                                                         t as *const ::libc::c_int)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListCInt::op_shl](../struct.ListCInt.html#method.op_shl) method.
  pub trait ListCIntOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> &'largs mut ::list::ListCInt;
  }
  impl<'largs> ListCIntOpShlArgs<'largs> for &'largs ::list::ListCInt {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> &'largs mut ::list::ListCInt {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_int_operator_shl_l(original_self as *mut ::list::ListCInt,
                                                  l as *const ::list::ListCInt)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListCIntOpShlArgs<'largs> for &'largs ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> &'largs mut ::list::ListCInt {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_int_operator_shl_t(original_self as *mut ::list::ListCInt,
                                                  t as *const ::libc::c_int)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListCInt::swap](../struct.ListCInt.html#method.swap) method.
  pub trait ListCIntSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> ();
  }
  impl<'largs> ListCIntSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_int_swap_i_j(original_self as *mut ::list::ListCInt, i, j) }
    }
  }
  impl<'largs> ListCIntSwapArgs<'largs> for &'largs mut ::list::ListCInt {
    fn exec(self, original_self: &'largs mut ::list::ListCInt) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_int_swap_other(original_self as *mut ::list::ListCInt,
                                              other as *mut ::list::ListCInt)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCInt::value](../struct.ListCInt.html#method.value) method.
  pub trait ListCIntValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int;
  }
  impl<'largs> ListCIntValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int {
      let i = self;
      unsafe { ::ffi::qt_core_c_QList_int_value_i(original_self as *const ::list::ListCInt, i) }
    }
  }
  impl<'largs> ListCIntValueArgs<'largs> for (::libc::c_int, &'largs ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListCInt) -> ::libc::c_int {
      let i = self.0;
      let default_value = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_int_value_i_defaultValue(original_self as *const ::list::ListCInt,
                                                        i,
                                                        default_value as *const ::libc::c_int)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCommandLineOption::append](../struct.ListCommandLineOption.html#method.append) method.
  pub trait ListCommandLineOptionAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListCommandLineOption) -> ();
  }
  impl<'largs> ListCommandLineOptionAppendArgs<'largs> for &'largs ::command_line_option::CommandLineOption {
    fn exec(self, original_self: &'largs mut ::list::ListCommandLineOption) -> () {
      let t = self;
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_append_QCommandLineOption(original_self as *mut ::list::ListCommandLineOption, t as *const ::command_line_option::CommandLineOption) }
    }
  }
  impl<'largs> ListCommandLineOptionAppendArgs<'largs> for &'largs ::list::ListCommandLineOption {
    fn exec(self, original_self: &'largs mut ::list::ListCommandLineOption) -> () {
      let t = self;
      unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_append_QList_QCommandLineOption(original_self as *mut ::list::ListCommandLineOption, t as *const ::list::ListCommandLineOption) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCommandLineOption::mid](../struct.ListCommandLineOption.html#method.mid) method.
  pub trait ListCommandLineOptionMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListCommandLineOption) -> ::list::ListCommandLineOption;
  }
  impl<'largs> ListCommandLineOptionMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListCommandLineOption) -> ::list::ListCommandLineOption {
      let pos = self;
      {
        let mut object: ::list::ListCommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QCommandLineOption_mid_to_output_pos(original_self as *const ::list::ListCommandLineOption, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListCommandLineOptionMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListCommandLineOption) -> ::list::ListCommandLineOption {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListCommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QCommandLineOption_mid_to_output_pos_length(original_self as *const ::list::ListCommandLineOption, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCommandLineOption::new](../struct.ListCommandLineOption.html#method.new) method.
  pub trait ListCommandLineOptionNewArgs {
    fn exec(self) -> ::list::ListCommandLineOption;
  }
  impl<'a> ListCommandLineOptionNewArgs for &'a ::list::ListCommandLineOption {
    fn exec(self) -> ::list::ListCommandLineOption {
      let l = self;
      {
        let mut object: ::list::ListCommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QCommandLineOption_constructor_l(l as *const ::list::ListCommandLineOption,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl ListCommandLineOptionNewArgs for () {
    fn exec(self) -> ::list::ListCommandLineOption {

      {
        let mut object: ::list::ListCommandLineOption =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QCommandLineOption_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCommandLineOption::op_add_assign](../struct.ListCommandLineOption.html#method.op_add_assign) method.
  pub trait ListCommandLineOptionOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListCommandLineOption)
            -> &'largs mut ::list::ListCommandLineOption;
  }
  impl<'largs> ListCommandLineOptionOpAddAssignArgs<'largs> for &'largs ::list::ListCommandLineOption {
    fn exec(self,
            original_self: &'largs mut ::list::ListCommandLineOption)
            -> &'largs mut ::list::ListCommandLineOption {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_operator_add_assign_l(original_self as *mut ::list::ListCommandLineOption, l as *const ::list::ListCommandLineOption) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListCommandLineOptionOpAddAssignArgs<'largs> for &'largs ::command_line_option::CommandLineOption {
    fn exec(self,
            original_self: &'largs mut ::list::ListCommandLineOption)
            -> &'largs mut ::list::ListCommandLineOption {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QCommandLineOption_operator_add_assign_t(original_self as *mut ::list::ListCommandLineOption, t as *const ::command_line_option::CommandLineOption) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListCommandLineOption::op_shl](../struct.ListCommandLineOption.html#method.op_shl) method.
  pub trait ListCommandLineOptionOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListCommandLineOption)
            -> &'largs mut ::list::ListCommandLineOption;
  }
  impl<'largs> ListCommandLineOptionOpShlArgs<'largs> for &'largs ::list::ListCommandLineOption {
    fn exec(self,
            original_self: &'largs mut ::list::ListCommandLineOption)
            -> &'largs mut ::list::ListCommandLineOption {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QCommandLineOption_operator_shl_l(original_self as *mut ::list::ListCommandLineOption, l as *const ::list::ListCommandLineOption)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListCommandLineOptionOpShlArgs<'largs> for &'largs ::command_line_option::CommandLineOption {
    fn exec(self,
            original_self: &'largs mut ::list::ListCommandLineOption)
            -> &'largs mut ::list::ListCommandLineOption {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QCommandLineOption_operator_shl_t(original_self as *mut ::list::ListCommandLineOption, t as *const ::command_line_option::CommandLineOption)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListCommandLineOption::swap](../struct.ListCommandLineOption.html#method.swap) method.
  pub trait ListCommandLineOptionSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListCommandLineOption) -> ();
  }
  impl<'largs> ListCommandLineOptionSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListCommandLineOption) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QCommandLineOption_swap_i_j(original_self as *mut ::list::ListCommandLineOption, i, j)
      }
    }
  }
  impl<'largs> ListCommandLineOptionSwapArgs<'largs> for &'largs mut ::list::ListCommandLineOption {
    fn exec(self, original_self: &'largs mut ::list::ListCommandLineOption) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QCommandLineOption_swap_other(original_self as *mut ::list::ListCommandLineOption,
                                                             other as *mut ::list::ListCommandLineOption)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFileInfo::append](../struct.ListFileInfo.html#method.append) method.
  pub trait ListFileInfoAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> ();
  }
  impl<'largs> ListFileInfoAppendArgs<'largs> for &'largs ::file_info::FileInfo {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_append_QFileInfo(original_self as *mut ::list::ListFileInfo,
                                                          t as *const ::file_info::FileInfo)
      }
    }
  }
  impl<'largs> ListFileInfoAppendArgs<'largs> for &'largs ::list::ListFileInfo {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_append_QList_QFileInfo(original_self as *mut ::list::ListFileInfo,
                                                                t as *const ::list::ListFileInfo)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFileInfo::count](../struct.ListFileInfo.html#method.count) method.
  pub trait ListFileInfoCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::libc::c_int;
  }
  impl<'largs> ListFileInfoCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QFileInfo_count_no_args(original_self as *const ::list::ListFileInfo) }
    }
  }
  impl<'largs> ListFileInfoCountArgs<'largs> for &'largs ::file_info::FileInfo {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_count_t(original_self as *const ::list::ListFileInfo,
                                                 t as *const ::file_info::FileInfo)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFileInfo::index_of](../struct.ListFileInfo.html#method.index_of) method.
  pub trait ListFileInfoIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::libc::c_int;
  }
  impl<'largs> ListFileInfoIndexOfArgs<'largs> for &'largs ::file_info::FileInfo {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_indexOf_t(original_self as *const ::list::ListFileInfo,
                                                   t as *const ::file_info::FileInfo)
      }
    }
  }
  impl<'largs> ListFileInfoIndexOfArgs<'largs> for (&'largs ::file_info::FileInfo, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_indexOf_t_from(original_self as *const ::list::ListFileInfo,
                                                        t as *const ::file_info::FileInfo,
                                                        from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFileInfo::last_index_of](../struct.ListFileInfo.html#method.last_index_of) method.
  pub trait ListFileInfoLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::libc::c_int;
  }
  impl<'largs> ListFileInfoLastIndexOfArgs<'largs> for &'largs ::file_info::FileInfo {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_lastIndexOf_t(original_self as *const ::list::ListFileInfo,
                                                       t as *const ::file_info::FileInfo)
      }
    }
  }
  impl<'largs> ListFileInfoLastIndexOfArgs<'largs> for (&'largs ::file_info::FileInfo, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_lastIndexOf_t_from(original_self as *const ::list::ListFileInfo,
                                                            t as *const ::file_info::FileInfo,
                                                            from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFileInfo::mid](../struct.ListFileInfo.html#method.mid) method.
  pub trait ListFileInfoMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::list::ListFileInfo;
  }
  impl<'largs> ListFileInfoMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::list::ListFileInfo {
      let pos = self;
      {
        let mut object: ::list::ListFileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QFileInfo_mid_to_output_pos(original_self as *const ::list::ListFileInfo,
                                                             pos,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListFileInfoMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::list::ListFileInfo {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListFileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QFileInfo_mid_to_output_pos_length(original_self as *const ::list::ListFileInfo,
                                                                    pos,
                                                                    length,
                                                                    &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFileInfo::new](../struct.ListFileInfo.html#method.new) method.
  pub trait ListFileInfoNewArgs {
    fn exec(self) -> ::list::ListFileInfo;
  }
  impl<'a> ListFileInfoNewArgs for &'a ::list::ListFileInfo {
    fn exec(self) -> ::list::ListFileInfo {
      let l = self;
      {
        let mut object: ::list::ListFileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QFileInfo_constructor_l(l as *const ::list::ListFileInfo, &mut object);
        }
        object
      }
    }
  }
  impl ListFileInfoNewArgs for () {
    fn exec(self) -> ::list::ListFileInfo {

      {
        let mut object: ::list::ListFileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QFileInfo_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFileInfo::op_add_assign](../struct.ListFileInfo.html#method.op_add_assign) method.
  pub trait ListFileInfoOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> &'largs mut ::list::ListFileInfo;
  }
  impl<'largs> ListFileInfoOpAddAssignArgs<'largs> for &'largs ::list::ListFileInfo {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> &'largs mut ::list::ListFileInfo {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QFileInfo_operator_add_assign_l(original_self as *mut ::list::ListFileInfo,
                                                                 l as *const ::list::ListFileInfo)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListFileInfoOpAddAssignArgs<'largs> for &'largs ::file_info::FileInfo {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> &'largs mut ::list::ListFileInfo {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QFileInfo_operator_add_assign_t(original_self as *mut ::list::ListFileInfo,
                                                                 t as *const ::file_info::FileInfo)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListFileInfo::op_shl](../struct.ListFileInfo.html#method.op_shl) method.
  pub trait ListFileInfoOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> &'largs mut ::list::ListFileInfo;
  }
  impl<'largs> ListFileInfoOpShlArgs<'largs> for &'largs ::list::ListFileInfo {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> &'largs mut ::list::ListFileInfo {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_operator_shl_l(original_self as *mut ::list::ListFileInfo,
                                                        l as *const ::list::ListFileInfo)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListFileInfoOpShlArgs<'largs> for &'largs ::file_info::FileInfo {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> &'largs mut ::list::ListFileInfo {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_operator_shl_t(original_self as *mut ::list::ListFileInfo,
                                                        t as *const ::file_info::FileInfo)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListFileInfo::swap](../struct.ListFileInfo.html#method.swap) method.
  pub trait ListFileInfoSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> ();
  }
  impl<'largs> ListFileInfoSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QFileInfo_swap_i_j(original_self as *mut ::list::ListFileInfo, i, j) }
    }
  }
  impl<'largs> ListFileInfoSwapArgs<'largs> for &'largs mut ::list::ListFileInfo {
    fn exec(self, original_self: &'largs mut ::list::ListFileInfo) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QFileInfo_swap_other(original_self as *mut ::list::ListFileInfo,
                                                    other as *mut ::list::ListFileInfo)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFileInfo::value](../struct.ListFileInfo.html#method.value) method.
  pub trait ListFileInfoValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::file_info::FileInfo;
  }
  impl<'largs> ListFileInfoValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::file_info::FileInfo {
      let i = self;
      {
        let mut object: ::file_info::FileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QFileInfo_value_to_output_i(original_self as *const ::list::ListFileInfo,
                                                             i,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListFileInfoValueArgs<'largs> for (::libc::c_int, &'largs ::file_info::FileInfo) {
    fn exec(self, original_self: &'largs ::list::ListFileInfo) -> ::file_info::FileInfo {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::file_info::FileInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QFileInfo_value_to_output_i_defaultValue(original_self as *const ::list::ListFileInfo, i, default_value as *const ::file_info::FileInfo, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListItemSelectionRange::append](../struct.ListItemSelectionRange.html#method.append) method.
  pub trait ListItemSelectionRangeAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListItemSelectionRange) -> ();
  }
  impl<'largs> ListItemSelectionRangeAppendArgs<'largs> for &'largs ::item_selection_range::ItemSelectionRange {
    fn exec(self, original_self: &'largs mut ::list::ListItemSelectionRange) -> () {
      let t = self;
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_append_QItemSelectionRange(original_self as *mut ::list::ListItemSelectionRange, t as *const ::item_selection_range::ItemSelectionRange) }
    }
  }
  impl<'largs> ListItemSelectionRangeAppendArgs<'largs> for &'largs ::list::ListItemSelectionRange {
    fn exec(self, original_self: &'largs mut ::list::ListItemSelectionRange) -> () {
      let t = self;
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_append_QList_QItemSelectionRange(original_self as *mut ::list::ListItemSelectionRange, t as *const ::list::ListItemSelectionRange) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListItemSelectionRange::count](../struct.ListItemSelectionRange.html#method.count) method.
  pub trait ListItemSelectionRangeCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::libc::c_int;
  }
  impl<'largs> ListItemSelectionRangeCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::libc::c_int {

      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_count_no_args(original_self as *const ::list::ListItemSelectionRange)
      }
    }
  }
  impl<'largs> ListItemSelectionRangeCountArgs<'largs> for &'largs ::item_selection_range::ItemSelectionRange {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_count_t(original_self as *const ::list::ListItemSelectionRange,
                                                           t as *const ::item_selection_range::ItemSelectionRange)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListItemSelectionRange::index_of](../struct.ListItemSelectionRange.html#method.index_of) method.
  pub trait ListItemSelectionRangeIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::libc::c_int;
  }
  impl<'largs> ListItemSelectionRangeIndexOfArgs<'largs> for &'largs ::item_selection_range::ItemSelectionRange {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_indexOf_t(original_self as *const ::list::ListItemSelectionRange,
                                                             t as *const ::item_selection_range::ItemSelectionRange)
      }
    }
  }
  impl<'largs> ListItemSelectionRangeIndexOfArgs<'largs>
    for (&'largs ::item_selection_range::ItemSelectionRange, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_indexOf_t_from(original_self as *const ::list::ListItemSelectionRange, t as *const ::item_selection_range::ItemSelectionRange, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListItemSelectionRange::last_index_of](../struct.ListItemSelectionRange.html#method.last_index_of) method.
  pub trait ListItemSelectionRangeLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::libc::c_int;
  }
  impl<'largs> ListItemSelectionRangeLastIndexOfArgs<'largs> for &'largs ::item_selection_range::ItemSelectionRange {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_lastIndexOf_t(original_self as *const ::list::ListItemSelectionRange, t as *const ::item_selection_range::ItemSelectionRange)
      }
    }
  }
  impl<'largs> ListItemSelectionRangeLastIndexOfArgs<'largs>
    for (&'largs ::item_selection_range::ItemSelectionRange, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_lastIndexOf_t_from(original_self as *const ::list::ListItemSelectionRange, t as *const ::item_selection_range::ItemSelectionRange, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListItemSelectionRange::mid](../struct.ListItemSelectionRange.html#method.mid) method.
  pub trait ListItemSelectionRangeMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::list::ListItemSelectionRange;
  }
  impl<'largs> ListItemSelectionRangeMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::list::ListItemSelectionRange {
      let pos = self;
      {
        let mut object: ::list::ListItemSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QItemSelectionRange_mid_to_output_pos(original_self as *const ::list::ListItemSelectionRange, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListItemSelectionRangeMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::list::ListItemSelectionRange {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListItemSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QItemSelectionRange_mid_to_output_pos_length(original_self as *const ::list::ListItemSelectionRange, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListItemSelectionRange::new](../struct.ListItemSelectionRange.html#method.new) method.
  pub trait ListItemSelectionRangeNewArgs {
    fn exec(self) -> ::list::ListItemSelectionRange;
  }
  impl<'a> ListItemSelectionRangeNewArgs for &'a ::list::ListItemSelectionRange {
    fn exec(self) -> ::list::ListItemSelectionRange {
      let l = self;
      {
        let mut object: ::list::ListItemSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QItemSelectionRange_constructor_l(l as *const ::list::ListItemSelectionRange,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl ListItemSelectionRangeNewArgs for () {
    fn exec(self) -> ::list::ListItemSelectionRange {

      {
        let mut object: ::list::ListItemSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QItemSelectionRange_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListItemSelectionRange::op_add_assign](../struct.ListItemSelectionRange.html#method.op_add_assign) method.
  pub trait ListItemSelectionRangeOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListItemSelectionRange)
            -> &'largs mut ::list::ListItemSelectionRange;
  }
  impl<'largs> ListItemSelectionRangeOpAddAssignArgs<'largs> for &'largs ::list::ListItemSelectionRange {
    fn exec(self,
            original_self: &'largs mut ::list::ListItemSelectionRange)
            -> &'largs mut ::list::ListItemSelectionRange {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_operator_add_assign_l(original_self as *mut ::list::ListItemSelectionRange, l as *const ::list::ListItemSelectionRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListItemSelectionRangeOpAddAssignArgs<'largs> for &'largs ::item_selection_range::ItemSelectionRange {
    fn exec(self,
            original_self: &'largs mut ::list::ListItemSelectionRange)
            -> &'largs mut ::list::ListItemSelectionRange {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_operator_add_assign_t(original_self as *mut ::list::ListItemSelectionRange, t as *const ::item_selection_range::ItemSelectionRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListItemSelectionRange::op_shl](../struct.ListItemSelectionRange.html#method.op_shl) method.
  pub trait ListItemSelectionRangeOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListItemSelectionRange)
            -> &'largs mut ::list::ListItemSelectionRange;
  }
  impl<'largs> ListItemSelectionRangeOpShlArgs<'largs> for &'largs ::list::ListItemSelectionRange {
    fn exec(self,
            original_self: &'largs mut ::list::ListItemSelectionRange)
            -> &'largs mut ::list::ListItemSelectionRange {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_operator_shl_l(original_self as *mut ::list::ListItemSelectionRange, l as *const ::list::ListItemSelectionRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListItemSelectionRangeOpShlArgs<'largs> for &'largs ::item_selection_range::ItemSelectionRange {
    fn exec(self,
            original_self: &'largs mut ::list::ListItemSelectionRange)
            -> &'largs mut ::list::ListItemSelectionRange {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QItemSelectionRange_operator_shl_t(original_self as *mut ::list::ListItemSelectionRange, t as *const ::item_selection_range::ItemSelectionRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListItemSelectionRange::swap](../struct.ListItemSelectionRange.html#method.swap) method.
  pub trait ListItemSelectionRangeSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListItemSelectionRange) -> ();
  }
  impl<'largs> ListItemSelectionRangeSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListItemSelectionRange) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_swap_i_j(original_self as *mut ::list::ListItemSelectionRange, i, j)
      }
    }
  }
  impl<'largs> ListItemSelectionRangeSwapArgs<'largs> for &'largs mut ::list::ListItemSelectionRange {
    fn exec(self, original_self: &'largs mut ::list::ListItemSelectionRange) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QItemSelectionRange_swap_other(original_self as *mut ::list::ListItemSelectionRange,
                                                              other as *mut ::list::ListItemSelectionRange)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListItemSelectionRange::value](../struct.ListItemSelectionRange.html#method.value) method.
  pub trait ListItemSelectionRangeValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::item_selection_range::ItemSelectionRange;
  }
  impl<'largs> ListItemSelectionRangeValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::item_selection_range::ItemSelectionRange {
      let i = self;
      {
        let mut object: ::item_selection_range::ItemSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QItemSelectionRange_value_to_output_i(original_self as *const ::list::ListItemSelectionRange, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListItemSelectionRangeValueArgs<'largs>
    for (::libc::c_int, &'largs ::item_selection_range::ItemSelectionRange) {
    fn exec(self, original_self: &'largs ::list::ListItemSelectionRange) -> ::item_selection_range::ItemSelectionRange {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::item_selection_range::ItemSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QItemSelectionRange_value_to_output_i_defaultValue(original_self as *const ::list::ListItemSelectionRange, i, default_value as *const ::item_selection_range::ItemSelectionRange, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocale::append](../struct.ListLocale.html#method.append) method.
  pub trait ListLocaleAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> ();
  }
  impl<'largs> ListLocaleAppendArgs<'largs> for &'largs ::list::ListLocale {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_append_QList_QLocale(original_self as *mut ::list::ListLocale,
                                                            t as *const ::list::ListLocale)
      }
    }
  }
  impl<'largs> ListLocaleAppendArgs<'largs> for &'largs ::locale::Locale {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_append_QLocale(original_self as *mut ::list::ListLocale,
                                                      t as *const ::locale::Locale)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocale::count](../struct.ListLocale.html#method.count) method.
  pub trait ListLocaleCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::libc::c_int;
  }
  impl<'largs> ListLocaleCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QLocale_count_no_args(original_self as *const ::list::ListLocale) }
    }
  }
  impl<'largs> ListLocaleCountArgs<'largs> for &'largs ::locale::Locale {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_count_t(original_self as *const ::list::ListLocale,
                                               t as *const ::locale::Locale)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocaleCountry::append](../struct.ListLocaleCountry.html#method.append) method.
  pub trait ListLocaleCountryAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> ();
  }
  impl<'largs> ListLocaleCountryAppendArgs<'largs> for &'largs ::list::ListLocaleCountry {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> () {
      let t = self;
      unsafe { ::ffi::qt_core_c_QList_QLocale_Country_append_QList_QLocale_Country(original_self as *mut ::list::ListLocaleCountry, t as *const ::list::ListLocaleCountry) }
    }
  }
  impl<'largs> ListLocaleCountryAppendArgs<'largs> for &'largs ::locale::Country {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_Country_append_QLocale_Country(original_self as *mut ::list::ListLocaleCountry,
                                                                      t as *const ::locale::Country)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocaleCountry::count](../struct.ListLocaleCountry.html#method.count) method.
  pub trait ListLocaleCountryCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::libc::c_int;
  }
  impl<'largs> ListLocaleCountryCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QLocale_Country_count_no_args(original_self as *const ::list::ListLocaleCountry) }
    }
  }
  impl<'largs> ListLocaleCountryCountArgs<'largs> for &'largs ::locale::Country {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_Country_count_t(original_self as *const ::list::ListLocaleCountry,
                                                       t as *const ::locale::Country)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocaleCountry::index_of](../struct.ListLocaleCountry.html#method.index_of) method.
  pub trait ListLocaleCountryIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::libc::c_int;
  }
  impl<'largs> ListLocaleCountryIndexOfArgs<'largs> for &'largs ::locale::Country {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_Country_indexOf_t(original_self as *const ::list::ListLocaleCountry,
                                                         t as *const ::locale::Country)
      }
    }
  }
  impl<'largs> ListLocaleCountryIndexOfArgs<'largs> for (&'largs ::locale::Country, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_Country_indexOf_t_from(original_self as *const ::list::ListLocaleCountry,
                                                              t as *const ::locale::Country,
                                                              from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocaleCountry::last_index_of](../struct.ListLocaleCountry.html#method.last_index_of) method.
  pub trait ListLocaleCountryLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::libc::c_int;
  }
  impl<'largs> ListLocaleCountryLastIndexOfArgs<'largs> for &'largs ::locale::Country {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_Country_lastIndexOf_t(original_self as *const ::list::ListLocaleCountry,
                                                             t as *const ::locale::Country)
      }
    }
  }
  impl<'largs> ListLocaleCountryLastIndexOfArgs<'largs> for (&'largs ::locale::Country, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_Country_lastIndexOf_t_from(original_self as *const ::list::ListLocaleCountry,
                                                                  t as *const ::locale::Country,
                                                                  from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocaleCountry::mid](../struct.ListLocaleCountry.html#method.mid) method.
  pub trait ListLocaleCountryMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::list::ListLocaleCountry;
  }
  impl<'largs> ListLocaleCountryMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::list::ListLocaleCountry {
      let pos = self;
      {
        let mut object: ::list::ListLocaleCountry =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_Country_mid_to_output_pos(original_self as *const ::list::ListLocaleCountry,
                                                                   pos,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListLocaleCountryMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::list::ListLocaleCountry {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListLocaleCountry =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_Country_mid_to_output_pos_length(original_self as *const ::list::ListLocaleCountry, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocaleCountry::new](../struct.ListLocaleCountry.html#method.new) method.
  pub trait ListLocaleCountryNewArgs {
    fn exec(self) -> ::list::ListLocaleCountry;
  }
  impl<'a> ListLocaleCountryNewArgs for &'a ::list::ListLocaleCountry {
    fn exec(self) -> ::list::ListLocaleCountry {
      let l = self;
      {
        let mut object: ::list::ListLocaleCountry =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_Country_constructor_l(l as *const ::list::ListLocaleCountry, &mut object);
        }
        object
      }
    }
  }
  impl ListLocaleCountryNewArgs for () {
    fn exec(self) -> ::list::ListLocaleCountry {

      {
        let mut object: ::list::ListLocaleCountry =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_Country_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocaleCountry::op_add_assign](../struct.ListLocaleCountry.html#method.op_add_assign) method.
  pub trait ListLocaleCountryOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> &'largs mut ::list::ListLocaleCountry;
  }
  impl<'largs> ListLocaleCountryOpAddAssignArgs<'largs> for &'largs ::list::ListLocaleCountry {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> &'largs mut ::list::ListLocaleCountry {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_Country_operator_add_assign_l(original_self as *mut ::list::ListLocaleCountry, l as *const ::list::ListLocaleCountry)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListLocaleCountryOpAddAssignArgs<'largs> for &'largs ::locale::Country {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> &'largs mut ::list::ListLocaleCountry {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_Country_operator_add_assign_t(original_self as *mut ::list::ListLocaleCountry, t as *const ::locale::Country)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocaleCountry::op_shl](../struct.ListLocaleCountry.html#method.op_shl) method.
  pub trait ListLocaleCountryOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> &'largs mut ::list::ListLocaleCountry;
  }
  impl<'largs> ListLocaleCountryOpShlArgs<'largs> for &'largs ::list::ListLocaleCountry {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> &'largs mut ::list::ListLocaleCountry {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_Country_operator_shl_l(original_self as *mut ::list::ListLocaleCountry,
                                                                l as *const ::list::ListLocaleCountry)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListLocaleCountryOpShlArgs<'largs> for &'largs ::locale::Country {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> &'largs mut ::list::ListLocaleCountry {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_Country_operator_shl_t(original_self as *mut ::list::ListLocaleCountry,
                                                                t as *const ::locale::Country)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocaleCountry::swap](../struct.ListLocaleCountry.html#method.swap) method.
  pub trait ListLocaleCountrySwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> ();
  }
  impl<'largs> ListLocaleCountrySwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QLocale_Country_swap_i_j(original_self as *mut ::list::ListLocaleCountry, i, j) }
    }
  }
  impl<'largs> ListLocaleCountrySwapArgs<'largs> for &'largs mut ::list::ListLocaleCountry {
    fn exec(self, original_self: &'largs mut ::list::ListLocaleCountry) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_Country_swap_other(original_self as *mut ::list::ListLocaleCountry,
                                                          other as *mut ::list::ListLocaleCountry)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocaleCountry::value](../struct.ListLocaleCountry.html#method.value) method.
  pub trait ListLocaleCountryValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::locale::Country;
  }
  impl<'largs> ListLocaleCountryValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::locale::Country {
      let i = self;
      unsafe { ::ffi::qt_core_c_QList_QLocale_Country_value_i(original_self as *const ::list::ListLocaleCountry, i) }
    }
  }
  impl<'largs> ListLocaleCountryValueArgs<'largs> for (::libc::c_int, &'largs ::locale::Country) {
    fn exec(self, original_self: &'largs ::list::ListLocaleCountry) -> ::locale::Country {
      let i = self.0;
      let default_value = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_Country_value_i_defaultValue(original_self as *const ::list::ListLocaleCountry,
                                                                    i,
                                                                    default_value as *const ::locale::Country)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocale::index_of](../struct.ListLocale.html#method.index_of) method.
  pub trait ListLocaleIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::libc::c_int;
  }
  impl<'largs> ListLocaleIndexOfArgs<'largs> for &'largs ::locale::Locale {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_indexOf_t(original_self as *const ::list::ListLocale,
                                                 t as *const ::locale::Locale)
      }
    }
  }
  impl<'largs> ListLocaleIndexOfArgs<'largs> for (&'largs ::locale::Locale, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_indexOf_t_from(original_self as *const ::list::ListLocale,
                                                      t as *const ::locale::Locale,
                                                      from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocale::last_index_of](../struct.ListLocale.html#method.last_index_of) method.
  pub trait ListLocaleLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::libc::c_int;
  }
  impl<'largs> ListLocaleLastIndexOfArgs<'largs> for &'largs ::locale::Locale {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_lastIndexOf_t(original_self as *const ::list::ListLocale,
                                                     t as *const ::locale::Locale)
      }
    }
  }
  impl<'largs> ListLocaleLastIndexOfArgs<'largs> for (&'largs ::locale::Locale, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_lastIndexOf_t_from(original_self as *const ::list::ListLocale,
                                                          t as *const ::locale::Locale,
                                                          from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocale::mid](../struct.ListLocale.html#method.mid) method.
  pub trait ListLocaleMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::list::ListLocale;
  }
  impl<'largs> ListLocaleMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::list::ListLocale {
      let pos = self;
      {
        let mut object: ::list::ListLocale =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_mid_to_output_pos(original_self as *const ::list::ListLocale,
                                                           pos,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListLocaleMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::list::ListLocale {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListLocale =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_mid_to_output_pos_length(original_self as *const ::list::ListLocale,
                                                                  pos,
                                                                  length,
                                                                  &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocale::new](../struct.ListLocale.html#method.new) method.
  pub trait ListLocaleNewArgs {
    fn exec(self) -> ::list::ListLocale;
  }
  impl<'a> ListLocaleNewArgs for &'a ::list::ListLocale {
    fn exec(self) -> ::list::ListLocale {
      let l = self;
      {
        let mut object: ::list::ListLocale =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_constructor_l(l as *const ::list::ListLocale, &mut object);
        }
        object
      }
    }
  }
  impl ListLocaleNewArgs for () {
    fn exec(self) -> ::list::ListLocale {

      {
        let mut object: ::list::ListLocale =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocale::op_add_assign](../struct.ListLocale.html#method.op_add_assign) method.
  pub trait ListLocaleOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> &'largs mut ::list::ListLocale;
  }
  impl<'largs> ListLocaleOpAddAssignArgs<'largs> for &'largs ::list::ListLocale {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> &'largs mut ::list::ListLocale {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QLocale_operator_add_assign_l(original_self as *mut ::list::ListLocale,
                                                             l as *const ::list::ListLocale)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListLocaleOpAddAssignArgs<'largs> for &'largs ::locale::Locale {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> &'largs mut ::list::ListLocale {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QLocale_operator_add_assign_t(original_self as *mut ::list::ListLocale,
                                                             t as *const ::locale::Locale)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocale::op_shl](../struct.ListLocale.html#method.op_shl) method.
  pub trait ListLocaleOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> &'largs mut ::list::ListLocale;
  }
  impl<'largs> ListLocaleOpShlArgs<'largs> for &'largs ::list::ListLocale {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> &'largs mut ::list::ListLocale {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QLocale_operator_shl_l(original_self as *mut ::list::ListLocale,
                                                      l as *const ::list::ListLocale)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListLocaleOpShlArgs<'largs> for &'largs ::locale::Locale {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> &'largs mut ::list::ListLocale {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QLocale_operator_shl_t(original_self as *mut ::list::ListLocale,
                                                      t as *const ::locale::Locale)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocale::swap](../struct.ListLocale.html#method.swap) method.
  pub trait ListLocaleSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> ();
  }
  impl<'largs> ListLocaleSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QLocale_swap_i_j(original_self as *mut ::list::ListLocale, i, j) }
    }
  }
  impl<'largs> ListLocaleSwapArgs<'largs> for &'largs mut ::list::ListLocale {
    fn exec(self, original_self: &'largs mut ::list::ListLocale) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QLocale_swap_other(original_self as *mut ::list::ListLocale,
                                                  other as *mut ::list::ListLocale)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListLocale::value](../struct.ListLocale.html#method.value) method.
  pub trait ListLocaleValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::locale::Locale;
  }
  impl<'largs> ListLocaleValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::locale::Locale {
      let i = self;
      {
        let mut object: ::locale::Locale =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_value_to_output_i(original_self as *const ::list::ListLocale, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListLocaleValueArgs<'largs> for (::libc::c_int, &'largs ::locale::Locale) {
    fn exec(self, original_self: &'largs ::list::ListLocale) -> ::locale::Locale {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::locale::Locale =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QLocale_value_to_output_i_defaultValue(original_self as *const ::list::ListLocale,
                                                                        i,
                                                                        default_value as *const ::locale::Locale,
                                                                        &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListMimeType::append](../struct.ListMimeType.html#method.append) method.
  pub trait ListMimeTypeAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> ();
  }
  impl<'largs> ListMimeTypeAppendArgs<'largs> for &'largs ::list::ListMimeType {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_append_QList_QMimeType(original_self as *mut ::list::ListMimeType,
                                                                t as *const ::list::ListMimeType)
      }
    }
  }
  impl<'largs> ListMimeTypeAppendArgs<'largs> for &'largs ::mime_type::MimeType {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_append_QMimeType(original_self as *mut ::list::ListMimeType,
                                                          t as *const ::mime_type::MimeType)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListMimeType::count](../struct.ListMimeType.html#method.count) method.
  pub trait ListMimeTypeCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::libc::c_int;
  }
  impl<'largs> ListMimeTypeCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QMimeType_count_no_args(original_self as *const ::list::ListMimeType) }
    }
  }
  impl<'largs> ListMimeTypeCountArgs<'largs> for &'largs ::mime_type::MimeType {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_count_t(original_self as *const ::list::ListMimeType,
                                                 t as *const ::mime_type::MimeType)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListMimeType::index_of](../struct.ListMimeType.html#method.index_of) method.
  pub trait ListMimeTypeIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::libc::c_int;
  }
  impl<'largs> ListMimeTypeIndexOfArgs<'largs> for &'largs ::mime_type::MimeType {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_indexOf_t(original_self as *const ::list::ListMimeType,
                                                   t as *const ::mime_type::MimeType)
      }
    }
  }
  impl<'largs> ListMimeTypeIndexOfArgs<'largs> for (&'largs ::mime_type::MimeType, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_indexOf_t_from(original_self as *const ::list::ListMimeType,
                                                        t as *const ::mime_type::MimeType,
                                                        from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListMimeType::last_index_of](../struct.ListMimeType.html#method.last_index_of) method.
  pub trait ListMimeTypeLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::libc::c_int;
  }
  impl<'largs> ListMimeTypeLastIndexOfArgs<'largs> for &'largs ::mime_type::MimeType {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_lastIndexOf_t(original_self as *const ::list::ListMimeType,
                                                       t as *const ::mime_type::MimeType)
      }
    }
  }
  impl<'largs> ListMimeTypeLastIndexOfArgs<'largs> for (&'largs ::mime_type::MimeType, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_lastIndexOf_t_from(original_self as *const ::list::ListMimeType,
                                                            t as *const ::mime_type::MimeType,
                                                            from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListMimeType::mid](../struct.ListMimeType.html#method.mid) method.
  pub trait ListMimeTypeMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::list::ListMimeType;
  }
  impl<'largs> ListMimeTypeMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::list::ListMimeType {
      let pos = self;
      {
        let mut object: ::list::ListMimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QMimeType_mid_to_output_pos(original_self as *const ::list::ListMimeType,
                                                             pos,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListMimeTypeMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::list::ListMimeType {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListMimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QMimeType_mid_to_output_pos_length(original_self as *const ::list::ListMimeType,
                                                                    pos,
                                                                    length,
                                                                    &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListMimeType::new](../struct.ListMimeType.html#method.new) method.
  pub trait ListMimeTypeNewArgs {
    fn exec(self) -> ::list::ListMimeType;
  }
  impl<'a> ListMimeTypeNewArgs for &'a ::list::ListMimeType {
    fn exec(self) -> ::list::ListMimeType {
      let l = self;
      {
        let mut object: ::list::ListMimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QMimeType_constructor_l(l as *const ::list::ListMimeType, &mut object);
        }
        object
      }
    }
  }
  impl ListMimeTypeNewArgs for () {
    fn exec(self) -> ::list::ListMimeType {

      {
        let mut object: ::list::ListMimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QMimeType_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListMimeType::op_add_assign](../struct.ListMimeType.html#method.op_add_assign) method.
  pub trait ListMimeTypeOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> &'largs mut ::list::ListMimeType;
  }
  impl<'largs> ListMimeTypeOpAddAssignArgs<'largs> for &'largs ::list::ListMimeType {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> &'largs mut ::list::ListMimeType {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QMimeType_operator_add_assign_l(original_self as *mut ::list::ListMimeType,
                                                                 l as *const ::list::ListMimeType)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListMimeTypeOpAddAssignArgs<'largs> for &'largs ::mime_type::MimeType {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> &'largs mut ::list::ListMimeType {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QMimeType_operator_add_assign_t(original_self as *mut ::list::ListMimeType,
                                                                 t as *const ::mime_type::MimeType)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListMimeType::op_shl](../struct.ListMimeType.html#method.op_shl) method.
  pub trait ListMimeTypeOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> &'largs mut ::list::ListMimeType;
  }
  impl<'largs> ListMimeTypeOpShlArgs<'largs> for &'largs ::list::ListMimeType {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> &'largs mut ::list::ListMimeType {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QMimeType_operator_shl_l(original_self as *mut ::list::ListMimeType,
                                                        l as *const ::list::ListMimeType)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListMimeTypeOpShlArgs<'largs> for &'largs ::mime_type::MimeType {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> &'largs mut ::list::ListMimeType {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QMimeType_operator_shl_t(original_self as *mut ::list::ListMimeType,
                                                        t as *const ::mime_type::MimeType)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListMimeType::swap](../struct.ListMimeType.html#method.swap) method.
  pub trait ListMimeTypeSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> ();
  }
  impl<'largs> ListMimeTypeSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QMimeType_swap_i_j(original_self as *mut ::list::ListMimeType, i, j) }
    }
  }
  impl<'largs> ListMimeTypeSwapArgs<'largs> for &'largs mut ::list::ListMimeType {
    fn exec(self, original_self: &'largs mut ::list::ListMimeType) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QMimeType_swap_other(original_self as *mut ::list::ListMimeType,
                                                    other as *mut ::list::ListMimeType)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListMimeType::value](../struct.ListMimeType.html#method.value) method.
  pub trait ListMimeTypeValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::mime_type::MimeType;
  }
  impl<'largs> ListMimeTypeValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::mime_type::MimeType {
      let i = self;
      {
        let mut object: ::mime_type::MimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QMimeType_value_to_output_i(original_self as *const ::list::ListMimeType,
                                                             i,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListMimeTypeValueArgs<'largs> for (::libc::c_int, &'largs ::mime_type::MimeType) {
    fn exec(self, original_self: &'largs ::list::ListMimeType) -> ::mime_type::MimeType {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::mime_type::MimeType =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QMimeType_value_to_output_i_defaultValue(original_self as *const ::list::ListMimeType, i, default_value as *const ::mime_type::MimeType, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListModelIndex::append](../struct.ListModelIndex.html#method.append) method.
  pub trait ListModelIndexAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> ();
  }
  impl<'largs> ListModelIndexAppendArgs<'largs> for &'largs ::list::ListModelIndex {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_append_QList_QModelIndex(original_self as *mut ::list::ListModelIndex,
                                                                    t as *const ::list::ListModelIndex)
      }
    }
  }
  impl<'largs> ListModelIndexAppendArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_append_QModelIndex(original_self as *mut ::list::ListModelIndex,
                                                              t as *const ::model_index::ModelIndex)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListModelIndex::count](../struct.ListModelIndex.html#method.count) method.
  pub trait ListModelIndexCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::libc::c_int;
  }
  impl<'largs> ListModelIndexCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QModelIndex_count_no_args(original_self as *const ::list::ListModelIndex) }
    }
  }
  impl<'largs> ListModelIndexCountArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_count_t(original_self as *const ::list::ListModelIndex,
                                                   t as *const ::model_index::ModelIndex)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListModelIndex::index_of](../struct.ListModelIndex.html#method.index_of) method.
  pub trait ListModelIndexIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::libc::c_int;
  }
  impl<'largs> ListModelIndexIndexOfArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_indexOf_t(original_self as *const ::list::ListModelIndex,
                                                     t as *const ::model_index::ModelIndex)
      }
    }
  }
  impl<'largs> ListModelIndexIndexOfArgs<'largs> for (&'largs ::model_index::ModelIndex, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_indexOf_t_from(original_self as *const ::list::ListModelIndex,
                                                          t as *const ::model_index::ModelIndex,
                                                          from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListModelIndex::last_index_of](../struct.ListModelIndex.html#method.last_index_of) method.
  pub trait ListModelIndexLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::libc::c_int;
  }
  impl<'largs> ListModelIndexLastIndexOfArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_lastIndexOf_t(original_self as *const ::list::ListModelIndex,
                                                         t as *const ::model_index::ModelIndex)
      }
    }
  }
  impl<'largs> ListModelIndexLastIndexOfArgs<'largs> for (&'largs ::model_index::ModelIndex, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_lastIndexOf_t_from(original_self as *const ::list::ListModelIndex,
                                                              t as *const ::model_index::ModelIndex,
                                                              from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListModelIndex::mid](../struct.ListModelIndex.html#method.mid) method.
  pub trait ListModelIndexMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::list::ListModelIndex;
  }
  impl<'largs> ListModelIndexMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::list::ListModelIndex {
      let pos = self;
      {
        let mut object: ::list::ListModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QModelIndex_mid_to_output_pos(original_self as *const ::list::ListModelIndex,
                                                               pos,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListModelIndexMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::list::ListModelIndex {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QModelIndex_mid_to_output_pos_length(original_self as *const ::list::ListModelIndex,
                                                                      pos,
                                                                      length,
                                                                      &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListModelIndex::new](../struct.ListModelIndex.html#method.new) method.
  pub trait ListModelIndexNewArgs {
    fn exec(self) -> ::list::ListModelIndex;
  }
  impl<'a> ListModelIndexNewArgs for &'a ::list::ListModelIndex {
    fn exec(self) -> ::list::ListModelIndex {
      let l = self;
      {
        let mut object: ::list::ListModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QModelIndex_constructor_l(l as *const ::list::ListModelIndex, &mut object);
        }
        object
      }
    }
  }
  impl ListModelIndexNewArgs for () {
    fn exec(self) -> ::list::ListModelIndex {

      {
        let mut object: ::list::ListModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QModelIndex_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListModelIndex::op_add_assign](../struct.ListModelIndex.html#method.op_add_assign) method.
  pub trait ListModelIndexOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> &'largs mut ::list::ListModelIndex;
  }
  impl<'largs> ListModelIndexOpAddAssignArgs<'largs> for &'largs ::list::ListModelIndex {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> &'largs mut ::list::ListModelIndex {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QModelIndex_operator_add_assign_l(original_self as *mut ::list::ListModelIndex,
                                                                   l as *const ::list::ListModelIndex)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListModelIndexOpAddAssignArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> &'largs mut ::list::ListModelIndex {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QModelIndex_operator_add_assign_t(original_self as *mut ::list::ListModelIndex,
                                                                   t as *const ::model_index::ModelIndex)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListModelIndex::op_shl](../struct.ListModelIndex.html#method.op_shl) method.
  pub trait ListModelIndexOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> &'largs mut ::list::ListModelIndex;
  }
  impl<'largs> ListModelIndexOpShlArgs<'largs> for &'largs ::list::ListModelIndex {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> &'largs mut ::list::ListModelIndex {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_operator_shl_l(original_self as *mut ::list::ListModelIndex,
                                                          l as *const ::list::ListModelIndex)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListModelIndexOpShlArgs<'largs> for &'largs ::model_index::ModelIndex {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> &'largs mut ::list::ListModelIndex {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_operator_shl_t(original_self as *mut ::list::ListModelIndex,
                                                          t as *const ::model_index::ModelIndex)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListModelIndex::swap](../struct.ListModelIndex.html#method.swap) method.
  pub trait ListModelIndexSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> ();
  }
  impl<'largs> ListModelIndexSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QModelIndex_swap_i_j(original_self as *mut ::list::ListModelIndex, i, j) }
    }
  }
  impl<'largs> ListModelIndexSwapArgs<'largs> for &'largs mut ::list::ListModelIndex {
    fn exec(self, original_self: &'largs mut ::list::ListModelIndex) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QModelIndex_swap_other(original_self as *mut ::list::ListModelIndex,
                                                      other as *mut ::list::ListModelIndex)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListModelIndex::value](../struct.ListModelIndex.html#method.value) method.
  pub trait ListModelIndexValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::model_index::ModelIndex;
  }
  impl<'largs> ListModelIndexValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::model_index::ModelIndex {
      let i = self;
      {
        let mut object: ::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QModelIndex_value_to_output_i(original_self as *const ::list::ListModelIndex,
                                                               i,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListModelIndexValueArgs<'largs> for (::libc::c_int, &'largs ::model_index::ModelIndex) {
    fn exec(self, original_self: &'largs ::list::ListModelIndex) -> ::model_index::ModelIndex {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::model_index::ModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QModelIndex_value_to_output_i_defaultValue(original_self as *const ::list::ListModelIndex, i, default_value as *const ::model_index::ModelIndex, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListObjectMutPtr::index_of](../struct.ListObjectMutPtr.html#method.index_of) method.
  pub trait ListObjectMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListObjectMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListObjectMutPtrIndexOfArgs<'largs> for &'largs *mut ::object::Object {
    unsafe fn exec(self, original_self: &'largs ::list::ListObjectMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_core_c_QList_QObject_ptr_indexOf_t(original_self as *const ::list::ListObjectMutPtr,
                                                   t as *const *mut ::object::Object)
    }
  }
  impl<'largs> ListObjectMutPtrIndexOfArgs<'largs> for (&'largs *mut ::object::Object, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListObjectMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_core_c_QList_QObject_ptr_indexOf_t_from(original_self as *const ::list::ListObjectMutPtr,
                                                        t as *const *mut ::object::Object,
                                                        from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListObjectMutPtr::last_index_of](../struct.ListObjectMutPtr.html#method.last_index_of) method.
  pub trait ListObjectMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListObjectMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListObjectMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::object::Object {
    unsafe fn exec(self, original_self: &'largs ::list::ListObjectMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_core_c_QList_QObject_ptr_lastIndexOf_t(original_self as *const ::list::ListObjectMutPtr,
                                                       t as *const *mut ::object::Object)
    }
  }
  impl<'largs> ListObjectMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::object::Object, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListObjectMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_core_c_QList_QObject_ptr_lastIndexOf_t_from(original_self as *const ::list::ListObjectMutPtr,
                                                            t as *const *mut ::object::Object,
                                                            from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListObjectMutPtr::mid](../struct.ListObjectMutPtr.html#method.mid) method.
  pub trait ListObjectMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListObjectMutPtr) -> ::list::ListObjectMutPtr;
  }
  impl<'largs> ListObjectMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListObjectMutPtr) -> ::list::ListObjectMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListObjectMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QObject_ptr_mid_to_output_pos(original_self as *const ::list::ListObjectMutPtr,
                                                               pos,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListObjectMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListObjectMutPtr) -> ::list::ListObjectMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListObjectMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QObject_ptr_mid_to_output_pos_length(original_self as *const ::list::ListObjectMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListObjectMutPtr::new](../struct.ListObjectMutPtr.html#method.new) method.
  pub trait ListObjectMutPtrNewArgs {
    fn exec(self) -> ::list::ListObjectMutPtr;
  }
  impl<'a> ListObjectMutPtrNewArgs for &'a ::list::ListObjectMutPtr {
    fn exec(self) -> ::list::ListObjectMutPtr {
      let l = self;
      {
        let mut object: ::list::ListObjectMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QObject_ptr_constructor_l(l as *const ::list::ListObjectMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListObjectMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListObjectMutPtr {

      {
        let mut object: ::list::ListObjectMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QObject_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListObjectMutPtr::swap](../struct.ListObjectMutPtr.html#method.swap) method.
  pub trait ListObjectMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListObjectMutPtr) -> ();
  }
  impl<'largs> ListObjectMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListObjectMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QObject_ptr_swap_i_j(original_self as *mut ::list::ListObjectMutPtr, i, j) }
    }
  }
  impl<'largs> ListObjectMutPtrSwapArgs<'largs> for &'largs mut ::list::ListObjectMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListObjectMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QObject_ptr_swap_other(original_self as *mut ::list::ListObjectMutPtr,
                                                      other as *mut ::list::ListObjectMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairStringString::append](../struct.ListPairPairStringString.html#method.append) method.
  pub trait ListPairPairStringStringAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairStringString) -> ();
  }
  impl<'largs> ListPairPairStringStringAppendArgs<'largs> for &'largs ::list::ListPairPairStringString {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairStringString) -> () {
      let t = self;
      unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_append_QList_QPair_QString_QString(original_self as *mut ::list::ListPairPairStringString, t as *const ::list::ListPairPairStringString) }
    }
  }
  impl<'largs> ListPairPairStringStringAppendArgs<'largs> for &'largs ::pair::PairStringString {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairStringString) -> () {
      let t = self;
      unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_append_QPair_QString_QString(original_self as *mut ::list::ListPairPairStringString, t as *const ::pair::PairStringString) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairStringString::count](../struct.ListPairPairStringString.html#method.count) method.
  pub trait ListPairPairStringStringCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::libc::c_int;
  }
  impl<'largs> ListPairPairStringStringCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_count_no_args(original_self as *const ::list::ListPairPairStringString) }
    }
  }
  impl<'largs> ListPairPairStringStringCountArgs<'largs> for &'largs ::pair::PairStringString {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_count_t(original_self as *const ::list::ListPairPairStringString,
                                                             t as *const ::pair::PairStringString)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairStringString::index_of](../struct.ListPairPairStringString.html#method.index_of) method.
  pub trait ListPairPairStringStringIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::libc::c_int;
  }
  impl<'largs> ListPairPairStringStringIndexOfArgs<'largs> for &'largs ::pair::PairStringString {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_indexOf_t(original_self as *const ::list::ListPairPairStringString, t as *const ::pair::PairStringString)
      }
    }
  }
  impl<'largs> ListPairPairStringStringIndexOfArgs<'largs> for (&'largs ::pair::PairStringString, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_indexOf_t_from(original_self as *const ::list::ListPairPairStringString, t as *const ::pair::PairStringString, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairStringString::last_index_of](../struct.ListPairPairStringString.html#method.last_index_of) method.
  pub trait ListPairPairStringStringLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::libc::c_int;
  }
  impl<'largs> ListPairPairStringStringLastIndexOfArgs<'largs> for &'largs ::pair::PairStringString {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_lastIndexOf_t(original_self as *const ::list::ListPairPairStringString, t as *const ::pair::PairStringString) }
    }
  }
  impl<'largs> ListPairPairStringStringLastIndexOfArgs<'largs> for (&'largs ::pair::PairStringString, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_lastIndexOf_t_from(original_self as *const ::list::ListPairPairStringString, t as *const ::pair::PairStringString, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairStringString::mid](../struct.ListPairPairStringString.html#method.mid) method.
  pub trait ListPairPairStringStringMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::list::ListPairPairStringString;
  }
  impl<'largs> ListPairPairStringStringMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::list::ListPairPairStringString {
      let pos = self;
      {
        let mut object: ::list::ListPairPairStringString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPair_QString_QString_mid_to_output_pos(original_self as *const ::list::ListPairPairStringString, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListPairPairStringStringMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::list::ListPairPairStringString {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListPairPairStringString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPair_QString_QString_mid_to_output_pos_length(original_self as *const ::list::ListPairPairStringString, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairStringString::new](../struct.ListPairPairStringString.html#method.new) method.
  pub trait ListPairPairStringStringNewArgs {
    fn exec(self) -> ::list::ListPairPairStringString;
  }
  impl<'a> ListPairPairStringStringNewArgs for &'a ::list::ListPairPairStringString {
    fn exec(self) -> ::list::ListPairPairStringString {
      let l = self;
      {
        let mut object: ::list::ListPairPairStringString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPair_QString_QString_constructor_l(l as *const ::list::ListPairPairStringString,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl ListPairPairStringStringNewArgs for () {
    fn exec(self) -> ::list::ListPairPairStringString {

      {
        let mut object: ::list::ListPairPairStringString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPair_QString_QString_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairStringString::op_add_assign](../struct.ListPairPairStringString.html#method.op_add_assign) method.
  pub trait ListPairPairStringStringOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairStringString)
            -> &'largs mut ::list::ListPairPairStringString;
  }
  impl<'largs> ListPairPairStringStringOpAddAssignArgs<'largs> for &'largs ::list::ListPairPairStringString {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairStringString)
            -> &'largs mut ::list::ListPairPairStringString {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_operator_add_assign_l(original_self as *mut ::list::ListPairPairStringString, l as *const ::list::ListPairPairStringString) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListPairPairStringStringOpAddAssignArgs<'largs> for &'largs ::pair::PairStringString {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairStringString)
            -> &'largs mut ::list::ListPairPairStringString {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_operator_add_assign_t(original_self as *mut ::list::ListPairPairStringString, t as *const ::pair::PairStringString) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairStringString::op_shl](../struct.ListPairPairStringString.html#method.op_shl) method.
  pub trait ListPairPairStringStringOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairStringString)
            -> &'largs mut ::list::ListPairPairStringString;
  }
  impl<'largs> ListPairPairStringStringOpShlArgs<'largs> for &'largs ::list::ListPairPairStringString {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairStringString)
            -> &'largs mut ::list::ListPairPairStringString {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_operator_shl_l(original_self as *mut ::list::ListPairPairStringString, l as *const ::list::ListPairPairStringString) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListPairPairStringStringOpShlArgs<'largs> for &'largs ::pair::PairStringString {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairStringString)
            -> &'largs mut ::list::ListPairPairStringString {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QPair_QString_QString_operator_shl_t(original_self as *mut ::list::ListPairPairStringString, t as *const ::pair::PairStringString) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairStringString::swap](../struct.ListPairPairStringString.html#method.swap) method.
  pub trait ListPairPairStringStringSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairStringString) -> ();
  }
  impl<'largs> ListPairPairStringStringSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairStringString) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_swap_i_j(original_self as *mut ::list::ListPairPairStringString,
                                                              i,
                                                              j)
      }
    }
  }
  impl<'largs> ListPairPairStringStringSwapArgs<'largs> for &'largs mut ::list::ListPairPairStringString {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairStringString) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QPair_QString_QString_swap_other(original_self as *mut ::list::ListPairPairStringString, other as *mut ::list::ListPairPairStringString)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairStringString::value](../struct.ListPairPairStringString.html#method.value) method.
  pub trait ListPairPairStringStringValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::pair::PairStringString;
  }
  impl<'largs> ListPairPairStringStringValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::pair::PairStringString {
      let i = self;
      {
        let mut object: ::pair::PairStringString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPair_QString_QString_value_to_output_i(original_self as *const ::list::ListPairPairStringString, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListPairPairStringStringValueArgs<'largs> for (::libc::c_int, &'largs ::pair::PairStringString) {
    fn exec(self, original_self: &'largs ::list::ListPairPairStringString) -> ::pair::PairStringString {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::pair::PairStringString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPair_QString_QString_value_to_output_i_defaultValue(original_self as *const ::list::ListPairPairStringString, i, default_value as *const ::pair::PairStringString, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPersistentModelIndex::append](../struct.ListPersistentModelIndex.html#method.append) method.
  pub trait ListPersistentModelIndexAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPersistentModelIndex) -> ();
  }
  impl<'largs> ListPersistentModelIndexAppendArgs<'largs> for &'largs ::list::ListPersistentModelIndex {
    fn exec(self, original_self: &'largs mut ::list::ListPersistentModelIndex) -> () {
      let t = self;
      unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_append_QList_QPersistentModelIndex(original_self as *mut ::list::ListPersistentModelIndex, t as *const ::list::ListPersistentModelIndex) }
    }
  }
  impl<'largs> ListPersistentModelIndexAppendArgs<'largs> for &'largs ::persistent_model_index::PersistentModelIndex {
    fn exec(self, original_self: &'largs mut ::list::ListPersistentModelIndex) -> () {
      let t = self;
      unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_append_QPersistentModelIndex(original_self as *mut ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPersistentModelIndex::count](../struct.ListPersistentModelIndex.html#method.count) method.
  pub trait ListPersistentModelIndexCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::libc::c_int;
  }
  impl<'largs> ListPersistentModelIndexCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_count_no_args(original_self as *const ::list::ListPersistentModelIndex) }
    }
  }
  impl<'largs> ListPersistentModelIndexCountArgs<'largs> for &'largs ::persistent_model_index::PersistentModelIndex {
    fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_count_t(original_self as *const ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPersistentModelIndex::index_of](../struct.ListPersistentModelIndex.html#method.index_of) method.
  pub trait ListPersistentModelIndexIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::libc::c_int;
  }
  impl<'largs> ListPersistentModelIndexIndexOfArgs<'largs> for &'largs ::persistent_model_index::PersistentModelIndex {
    fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_indexOf_t(original_self as *const ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex) }
    }
  }
  impl<'largs> ListPersistentModelIndexIndexOfArgs<'largs> for (&'largs ::persistent_model_index::PersistentModelIndex,::libc::c_int) {

  fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_indexOf_t_from(original_self as *const ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex, from) }
  }
}
  /// This trait represents a set of arguments accepted by [ListPersistentModelIndex::last_index_of](../struct.ListPersistentModelIndex.html#method.last_index_of) method.
  pub trait ListPersistentModelIndexLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::libc::c_int;
  }
  impl<'largs> ListPersistentModelIndexLastIndexOfArgs<'largs> for &'largs ::persistent_model_index::PersistentModelIndex {

  fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::libc::c_int {
    let t = self;
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_lastIndexOf_t(original_self as *const ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex) }
  }
}
  impl<'largs> ListPersistentModelIndexLastIndexOfArgs<'largs> for (&'largs ::persistent_model_index::PersistentModelIndex,::libc::c_int) {

  fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_lastIndexOf_t_from(original_self as *const ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex, from) }
  }
}
  /// This trait represents a set of arguments accepted by [ListPersistentModelIndex::mid](../struct.ListPersistentModelIndex.html#method.mid) method.
  pub trait ListPersistentModelIndexMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::list::ListPersistentModelIndex;
  }
  impl<'largs> ListPersistentModelIndexMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::list::ListPersistentModelIndex {
      let pos = self;
      {
        let mut object: ::list::ListPersistentModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPersistentModelIndex_mid_to_output_pos(original_self as *const ::list::ListPersistentModelIndex, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListPersistentModelIndexMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPersistentModelIndex) -> ::list::ListPersistentModelIndex {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListPersistentModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPersistentModelIndex_mid_to_output_pos_length(original_self as *const ::list::ListPersistentModelIndex, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPersistentModelIndex::new](../struct.ListPersistentModelIndex.html#method.new) method.
  pub trait ListPersistentModelIndexNewArgs {
    fn exec(self) -> ::list::ListPersistentModelIndex;
  }
  impl<'a> ListPersistentModelIndexNewArgs for &'a ::list::ListPersistentModelIndex {
    fn exec(self) -> ::list::ListPersistentModelIndex {
      let l = self;
      {
        let mut object: ::list::ListPersistentModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPersistentModelIndex_constructor_l(l as *const ::list::ListPersistentModelIndex,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl ListPersistentModelIndexNewArgs for () {
    fn exec(self) -> ::list::ListPersistentModelIndex {

      {
        let mut object: ::list::ListPersistentModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPersistentModelIndex_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPersistentModelIndex::op_add_assign](../struct.ListPersistentModelIndex.html#method.op_add_assign) method.
  pub trait ListPersistentModelIndexOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListPersistentModelIndex)
            -> &'largs mut ::list::ListPersistentModelIndex;
  }
  impl<'largs> ListPersistentModelIndexOpAddAssignArgs<'largs> for &'largs ::list::ListPersistentModelIndex {
    fn exec(self,
            original_self: &'largs mut ::list::ListPersistentModelIndex)
            -> &'largs mut ::list::ListPersistentModelIndex {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_operator_add_assign_l(original_self as *mut ::list::ListPersistentModelIndex, l as *const ::list::ListPersistentModelIndex) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListPersistentModelIndexOpAddAssignArgs<'largs> for &'largs ::persistent_model_index::PersistentModelIndex {

  fn exec(self, original_self: &'largs mut ::list::ListPersistentModelIndex) -> &'largs mut ::list::ListPersistentModelIndex {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_operator_add_assign_t(original_self as *mut ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [ListPersistentModelIndex::op_shl](../struct.ListPersistentModelIndex.html#method.op_shl) method.
  pub trait ListPersistentModelIndexOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListPersistentModelIndex)
            -> &'largs mut ::list::ListPersistentModelIndex;
  }
  impl<'largs> ListPersistentModelIndexOpShlArgs<'largs> for &'largs ::list::ListPersistentModelIndex {
    fn exec(self,
            original_self: &'largs mut ::list::ListPersistentModelIndex)
            -> &'largs mut ::list::ListPersistentModelIndex {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_operator_shl_l(original_self as *mut ::list::ListPersistentModelIndex, l as *const ::list::ListPersistentModelIndex) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListPersistentModelIndexOpShlArgs<'largs> for &'largs ::persistent_model_index::PersistentModelIndex {
    fn exec(self,
            original_self: &'largs mut ::list::ListPersistentModelIndex)
            -> &'largs mut ::list::ListPersistentModelIndex {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_core_c_QList_QPersistentModelIndex_operator_shl_t(original_self as *mut ::list::ListPersistentModelIndex, t as *const ::persistent_model_index::PersistentModelIndex) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListPersistentModelIndex::swap](../struct.ListPersistentModelIndex.html#method.swap) method.
  pub trait ListPersistentModelIndexSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPersistentModelIndex) -> ();
  }
  impl<'largs> ListPersistentModelIndexSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListPersistentModelIndex) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_swap_i_j(original_self as *mut ::list::ListPersistentModelIndex,
                                                              i,
                                                              j)
      }
    }
  }
  impl<'largs> ListPersistentModelIndexSwapArgs<'largs> for &'largs mut ::list::ListPersistentModelIndex {
    fn exec(self, original_self: &'largs mut ::list::ListPersistentModelIndex) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QPersistentModelIndex_swap_other(original_self as *mut ::list::ListPersistentModelIndex, other as *mut ::list::ListPersistentModelIndex)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPersistentModelIndex::value](../struct.ListPersistentModelIndex.html#method.value) method.
  pub trait ListPersistentModelIndexValueArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::list::ListPersistentModelIndex)
            -> ::persistent_model_index::PersistentModelIndex;
  }
  impl<'largs> ListPersistentModelIndexValueArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::list::ListPersistentModelIndex)
            -> ::persistent_model_index::PersistentModelIndex {
      let i = self;
      {
        let mut object: ::persistent_model_index::PersistentModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPersistentModelIndex_value_to_output_i(original_self as *const ::list::ListPersistentModelIndex, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListPersistentModelIndexValueArgs<'largs>
    for (::libc::c_int, &'largs ::persistent_model_index::PersistentModelIndex) {
    fn exec(self,
            original_self: &'largs ::list::ListPersistentModelIndex)
            -> ::persistent_model_index::PersistentModelIndex {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::persistent_model_index::PersistentModelIndex =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QPersistentModelIndex_value_to_output_i_defaultValue(original_self as *const ::list::ListPersistentModelIndex, i, default_value as *const ::persistent_model_index::PersistentModelIndex, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtDayOfWeek::append](../struct.ListQtDayOfWeek.html#method.append) method.
  pub trait ListQtDayOfWeekAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> ();
  }
  impl<'largs> ListQtDayOfWeekAppendArgs<'largs> for &'largs ::list::ListQtDayOfWeek {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_Qt_DayOfWeek_append_QList_Qt_DayOfWeek(original_self as *mut ::list::ListQtDayOfWeek,
                                                                      t as *const ::list::ListQtDayOfWeek)
      }
    }
  }
  impl<'largs> ListQtDayOfWeekAppendArgs<'largs> for &'largs ::qt::DayOfWeek {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_Qt_DayOfWeek_append_Qt_DayOfWeek(original_self as *mut ::list::ListQtDayOfWeek,
                                                                t as *const ::qt::DayOfWeek)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtDayOfWeek::count](../struct.ListQtDayOfWeek.html#method.count) method.
  pub trait ListQtDayOfWeekCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::libc::c_int;
  }
  impl<'largs> ListQtDayOfWeekCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_count_no_args(original_self as *const ::list::ListQtDayOfWeek) }
    }
  }
  impl<'largs> ListQtDayOfWeekCountArgs<'largs> for &'largs ::qt::DayOfWeek {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_Qt_DayOfWeek_count_t(original_self as *const ::list::ListQtDayOfWeek,
                                                    t as *const ::qt::DayOfWeek)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtDayOfWeek::index_of](../struct.ListQtDayOfWeek.html#method.index_of) method.
  pub trait ListQtDayOfWeekIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::libc::c_int;
  }
  impl<'largs> ListQtDayOfWeekIndexOfArgs<'largs> for &'largs ::qt::DayOfWeek {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_Qt_DayOfWeek_indexOf_t(original_self as *const ::list::ListQtDayOfWeek,
                                                      t as *const ::qt::DayOfWeek)
      }
    }
  }
  impl<'largs> ListQtDayOfWeekIndexOfArgs<'largs> for (&'largs ::qt::DayOfWeek, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_Qt_DayOfWeek_indexOf_t_from(original_self as *const ::list::ListQtDayOfWeek,
                                                           t as *const ::qt::DayOfWeek,
                                                           from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtDayOfWeek::last_index_of](../struct.ListQtDayOfWeek.html#method.last_index_of) method.
  pub trait ListQtDayOfWeekLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::libc::c_int;
  }
  impl<'largs> ListQtDayOfWeekLastIndexOfArgs<'largs> for &'largs ::qt::DayOfWeek {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_Qt_DayOfWeek_lastIndexOf_t(original_self as *const ::list::ListQtDayOfWeek,
                                                          t as *const ::qt::DayOfWeek)
      }
    }
  }
  impl<'largs> ListQtDayOfWeekLastIndexOfArgs<'largs> for (&'largs ::qt::DayOfWeek, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_Qt_DayOfWeek_lastIndexOf_t_from(original_self as *const ::list::ListQtDayOfWeek,
                                                               t as *const ::qt::DayOfWeek,
                                                               from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtDayOfWeek::mid](../struct.ListQtDayOfWeek.html#method.mid) method.
  pub trait ListQtDayOfWeekMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::list::ListQtDayOfWeek;
  }
  impl<'largs> ListQtDayOfWeekMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::list::ListQtDayOfWeek {
      let pos = self;
      {
        let mut object: ::list::ListQtDayOfWeek =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_Qt_DayOfWeek_mid_to_output_pos(original_self as *const ::list::ListQtDayOfWeek,
                                                                pos,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListQtDayOfWeekMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::list::ListQtDayOfWeek {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListQtDayOfWeek =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_Qt_DayOfWeek_mid_to_output_pos_length(original_self as *const ::list::ListQtDayOfWeek, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtDayOfWeek::new](../struct.ListQtDayOfWeek.html#method.new) method.
  pub trait ListQtDayOfWeekNewArgs {
    fn exec(self) -> ::list::ListQtDayOfWeek;
  }
  impl<'a> ListQtDayOfWeekNewArgs for &'a ::list::ListQtDayOfWeek {
    fn exec(self) -> ::list::ListQtDayOfWeek {
      let l = self;
      {
        let mut object: ::list::ListQtDayOfWeek =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_Qt_DayOfWeek_constructor_l(l as *const ::list::ListQtDayOfWeek, &mut object);
        }
        object
      }
    }
  }
  impl ListQtDayOfWeekNewArgs for () {
    fn exec(self) -> ::list::ListQtDayOfWeek {

      {
        let mut object: ::list::ListQtDayOfWeek =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_Qt_DayOfWeek_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtDayOfWeek::op_add_assign](../struct.ListQtDayOfWeek.html#method.op_add_assign) method.
  pub trait ListQtDayOfWeekOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> &'largs mut ::list::ListQtDayOfWeek;
  }
  impl<'largs> ListQtDayOfWeekOpAddAssignArgs<'largs> for &'largs ::list::ListQtDayOfWeek {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> &'largs mut ::list::ListQtDayOfWeek {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_Qt_DayOfWeek_operator_add_assign_l(original_self as *mut ::list::ListQtDayOfWeek,
                                                                    l as *const ::list::ListQtDayOfWeek)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListQtDayOfWeekOpAddAssignArgs<'largs> for &'largs ::qt::DayOfWeek {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> &'largs mut ::list::ListQtDayOfWeek {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_Qt_DayOfWeek_operator_add_assign_t(original_self as *mut ::list::ListQtDayOfWeek,
                                                                    t as *const ::qt::DayOfWeek)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtDayOfWeek::op_shl](../struct.ListQtDayOfWeek.html#method.op_shl) method.
  pub trait ListQtDayOfWeekOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> &'largs mut ::list::ListQtDayOfWeek;
  }
  impl<'largs> ListQtDayOfWeekOpShlArgs<'largs> for &'largs ::list::ListQtDayOfWeek {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> &'largs mut ::list::ListQtDayOfWeek {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_Qt_DayOfWeek_operator_shl_l(original_self as *mut ::list::ListQtDayOfWeek,
                                                             l as *const ::list::ListQtDayOfWeek)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListQtDayOfWeekOpShlArgs<'largs> for &'largs ::qt::DayOfWeek {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> &'largs mut ::list::ListQtDayOfWeek {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_Qt_DayOfWeek_operator_shl_t(original_self as *mut ::list::ListQtDayOfWeek,
                                                             t as *const ::qt::DayOfWeek)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtDayOfWeek::swap](../struct.ListQtDayOfWeek.html#method.swap) method.
  pub trait ListQtDayOfWeekSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> ();
  }
  impl<'largs> ListQtDayOfWeekSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_swap_i_j(original_self as *mut ::list::ListQtDayOfWeek, i, j) }
    }
  }
  impl<'largs> ListQtDayOfWeekSwapArgs<'largs> for &'largs mut ::list::ListQtDayOfWeek {
    fn exec(self, original_self: &'largs mut ::list::ListQtDayOfWeek) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_Qt_DayOfWeek_swap_other(original_self as *mut ::list::ListQtDayOfWeek,
                                                       other as *mut ::list::ListQtDayOfWeek)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtDayOfWeek::value](../struct.ListQtDayOfWeek.html#method.value) method.
  pub trait ListQtDayOfWeekValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::qt::DayOfWeek;
  }
  impl<'largs> ListQtDayOfWeekValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::qt::DayOfWeek {
      let i = self;
      unsafe { ::ffi::qt_core_c_QList_Qt_DayOfWeek_value_i(original_self as *const ::list::ListQtDayOfWeek, i) }
    }
  }
  impl<'largs> ListQtDayOfWeekValueArgs<'largs> for (::libc::c_int, &'largs ::qt::DayOfWeek) {
    fn exec(self, original_self: &'largs ::list::ListQtDayOfWeek) -> ::qt::DayOfWeek {
      let i = self.0;
      let default_value = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_Qt_DayOfWeek_value_i_defaultValue(original_self as *const ::list::ListQtDayOfWeek,
                                                                 i,
                                                                 default_value as *const ::qt::DayOfWeek)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListStorageInfo::append](../struct.ListStorageInfo.html#method.append) method.
  pub trait ListStorageInfoAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> ();
  }
  impl<'largs> ListStorageInfoAppendArgs<'largs> for &'largs ::list::ListStorageInfo {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_append_QList_QStorageInfo(original_self as *mut ::list::ListStorageInfo,
                                                                      t as *const ::list::ListStorageInfo)
      }
    }
  }
  impl<'largs> ListStorageInfoAppendArgs<'largs> for &'largs ::storage_info::StorageInfo {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_append_QStorageInfo(original_self as *mut ::list::ListStorageInfo,
                                                                t as *const ::storage_info::StorageInfo)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListStorageInfo::count](../struct.ListStorageInfo.html#method.count) method.
  pub trait ListStorageInfoCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::libc::c_int;
  }
  impl<'largs> ListStorageInfoCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QStorageInfo_count_no_args(original_self as *const ::list::ListStorageInfo) }
    }
  }
  impl<'largs> ListStorageInfoCountArgs<'largs> for &'largs ::storage_info::StorageInfo {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_count_t(original_self as *const ::list::ListStorageInfo,
                                                    t as *const ::storage_info::StorageInfo)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListStorageInfo::index_of](../struct.ListStorageInfo.html#method.index_of) method.
  pub trait ListStorageInfoIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::libc::c_int;
  }
  impl<'largs> ListStorageInfoIndexOfArgs<'largs> for &'largs ::storage_info::StorageInfo {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_indexOf_t(original_self as *const ::list::ListStorageInfo,
                                                      t as *const ::storage_info::StorageInfo)
      }
    }
  }
  impl<'largs> ListStorageInfoIndexOfArgs<'largs> for (&'largs ::storage_info::StorageInfo, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_indexOf_t_from(original_self as *const ::list::ListStorageInfo,
                                                           t as *const ::storage_info::StorageInfo,
                                                           from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListStorageInfo::last_index_of](../struct.ListStorageInfo.html#method.last_index_of) method.
  pub trait ListStorageInfoLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::libc::c_int;
  }
  impl<'largs> ListStorageInfoLastIndexOfArgs<'largs> for &'largs ::storage_info::StorageInfo {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_lastIndexOf_t(original_self as *const ::list::ListStorageInfo,
                                                          t as *const ::storage_info::StorageInfo)
      }
    }
  }
  impl<'largs> ListStorageInfoLastIndexOfArgs<'largs> for (&'largs ::storage_info::StorageInfo, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_lastIndexOf_t_from(original_self as *const ::list::ListStorageInfo,
                                                               t as *const ::storage_info::StorageInfo,
                                                               from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListStorageInfo::mid](../struct.ListStorageInfo.html#method.mid) method.
  pub trait ListStorageInfoMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::list::ListStorageInfo;
  }
  impl<'largs> ListStorageInfoMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::list::ListStorageInfo {
      let pos = self;
      {
        let mut object: ::list::ListStorageInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QStorageInfo_mid_to_output_pos(original_self as *const ::list::ListStorageInfo,
                                                                pos,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListStorageInfoMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::list::ListStorageInfo {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListStorageInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QStorageInfo_mid_to_output_pos_length(original_self as *const ::list::ListStorageInfo, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListStorageInfo::new](../struct.ListStorageInfo.html#method.new) method.
  pub trait ListStorageInfoNewArgs {
    fn exec(self) -> ::list::ListStorageInfo;
  }
  impl<'a> ListStorageInfoNewArgs for &'a ::list::ListStorageInfo {
    fn exec(self) -> ::list::ListStorageInfo {
      let l = self;
      {
        let mut object: ::list::ListStorageInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QStorageInfo_constructor_l(l as *const ::list::ListStorageInfo, &mut object);
        }
        object
      }
    }
  }
  impl ListStorageInfoNewArgs for () {
    fn exec(self) -> ::list::ListStorageInfo {

      {
        let mut object: ::list::ListStorageInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QStorageInfo_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListStorageInfo::op_add_assign](../struct.ListStorageInfo.html#method.op_add_assign) method.
  pub trait ListStorageInfoOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> &'largs mut ::list::ListStorageInfo;
  }
  impl<'largs> ListStorageInfoOpAddAssignArgs<'largs> for &'largs ::list::ListStorageInfo {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> &'largs mut ::list::ListStorageInfo {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QStorageInfo_operator_add_assign_l(original_self as *mut ::list::ListStorageInfo,
                                                                    l as *const ::list::ListStorageInfo)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListStorageInfoOpAddAssignArgs<'largs> for &'largs ::storage_info::StorageInfo {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> &'largs mut ::list::ListStorageInfo {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QStorageInfo_operator_add_assign_t(original_self as *mut ::list::ListStorageInfo,
                                                                    t as *const ::storage_info::StorageInfo)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListStorageInfo::op_shl](../struct.ListStorageInfo.html#method.op_shl) method.
  pub trait ListStorageInfoOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> &'largs mut ::list::ListStorageInfo;
  }
  impl<'largs> ListStorageInfoOpShlArgs<'largs> for &'largs ::list::ListStorageInfo {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> &'largs mut ::list::ListStorageInfo {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QStorageInfo_operator_shl_l(original_self as *mut ::list::ListStorageInfo,
                                                             l as *const ::list::ListStorageInfo)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListStorageInfoOpShlArgs<'largs> for &'largs ::storage_info::StorageInfo {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> &'largs mut ::list::ListStorageInfo {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QStorageInfo_operator_shl_t(original_self as *mut ::list::ListStorageInfo,
                                                             t as *const ::storage_info::StorageInfo)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListStorageInfo::swap](../struct.ListStorageInfo.html#method.swap) method.
  pub trait ListStorageInfoSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> ();
  }
  impl<'largs> ListStorageInfoSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QStorageInfo_swap_i_j(original_self as *mut ::list::ListStorageInfo, i, j) }
    }
  }
  impl<'largs> ListStorageInfoSwapArgs<'largs> for &'largs mut ::list::ListStorageInfo {
    fn exec(self, original_self: &'largs mut ::list::ListStorageInfo) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QStorageInfo_swap_other(original_self as *mut ::list::ListStorageInfo,
                                                       other as *mut ::list::ListStorageInfo)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListStorageInfo::value](../struct.ListStorageInfo.html#method.value) method.
  pub trait ListStorageInfoValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::storage_info::StorageInfo;
  }
  impl<'largs> ListStorageInfoValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::storage_info::StorageInfo {
      let i = self;
      {
        let mut object: ::storage_info::StorageInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QStorageInfo_value_to_output_i(original_self as *const ::list::ListStorageInfo,
                                                                i,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListStorageInfoValueArgs<'largs> for (::libc::c_int, &'largs ::storage_info::StorageInfo) {
    fn exec(self, original_self: &'largs ::list::ListStorageInfo) -> ::storage_info::StorageInfo {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::storage_info::StorageInfo =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QStorageInfo_value_to_output_i_defaultValue(original_self as *const ::list::ListStorageInfo, i, default_value as *const ::storage_info::StorageInfo, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListString::append](../struct.ListString.html#method.append) method.
  pub trait ListStringAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> ();
  }
  impl<'largs> ListStringAppendArgs<'largs> for &'largs ::list::ListString {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QString_append_QList_QString(original_self as *mut ::list::ListString,
                                                            t as *const ::list::ListString)
      }
    }
  }
  impl<'largs> ListStringAppendArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QString_append_QString(original_self as *mut ::list::ListString,
                                                      t as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListString::count](../struct.ListString.html#method.count) method.
  pub trait ListStringCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::libc::c_int;
  }
  impl<'largs> ListStringCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QString_count_no_args(original_self as *const ::list::ListString) }
    }
  }
  impl<'largs> ListStringCountArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QString_count_t(original_self as *const ::list::ListString,
                                               t as *const ::string::String)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListString::index_of](../struct.ListString.html#method.index_of) method.
  pub trait ListStringIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::libc::c_int;
  }
  impl<'largs> ListStringIndexOfArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QString_indexOf_t(original_self as *const ::list::ListString,
                                                 t as *const ::string::String)
      }
    }
  }
  impl<'largs> ListStringIndexOfArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QString_indexOf_t_from(original_self as *const ::list::ListString,
                                                      t as *const ::string::String,
                                                      from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListString::last_index_of](../struct.ListString.html#method.last_index_of) method.
  pub trait ListStringLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::libc::c_int;
  }
  impl<'largs> ListStringLastIndexOfArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QString_lastIndexOf_t(original_self as *const ::list::ListString,
                                                     t as *const ::string::String)
      }
    }
  }
  impl<'largs> ListStringLastIndexOfArgs<'largs> for (&'largs ::string::String, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QString_lastIndexOf_t_from(original_self as *const ::list::ListString,
                                                          t as *const ::string::String,
                                                          from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListString::mid](../struct.ListString.html#method.mid) method.
  pub trait ListStringMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::list::ListString;
  }
  impl<'largs> ListStringMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::list::ListString {
      let pos = self;
      {
        let mut object: ::list::ListString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QString_mid_to_output_pos(original_self as *const ::list::ListString,
                                                           pos,
                                                           &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListStringMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::list::ListString {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QString_mid_to_output_pos_length(original_self as *const ::list::ListString,
                                                                  pos,
                                                                  length,
                                                                  &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListString::new](../struct.ListString.html#method.new) method.
  pub trait ListStringNewArgs {
    fn exec(self) -> ::list::ListString;
  }
  impl<'a> ListStringNewArgs for &'a ::list::ListString {
    fn exec(self) -> ::list::ListString {
      let l = self;
      {
        let mut object: ::list::ListString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QString_constructor_l(l as *const ::list::ListString, &mut object);
        }
        object
      }
    }
  }
  impl ListStringNewArgs for () {
    fn exec(self) -> ::list::ListString {

      {
        let mut object: ::list::ListString =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QString_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListString::op_add_assign](../struct.ListString.html#method.op_add_assign) method.
  pub trait ListStringOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> &'largs mut ::list::ListString;
  }
  impl<'largs> ListStringOpAddAssignArgs<'largs> for &'largs ::list::ListString {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> &'largs mut ::list::ListString {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QString_operator_add_assign_l(original_self as *mut ::list::ListString,
                                                             l as *const ::list::ListString)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListStringOpAddAssignArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> &'largs mut ::list::ListString {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QString_operator_add_assign_t(original_self as *mut ::list::ListString,
                                                             t as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListString::op_shl](../struct.ListString.html#method.op_shl) method.
  pub trait ListStringOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> &'largs mut ::list::ListString;
  }
  impl<'largs> ListStringOpShlArgs<'largs> for &'largs ::list::ListString {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> &'largs mut ::list::ListString {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QString_operator_shl_l(original_self as *mut ::list::ListString,
                                                      l as *const ::list::ListString)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListStringOpShlArgs<'largs> for &'largs ::string::String {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> &'largs mut ::list::ListString {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QString_operator_shl_t(original_self as *mut ::list::ListString,
                                                      t as *const ::string::String)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListString::swap](../struct.ListString.html#method.swap) method.
  pub trait ListStringSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> ();
  }
  impl<'largs> ListStringSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QString_swap_i_j(original_self as *mut ::list::ListString, i, j) }
    }
  }
  impl<'largs> ListStringSwapArgs<'largs> for &'largs mut ::list::ListString {
    fn exec(self, original_self: &'largs mut ::list::ListString) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QString_swap_other(original_self as *mut ::list::ListString,
                                                  other as *mut ::list::ListString)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListString::value](../struct.ListString.html#method.value) method.
  pub trait ListStringValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::string::String;
  }
  impl<'largs> ListStringValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::string::String {
      let i = self;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QString_value_to_output_i(original_self as *const ::list::ListString, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListStringValueArgs<'largs> for (::libc::c_int, &'largs ::string::String) {
    fn exec(self, original_self: &'largs ::list::ListString) -> ::string::String {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::string::String =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QString_value_to_output_i_defaultValue(original_self as *const ::list::ListString,
                                                                        i,
                                                                        default_value as *const ::string::String,
                                                                        &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListUrl::append](../struct.ListUrl.html#method.append) method.
  pub trait ListUrlAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> ();
  }
  impl<'largs> ListUrlAppendArgs<'largs> for &'largs ::list::ListUrl {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_append_QList_QUrl(original_self as *mut ::list::ListUrl,
                                                      t as *const ::list::ListUrl)
      }
    }
  }
  impl<'largs> ListUrlAppendArgs<'largs> for &'largs ::url::Url {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_append_QUrl(original_self as *mut ::list::ListUrl,
                                                t as *const ::url::Url)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListUrl::count](../struct.ListUrl.html#method.count) method.
  pub trait ListUrlCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::libc::c_int;
  }
  impl<'largs> ListUrlCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QUrl_count_no_args(original_self as *const ::list::ListUrl) }
    }
  }
  impl<'largs> ListUrlCountArgs<'largs> for &'largs ::url::Url {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_count_t(original_self as *const ::list::ListUrl,
                                            t as *const ::url::Url)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListUrl::index_of](../struct.ListUrl.html#method.index_of) method.
  pub trait ListUrlIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::libc::c_int;
  }
  impl<'largs> ListUrlIndexOfArgs<'largs> for &'largs ::url::Url {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_indexOf_t(original_self as *const ::list::ListUrl,
                                              t as *const ::url::Url)
      }
    }
  }
  impl<'largs> ListUrlIndexOfArgs<'largs> for (&'largs ::url::Url, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_indexOf_t_from(original_self as *const ::list::ListUrl,
                                                   t as *const ::url::Url,
                                                   from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListUrl::last_index_of](../struct.ListUrl.html#method.last_index_of) method.
  pub trait ListUrlLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::libc::c_int;
  }
  impl<'largs> ListUrlLastIndexOfArgs<'largs> for &'largs ::url::Url {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_lastIndexOf_t(original_self as *const ::list::ListUrl,
                                                  t as *const ::url::Url)
      }
    }
  }
  impl<'largs> ListUrlLastIndexOfArgs<'largs> for (&'largs ::url::Url, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_lastIndexOf_t_from(original_self as *const ::list::ListUrl,
                                                       t as *const ::url::Url,
                                                       from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListUrl::mid](../struct.ListUrl.html#method.mid) method.
  pub trait ListUrlMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::list::ListUrl;
  }
  impl<'largs> ListUrlMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::list::ListUrl {
      let pos = self;
      {
        let mut object: ::list::ListUrl =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QUrl_mid_to_output_pos(original_self as *const ::list::ListUrl, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListUrlMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::list::ListUrl {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListUrl =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QUrl_mid_to_output_pos_length(original_self as *const ::list::ListUrl,
                                                               pos,
                                                               length,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListUrl::new](../struct.ListUrl.html#method.new) method.
  pub trait ListUrlNewArgs {
    fn exec(self) -> ::list::ListUrl;
  }
  impl<'a> ListUrlNewArgs for &'a ::list::ListUrl {
    fn exec(self) -> ::list::ListUrl {
      let l = self;
      {
        let mut object: ::list::ListUrl =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QUrl_constructor_l(l as *const ::list::ListUrl, &mut object);
        }
        object
      }
    }
  }
  impl ListUrlNewArgs for () {
    fn exec(self) -> ::list::ListUrl {

      {
        let mut object: ::list::ListUrl =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QUrl_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListUrl::op_add_assign](../struct.ListUrl.html#method.op_add_assign) method.
  pub trait ListUrlOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> &'largs mut ::list::ListUrl;
  }
  impl<'largs> ListUrlOpAddAssignArgs<'largs> for &'largs ::list::ListUrl {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> &'largs mut ::list::ListUrl {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QUrl_operator_add_assign_l(original_self as *mut ::list::ListUrl,
                                                          l as *const ::list::ListUrl)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListUrlOpAddAssignArgs<'largs> for &'largs ::url::Url {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> &'largs mut ::list::ListUrl {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QUrl_operator_add_assign_t(original_self as *mut ::list::ListUrl,
                                                          t as *const ::url::Url)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListUrl::op_shl](../struct.ListUrl.html#method.op_shl) method.
  pub trait ListUrlOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> &'largs mut ::list::ListUrl;
  }
  impl<'largs> ListUrlOpShlArgs<'largs> for &'largs ::list::ListUrl {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> &'largs mut ::list::ListUrl {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QUrl_operator_shl_l(original_self as *mut ::list::ListUrl,
                                                   l as *const ::list::ListUrl)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListUrlOpShlArgs<'largs> for &'largs ::url::Url {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> &'largs mut ::list::ListUrl {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QUrl_operator_shl_t(original_self as *mut ::list::ListUrl,
                                                   t as *const ::url::Url)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListUrl::swap](../struct.ListUrl.html#method.swap) method.
  pub trait ListUrlSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> ();
  }
  impl<'largs> ListUrlSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QUrl_swap_i_j(original_self as *mut ::list::ListUrl, i, j) }
    }
  }
  impl<'largs> ListUrlSwapArgs<'largs> for &'largs mut ::list::ListUrl {
    fn exec(self, original_self: &'largs mut ::list::ListUrl) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QUrl_swap_other(original_self as *mut ::list::ListUrl,
                                               other as *mut ::list::ListUrl)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListUrl::value](../struct.ListUrl.html#method.value) method.
  pub trait ListUrlValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::url::Url;
  }
  impl<'largs> ListUrlValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::url::Url {
      let i = self;
      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QUrl_value_to_output_i(original_self as *const ::list::ListUrl, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListUrlValueArgs<'largs> for (::libc::c_int, &'largs ::url::Url) {
    fn exec(self, original_self: &'largs ::list::ListUrl) -> ::url::Url {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::url::Url = unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QUrl_value_to_output_i_defaultValue(original_self as *const ::list::ListUrl,
                                                                     i,
                                                                     default_value as *const ::url::Url,
                                                                     &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListVariant::append](../struct.ListVariant.html#method.append) method.
  pub trait ListVariantAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> ();
  }
  impl<'largs> ListVariantAppendArgs<'largs> for &'largs ::list::ListVariant {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_append_QList_QVariant(original_self as *mut ::list::ListVariant,
                                                              t as *const ::list::ListVariant)
      }
    }
  }
  impl<'largs> ListVariantAppendArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_append_QVariant(original_self as *mut ::list::ListVariant,
                                                        t as *const ::variant::Variant)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListVariant::count](../struct.ListVariant.html#method.count) method.
  pub trait ListVariantCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::libc::c_int;
  }
  impl<'largs> ListVariantCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::libc::c_int {

      unsafe { ::ffi::qt_core_c_QList_QVariant_count_no_args(original_self as *const ::list::ListVariant) }
    }
  }
  impl<'largs> ListVariantCountArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_count_t(original_self as *const ::list::ListVariant,
                                                t as *const ::variant::Variant)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListVariant::index_of](../struct.ListVariant.html#method.index_of) method.
  pub trait ListVariantIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::libc::c_int;
  }
  impl<'largs> ListVariantIndexOfArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_indexOf_t(original_self as *const ::list::ListVariant,
                                                  t as *const ::variant::Variant)
      }
    }
  }
  impl<'largs> ListVariantIndexOfArgs<'largs> for (&'largs ::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_indexOf_t_from(original_self as *const ::list::ListVariant,
                                                       t as *const ::variant::Variant,
                                                       from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListVariant::last_index_of](../struct.ListVariant.html#method.last_index_of) method.
  pub trait ListVariantLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::libc::c_int;
  }
  impl<'largs> ListVariantLastIndexOfArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_lastIndexOf_t(original_self as *const ::list::ListVariant,
                                                      t as *const ::variant::Variant)
      }
    }
  }
  impl<'largs> ListVariantLastIndexOfArgs<'largs> for (&'largs ::variant::Variant, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_lastIndexOf_t_from(original_self as *const ::list::ListVariant,
                                                           t as *const ::variant::Variant,
                                                           from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListVariant::mid](../struct.ListVariant.html#method.mid) method.
  pub trait ListVariantMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::list::ListVariant;
  }
  impl<'largs> ListVariantMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::list::ListVariant {
      let pos = self;
      {
        let mut object: ::list::ListVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QVariant_mid_to_output_pos(original_self as *const ::list::ListVariant,
                                                            pos,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListVariantMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::list::ListVariant {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QVariant_mid_to_output_pos_length(original_self as *const ::list::ListVariant,
                                                                   pos,
                                                                   length,
                                                                   &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListVariant::new](../struct.ListVariant.html#method.new) method.
  pub trait ListVariantNewArgs {
    fn exec(self) -> ::list::ListVariant;
  }
  impl<'a> ListVariantNewArgs for &'a ::list::ListVariant {
    fn exec(self) -> ::list::ListVariant {
      let l = self;
      {
        let mut object: ::list::ListVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QVariant_constructor_l(l as *const ::list::ListVariant, &mut object);
        }
        object
      }
    }
  }
  impl ListVariantNewArgs for () {
    fn exec(self) -> ::list::ListVariant {

      {
        let mut object: ::list::ListVariant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QVariant_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListVariant::op_add_assign](../struct.ListVariant.html#method.op_add_assign) method.
  pub trait ListVariantOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> &'largs mut ::list::ListVariant;
  }
  impl<'largs> ListVariantOpAddAssignArgs<'largs> for &'largs ::list::ListVariant {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> &'largs mut ::list::ListVariant {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QVariant_operator_add_assign_l(original_self as *mut ::list::ListVariant,
                                                                l as *const ::list::ListVariant)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListVariantOpAddAssignArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> &'largs mut ::list::ListVariant {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_core_c_QList_QVariant_operator_add_assign_t(original_self as *mut ::list::ListVariant,
                                                                t as *const ::variant::Variant)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListVariant::op_shl](../struct.ListVariant.html#method.op_shl) method.
  pub trait ListVariantOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> &'largs mut ::list::ListVariant;
  }
  impl<'largs> ListVariantOpShlArgs<'largs> for &'largs ::list::ListVariant {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> &'largs mut ::list::ListVariant {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QVariant_operator_shl_l(original_self as *mut ::list::ListVariant,
                                                       l as *const ::list::ListVariant)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListVariantOpShlArgs<'largs> for &'largs ::variant::Variant {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> &'largs mut ::list::ListVariant {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_core_c_QList_QVariant_operator_shl_t(original_self as *mut ::list::ListVariant,
                                                       t as *const ::variant::Variant)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListVariant::swap](../struct.ListVariant.html#method.swap) method.
  pub trait ListVariantSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> ();
  }
  impl<'largs> ListVariantSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_core_c_QList_QVariant_swap_i_j(original_self as *mut ::list::ListVariant, i, j) }
    }
  }
  impl<'largs> ListVariantSwapArgs<'largs> for &'largs mut ::list::ListVariant {
    fn exec(self, original_self: &'largs mut ::list::ListVariant) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_core_c_QList_QVariant_swap_other(original_self as *mut ::list::ListVariant,
                                                   other as *mut ::list::ListVariant)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListVariant::value](../struct.ListVariant.html#method.value) method.
  pub trait ListVariantValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::variant::Variant;
  }
  impl<'largs> ListVariantValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::variant::Variant {
      let i = self;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QVariant_value_to_output_i(original_self as *const ::list::ListVariant,
                                                            i,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListVariantValueArgs<'largs> for (::libc::c_int, &'largs ::variant::Variant) {
    fn exec(self, original_self: &'largs ::list::ListVariant) -> ::variant::Variant {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::variant::Variant =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_core_c_QList_QVariant_value_to_output_i_defaultValue(original_self as *const ::list::ListVariant,
                                                                         i,
                                                                         default_value as *const ::variant::Variant,
                                                                         &mut object);
        }
        object
      }
    }
  }
}
