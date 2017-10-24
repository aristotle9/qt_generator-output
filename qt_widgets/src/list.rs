/// C++ type: <span style='color: green;'>```QList<QAbstractButton*>```</span>
#[repr(C)]
pub struct ListAbstractButtonMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_ABSTRACT_BUTTON_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListAbstractButtonMutPtr {
  unsafe fn new_uninitialized() -> ListAbstractButtonMutPtr {
    ListAbstractButtonMutPtr(::std::mem::uninitialized())
  }
}

impl ListAbstractButtonMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::append(const QList<QAbstractButton*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListAbstractButtonMutPtr) {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_append_QList_QAbstractButton_ptr(self as *mut ::list::ListAbstractButtonMutPtr, t as *const ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::append(QAbstractButton* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::abstract_button::AbstractButton) {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_append_QAbstractButton(self as *mut ::list::ListAbstractButtonMutPtr, t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* const & QList<QAbstractButton*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_at(self as *const ::list::ListAbstractButtonMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* const & QList<QAbstractButton*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_back_const(self as *const ::list::ListAbstractButtonMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton*& QList<QAbstractButton*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_back(self as *mut ::list::ListAbstractButtonMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_clear(self as *mut ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* const & QList<QAbstractButton*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_constFirst(self as *const ::list::ListAbstractButtonMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* const & QList<QAbstractButton*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_constLast(self as *const ::list::ListAbstractButtonMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractButton*>::contains(QAbstractButton* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::abstract_button::AbstractButton) -> bool {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_contains(self as *const ::list::ListAbstractButtonMutPtr,
                                                           t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractButton*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_count_no_args(self as *const ::list::ListAbstractButtonMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractButton*>::count(QAbstractButton* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::abstract_button::AbstractButton) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_count_t(self as *const ::list::ListAbstractButtonMutPtr,
                                                          t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractButton*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_empty(self as *const ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractButton*>::endsWith(QAbstractButton* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::abstract_button::AbstractButton) -> bool {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_endsWith(self as *const ::list::ListAbstractButtonMutPtr,
                                                           t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* const & QList<QAbstractButton*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_first_const(self as *const ::list::ListAbstractButtonMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton*& QList<QAbstractButton*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_first(self as *mut ::list::ListAbstractButtonMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* const & QList<QAbstractButton*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_front_const(self as *const ::list::ListAbstractButtonMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton*& QList<QAbstractButton*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_front(self as *mut ::list::ListAbstractButtonMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::abstract_button::AbstractButton) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractButton*>::indexOf(QAbstractButton* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::abstract_button::AbstractButton, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractButton*>::indexOf(QAbstractButton* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListAbstractButtonMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::insert(int i, QAbstractButton* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::abstract_button::AbstractButton) {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_insert(self as *mut ::list::ListAbstractButtonMutPtr,
                                                         i,
                                                         t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractButton*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_isEmpty(self as *const ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* const & QList<QAbstractButton*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_last_const(self as *const ::list::ListAbstractButtonMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::abstract_button::AbstractButton) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractButton*>::lastIndexOf(QAbstractButton* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::abstract_button::AbstractButton, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAbstractButton*>::lastIndexOf(QAbstractButton* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListAbstractButtonMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractButton*& QList<QAbstractButton*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_last(self as *mut ::list::ListAbstractButtonMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractButton*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_length(self as *const ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListAbstractButtonMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*> QList<QAbstractButton*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListAbstractButtonMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*> QList<QAbstractButton*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListAbstractButtonMutPtr
    where Args: overloading::ListAbstractButtonMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_move(self as *mut ::list::ListAbstractButtonMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListAbstractButtonMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAbstractButton*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListAbstractButtonMutPtr) -> ::list::ListAbstractButtonMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAbstractButton*>::QList(const QList<QAbstractButton*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListAbstractButtonMutPtr
    where Args: overloading::ListAbstractButtonMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*> QList<QAbstractButton*>::operator+(const QList<QAbstractButton*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListAbstractButtonMutPtr) -> ::list::ListAbstractButtonMutPtr {
    {
      let mut object: ::list::ListAbstractButtonMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_operator_add_to_output(self as *const ::list::ListAbstractButtonMutPtr, l as *const ::list::ListAbstractButtonMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*>& QList<QAbstractButton*>::operator+=(const QList<QAbstractButton*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListAbstractButtonMutPtr)
                                 -> &'l0 mut ::list::ListAbstractButtonMutPtr {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_operator_add_assign_l(self as *mut ::list::ListAbstractButtonMutPtr, l as *const ::list::ListAbstractButtonMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*>& QList<QAbstractButton*>::operator+=(QAbstractButton* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::abstract_button::AbstractButton)
                                               -> &'l0 mut ::list::ListAbstractButtonMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_operator_add_assign_t(self as *mut ::list::ListAbstractButtonMutPtr, t as *const *mut ::abstract_button::AbstractButton);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*>& QList<QAbstractButton*>::operator=(const QList<QAbstractButton*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListAbstractButtonMutPtr)
                             -> &'l0 mut ::list::ListAbstractButtonMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_operator_assign(self as *mut ::list::ListAbstractButtonMutPtr,
                                                                      l as *const ::list::ListAbstractButtonMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractButton*>::operator==(const QList<QAbstractButton*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListAbstractButtonMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_operator_eq(self as *const ::list::ListAbstractButtonMutPtr,
                                                                l as *const ::list::ListAbstractButtonMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* const & QList<QAbstractButton*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_button::AbstractButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_operator_index_const(self as *const ::list::ListAbstractButtonMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton*& QList<QAbstractButton*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::abstract_button::AbstractButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_operator_index(self as *mut ::list::ListAbstractButtonMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractButton*>::operator!=(const QList<QAbstractButton*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListAbstractButtonMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_operator_neq(self as *const ::list::ListAbstractButtonMutPtr,
                                                                 l as *const ::list::ListAbstractButtonMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*>& QList<QAbstractButton*>::operator<<(const QList<QAbstractButton*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListAbstractButtonMutPtr)
                          -> &'l0 mut ::list::ListAbstractButtonMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_operator_shl_l(self as *mut ::list::ListAbstractButtonMutPtr,
                                                                     l as *const ::list::ListAbstractButtonMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*>& QList<QAbstractButton*>::operator<<(QAbstractButton* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::abstract_button::AbstractButton)
                                        -> &'l0 mut ::list::ListAbstractButtonMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_operator_shl_t(self as *mut ::list::ListAbstractButtonMutPtr, t as *const *mut ::abstract_button::AbstractButton);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_pop_back(self as *mut ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_pop_front(self as *mut ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::prepend(QAbstractButton* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::abstract_button::AbstractButton) {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_prepend(self as *mut ::list::ListAbstractButtonMutPtr,
                                                          t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::push_back(QAbstractButton* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::abstract_button::AbstractButton) {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_push_back(self as *mut ::list::ListAbstractButtonMutPtr,
                                                            t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::push_front(QAbstractButton* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::abstract_button::AbstractButton) {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_push_front(self as *mut ::list::ListAbstractButtonMutPtr,
                                                             t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractButton*>::removeAll(QAbstractButton* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::abstract_button::AbstractButton) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_removeAll(self as *mut ::list::ListAbstractButtonMutPtr,
                                                            t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_removeAt(self as *mut ::list::ListAbstractButtonMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_removeFirst(self as *mut ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_removeLast(self as *mut ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractButton*>::removeOne(QAbstractButton* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::abstract_button::AbstractButton) -> bool {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_removeOne(self as *mut ::list::ListAbstractButtonMutPtr,
                                                            t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::replace(int i, QAbstractButton* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::abstract_button::AbstractButton) {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_replace(self as *mut ::list::ListAbstractButtonMutPtr,
                                                          i,
                                                          t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_reserve(self as *mut ::list::ListAbstractButtonMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAbstractButton*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_size(self as *const ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAbstractButton*>::startsWith(QAbstractButton* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::abstract_button::AbstractButton) -> bool {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_startsWith(self as *const ::list::ListAbstractButtonMutPtr,
                                                             t as *const *mut ::abstract_button::AbstractButton)
  }

  /// C++ method: <span style='color: green;'>```QList<QAbstractButton*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListAbstractButtonMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::swap(QList<QAbstractButton*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAbstractButton*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListAbstractButtonMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAbstractButton* QList<QAbstractButton*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::abstract_button::AbstractButton {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_takeAt(self as *mut ::list::ListAbstractButtonMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* QList<QAbstractButton*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::abstract_button::AbstractButton {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_takeFirst(self as *mut ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* QList<QAbstractButton*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::abstract_button::AbstractButton {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_takeLast(self as *mut ::list::ListAbstractButtonMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* QList<QAbstractButton*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::abstract_button::AbstractButton {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_value_i(self as *const ::list::ListAbstractButtonMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QAbstractButton* QList<QAbstractButton*>::value(int i, QAbstractButton* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::abstract_button::AbstractButton)
                             -> *mut ::abstract_button::AbstractButton {
    ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_value_i_defaultValue(self as *const ::list::ListAbstractButtonMutPtr, i, default_value as *const *mut ::abstract_button::AbstractButton)
  }
}

impl Drop for ::list::ListAbstractButtonMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QAbstractButton*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_destructor(self as *mut ::list::ListAbstractButtonMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QAction*>```</span>
#[repr(C)]
pub struct ListActionMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_ACTION_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListActionMutPtr {
  unsafe fn new_uninitialized() -> ListActionMutPtr {
    ListActionMutPtr(::std::mem::uninitialized())
  }
}

impl ListActionMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QAction*>::append(const QList<QAction*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListActionMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QAction_ptr_append_QList_QAction_ptr(self as *mut ::list::ListActionMutPtr,
                                                                     t as *const ::list::ListActionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::append(QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::action::Action) {
    ::ffi::qt_widgets_c_QList_QAction_ptr_append_QAction(self as *mut ::list::ListActionMutPtr,
                                                         t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```QAction* const & QList<QAction*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::action::Action {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_at(self as *const ::list::ListActionMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAction* const & QList<QAction*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_back_const(self as *const ::list::ListActionMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAction*& QList<QAction*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::action::Action {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_back(self as *mut ::list::ListActionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_clear(self as *mut ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAction* const & QList<QAction*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_constFirst(self as *const ::list::ListActionMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAction* const & QList<QAction*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_constLast(self as *const ::list::ListActionMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAction*>::contains(QAction* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::action::Action) -> bool {
    ::ffi::qt_widgets_c_QList_QAction_ptr_contains(self as *const ::list::ListActionMutPtr,
                                                   t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAction*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_count_no_args(self as *const ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAction*>::count(QAction* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::action::Action) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QAction_ptr_count_t(self as *const ::list::ListActionMutPtr,
                                                  t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAction*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_empty(self as *const ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAction*>::endsWith(QAction* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::action::Action) -> bool {
    ::ffi::qt_widgets_c_QList_QAction_ptr_endsWith(self as *const ::list::ListActionMutPtr,
                                                   t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```QAction* const & QList<QAction*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_first_const(self as *const ::list::ListActionMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAction*& QList<QAction*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::action::Action {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_first(self as *mut ::list::ListActionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAction* const & QList<QAction*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_front_const(self as *const ::list::ListActionMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAction*& QList<QAction*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::action::Action {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_front(self as *mut ::list::ListActionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAction*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::action::Action) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAction*>::indexOf(QAction* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::action::Action, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAction*>::indexOf(QAction* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListActionMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAction*>::insert(int i, QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::action::Action) {
    ::ffi::qt_widgets_c_QList_QAction_ptr_insert(self as *mut ::list::ListActionMutPtr,
                                                 i,
                                                 t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAction*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_isEmpty(self as *const ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAction* const & QList<QAction*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_last_const(self as *const ::list::ListActionMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAction*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::action::Action) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAction*>::lastIndexOf(QAction* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::action::Action, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAction*>::lastIndexOf(QAction* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListActionMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAction*& QList<QAction*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::action::Action {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_last(self as *mut ::list::ListActionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QAction*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_length(self as *const ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QAction*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListActionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAction*> QList<QAction*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListActionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAction*> QList<QAction*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListActionMutPtr
    where Args: overloading::ListActionMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAction*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_move(self as *mut ::list::ListActionMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QAction*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListActionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAction*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListActionMutPtr) -> ::list::ListActionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAction*>::QList(const QList<QAction*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListActionMutPtr
    where Args: overloading::ListActionMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QAction*> QList<QAction*>::operator+(const QList<QAction*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListActionMutPtr) -> ::list::ListActionMutPtr {
    {
      let mut object: ::list::ListActionMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QAction_ptr_operator_add_to_output(self as *const ::list::ListActionMutPtr,
                                                                     l as *const ::list::ListActionMutPtr,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAction*>& QList<QAction*>::operator+=(const QList<QAction*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListActionMutPtr) -> &'l0 mut ::list::ListActionMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QAction_ptr_operator_add_assign_l(self as *mut ::list::ListActionMutPtr,
                                                                    l as *const ::list::ListActionMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAction*>& QList<QAction*>::operator+=(QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::action::Action)
                                               -> &'l0 mut ::list::ListActionMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QList_QAction_ptr_operator_add_assign_t(self as *mut ::list::ListActionMutPtr,
                                                                  t as *const *mut ::action::Action);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAction*>& QList<QAction*>::operator=(const QList<QAction*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListActionMutPtr) -> &'l0 mut ::list::ListActionMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QList_QAction_ptr_operator_assign(self as *mut ::list::ListActionMutPtr,
                                                            l as *const ::list::ListActionMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAction*>::operator==(const QList<QAction*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListActionMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QAction_ptr_operator_eq(self as *const ::list::ListActionMutPtr,
                                                        l as *const ::list::ListActionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAction* const & QList<QAction*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_operator_index_const(self as *const ::list::ListActionMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAction*& QList<QAction*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_operator_index(self as *mut ::list::ListActionMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAction*>::operator!=(const QList<QAction*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListActionMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QAction_ptr_operator_neq(self as *const ::list::ListActionMutPtr,
                                                         l as *const ::list::ListActionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAction*>& QList<QAction*>::operator<<(const QList<QAction*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListActionMutPtr) -> &'l0 mut ::list::ListActionMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QList_QAction_ptr_operator_shl_l(self as *mut ::list::ListActionMutPtr,
                                                           l as *const ::list::ListActionMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAction*>& QList<QAction*>::operator<<(QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::action::Action)
                                        -> &'l0 mut ::list::ListActionMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QAction_ptr_operator_shl_t(self as *mut ::list::ListActionMutPtr,
                                                                          t as *const *mut ::action::Action);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_pop_back(self as *mut ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_pop_front(self as *mut ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::prepend(QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::action::Action) {
    ::ffi::qt_widgets_c_QList_QAction_ptr_prepend(self as *mut ::list::ListActionMutPtr,
                                                  t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::push_back(QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::action::Action) {
    ::ffi::qt_widgets_c_QList_QAction_ptr_push_back(self as *mut ::list::ListActionMutPtr,
                                                    t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::push_front(QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::action::Action) {
    ::ffi::qt_widgets_c_QList_QAction_ptr_push_front(self as *mut ::list::ListActionMutPtr,
                                                     t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAction*>::removeAll(QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::action::Action) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QAction_ptr_removeAll(self as *mut ::list::ListActionMutPtr,
                                                    t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_removeAt(self as *mut ::list::ListActionMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_removeFirst(self as *mut ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_removeLast(self as *mut ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAction*>::removeOne(QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::action::Action) -> bool {
    ::ffi::qt_widgets_c_QList_QAction_ptr_removeOne(self as *mut ::list::ListActionMutPtr,
                                                    t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::replace(int i, QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::action::Action) {
    ::ffi::qt_widgets_c_QList_QAction_ptr_replace(self as *mut ::list::ListActionMutPtr,
                                                  i,
                                                  t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAction*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_reserve(self as *mut ::list::ListActionMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAction*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_size(self as *const ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAction*>::startsWith(QAction* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::action::Action) -> bool {
    ::ffi::qt_widgets_c_QList_QAction_ptr_startsWith(self as *const ::list::ListActionMutPtr,
                                                     t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```QList<QAction*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListActionMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAction*>::swap(QList<QAction*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAction*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListActionMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAction* QList<QAction*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_takeAt(self as *mut ::list::ListActionMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QAction* QList<QAction*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_takeFirst(self as *mut ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAction* QList<QAction*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_takeLast(self as *mut ::list::ListActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAction* QList<QAction*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::action::Action {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_value_i(self as *const ::list::ListActionMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QAction* QList<QAction*>::value(int i, QAction* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self, i: ::libc::c_int, default_value: &*mut ::action::Action) -> *mut ::action::Action {
    ::ffi::qt_widgets_c_QList_QAction_ptr_value_i_defaultValue(self as *const ::list::ListActionMutPtr,
                                                               i,
                                                               default_value as *const *mut ::action::Action)
  }
}

impl Drop for ::list::ListActionMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QAction*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_destructor(self as *mut ::list::ListActionMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QDockWidget*>```</span>
#[repr(C)]
pub struct ListDockWidgetMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_DOCK_WIDGET_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListDockWidgetMutPtr {
  unsafe fn new_uninitialized() -> ListDockWidgetMutPtr {
    ListDockWidgetMutPtr(::std::mem::uninitialized())
  }
}

impl ListDockWidgetMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::append(const QList<QDockWidget*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListDockWidgetMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QDockWidget_ptr_append_QList_QDockWidget_ptr(self as *mut ::list::ListDockWidgetMutPtr, t as *const ::list::ListDockWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::append(QDockWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::dock_widget::DockWidget) {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_append_QDockWidget(self as *mut ::list::ListDockWidgetMutPtr,
                                                                 t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* const & QList<QDockWidget*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_at(self as *const ::list::ListDockWidgetMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* const & QList<QDockWidget*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_back_const(self as *const ::list::ListDockWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDockWidget*& QList<QDockWidget*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_back(self as *mut ::list::ListDockWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_clear(self as *mut ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* const & QList<QDockWidget*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_constFirst(self as *const ::list::ListDockWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* const & QList<QDockWidget*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_constLast(self as *const ::list::ListDockWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QDockWidget*>::contains(QDockWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::dock_widget::DockWidget) -> bool {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_contains(self as *const ::list::ListDockWidgetMutPtr,
                                                       t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```int QList<QDockWidget*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_count_no_args(self as *const ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QDockWidget*>::count(QDockWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::dock_widget::DockWidget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_count_t(self as *const ::list::ListDockWidgetMutPtr,
                                                      t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QDockWidget*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_empty(self as *const ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QDockWidget*>::endsWith(QDockWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::dock_widget::DockWidget) -> bool {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_endsWith(self as *const ::list::ListDockWidgetMutPtr,
                                                       t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* const & QList<QDockWidget*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_first_const(self as *const ::list::ListDockWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDockWidget*& QList<QDockWidget*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_first(self as *mut ::list::ListDockWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* const & QList<QDockWidget*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_front_const(self as *const ::list::ListDockWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDockWidget*& QList<QDockWidget*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_front(self as *mut ::list::ListDockWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QDockWidget*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::dock_widget::DockWidget) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QDockWidget*>::indexOf(QDockWidget* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::dock_widget::DockWidget, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QDockWidget*>::indexOf(QDockWidget* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListDockWidgetMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::insert(int i, QDockWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::dock_widget::DockWidget) {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_insert(self as *mut ::list::ListDockWidgetMutPtr,
                                                     i,
                                                     t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QDockWidget*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_isEmpty(self as *const ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* const & QList<QDockWidget*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_last_const(self as *const ::list::ListDockWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QDockWidget*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::dock_widget::DockWidget) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QDockWidget*>::lastIndexOf(QDockWidget* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::dock_widget::DockWidget, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QDockWidget*>::lastIndexOf(QDockWidget* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListDockWidgetMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDockWidget*& QList<QDockWidget*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_last(self as *mut ::list::ListDockWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QDockWidget*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_length(self as *const ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QDockWidget*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListDockWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QDockWidget*> QList<QDockWidget*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListDockWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QDockWidget*> QList<QDockWidget*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListDockWidgetMutPtr
    where Args: overloading::ListDockWidgetMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_move(self as *mut ::list::ListDockWidgetMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QDockWidget*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListDockWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QDockWidget*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListDockWidgetMutPtr) -> ::list::ListDockWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QDockWidget*>::QList(const QList<QDockWidget*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListDockWidgetMutPtr
    where Args: overloading::ListDockWidgetMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QDockWidget*> QList<QDockWidget*>::operator+(const QList<QDockWidget*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListDockWidgetMutPtr) -> ::list::ListDockWidgetMutPtr {
    {
      let mut object: ::list::ListDockWidgetMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QDockWidget_ptr_operator_add_to_output(self as *const ::list::ListDockWidgetMutPtr,
                                                                         l as *const ::list::ListDockWidgetMutPtr,
                                                                         &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QDockWidget*>& QList<QDockWidget*>::operator+=(const QList<QDockWidget*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListDockWidgetMutPtr)
                                 -> &'l0 mut ::list::ListDockWidgetMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QDockWidget_ptr_operator_add_assign_l(self as *mut ::list::ListDockWidgetMutPtr,
                                                                        l as *const ::list::ListDockWidgetMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QDockWidget*>& QList<QDockWidget*>::operator+=(QDockWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::dock_widget::DockWidget)
                                               -> &'l0 mut ::list::ListDockWidgetMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QList_QDockWidget_ptr_operator_add_assign_t(self as *mut ::list::ListDockWidgetMutPtr,
                                                                      t as *const *mut ::dock_widget::DockWidget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QDockWidget*>& QList<QDockWidget*>::operator=(const QList<QDockWidget*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListDockWidgetMutPtr)
                             -> &'l0 mut ::list::ListDockWidgetMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QDockWidget_ptr_operator_assign(self as *mut ::list::ListDockWidgetMutPtr,
                                                                  l as *const ::list::ListDockWidgetMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QDockWidget*>::operator==(const QList<QDockWidget*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListDockWidgetMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QDockWidget_ptr_operator_eq(self as *const ::list::ListDockWidgetMutPtr,
                                                            l as *const ::list::ListDockWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* const & QList<QDockWidget*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QDockWidget_ptr_operator_index_const(self as *const ::list::ListDockWidgetMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QDockWidget*& QList<QDockWidget*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::dock_widget::DockWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_operator_index(self as *mut ::list::ListDockWidgetMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QDockWidget*>::operator!=(const QList<QDockWidget*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListDockWidgetMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QDockWidget_ptr_operator_neq(self as *const ::list::ListDockWidgetMutPtr,
                                                             l as *const ::list::ListDockWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QDockWidget*>& QList<QDockWidget*>::operator<<(const QList<QDockWidget*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListDockWidgetMutPtr)
                          -> &'l0 mut ::list::ListDockWidgetMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QDockWidget_ptr_operator_shl_l(self as *mut ::list::ListDockWidgetMutPtr,
                                                                 l as *const ::list::ListDockWidgetMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QDockWidget*>& QList<QDockWidget*>::operator<<(QDockWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::dock_widget::DockWidget)
                                        -> &'l0 mut ::list::ListDockWidgetMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QList_QDockWidget_ptr_operator_shl_t(self as *mut ::list::ListDockWidgetMutPtr,
                                                               t as *const *mut ::dock_widget::DockWidget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_pop_back(self as *mut ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_pop_front(self as *mut ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::prepend(QDockWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::dock_widget::DockWidget) {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_prepend(self as *mut ::list::ListDockWidgetMutPtr,
                                                      t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::push_back(QDockWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::dock_widget::DockWidget) {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_push_back(self as *mut ::list::ListDockWidgetMutPtr,
                                                        t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::push_front(QDockWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::dock_widget::DockWidget) {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_push_front(self as *mut ::list::ListDockWidgetMutPtr,
                                                         t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```int QList<QDockWidget*>::removeAll(QDockWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::dock_widget::DockWidget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_removeAll(self as *mut ::list::ListDockWidgetMutPtr,
                                                        t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_removeAt(self as *mut ::list::ListDockWidgetMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_removeFirst(self as *mut ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_removeLast(self as *mut ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QDockWidget*>::removeOne(QDockWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::dock_widget::DockWidget) -> bool {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_removeOne(self as *mut ::list::ListDockWidgetMutPtr,
                                                        t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::replace(int i, QDockWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::dock_widget::DockWidget) {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_replace(self as *mut ::list::ListDockWidgetMutPtr,
                                                      i,
                                                      t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_reserve(self as *mut ::list::ListDockWidgetMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QDockWidget*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_size(self as *const ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QDockWidget*>::startsWith(QDockWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::dock_widget::DockWidget) -> bool {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_startsWith(self as *const ::list::ListDockWidgetMutPtr,
                                                         t as *const *mut ::dock_widget::DockWidget)
  }

  /// C++ method: <span style='color: green;'>```QList<QDockWidget*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListDockWidgetMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::swap(QList<QDockWidget*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QDockWidget*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListDockWidgetMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QDockWidget* QList<QDockWidget*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::dock_widget::DockWidget {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_takeAt(self as *mut ::list::ListDockWidgetMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* QList<QDockWidget*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::dock_widget::DockWidget {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_takeFirst(self as *mut ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* QList<QDockWidget*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::dock_widget::DockWidget {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_takeLast(self as *mut ::list::ListDockWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* QList<QDockWidget*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::dock_widget::DockWidget {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_value_i(self as *const ::list::ListDockWidgetMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QDockWidget* QList<QDockWidget*>::value(int i, QDockWidget* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::dock_widget::DockWidget)
                             -> *mut ::dock_widget::DockWidget {
    ::ffi::qt_widgets_c_QList_QDockWidget_ptr_value_i_defaultValue(self as *const ::list::ListDockWidgetMutPtr, i, default_value as *const *mut ::dock_widget::DockWidget)
  }
}

impl Drop for ::list::ListDockWidgetMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QDockWidget*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QDockWidget_ptr_destructor(self as *mut ::list::ListDockWidgetMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QGesture*>```</span>
#[repr(C)]
pub struct ListGestureMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_GESTURE_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListGestureMutPtr {
  unsafe fn new_uninitialized() -> ListGestureMutPtr {
    ListGestureMutPtr(::std::mem::uninitialized())
  }
}

impl ListGestureMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::append(const QList<QGesture*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListGestureMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGesture_ptr_append_QList_QGesture_ptr(self as *mut ::list::ListGestureMutPtr,
                                                                       t as *const ::list::ListGestureMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::append(QGesture* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::gesture::Gesture) {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_append_QGesture(self as *mut ::list::ListGestureMutPtr,
                                                           t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```QGesture* const & QList<QGesture*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_at(self as *const ::list::ListGestureMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGesture* const & QList<QGesture*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::gesture::Gesture {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_back_const(self as *const ::list::ListGestureMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGesture*& QList<QGesture*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_back(self as *mut ::list::ListGestureMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_clear(self as *mut ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGesture* const & QList<QGesture*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::gesture::Gesture {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_constFirst(self as *const ::list::ListGestureMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGesture* const & QList<QGesture*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::gesture::Gesture {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_constLast(self as *const ::list::ListGestureMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGesture*>::contains(QGesture* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::gesture::Gesture) -> bool {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_contains(self as *const ::list::ListGestureMutPtr,
                                                    t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```int QList<QGesture*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_count_no_args(self as *const ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGesture*>::count(QGesture* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::gesture::Gesture) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_count_t(self as *const ::list::ListGestureMutPtr,
                                                   t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGesture*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_empty(self as *const ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGesture*>::endsWith(QGesture* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::gesture::Gesture) -> bool {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_endsWith(self as *const ::list::ListGestureMutPtr,
                                                    t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```QGesture* const & QList<QGesture*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::gesture::Gesture {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_first_const(self as *const ::list::ListGestureMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGesture*& QList<QGesture*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_first(self as *mut ::list::ListGestureMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGesture* const & QList<QGesture*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::gesture::Gesture {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_front_const(self as *const ::list::ListGestureMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGesture*& QList<QGesture*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_front(self as *mut ::list::ListGestureMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::gesture::Gesture) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGesture*>::indexOf(QGesture* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::gesture::Gesture, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGesture*>::indexOf(QGesture* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGestureMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::insert(int i, QGesture* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::gesture::Gesture) {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_insert(self as *mut ::list::ListGestureMutPtr,
                                                  i,
                                                  t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGesture*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_isEmpty(self as *const ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGesture* const & QList<QGesture*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::gesture::Gesture {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_last_const(self as *const ::list::ListGestureMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::gesture::Gesture) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGesture*>::lastIndexOf(QGesture* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::gesture::Gesture, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGesture*>::lastIndexOf(QGesture* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGestureMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGesture*& QList<QGesture*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::gesture::Gesture {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_last(self as *mut ::list::ListGestureMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QGesture*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_length(self as *const ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListGestureMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGesture*> QList<QGesture*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListGestureMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGesture*> QList<QGesture*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListGestureMutPtr
    where Args: overloading::ListGestureMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_move(self as *mut ::list::ListGestureMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListGestureMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGesture*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListGestureMutPtr) -> ::list::ListGestureMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGesture*>::QList(const QList<QGesture*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListGestureMutPtr
    where Args: overloading::ListGestureMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QGesture*> QList<QGesture*>::operator+(const QList<QGesture*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListGestureMutPtr) -> ::list::ListGestureMutPtr {
    {
      let mut object: ::list::ListGestureMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QGesture_ptr_operator_add_to_output(self as *const ::list::ListGestureMutPtr,
                                                                      l as *const ::list::ListGestureMutPtr,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*>& QList<QGesture*>::operator+=(const QList<QGesture*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListGestureMutPtr)
                                 -> &'l0 mut ::list::ListGestureMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGesture_ptr_operator_add_assign_l(self as *mut ::list::ListGestureMutPtr,
                                                                     l as *const ::list::ListGestureMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*>& QList<QGesture*>::operator+=(QGesture* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::gesture::Gesture)
                                               -> &'l0 mut ::list::ListGestureMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QList_QGesture_ptr_operator_add_assign_t(self as *mut ::list::ListGestureMutPtr,
                                                                   t as *const *mut ::gesture::Gesture);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*>& QList<QGesture*>::operator=(const QList<QGesture*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListGestureMutPtr) -> &'l0 mut ::list::ListGestureMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QList_QGesture_ptr_operator_assign(self as *mut ::list::ListGestureMutPtr,
                                                             l as *const ::list::ListGestureMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGesture*>::operator==(const QList<QGesture*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListGestureMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGesture_ptr_operator_eq(self as *const ::list::ListGestureMutPtr,
                                                         l as *const ::list::ListGestureMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QGesture* const & QList<QGesture*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::gesture::Gesture {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGesture_ptr_operator_index_const(self as *const ::list::ListGestureMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGesture*& QList<QGesture*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::gesture::Gesture {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_operator_index(self as *mut ::list::ListGestureMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGesture*>::operator!=(const QList<QGesture*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListGestureMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGesture_ptr_operator_neq(self as *const ::list::ListGestureMutPtr,
                                                          l as *const ::list::ListGestureMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*>& QList<QGesture*>::operator<<(const QList<QGesture*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListGestureMutPtr) -> &'l0 mut ::list::ListGestureMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QList_QGesture_ptr_operator_shl_l(self as *mut ::list::ListGestureMutPtr,
                                                            l as *const ::list::ListGestureMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*>& QList<QGesture*>::operator<<(QGesture* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::gesture::Gesture)
                                        -> &'l0 mut ::list::ListGestureMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QGesture_ptr_operator_shl_t(self as *mut ::list::ListGestureMutPtr,
                                                                           t as *const *mut ::gesture::Gesture);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_pop_back(self as *mut ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_pop_front(self as *mut ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::prepend(QGesture* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::gesture::Gesture) {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_prepend(self as *mut ::list::ListGestureMutPtr,
                                                   t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::push_back(QGesture* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::gesture::Gesture) {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_push_back(self as *mut ::list::ListGestureMutPtr,
                                                     t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::push_front(QGesture* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::gesture::Gesture) {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_push_front(self as *mut ::list::ListGestureMutPtr,
                                                      t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```int QList<QGesture*>::removeAll(QGesture* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::gesture::Gesture) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_removeAll(self as *mut ::list::ListGestureMutPtr,
                                                     t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_removeAt(self as *mut ::list::ListGestureMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_removeFirst(self as *mut ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_removeLast(self as *mut ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGesture*>::removeOne(QGesture* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::gesture::Gesture) -> bool {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_removeOne(self as *mut ::list::ListGestureMutPtr,
                                                     t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::replace(int i, QGesture* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::gesture::Gesture) {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_replace(self as *mut ::list::ListGestureMutPtr,
                                                   i,
                                                   t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_reserve(self as *mut ::list::ListGestureMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGesture*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_size(self as *const ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGesture*>::startsWith(QGesture* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::gesture::Gesture) -> bool {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_startsWith(self as *const ::list::ListGestureMutPtr,
                                                      t as *const *mut ::gesture::Gesture)
  }

  /// C++ method: <span style='color: green;'>```QList<QGesture*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListGestureMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::swap(QList<QGesture*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGesture*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListGestureMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGesture* QList<QGesture*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::gesture::Gesture {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_takeAt(self as *mut ::list::ListGestureMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QGesture* QList<QGesture*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::gesture::Gesture {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_takeFirst(self as *mut ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGesture* QList<QGesture*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::gesture::Gesture {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_takeLast(self as *mut ::list::ListGestureMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGesture* QList<QGesture*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::gesture::Gesture {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_value_i(self as *const ::list::ListGestureMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QGesture* QList<QGesture*>::value(int i, QGesture* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::gesture::Gesture)
                             -> *mut ::gesture::Gesture {
    ::ffi::qt_widgets_c_QList_QGesture_ptr_value_i_defaultValue(self as *const ::list::ListGestureMutPtr,
                                                                i,
                                                                default_value as *const *mut ::gesture::Gesture)
  }
}

impl Drop for ::list::ListGestureMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QGesture*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_destructor(self as *mut ::list::ListGestureMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QGraphicsItem*>```</span>
#[repr(C)]
pub struct ListGraphicsItemMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_GRAPHICS_ITEM_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListGraphicsItemMutPtr {
  unsafe fn new_uninitialized() -> ListGraphicsItemMutPtr {
    ListGraphicsItemMutPtr(::std::mem::uninitialized())
  }
}

impl ListGraphicsItemMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::append(const QList<QGraphicsItem*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListGraphicsItemMutPtr) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_append_QList_QGraphicsItem_ptr(self as *mut ::list::ListGraphicsItemMutPtr, t as *const ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::append(QGraphicsItem* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_append_QGraphicsItem(self as *mut ::list::ListGraphicsItemMutPtr,
                                                                     t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* const & QList<QGraphicsItem*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_at(self as *const ::list::ListGraphicsItemMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* const & QList<QGraphicsItem*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_back_const(self as *const ::list::ListGraphicsItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem*& QList<QGraphicsItem*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_back(self as *mut ::list::ListGraphicsItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_clear(self as *mut ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* const & QList<QGraphicsItem*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_constFirst(self as *const ::list::ListGraphicsItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* const & QList<QGraphicsItem*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_constLast(self as *const ::list::ListGraphicsItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsItem*>::contains(QGraphicsItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_contains(self as *const ::list::ListGraphicsItemMutPtr,
                                                         t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsItem*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_count_no_args(self as *const ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsItem*>::count(QGraphicsItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::graphics_item::GraphicsItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_count_t(self as *const ::list::ListGraphicsItemMutPtr,
                                                        t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsItem*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_empty(self as *const ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsItem*>::endsWith(QGraphicsItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_endsWith(self as *const ::list::ListGraphicsItemMutPtr,
                                                         t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* const & QList<QGraphicsItem*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_first_const(self as *const ::list::ListGraphicsItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem*& QList<QGraphicsItem*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_first(self as *mut ::list::ListGraphicsItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* const & QList<QGraphicsItem*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_front_const(self as *const ::list::ListGraphicsItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem*& QList<QGraphicsItem*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_front(self as *mut ::list::ListGraphicsItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::graphics_item::GraphicsItem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsItem*>::indexOf(QGraphicsItem* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::graphics_item::GraphicsItem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsItem*>::indexOf(QGraphicsItem* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGraphicsItemMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::insert(int i, QGraphicsItem* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_insert(self as *mut ::list::ListGraphicsItemMutPtr,
                                                       i,
                                                       t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsItem*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_isEmpty(self as *const ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* const & QList<QGraphicsItem*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_last_const(self as *const ::list::ListGraphicsItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::graphics_item::GraphicsItem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsItem*>::lastIndexOf(QGraphicsItem* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::graphics_item::GraphicsItem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsItem*>::lastIndexOf(QGraphicsItem* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGraphicsItemMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem*& QList<QGraphicsItem*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_last(self as *mut ::list::ListGraphicsItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsItem*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_length(self as *const ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QList<QGraphicsItem*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QList<QGraphicsItem*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListGraphicsItemMutPtr
    where Args: overloading::ListGraphicsItemMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_move(self as *mut ::list::ListGraphicsItemMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGraphicsItem*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListGraphicsItemMutPtr) -> ::list::ListGraphicsItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGraphicsItem*>::QList(const QList<QGraphicsItem*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListGraphicsItemMutPtr
    where Args: overloading::ListGraphicsItemMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*> QList<QGraphicsItem*>::operator+(const QList<QGraphicsItem*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListGraphicsItemMutPtr) -> ::list::ListGraphicsItemMutPtr {
    {
      let mut object: ::list::ListGraphicsItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_operator_add_to_output(self as *const ::list::ListGraphicsItemMutPtr, l as *const ::list::ListGraphicsItemMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*>& QList<QGraphicsItem*>::operator+=(const QList<QGraphicsItem*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListGraphicsItemMutPtr)
                                 -> &'l0 mut ::list::ListGraphicsItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_operator_add_assign_l(self as *mut ::list::ListGraphicsItemMutPtr,
                                                                          l as *const ::list::ListGraphicsItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*>& QList<QGraphicsItem*>::operator+=(QGraphicsItem* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::graphics_item::GraphicsItem)
                                               -> &'l0 mut ::list::ListGraphicsItemMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_operator_add_assign_t(self as *mut ::list::ListGraphicsItemMutPtr, t as *const *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*>& QList<QGraphicsItem*>::operator=(const QList<QGraphicsItem*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListGraphicsItemMutPtr)
                             -> &'l0 mut ::list::ListGraphicsItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_operator_assign(self as *mut ::list::ListGraphicsItemMutPtr,
                                                                    l as *const ::list::ListGraphicsItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsItem*>::operator==(const QList<QGraphicsItem*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListGraphicsItemMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_operator_eq(self as *const ::list::ListGraphicsItemMutPtr,
                                                              l as *const ::list::ListGraphicsItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* const & QList<QGraphicsItem*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_operator_index_const(self as *const ::list::ListGraphicsItemMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem*& QList<QGraphicsItem*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::graphics_item::GraphicsItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_operator_index(self as *mut ::list::ListGraphicsItemMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsItem*>::operator!=(const QList<QGraphicsItem*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListGraphicsItemMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_operator_neq(self as *const ::list::ListGraphicsItemMutPtr,
                                                               l as *const ::list::ListGraphicsItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*>& QList<QGraphicsItem*>::operator<<(const QList<QGraphicsItem*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListGraphicsItemMutPtr)
                          -> &'l0 mut ::list::ListGraphicsItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_operator_shl_l(self as *mut ::list::ListGraphicsItemMutPtr,
                                                                   l as *const ::list::ListGraphicsItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*>& QList<QGraphicsItem*>::operator<<(QGraphicsItem* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::graphics_item::GraphicsItem)
                                        -> &'l0 mut ::list::ListGraphicsItemMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_operator_shl_t(self as *mut ::list::ListGraphicsItemMutPtr,
                                                                 t as *const *mut ::graphics_item::GraphicsItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_pop_back(self as *mut ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_pop_front(self as *mut ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::prepend(QGraphicsItem* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_prepend(self as *mut ::list::ListGraphicsItemMutPtr,
                                                        t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::push_back(QGraphicsItem* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_push_back(self as *mut ::list::ListGraphicsItemMutPtr,
                                                          t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::push_front(QGraphicsItem* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_push_front(self as *mut ::list::ListGraphicsItemMutPtr,
                                                           t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsItem*>::removeAll(QGraphicsItem* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::graphics_item::GraphicsItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_removeAll(self as *mut ::list::ListGraphicsItemMutPtr,
                                                          t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_removeAt(self as *mut ::list::ListGraphicsItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_removeFirst(self as *mut ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_removeLast(self as *mut ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsItem*>::removeOne(QGraphicsItem* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_removeOne(self as *mut ::list::ListGraphicsItemMutPtr,
                                                          t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::replace(int i, QGraphicsItem* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::graphics_item::GraphicsItem) {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_replace(self as *mut ::list::ListGraphicsItemMutPtr,
                                                        i,
                                                        t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_reserve(self as *mut ::list::ListGraphicsItemMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsItem*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_size(self as *const ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsItem*>::startsWith(QGraphicsItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::graphics_item::GraphicsItem) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_startsWith(self as *const ::list::ListGraphicsItemMutPtr,
                                                           t as *const *mut ::graphics_item::GraphicsItem)
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsItem*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListGraphicsItemMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::swap(QList<QGraphicsItem*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsItem*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListGraphicsItemMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsItem* QList<QGraphicsItem*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_takeAt(self as *mut ::list::ListGraphicsItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QList<QGraphicsItem*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_takeFirst(self as *mut ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QList<QGraphicsItem*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_takeLast(self as *mut ::list::ListGraphicsItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QList<QGraphicsItem*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::graphics_item::GraphicsItem {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_value_i(self as *const ::list::ListGraphicsItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsItem* QList<QGraphicsItem*>::value(int i, QGraphicsItem* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::graphics_item::GraphicsItem)
                             -> *mut ::graphics_item::GraphicsItem {
    ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_value_i_defaultValue(self as *const ::list::ListGraphicsItemMutPtr, i, default_value as *const *mut ::graphics_item::GraphicsItem)
  }
}

impl Drop for ::list::ListGraphicsItemMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QGraphicsItem*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_destructor(self as *mut ::list::ListGraphicsItemMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QGraphicsTransform*>```</span>
#[repr(C)]
pub struct ListGraphicsTransformMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_GRAPHICS_TRANSFORM_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListGraphicsTransformMutPtr {
  unsafe fn new_uninitialized() -> ListGraphicsTransformMutPtr {
    ListGraphicsTransformMutPtr(::std::mem::uninitialized())
  }
}

impl ListGraphicsTransformMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::append(const QList<QGraphicsTransform*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListGraphicsTransformMutPtr) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_append_QList_QGraphicsTransform_ptr(self as *mut ::list::ListGraphicsTransformMutPtr, t as *const ::list::ListGraphicsTransformMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::append(QGraphicsTransform* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::graphics_transform::GraphicsTransform) {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_append_QGraphicsTransform(self as *mut ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* const & QList<QGraphicsTransform*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::graphics_transform::GraphicsTransform {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_at(self as *const ::list::ListGraphicsTransformMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* const & QList<QGraphicsTransform*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::graphics_transform::GraphicsTransform {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_back_const(self as *const ::list::ListGraphicsTransformMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform*& QList<QGraphicsTransform*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_transform::GraphicsTransform {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_back(self as *mut ::list::ListGraphicsTransformMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_clear(self as *mut ::list::ListGraphicsTransformMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* const & QList<QGraphicsTransform*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::graphics_transform::GraphicsTransform {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_constFirst(self as *const ::list::ListGraphicsTransformMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* const & QList<QGraphicsTransform*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::graphics_transform::GraphicsTransform {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_constLast(self as *const ::list::ListGraphicsTransformMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsTransform*>::contains(QGraphicsTransform* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::graphics_transform::GraphicsTransform) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_contains(self as *const ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsTransform*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_count_no_args(self as *const ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsTransform*>::count(QGraphicsTransform* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::graphics_transform::GraphicsTransform) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_count_t(self as *const ::list::ListGraphicsTransformMutPtr,
                                                             t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsTransform*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_empty(self as *const ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsTransform*>::endsWith(QGraphicsTransform* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::graphics_transform::GraphicsTransform) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_endsWith(self as *const ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* const & QList<QGraphicsTransform*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::graphics_transform::GraphicsTransform {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_first_const(self as *const ::list::ListGraphicsTransformMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform*& QList<QGraphicsTransform*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_transform::GraphicsTransform {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_first(self as *mut ::list::ListGraphicsTransformMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* const & QList<QGraphicsTransform*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::graphics_transform::GraphicsTransform {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_front_const(self as *const ::list::ListGraphicsTransformMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform*& QList<QGraphicsTransform*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_transform::GraphicsTransform {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_front(self as *mut ::list::ListGraphicsTransformMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::graphics_transform::GraphicsTransform) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsTransform*>::indexOf(QGraphicsTransform* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::graphics_transform::GraphicsTransform, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsTransform*>::indexOf(QGraphicsTransform* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGraphicsTransformMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::insert(int i, QGraphicsTransform* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::graphics_transform::GraphicsTransform) {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_insert(self as *mut ::list::ListGraphicsTransformMutPtr,
                                                            i,
                                                            t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsTransform*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_isEmpty(self as *const ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* const & QList<QGraphicsTransform*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::graphics_transform::GraphicsTransform {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_last_const(self as *const ::list::ListGraphicsTransformMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::graphics_transform::GraphicsTransform) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsTransform*>::lastIndexOf(QGraphicsTransform* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::graphics_transform::GraphicsTransform, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsTransform*>::lastIndexOf(QGraphicsTransform* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGraphicsTransformMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsTransform*& QList<QGraphicsTransform*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_transform::GraphicsTransform {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_last(self as *mut ::list::ListGraphicsTransformMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsTransform*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_length(self as *const ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListGraphicsTransformMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*> QList<QGraphicsTransform*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListGraphicsTransformMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*> QList<QGraphicsTransform*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListGraphicsTransformMutPtr
    where Args: overloading::ListGraphicsTransformMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_move(self as *mut ::list::ListGraphicsTransformMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListGraphicsTransformMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGraphicsTransform*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListGraphicsTransformMutPtr) -> ::list::ListGraphicsTransformMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGraphicsTransform*>::QList(const QList<QGraphicsTransform*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListGraphicsTransformMutPtr
    where Args: overloading::ListGraphicsTransformMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*> QList<QGraphicsTransform*>::operator+(const QList<QGraphicsTransform*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListGraphicsTransformMutPtr) -> ::list::ListGraphicsTransformMutPtr {
    {
      let mut object: ::list::ListGraphicsTransformMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_operator_add_to_output(self as *const ::list::ListGraphicsTransformMutPtr, l as *const ::list::ListGraphicsTransformMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*>& QList<QGraphicsTransform*>::operator+=(const QList<QGraphicsTransform*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListGraphicsTransformMutPtr)
                                 -> &'l0 mut ::list::ListGraphicsTransformMutPtr {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_operator_add_assign_l(self as *mut ::list::ListGraphicsTransformMutPtr, l as *const ::list::ListGraphicsTransformMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*>& QList<QGraphicsTransform*>::operator+=(QGraphicsTransform* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::graphics_transform::GraphicsTransform)
                                               -> &'l0 mut ::list::ListGraphicsTransformMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_operator_add_assign_t(self as *mut ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*>& QList<QGraphicsTransform*>::operator=(const QList<QGraphicsTransform*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListGraphicsTransformMutPtr)
                             -> &'l0 mut ::list::ListGraphicsTransformMutPtr {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_operator_assign(self as *mut ::list::ListGraphicsTransformMutPtr, l as *const ::list::ListGraphicsTransformMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsTransform*>::operator==(const QList<QGraphicsTransform*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListGraphicsTransformMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_operator_eq(self as *const ::list::ListGraphicsTransformMutPtr,
                                                                   l as *const ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* const & QList<QGraphicsTransform*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::graphics_transform::GraphicsTransform {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_operator_index_const(self as *const ::list::ListGraphicsTransformMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform*& QList<QGraphicsTransform*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::graphics_transform::GraphicsTransform {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_operator_index(self as *mut ::list::ListGraphicsTransformMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsTransform*>::operator!=(const QList<QGraphicsTransform*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListGraphicsTransformMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_operator_neq(self as *const ::list::ListGraphicsTransformMutPtr, l as *const ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*>& QList<QGraphicsTransform*>::operator<<(const QList<QGraphicsTransform*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListGraphicsTransformMutPtr)
                          -> &'l0 mut ::list::ListGraphicsTransformMutPtr {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_operator_shl_l(self as *mut ::list::ListGraphicsTransformMutPtr, l as *const ::list::ListGraphicsTransformMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*>& QList<QGraphicsTransform*>::operator<<(QGraphicsTransform* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::graphics_transform::GraphicsTransform)
                                        -> &'l0 mut ::list::ListGraphicsTransformMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_operator_shl_t(self as *mut ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_pop_back(self as *mut ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_pop_front(self as *mut ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::prepend(QGraphicsTransform* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::graphics_transform::GraphicsTransform) {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_prepend(self as *mut ::list::ListGraphicsTransformMutPtr,
                                                             t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::push_back(QGraphicsTransform* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::graphics_transform::GraphicsTransform) {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_push_back(self as *mut ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::push_front(QGraphicsTransform* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::graphics_transform::GraphicsTransform) {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_push_front(self as *mut ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsTransform*>::removeAll(QGraphicsTransform* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::graphics_transform::GraphicsTransform) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_removeAll(self as *mut ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_removeAt(self as *mut ::list::ListGraphicsTransformMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_removeFirst(self as *mut ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_removeLast(self as *mut ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsTransform*>::removeOne(QGraphicsTransform* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::graphics_transform::GraphicsTransform) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_removeOne(self as *mut ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::replace(int i, QGraphicsTransform* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::graphics_transform::GraphicsTransform) {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_replace(self as *mut ::list::ListGraphicsTransformMutPtr,
                                                             i,
                                                             t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_reserve(self as *mut ::list::ListGraphicsTransformMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsTransform*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_size(self as *const ::list::ListGraphicsTransformMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsTransform*>::startsWith(QGraphicsTransform* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::graphics_transform::GraphicsTransform) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_startsWith(self as *const ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform)
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsTransform*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListGraphicsTransformMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::swap(QList<QGraphicsTransform*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsTransform*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListGraphicsTransformMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsTransform* QList<QGraphicsTransform*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::graphics_transform::GraphicsTransform {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_takeAt(self as *mut ::list::ListGraphicsTransformMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* QList<QGraphicsTransform*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::graphics_transform::GraphicsTransform {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_takeFirst(self as *mut ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* QList<QGraphicsTransform*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::graphics_transform::GraphicsTransform {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_takeLast(self as *mut ::list::ListGraphicsTransformMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* QList<QGraphicsTransform*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::graphics_transform::GraphicsTransform {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_value_i(self as *const ::list::ListGraphicsTransformMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsTransform* QList<QGraphicsTransform*>::value(int i, QGraphicsTransform* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::graphics_transform::GraphicsTransform)
                             -> *mut ::graphics_transform::GraphicsTransform {
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_value_i_defaultValue(self as *const ::list::ListGraphicsTransformMutPtr, i, default_value as *const *mut ::graphics_transform::GraphicsTransform)
  }
}

impl Drop for ::list::ListGraphicsTransformMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QGraphicsTransform*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_destructor(self as *mut ::list::ListGraphicsTransformMutPtr)
    }
  }
}

/// C++ type: <span style='color: green;'>```QList<QGraphicsView*>```</span>
#[repr(C)]
pub struct ListGraphicsViewMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_GRAPHICS_VIEW_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListGraphicsViewMutPtr {
  unsafe fn new_uninitialized() -> ListGraphicsViewMutPtr {
    ListGraphicsViewMutPtr(::std::mem::uninitialized())
  }
}

impl ListGraphicsViewMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::append(const QList<QGraphicsView*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListGraphicsViewMutPtr) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_append_QList_QGraphicsView_ptr(self as *mut ::list::ListGraphicsViewMutPtr, t as *const ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::append(QGraphicsView* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::graphics_view::GraphicsView) {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_append_QGraphicsView(self as *mut ::list::ListGraphicsViewMutPtr,
                                                                     t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* const & QList<QGraphicsView*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_at(self as *const ::list::ListGraphicsViewMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* const & QList<QGraphicsView*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_back_const(self as *const ::list::ListGraphicsViewMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView*& QList<QGraphicsView*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_back(self as *mut ::list::ListGraphicsViewMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_clear(self as *mut ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* const & QList<QGraphicsView*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_constFirst(self as *const ::list::ListGraphicsViewMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* const & QList<QGraphicsView*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_constLast(self as *const ::list::ListGraphicsViewMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsView*>::contains(QGraphicsView* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::graphics_view::GraphicsView) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_contains(self as *const ::list::ListGraphicsViewMutPtr,
                                                         t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsView*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_count_no_args(self as *const ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsView*>::count(QGraphicsView* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::graphics_view::GraphicsView) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_count_t(self as *const ::list::ListGraphicsViewMutPtr,
                                                        t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsView*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_empty(self as *const ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsView*>::endsWith(QGraphicsView* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::graphics_view::GraphicsView) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_endsWith(self as *const ::list::ListGraphicsViewMutPtr,
                                                         t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* const & QList<QGraphicsView*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_first_const(self as *const ::list::ListGraphicsViewMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView*& QList<QGraphicsView*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_first(self as *mut ::list::ListGraphicsViewMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* const & QList<QGraphicsView*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_front_const(self as *const ::list::ListGraphicsViewMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView*& QList<QGraphicsView*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_front(self as *mut ::list::ListGraphicsViewMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::graphics_view::GraphicsView) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsView*>::indexOf(QGraphicsView* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::graphics_view::GraphicsView, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsView*>::indexOf(QGraphicsView* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGraphicsViewMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::insert(int i, QGraphicsView* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::graphics_view::GraphicsView) {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_insert(self as *mut ::list::ListGraphicsViewMutPtr,
                                                       i,
                                                       t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsView*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_isEmpty(self as *const ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* const & QList<QGraphicsView*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_last_const(self as *const ::list::ListGraphicsViewMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::graphics_view::GraphicsView) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsView*>::lastIndexOf(QGraphicsView* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::graphics_view::GraphicsView, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsView*>::lastIndexOf(QGraphicsView* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGraphicsViewMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsView*& QList<QGraphicsView*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_last(self as *mut ::list::ListGraphicsViewMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsView*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_length(self as *const ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListGraphicsViewMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*> QList<QGraphicsView*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListGraphicsViewMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*> QList<QGraphicsView*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListGraphicsViewMutPtr
    where Args: overloading::ListGraphicsViewMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_move(self as *mut ::list::ListGraphicsViewMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListGraphicsViewMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGraphicsView*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListGraphicsViewMutPtr) -> ::list::ListGraphicsViewMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGraphicsView*>::QList(const QList<QGraphicsView*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListGraphicsViewMutPtr
    where Args: overloading::ListGraphicsViewMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*> QList<QGraphicsView*>::operator+(const QList<QGraphicsView*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListGraphicsViewMutPtr) -> ::list::ListGraphicsViewMutPtr {
    {
      let mut object: ::list::ListGraphicsViewMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_operator_add_to_output(self as *const ::list::ListGraphicsViewMutPtr, l as *const ::list::ListGraphicsViewMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*>& QList<QGraphicsView*>::operator+=(const QList<QGraphicsView*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListGraphicsViewMutPtr)
                                 -> &'l0 mut ::list::ListGraphicsViewMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_operator_add_assign_l(self as *mut ::list::ListGraphicsViewMutPtr,
                                                                          l as *const ::list::ListGraphicsViewMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*>& QList<QGraphicsView*>::operator+=(QGraphicsView* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::graphics_view::GraphicsView)
                                               -> &'l0 mut ::list::ListGraphicsViewMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_operator_add_assign_t(self as *mut ::list::ListGraphicsViewMutPtr, t as *const *mut ::graphics_view::GraphicsView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*>& QList<QGraphicsView*>::operator=(const QList<QGraphicsView*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListGraphicsViewMutPtr)
                             -> &'l0 mut ::list::ListGraphicsViewMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_operator_assign(self as *mut ::list::ListGraphicsViewMutPtr,
                                                                    l as *const ::list::ListGraphicsViewMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsView*>::operator==(const QList<QGraphicsView*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListGraphicsViewMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_operator_eq(self as *const ::list::ListGraphicsViewMutPtr,
                                                              l as *const ::list::ListGraphicsViewMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* const & QList<QGraphicsView*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_operator_index_const(self as *const ::list::ListGraphicsViewMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView*& QList<QGraphicsView*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::graphics_view::GraphicsView {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_operator_index(self as *mut ::list::ListGraphicsViewMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsView*>::operator!=(const QList<QGraphicsView*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListGraphicsViewMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_operator_neq(self as *const ::list::ListGraphicsViewMutPtr,
                                                               l as *const ::list::ListGraphicsViewMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*>& QList<QGraphicsView*>::operator<<(const QList<QGraphicsView*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListGraphicsViewMutPtr)
                          -> &'l0 mut ::list::ListGraphicsViewMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_operator_shl_l(self as *mut ::list::ListGraphicsViewMutPtr,
                                                                   l as *const ::list::ListGraphicsViewMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*>& QList<QGraphicsView*>::operator<<(QGraphicsView* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::graphics_view::GraphicsView)
                                        -> &'l0 mut ::list::ListGraphicsViewMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_operator_shl_t(self as *mut ::list::ListGraphicsViewMutPtr,
                                                                 t as *const *mut ::graphics_view::GraphicsView);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_pop_back(self as *mut ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_pop_front(self as *mut ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::prepend(QGraphicsView* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::graphics_view::GraphicsView) {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_prepend(self as *mut ::list::ListGraphicsViewMutPtr,
                                                        t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::push_back(QGraphicsView* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::graphics_view::GraphicsView) {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_push_back(self as *mut ::list::ListGraphicsViewMutPtr,
                                                          t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::push_front(QGraphicsView* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::graphics_view::GraphicsView) {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_push_front(self as *mut ::list::ListGraphicsViewMutPtr,
                                                           t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsView*>::removeAll(QGraphicsView* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::graphics_view::GraphicsView) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_removeAll(self as *mut ::list::ListGraphicsViewMutPtr,
                                                          t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_removeAt(self as *mut ::list::ListGraphicsViewMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_removeFirst(self as *mut ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_removeLast(self as *mut ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsView*>::removeOne(QGraphicsView* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::graphics_view::GraphicsView) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_removeOne(self as *mut ::list::ListGraphicsViewMutPtr,
                                                          t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::replace(int i, QGraphicsView* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::graphics_view::GraphicsView) {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_replace(self as *mut ::list::ListGraphicsViewMutPtr,
                                                        i,
                                                        t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_reserve(self as *mut ::list::ListGraphicsViewMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsView*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_size(self as *const ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsView*>::startsWith(QGraphicsView* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::graphics_view::GraphicsView) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_startsWith(self as *const ::list::ListGraphicsViewMutPtr,
                                                           t as *const *mut ::graphics_view::GraphicsView)
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsView*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListGraphicsViewMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::swap(QList<QGraphicsView*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsView*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListGraphicsViewMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsView* QList<QGraphicsView*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::graphics_view::GraphicsView {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_takeAt(self as *mut ::list::ListGraphicsViewMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* QList<QGraphicsView*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::graphics_view::GraphicsView {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_takeFirst(self as *mut ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* QList<QGraphicsView*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::graphics_view::GraphicsView {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_takeLast(self as *mut ::list::ListGraphicsViewMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* QList<QGraphicsView*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::graphics_view::GraphicsView {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_value_i(self as *const ::list::ListGraphicsViewMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsView* QList<QGraphicsView*>::value(int i, QGraphicsView* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::graphics_view::GraphicsView)
                             -> *mut ::graphics_view::GraphicsView {
    ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_value_i_defaultValue(self as *const ::list::ListGraphicsViewMutPtr, i, default_value as *const *mut ::graphics_view::GraphicsView)
  }
}

impl Drop for ::list::ListGraphicsViewMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QGraphicsView*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_destructor(self as *mut ::list::ListGraphicsViewMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QGraphicsWidget*>```</span>
#[repr(C)]
pub struct ListGraphicsWidgetMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_GRAPHICS_WIDGET_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListGraphicsWidgetMutPtr {
  unsafe fn new_uninitialized() -> ListGraphicsWidgetMutPtr {
    ListGraphicsWidgetMutPtr(::std::mem::uninitialized())
  }
}

impl ListGraphicsWidgetMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::append(const QList<QGraphicsWidget*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListGraphicsWidgetMutPtr) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_append_QList_QGraphicsWidget_ptr(self as *mut ::list::ListGraphicsWidgetMutPtr, t as *const ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::append(QGraphicsWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::graphics_widget::GraphicsWidget) {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_append_QGraphicsWidget(self as *mut ::list::ListGraphicsWidgetMutPtr, t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* const & QList<QGraphicsWidget*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_at(self as *const ::list::ListGraphicsWidgetMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* const & QList<QGraphicsWidget*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_back_const(self as *const ::list::ListGraphicsWidgetMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget*& QList<QGraphicsWidget*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_back(self as *mut ::list::ListGraphicsWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_clear(self as *mut ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* const & QList<QGraphicsWidget*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_constFirst(self as *const ::list::ListGraphicsWidgetMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* const & QList<QGraphicsWidget*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_constLast(self as *const ::list::ListGraphicsWidgetMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsWidget*>::contains(QGraphicsWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::graphics_widget::GraphicsWidget) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_contains(self as *const ::list::ListGraphicsWidgetMutPtr,
                                                           t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsWidget*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_count_no_args(self as *const ::list::ListGraphicsWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsWidget*>::count(QGraphicsWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::graphics_widget::GraphicsWidget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_count_t(self as *const ::list::ListGraphicsWidgetMutPtr,
                                                          t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsWidget*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_empty(self as *const ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsWidget*>::endsWith(QGraphicsWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::graphics_widget::GraphicsWidget) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_endsWith(self as *const ::list::ListGraphicsWidgetMutPtr,
                                                           t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* const & QList<QGraphicsWidget*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_first_const(self as *const ::list::ListGraphicsWidgetMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget*& QList<QGraphicsWidget*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_first(self as *mut ::list::ListGraphicsWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* const & QList<QGraphicsWidget*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_front_const(self as *const ::list::ListGraphicsWidgetMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget*& QList<QGraphicsWidget*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_front(self as *mut ::list::ListGraphicsWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::graphics_widget::GraphicsWidget) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsWidget*>::indexOf(QGraphicsWidget* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::graphics_widget::GraphicsWidget, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsWidget*>::indexOf(QGraphicsWidget* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGraphicsWidgetMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::insert(int i, QGraphicsWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::graphics_widget::GraphicsWidget) {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_insert(self as *mut ::list::ListGraphicsWidgetMutPtr,
                                                         i,
                                                         t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsWidget*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_isEmpty(self as *const ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* const & QList<QGraphicsWidget*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_last_const(self as *const ::list::ListGraphicsWidgetMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::graphics_widget::GraphicsWidget) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsWidget*>::lastIndexOf(QGraphicsWidget* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::graphics_widget::GraphicsWidget, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGraphicsWidget*>::lastIndexOf(QGraphicsWidget* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGraphicsWidgetMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsWidget*& QList<QGraphicsWidget*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_last(self as *mut ::list::ListGraphicsWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsWidget*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_length(self as *const ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListGraphicsWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*> QList<QGraphicsWidget*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListGraphicsWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*> QList<QGraphicsWidget*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListGraphicsWidgetMutPtr
    where Args: overloading::ListGraphicsWidgetMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_move(self as *mut ::list::ListGraphicsWidgetMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListGraphicsWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGraphicsWidget*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListGraphicsWidgetMutPtr) -> ::list::ListGraphicsWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGraphicsWidget*>::QList(const QList<QGraphicsWidget*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListGraphicsWidgetMutPtr
    where Args: overloading::ListGraphicsWidgetMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*> QList<QGraphicsWidget*>::operator+(const QList<QGraphicsWidget*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListGraphicsWidgetMutPtr) -> ::list::ListGraphicsWidgetMutPtr {
    {
      let mut object: ::list::ListGraphicsWidgetMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_operator_add_to_output(self as *const ::list::ListGraphicsWidgetMutPtr, l as *const ::list::ListGraphicsWidgetMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*>& QList<QGraphicsWidget*>::operator+=(const QList<QGraphicsWidget*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListGraphicsWidgetMutPtr)
                                 -> &'l0 mut ::list::ListGraphicsWidgetMutPtr {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_operator_add_assign_l(self as *mut ::list::ListGraphicsWidgetMutPtr, l as *const ::list::ListGraphicsWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*>& QList<QGraphicsWidget*>::operator+=(QGraphicsWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::graphics_widget::GraphicsWidget)
                                               -> &'l0 mut ::list::ListGraphicsWidgetMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_operator_add_assign_t(self as *mut ::list::ListGraphicsWidgetMutPtr, t as *const *mut ::graphics_widget::GraphicsWidget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*>& QList<QGraphicsWidget*>::operator=(const QList<QGraphicsWidget*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListGraphicsWidgetMutPtr)
                             -> &'l0 mut ::list::ListGraphicsWidgetMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_operator_assign(self as *mut ::list::ListGraphicsWidgetMutPtr,
                                                                      l as *const ::list::ListGraphicsWidgetMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsWidget*>::operator==(const QList<QGraphicsWidget*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListGraphicsWidgetMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_operator_eq(self as *const ::list::ListGraphicsWidgetMutPtr,
                                                                l as *const ::list::ListGraphicsWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* const & QList<QGraphicsWidget*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::graphics_widget::GraphicsWidget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_operator_index_const(self as *const ::list::ListGraphicsWidgetMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget*& QList<QGraphicsWidget*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::graphics_widget::GraphicsWidget {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_operator_index(self as *mut ::list::ListGraphicsWidgetMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsWidget*>::operator!=(const QList<QGraphicsWidget*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListGraphicsWidgetMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_operator_neq(self as *const ::list::ListGraphicsWidgetMutPtr,
                                                                 l as *const ::list::ListGraphicsWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*>& QList<QGraphicsWidget*>::operator<<(const QList<QGraphicsWidget*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListGraphicsWidgetMutPtr)
                          -> &'l0 mut ::list::ListGraphicsWidgetMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_operator_shl_l(self as *mut ::list::ListGraphicsWidgetMutPtr,
                                                                     l as *const ::list::ListGraphicsWidgetMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*>& QList<QGraphicsWidget*>::operator<<(QGraphicsWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::graphics_widget::GraphicsWidget)
                                        -> &'l0 mut ::list::ListGraphicsWidgetMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_operator_shl_t(self as *mut ::list::ListGraphicsWidgetMutPtr, t as *const *mut ::graphics_widget::GraphicsWidget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_pop_back(self as *mut ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_pop_front(self as *mut ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::prepend(QGraphicsWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::graphics_widget::GraphicsWidget) {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_prepend(self as *mut ::list::ListGraphicsWidgetMutPtr,
                                                          t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::push_back(QGraphicsWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::graphics_widget::GraphicsWidget) {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_push_back(self as *mut ::list::ListGraphicsWidgetMutPtr,
                                                            t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::push_front(QGraphicsWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::graphics_widget::GraphicsWidget) {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_push_front(self as *mut ::list::ListGraphicsWidgetMutPtr,
                                                             t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsWidget*>::removeAll(QGraphicsWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::graphics_widget::GraphicsWidget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_removeAll(self as *mut ::list::ListGraphicsWidgetMutPtr,
                                                            t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_removeAt(self as *mut ::list::ListGraphicsWidgetMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_removeFirst(self as *mut ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_removeLast(self as *mut ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsWidget*>::removeOne(QGraphicsWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::graphics_widget::GraphicsWidget) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_removeOne(self as *mut ::list::ListGraphicsWidgetMutPtr,
                                                            t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::replace(int i, QGraphicsWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::graphics_widget::GraphicsWidget) {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_replace(self as *mut ::list::ListGraphicsWidgetMutPtr,
                                                          i,
                                                          t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_reserve(self as *mut ::list::ListGraphicsWidgetMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGraphicsWidget*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_size(self as *const ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGraphicsWidget*>::startsWith(QGraphicsWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::graphics_widget::GraphicsWidget) -> bool {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_startsWith(self as *const ::list::ListGraphicsWidgetMutPtr,
                                                             t as *const *mut ::graphics_widget::GraphicsWidget)
  }

  /// C++ method: <span style='color: green;'>```QList<QGraphicsWidget*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListGraphicsWidgetMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::swap(QList<QGraphicsWidget*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGraphicsWidget*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListGraphicsWidgetMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGraphicsWidget* QList<QGraphicsWidget*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::graphics_widget::GraphicsWidget {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_takeAt(self as *mut ::list::ListGraphicsWidgetMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* QList<QGraphicsWidget*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::graphics_widget::GraphicsWidget {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_takeFirst(self as *mut ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* QList<QGraphicsWidget*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::graphics_widget::GraphicsWidget {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_takeLast(self as *mut ::list::ListGraphicsWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* QList<QGraphicsWidget*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::graphics_widget::GraphicsWidget {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_value_i(self as *const ::list::ListGraphicsWidgetMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QGraphicsWidget* QList<QGraphicsWidget*>::value(int i, QGraphicsWidget* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::graphics_widget::GraphicsWidget)
                             -> *mut ::graphics_widget::GraphicsWidget {
    ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_value_i_defaultValue(self as *const ::list::ListGraphicsWidgetMutPtr, i, default_value as *const *mut ::graphics_widget::GraphicsWidget)
  }
}

impl Drop for ::list::ListGraphicsWidgetMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QGraphicsWidget*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_destructor(self as *mut ::list::ListGraphicsWidgetMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QListWidgetItem*>```</span>
#[repr(C)]
pub struct ListListWidgetItemMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_LIST_WIDGET_ITEM_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListListWidgetItemMutPtr {
  unsafe fn new_uninitialized() -> ListListWidgetItemMutPtr {
    ListListWidgetItemMutPtr(::std::mem::uninitialized())
  }
}

impl ListListWidgetItemMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::append(const QList<QListWidgetItem*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListListWidgetItemMutPtr) {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_append_QList_QListWidgetItem_ptr(self as *mut ::list::ListListWidgetItemMutPtr, t as *const ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::append(QListWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_append_QListWidgetItem(self as *mut ::list::ListListWidgetItemMutPtr, t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* const & QList<QListWidgetItem*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_at(self as *const ::list::ListListWidgetItemMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* const & QList<QListWidgetItem*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_back_const(self as *const ::list::ListListWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem*& QList<QListWidgetItem*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_back(self as *mut ::list::ListListWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_clear(self as *mut ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* const & QList<QListWidgetItem*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_constFirst(self as *const ::list::ListListWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* const & QList<QListWidgetItem*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_constLast(self as *const ::list::ListListWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QListWidgetItem*>::contains(QListWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::list_widget_item::ListWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_contains(self as *const ::list::ListListWidgetItemMutPtr,
                                                           t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```int QList<QListWidgetItem*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_count_no_args(self as *const ::list::ListListWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QListWidgetItem*>::count(QListWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::list_widget_item::ListWidgetItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_count_t(self as *const ::list::ListListWidgetItemMutPtr,
                                                          t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QListWidgetItem*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_empty(self as *const ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QListWidgetItem*>::endsWith(QListWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::list_widget_item::ListWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_endsWith(self as *const ::list::ListListWidgetItemMutPtr,
                                                           t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* const & QList<QListWidgetItem*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_first_const(self as *const ::list::ListListWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem*& QList<QListWidgetItem*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_first(self as *mut ::list::ListListWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* const & QList<QListWidgetItem*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_front_const(self as *const ::list::ListListWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem*& QList<QListWidgetItem*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_front(self as *mut ::list::ListListWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::list_widget_item::ListWidgetItem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QListWidgetItem*>::indexOf(QListWidgetItem* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::list_widget_item::ListWidgetItem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QListWidgetItem*>::indexOf(QListWidgetItem* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListListWidgetItemMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::insert(int i, QListWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_insert(self as *mut ::list::ListListWidgetItemMutPtr,
                                                         i,
                                                         t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QListWidgetItem*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_isEmpty(self as *const ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* const & QList<QListWidgetItem*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_last_const(self as *const ::list::ListListWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::list_widget_item::ListWidgetItem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QListWidgetItem*>::lastIndexOf(QListWidgetItem* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::list_widget_item::ListWidgetItem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QListWidgetItem*>::lastIndexOf(QListWidgetItem* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListListWidgetItemMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QListWidgetItem*& QList<QListWidgetItem*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_last(self as *mut ::list::ListListWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QListWidgetItem*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_length(self as *const ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListListWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*> QList<QListWidgetItem*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListListWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*> QList<QListWidgetItem*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListListWidgetItemMutPtr
    where Args: overloading::ListListWidgetItemMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_move(self as *mut ::list::ListListWidgetItemMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListListWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QListWidgetItem*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListListWidgetItemMutPtr) -> ::list::ListListWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QListWidgetItem*>::QList(const QList<QListWidgetItem*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListListWidgetItemMutPtr
    where Args: overloading::ListListWidgetItemMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*> QList<QListWidgetItem*>::operator+(const QList<QListWidgetItem*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListListWidgetItemMutPtr) -> ::list::ListListWidgetItemMutPtr {
    {
      let mut object: ::list::ListListWidgetItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_operator_add_to_output(self as *const ::list::ListListWidgetItemMutPtr, l as *const ::list::ListListWidgetItemMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*>& QList<QListWidgetItem*>::operator+=(const QList<QListWidgetItem*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListListWidgetItemMutPtr)
                                 -> &'l0 mut ::list::ListListWidgetItemMutPtr {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_operator_add_assign_l(self as *mut ::list::ListListWidgetItemMutPtr, l as *const ::list::ListListWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*>& QList<QListWidgetItem*>::operator+=(QListWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::list_widget_item::ListWidgetItem)
                                               -> &'l0 mut ::list::ListListWidgetItemMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_operator_add_assign_t(self as *mut ::list::ListListWidgetItemMutPtr, t as *const *mut ::list_widget_item::ListWidgetItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*>& QList<QListWidgetItem*>::operator=(const QList<QListWidgetItem*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListListWidgetItemMutPtr)
                             -> &'l0 mut ::list::ListListWidgetItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_operator_assign(self as *mut ::list::ListListWidgetItemMutPtr,
                                                                      l as *const ::list::ListListWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QListWidgetItem*>::operator==(const QList<QListWidgetItem*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListListWidgetItemMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_operator_eq(self as *const ::list::ListListWidgetItemMutPtr,
                                                                l as *const ::list::ListListWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* const & QList<QListWidgetItem*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::list_widget_item::ListWidgetItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_operator_index_const(self as *const ::list::ListListWidgetItemMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem*& QList<QListWidgetItem*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::list_widget_item::ListWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_operator_index(self as *mut ::list::ListListWidgetItemMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QListWidgetItem*>::operator!=(const QList<QListWidgetItem*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListListWidgetItemMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_operator_neq(self as *const ::list::ListListWidgetItemMutPtr,
                                                                 l as *const ::list::ListListWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*>& QList<QListWidgetItem*>::operator<<(const QList<QListWidgetItem*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListListWidgetItemMutPtr)
                          -> &'l0 mut ::list::ListListWidgetItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_operator_shl_l(self as *mut ::list::ListListWidgetItemMutPtr,
                                                                     l as *const ::list::ListListWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*>& QList<QListWidgetItem*>::operator<<(QListWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::list_widget_item::ListWidgetItem)
                                        -> &'l0 mut ::list::ListListWidgetItemMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_operator_shl_t(self as *mut ::list::ListListWidgetItemMutPtr, t as *const *mut ::list_widget_item::ListWidgetItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_pop_back(self as *mut ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_pop_front(self as *mut ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::prepend(QListWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_prepend(self as *mut ::list::ListListWidgetItemMutPtr,
                                                          t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::push_back(QListWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_push_back(self as *mut ::list::ListListWidgetItemMutPtr,
                                                            t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::push_front(QListWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_push_front(self as *mut ::list::ListListWidgetItemMutPtr,
                                                             t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```int QList<QListWidgetItem*>::removeAll(QListWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::list_widget_item::ListWidgetItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_removeAll(self as *mut ::list::ListListWidgetItemMutPtr,
                                                            t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_removeAt(self as *mut ::list::ListListWidgetItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_removeFirst(self as *mut ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_removeLast(self as *mut ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QListWidgetItem*>::removeOne(QListWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::list_widget_item::ListWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_removeOne(self as *mut ::list::ListListWidgetItemMutPtr,
                                                            t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::replace(int i, QListWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::list_widget_item::ListWidgetItem) {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_replace(self as *mut ::list::ListListWidgetItemMutPtr,
                                                          i,
                                                          t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_reserve(self as *mut ::list::ListListWidgetItemMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QListWidgetItem*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_size(self as *const ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QListWidgetItem*>::startsWith(QListWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::list_widget_item::ListWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_startsWith(self as *const ::list::ListListWidgetItemMutPtr,
                                                             t as *const *mut ::list_widget_item::ListWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```QList<QListWidgetItem*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListListWidgetItemMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::swap(QList<QListWidgetItem*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QListWidgetItem*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListListWidgetItemMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QListWidgetItem* QList<QListWidgetItem*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::list_widget_item::ListWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_takeAt(self as *mut ::list::ListListWidgetItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* QList<QListWidgetItem*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::list_widget_item::ListWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_takeFirst(self as *mut ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* QList<QListWidgetItem*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::list_widget_item::ListWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_takeLast(self as *mut ::list::ListListWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* QList<QListWidgetItem*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::list_widget_item::ListWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_value_i(self as *const ::list::ListListWidgetItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QListWidgetItem* QList<QListWidgetItem*>::value(int i, QListWidgetItem* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::list_widget_item::ListWidgetItem)
                             -> *mut ::list_widget_item::ListWidgetItem {
    ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_value_i_defaultValue(self as *const ::list::ListListWidgetItemMutPtr, i, default_value as *const *mut ::list_widget_item::ListWidgetItem)
  }
}

impl Drop for ::list::ListListWidgetItemMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QListWidgetItem*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_destructor(self as *mut ::list::ListListWidgetItemMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QMdiSubWindow*>```</span>
#[repr(C)]
pub struct ListMdiSubWindowMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_MDI_SUB_WINDOW_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListMdiSubWindowMutPtr {
  unsafe fn new_uninitialized() -> ListMdiSubWindowMutPtr {
    ListMdiSubWindowMutPtr(::std::mem::uninitialized())
  }
}

impl ListMdiSubWindowMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::append(const QList<QMdiSubWindow*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListMdiSubWindowMutPtr) {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_append_QList_QMdiSubWindow_ptr(self as *mut ::list::ListMdiSubWindowMutPtr, t as *const ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::append(QMdiSubWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::mdi_sub_window::MdiSubWindow) {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_append_QMdiSubWindow(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                                     t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* const & QList<QMdiSubWindow*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_at(self as *const ::list::ListMdiSubWindowMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* const & QList<QMdiSubWindow*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_back_const(self as *const ::list::ListMdiSubWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow*& QList<QMdiSubWindow*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_back(self as *mut ::list::ListMdiSubWindowMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_clear(self as *mut ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* const & QList<QMdiSubWindow*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_constFirst(self as *const ::list::ListMdiSubWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* const & QList<QMdiSubWindow*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_constLast(self as *const ::list::ListMdiSubWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMdiSubWindow*>::contains(QMdiSubWindow* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::mdi_sub_window::MdiSubWindow) -> bool {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_contains(self as *const ::list::ListMdiSubWindowMutPtr,
                                                         t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```int QList<QMdiSubWindow*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_count_no_args(self as *const ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QMdiSubWindow*>::count(QMdiSubWindow* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::mdi_sub_window::MdiSubWindow) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_count_t(self as *const ::list::ListMdiSubWindowMutPtr,
                                                        t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMdiSubWindow*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_empty(self as *const ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMdiSubWindow*>::endsWith(QMdiSubWindow* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::mdi_sub_window::MdiSubWindow) -> bool {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_endsWith(self as *const ::list::ListMdiSubWindowMutPtr,
                                                         t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* const & QList<QMdiSubWindow*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_first_const(self as *const ::list::ListMdiSubWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow*& QList<QMdiSubWindow*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_first(self as *mut ::list::ListMdiSubWindowMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* const & QList<QMdiSubWindow*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_front_const(self as *const ::list::ListMdiSubWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow*& QList<QMdiSubWindow*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_front(self as *mut ::list::ListMdiSubWindowMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::mdi_sub_window::MdiSubWindow) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QMdiSubWindow*>::indexOf(QMdiSubWindow* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::mdi_sub_window::MdiSubWindow, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QMdiSubWindow*>::indexOf(QMdiSubWindow* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListMdiSubWindowMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::insert(int i, QMdiSubWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::mdi_sub_window::MdiSubWindow) {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_insert(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                       i,
                                                       t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMdiSubWindow*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_isEmpty(self as *const ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* const & QList<QMdiSubWindow*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_last_const(self as *const ::list::ListMdiSubWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::mdi_sub_window::MdiSubWindow) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QMdiSubWindow*>::lastIndexOf(QMdiSubWindow* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::mdi_sub_window::MdiSubWindow, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QMdiSubWindow*>::lastIndexOf(QMdiSubWindow* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListMdiSubWindowMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMdiSubWindow*& QList<QMdiSubWindow*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_last(self as *mut ::list::ListMdiSubWindowMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QMdiSubWindow*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_length(self as *const ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListMdiSubWindowMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*> QList<QMdiSubWindow*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListMdiSubWindowMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*> QList<QMdiSubWindow*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListMdiSubWindowMutPtr
    where Args: overloading::ListMdiSubWindowMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_move(self as *mut ::list::ListMdiSubWindowMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListMdiSubWindowMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QMdiSubWindow*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListMdiSubWindowMutPtr) -> ::list::ListMdiSubWindowMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QMdiSubWindow*>::QList(const QList<QMdiSubWindow*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListMdiSubWindowMutPtr
    where Args: overloading::ListMdiSubWindowMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*> QList<QMdiSubWindow*>::operator+(const QList<QMdiSubWindow*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListMdiSubWindowMutPtr) -> ::list::ListMdiSubWindowMutPtr {
    {
      let mut object: ::list::ListMdiSubWindowMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_operator_add_to_output(self as *const ::list::ListMdiSubWindowMutPtr, l as *const ::list::ListMdiSubWindowMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*>& QList<QMdiSubWindow*>::operator+=(const QList<QMdiSubWindow*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListMdiSubWindowMutPtr)
                                 -> &'l0 mut ::list::ListMdiSubWindowMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_operator_add_assign_l(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                                          l as *const ::list::ListMdiSubWindowMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*>& QList<QMdiSubWindow*>::operator+=(QMdiSubWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::mdi_sub_window::MdiSubWindow)
                                               -> &'l0 mut ::list::ListMdiSubWindowMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_operator_add_assign_t(self as *mut ::list::ListMdiSubWindowMutPtr, t as *const *mut ::mdi_sub_window::MdiSubWindow);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*>& QList<QMdiSubWindow*>::operator=(const QList<QMdiSubWindow*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListMdiSubWindowMutPtr)
                             -> &'l0 mut ::list::ListMdiSubWindowMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_operator_assign(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                                    l as *const ::list::ListMdiSubWindowMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMdiSubWindow*>::operator==(const QList<QMdiSubWindow*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListMdiSubWindowMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_operator_eq(self as *const ::list::ListMdiSubWindowMutPtr,
                                                              l as *const ::list::ListMdiSubWindowMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* const & QList<QMdiSubWindow*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_operator_index_const(self as *const ::list::ListMdiSubWindowMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow*& QList<QMdiSubWindow*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::mdi_sub_window::MdiSubWindow {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_operator_index(self as *mut ::list::ListMdiSubWindowMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMdiSubWindow*>::operator!=(const QList<QMdiSubWindow*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListMdiSubWindowMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_operator_neq(self as *const ::list::ListMdiSubWindowMutPtr,
                                                               l as *const ::list::ListMdiSubWindowMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*>& QList<QMdiSubWindow*>::operator<<(const QList<QMdiSubWindow*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListMdiSubWindowMutPtr)
                          -> &'l0 mut ::list::ListMdiSubWindowMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_operator_shl_l(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                                   l as *const ::list::ListMdiSubWindowMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*>& QList<QMdiSubWindow*>::operator<<(QMdiSubWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::mdi_sub_window::MdiSubWindow)
                                        -> &'l0 mut ::list::ListMdiSubWindowMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_operator_shl_t(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                                 t as *const *mut ::mdi_sub_window::MdiSubWindow);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_pop_back(self as *mut ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_pop_front(self as *mut ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::prepend(QMdiSubWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::mdi_sub_window::MdiSubWindow) {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_prepend(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                        t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::push_back(QMdiSubWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::mdi_sub_window::MdiSubWindow) {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_push_back(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                          t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::push_front(QMdiSubWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::mdi_sub_window::MdiSubWindow) {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_push_front(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                           t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```int QList<QMdiSubWindow*>::removeAll(QMdiSubWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::mdi_sub_window::MdiSubWindow) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_removeAll(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                          t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_removeAt(self as *mut ::list::ListMdiSubWindowMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_removeFirst(self as *mut ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_removeLast(self as *mut ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMdiSubWindow*>::removeOne(QMdiSubWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::mdi_sub_window::MdiSubWindow) -> bool {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_removeOne(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                          t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::replace(int i, QMdiSubWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::mdi_sub_window::MdiSubWindow) {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_replace(self as *mut ::list::ListMdiSubWindowMutPtr,
                                                        i,
                                                        t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_reserve(self as *mut ::list::ListMdiSubWindowMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QMdiSubWindow*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_size(self as *const ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QMdiSubWindow*>::startsWith(QMdiSubWindow* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::mdi_sub_window::MdiSubWindow) -> bool {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_startsWith(self as *const ::list::ListMdiSubWindowMutPtr,
                                                           t as *const *mut ::mdi_sub_window::MdiSubWindow)
  }

  /// C++ method: <span style='color: green;'>```QList<QMdiSubWindow*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListMdiSubWindowMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::swap(QList<QMdiSubWindow*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QMdiSubWindow*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListMdiSubWindowMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QMdiSubWindow* QList<QMdiSubWindow*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::mdi_sub_window::MdiSubWindow {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_takeAt(self as *mut ::list::ListMdiSubWindowMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* QList<QMdiSubWindow*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::mdi_sub_window::MdiSubWindow {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_takeFirst(self as *mut ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* QList<QMdiSubWindow*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::mdi_sub_window::MdiSubWindow {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_takeLast(self as *mut ::list::ListMdiSubWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* QList<QMdiSubWindow*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::mdi_sub_window::MdiSubWindow {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_value_i(self as *const ::list::ListMdiSubWindowMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QMdiSubWindow* QList<QMdiSubWindow*>::value(int i, QMdiSubWindow* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::mdi_sub_window::MdiSubWindow)
                             -> *mut ::mdi_sub_window::MdiSubWindow {
    ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_value_i_defaultValue(self as *const ::list::ListMdiSubWindowMutPtr, i, default_value as *const *mut ::mdi_sub_window::MdiSubWindow)
  }
}

impl Drop for ::list::ListMdiSubWindowMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QMdiSubWindow*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_destructor(self as *mut ::list::ListMdiSubWindowMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QPair<double, double>>```</span>
#[repr(C)]
pub struct ListPairPairCDoubleCDouble([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_PAIR_PAIR_C_DOUBLE_C_DOUBLE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListPairPairCDoubleCDouble {
  unsafe fn new_uninitialized() -> ListPairPairCDoubleCDouble {
    ListPairPairCDoubleCDouble(::std::mem::uninitialized())
  }
}

impl ListPairPairCDoubleCDouble {
  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListPairPairCDoubleCDouble) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::append(const QList<QPair<double, double>>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::pair::PairCDoubleCDouble) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::append(const QPair<double, double>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListPairPairCDoubleCDoubleAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPair<double, double>& QList<QPair<double, double>>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_at(self as *const ::list::ListPairPairCDoubleCDouble, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, double>& QList<QPair<double, double>>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_back_const(self as *const ::list::ListPairPairCDoubleCDouble)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, double>& QList<QPair<double, double>>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_back(self as *mut ::list::ListPairPairCDoubleCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_clear(self as *mut ::list::ListPairPairCDoubleCDouble) }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, double>& QList<QPair<double, double>>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_constFirst(self as *const ::list::ListPairPairCDoubleCDouble)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, double>& QList<QPair<double, double>>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_constLast(self as *const ::list::ListPairPairCDoubleCDouble)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, double>>::contains(const QPair<double, double>& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::pair::PairCDoubleCDouble) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_contains(self as *const ::list::ListPairPairCDoubleCDouble,
                                                             t as *const ::pair::PairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, double>>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::pair::PairCDoubleCDouble) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, double>>::count(const QPair<double, double>& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPairPairCDoubleCDoubleCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, double>>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_empty(self as *const ::list::ListPairPairCDoubleCDouble) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, double>>::endsWith(const QPair<double, double>& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::pair::PairCDoubleCDouble) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_endsWith(self as *const ::list::ListPairPairCDoubleCDouble,
                                                             t as *const ::pair::PairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, double>& QList<QPair<double, double>>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_first_const(self as *const ::list::ListPairPairCDoubleCDouble)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, double>& QList<QPair<double, double>>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_first(self as *mut ::list::ListPairPairCDoubleCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, double>& QList<QPair<double, double>>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_front_const(self as *const ::list::ListPairPairCDoubleCDouble)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, double>& QList<QPair<double, double>>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_front(self as *mut ::list::ListPairPairCDoubleCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::pair::PairCDoubleCDouble) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, double>>::indexOf(const QPair<double, double>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::pair::PairCDoubleCDouble, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, double>>::indexOf(const QPair<double, double>& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPairPairCDoubleCDoubleIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::insert(int i, const QPair<double, double>& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::pair::PairCDoubleCDouble) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_insert(self as *mut ::list::ListPairPairCDoubleCDouble,
                                                           i,
                                                           t as *const ::pair::PairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, double>>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_isEmpty(self as *const ::list::ListPairPairCDoubleCDouble) }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, double>& QList<QPair<double, double>>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_last_const(self as *const ::list::ListPairPairCDoubleCDouble)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::pair::PairCDoubleCDouble) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, double>>::lastIndexOf(const QPair<double, double>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::pair::PairCDoubleCDouble, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, double>>::lastIndexOf(const QPair<double, double>& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPairPairCDoubleCDoubleLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPair<double, double>& QList<QPair<double, double>>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_last(self as *mut ::list::ListPairPairCDoubleCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QPair<double, double>>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_length(self as *const ::list::ListPairPairCDoubleCDouble) }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListPairPairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>> QList<QPair<double, double>>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListPairPairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>> QList<QPair<double, double>>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListPairPairCDoubleCDouble
    where Args: overloading::ListPairPairCDoubleCDoubleMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_move(self as *mut ::list::ListPairPairCDoubleCDouble, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListPairPairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QPair<double, double>>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListPairPairCDoubleCDouble) -> ::list::ListPairPairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QPair<double, double>>::QList(const QList<QPair<double, double>>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListPairPairCDoubleCDouble
    where Args: overloading::ListPairPairCDoubleCDoubleNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>> QList<QPair<double, double>>::operator+(const QList<QPair<double, double>>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListPairPairCDoubleCDouble) -> ::list::ListPairPairCDoubleCDouble {
    {
      let mut object: ::list::ListPairPairCDoubleCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_operator_add_to_output(self as *const ::list::ListPairPairCDoubleCDouble, l as *const ::list::ListPairPairCDoubleCDouble, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListPairPairCDoubleCDouble) -> &'l0 mut ::list::ListPairPairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>& QList<QPair<double, double>>::operator+=(const QList<QPair<double, double>>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::pair::PairCDoubleCDouble) -> &'l0 mut ::list::ListPairPairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>& QList<QPair<double, double>>::operator+=(const QPair<double, double>& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListPairPairCDoubleCDouble
    where Args: overloading::ListPairPairCDoubleCDoubleOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>& QList<QPair<double, double>>::operator=(const QList<QPair<double, double>>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListPairPairCDoubleCDouble)
                             -> &'l0 mut ::list::ListPairPairCDoubleCDouble {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_operator_assign(self as *mut ::list::ListPairPairCDoubleCDouble,
                                                                      l as *const ::list::ListPairPairCDoubleCDouble)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, double>>::operator==(const QList<QPair<double, double>>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListPairPairCDoubleCDouble) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_operator_eq(self as *const ::list::ListPairPairCDoubleCDouble,
                                                                l as *const ::list::ListPairPairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, double>& QList<QPair<double, double>>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::pair::PairCDoubleCDouble {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_operator_index_const(self as *const ::list::ListPairPairCDoubleCDouble, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, double>& QList<QPair<double, double>>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::pair::PairCDoubleCDouble {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_operator_index(self as *mut ::list::ListPairPairCDoubleCDouble, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, double>>::operator!=(const QList<QPair<double, double>>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListPairPairCDoubleCDouble) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_operator_neq(self as *const ::list::ListPairPairCDoubleCDouble,
                                                                 l as *const ::list::ListPairPairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListPairPairCDoubleCDouble) -> &'l0 mut ::list::ListPairPairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>& QList<QPair<double, double>>::operator<<(const QList<QPair<double, double>>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::pair::PairCDoubleCDouble) -> &'l0 mut ::list::ListPairPairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>& QList<QPair<double, double>>::operator<<(const QPair<double, double>& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListPairPairCDoubleCDouble
    where Args: overloading::ListPairPairCDoubleCDoubleOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_pop_back(self as *mut ::list::ListPairPairCDoubleCDouble) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_pop_front(self as *mut ::list::ListPairPairCDoubleCDouble) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::prepend(const QPair<double, double>& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::pair::PairCDoubleCDouble) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_prepend(self as *mut ::list::ListPairPairCDoubleCDouble,
                                                            t as *const ::pair::PairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::push_back(const QPair<double, double>& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::pair::PairCDoubleCDouble) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_push_back(self as *mut ::list::ListPairPairCDoubleCDouble,
                                                              t as *const ::pair::PairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::push_front(const QPair<double, double>& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::pair::PairCDoubleCDouble) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_push_front(self as *mut ::list::ListPairPairCDoubleCDouble,
                                                               t as *const ::pair::PairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QPair<double, double>>::removeAll(const QPair<double, double>& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::pair::PairCDoubleCDouble) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_removeAll(self as *mut ::list::ListPairPairCDoubleCDouble,
                                                              t as *const ::pair::PairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_removeAt(self as *mut ::list::ListPairPairCDoubleCDouble, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_removeFirst(self as *mut ::list::ListPairPairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_removeLast(self as *mut ::list::ListPairPairCDoubleCDouble) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, double>>::removeOne(const QPair<double, double>& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::pair::PairCDoubleCDouble) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_removeOne(self as *mut ::list::ListPairPairCDoubleCDouble,
                                                              t as *const ::pair::PairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::replace(int i, const QPair<double, double>& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::pair::PairCDoubleCDouble) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_replace(self as *mut ::list::ListPairPairCDoubleCDouble,
                                                            i,
                                                            t as *const ::pair::PairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_reserve(self as *mut ::list::ListPairPairCDoubleCDouble, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QPair<double, double>>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_size(self as *const ::list::ListPairPairCDoubleCDouble) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, double>>::startsWith(const QPair<double, double>& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::pair::PairCDoubleCDouble) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_double_startsWith(self as *const ::list::ListPairPairCDoubleCDouble,
                                                               t as *const ::pair::PairCDoubleCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListPairPairCDoubleCDouble) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::swap(QList<QPair<double, double>>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, double>>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListPairPairCDoubleCDoubleSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPair<double, double> QList<QPair<double, double>>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::pair::PairCDoubleCDouble {
    {
      let mut object: ::pair::PairCDoubleCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_takeAt_to_output(self as *mut ::list::ListPairPairCDoubleCDouble, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, double> QList<QPair<double, double>>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::pair::PairCDoubleCDouble {
    {
      let mut object: ::pair::PairCDoubleCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_takeFirst_to_output(self as *mut ::list::ListPairPairCDoubleCDouble, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, double> QList<QPair<double, double>>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::pair::PairCDoubleCDouble {
    {
      let mut object: ::pair::PairCDoubleCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_takeLast_to_output(self as *mut ::list::ListPairPairCDoubleCDouble, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, double>>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::pair::PairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```QPair<double, double> QList<QPair<double, double>>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::pair::PairCDoubleCDouble)) -> ::pair::PairCDoubleCDouble```<br>
  /// C++ method: <span style='color: green;'>```QPair<double, double> QList<QPair<double, double>>::value(int i, const QPair<double, double>& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::pair::PairCDoubleCDouble
    where Args: overloading::ListPairPairCDoubleCDoubleValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListPairPairCDoubleCDouble {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QPair<double, double>>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_destructor(self as *mut ::list::ListPairPairCDoubleCDouble) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QPair<double, QPointF>>```</span>
#[repr(C)]
pub struct ListPairPairCDoubleQtCorePointF([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_PAIR_PAIR_C_DOUBLE_QT_CORE_POINT_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListPairPairCDoubleQtCorePointF {
  unsafe fn new_uninitialized() -> ListPairPairCDoubleQtCorePointF {
    ListPairPairCDoubleQtCorePointF(::std::mem::uninitialized())
  }
}

impl ListPairPairCDoubleQtCorePointF {
  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListPairPairCDoubleQtCorePointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::append(const QList<QPair<double, QPointF>>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::pair::PairCDoubleQtCorePointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::append(const QPair<double, QPointF>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListPairPairCDoubleQtCorePointFAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPair<double, QPointF>& QList<QPair<double, QPointF>>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::pair::PairCDoubleQtCorePointF {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_QPointF_at(self as *const ::list::ListPairPairCDoubleQtCorePointF, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QPointF>& QList<QPair<double, QPointF>>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleQtCorePointF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_back_const(self as *const ::list::ListPairPairCDoubleQtCorePointF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QPointF>& QList<QPair<double, QPointF>>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleQtCorePointF {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_QPointF_back(self as *mut ::list::ListPairPairCDoubleQtCorePointF)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_clear(self as *mut ::list::ListPairPairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QPointF>& QList<QPair<double, QPointF>>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleQtCorePointF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_constFirst(self as *const ::list::ListPairPairCDoubleQtCorePointF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QPointF>& QList<QPair<double, QPointF>>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleQtCorePointF {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_QPointF_constLast(self as *const ::list::ListPairPairCDoubleQtCorePointF)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, QPointF>>::contains(const QPair<double, QPointF>& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::pair::PairCDoubleQtCorePointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_contains(self as *const ::list::ListPairPairCDoubleQtCorePointF,
                                                              t as *const ::pair::PairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, QPointF>>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::pair::PairCDoubleQtCorePointF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, QPointF>>::count(const QPair<double, QPointF>& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPairPairCDoubleQtCorePointFCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, QPointF>>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_empty(self as *const ::list::ListPairPairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, QPointF>>::endsWith(const QPair<double, QPointF>& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::pair::PairCDoubleQtCorePointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_endsWith(self as *const ::list::ListPairPairCDoubleQtCorePointF,
                                                              t as *const ::pair::PairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QPointF>& QList<QPair<double, QPointF>>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleQtCorePointF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_first_const(self as *const ::list::ListPairPairCDoubleQtCorePointF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QPointF>& QList<QPair<double, QPointF>>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleQtCorePointF {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_QPointF_first(self as *mut ::list::ListPairPairCDoubleQtCorePointF)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QPointF>& QList<QPair<double, QPointF>>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleQtCorePointF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_front_const(self as *const ::list::ListPairPairCDoubleQtCorePointF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QPointF>& QList<QPair<double, QPointF>>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleQtCorePointF {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_QPointF_front(self as *mut ::list::ListPairPairCDoubleQtCorePointF)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::pair::PairCDoubleQtCorePointF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, QPointF>>::indexOf(const QPair<double, QPointF>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::pair::PairCDoubleQtCorePointF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, QPointF>>::indexOf(const QPair<double, QPointF>& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPairPairCDoubleQtCorePointFIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::insert(int i, const QPair<double, QPointF>& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::pair::PairCDoubleQtCorePointF) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_insert(self as *mut ::list::ListPairPairCDoubleQtCorePointF,
                                                            i,
                                                            t as *const ::pair::PairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, QPointF>>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_isEmpty(self as *const ::list::ListPairPairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QPointF>& QList<QPair<double, QPointF>>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::pair::PairCDoubleQtCorePointF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_last_const(self as *const ::list::ListPairPairCDoubleQtCorePointF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::pair::PairCDoubleQtCorePointF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, QPointF>>::lastIndexOf(const QPair<double, QPointF>& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::pair::PairCDoubleQtCorePointF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPair<double, QPointF>>::lastIndexOf(const QPair<double, QPointF>& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPairPairCDoubleQtCorePointFLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPair<double, QPointF>& QList<QPair<double, QPointF>>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::pair::PairCDoubleQtCorePointF {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_QPointF_last(self as *mut ::list::ListPairPairCDoubleQtCorePointF)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QPair<double, QPointF>>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_length(self as *const ::list::ListPairPairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListPairPairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>> QList<QPair<double, QPointF>>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListPairPairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>> QList<QPair<double, QPointF>>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListPairPairCDoubleQtCorePointF
    where Args: overloading::ListPairPairCDoubleQtCorePointFMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_move(self as *mut ::list::ListPairPairCDoubleQtCorePointF,
                                                          from,
                                                          to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListPairPairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QPair<double, QPointF>>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListPairPairCDoubleQtCorePointF) -> ::list::ListPairPairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QPair<double, QPointF>>::QList(const QList<QPair<double, QPointF>>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListPairPairCDoubleQtCorePointF
    where Args: overloading::ListPairPairCDoubleQtCorePointFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>> QList<QPair<double, QPointF>>::operator+(const QList<QPair<double, QPointF>>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListPairPairCDoubleQtCorePointF) -> ::list::ListPairPairCDoubleQtCorePointF {
    {
      let mut object: ::list::ListPairPairCDoubleQtCorePointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_QPointF_operator_add_to_output(self as *const ::list::ListPairPairCDoubleQtCorePointF, l as *const ::list::ListPairPairCDoubleQtCorePointF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListPairPairCDoubleQtCorePointF) -> &'l0 mut ::list::ListPairPairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>& QList<QPair<double, QPointF>>::operator+=(const QList<QPair<double, QPointF>>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::pair::PairCDoubleQtCorePointF) -> &'l0 mut ::list::ListPairPairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>& QList<QPair<double, QPointF>>::operator+=(const QPair<double, QPointF>& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self,
                                     args: Args)
                                     -> &'largs mut ::list::ListPairPairCDoubleQtCorePointF
    where Args: overloading::ListPairPairCDoubleQtCorePointFOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>& QList<QPair<double, QPointF>>::operator=(const QList<QPair<double, QPointF>>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListPairPairCDoubleQtCorePointF)
                             -> &'l0 mut ::list::ListPairPairCDoubleQtCorePointF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_operator_assign(self as *mut ::list::ListPairPairCDoubleQtCorePointF, l as *const ::list::ListPairPairCDoubleQtCorePointF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, QPointF>>::operator==(const QList<QPair<double, QPointF>>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListPairPairCDoubleQtCorePointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_operator_eq(self as *const ::list::ListPairPairCDoubleQtCorePointF, l as *const ::list::ListPairPairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPair<double, QPointF>& QList<QPair<double, QPointF>>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::pair::PairCDoubleQtCorePointF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_operator_index_const(self as *const ::list::ListPairPairCDoubleQtCorePointF, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QPointF>& QList<QPair<double, QPointF>>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::pair::PairCDoubleQtCorePointF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_operator_index(self as *mut ::list::ListPairPairCDoubleQtCorePointF, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, QPointF>>::operator!=(const QList<QPair<double, QPointF>>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListPairPairCDoubleQtCorePointF) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_operator_neq(self as *const ::list::ListPairPairCDoubleQtCorePointF, l as *const ::list::ListPairPairCDoubleQtCorePointF) }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListPairPairCDoubleQtCorePointF) -> &'l0 mut ::list::ListPairPairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>& QList<QPair<double, QPointF>>::operator<<(const QList<QPair<double, QPointF>>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::pair::PairCDoubleQtCorePointF) -> &'l0 mut ::list::ListPairPairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>& QList<QPair<double, QPointF>>::operator<<(const QPair<double, QPointF>& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListPairPairCDoubleQtCorePointF
    where Args: overloading::ListPairPairCDoubleQtCorePointFOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_pop_back(self as *mut ::list::ListPairPairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_pop_front(self as *mut ::list::ListPairPairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::prepend(const QPair<double, QPointF>& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::pair::PairCDoubleQtCorePointF) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_prepend(self as *mut ::list::ListPairPairCDoubleQtCorePointF,
                                                             t as *const ::pair::PairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::push_back(const QPair<double, QPointF>& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::pair::PairCDoubleQtCorePointF) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_push_back(self as *mut ::list::ListPairPairCDoubleQtCorePointF,
                                                               t as *const ::pair::PairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::push_front(const QPair<double, QPointF>& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::pair::PairCDoubleQtCorePointF) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_push_front(self as *mut ::list::ListPairPairCDoubleQtCorePointF,
                                                                t as *const ::pair::PairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QPair<double, QPointF>>::removeAll(const QPair<double, QPointF>& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::pair::PairCDoubleQtCorePointF) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_removeAll(self as *mut ::list::ListPairPairCDoubleQtCorePointF,
                                                               t as *const ::pair::PairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_removeAt(self as *mut ::list::ListPairPairCDoubleQtCorePointF, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_removeFirst(self as *mut ::list::ListPairPairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_removeLast(self as *mut ::list::ListPairPairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, QPointF>>::removeOne(const QPair<double, QPointF>& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::pair::PairCDoubleQtCorePointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_removeOne(self as *mut ::list::ListPairPairCDoubleQtCorePointF,
                                                               t as *const ::pair::PairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::replace(int i, const QPair<double, QPointF>& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::pair::PairCDoubleQtCorePointF) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_replace(self as *mut ::list::ListPairPairCDoubleQtCorePointF,
                                                             i,
                                                             t as *const ::pair::PairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_reserve(self as *mut ::list::ListPairPairCDoubleQtCorePointF, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QPair<double, QPointF>>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_size(self as *const ::list::ListPairPairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPair<double, QPointF>>::startsWith(const QPair<double, QPointF>& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::pair::PairCDoubleQtCorePointF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_startsWith(self as *const ::list::ListPairPairCDoubleQtCorePointF, t as *const ::pair::PairCDoubleQtCorePointF)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListPairPairCDoubleQtCorePointF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::swap(QList<QPair<double, QPointF>>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPair<double, QPointF>>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListPairPairCDoubleQtCorePointFSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPair<double, QPointF> QList<QPair<double, QPointF>>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::pair::PairCDoubleQtCorePointF {
    {
      let mut object: ::pair::PairCDoubleQtCorePointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_QPointF_takeAt_to_output(self as *mut ::list::ListPairPairCDoubleQtCorePointF, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QPointF> QList<QPair<double, QPointF>>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::pair::PairCDoubleQtCorePointF {
    {
      let mut object: ::pair::PairCDoubleQtCorePointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_QPointF_takeFirst_to_output(self as *mut ::list::ListPairPairCDoubleQtCorePointF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPair<double, QPointF> QList<QPair<double, QPointF>>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::pair::PairCDoubleQtCorePointF {
    {
      let mut object: ::pair::PairCDoubleQtCorePointF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_QPointF_takeLast_to_output(self as *mut ::list::ListPairPairCDoubleQtCorePointF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPair<double, QPointF>>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::pair::PairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```QPair<double, QPointF> QList<QPair<double, QPointF>>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::pair::PairCDoubleQtCorePointF)) -> ::pair::PairCDoubleQtCorePointF```<br>
  /// C++ method: <span style='color: green;'>```QPair<double, QPointF> QList<QPair<double, QPointF>>::value(int i, const QPair<double, QPointF>& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::pair::PairCDoubleQtCorePointF
    where Args: overloading::ListPairPairCDoubleQtCorePointFValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListPairPairCDoubleQtCorePointF {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QPair<double, QPointF>>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QPair_double_QPointF_destructor(self as *mut ::list::ListPairPairCDoubleQtCorePointF)
    }
  }
}

/// C++ type: <span style='color: green;'>```QList<QRectF>```</span>
#[repr(C)]
pub struct ListQtCoreRectF([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_QT_CORE_RECT_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListQtCoreRectF {
  unsafe fn new_uninitialized() -> ListQtCoreRectF {
    ListQtCoreRectF(::std::mem::uninitialized())
  }
}

impl ListQtCoreRectF {
  /// C++ method: <span style='color: green;'>```QList<QRectF>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListQtCoreRectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QRectF>::append(const QList<QRectF>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::qt_core::rect_f::RectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QRectF>::append(const QRectF& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListQtCoreRectFAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QRectF& QList<QRectF>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QRectF_at(self as *const ::list::ListQtCoreRectF, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QList<QRectF>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QRectF_back_const(self as *const ::list::ListQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF& QList<QRectF>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QRectF_back(self as *mut ::list::ListQtCoreRectF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QRectF>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_clear(self as *mut ::list::ListQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QList<QRectF>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QRectF_constFirst(self as *const ::list::ListQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QList<QRectF>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QRectF_constLast(self as *const ::list::ListQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QRectF>::contains(const QRectF& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::qt_core::rect_f::RectF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_contains(self as *const ::list::ListQtCoreRectF,
                                                t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QRectF>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QRectF>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_core::rect_f::RectF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QRectF>::count(const QRectF& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListQtCoreRectFCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QRectF>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_empty(self as *const ::list::ListQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QRectF>::endsWith(const QRectF& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::qt_core::rect_f::RectF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_endsWith(self as *const ::list::ListQtCoreRectF,
                                                t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QList<QRectF>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QRectF_first_const(self as *const ::list::ListQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF& QList<QRectF>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QRectF_first(self as *mut ::list::ListQtCoreRectF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QList<QRectF> QList<QRectF>::fromVector(const QVector<QRectF>& vector)```</span>
  ///
  ///
  pub fn from_vector(vector: &::qt_gui::vector::VectorQtCoreRectF) -> ::list::ListQtCoreRectF {
    {
      let mut object: ::list::ListQtCoreRectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_fromVector_to_output(vector as *const ::qt_gui::vector::VectorQtCoreRectF,
                                                              &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QList<QRectF>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QRectF_front_const(self as *const ::list::ListQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF& QList<QRectF>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QRectF_front(self as *mut ::list::ListQtCoreRectF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QRectF>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::qt_core::rect_f::RectF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QRectF>::indexOf(const QRectF& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::qt_core::rect_f::RectF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QRectF>::indexOf(const QRectF& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListQtCoreRectFIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QRectF>::insert(int i, const QRectF& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_insert(self as *mut ::list::ListQtCoreRectF,
                                              i,
                                              t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QRectF>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_isEmpty(self as *const ::list::ListQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QList<QRectF>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QRectF_last_const(self as *const ::list::ListQtCoreRectF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QRectF>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::qt_core::rect_f::RectF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QRectF>::lastIndexOf(const QRectF& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::qt_core::rect_f::RectF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QRectF>::lastIndexOf(const QRectF& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListQtCoreRectFLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF& QList<QRectF>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QRectF_last(self as *mut ::list::ListQtCoreRectF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QRectF>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_length(self as *const ::list::ListQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```QList<QRectF>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QList<QRectF> QList<QRectF>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QList<QRectF> QList<QRectF>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListQtCoreRectF
    where Args: overloading::ListQtCoreRectFMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QRectF>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_move(self as *mut ::list::ListQtCoreRectF, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QRectF>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QRectF>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListQtCoreRectF) -> ::list::ListQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QRectF>::QList(const QList<QRectF>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListQtCoreRectF
    where Args: overloading::ListQtCoreRectFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QRectF> QList<QRectF>::operator+(const QList<QRectF>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListQtCoreRectF) -> ::list::ListQtCoreRectF {
    {
      let mut object: ::list::ListQtCoreRectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_operator_add_to_output(self as *const ::list::ListQtCoreRectF,
                                                                l as *const ::list::ListQtCoreRectF,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QRectF>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListQtCoreRectF) -> &'l0 mut ::list::ListQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QList<QRectF>& QList<QRectF>::operator+=(const QList<QRectF>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt_core::rect_f::RectF) -> &'l0 mut ::list::ListQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QList<QRectF>& QList<QRectF>::operator+=(const QRectF& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListQtCoreRectF
    where Args: overloading::ListQtCoreRectFOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QRectF>& QList<QRectF>::operator=(const QList<QRectF>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListQtCoreRectF) -> &'l0 mut ::list::ListQtCoreRectF {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_operator_assign(self as *mut ::list::ListQtCoreRectF,
                                                       l as *const ::list::ListQtCoreRectF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QRectF>::operator==(const QList<QRectF>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListQtCoreRectF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_operator_eq(self as *const ::list::ListQtCoreRectF,
                                                   l as *const ::list::ListQtCoreRectF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QRectF& QList<QRectF>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::rect_f::RectF {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QRectF_operator_index_const(self as *const ::list::ListQtCoreRectF, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QRectF& QList<QRectF>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::qt_core::rect_f::RectF {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QRectF_operator_index(self as *mut ::list::ListQtCoreRectF, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QRectF>::operator!=(const QList<QRectF>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListQtCoreRectF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_operator_neq(self as *const ::list::ListQtCoreRectF,
                                                    l as *const ::list::ListQtCoreRectF)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QRectF>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListQtCoreRectF) -> &'l0 mut ::list::ListQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QList<QRectF>& QList<QRectF>::operator<<(const QList<QRectF>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::qt_core::rect_f::RectF) -> &'l0 mut ::list::ListQtCoreRectF```<br>
  /// C++ method: <span style='color: green;'>```QList<QRectF>& QList<QRectF>::operator<<(const QRectF& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListQtCoreRectF
    where Args: overloading::ListQtCoreRectFOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QRectF>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_pop_back(self as *mut ::list::ListQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QRectF>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_pop_front(self as *mut ::list::ListQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QRectF>::prepend(const QRectF& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_prepend(self as *mut ::list::ListQtCoreRectF,
                                               t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QRectF>::push_back(const QRectF& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_push_back(self as *mut ::list::ListQtCoreRectF,
                                                 t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QRectF>::push_front(const QRectF& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_push_front(self as *mut ::list::ListQtCoreRectF,
                                                  t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QRectF>::removeAll(const QRectF& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::qt_core::rect_f::RectF) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_removeAll(self as *mut ::list::ListQtCoreRectF,
                                                 t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QRectF>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_removeAt(self as *mut ::list::ListQtCoreRectF, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QRectF>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_removeFirst(self as *mut ::list::ListQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QRectF>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_removeLast(self as *mut ::list::ListQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QRectF>::removeOne(const QRectF& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::qt_core::rect_f::RectF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_removeOne(self as *mut ::list::ListQtCoreRectF,
                                                 t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QRectF>::replace(int i, const QRectF& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::qt_core::rect_f::RectF) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_replace(self as *mut ::list::ListQtCoreRectF,
                                               i,
                                               t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QRectF>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_reserve(self as *mut ::list::ListQtCoreRectF, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QRectF>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_size(self as *const ::list::ListQtCoreRectF) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QRectF>::startsWith(const QRectF& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::qt_core::rect_f::RectF) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QRectF_startsWith(self as *const ::list::ListQtCoreRectF,
                                                  t as *const ::qt_core::rect_f::RectF)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QRectF>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListQtCoreRectF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QRectF>::swap(QList<QRectF>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QRectF>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListQtCoreRectFSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QRectF QList<QRectF>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_takeAt_to_output(self as *mut ::list::ListQtCoreRectF, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QList<QRectF>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_takeFirst_to_output(self as *mut ::list::ListQtCoreRectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QRectF QList<QRectF>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::qt_core::rect_f::RectF {
    {
      let mut object: ::qt_core::rect_f::RectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_takeLast_to_output(self as *mut ::list::ListQtCoreRectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QRectF> QList<QRectF>::toVector() const```</span>
  ///
  ///
  pub fn to_vector(&self) -> ::qt_gui::vector::VectorQtCoreRectF {
    {
      let mut object: ::qt_gui::vector::VectorQtCoreRectF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_toVector_to_output(self as *const ::list::ListQtCoreRectF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QRectF>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QList<QRectF>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::qt_core::rect_f::RectF)) -> ::qt_core::rect_f::RectF```<br>
  /// C++ method: <span style='color: green;'>```QRectF QList<QRectF>::value(int i, const QRectF& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::qt_core::rect_f::RectF
    where Args: overloading::ListQtCoreRectFValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListQtCoreRectF {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QRectF>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QRectF_destructor(self as *mut ::list::ListQtCoreRectF) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QScroller*>```</span>
#[repr(C)]
pub struct ListScrollerMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_SCROLLER_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListScrollerMutPtr {
  unsafe fn new_uninitialized() -> ListScrollerMutPtr {
    ListScrollerMutPtr(::std::mem::uninitialized())
  }
}

impl ListScrollerMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::append(const QList<QScroller*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListScrollerMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QScroller_ptr_append_QList_QScroller_ptr(self as *mut ::list::ListScrollerMutPtr,
                                                                         t as *const ::list::ListScrollerMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::append(QScroller* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::scroller::Scroller) {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_append_QScroller(self as *mut ::list::ListScrollerMutPtr,
                                                             t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```QScroller* const & QList<QScroller*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::scroller::Scroller {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_at(self as *const ::list::ListScrollerMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScroller* const & QList<QScroller*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::scroller::Scroller {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_back_const(self as *const ::list::ListScrollerMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScroller*& QList<QScroller*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::scroller::Scroller {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_back(self as *mut ::list::ListScrollerMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_clear(self as *mut ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QScroller* const & QList<QScroller*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::scroller::Scroller {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_constFirst(self as *const ::list::ListScrollerMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScroller* const & QList<QScroller*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::scroller::Scroller {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_constLast(self as *const ::list::ListScrollerMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScroller*>::contains(QScroller* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::scroller::Scroller) -> bool {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_contains(self as *const ::list::ListScrollerMutPtr,
                                                     t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```int QList<QScroller*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_count_no_args(self as *const ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QScroller*>::count(QScroller* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::scroller::Scroller) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_count_t(self as *const ::list::ListScrollerMutPtr,
                                                    t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScroller*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_empty(self as *const ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScroller*>::endsWith(QScroller* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::scroller::Scroller) -> bool {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_endsWith(self as *const ::list::ListScrollerMutPtr,
                                                     t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```QScroller* const & QList<QScroller*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::scroller::Scroller {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_first_const(self as *const ::list::ListScrollerMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScroller*& QList<QScroller*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::scroller::Scroller {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_first(self as *mut ::list::ListScrollerMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScroller* const & QList<QScroller*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::scroller::Scroller {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_front_const(self as *const ::list::ListScrollerMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScroller*& QList<QScroller*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::scroller::Scroller {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_front(self as *mut ::list::ListScrollerMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QScroller*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::scroller::Scroller) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QScroller*>::indexOf(QScroller* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::scroller::Scroller, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QScroller*>::indexOf(QScroller* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListScrollerMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::insert(int i, QScroller* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::scroller::Scroller) {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_insert(self as *mut ::list::ListScrollerMutPtr,
                                                   i,
                                                   t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScroller*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_isEmpty(self as *const ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QScroller* const & QList<QScroller*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::scroller::Scroller {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_last_const(self as *const ::list::ListScrollerMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QScroller*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::scroller::Scroller) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QScroller*>::lastIndexOf(QScroller* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::scroller::Scroller, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QScroller*>::lastIndexOf(QScroller* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListScrollerMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QScroller*& QList<QScroller*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::scroller::Scroller {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_last(self as *mut ::list::ListScrollerMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QScroller*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_length(self as *const ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QScroller*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListScrollerMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QScroller*> QList<QScroller*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListScrollerMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QScroller*> QList<QScroller*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListScrollerMutPtr
    where Args: overloading::ListScrollerMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_move(self as *mut ::list::ListScrollerMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QScroller*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListScrollerMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QScroller*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListScrollerMutPtr) -> ::list::ListScrollerMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QScroller*>::QList(const QList<QScroller*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListScrollerMutPtr
    where Args: overloading::ListScrollerMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QScroller*> QList<QScroller*>::operator+(const QList<QScroller*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListScrollerMutPtr) -> ::list::ListScrollerMutPtr {
    {
      let mut object: ::list::ListScrollerMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QScroller_ptr_operator_add_to_output(self as *const ::list::ListScrollerMutPtr,
                                                                       l as *const ::list::ListScrollerMutPtr,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QScroller*>& QList<QScroller*>::operator+=(const QList<QScroller*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListScrollerMutPtr)
                                 -> &'l0 mut ::list::ListScrollerMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QScroller_ptr_operator_add_assign_l(self as *mut ::list::ListScrollerMutPtr,
                                                                      l as *const ::list::ListScrollerMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QScroller*>& QList<QScroller*>::operator+=(QScroller* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::scroller::Scroller)
                                               -> &'l0 mut ::list::ListScrollerMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QList_QScroller_ptr_operator_add_assign_t(self as *mut ::list::ListScrollerMutPtr,
                                                                    t as *const *mut ::scroller::Scroller);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QScroller*>& QList<QScroller*>::operator=(const QList<QScroller*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListScrollerMutPtr) -> &'l0 mut ::list::ListScrollerMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QList_QScroller_ptr_operator_assign(self as *mut ::list::ListScrollerMutPtr,
                                                              l as *const ::list::ListScrollerMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScroller*>::operator==(const QList<QScroller*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListScrollerMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QScroller_ptr_operator_eq(self as *const ::list::ListScrollerMutPtr,
                                                          l as *const ::list::ListScrollerMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QScroller* const & QList<QScroller*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::scroller::Scroller {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QScroller_ptr_operator_index_const(self as *const ::list::ListScrollerMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScroller*& QList<QScroller*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::scroller::Scroller {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_operator_index(self as *mut ::list::ListScrollerMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScroller*>::operator!=(const QList<QScroller*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListScrollerMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QScroller_ptr_operator_neq(self as *const ::list::ListScrollerMutPtr,
                                                           l as *const ::list::ListScrollerMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QScroller*>& QList<QScroller*>::operator<<(const QList<QScroller*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListScrollerMutPtr) -> &'l0 mut ::list::ListScrollerMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QList_QScroller_ptr_operator_shl_l(self as *mut ::list::ListScrollerMutPtr,
                                                             l as *const ::list::ListScrollerMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QScroller*>& QList<QScroller*>::operator<<(QScroller* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::scroller::Scroller)
                                        -> &'l0 mut ::list::ListScrollerMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QScroller_ptr_operator_shl_t(self as *mut ::list::ListScrollerMutPtr,
                                                                            t as *const *mut ::scroller::Scroller);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_pop_back(self as *mut ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_pop_front(self as *mut ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::prepend(QScroller* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::scroller::Scroller) {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_prepend(self as *mut ::list::ListScrollerMutPtr,
                                                    t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::push_back(QScroller* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::scroller::Scroller) {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_push_back(self as *mut ::list::ListScrollerMutPtr,
                                                      t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::push_front(QScroller* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::scroller::Scroller) {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_push_front(self as *mut ::list::ListScrollerMutPtr,
                                                       t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```int QList<QScroller*>::removeAll(QScroller* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::scroller::Scroller) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_removeAll(self as *mut ::list::ListScrollerMutPtr,
                                                      t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_removeAt(self as *mut ::list::ListScrollerMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_removeFirst(self as *mut ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_removeLast(self as *mut ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScroller*>::removeOne(QScroller* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::scroller::Scroller) -> bool {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_removeOne(self as *mut ::list::ListScrollerMutPtr,
                                                      t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::replace(int i, QScroller* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::scroller::Scroller) {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_replace(self as *mut ::list::ListScrollerMutPtr,
                                                    i,
                                                    t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_reserve(self as *mut ::list::ListScrollerMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QScroller*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_size(self as *const ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScroller*>::startsWith(QScroller* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::scroller::Scroller) -> bool {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_startsWith(self as *const ::list::ListScrollerMutPtr,
                                                       t as *const *mut ::scroller::Scroller)
  }

  /// C++ method: <span style='color: green;'>```QList<QScroller*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListScrollerMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::swap(QList<QScroller*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QScroller*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListScrollerMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QScroller* QList<QScroller*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::scroller::Scroller {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_takeAt(self as *mut ::list::ListScrollerMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QScroller* QList<QScroller*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::scroller::Scroller {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_takeFirst(self as *mut ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QScroller* QList<QScroller*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::scroller::Scroller {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_takeLast(self as *mut ::list::ListScrollerMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QScroller* QList<QScroller*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::scroller::Scroller {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_value_i(self as *const ::list::ListScrollerMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QScroller* QList<QScroller*>::value(int i, QScroller* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::scroller::Scroller)
                             -> *mut ::scroller::Scroller {
    ::ffi::qt_widgets_c_QList_QScroller_ptr_value_i_defaultValue(self as *const ::list::ListScrollerMutPtr,
                                                                 i,
                                                                 default_value as *const *mut ::scroller::Scroller)
  }
}

impl Drop for ::list::ListScrollerMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QScroller*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QScroller_ptr_destructor(self as *mut ::list::ListScrollerMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QTableWidgetItem*>```</span>
#[repr(C)]
pub struct ListTableWidgetItemMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_TABLE_WIDGET_ITEM_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListTableWidgetItemMutPtr {
  unsafe fn new_uninitialized() -> ListTableWidgetItemMutPtr {
    ListTableWidgetItemMutPtr(::std::mem::uninitialized())
  }
}

impl ListTableWidgetItemMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::append(const QList<QTableWidgetItem*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListTableWidgetItemMutPtr) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_append_QList_QTableWidgetItem_ptr(self as *mut ::list::ListTableWidgetItemMutPtr, t as *const ::list::ListTableWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::append(QTableWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_append_QTableWidgetItem(self as *mut ::list::ListTableWidgetItemMutPtr, t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* const & QList<QTableWidgetItem*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_at(self as *const ::list::ListTableWidgetItemMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* const & QList<QTableWidgetItem*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_back_const(self as *const ::list::ListTableWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem*& QList<QTableWidgetItem*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_back(self as *mut ::list::ListTableWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_clear(self as *mut ::list::ListTableWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* const & QList<QTableWidgetItem*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_constFirst(self as *const ::list::ListTableWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* const & QList<QTableWidgetItem*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_constLast(self as *const ::list::ListTableWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTableWidgetItem*>::contains(QTableWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::table_widget_item::TableWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_contains(self as *const ::list::ListTableWidgetItemMutPtr,
                                                            t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetItem*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_count_no_args(self as *const ::list::ListTableWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetItem*>::count(QTableWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::table_widget_item::TableWidgetItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_count_t(self as *const ::list::ListTableWidgetItemMutPtr,
                                                           t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTableWidgetItem*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_empty(self as *const ::list::ListTableWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTableWidgetItem*>::endsWith(QTableWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::table_widget_item::TableWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_endsWith(self as *const ::list::ListTableWidgetItemMutPtr,
                                                            t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* const & QList<QTableWidgetItem*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_first_const(self as *const ::list::ListTableWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem*& QList<QTableWidgetItem*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_first(self as *mut ::list::ListTableWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* const & QList<QTableWidgetItem*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_front_const(self as *const ::list::ListTableWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem*& QList<QTableWidgetItem*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_front(self as *mut ::list::ListTableWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::table_widget_item::TableWidgetItem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetItem*>::indexOf(QTableWidgetItem* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::table_widget_item::TableWidgetItem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetItem*>::indexOf(QTableWidgetItem* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTableWidgetItemMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::insert(int i, QTableWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_insert(self as *mut ::list::ListTableWidgetItemMutPtr,
                                                          i,
                                                          t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTableWidgetItem*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_isEmpty(self as *const ::list::ListTableWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* const & QList<QTableWidgetItem*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_last_const(self as *const ::list::ListTableWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::table_widget_item::TableWidgetItem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetItem*>::lastIndexOf(QTableWidgetItem* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::table_widget_item::TableWidgetItem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetItem*>::lastIndexOf(QTableWidgetItem* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTableWidgetItemMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTableWidgetItem*& QList<QTableWidgetItem*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_last(self as *mut ::list::ListTableWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetItem*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_length(self as *const ::list::ListTableWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListTableWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*> QList<QTableWidgetItem*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListTableWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*> QList<QTableWidgetItem*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListTableWidgetItemMutPtr
    where Args: overloading::ListTableWidgetItemMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_move(self as *mut ::list::ListTableWidgetItemMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListTableWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTableWidgetItem*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListTableWidgetItemMutPtr) -> ::list::ListTableWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTableWidgetItem*>::QList(const QList<QTableWidgetItem*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListTableWidgetItemMutPtr
    where Args: overloading::ListTableWidgetItemMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*> QList<QTableWidgetItem*>::operator+(const QList<QTableWidgetItem*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListTableWidgetItemMutPtr) -> ::list::ListTableWidgetItemMutPtr {
    {
      let mut object: ::list::ListTableWidgetItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_operator_add_to_output(self as *const ::list::ListTableWidgetItemMutPtr, l as *const ::list::ListTableWidgetItemMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*>& QList<QTableWidgetItem*>::operator+=(const QList<QTableWidgetItem*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListTableWidgetItemMutPtr)
                                 -> &'l0 mut ::list::ListTableWidgetItemMutPtr {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_operator_add_assign_l(self as *mut ::list::ListTableWidgetItemMutPtr, l as *const ::list::ListTableWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*>& QList<QTableWidgetItem*>::operator+=(QTableWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::table_widget_item::TableWidgetItem)
                                               -> &'l0 mut ::list::ListTableWidgetItemMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_operator_add_assign_t(self as *mut ::list::ListTableWidgetItemMutPtr, t as *const *mut ::table_widget_item::TableWidgetItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*>& QList<QTableWidgetItem*>::operator=(const QList<QTableWidgetItem*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListTableWidgetItemMutPtr)
                             -> &'l0 mut ::list::ListTableWidgetItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_operator_assign(self as *mut ::list::ListTableWidgetItemMutPtr,
                                                                       l as *const ::list::ListTableWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTableWidgetItem*>::operator==(const QList<QTableWidgetItem*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListTableWidgetItemMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_operator_eq(self as *const ::list::ListTableWidgetItemMutPtr,
                                                                 l as *const ::list::ListTableWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* const & QList<QTableWidgetItem*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::table_widget_item::TableWidgetItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_operator_index_const(self as *const ::list::ListTableWidgetItemMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem*& QList<QTableWidgetItem*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::table_widget_item::TableWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_operator_index(self as *mut ::list::ListTableWidgetItemMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTableWidgetItem*>::operator!=(const QList<QTableWidgetItem*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListTableWidgetItemMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_operator_neq(self as *const ::list::ListTableWidgetItemMutPtr,
                                                                  l as *const ::list::ListTableWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*>& QList<QTableWidgetItem*>::operator<<(const QList<QTableWidgetItem*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListTableWidgetItemMutPtr)
                          -> &'l0 mut ::list::ListTableWidgetItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_operator_shl_l(self as *mut ::list::ListTableWidgetItemMutPtr,
                                                                      l as *const ::list::ListTableWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*>& QList<QTableWidgetItem*>::operator<<(QTableWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::table_widget_item::TableWidgetItem)
                                        -> &'l0 mut ::list::ListTableWidgetItemMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_operator_shl_t(self as *mut ::list::ListTableWidgetItemMutPtr, t as *const *mut ::table_widget_item::TableWidgetItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_pop_back(self as *mut ::list::ListTableWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_pop_front(self as *mut ::list::ListTableWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::prepend(QTableWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_prepend(self as *mut ::list::ListTableWidgetItemMutPtr,
                                                           t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::push_back(QTableWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_push_back(self as *mut ::list::ListTableWidgetItemMutPtr,
                                                             t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::push_front(QTableWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_push_front(self as *mut ::list::ListTableWidgetItemMutPtr,
                                                              t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetItem*>::removeAll(QTableWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::table_widget_item::TableWidgetItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_removeAll(self as *mut ::list::ListTableWidgetItemMutPtr,
                                                             t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_removeAt(self as *mut ::list::ListTableWidgetItemMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_removeFirst(self as *mut ::list::ListTableWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_removeLast(self as *mut ::list::ListTableWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTableWidgetItem*>::removeOne(QTableWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::table_widget_item::TableWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_removeOne(self as *mut ::list::ListTableWidgetItemMutPtr,
                                                             t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::replace(int i, QTableWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::table_widget_item::TableWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_replace(self as *mut ::list::ListTableWidgetItemMutPtr,
                                                           i,
                                                           t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_reserve(self as *mut ::list::ListTableWidgetItemMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetItem*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_size(self as *const ::list::ListTableWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTableWidgetItem*>::startsWith(QTableWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::table_widget_item::TableWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_startsWith(self as *const ::list::ListTableWidgetItemMutPtr,
                                                              t as *const *mut ::table_widget_item::TableWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetItem*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListTableWidgetItemMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::swap(QList<QTableWidgetItem*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetItem*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTableWidgetItemMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QList<QTableWidgetItem*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_takeAt(self as *mut ::list::ListTableWidgetItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QList<QTableWidgetItem*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_takeFirst(self as *mut ::list::ListTableWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QList<QTableWidgetItem*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_takeLast(self as *mut ::list::ListTableWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QList<QTableWidgetItem*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::table_widget_item::TableWidgetItem {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_value_i(self as *const ::list::ListTableWidgetItemMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetItem* QList<QTableWidgetItem*>::value(int i, QTableWidgetItem* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::table_widget_item::TableWidgetItem)
                             -> *mut ::table_widget_item::TableWidgetItem {
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_value_i_defaultValue(self as *const ::list::ListTableWidgetItemMutPtr, i, default_value as *const *mut ::table_widget_item::TableWidgetItem)
  }
}

impl Drop for ::list::ListTableWidgetItemMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QTableWidgetItem*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_destructor(self as *mut ::list::ListTableWidgetItemMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QTableWidgetSelectionRange>```</span>
#[repr(C)]
pub struct ListTableWidgetSelectionRange([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_TABLE_WIDGET_SELECTION_RANGE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListTableWidgetSelectionRange {
  unsafe fn new_uninitialized() -> ListTableWidgetSelectionRange {
    ListTableWidgetSelectionRange(::std::mem::uninitialized())
  }
}

impl ListTableWidgetSelectionRange {
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListTableWidgetSelectionRange) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::append(const QList<QTableWidgetSelectionRange>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::table_widget_selection_range::TableWidgetSelectionRange) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::append(const QTableWidgetSelectionRange& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTableWidgetSelectionRangeAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_at(self as *const ::list::ListTableWidgetSelectionRange, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_back_const(self as *const ::list::ListTableWidgetSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_back(self as *mut ::list::ListTableWidgetSelectionRange)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_clear(self as *mut ::list::ListTableWidgetSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_constFirst(self as *const ::list::ListTableWidgetSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_constLast(self as *const ::list::ListTableWidgetSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetSelectionRange>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_count(self as *const ::list::ListTableWidgetSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTableWidgetSelectionRange>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_empty(self as *const ::list::ListTableWidgetSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_first_const(self as *const ::list::ListTableWidgetSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_first(self as *mut ::list::ListTableWidgetSelectionRange)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_front_const(self as *const ::list::ListTableWidgetSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_front(self as *mut ::list::ListTableWidgetSelectionRange)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::insert(int i, const QTableWidgetSelectionRange& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::table_widget_selection_range::TableWidgetSelectionRange) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_insert(self as *mut ::list::ListTableWidgetSelectionRange, i, t as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTableWidgetSelectionRange>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_isEmpty(self as *const ::list::ListTableWidgetSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_last_const(self as *const ::list::ListTableWidgetSelectionRange) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_last(self as *mut ::list::ListTableWidgetSelectionRange)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetSelectionRange>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_length(self as *const ::list::ListTableWidgetSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListTableWidgetSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange> QList<QTableWidgetSelectionRange>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListTableWidgetSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange> QList<QTableWidgetSelectionRange>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListTableWidgetSelectionRange
    where Args: overloading::ListTableWidgetSelectionRangeMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_move(self as *mut ::list::ListTableWidgetSelectionRange,
                                                                from,
                                                                to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListTableWidgetSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTableWidgetSelectionRange>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListTableWidgetSelectionRange) -> ::list::ListTableWidgetSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTableWidgetSelectionRange>::QList(const QList<QTableWidgetSelectionRange>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListTableWidgetSelectionRange
    where Args: overloading::ListTableWidgetSelectionRangeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange> QList<QTableWidgetSelectionRange>::operator+(const QList<QTableWidgetSelectionRange>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListTableWidgetSelectionRange) -> ::list::ListTableWidgetSelectionRange {
    {
      let mut object: ::list::ListTableWidgetSelectionRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_operator_add_to_output(self as *const ::list::ListTableWidgetSelectionRange, l as *const ::list::ListTableWidgetSelectionRange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListTableWidgetSelectionRange) -> &'l0 mut ::list::ListTableWidgetSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange>& QList<QTableWidgetSelectionRange>::operator+=(const QList<QTableWidgetSelectionRange>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::table_widget_selection_range::TableWidgetSelectionRange) -> &'l0 mut ::list::ListTableWidgetSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange>& QList<QTableWidgetSelectionRange>::operator+=(const QTableWidgetSelectionRange& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTableWidgetSelectionRange
    where Args: overloading::ListTableWidgetSelectionRangeOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange>& QList<QTableWidgetSelectionRange>::operator=(const QList<QTableWidgetSelectionRange>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListTableWidgetSelectionRange)
                             -> &'l0 mut ::list::ListTableWidgetSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_operator_assign(self as *mut ::list::ListTableWidgetSelectionRange, l as *const ::list::ListTableWidgetSelectionRange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_operator_index_const(self as *const ::list::ListTableWidgetSelectionRange, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetSelectionRange& QList<QTableWidgetSelectionRange>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self,
                           i: ::libc::c_int)
                           -> &'l0 mut ::table_widget_selection_range::TableWidgetSelectionRange {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_operator_index(self as *mut ::list::ListTableWidgetSelectionRange, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListTableWidgetSelectionRange) -> &'l0 mut ::list::ListTableWidgetSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange>& QList<QTableWidgetSelectionRange>::operator<<(const QList<QTableWidgetSelectionRange>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::table_widget_selection_range::TableWidgetSelectionRange) -> &'l0 mut ::list::ListTableWidgetSelectionRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange>& QList<QTableWidgetSelectionRange>::operator<<(const QTableWidgetSelectionRange& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTableWidgetSelectionRange
    where Args: overloading::ListTableWidgetSelectionRangeOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_pop_back(self as *mut ::list::ListTableWidgetSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_pop_front(self as *mut ::list::ListTableWidgetSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::prepend(const QTableWidgetSelectionRange& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::table_widget_selection_range::TableWidgetSelectionRange) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_prepend(self as *mut ::list::ListTableWidgetSelectionRange, t as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::push_back(const QTableWidgetSelectionRange& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::table_widget_selection_range::TableWidgetSelectionRange) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_push_back(self as *mut ::list::ListTableWidgetSelectionRange, t as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::push_front(const QTableWidgetSelectionRange& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::table_widget_selection_range::TableWidgetSelectionRange) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_push_front(self as *mut ::list::ListTableWidgetSelectionRange, t as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_removeAt(self as *mut ::list::ListTableWidgetSelectionRange, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_removeFirst(self as *mut ::list::ListTableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_removeLast(self as *mut ::list::ListTableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::replace(int i, const QTableWidgetSelectionRange& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::table_widget_selection_range::TableWidgetSelectionRange) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_replace(self as *mut ::list::ListTableWidgetSelectionRange, i, t as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_reserve(self as *mut ::list::ListTableWidgetSelectionRange,
                                                                   size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTableWidgetSelectionRange>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_size(self as *const ::list::ListTableWidgetSelectionRange)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTableWidgetSelectionRange>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListTableWidgetSelectionRange) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::swap(QList<QTableWidgetSelectionRange>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTableWidgetSelectionRange>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTableWidgetSelectionRangeSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTableWidgetSelectionRange QList<QTableWidgetSelectionRange>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::table_widget_selection_range::TableWidgetSelectionRange {
    {
      let mut object: ::table_widget_selection_range::TableWidgetSelectionRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_takeAt_to_output(self as *mut ::list::ListTableWidgetSelectionRange, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetSelectionRange QList<QTableWidgetSelectionRange>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::table_widget_selection_range::TableWidgetSelectionRange {
    {
      let mut object: ::table_widget_selection_range::TableWidgetSelectionRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_takeFirst_to_output(self as *mut ::list::ListTableWidgetSelectionRange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTableWidgetSelectionRange QList<QTableWidgetSelectionRange>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::table_widget_selection_range::TableWidgetSelectionRange {
    {
      let mut object: ::table_widget_selection_range::TableWidgetSelectionRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_takeLast_to_output(self as *mut ::list::ListTableWidgetSelectionRange, &mut object);
      }
      object
    }
  }
}

impl Drop for ::list::ListTableWidgetSelectionRange {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QTableWidgetSelectionRange>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_destructor(self as *mut ::list::ListTableWidgetSelectionRange) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>```</span>
#[repr(C)]
pub struct ListTextEditExtraSelection([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_TEXT_EDIT_EXTRA_SELECTION]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListTextEditExtraSelection {
  unsafe fn new_uninitialized() -> ListTextEditExtraSelection {
    ListTextEditExtraSelection(::std::mem::uninitialized())
  }
}

impl ListTextEditExtraSelection {
  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListTextEditExtraSelection) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::append(const QList<QTextEdit::ExtraSelection>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::text_edit::ExtraSelection) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::append(const QTextEdit::ExtraSelection& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTextEditExtraSelectionAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_edit::ExtraSelection {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_at(self as *const ::list::ListTextEditExtraSelection, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::text_edit::ExtraSelection {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_back_const(self as *const ::list::ListTextEditExtraSelection)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_edit::ExtraSelection {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_back(self as *mut ::list::ListTextEditExtraSelection)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_clear(self as *mut ::list::ListTextEditExtraSelection) }
  }

  /// C++ method: <span style='color: green;'>```const QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::text_edit::ExtraSelection {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_constFirst(self as *const ::list::ListTextEditExtraSelection)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::text_edit::ExtraSelection {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_constLast(self as *const ::list::ListTextEditExtraSelection)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextEdit::ExtraSelection>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_count(self as *const ::list::ListTextEditExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextEdit::ExtraSelection>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_empty(self as *const ::list::ListTextEditExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::text_edit::ExtraSelection {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_first_const(self as *const ::list::ListTextEditExtraSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_edit::ExtraSelection {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_first(self as *mut ::list::ListTextEditExtraSelection)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::text_edit::ExtraSelection {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_front_const(self as *const ::list::ListTextEditExtraSelection) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_edit::ExtraSelection {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_front(self as *mut ::list::ListTextEditExtraSelection)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::insert(int i, const QTextEdit::ExtraSelection& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::text_edit::ExtraSelection) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_insert(self as *mut ::list::ListTextEditExtraSelection,
                                                                i,
                                                                t as *const ::text_edit::ExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextEdit::ExtraSelection>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_isEmpty(self as *const ::list::ListTextEditExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::text_edit::ExtraSelection {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_last_const(self as *const ::list::ListTextEditExtraSelection)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_edit::ExtraSelection {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_last(self as *mut ::list::ListTextEditExtraSelection)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextEdit::ExtraSelection>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_length(self as *const ::list::ListTextEditExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListTextEditExtraSelection```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection> QList<QTextEdit::ExtraSelection>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListTextEditExtraSelection```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection> QList<QTextEdit::ExtraSelection>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListTextEditExtraSelection
    where Args: overloading::ListTextEditExtraSelectionMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_move(self as *mut ::list::ListTextEditExtraSelection, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListTextEditExtraSelection```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTextEdit::ExtraSelection>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListTextEditExtraSelection) -> ::list::ListTextEditExtraSelection```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTextEdit::ExtraSelection>::QList(const QList<QTextEdit::ExtraSelection>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListTextEditExtraSelection
    where Args: overloading::ListTextEditExtraSelectionNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection> QList<QTextEdit::ExtraSelection>::operator+(const QList<QTextEdit::ExtraSelection>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListTextEditExtraSelection) -> ::list::ListTextEditExtraSelection {
    {
      let mut object: ::list::ListTextEditExtraSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_add_to_output(self as *const ::list::ListTextEditExtraSelection, l as *const ::list::ListTextEditExtraSelection, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListTextEditExtraSelection) -> &'l0 mut ::list::ListTextEditExtraSelection```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>& QList<QTextEdit::ExtraSelection>::operator+=(const QList<QTextEdit::ExtraSelection>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::text_edit::ExtraSelection) -> &'l0 mut ::list::ListTextEditExtraSelection```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>& QList<QTextEdit::ExtraSelection>::operator+=(const QTextEdit::ExtraSelection& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTextEditExtraSelection
    where Args: overloading::ListTextEditExtraSelectionOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>& QList<QTextEdit::ExtraSelection>::operator=(const QList<QTextEdit::ExtraSelection>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListTextEditExtraSelection)
                             -> &'l0 mut ::list::ListTextEditExtraSelection {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_assign(self as *mut ::list::ListTextEditExtraSelection, l as *const ::list::ListTextEditExtraSelection) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_edit::ExtraSelection {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_index_const(self as *const ::list::ListTextEditExtraSelection, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::ExtraSelection& QList<QTextEdit::ExtraSelection>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::text_edit::ExtraSelection {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_index(self as *mut ::list::ListTextEditExtraSelection, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListTextEditExtraSelection) -> &'l0 mut ::list::ListTextEditExtraSelection```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>& QList<QTextEdit::ExtraSelection>::operator<<(const QList<QTextEdit::ExtraSelection>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::text_edit::ExtraSelection) -> &'l0 mut ::list::ListTextEditExtraSelection```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>& QList<QTextEdit::ExtraSelection>::operator<<(const QTextEdit::ExtraSelection& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTextEditExtraSelection
    where Args: overloading::ListTextEditExtraSelectionOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_pop_back(self as *mut ::list::ListTextEditExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_pop_front(self as *mut ::list::ListTextEditExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::prepend(const QTextEdit::ExtraSelection& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::text_edit::ExtraSelection) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_prepend(self as *mut ::list::ListTextEditExtraSelection,
                                                                 t as *const ::text_edit::ExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::push_back(const QTextEdit::ExtraSelection& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::text_edit::ExtraSelection) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_push_back(self as *mut ::list::ListTextEditExtraSelection,
                                                                   t as *const ::text_edit::ExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::push_front(const QTextEdit::ExtraSelection& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::text_edit::ExtraSelection) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_push_front(self as *mut ::list::ListTextEditExtraSelection,
                                                                    t as *const ::text_edit::ExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_removeAt(self as *mut ::list::ListTextEditExtraSelection, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_removeFirst(self as *mut ::list::ListTextEditExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_removeLast(self as *mut ::list::ListTextEditExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::replace(int i, const QTextEdit::ExtraSelection& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::text_edit::ExtraSelection) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_replace(self as *mut ::list::ListTextEditExtraSelection,
                                                                 i,
                                                                 t as *const ::text_edit::ExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_reserve(self as *mut ::list::ListTextEditExtraSelection, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextEdit::ExtraSelection>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_size(self as *const ::list::ListTextEditExtraSelection)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextEdit::ExtraSelection>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListTextEditExtraSelection) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::swap(QList<QTextEdit::ExtraSelection>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextEdit::ExtraSelection>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTextEditExtraSelectionSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextEdit::ExtraSelection QList<QTextEdit::ExtraSelection>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::text_edit::ExtraSelection {
    {
      let mut object: ::text_edit::ExtraSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_takeAt_to_output(self as *mut ::list::ListTextEditExtraSelection, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::ExtraSelection QList<QTextEdit::ExtraSelection>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::text_edit::ExtraSelection {
    {
      let mut object: ::text_edit::ExtraSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_takeFirst_to_output(self as *mut ::list::ListTextEditExtraSelection, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextEdit::ExtraSelection QList<QTextEdit::ExtraSelection>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::text_edit::ExtraSelection {
    {
      let mut object: ::text_edit::ExtraSelection =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_takeLast_to_output(self as *mut ::list::ListTextEditExtraSelection, &mut object);
      }
      object
    }
  }
}

impl Drop for ::list::ListTextEditExtraSelection {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QTextEdit::ExtraSelection>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_destructor(self as *mut ::list::ListTextEditExtraSelection)
    }
  }
}

/// C++ type: <span style='color: green;'>```QList<QTreeWidgetItem*>```</span>
#[repr(C)]
pub struct ListTreeWidgetItemMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_TREE_WIDGET_ITEM_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListTreeWidgetItemMutPtr {
  unsafe fn new_uninitialized() -> ListTreeWidgetItemMutPtr {
    ListTreeWidgetItemMutPtr(::std::mem::uninitialized())
  }
}

impl ListTreeWidgetItemMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::append(const QList<QTreeWidgetItem*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListTreeWidgetItemMutPtr) {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_append_QList_QTreeWidgetItem_ptr(self as *mut ::list::ListTreeWidgetItemMutPtr, t as *const ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::append(QTreeWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_append_QTreeWidgetItem(self as *mut ::list::ListTreeWidgetItemMutPtr, t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* const & QList<QTreeWidgetItem*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_at(self as *const ::list::ListTreeWidgetItemMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* const & QList<QTreeWidgetItem*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_back_const(self as *const ::list::ListTreeWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem*& QList<QTreeWidgetItem*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_back(self as *mut ::list::ListTreeWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_clear(self as *mut ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* const & QList<QTreeWidgetItem*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_constFirst(self as *const ::list::ListTreeWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* const & QList<QTreeWidgetItem*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_constLast(self as *const ::list::ListTreeWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTreeWidgetItem*>::contains(QTreeWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::tree_widget_item::TreeWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_contains(self as *const ::list::ListTreeWidgetItemMutPtr,
                                                           t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```int QList<QTreeWidgetItem*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_count_no_args(self as *const ::list::ListTreeWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTreeWidgetItem*>::count(QTreeWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::tree_widget_item::TreeWidgetItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_count_t(self as *const ::list::ListTreeWidgetItemMutPtr,
                                                          t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTreeWidgetItem*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_empty(self as *const ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTreeWidgetItem*>::endsWith(QTreeWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::tree_widget_item::TreeWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_endsWith(self as *const ::list::ListTreeWidgetItemMutPtr,
                                                           t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* const & QList<QTreeWidgetItem*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_first_const(self as *const ::list::ListTreeWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem*& QList<QTreeWidgetItem*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_first(self as *mut ::list::ListTreeWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* const & QList<QTreeWidgetItem*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_front_const(self as *const ::list::ListTreeWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem*& QList<QTreeWidgetItem*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_front(self as *mut ::list::ListTreeWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::tree_widget_item::TreeWidgetItem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTreeWidgetItem*>::indexOf(QTreeWidgetItem* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTreeWidgetItem*>::indexOf(QTreeWidgetItem* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTreeWidgetItemMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::insert(int i, QTreeWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_insert(self as *mut ::list::ListTreeWidgetItemMutPtr,
                                                         i,
                                                         t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTreeWidgetItem*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_isEmpty(self as *const ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* const & QList<QTreeWidgetItem*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_last_const(self as *const ::list::ListTreeWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::tree_widget_item::TreeWidgetItem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTreeWidgetItem*>::lastIndexOf(QTreeWidgetItem* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTreeWidgetItem*>::lastIndexOf(QTreeWidgetItem* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTreeWidgetItemMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTreeWidgetItem*& QList<QTreeWidgetItem*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_last(self as *mut ::list::ListTreeWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTreeWidgetItem*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_length(self as *const ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListTreeWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*> QList<QTreeWidgetItem*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListTreeWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*> QList<QTreeWidgetItem*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListTreeWidgetItemMutPtr
    where Args: overloading::ListTreeWidgetItemMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_move(self as *mut ::list::ListTreeWidgetItemMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListTreeWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTreeWidgetItem*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListTreeWidgetItemMutPtr) -> ::list::ListTreeWidgetItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTreeWidgetItem*>::QList(const QList<QTreeWidgetItem*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListTreeWidgetItemMutPtr
    where Args: overloading::ListTreeWidgetItemMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*> QList<QTreeWidgetItem*>::operator+(const QList<QTreeWidgetItem*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListTreeWidgetItemMutPtr) -> ::list::ListTreeWidgetItemMutPtr {
    {
      let mut object: ::list::ListTreeWidgetItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_add_to_output(self as *const ::list::ListTreeWidgetItemMutPtr, l as *const ::list::ListTreeWidgetItemMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*>& QList<QTreeWidgetItem*>::operator+=(const QList<QTreeWidgetItem*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListTreeWidgetItemMutPtr)
                                 -> &'l0 mut ::list::ListTreeWidgetItemMutPtr {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_add_assign_l(self as *mut ::list::ListTreeWidgetItemMutPtr, l as *const ::list::ListTreeWidgetItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*>& QList<QTreeWidgetItem*>::operator+=(QTreeWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::tree_widget_item::TreeWidgetItem)
                                               -> &'l0 mut ::list::ListTreeWidgetItemMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_add_assign_t(self as *mut ::list::ListTreeWidgetItemMutPtr, t as *const *mut ::tree_widget_item::TreeWidgetItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*>& QList<QTreeWidgetItem*>::operator=(const QList<QTreeWidgetItem*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListTreeWidgetItemMutPtr)
                             -> &'l0 mut ::list::ListTreeWidgetItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_assign(self as *mut ::list::ListTreeWidgetItemMutPtr,
                                                                      l as *const ::list::ListTreeWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTreeWidgetItem*>::operator==(const QList<QTreeWidgetItem*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListTreeWidgetItemMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_eq(self as *const ::list::ListTreeWidgetItemMutPtr,
                                                                l as *const ::list::ListTreeWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* const & QList<QTreeWidgetItem*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_index_const(self as *const ::list::ListTreeWidgetItemMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem*& QList<QTreeWidgetItem*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::tree_widget_item::TreeWidgetItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_index(self as *mut ::list::ListTreeWidgetItemMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTreeWidgetItem*>::operator!=(const QList<QTreeWidgetItem*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListTreeWidgetItemMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_neq(self as *const ::list::ListTreeWidgetItemMutPtr,
                                                                 l as *const ::list::ListTreeWidgetItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*>& QList<QTreeWidgetItem*>::operator<<(const QList<QTreeWidgetItem*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListTreeWidgetItemMutPtr)
                          -> &'l0 mut ::list::ListTreeWidgetItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_shl_l(self as *mut ::list::ListTreeWidgetItemMutPtr,
                                                                     l as *const ::list::ListTreeWidgetItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*>& QList<QTreeWidgetItem*>::operator<<(QTreeWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::tree_widget_item::TreeWidgetItem)
                                        -> &'l0 mut ::list::ListTreeWidgetItemMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_operator_shl_t(self as *mut ::list::ListTreeWidgetItemMutPtr, t as *const *mut ::tree_widget_item::TreeWidgetItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_pop_back(self as *mut ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_pop_front(self as *mut ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::prepend(QTreeWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_prepend(self as *mut ::list::ListTreeWidgetItemMutPtr,
                                                          t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::push_back(QTreeWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_push_back(self as *mut ::list::ListTreeWidgetItemMutPtr,
                                                            t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::push_front(QTreeWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_push_front(self as *mut ::list::ListTreeWidgetItemMutPtr,
                                                             t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```int QList<QTreeWidgetItem*>::removeAll(QTreeWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::tree_widget_item::TreeWidgetItem) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_removeAll(self as *mut ::list::ListTreeWidgetItemMutPtr,
                                                            t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_removeAt(self as *mut ::list::ListTreeWidgetItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_removeFirst(self as *mut ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_removeLast(self as *mut ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTreeWidgetItem*>::removeOne(QTreeWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::tree_widget_item::TreeWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_removeOne(self as *mut ::list::ListTreeWidgetItemMutPtr,
                                                            t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::replace(int i, QTreeWidgetItem* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::tree_widget_item::TreeWidgetItem) {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_replace(self as *mut ::list::ListTreeWidgetItemMutPtr,
                                                          i,
                                                          t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_reserve(self as *mut ::list::ListTreeWidgetItemMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTreeWidgetItem*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_size(self as *const ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTreeWidgetItem*>::startsWith(QTreeWidgetItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::tree_widget_item::TreeWidgetItem) -> bool {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_startsWith(self as *const ::list::ListTreeWidgetItemMutPtr,
                                                             t as *const *mut ::tree_widget_item::TreeWidgetItem)
  }

  /// C++ method: <span style='color: green;'>```QList<QTreeWidgetItem*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListTreeWidgetItemMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::swap(QList<QTreeWidgetItem*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTreeWidgetItem*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTreeWidgetItemMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QList<QTreeWidgetItem*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_takeAt(self as *mut ::list::ListTreeWidgetItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QList<QTreeWidgetItem*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_takeFirst(self as *mut ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QList<QTreeWidgetItem*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_takeLast(self as *mut ::list::ListTreeWidgetItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QList<QTreeWidgetItem*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::tree_widget_item::TreeWidgetItem {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_value_i(self as *const ::list::ListTreeWidgetItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QTreeWidgetItem* QList<QTreeWidgetItem*>::value(int i, QTreeWidgetItem* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::tree_widget_item::TreeWidgetItem)
                             -> *mut ::tree_widget_item::TreeWidgetItem {
    ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_value_i_defaultValue(self as *const ::list::ListTreeWidgetItemMutPtr, i, default_value as *const *mut ::tree_widget_item::TreeWidgetItem)
  }
}

impl Drop for ::list::ListTreeWidgetItemMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QTreeWidgetItem*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_destructor(self as *mut ::list::ListTreeWidgetItemMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QUndoStack*>```</span>
#[repr(C)]
pub struct ListUndoStackMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_UNDO_STACK_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListUndoStackMutPtr {
  unsafe fn new_uninitialized() -> ListUndoStackMutPtr {
    ListUndoStackMutPtr(::std::mem::uninitialized())
  }
}

impl ListUndoStackMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::append(const QList<QUndoStack*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListUndoStackMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QUndoStack_ptr_append_QList_QUndoStack_ptr(self as *mut ::list::ListUndoStackMutPtr,
                                                                           t as *const ::list::ListUndoStackMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::append(QUndoStack* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::undo_stack::UndoStack) {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_append_QUndoStack(self as *mut ::list::ListUndoStackMutPtr,
                                                               t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* const & QList<QUndoStack*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::undo_stack::UndoStack {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_at(self as *const ::list::ListUndoStackMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* const & QList<QUndoStack*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::undo_stack::UndoStack {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_back_const(self as *const ::list::ListUndoStackMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QUndoStack*& QList<QUndoStack*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::undo_stack::UndoStack {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_back(self as *mut ::list::ListUndoStackMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_clear(self as *mut ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* const & QList<QUndoStack*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::undo_stack::UndoStack {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_constFirst(self as *const ::list::ListUndoStackMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* const & QList<QUndoStack*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::undo_stack::UndoStack {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_constLast(self as *const ::list::ListUndoStackMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUndoStack*>::contains(QUndoStack* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::undo_stack::UndoStack) -> bool {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_contains(self as *const ::list::ListUndoStackMutPtr,
                                                      t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```int QList<QUndoStack*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_count_no_args(self as *const ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QUndoStack*>::count(QUndoStack* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::undo_stack::UndoStack) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_count_t(self as *const ::list::ListUndoStackMutPtr,
                                                     t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUndoStack*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_empty(self as *const ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUndoStack*>::endsWith(QUndoStack* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::undo_stack::UndoStack) -> bool {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_endsWith(self as *const ::list::ListUndoStackMutPtr,
                                                      t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* const & QList<QUndoStack*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::undo_stack::UndoStack {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_first_const(self as *const ::list::ListUndoStackMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QUndoStack*& QList<QUndoStack*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::undo_stack::UndoStack {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_first(self as *mut ::list::ListUndoStackMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* const & QList<QUndoStack*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::undo_stack::UndoStack {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_front_const(self as *const ::list::ListUndoStackMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QUndoStack*& QList<QUndoStack*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::undo_stack::UndoStack {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_front(self as *mut ::list::ListUndoStackMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QUndoStack*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::undo_stack::UndoStack) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QUndoStack*>::indexOf(QUndoStack* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::undo_stack::UndoStack, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QUndoStack*>::indexOf(QUndoStack* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListUndoStackMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::insert(int i, QUndoStack* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::undo_stack::UndoStack) {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_insert(self as *mut ::list::ListUndoStackMutPtr,
                                                    i,
                                                    t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUndoStack*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_isEmpty(self as *const ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* const & QList<QUndoStack*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::undo_stack::UndoStack {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_last_const(self as *const ::list::ListUndoStackMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QUndoStack*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::undo_stack::UndoStack) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QUndoStack*>::lastIndexOf(QUndoStack* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::undo_stack::UndoStack, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QUndoStack*>::lastIndexOf(QUndoStack* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListUndoStackMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUndoStack*& QList<QUndoStack*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::undo_stack::UndoStack {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_last(self as *mut ::list::ListUndoStackMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QUndoStack*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_length(self as *const ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QUndoStack*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListUndoStackMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QUndoStack*> QList<QUndoStack*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListUndoStackMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QUndoStack*> QList<QUndoStack*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListUndoStackMutPtr
    where Args: overloading::ListUndoStackMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_move(self as *mut ::list::ListUndoStackMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QUndoStack*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListUndoStackMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QUndoStack*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListUndoStackMutPtr) -> ::list::ListUndoStackMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QUndoStack*>::QList(const QList<QUndoStack*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListUndoStackMutPtr
    where Args: overloading::ListUndoStackMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QUndoStack*> QList<QUndoStack*>::operator+(const QList<QUndoStack*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListUndoStackMutPtr) -> ::list::ListUndoStackMutPtr {
    {
      let mut object: ::list::ListUndoStackMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QUndoStack_ptr_operator_add_to_output(self as *const ::list::ListUndoStackMutPtr,
                                                                        l as *const ::list::ListUndoStackMutPtr,
                                                                        &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QUndoStack*>& QList<QUndoStack*>::operator+=(const QList<QUndoStack*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListUndoStackMutPtr)
                                 -> &'l0 mut ::list::ListUndoStackMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QUndoStack_ptr_operator_add_assign_l(self as *mut ::list::ListUndoStackMutPtr,
                                                                       l as *const ::list::ListUndoStackMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QUndoStack*>& QList<QUndoStack*>::operator+=(QUndoStack* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::undo_stack::UndoStack)
                                               -> &'l0 mut ::list::ListUndoStackMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QList_QUndoStack_ptr_operator_add_assign_t(self as *mut ::list::ListUndoStackMutPtr,
                                                                     t as *const *mut ::undo_stack::UndoStack);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QUndoStack*>& QList<QUndoStack*>::operator=(const QList<QUndoStack*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListUndoStackMutPtr)
                             -> &'l0 mut ::list::ListUndoStackMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QUndoStack_ptr_operator_assign(self as *mut ::list::ListUndoStackMutPtr,
                                                                 l as *const ::list::ListUndoStackMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUndoStack*>::operator==(const QList<QUndoStack*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListUndoStackMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QUndoStack_ptr_operator_eq(self as *const ::list::ListUndoStackMutPtr,
                                                           l as *const ::list::ListUndoStackMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* const & QList<QUndoStack*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::undo_stack::UndoStack {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QUndoStack_ptr_operator_index_const(self as *const ::list::ListUndoStackMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QUndoStack*& QList<QUndoStack*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::undo_stack::UndoStack {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_operator_index(self as *mut ::list::ListUndoStackMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUndoStack*>::operator!=(const QList<QUndoStack*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListUndoStackMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QUndoStack_ptr_operator_neq(self as *const ::list::ListUndoStackMutPtr,
                                                            l as *const ::list::ListUndoStackMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QUndoStack*>& QList<QUndoStack*>::operator<<(const QList<QUndoStack*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListUndoStackMutPtr) -> &'l0 mut ::list::ListUndoStackMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QList_QUndoStack_ptr_operator_shl_l(self as *mut ::list::ListUndoStackMutPtr,
                                                              l as *const ::list::ListUndoStackMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QUndoStack*>& QList<QUndoStack*>::operator<<(QUndoStack* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::undo_stack::UndoStack)
                                        -> &'l0 mut ::list::ListUndoStackMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QList_QUndoStack_ptr_operator_shl_t(self as *mut ::list::ListUndoStackMutPtr,
                                                              t as *const *mut ::undo_stack::UndoStack);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_pop_back(self as *mut ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_pop_front(self as *mut ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::prepend(QUndoStack* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::undo_stack::UndoStack) {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_prepend(self as *mut ::list::ListUndoStackMutPtr,
                                                     t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::push_back(QUndoStack* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::undo_stack::UndoStack) {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_push_back(self as *mut ::list::ListUndoStackMutPtr,
                                                       t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::push_front(QUndoStack* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::undo_stack::UndoStack) {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_push_front(self as *mut ::list::ListUndoStackMutPtr,
                                                        t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```int QList<QUndoStack*>::removeAll(QUndoStack* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::undo_stack::UndoStack) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_removeAll(self as *mut ::list::ListUndoStackMutPtr,
                                                       t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_removeAt(self as *mut ::list::ListUndoStackMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_removeFirst(self as *mut ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_removeLast(self as *mut ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUndoStack*>::removeOne(QUndoStack* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::undo_stack::UndoStack) -> bool {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_removeOne(self as *mut ::list::ListUndoStackMutPtr,
                                                       t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::replace(int i, QUndoStack* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::undo_stack::UndoStack) {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_replace(self as *mut ::list::ListUndoStackMutPtr,
                                                     i,
                                                     t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_reserve(self as *mut ::list::ListUndoStackMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QUndoStack*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_size(self as *const ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QUndoStack*>::startsWith(QUndoStack* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::undo_stack::UndoStack) -> bool {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_startsWith(self as *const ::list::ListUndoStackMutPtr,
                                                        t as *const *mut ::undo_stack::UndoStack)
  }

  /// C++ method: <span style='color: green;'>```QList<QUndoStack*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListUndoStackMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::swap(QList<QUndoStack*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QUndoStack*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListUndoStackMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QUndoStack* QList<QUndoStack*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::undo_stack::UndoStack {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_takeAt(self as *mut ::list::ListUndoStackMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* QList<QUndoStack*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::undo_stack::UndoStack {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_takeFirst(self as *mut ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* QList<QUndoStack*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::undo_stack::UndoStack {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_takeLast(self as *mut ::list::ListUndoStackMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* QList<QUndoStack*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::undo_stack::UndoStack {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_value_i(self as *const ::list::ListUndoStackMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QUndoStack* QList<QUndoStack*>::value(int i, QUndoStack* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::undo_stack::UndoStack)
                             -> *mut ::undo_stack::UndoStack {
    ::ffi::qt_widgets_c_QList_QUndoStack_ptr_value_i_defaultValue(self as *const ::list::ListUndoStackMutPtr, i, default_value as *const *mut ::undo_stack::UndoStack)
  }
}

impl Drop for ::list::ListUndoStackMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QUndoStack*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QUndoStack_ptr_destructor(self as *mut ::list::ListUndoStackMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QWidget*>```</span>
#[repr(C)]
pub struct ListWidgetMutPtr([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_WIDGET_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListWidgetMutPtr {
  unsafe fn new_uninitialized() -> ListWidgetMutPtr {
    ListWidgetMutPtr(::std::mem::uninitialized())
  }
}

impl ListWidgetMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::append(const QList<QWidget*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListWidgetMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWidget_ptr_append_QList_QWidget_ptr(self as *mut ::list::ListWidgetMutPtr,
                                                                     t as *const ::list::ListWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::append(QWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_append_QWidget(self as *mut ::list::ListWidgetMutPtr,
                                                         t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```QWidget* const & QList<QWidget*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_at(self as *const ::list::ListWidgetMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWidget* const & QList<QWidget*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_back_const(self as *const ::list::ListWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWidget*& QList<QWidget*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_back(self as *mut ::list::ListWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_clear(self as *mut ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* const & QList<QWidget*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_constFirst(self as *const ::list::ListWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWidget* const & QList<QWidget*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_constLast(self as *const ::list::ListWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWidget*>::contains(QWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::widget::Widget) -> bool {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_contains(self as *const ::list::ListWidgetMutPtr,
                                                   t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```int QList<QWidget*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_count_no_args(self as *const ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QWidget*>::count(QWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_count_t(self as *const ::list::ListWidgetMutPtr,
                                                  t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWidget*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_empty(self as *const ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWidget*>::endsWith(QWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::widget::Widget) -> bool {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_endsWith(self as *const ::list::ListWidgetMutPtr,
                                                   t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```QWidget* const & QList<QWidget*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_first_const(self as *const ::list::ListWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWidget*& QList<QWidget*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_first(self as *mut ::list::ListWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWidget* const & QList<QWidget*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_front_const(self as *const ::list::ListWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWidget*& QList<QWidget*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_front(self as *mut ::list::ListWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWidget*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::widget::Widget) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWidget*>::indexOf(QWidget* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::widget::Widget, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWidget*>::indexOf(QWidget* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListWidgetMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::insert(int i, QWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_insert(self as *mut ::list::ListWidgetMutPtr,
                                                 i,
                                                 t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWidget*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_isEmpty(self as *const ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* const & QList<QWidget*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_last_const(self as *const ::list::ListWidgetMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWidget*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::widget::Widget) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWidget*>::lastIndexOf(QWidget* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::widget::Widget, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWidget*>::lastIndexOf(QWidget* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListWidgetMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget*& QList<QWidget*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::widget::Widget {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_last(self as *mut ::list::ListWidgetMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QWidget*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_length(self as *const ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QWidget*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QWidget*> QList<QWidget*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QWidget*> QList<QWidget*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListWidgetMutPtr
    where Args: overloading::ListWidgetMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_move(self as *mut ::list::ListWidgetMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QWidget*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QWidget*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListWidgetMutPtr) -> ::list::ListWidgetMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QWidget*>::QList(const QList<QWidget*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListWidgetMutPtr
    where Args: overloading::ListWidgetMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QWidget*> QList<QWidget*>::operator+(const QList<QWidget*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListWidgetMutPtr) -> ::list::ListWidgetMutPtr {
    {
      let mut object: ::list::ListWidgetMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QWidget_ptr_operator_add_to_output(self as *const ::list::ListWidgetMutPtr,
                                                                     l as *const ::list::ListWidgetMutPtr,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QWidget*>& QList<QWidget*>::operator+=(const QList<QWidget*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListWidgetMutPtr) -> &'l0 mut ::list::ListWidgetMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QWidget_ptr_operator_add_assign_l(self as *mut ::list::ListWidgetMutPtr,
                                                                    l as *const ::list::ListWidgetMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWidget*>& QList<QWidget*>::operator+=(QWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::widget::Widget)
                                               -> &'l0 mut ::list::ListWidgetMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QList_QWidget_ptr_operator_add_assign_t(self as *mut ::list::ListWidgetMutPtr,
                                                                  t as *const *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWidget*>& QList<QWidget*>::operator=(const QList<QWidget*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListWidgetMutPtr) -> &'l0 mut ::list::ListWidgetMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QList_QWidget_ptr_operator_assign(self as *mut ::list::ListWidgetMutPtr,
                                                            l as *const ::list::ListWidgetMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWidget*>::operator==(const QList<QWidget*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListWidgetMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWidget_ptr_operator_eq(self as *const ::list::ListWidgetMutPtr,
                                                        l as *const ::list::ListWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QWidget* const & QList<QWidget*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_operator_index_const(self as *const ::list::ListWidgetMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWidget*& QList<QWidget*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::widget::Widget {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_operator_index(self as *mut ::list::ListWidgetMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWidget*>::operator!=(const QList<QWidget*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListWidgetMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWidget_ptr_operator_neq(self as *const ::list::ListWidgetMutPtr,
                                                         l as *const ::list::ListWidgetMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QWidget*>& QList<QWidget*>::operator<<(const QList<QWidget*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListWidgetMutPtr) -> &'l0 mut ::list::ListWidgetMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QList_QWidget_ptr_operator_shl_l(self as *mut ::list::ListWidgetMutPtr,
                                                           l as *const ::list::ListWidgetMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWidget*>& QList<QWidget*>::operator<<(QWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::widget::Widget)
                                        -> &'l0 mut ::list::ListWidgetMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QList_QWidget_ptr_operator_shl_t(self as *mut ::list::ListWidgetMutPtr,
                                                                          t as *const *mut ::widget::Widget);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_pop_back(self as *mut ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_pop_front(self as *mut ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::prepend(QWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_prepend(self as *mut ::list::ListWidgetMutPtr,
                                                  t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::push_back(QWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_push_back(self as *mut ::list::ListWidgetMutPtr,
                                                    t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::push_front(QWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_push_front(self as *mut ::list::ListWidgetMutPtr,
                                                     t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```int QList<QWidget*>::removeAll(QWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::widget::Widget) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_removeAll(self as *mut ::list::ListWidgetMutPtr,
                                                    t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_removeAt(self as *mut ::list::ListWidgetMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_removeFirst(self as *mut ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_removeLast(self as *mut ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWidget*>::removeOne(QWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::widget::Widget) -> bool {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_removeOne(self as *mut ::list::ListWidgetMutPtr,
                                                    t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::replace(int i, QWidget* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::widget::Widget) {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_replace(self as *mut ::list::ListWidgetMutPtr,
                                                  i,
                                                  t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_reserve(self as *mut ::list::ListWidgetMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QWidget*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_size(self as *const ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWidget*>::startsWith(QWidget* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::widget::Widget) -> bool {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_startsWith(self as *const ::list::ListWidgetMutPtr,
                                                     t as *const *mut ::widget::Widget)
  }

  /// C++ method: <span style='color: green;'>```QList<QWidget*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListWidgetMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::swap(QList<QWidget*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QWidget*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListWidgetMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWidget* QList<QWidget*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_takeAt(self as *mut ::list::ListWidgetMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QList<QWidget*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_takeFirst(self as *mut ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QList<QWidget*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_takeLast(self as *mut ::list::ListWidgetMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QList<QWidget*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::widget::Widget {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_value_i(self as *const ::list::ListWidgetMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QWidget* QList<QWidget*>::value(int i, QWidget* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self, i: ::libc::c_int, default_value: &*mut ::widget::Widget) -> *mut ::widget::Widget {
    ::ffi::qt_widgets_c_QList_QWidget_ptr_value_i_defaultValue(self as *const ::list::ListWidgetMutPtr,
                                                               i,
                                                               default_value as *const *mut ::widget::Widget)
  }
}

impl Drop for ::list::ListWidgetMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QWidget*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_destructor(self as *mut ::list::ListWidgetMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QWizard::WizardButton>```</span>
#[repr(C)]
pub struct ListWizardWizardButton([u8; ::type_sizes::QT_WIDGETS_LIST_LIST_WIZARD_WIZARD_BUTTON]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListWizardWizardButton {
  unsafe fn new_uninitialized() -> ListWizardWizardButton {
    ListWizardWizardButton(::std::mem::uninitialized())
  }
}

impl ListWizardWizardButton {
  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListWizardWizardButton) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::append(const QList<QWizard::WizardButton>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::wizard::WizardButton) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::append(const QWizard::WizardButton& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListWizardWizardButtonAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QWizard::WizardButton& QList<QWizard::WizardButton>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::wizard::WizardButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_at(self as *const ::list::ListWizardWizardButton, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QWizard::WizardButton& QList<QWizard::WizardButton>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::wizard::WizardButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_back_const(self as *const ::list::ListWizardWizardButton)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWizard::WizardButton& QList<QWizard::WizardButton>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::wizard::WizardButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_back(self as *mut ::list::ListWizardWizardButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_clear(self as *mut ::list::ListWizardWizardButton) }
  }

  /// C++ method: <span style='color: green;'>```const QWizard::WizardButton& QList<QWizard::WizardButton>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::wizard::WizardButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_constFirst(self as *const ::list::ListWizardWizardButton)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QWizard::WizardButton& QList<QWizard::WizardButton>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::wizard::WizardButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_constLast(self as *const ::list::ListWizardWizardButton)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWizard::WizardButton>::contains(const QWizard::WizardButton& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::wizard::WizardButton) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_contains(self as *const ::list::ListWizardWizardButton,
                                                              t as *const ::wizard::WizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWizard::WizardButton>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::wizard::WizardButton) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWizard::WizardButton>::count(const QWizard::WizardButton& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListWizardWizardButtonCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QWizard::WizardButton>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_empty(self as *const ::list::ListWizardWizardButton) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWizard::WizardButton>::endsWith(const QWizard::WizardButton& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::wizard::WizardButton) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_endsWith(self as *const ::list::ListWizardWizardButton,
                                                              t as *const ::wizard::WizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```const QWizard::WizardButton& QList<QWizard::WizardButton>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::wizard::WizardButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_first_const(self as *const ::list::ListWizardWizardButton)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWizard::WizardButton& QList<QWizard::WizardButton>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::wizard::WizardButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_first(self as *mut ::list::ListWizardWizardButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QWizard::WizardButton& QList<QWizard::WizardButton>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::wizard::WizardButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_front_const(self as *const ::list::ListWizardWizardButton)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWizard::WizardButton& QList<QWizard::WizardButton>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::wizard::WizardButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_front(self as *mut ::list::ListWizardWizardButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::wizard::WizardButton) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWizard::WizardButton>::indexOf(const QWizard::WizardButton& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::wizard::WizardButton, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWizard::WizardButton>::indexOf(const QWizard::WizardButton& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListWizardWizardButtonIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::insert(int i, const QWizard::WizardButton& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::wizard::WizardButton) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_insert(self as *mut ::list::ListWizardWizardButton,
                                                            i,
                                                            t as *const ::wizard::WizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWizard::WizardButton>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_isEmpty(self as *const ::list::ListWizardWizardButton) }
  }

  /// C++ method: <span style='color: green;'>```const QWizard::WizardButton& QList<QWizard::WizardButton>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::wizard::WizardButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_last_const(self as *const ::list::ListWizardWizardButton)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::wizard::WizardButton) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWizard::WizardButton>::lastIndexOf(const QWizard::WizardButton& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::wizard::WizardButton, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWizard::WizardButton>::lastIndexOf(const QWizard::WizardButton& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListWizardWizardButtonLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWizard::WizardButton& QList<QWizard::WizardButton>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::wizard::WizardButton {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_last(self as *mut ::list::ListWizardWizardButton) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QWizard::WizardButton>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_length(self as *const ::list::ListWizardWizardButton) }
  }

  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListWizardWizardButton```<br>
  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton> QList<QWizard::WizardButton>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListWizardWizardButton```<br>
  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton> QList<QWizard::WizardButton>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListWizardWizardButton
    where Args: overloading::ListWizardWizardButtonMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_move(self as *mut ::list::ListWizardWizardButton, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListWizardWizardButton```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QWizard::WizardButton>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListWizardWizardButton) -> ::list::ListWizardWizardButton```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QWizard::WizardButton>::QList(const QList<QWizard::WizardButton>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListWizardWizardButton
    where Args: overloading::ListWizardWizardButtonNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton> QList<QWizard::WizardButton>::operator+(const QList<QWizard::WizardButton>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListWizardWizardButton) -> ::list::ListWizardWizardButton {
    {
      let mut object: ::list::ListWizardWizardButton =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_operator_add_to_output(self as *const ::list::ListWizardWizardButton, l as *const ::list::ListWizardWizardButton, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListWizardWizardButton) -> &'l0 mut ::list::ListWizardWizardButton```<br>
  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>& QList<QWizard::WizardButton>::operator+=(const QList<QWizard::WizardButton>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::wizard::WizardButton) -> &'l0 mut ::list::ListWizardWizardButton```<br>
  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>& QList<QWizard::WizardButton>::operator+=(const QWizard::WizardButton& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListWizardWizardButton
    where Args: overloading::ListWizardWizardButtonOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>& QList<QWizard::WizardButton>::operator=(const QList<QWizard::WizardButton>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListWizardWizardButton)
                             -> &'l0 mut ::list::ListWizardWizardButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_operator_assign(self as *mut ::list::ListWizardWizardButton,
                                                                       l as *const ::list::ListWizardWizardButton)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWizard::WizardButton>::operator==(const QList<QWizard::WizardButton>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListWizardWizardButton) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_operator_eq(self as *const ::list::ListWizardWizardButton,
                                                                 l as *const ::list::ListWizardWizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```const QWizard::WizardButton& QList<QWizard::WizardButton>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::wizard::WizardButton {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_operator_index_const(self as *const ::list::ListWizardWizardButton, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWizard::WizardButton& QList<QWizard::WizardButton>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::wizard::WizardButton {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_operator_index(self as *mut ::list::ListWizardWizardButton, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWizard::WizardButton>::operator!=(const QList<QWizard::WizardButton>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListWizardWizardButton) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_operator_neq(self as *const ::list::ListWizardWizardButton,
                                                                  l as *const ::list::ListWizardWizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListWizardWizardButton) -> &'l0 mut ::list::ListWizardWizardButton```<br>
  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>& QList<QWizard::WizardButton>::operator<<(const QList<QWizard::WizardButton>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::wizard::WizardButton) -> &'l0 mut ::list::ListWizardWizardButton```<br>
  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>& QList<QWizard::WizardButton>::operator<<(const QWizard::WizardButton& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListWizardWizardButton
    where Args: overloading::ListWizardWizardButtonOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_pop_back(self as *mut ::list::ListWizardWizardButton) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_pop_front(self as *mut ::list::ListWizardWizardButton) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::prepend(const QWizard::WizardButton& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::wizard::WizardButton) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_prepend(self as *mut ::list::ListWizardWizardButton,
                                                             t as *const ::wizard::WizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::push_back(const QWizard::WizardButton& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::wizard::WizardButton) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_push_back(self as *mut ::list::ListWizardWizardButton,
                                                               t as *const ::wizard::WizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::push_front(const QWizard::WizardButton& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::wizard::WizardButton) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_push_front(self as *mut ::list::ListWizardWizardButton,
                                                                t as *const ::wizard::WizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QWizard::WizardButton>::removeAll(const QWizard::WizardButton& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::wizard::WizardButton) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_removeAll(self as *mut ::list::ListWizardWizardButton,
                                                               t as *const ::wizard::WizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_removeAt(self as *mut ::list::ListWizardWizardButton, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_removeFirst(self as *mut ::list::ListWizardWizardButton) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_removeLast(self as *mut ::list::ListWizardWizardButton) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWizard::WizardButton>::removeOne(const QWizard::WizardButton& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::wizard::WizardButton) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_removeOne(self as *mut ::list::ListWizardWizardButton,
                                                               t as *const ::wizard::WizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::replace(int i, const QWizard::WizardButton& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::wizard::WizardButton) {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_replace(self as *mut ::list::ListWizardWizardButton,
                                                             i,
                                                             t as *const ::wizard::WizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_reserve(self as *mut ::list::ListWizardWizardButton, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QWizard::WizardButton>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_size(self as *const ::list::ListWizardWizardButton) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWizard::WizardButton>::startsWith(const QWizard::WizardButton& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::wizard::WizardButton) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QList_QWizard_WizardButton_startsWith(self as *const ::list::ListWizardWizardButton,
                                                                t as *const ::wizard::WizardButton)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListWizardWizardButton) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::swap(QList<QWizard::WizardButton>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QWizard::WizardButton>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListWizardWizardButtonSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWizard::WizardButton QList<QWizard::WizardButton>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::wizard::WizardButton {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_takeAt(self as *mut ::list::ListWizardWizardButton, i) }
  }

  /// C++ method: <span style='color: green;'>```QWizard::WizardButton QList<QWizard::WizardButton>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::wizard::WizardButton {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_takeFirst(self as *mut ::list::ListWizardWizardButton) }
  }

  /// C++ method: <span style='color: green;'>```QWizard::WizardButton QList<QWizard::WizardButton>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::wizard::WizardButton {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_takeLast(self as *mut ::list::ListWizardWizardButton) }
  }

  /// C++ method: <span style='color: green;'>```QList<QWizard::WizardButton>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::wizard::WizardButton```<br>
  /// C++ method: <span style='color: green;'>```QWizard::WizardButton QList<QWizard::WizardButton>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::wizard::WizardButton)) -> ::wizard::WizardButton```<br>
  /// C++ method: <span style='color: green;'>```QWizard::WizardButton QList<QWizard::WizardButton>::value(int i, const QWizard::WizardButton& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::wizard::WizardButton
    where Args: overloading::ListWizardWizardButtonValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListWizardWizardButton {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QWizard::WizardButton>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_destructor(self as *mut ::list::ListWizardWizardButton) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ListAbstractButtonMutPtr::index_of](../struct.ListAbstractButtonMutPtr.html#method.index_of) method.
  pub trait ListAbstractButtonMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractButtonMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListAbstractButtonMutPtrIndexOfArgs<'largs> for &'largs *mut ::abstract_button::AbstractButton {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractButtonMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_indexOf_t(original_self as *const ::list::ListAbstractButtonMutPtr, t as *const *mut ::abstract_button::AbstractButton)
    }
  }
  impl<'largs> ListAbstractButtonMutPtrIndexOfArgs<'largs>
    for (&'largs *mut ::abstract_button::AbstractButton, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractButtonMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_indexOf_t_from(original_self as *const ::list::ListAbstractButtonMutPtr, t as *const *mut ::abstract_button::AbstractButton, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractButtonMutPtr::last_index_of](../struct.ListAbstractButtonMutPtr.html#method.last_index_of) method.
  pub trait ListAbstractButtonMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractButtonMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListAbstractButtonMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::abstract_button::AbstractButton {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractButtonMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_lastIndexOf_t(original_self as *const ::list::ListAbstractButtonMutPtr, t as *const *mut ::abstract_button::AbstractButton)
    }
  }
  impl<'largs> ListAbstractButtonMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::abstract_button::AbstractButton, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListAbstractButtonMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_lastIndexOf_t_from(original_self as *const ::list::ListAbstractButtonMutPtr, t as *const *mut ::abstract_button::AbstractButton, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractButtonMutPtr::mid](../struct.ListAbstractButtonMutPtr.html#method.mid) method.
  pub trait ListAbstractButtonMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListAbstractButtonMutPtr) -> ::list::ListAbstractButtonMutPtr;
  }
  impl<'largs> ListAbstractButtonMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListAbstractButtonMutPtr) -> ::list::ListAbstractButtonMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListAbstractButtonMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_mid_to_output_pos(original_self as *const ::list::ListAbstractButtonMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListAbstractButtonMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListAbstractButtonMutPtr) -> ::list::ListAbstractButtonMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListAbstractButtonMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_mid_to_output_pos_length(original_self as *const ::list::ListAbstractButtonMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractButtonMutPtr::new](../struct.ListAbstractButtonMutPtr.html#method.new) method.
  pub trait ListAbstractButtonMutPtrNewArgs {
    fn exec(self) -> ::list::ListAbstractButtonMutPtr;
  }
  impl<'a> ListAbstractButtonMutPtrNewArgs for &'a ::list::ListAbstractButtonMutPtr {
    fn exec(self) -> ::list::ListAbstractButtonMutPtr {
      let l = self;
      {
        let mut object: ::list::ListAbstractButtonMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_constructor_l(l as *const ::list::ListAbstractButtonMutPtr,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl ListAbstractButtonMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListAbstractButtonMutPtr {

      {
        let mut object: ::list::ListAbstractButtonMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAbstractButtonMutPtr::swap](../struct.ListAbstractButtonMutPtr.html#method.swap) method.
  pub trait ListAbstractButtonMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractButtonMutPtr) -> ();
  }
  impl<'largs> ListAbstractButtonMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractButtonMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_swap_i_j(original_self as *mut ::list::ListAbstractButtonMutPtr,
                                                               i,
                                                               j)
      }
    }
  }
  impl<'largs> ListAbstractButtonMutPtrSwapArgs<'largs> for &'largs mut ::list::ListAbstractButtonMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListAbstractButtonMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QAbstractButton_ptr_swap_other(original_self as *mut ::list::ListAbstractButtonMutPtr, other as *mut ::list::ListAbstractButtonMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListActionMutPtr::index_of](../struct.ListActionMutPtr.html#method.index_of) method.
  pub trait ListActionMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListActionMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListActionMutPtrIndexOfArgs<'largs> for &'largs *mut ::action::Action {
    unsafe fn exec(self, original_self: &'largs ::list::ListActionMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QAction_ptr_indexOf_t(original_self as *const ::list::ListActionMutPtr,
                                                      t as *const *mut ::action::Action)
    }
  }
  impl<'largs> ListActionMutPtrIndexOfArgs<'largs> for (&'largs *mut ::action::Action, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListActionMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QAction_ptr_indexOf_t_from(original_self as *const ::list::ListActionMutPtr,
                                                           t as *const *mut ::action::Action,
                                                           from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListActionMutPtr::last_index_of](../struct.ListActionMutPtr.html#method.last_index_of) method.
  pub trait ListActionMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListActionMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListActionMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::action::Action {
    unsafe fn exec(self, original_self: &'largs ::list::ListActionMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QAction_ptr_lastIndexOf_t(original_self as *const ::list::ListActionMutPtr,
                                                          t as *const *mut ::action::Action)
    }
  }
  impl<'largs> ListActionMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::action::Action, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListActionMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QAction_ptr_lastIndexOf_t_from(original_self as *const ::list::ListActionMutPtr,
                                                               t as *const *mut ::action::Action,
                                                               from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListActionMutPtr::mid](../struct.ListActionMutPtr.html#method.mid) method.
  pub trait ListActionMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListActionMutPtr) -> ::list::ListActionMutPtr;
  }
  impl<'largs> ListActionMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListActionMutPtr) -> ::list::ListActionMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListActionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QAction_ptr_mid_to_output_pos(original_self as *const ::list::ListActionMutPtr,
                                                                  pos,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListActionMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListActionMutPtr) -> ::list::ListActionMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListActionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QAction_ptr_mid_to_output_pos_length(original_self as *const ::list::ListActionMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListActionMutPtr::new](../struct.ListActionMutPtr.html#method.new) method.
  pub trait ListActionMutPtrNewArgs {
    fn exec(self) -> ::list::ListActionMutPtr;
  }
  impl<'a> ListActionMutPtrNewArgs for &'a ::list::ListActionMutPtr {
    fn exec(self) -> ::list::ListActionMutPtr {
      let l = self;
      {
        let mut object: ::list::ListActionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QAction_ptr_constructor_l(l as *const ::list::ListActionMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListActionMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListActionMutPtr {

      {
        let mut object: ::list::ListActionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QAction_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListActionMutPtr::swap](../struct.ListActionMutPtr.html#method.swap) method.
  pub trait ListActionMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListActionMutPtr) -> ();
  }
  impl<'largs> ListActionMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListActionMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QAction_ptr_swap_i_j(original_self as *mut ::list::ListActionMutPtr, i, j) }
    }
  }
  impl<'largs> ListActionMutPtrSwapArgs<'largs> for &'largs mut ::list::ListActionMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListActionMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QAction_ptr_swap_other(original_self as *mut ::list::ListActionMutPtr,
                                                         other as *mut ::list::ListActionMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListDockWidgetMutPtr::index_of](../struct.ListDockWidgetMutPtr.html#method.index_of) method.
  pub trait ListDockWidgetMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListDockWidgetMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListDockWidgetMutPtrIndexOfArgs<'largs> for &'largs *mut ::dock_widget::DockWidget {
    unsafe fn exec(self, original_self: &'largs ::list::ListDockWidgetMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QDockWidget_ptr_indexOf_t(original_self as *const ::list::ListDockWidgetMutPtr,
                                                          t as *const *mut ::dock_widget::DockWidget)
    }
  }
  impl<'largs> ListDockWidgetMutPtrIndexOfArgs<'largs> for (&'largs *mut ::dock_widget::DockWidget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListDockWidgetMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QDockWidget_ptr_indexOf_t_from(original_self as *const ::list::ListDockWidgetMutPtr,
                                                               t as *const *mut ::dock_widget::DockWidget,
                                                               from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListDockWidgetMutPtr::last_index_of](../struct.ListDockWidgetMutPtr.html#method.last_index_of) method.
  pub trait ListDockWidgetMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListDockWidgetMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListDockWidgetMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::dock_widget::DockWidget {
    unsafe fn exec(self, original_self: &'largs ::list::ListDockWidgetMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QDockWidget_ptr_lastIndexOf_t(original_self as *const ::list::ListDockWidgetMutPtr,
                                                              t as *const *mut ::dock_widget::DockWidget)
    }
  }
  impl<'largs> ListDockWidgetMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::dock_widget::DockWidget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListDockWidgetMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QDockWidget_ptr_lastIndexOf_t_from(original_self as *const ::list::ListDockWidgetMutPtr, t as *const *mut ::dock_widget::DockWidget, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListDockWidgetMutPtr::mid](../struct.ListDockWidgetMutPtr.html#method.mid) method.
  pub trait ListDockWidgetMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListDockWidgetMutPtr) -> ::list::ListDockWidgetMutPtr;
  }
  impl<'largs> ListDockWidgetMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListDockWidgetMutPtr) -> ::list::ListDockWidgetMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListDockWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QDockWidget_ptr_mid_to_output_pos(original_self as *const ::list::ListDockWidgetMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListDockWidgetMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListDockWidgetMutPtr) -> ::list::ListDockWidgetMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListDockWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QDockWidget_ptr_mid_to_output_pos_length(original_self as *const ::list::ListDockWidgetMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListDockWidgetMutPtr::new](../struct.ListDockWidgetMutPtr.html#method.new) method.
  pub trait ListDockWidgetMutPtrNewArgs {
    fn exec(self) -> ::list::ListDockWidgetMutPtr;
  }
  impl<'a> ListDockWidgetMutPtrNewArgs for &'a ::list::ListDockWidgetMutPtr {
    fn exec(self) -> ::list::ListDockWidgetMutPtr {
      let l = self;
      {
        let mut object: ::list::ListDockWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QDockWidget_ptr_constructor_l(l as *const ::list::ListDockWidgetMutPtr,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl ListDockWidgetMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListDockWidgetMutPtr {

      {
        let mut object: ::list::ListDockWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QDockWidget_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListDockWidgetMutPtr::swap](../struct.ListDockWidgetMutPtr.html#method.swap) method.
  pub trait ListDockWidgetMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListDockWidgetMutPtr) -> ();
  }
  impl<'largs> ListDockWidgetMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListDockWidgetMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QDockWidget_ptr_swap_i_j(original_self as *mut ::list::ListDockWidgetMutPtr, i, j)
      }
    }
  }
  impl<'largs> ListDockWidgetMutPtrSwapArgs<'largs> for &'largs mut ::list::ListDockWidgetMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListDockWidgetMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QDockWidget_ptr_swap_other(original_self as *mut ::list::ListDockWidgetMutPtr,
                                                             other as *mut ::list::ListDockWidgetMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGestureMutPtr::index_of](../struct.ListGestureMutPtr.html#method.index_of) method.
  pub trait ListGestureMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListGestureMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListGestureMutPtrIndexOfArgs<'largs> for &'largs *mut ::gesture::Gesture {
    unsafe fn exec(self, original_self: &'largs ::list::ListGestureMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QGesture_ptr_indexOf_t(original_self as *const ::list::ListGestureMutPtr,
                                                       t as *const *mut ::gesture::Gesture)
    }
  }
  impl<'largs> ListGestureMutPtrIndexOfArgs<'largs> for (&'largs *mut ::gesture::Gesture, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListGestureMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QGesture_ptr_indexOf_t_from(original_self as *const ::list::ListGestureMutPtr,
                                                            t as *const *mut ::gesture::Gesture,
                                                            from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListGestureMutPtr::last_index_of](../struct.ListGestureMutPtr.html#method.last_index_of) method.
  pub trait ListGestureMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListGestureMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListGestureMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::gesture::Gesture {
    unsafe fn exec(self, original_self: &'largs ::list::ListGestureMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QGesture_ptr_lastIndexOf_t(original_self as *const ::list::ListGestureMutPtr,
                                                           t as *const *mut ::gesture::Gesture)
    }
  }
  impl<'largs> ListGestureMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::gesture::Gesture, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListGestureMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QGesture_ptr_lastIndexOf_t_from(original_self as *const ::list::ListGestureMutPtr,
                                                                t as *const *mut ::gesture::Gesture,
                                                                from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListGestureMutPtr::mid](../struct.ListGestureMutPtr.html#method.mid) method.
  pub trait ListGestureMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListGestureMutPtr) -> ::list::ListGestureMutPtr;
  }
  impl<'largs> ListGestureMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListGestureMutPtr) -> ::list::ListGestureMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListGestureMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGesture_ptr_mid_to_output_pos(original_self as *const ::list::ListGestureMutPtr,
                                                                   pos,
                                                                   &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListGestureMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListGestureMutPtr) -> ::list::ListGestureMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListGestureMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGesture_ptr_mid_to_output_pos_length(original_self as *const ::list::ListGestureMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGestureMutPtr::new](../struct.ListGestureMutPtr.html#method.new) method.
  pub trait ListGestureMutPtrNewArgs {
    fn exec(self) -> ::list::ListGestureMutPtr;
  }
  impl<'a> ListGestureMutPtrNewArgs for &'a ::list::ListGestureMutPtr {
    fn exec(self) -> ::list::ListGestureMutPtr {
      let l = self;
      {
        let mut object: ::list::ListGestureMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGesture_ptr_constructor_l(l as *const ::list::ListGestureMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListGestureMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListGestureMutPtr {

      {
        let mut object: ::list::ListGestureMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGesture_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGestureMutPtr::swap](../struct.ListGestureMutPtr.html#method.swap) method.
  pub trait ListGestureMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListGestureMutPtr) -> ();
  }
  impl<'largs> ListGestureMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListGestureMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QGesture_ptr_swap_i_j(original_self as *mut ::list::ListGestureMutPtr, i, j) }
    }
  }
  impl<'largs> ListGestureMutPtrSwapArgs<'largs> for &'largs mut ::list::ListGestureMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListGestureMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QGesture_ptr_swap_other(original_self as *mut ::list::ListGestureMutPtr,
                                                          other as *mut ::list::ListGestureMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsItemMutPtr::index_of](../struct.ListGraphicsItemMutPtr.html#method.index_of) method.
  pub trait ListGraphicsItemMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsItemMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListGraphicsItemMutPtrIndexOfArgs<'largs> for &'largs *mut ::graphics_item::GraphicsItem {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsItemMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_indexOf_t(original_self as *const ::list::ListGraphicsItemMutPtr,
                                                            t as *const *mut ::graphics_item::GraphicsItem)
    }
  }
  impl<'largs> ListGraphicsItemMutPtrIndexOfArgs<'largs> for (&'largs *mut ::graphics_item::GraphicsItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsItemMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_indexOf_t_from(original_self as *const ::list::ListGraphicsItemMutPtr, t as *const *mut ::graphics_item::GraphicsItem, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsItemMutPtr::last_index_of](../struct.ListGraphicsItemMutPtr.html#method.last_index_of) method.
  pub trait ListGraphicsItemMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsItemMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListGraphicsItemMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::graphics_item::GraphicsItem {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsItemMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_lastIndexOf_t(original_self as *const ::list::ListGraphicsItemMutPtr, t as *const *mut ::graphics_item::GraphicsItem)
    }
  }
  impl<'largs> ListGraphicsItemMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::graphics_item::GraphicsItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsItemMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_lastIndexOf_t_from(original_self as *const ::list::ListGraphicsItemMutPtr, t as *const *mut ::graphics_item::GraphicsItem, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsItemMutPtr::mid](../struct.ListGraphicsItemMutPtr.html#method.mid) method.
  pub trait ListGraphicsItemMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListGraphicsItemMutPtr) -> ::list::ListGraphicsItemMutPtr;
  }
  impl<'largs> ListGraphicsItemMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListGraphicsItemMutPtr) -> ::list::ListGraphicsItemMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_mid_to_output_pos(original_self as *const ::list::ListGraphicsItemMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListGraphicsItemMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListGraphicsItemMutPtr) -> ::list::ListGraphicsItemMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_mid_to_output_pos_length(original_self as *const ::list::ListGraphicsItemMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsItemMutPtr::new](../struct.ListGraphicsItemMutPtr.html#method.new) method.
  pub trait ListGraphicsItemMutPtrNewArgs {
    fn exec(self) -> ::list::ListGraphicsItemMutPtr;
  }
  impl<'a> ListGraphicsItemMutPtrNewArgs for &'a ::list::ListGraphicsItemMutPtr {
    fn exec(self) -> ::list::ListGraphicsItemMutPtr {
      let l = self;
      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_constructor_l(l as *const ::list::ListGraphicsItemMutPtr,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl ListGraphicsItemMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListGraphicsItemMutPtr {

      {
        let mut object: ::list::ListGraphicsItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsItemMutPtr::swap](../struct.ListGraphicsItemMutPtr.html#method.swap) method.
  pub trait ListGraphicsItemMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsItemMutPtr) -> ();
  }
  impl<'largs> ListGraphicsItemMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsItemMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_swap_i_j(original_self as *mut ::list::ListGraphicsItemMutPtr, i, j)
      }
    }
  }
  impl<'largs> ListGraphicsItemMutPtrSwapArgs<'largs> for &'largs mut ::list::ListGraphicsItemMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsItemMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsItem_ptr_swap_other(original_self as *mut ::list::ListGraphicsItemMutPtr,
                                                               other as *mut ::list::ListGraphicsItemMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsTransformMutPtr::index_of](../struct.ListGraphicsTransformMutPtr.html#method.index_of) method.
  pub trait ListGraphicsTransformMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsTransformMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListGraphicsTransformMutPtrIndexOfArgs<'largs> for &'largs *mut ::graphics_transform::GraphicsTransform {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsTransformMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_indexOf_t(original_self as *const ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform)
    }
  }
  impl<'largs> ListGraphicsTransformMutPtrIndexOfArgs<'largs> for (&'largs *mut ::graphics_transform::GraphicsTransform,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsTransformMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_indexOf_t_from(original_self as *const ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform, from)
  }
}
  /// This trait represents a set of arguments accepted by [ListGraphicsTransformMutPtr::last_index_of](../struct.ListGraphicsTransformMutPtr.html#method.last_index_of) method.
  pub trait ListGraphicsTransformMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsTransformMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListGraphicsTransformMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::graphics_transform::GraphicsTransform {

  unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsTransformMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_lastIndexOf_t(original_self as *const ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform)
  }
}
  impl<'largs> ListGraphicsTransformMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::graphics_transform::GraphicsTransform,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsTransformMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_lastIndexOf_t_from(original_self as *const ::list::ListGraphicsTransformMutPtr, t as *const *mut ::graphics_transform::GraphicsTransform, from)
  }
}
  /// This trait represents a set of arguments accepted by [ListGraphicsTransformMutPtr::mid](../struct.ListGraphicsTransformMutPtr.html#method.mid) method.
  pub trait ListGraphicsTransformMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListGraphicsTransformMutPtr) -> ::list::ListGraphicsTransformMutPtr;
  }
  impl<'largs> ListGraphicsTransformMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListGraphicsTransformMutPtr) -> ::list::ListGraphicsTransformMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListGraphicsTransformMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_mid_to_output_pos(original_self as *const ::list::ListGraphicsTransformMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListGraphicsTransformMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListGraphicsTransformMutPtr) -> ::list::ListGraphicsTransformMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListGraphicsTransformMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_mid_to_output_pos_length(original_self as *const ::list::ListGraphicsTransformMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsTransformMutPtr::new](../struct.ListGraphicsTransformMutPtr.html#method.new) method.
  pub trait ListGraphicsTransformMutPtrNewArgs {
    fn exec(self) -> ::list::ListGraphicsTransformMutPtr;
  }
  impl<'a> ListGraphicsTransformMutPtrNewArgs for &'a ::list::ListGraphicsTransformMutPtr {
    fn exec(self) -> ::list::ListGraphicsTransformMutPtr {
      let l = self;
      {
        let mut object: ::list::ListGraphicsTransformMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_constructor_l(l as *const ::list::ListGraphicsTransformMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListGraphicsTransformMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListGraphicsTransformMutPtr {

      {
        let mut object: ::list::ListGraphicsTransformMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsTransformMutPtr::swap](../struct.ListGraphicsTransformMutPtr.html#method.swap) method.
  pub trait ListGraphicsTransformMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsTransformMutPtr) -> ();
  }
  impl<'largs> ListGraphicsTransformMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsTransformMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_swap_i_j(original_self as *mut ::list::ListGraphicsTransformMutPtr, i, j) }
    }
  }
  impl<'largs> ListGraphicsTransformMutPtrSwapArgs<'largs> for &'largs mut ::list::ListGraphicsTransformMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsTransformMutPtr) -> () {
      let other = self;
      unsafe { ::ffi::qt_widgets_c_QList_QGraphicsTransform_ptr_swap_other(original_self as *mut ::list::ListGraphicsTransformMutPtr, other as *mut ::list::ListGraphicsTransformMutPtr) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsViewMutPtr::index_of](../struct.ListGraphicsViewMutPtr.html#method.index_of) method.
  pub trait ListGraphicsViewMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsViewMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListGraphicsViewMutPtrIndexOfArgs<'largs> for &'largs *mut ::graphics_view::GraphicsView {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsViewMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_indexOf_t(original_self as *const ::list::ListGraphicsViewMutPtr,
                                                            t as *const *mut ::graphics_view::GraphicsView)
    }
  }
  impl<'largs> ListGraphicsViewMutPtrIndexOfArgs<'largs> for (&'largs *mut ::graphics_view::GraphicsView, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsViewMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_indexOf_t_from(original_self as *const ::list::ListGraphicsViewMutPtr, t as *const *mut ::graphics_view::GraphicsView, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsViewMutPtr::last_index_of](../struct.ListGraphicsViewMutPtr.html#method.last_index_of) method.
  pub trait ListGraphicsViewMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsViewMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListGraphicsViewMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::graphics_view::GraphicsView {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsViewMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_lastIndexOf_t(original_self as *const ::list::ListGraphicsViewMutPtr, t as *const *mut ::graphics_view::GraphicsView)
    }
  }
  impl<'largs> ListGraphicsViewMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::graphics_view::GraphicsView, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsViewMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_lastIndexOf_t_from(original_self as *const ::list::ListGraphicsViewMutPtr, t as *const *mut ::graphics_view::GraphicsView, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsViewMutPtr::mid](../struct.ListGraphicsViewMutPtr.html#method.mid) method.
  pub trait ListGraphicsViewMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListGraphicsViewMutPtr) -> ::list::ListGraphicsViewMutPtr;
  }
  impl<'largs> ListGraphicsViewMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListGraphicsViewMutPtr) -> ::list::ListGraphicsViewMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListGraphicsViewMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_mid_to_output_pos(original_self as *const ::list::ListGraphicsViewMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListGraphicsViewMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListGraphicsViewMutPtr) -> ::list::ListGraphicsViewMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListGraphicsViewMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_mid_to_output_pos_length(original_self as *const ::list::ListGraphicsViewMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsViewMutPtr::new](../struct.ListGraphicsViewMutPtr.html#method.new) method.
  pub trait ListGraphicsViewMutPtrNewArgs {
    fn exec(self) -> ::list::ListGraphicsViewMutPtr;
  }
  impl<'a> ListGraphicsViewMutPtrNewArgs for &'a ::list::ListGraphicsViewMutPtr {
    fn exec(self) -> ::list::ListGraphicsViewMutPtr {
      let l = self;
      {
        let mut object: ::list::ListGraphicsViewMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_constructor_l(l as *const ::list::ListGraphicsViewMutPtr,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl ListGraphicsViewMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListGraphicsViewMutPtr {

      {
        let mut object: ::list::ListGraphicsViewMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsViewMutPtr::swap](../struct.ListGraphicsViewMutPtr.html#method.swap) method.
  pub trait ListGraphicsViewMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsViewMutPtr) -> ();
  }
  impl<'largs> ListGraphicsViewMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsViewMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_swap_i_j(original_self as *mut ::list::ListGraphicsViewMutPtr, i, j)
      }
    }
  }
  impl<'largs> ListGraphicsViewMutPtrSwapArgs<'largs> for &'largs mut ::list::ListGraphicsViewMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsViewMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsView_ptr_swap_other(original_self as *mut ::list::ListGraphicsViewMutPtr,
                                                               other as *mut ::list::ListGraphicsViewMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsWidgetMutPtr::index_of](../struct.ListGraphicsWidgetMutPtr.html#method.index_of) method.
  pub trait ListGraphicsWidgetMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsWidgetMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListGraphicsWidgetMutPtrIndexOfArgs<'largs> for &'largs *mut ::graphics_widget::GraphicsWidget {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsWidgetMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_indexOf_t(original_self as *const ::list::ListGraphicsWidgetMutPtr, t as *const *mut ::graphics_widget::GraphicsWidget)
    }
  }
  impl<'largs> ListGraphicsWidgetMutPtrIndexOfArgs<'largs>
    for (&'largs *mut ::graphics_widget::GraphicsWidget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsWidgetMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_indexOf_t_from(original_self as *const ::list::ListGraphicsWidgetMutPtr, t as *const *mut ::graphics_widget::GraphicsWidget, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsWidgetMutPtr::last_index_of](../struct.ListGraphicsWidgetMutPtr.html#method.last_index_of) method.
  pub trait ListGraphicsWidgetMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsWidgetMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListGraphicsWidgetMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::graphics_widget::GraphicsWidget {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsWidgetMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_lastIndexOf_t(original_self as *const ::list::ListGraphicsWidgetMutPtr, t as *const *mut ::graphics_widget::GraphicsWidget)
    }
  }
  impl<'largs> ListGraphicsWidgetMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::graphics_widget::GraphicsWidget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListGraphicsWidgetMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_lastIndexOf_t_from(original_self as *const ::list::ListGraphicsWidgetMutPtr, t as *const *mut ::graphics_widget::GraphicsWidget, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsWidgetMutPtr::mid](../struct.ListGraphicsWidgetMutPtr.html#method.mid) method.
  pub trait ListGraphicsWidgetMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListGraphicsWidgetMutPtr) -> ::list::ListGraphicsWidgetMutPtr;
  }
  impl<'largs> ListGraphicsWidgetMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListGraphicsWidgetMutPtr) -> ::list::ListGraphicsWidgetMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListGraphicsWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_mid_to_output_pos(original_self as *const ::list::ListGraphicsWidgetMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListGraphicsWidgetMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListGraphicsWidgetMutPtr) -> ::list::ListGraphicsWidgetMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListGraphicsWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_mid_to_output_pos_length(original_self as *const ::list::ListGraphicsWidgetMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsWidgetMutPtr::new](../struct.ListGraphicsWidgetMutPtr.html#method.new) method.
  pub trait ListGraphicsWidgetMutPtrNewArgs {
    fn exec(self) -> ::list::ListGraphicsWidgetMutPtr;
  }
  impl<'a> ListGraphicsWidgetMutPtrNewArgs for &'a ::list::ListGraphicsWidgetMutPtr {
    fn exec(self) -> ::list::ListGraphicsWidgetMutPtr {
      let l = self;
      {
        let mut object: ::list::ListGraphicsWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_constructor_l(l as *const ::list::ListGraphicsWidgetMutPtr,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl ListGraphicsWidgetMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListGraphicsWidgetMutPtr {

      {
        let mut object: ::list::ListGraphicsWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGraphicsWidgetMutPtr::swap](../struct.ListGraphicsWidgetMutPtr.html#method.swap) method.
  pub trait ListGraphicsWidgetMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsWidgetMutPtr) -> ();
  }
  impl<'largs> ListGraphicsWidgetMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsWidgetMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_swap_i_j(original_self as *mut ::list::ListGraphicsWidgetMutPtr,
                                                               i,
                                                               j)
      }
    }
  }
  impl<'largs> ListGraphicsWidgetMutPtrSwapArgs<'largs> for &'largs mut ::list::ListGraphicsWidgetMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListGraphicsWidgetMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QGraphicsWidget_ptr_swap_other(original_self as *mut ::list::ListGraphicsWidgetMutPtr, other as *mut ::list::ListGraphicsWidgetMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListListWidgetItemMutPtr::index_of](../struct.ListListWidgetItemMutPtr.html#method.index_of) method.
  pub trait ListListWidgetItemMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListListWidgetItemMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListListWidgetItemMutPtrIndexOfArgs<'largs> for &'largs *mut ::list_widget_item::ListWidgetItem {
    unsafe fn exec(self, original_self: &'largs ::list::ListListWidgetItemMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_indexOf_t(original_self as *const ::list::ListListWidgetItemMutPtr, t as *const *mut ::list_widget_item::ListWidgetItem)
    }
  }
  impl<'largs> ListListWidgetItemMutPtrIndexOfArgs<'largs>
    for (&'largs *mut ::list_widget_item::ListWidgetItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListListWidgetItemMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_indexOf_t_from(original_self as *const ::list::ListListWidgetItemMutPtr, t as *const *mut ::list_widget_item::ListWidgetItem, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListListWidgetItemMutPtr::last_index_of](../struct.ListListWidgetItemMutPtr.html#method.last_index_of) method.
  pub trait ListListWidgetItemMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListListWidgetItemMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListListWidgetItemMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::list_widget_item::ListWidgetItem {
    unsafe fn exec(self, original_self: &'largs ::list::ListListWidgetItemMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_lastIndexOf_t(original_self as *const ::list::ListListWidgetItemMutPtr, t as *const *mut ::list_widget_item::ListWidgetItem)
    }
  }
  impl<'largs> ListListWidgetItemMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::list_widget_item::ListWidgetItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListListWidgetItemMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_lastIndexOf_t_from(original_self as *const ::list::ListListWidgetItemMutPtr, t as *const *mut ::list_widget_item::ListWidgetItem, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListListWidgetItemMutPtr::mid](../struct.ListListWidgetItemMutPtr.html#method.mid) method.
  pub trait ListListWidgetItemMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListListWidgetItemMutPtr) -> ::list::ListListWidgetItemMutPtr;
  }
  impl<'largs> ListListWidgetItemMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListListWidgetItemMutPtr) -> ::list::ListListWidgetItemMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListListWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_mid_to_output_pos(original_self as *const ::list::ListListWidgetItemMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListListWidgetItemMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListListWidgetItemMutPtr) -> ::list::ListListWidgetItemMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListListWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_mid_to_output_pos_length(original_self as *const ::list::ListListWidgetItemMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListListWidgetItemMutPtr::new](../struct.ListListWidgetItemMutPtr.html#method.new) method.
  pub trait ListListWidgetItemMutPtrNewArgs {
    fn exec(self) -> ::list::ListListWidgetItemMutPtr;
  }
  impl<'a> ListListWidgetItemMutPtrNewArgs for &'a ::list::ListListWidgetItemMutPtr {
    fn exec(self) -> ::list::ListListWidgetItemMutPtr {
      let l = self;
      {
        let mut object: ::list::ListListWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_constructor_l(l as *const ::list::ListListWidgetItemMutPtr,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl ListListWidgetItemMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListListWidgetItemMutPtr {

      {
        let mut object: ::list::ListListWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListListWidgetItemMutPtr::swap](../struct.ListListWidgetItemMutPtr.html#method.swap) method.
  pub trait ListListWidgetItemMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListListWidgetItemMutPtr) -> ();
  }
  impl<'largs> ListListWidgetItemMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListListWidgetItemMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_swap_i_j(original_self as *mut ::list::ListListWidgetItemMutPtr,
                                                               i,
                                                               j)
      }
    }
  }
  impl<'largs> ListListWidgetItemMutPtrSwapArgs<'largs> for &'largs mut ::list::ListListWidgetItemMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListListWidgetItemMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QListWidgetItem_ptr_swap_other(original_self as *mut ::list::ListListWidgetItemMutPtr, other as *mut ::list::ListListWidgetItemMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListMdiSubWindowMutPtr::index_of](../struct.ListMdiSubWindowMutPtr.html#method.index_of) method.
  pub trait ListMdiSubWindowMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListMdiSubWindowMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListMdiSubWindowMutPtrIndexOfArgs<'largs> for &'largs *mut ::mdi_sub_window::MdiSubWindow {
    unsafe fn exec(self, original_self: &'largs ::list::ListMdiSubWindowMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_indexOf_t(original_self as *const ::list::ListMdiSubWindowMutPtr,
                                                            t as *const *mut ::mdi_sub_window::MdiSubWindow)
    }
  }
  impl<'largs> ListMdiSubWindowMutPtrIndexOfArgs<'largs>
    for (&'largs *mut ::mdi_sub_window::MdiSubWindow, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListMdiSubWindowMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_indexOf_t_from(original_self as *const ::list::ListMdiSubWindowMutPtr, t as *const *mut ::mdi_sub_window::MdiSubWindow, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListMdiSubWindowMutPtr::last_index_of](../struct.ListMdiSubWindowMutPtr.html#method.last_index_of) method.
  pub trait ListMdiSubWindowMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListMdiSubWindowMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListMdiSubWindowMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::mdi_sub_window::MdiSubWindow {
    unsafe fn exec(self, original_self: &'largs ::list::ListMdiSubWindowMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_lastIndexOf_t(original_self as *const ::list::ListMdiSubWindowMutPtr, t as *const *mut ::mdi_sub_window::MdiSubWindow)
    }
  }
  impl<'largs> ListMdiSubWindowMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::mdi_sub_window::MdiSubWindow, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListMdiSubWindowMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_lastIndexOf_t_from(original_self as *const ::list::ListMdiSubWindowMutPtr, t as *const *mut ::mdi_sub_window::MdiSubWindow, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListMdiSubWindowMutPtr::mid](../struct.ListMdiSubWindowMutPtr.html#method.mid) method.
  pub trait ListMdiSubWindowMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListMdiSubWindowMutPtr) -> ::list::ListMdiSubWindowMutPtr;
  }
  impl<'largs> ListMdiSubWindowMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListMdiSubWindowMutPtr) -> ::list::ListMdiSubWindowMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListMdiSubWindowMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_mid_to_output_pos(original_self as *const ::list::ListMdiSubWindowMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListMdiSubWindowMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListMdiSubWindowMutPtr) -> ::list::ListMdiSubWindowMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListMdiSubWindowMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_mid_to_output_pos_length(original_self as *const ::list::ListMdiSubWindowMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListMdiSubWindowMutPtr::new](../struct.ListMdiSubWindowMutPtr.html#method.new) method.
  pub trait ListMdiSubWindowMutPtrNewArgs {
    fn exec(self) -> ::list::ListMdiSubWindowMutPtr;
  }
  impl<'a> ListMdiSubWindowMutPtrNewArgs for &'a ::list::ListMdiSubWindowMutPtr {
    fn exec(self) -> ::list::ListMdiSubWindowMutPtr {
      let l = self;
      {
        let mut object: ::list::ListMdiSubWindowMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_constructor_l(l as *const ::list::ListMdiSubWindowMutPtr,
                                                                    &mut object);
        }
        object
      }
    }
  }
  impl ListMdiSubWindowMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListMdiSubWindowMutPtr {

      {
        let mut object: ::list::ListMdiSubWindowMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListMdiSubWindowMutPtr::swap](../struct.ListMdiSubWindowMutPtr.html#method.swap) method.
  pub trait ListMdiSubWindowMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListMdiSubWindowMutPtr) -> ();
  }
  impl<'largs> ListMdiSubWindowMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListMdiSubWindowMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_swap_i_j(original_self as *mut ::list::ListMdiSubWindowMutPtr, i, j)
      }
    }
  }
  impl<'largs> ListMdiSubWindowMutPtrSwapArgs<'largs> for &'largs mut ::list::ListMdiSubWindowMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListMdiSubWindowMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QMdiSubWindow_ptr_swap_other(original_self as *mut ::list::ListMdiSubWindowMutPtr,
                                                               other as *mut ::list::ListMdiSubWindowMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleCDouble::append](../struct.ListPairPairCDoubleCDouble.html#method.append) method.
  pub trait ListPairPairCDoubleCDoubleAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleCDouble) -> ();
  }
  impl<'largs> ListPairPairCDoubleCDoubleAppendArgs<'largs> for &'largs ::list::ListPairPairCDoubleCDouble {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleCDouble) -> () {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_append_QList_QPair_double_double(original_self as *mut ::list::ListPairPairCDoubleCDouble, t as *const ::list::ListPairPairCDoubleCDouble) }
    }
  }
  impl<'largs> ListPairPairCDoubleCDoubleAppendArgs<'largs> for &'largs ::pair::PairCDoubleCDouble {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleCDouble) -> () {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_append_QPair_double_double(original_self as *mut ::list::ListPairPairCDoubleCDouble, t as *const ::pair::PairCDoubleCDouble) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleCDouble::count](../struct.ListPairPairCDoubleCDouble.html#method.count) method.
  pub trait ListPairPairCDoubleCDoubleCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::libc::c_int;
  }
  impl<'largs> ListPairPairCDoubleCDoubleCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_count_no_args(original_self as *const ::list::ListPairPairCDoubleCDouble) }
    }
  }
  impl<'largs> ListPairPairCDoubleCDoubleCountArgs<'largs> for &'largs ::pair::PairCDoubleCDouble {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_count_t(original_self as *const ::list::ListPairPairCDoubleCDouble, t as *const ::pair::PairCDoubleCDouble) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleCDouble::index_of](../struct.ListPairPairCDoubleCDouble.html#method.index_of) method.
  pub trait ListPairPairCDoubleCDoubleIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::libc::c_int;
  }
  impl<'largs> ListPairPairCDoubleCDoubleIndexOfArgs<'largs> for &'largs ::pair::PairCDoubleCDouble {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_indexOf_t(original_self as *const ::list::ListPairPairCDoubleCDouble, t as *const ::pair::PairCDoubleCDouble) }
    }
  }
  impl<'largs> ListPairPairCDoubleCDoubleIndexOfArgs<'largs> for (&'largs ::pair::PairCDoubleCDouble, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_indexOf_t_from(original_self as *const ::list::ListPairPairCDoubleCDouble, t as *const ::pair::PairCDoubleCDouble, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleCDouble::last_index_of](../struct.ListPairPairCDoubleCDouble.html#method.last_index_of) method.
  pub trait ListPairPairCDoubleCDoubleLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::libc::c_int;
  }
  impl<'largs> ListPairPairCDoubleCDoubleLastIndexOfArgs<'largs> for &'largs ::pair::PairCDoubleCDouble {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_lastIndexOf_t(original_self as *const ::list::ListPairPairCDoubleCDouble, t as *const ::pair::PairCDoubleCDouble) }
    }
  }
  impl<'largs> ListPairPairCDoubleCDoubleLastIndexOfArgs<'largs> for (&'largs ::pair::PairCDoubleCDouble, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_lastIndexOf_t_from(original_self as *const ::list::ListPairPairCDoubleCDouble, t as *const ::pair::PairCDoubleCDouble, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleCDouble::mid](../struct.ListPairPairCDoubleCDouble.html#method.mid) method.
  pub trait ListPairPairCDoubleCDoubleMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::list::ListPairPairCDoubleCDouble;
  }
  impl<'largs> ListPairPairCDoubleCDoubleMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::list::ListPairPairCDoubleCDouble {
      let pos = self;
      {
        let mut object: ::list::ListPairPairCDoubleCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_double_mid_to_output_pos(original_self as *const ::list::ListPairPairCDoubleCDouble, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListPairPairCDoubleCDoubleMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::list::ListPairPairCDoubleCDouble {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListPairPairCDoubleCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_double_mid_to_output_pos_length(original_self as *const ::list::ListPairPairCDoubleCDouble, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleCDouble::new](../struct.ListPairPairCDoubleCDouble.html#method.new) method.
  pub trait ListPairPairCDoubleCDoubleNewArgs {
    fn exec(self) -> ::list::ListPairPairCDoubleCDouble;
  }
  impl<'a> ListPairPairCDoubleCDoubleNewArgs for &'a ::list::ListPairPairCDoubleCDouble {
    fn exec(self) -> ::list::ListPairPairCDoubleCDouble {
      let l = self;
      {
        let mut object: ::list::ListPairPairCDoubleCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_double_constructor_l(l as *const ::list::ListPairPairCDoubleCDouble,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl ListPairPairCDoubleCDoubleNewArgs for () {
    fn exec(self) -> ::list::ListPairPairCDoubleCDouble {

      {
        let mut object: ::list::ListPairPairCDoubleCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_double_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleCDouble::op_add_assign](../struct.ListPairPairCDoubleCDouble.html#method.op_add_assign) method.
  pub trait ListPairPairCDoubleCDoubleOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairCDoubleCDouble)
            -> &'largs mut ::list::ListPairPairCDoubleCDouble;
  }
  impl<'largs> ListPairPairCDoubleCDoubleOpAddAssignArgs<'largs> for &'largs ::list::ListPairPairCDoubleCDouble {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairCDoubleCDouble)
            -> &'largs mut ::list::ListPairPairCDoubleCDouble {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_operator_add_assign_l(original_self as *mut ::list::ListPairPairCDoubleCDouble, l as *const ::list::ListPairPairCDoubleCDouble) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListPairPairCDoubleCDoubleOpAddAssignArgs<'largs> for &'largs ::pair::PairCDoubleCDouble {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairCDoubleCDouble)
            -> &'largs mut ::list::ListPairPairCDoubleCDouble {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_operator_add_assign_t(original_self as *mut ::list::ListPairPairCDoubleCDouble, t as *const ::pair::PairCDoubleCDouble) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleCDouble::op_shl](../struct.ListPairPairCDoubleCDouble.html#method.op_shl) method.
  pub trait ListPairPairCDoubleCDoubleOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairCDoubleCDouble)
            -> &'largs mut ::list::ListPairPairCDoubleCDouble;
  }
  impl<'largs> ListPairPairCDoubleCDoubleOpShlArgs<'largs> for &'largs ::list::ListPairPairCDoubleCDouble {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairCDoubleCDouble)
            -> &'largs mut ::list::ListPairPairCDoubleCDouble {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_operator_shl_l(original_self as *mut ::list::ListPairPairCDoubleCDouble, l as *const ::list::ListPairPairCDoubleCDouble) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListPairPairCDoubleCDoubleOpShlArgs<'largs> for &'largs ::pair::PairCDoubleCDouble {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairCDoubleCDouble)
            -> &'largs mut ::list::ListPairPairCDoubleCDouble {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_operator_shl_t(original_self as *mut ::list::ListPairPairCDoubleCDouble, t as *const ::pair::PairCDoubleCDouble) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleCDouble::swap](../struct.ListPairPairCDoubleCDouble.html#method.swap) method.
  pub trait ListPairPairCDoubleCDoubleSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleCDouble) -> ();
  }
  impl<'largs> ListPairPairCDoubleCDoubleSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleCDouble) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QPair_double_double_swap_i_j(original_self as *mut ::list::ListPairPairCDoubleCDouble, i, j)
      }
    }
  }
  impl<'largs> ListPairPairCDoubleCDoubleSwapArgs<'largs> for &'largs mut ::list::ListPairPairCDoubleCDouble {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleCDouble) -> () {
      let other = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_double_swap_other(original_self as *mut ::list::ListPairPairCDoubleCDouble, other as *mut ::list::ListPairPairCDoubleCDouble) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleCDouble::value](../struct.ListPairPairCDoubleCDouble.html#method.value) method.
  pub trait ListPairPairCDoubleCDoubleValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::pair::PairCDoubleCDouble;
  }
  impl<'largs> ListPairPairCDoubleCDoubleValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::pair::PairCDoubleCDouble {
      let i = self;
      {
        let mut object: ::pair::PairCDoubleCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_double_value_to_output_i(original_self as *const ::list::ListPairPairCDoubleCDouble, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListPairPairCDoubleCDoubleValueArgs<'largs> for (::libc::c_int, &'largs ::pair::PairCDoubleCDouble) {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleCDouble) -> ::pair::PairCDoubleCDouble {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::pair::PairCDoubleCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_double_value_to_output_i_defaultValue(original_self as *const ::list::ListPairPairCDoubleCDouble, i, default_value as *const ::pair::PairCDoubleCDouble, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleQtCorePointF::append](../struct.ListPairPairCDoubleQtCorePointF.html#method.append) method.
  pub trait ListPairPairCDoubleQtCorePointFAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF) -> ();
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFAppendArgs<'largs> for &'largs ::list::ListPairPairCDoubleQtCorePointF {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF) -> () {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_append_QList_QPair_double_QPointF(original_self as *mut ::list::ListPairPairCDoubleQtCorePointF, t as *const ::list::ListPairPairCDoubleQtCorePointF) }
    }
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFAppendArgs<'largs> for &'largs ::pair::PairCDoubleQtCorePointF {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF) -> () {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_append_QPair_double_QPointF(original_self as *mut ::list::ListPairPairCDoubleQtCorePointF, t as *const ::pair::PairCDoubleQtCorePointF) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleQtCorePointF::count](../struct.ListPairPairCDoubleQtCorePointF.html#method.count) method.
  pub trait ListPairPairCDoubleQtCorePointFCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::libc::c_int;
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_count_no_args(original_self as *const ::list::ListPairPairCDoubleQtCorePointF) }
    }
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFCountArgs<'largs> for &'largs ::pair::PairCDoubleQtCorePointF {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_count_t(original_self as *const ::list::ListPairPairCDoubleQtCorePointF, t as *const ::pair::PairCDoubleQtCorePointF) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleQtCorePointF::index_of](../struct.ListPairPairCDoubleQtCorePointF.html#method.index_of) method.
  pub trait ListPairPairCDoubleQtCorePointFIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::libc::c_int;
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFIndexOfArgs<'largs> for &'largs ::pair::PairCDoubleQtCorePointF {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_indexOf_t(original_self as *const ::list::ListPairPairCDoubleQtCorePointF, t as *const ::pair::PairCDoubleQtCorePointF) }
    }
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFIndexOfArgs<'largs>
    for (&'largs ::pair::PairCDoubleQtCorePointF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_indexOf_t_from(original_self as *const ::list::ListPairPairCDoubleQtCorePointF, t as *const ::pair::PairCDoubleQtCorePointF, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleQtCorePointF::last_index_of](../struct.ListPairPairCDoubleQtCorePointF.html#method.last_index_of) method.
  pub trait ListPairPairCDoubleQtCorePointFLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::libc::c_int;
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFLastIndexOfArgs<'largs> for &'largs ::pair::PairCDoubleQtCorePointF {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_lastIndexOf_t(original_self as *const ::list::ListPairPairCDoubleQtCorePointF, t as *const ::pair::PairCDoubleQtCorePointF) }
    }
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFLastIndexOfArgs<'largs>
    for (&'largs ::pair::PairCDoubleQtCorePointF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_lastIndexOf_t_from(original_self as *const ::list::ListPairPairCDoubleQtCorePointF, t as *const ::pair::PairCDoubleQtCorePointF, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleQtCorePointF::mid](../struct.ListPairPairCDoubleQtCorePointF.html#method.mid) method.
  pub trait ListPairPairCDoubleQtCorePointFMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF)
            -> ::list::ListPairPairCDoubleQtCorePointF;
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF)
            -> ::list::ListPairPairCDoubleQtCorePointF {
      let pos = self;
      {
        let mut object: ::list::ListPairPairCDoubleQtCorePointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_QPointF_mid_to_output_pos(original_self as *const ::list::ListPairPairCDoubleQtCorePointF, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF)
            -> ::list::ListPairPairCDoubleQtCorePointF {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListPairPairCDoubleQtCorePointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_QPointF_mid_to_output_pos_length(original_self as *const ::list::ListPairPairCDoubleQtCorePointF, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleQtCorePointF::new](../struct.ListPairPairCDoubleQtCorePointF.html#method.new) method.
  pub trait ListPairPairCDoubleQtCorePointFNewArgs {
    fn exec(self) -> ::list::ListPairPairCDoubleQtCorePointF;
  }
  impl<'a> ListPairPairCDoubleQtCorePointFNewArgs for &'a ::list::ListPairPairCDoubleQtCorePointF {
    fn exec(self) -> ::list::ListPairPairCDoubleQtCorePointF {
      let l = self;
      {
        let mut object: ::list::ListPairPairCDoubleQtCorePointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_QPointF_constructor_l(l as *const ::list::ListPairPairCDoubleQtCorePointF, &mut object);
        }
        object
      }
    }
  }
  impl ListPairPairCDoubleQtCorePointFNewArgs for () {
    fn exec(self) -> ::list::ListPairPairCDoubleQtCorePointF {

      {
        let mut object: ::list::ListPairPairCDoubleQtCorePointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_QPointF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleQtCorePointF::op_add_assign](../struct.ListPairPairCDoubleQtCorePointF.html#method.op_add_assign) method.
  pub trait ListPairPairCDoubleQtCorePointFOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF)
            -> &'largs mut ::list::ListPairPairCDoubleQtCorePointF;
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFOpAddAssignArgs<'largs> for &'largs ::list::ListPairPairCDoubleQtCorePointF {

  fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF) -> &'largs mut ::list::ListPairPairCDoubleQtCorePointF {
    let l = self;
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_operator_add_assign_l(original_self as *mut ::list::ListPairPairCDoubleQtCorePointF, l as *const ::list::ListPairPairCDoubleQtCorePointF) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> ListPairPairCDoubleQtCorePointFOpAddAssignArgs<'largs> for &'largs ::pair::PairCDoubleQtCorePointF {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF)
            -> &'largs mut ::list::ListPairPairCDoubleQtCorePointF {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_operator_add_assign_t(original_self as *mut ::list::ListPairPairCDoubleQtCorePointF, t as *const ::pair::PairCDoubleQtCorePointF) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleQtCorePointF::op_shl](../struct.ListPairPairCDoubleQtCorePointF.html#method.op_shl) method.
  pub trait ListPairPairCDoubleQtCorePointFOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF)
            -> &'largs mut ::list::ListPairPairCDoubleQtCorePointF;
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFOpShlArgs<'largs> for &'largs ::list::ListPairPairCDoubleQtCorePointF {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF)
            -> &'largs mut ::list::ListPairPairCDoubleQtCorePointF {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_operator_shl_l(original_self as *mut ::list::ListPairPairCDoubleQtCorePointF, l as *const ::list::ListPairPairCDoubleQtCorePointF) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFOpShlArgs<'largs> for &'largs ::pair::PairCDoubleQtCorePointF {
    fn exec(self,
            original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF)
            -> &'largs mut ::list::ListPairPairCDoubleQtCorePointF {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_operator_shl_t(original_self as *mut ::list::ListPairPairCDoubleQtCorePointF, t as *const ::pair::PairCDoubleQtCorePointF) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleQtCorePointF::swap](../struct.ListPairPairCDoubleQtCorePointF.html#method.swap) method.
  pub trait ListPairPairCDoubleQtCorePointFSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF) -> ();
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_swap_i_j(original_self as *mut ::list::ListPairPairCDoubleQtCorePointF, i, j) }
    }
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFSwapArgs<'largs> for &'largs mut ::list::ListPairPairCDoubleQtCorePointF {
    fn exec(self, original_self: &'largs mut ::list::ListPairPairCDoubleQtCorePointF) -> () {
      let other = self;
      unsafe { ::ffi::qt_widgets_c_QList_QPair_double_QPointF_swap_other(original_self as *mut ::list::ListPairPairCDoubleQtCorePointF, other as *mut ::list::ListPairPairCDoubleQtCorePointF) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPairPairCDoubleQtCorePointF::value](../struct.ListPairPairCDoubleQtCorePointF.html#method.value) method.
  pub trait ListPairPairCDoubleQtCorePointFValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::pair::PairCDoubleQtCorePointF;
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::pair::PairCDoubleQtCorePointF {
      let i = self;
      {
        let mut object: ::pair::PairCDoubleQtCorePointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_QPointF_value_to_output_i(original_self as *const ::list::ListPairPairCDoubleQtCorePointF, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListPairPairCDoubleQtCorePointFValueArgs<'largs>
    for (::libc::c_int, &'largs ::pair::PairCDoubleQtCorePointF) {
    fn exec(self, original_self: &'largs ::list::ListPairPairCDoubleQtCorePointF) -> ::pair::PairCDoubleQtCorePointF {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::pair::PairCDoubleQtCorePointF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QPair_double_QPointF_value_to_output_i_defaultValue(original_self as *const ::list::ListPairPairCDoubleQtCorePointF, i, default_value as *const ::pair::PairCDoubleQtCorePointF, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreRectF::append](../struct.ListQtCoreRectF.html#method.append) method.
  pub trait ListQtCoreRectFAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> ();
  }
  impl<'largs> ListQtCoreRectFAppendArgs<'largs> for &'largs ::list::ListQtCoreRectF {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_append_QList_QRectF(original_self as *mut ::list::ListQtCoreRectF,
                                                             t as *const ::list::ListQtCoreRectF)
      }
    }
  }
  impl<'largs> ListQtCoreRectFAppendArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_append_QRectF(original_self as *mut ::list::ListQtCoreRectF,
                                                       t as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreRectF::count](../struct.ListQtCoreRectF.html#method.count) method.
  pub trait ListQtCoreRectFCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::libc::c_int;
  }
  impl<'largs> ListQtCoreRectFCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QList_QRectF_count_no_args(original_self as *const ::list::ListQtCoreRectF) }
    }
  }
  impl<'largs> ListQtCoreRectFCountArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_count_t(original_self as *const ::list::ListQtCoreRectF,
                                                 t as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreRectF::index_of](../struct.ListQtCoreRectF.html#method.index_of) method.
  pub trait ListQtCoreRectFIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::libc::c_int;
  }
  impl<'largs> ListQtCoreRectFIndexOfArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_indexOf_t(original_self as *const ::list::ListQtCoreRectF,
                                                   t as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> ListQtCoreRectFIndexOfArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_indexOf_t_from(original_self as *const ::list::ListQtCoreRectF,
                                                        t as *const ::qt_core::rect_f::RectF,
                                                        from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreRectF::last_index_of](../struct.ListQtCoreRectF.html#method.last_index_of) method.
  pub trait ListQtCoreRectFLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::libc::c_int;
  }
  impl<'largs> ListQtCoreRectFLastIndexOfArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_lastIndexOf_t(original_self as *const ::list::ListQtCoreRectF,
                                                       t as *const ::qt_core::rect_f::RectF)
      }
    }
  }
  impl<'largs> ListQtCoreRectFLastIndexOfArgs<'largs> for (&'largs ::qt_core::rect_f::RectF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_lastIndexOf_t_from(original_self as *const ::list::ListQtCoreRectF,
                                                            t as *const ::qt_core::rect_f::RectF,
                                                            from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreRectF::mid](../struct.ListQtCoreRectF.html#method.mid) method.
  pub trait ListQtCoreRectFMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::list::ListQtCoreRectF;
  }
  impl<'largs> ListQtCoreRectFMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::list::ListQtCoreRectF {
      let pos = self;
      {
        let mut object: ::list::ListQtCoreRectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QRectF_mid_to_output_pos(original_self as *const ::list::ListQtCoreRectF,
                                                             pos,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListQtCoreRectFMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::list::ListQtCoreRectF {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListQtCoreRectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QRectF_mid_to_output_pos_length(original_self as *const ::list::ListQtCoreRectF,
                                                                    pos,
                                                                    length,
                                                                    &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreRectF::new](../struct.ListQtCoreRectF.html#method.new) method.
  pub trait ListQtCoreRectFNewArgs {
    fn exec(self) -> ::list::ListQtCoreRectF;
  }
  impl<'a> ListQtCoreRectFNewArgs for &'a ::list::ListQtCoreRectF {
    fn exec(self) -> ::list::ListQtCoreRectF {
      let l = self;
      {
        let mut object: ::list::ListQtCoreRectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QRectF_constructor_l(l as *const ::list::ListQtCoreRectF, &mut object);
        }
        object
      }
    }
  }
  impl ListQtCoreRectFNewArgs for () {
    fn exec(self) -> ::list::ListQtCoreRectF {

      {
        let mut object: ::list::ListQtCoreRectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QRectF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreRectF::op_add_assign](../struct.ListQtCoreRectF.html#method.op_add_assign) method.
  pub trait ListQtCoreRectFOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> &'largs mut ::list::ListQtCoreRectF;
  }
  impl<'largs> ListQtCoreRectFOpAddAssignArgs<'largs> for &'largs ::list::ListQtCoreRectF {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> &'largs mut ::list::ListQtCoreRectF {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QList_QRectF_operator_add_assign_l(original_self as *mut ::list::ListQtCoreRectF,
                                                                 l as *const ::list::ListQtCoreRectF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListQtCoreRectFOpAddAssignArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> &'largs mut ::list::ListQtCoreRectF {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QList_QRectF_operator_add_assign_t(original_self as *mut ::list::ListQtCoreRectF,
                                                                 t as *const ::qt_core::rect_f::RectF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreRectF::op_shl](../struct.ListQtCoreRectF.html#method.op_shl) method.
  pub trait ListQtCoreRectFOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> &'largs mut ::list::ListQtCoreRectF;
  }
  impl<'largs> ListQtCoreRectFOpShlArgs<'largs> for &'largs ::list::ListQtCoreRectF {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> &'largs mut ::list::ListQtCoreRectF {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_operator_shl_l(original_self as *mut ::list::ListQtCoreRectF,
                                                        l as *const ::list::ListQtCoreRectF)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListQtCoreRectFOpShlArgs<'largs> for &'largs ::qt_core::rect_f::RectF {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> &'largs mut ::list::ListQtCoreRectF {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_operator_shl_t(original_self as *mut ::list::ListQtCoreRectF,
                                                        t as *const ::qt_core::rect_f::RectF)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreRectF::swap](../struct.ListQtCoreRectF.html#method.swap) method.
  pub trait ListQtCoreRectFSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> ();
  }
  impl<'largs> ListQtCoreRectFSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QRectF_swap_i_j(original_self as *mut ::list::ListQtCoreRectF, i, j) }
    }
  }
  impl<'largs> ListQtCoreRectFSwapArgs<'largs> for &'largs mut ::list::ListQtCoreRectF {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreRectF) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QRectF_swap_other(original_self as *mut ::list::ListQtCoreRectF,
                                                    other as *mut ::list::ListQtCoreRectF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreRectF::value](../struct.ListQtCoreRectF.html#method.value) method.
  pub trait ListQtCoreRectFValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::qt_core::rect_f::RectF;
  }
  impl<'largs> ListQtCoreRectFValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::qt_core::rect_f::RectF {
      let i = self;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QRectF_value_to_output_i(original_self as *const ::list::ListQtCoreRectF,
                                                             i,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListQtCoreRectFValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::rect_f::RectF) {
    fn exec(self, original_self: &'largs ::list::ListQtCoreRectF) -> ::qt_core::rect_f::RectF {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::qt_core::rect_f::RectF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QRectF_value_to_output_i_defaultValue(original_self as *const ::list::ListQtCoreRectF, i, default_value as *const ::qt_core::rect_f::RectF, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListScrollerMutPtr::index_of](../struct.ListScrollerMutPtr.html#method.index_of) method.
  pub trait ListScrollerMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListScrollerMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListScrollerMutPtrIndexOfArgs<'largs> for &'largs *mut ::scroller::Scroller {
    unsafe fn exec(self, original_self: &'largs ::list::ListScrollerMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QScroller_ptr_indexOf_t(original_self as *const ::list::ListScrollerMutPtr,
                                                        t as *const *mut ::scroller::Scroller)
    }
  }
  impl<'largs> ListScrollerMutPtrIndexOfArgs<'largs> for (&'largs *mut ::scroller::Scroller, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListScrollerMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QScroller_ptr_indexOf_t_from(original_self as *const ::list::ListScrollerMutPtr,
                                                             t as *const *mut ::scroller::Scroller,
                                                             from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListScrollerMutPtr::last_index_of](../struct.ListScrollerMutPtr.html#method.last_index_of) method.
  pub trait ListScrollerMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListScrollerMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListScrollerMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::scroller::Scroller {
    unsafe fn exec(self, original_self: &'largs ::list::ListScrollerMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QScroller_ptr_lastIndexOf_t(original_self as *const ::list::ListScrollerMutPtr,
                                                            t as *const *mut ::scroller::Scroller)
    }
  }
  impl<'largs> ListScrollerMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::scroller::Scroller, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListScrollerMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QScroller_ptr_lastIndexOf_t_from(original_self as *const ::list::ListScrollerMutPtr,
                                                                 t as *const *mut ::scroller::Scroller,
                                                                 from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListScrollerMutPtr::mid](../struct.ListScrollerMutPtr.html#method.mid) method.
  pub trait ListScrollerMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListScrollerMutPtr) -> ::list::ListScrollerMutPtr;
  }
  impl<'largs> ListScrollerMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListScrollerMutPtr) -> ::list::ListScrollerMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListScrollerMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QScroller_ptr_mid_to_output_pos(original_self as *const ::list::ListScrollerMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListScrollerMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListScrollerMutPtr) -> ::list::ListScrollerMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListScrollerMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QScroller_ptr_mid_to_output_pos_length(original_self as *const ::list::ListScrollerMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListScrollerMutPtr::new](../struct.ListScrollerMutPtr.html#method.new) method.
  pub trait ListScrollerMutPtrNewArgs {
    fn exec(self) -> ::list::ListScrollerMutPtr;
  }
  impl<'a> ListScrollerMutPtrNewArgs for &'a ::list::ListScrollerMutPtr {
    fn exec(self) -> ::list::ListScrollerMutPtr {
      let l = self;
      {
        let mut object: ::list::ListScrollerMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QScroller_ptr_constructor_l(l as *const ::list::ListScrollerMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListScrollerMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListScrollerMutPtr {

      {
        let mut object: ::list::ListScrollerMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QScroller_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListScrollerMutPtr::swap](../struct.ListScrollerMutPtr.html#method.swap) method.
  pub trait ListScrollerMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListScrollerMutPtr) -> ();
  }
  impl<'largs> ListScrollerMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListScrollerMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QScroller_ptr_swap_i_j(original_self as *mut ::list::ListScrollerMutPtr, i, j)
      }
    }
  }
  impl<'largs> ListScrollerMutPtrSwapArgs<'largs> for &'largs mut ::list::ListScrollerMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListScrollerMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QScroller_ptr_swap_other(original_self as *mut ::list::ListScrollerMutPtr,
                                                           other as *mut ::list::ListScrollerMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTableWidgetItemMutPtr::index_of](../struct.ListTableWidgetItemMutPtr.html#method.index_of) method.
  pub trait ListTableWidgetItemMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListTableWidgetItemMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListTableWidgetItemMutPtrIndexOfArgs<'largs> for &'largs *mut ::table_widget_item::TableWidgetItem {
    unsafe fn exec(self, original_self: &'largs ::list::ListTableWidgetItemMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_indexOf_t(original_self as *const ::list::ListTableWidgetItemMutPtr, t as *const *mut ::table_widget_item::TableWidgetItem)
    }
  }
  impl<'largs> ListTableWidgetItemMutPtrIndexOfArgs<'largs>
    for (&'largs *mut ::table_widget_item::TableWidgetItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListTableWidgetItemMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_indexOf_t_from(original_self as *const ::list::ListTableWidgetItemMutPtr, t as *const *mut ::table_widget_item::TableWidgetItem, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListTableWidgetItemMutPtr::last_index_of](../struct.ListTableWidgetItemMutPtr.html#method.last_index_of) method.
  pub trait ListTableWidgetItemMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListTableWidgetItemMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListTableWidgetItemMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::table_widget_item::TableWidgetItem {
    unsafe fn exec(self, original_self: &'largs ::list::ListTableWidgetItemMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_lastIndexOf_t(original_self as *const ::list::ListTableWidgetItemMutPtr, t as *const *mut ::table_widget_item::TableWidgetItem)
    }
  }
  impl<'largs> ListTableWidgetItemMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::table_widget_item::TableWidgetItem,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::list::ListTableWidgetItemMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_lastIndexOf_t_from(original_self as *const ::list::ListTableWidgetItemMutPtr, t as *const *mut ::table_widget_item::TableWidgetItem, from)
  }
}
  /// This trait represents a set of arguments accepted by [ListTableWidgetItemMutPtr::mid](../struct.ListTableWidgetItemMutPtr.html#method.mid) method.
  pub trait ListTableWidgetItemMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTableWidgetItemMutPtr) -> ::list::ListTableWidgetItemMutPtr;
  }
  impl<'largs> ListTableWidgetItemMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListTableWidgetItemMutPtr) -> ::list::ListTableWidgetItemMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListTableWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_mid_to_output_pos(original_self as *const ::list::ListTableWidgetItemMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTableWidgetItemMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTableWidgetItemMutPtr) -> ::list::ListTableWidgetItemMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListTableWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_mid_to_output_pos_length(original_self as *const ::list::ListTableWidgetItemMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTableWidgetItemMutPtr::new](../struct.ListTableWidgetItemMutPtr.html#method.new) method.
  pub trait ListTableWidgetItemMutPtrNewArgs {
    fn exec(self) -> ::list::ListTableWidgetItemMutPtr;
  }
  impl<'a> ListTableWidgetItemMutPtrNewArgs for &'a ::list::ListTableWidgetItemMutPtr {
    fn exec(self) -> ::list::ListTableWidgetItemMutPtr {
      let l = self;
      {
        let mut object: ::list::ListTableWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_constructor_l(l as *const ::list::ListTableWidgetItemMutPtr,
                                                                       &mut object);
        }
        object
      }
    }
  }
  impl ListTableWidgetItemMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListTableWidgetItemMutPtr {

      {
        let mut object: ::list::ListTableWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTableWidgetItemMutPtr::swap](../struct.ListTableWidgetItemMutPtr.html#method.swap) method.
  pub trait ListTableWidgetItemMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTableWidgetItemMutPtr) -> ();
  }
  impl<'largs> ListTableWidgetItemMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListTableWidgetItemMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_swap_i_j(original_self as *mut ::list::ListTableWidgetItemMutPtr, i, j)
      }
    }
  }
  impl<'largs> ListTableWidgetItemMutPtrSwapArgs<'largs> for &'largs mut ::list::ListTableWidgetItemMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListTableWidgetItemMutPtr) -> () {
      let other = self;
      unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetItem_ptr_swap_other(original_self as *mut ::list::ListTableWidgetItemMutPtr, other as *mut ::list::ListTableWidgetItemMutPtr) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTableWidgetSelectionRange::append](../struct.ListTableWidgetSelectionRange.html#method.append) method.
  pub trait ListTableWidgetSelectionRangeAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTableWidgetSelectionRange) -> ();
  }
  impl<'largs> ListTableWidgetSelectionRangeAppendArgs<'largs> for &'largs ::list::ListTableWidgetSelectionRange {
    fn exec(self, original_self: &'largs mut ::list::ListTableWidgetSelectionRange) -> () {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_append_QList_QTableWidgetSelectionRange(original_self as *mut ::list::ListTableWidgetSelectionRange, t as *const ::list::ListTableWidgetSelectionRange) }
    }
  }
  impl<'largs> ListTableWidgetSelectionRangeAppendArgs<'largs> for &'largs ::table_widget_selection_range::TableWidgetSelectionRange {

  fn exec(self, original_self: &'largs mut ::list::ListTableWidgetSelectionRange) -> () {
    let t = self;
    unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_append_QTableWidgetSelectionRange(original_self as *mut ::list::ListTableWidgetSelectionRange, t as *const ::table_widget_selection_range::TableWidgetSelectionRange) }
  }
}
  /// This trait represents a set of arguments accepted by [ListTableWidgetSelectionRange::mid](../struct.ListTableWidgetSelectionRange.html#method.mid) method.
  pub trait ListTableWidgetSelectionRangeMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::list::ListTableWidgetSelectionRange)
            -> ::list::ListTableWidgetSelectionRange;
  }
  impl<'largs> ListTableWidgetSelectionRangeMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::list::ListTableWidgetSelectionRange)
            -> ::list::ListTableWidgetSelectionRange {
      let pos = self;
      {
        let mut object: ::list::ListTableWidgetSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_mid_to_output_pos(original_self as *const ::list::ListTableWidgetSelectionRange, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTableWidgetSelectionRangeMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::list::ListTableWidgetSelectionRange)
            -> ::list::ListTableWidgetSelectionRange {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListTableWidgetSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_mid_to_output_pos_length(original_self as *const ::list::ListTableWidgetSelectionRange, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTableWidgetSelectionRange::new](../struct.ListTableWidgetSelectionRange.html#method.new) method.
  pub trait ListTableWidgetSelectionRangeNewArgs {
    fn exec(self) -> ::list::ListTableWidgetSelectionRange;
  }
  impl<'a> ListTableWidgetSelectionRangeNewArgs for &'a ::list::ListTableWidgetSelectionRange {
    fn exec(self) -> ::list::ListTableWidgetSelectionRange {
      let l = self;
      {
        let mut object: ::list::ListTableWidgetSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_constructor_l(l as *const ::list::ListTableWidgetSelectionRange, &mut object);
        }
        object
      }
    }
  }
  impl ListTableWidgetSelectionRangeNewArgs for () {
    fn exec(self) -> ::list::ListTableWidgetSelectionRange {

      {
        let mut object: ::list::ListTableWidgetSelectionRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTableWidgetSelectionRange::op_add_assign](../struct.ListTableWidgetSelectionRange.html#method.op_add_assign) method.
  pub trait ListTableWidgetSelectionRangeOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListTableWidgetSelectionRange)
            -> &'largs mut ::list::ListTableWidgetSelectionRange;
  }
  impl<'largs> ListTableWidgetSelectionRangeOpAddAssignArgs<'largs> for &'largs ::list::ListTableWidgetSelectionRange {
    fn exec(self,
            original_self: &'largs mut ::list::ListTableWidgetSelectionRange)
            -> &'largs mut ::list::ListTableWidgetSelectionRange {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_operator_add_assign_l(original_self as *mut ::list::ListTableWidgetSelectionRange, l as *const ::list::ListTableWidgetSelectionRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTableWidgetSelectionRangeOpAddAssignArgs<'largs> for &'largs ::table_widget_selection_range::TableWidgetSelectionRange {

  fn exec(self, original_self: &'largs mut ::list::ListTableWidgetSelectionRange) -> &'largs mut ::list::ListTableWidgetSelectionRange {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_operator_add_assign_t(original_self as *mut ::list::ListTableWidgetSelectionRange, t as *const ::table_widget_selection_range::TableWidgetSelectionRange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [ListTableWidgetSelectionRange::op_shl](../struct.ListTableWidgetSelectionRange.html#method.op_shl) method.
  pub trait ListTableWidgetSelectionRangeOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListTableWidgetSelectionRange)
            -> &'largs mut ::list::ListTableWidgetSelectionRange;
  }
  impl<'largs> ListTableWidgetSelectionRangeOpShlArgs<'largs> for &'largs ::list::ListTableWidgetSelectionRange {
    fn exec(self,
            original_self: &'largs mut ::list::ListTableWidgetSelectionRange)
            -> &'largs mut ::list::ListTableWidgetSelectionRange {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_operator_shl_l(original_self as *mut ::list::ListTableWidgetSelectionRange, l as *const ::list::ListTableWidgetSelectionRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTableWidgetSelectionRangeOpShlArgs<'largs> for &'largs ::table_widget_selection_range::TableWidgetSelectionRange {

  fn exec(self, original_self: &'largs mut ::list::ListTableWidgetSelectionRange) -> &'largs mut ::list::ListTableWidgetSelectionRange {
    let t = self;
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_operator_shl_t(original_self as *mut ::list::ListTableWidgetSelectionRange, t as *const ::table_widget_selection_range::TableWidgetSelectionRange) };
unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [ListTableWidgetSelectionRange::swap](../struct.ListTableWidgetSelectionRange.html#method.swap) method.
  pub trait ListTableWidgetSelectionRangeSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTableWidgetSelectionRange) -> ();
  }
  impl<'largs> ListTableWidgetSelectionRangeSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListTableWidgetSelectionRange) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_swap_i_j(original_self as *mut ::list::ListTableWidgetSelectionRange, i, j) }
    }
  }
  impl<'largs> ListTableWidgetSelectionRangeSwapArgs<'largs> for &'largs mut ::list::ListTableWidgetSelectionRange {
    fn exec(self, original_self: &'largs mut ::list::ListTableWidgetSelectionRange) -> () {
      let other = self;
      unsafe { ::ffi::qt_widgets_c_QList_QTableWidgetSelectionRange_swap_other(original_self as *mut ::list::ListTableWidgetSelectionRange, other as *mut ::list::ListTableWidgetSelectionRange) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextEditExtraSelection::append](../struct.ListTextEditExtraSelection.html#method.append) method.
  pub trait ListTextEditExtraSelectionAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextEditExtraSelection) -> ();
  }
  impl<'largs> ListTextEditExtraSelectionAppendArgs<'largs> for &'largs ::list::ListTextEditExtraSelection {
    fn exec(self, original_self: &'largs mut ::list::ListTextEditExtraSelection) -> () {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_append_QList_QTextEdit_ExtraSelection(original_self as *mut ::list::ListTextEditExtraSelection, t as *const ::list::ListTextEditExtraSelection) }
    }
  }
  impl<'largs> ListTextEditExtraSelectionAppendArgs<'largs> for &'largs ::text_edit::ExtraSelection {
    fn exec(self, original_self: &'largs mut ::list::ListTextEditExtraSelection) -> () {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_append_QTextEdit_ExtraSelection(original_self as *mut ::list::ListTextEditExtraSelection, t as *const ::text_edit::ExtraSelection) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextEditExtraSelection::mid](../struct.ListTextEditExtraSelection.html#method.mid) method.
  pub trait ListTextEditExtraSelectionMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextEditExtraSelection) -> ::list::ListTextEditExtraSelection;
  }
  impl<'largs> ListTextEditExtraSelectionMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListTextEditExtraSelection) -> ::list::ListTextEditExtraSelection {
      let pos = self;
      {
        let mut object: ::list::ListTextEditExtraSelection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_mid_to_output_pos(original_self as *const ::list::ListTextEditExtraSelection, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTextEditExtraSelectionMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTextEditExtraSelection) -> ::list::ListTextEditExtraSelection {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListTextEditExtraSelection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_mid_to_output_pos_length(original_self as *const ::list::ListTextEditExtraSelection, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextEditExtraSelection::new](../struct.ListTextEditExtraSelection.html#method.new) method.
  pub trait ListTextEditExtraSelectionNewArgs {
    fn exec(self) -> ::list::ListTextEditExtraSelection;
  }
  impl<'a> ListTextEditExtraSelectionNewArgs for &'a ::list::ListTextEditExtraSelection {
    fn exec(self) -> ::list::ListTextEditExtraSelection {
      let l = self;
      {
        let mut object: ::list::ListTextEditExtraSelection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_constructor_l(l as *const ::list::ListTextEditExtraSelection, &mut object);
        }
        object
      }
    }
  }
  impl ListTextEditExtraSelectionNewArgs for () {
    fn exec(self) -> ::list::ListTextEditExtraSelection {

      {
        let mut object: ::list::ListTextEditExtraSelection =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextEditExtraSelection::op_add_assign](../struct.ListTextEditExtraSelection.html#method.op_add_assign) method.
  pub trait ListTextEditExtraSelectionOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextEditExtraSelection)
            -> &'largs mut ::list::ListTextEditExtraSelection;
  }
  impl<'largs> ListTextEditExtraSelectionOpAddAssignArgs<'largs> for &'largs ::list::ListTextEditExtraSelection {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextEditExtraSelection)
            -> &'largs mut ::list::ListTextEditExtraSelection {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_add_assign_l(original_self as *mut ::list::ListTextEditExtraSelection, l as *const ::list::ListTextEditExtraSelection) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTextEditExtraSelectionOpAddAssignArgs<'largs> for &'largs ::text_edit::ExtraSelection {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextEditExtraSelection)
            -> &'largs mut ::list::ListTextEditExtraSelection {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_add_assign_t(original_self as *mut ::list::ListTextEditExtraSelection, t as *const ::text_edit::ExtraSelection) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextEditExtraSelection::op_shl](../struct.ListTextEditExtraSelection.html#method.op_shl) method.
  pub trait ListTextEditExtraSelectionOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextEditExtraSelection)
            -> &'largs mut ::list::ListTextEditExtraSelection;
  }
  impl<'largs> ListTextEditExtraSelectionOpShlArgs<'largs> for &'largs ::list::ListTextEditExtraSelection {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextEditExtraSelection)
            -> &'largs mut ::list::ListTextEditExtraSelection {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_shl_l(original_self as *mut ::list::ListTextEditExtraSelection, l as *const ::list::ListTextEditExtraSelection) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTextEditExtraSelectionOpShlArgs<'largs> for &'largs ::text_edit::ExtraSelection {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextEditExtraSelection)
            -> &'largs mut ::list::ListTextEditExtraSelection {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_operator_shl_t(original_self as *mut ::list::ListTextEditExtraSelection, t as *const ::text_edit::ExtraSelection) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextEditExtraSelection::swap](../struct.ListTextEditExtraSelection.html#method.swap) method.
  pub trait ListTextEditExtraSelectionSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextEditExtraSelection) -> ();
  }
  impl<'largs> ListTextEditExtraSelectionSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListTextEditExtraSelection) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_swap_i_j(original_self as *mut ::list::ListTextEditExtraSelection, i, j) }
    }
  }
  impl<'largs> ListTextEditExtraSelectionSwapArgs<'largs> for &'largs mut ::list::ListTextEditExtraSelection {
    fn exec(self, original_self: &'largs mut ::list::ListTextEditExtraSelection) -> () {
      let other = self;
      unsafe { ::ffi::qt_widgets_c_QList_QTextEdit_ExtraSelection_swap_other(original_self as *mut ::list::ListTextEditExtraSelection, other as *mut ::list::ListTextEditExtraSelection) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTreeWidgetItemMutPtr::index_of](../struct.ListTreeWidgetItemMutPtr.html#method.index_of) method.
  pub trait ListTreeWidgetItemMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListTreeWidgetItemMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListTreeWidgetItemMutPtrIndexOfArgs<'largs> for &'largs *mut ::tree_widget_item::TreeWidgetItem {
    unsafe fn exec(self, original_self: &'largs ::list::ListTreeWidgetItemMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_indexOf_t(original_self as *const ::list::ListTreeWidgetItemMutPtr, t as *const *mut ::tree_widget_item::TreeWidgetItem)
    }
  }
  impl<'largs> ListTreeWidgetItemMutPtrIndexOfArgs<'largs>
    for (&'largs *mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListTreeWidgetItemMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_indexOf_t_from(original_self as *const ::list::ListTreeWidgetItemMutPtr, t as *const *mut ::tree_widget_item::TreeWidgetItem, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListTreeWidgetItemMutPtr::last_index_of](../struct.ListTreeWidgetItemMutPtr.html#method.last_index_of) method.
  pub trait ListTreeWidgetItemMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListTreeWidgetItemMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListTreeWidgetItemMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::tree_widget_item::TreeWidgetItem {
    unsafe fn exec(self, original_self: &'largs ::list::ListTreeWidgetItemMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_lastIndexOf_t(original_self as *const ::list::ListTreeWidgetItemMutPtr, t as *const *mut ::tree_widget_item::TreeWidgetItem)
    }
  }
  impl<'largs> ListTreeWidgetItemMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::tree_widget_item::TreeWidgetItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListTreeWidgetItemMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_lastIndexOf_t_from(original_self as *const ::list::ListTreeWidgetItemMutPtr, t as *const *mut ::tree_widget_item::TreeWidgetItem, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListTreeWidgetItemMutPtr::mid](../struct.ListTreeWidgetItemMutPtr.html#method.mid) method.
  pub trait ListTreeWidgetItemMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTreeWidgetItemMutPtr) -> ::list::ListTreeWidgetItemMutPtr;
  }
  impl<'largs> ListTreeWidgetItemMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListTreeWidgetItemMutPtr) -> ::list::ListTreeWidgetItemMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListTreeWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_mid_to_output_pos(original_self as *const ::list::ListTreeWidgetItemMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTreeWidgetItemMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTreeWidgetItemMutPtr) -> ::list::ListTreeWidgetItemMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListTreeWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_mid_to_output_pos_length(original_self as *const ::list::ListTreeWidgetItemMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTreeWidgetItemMutPtr::new](../struct.ListTreeWidgetItemMutPtr.html#method.new) method.
  pub trait ListTreeWidgetItemMutPtrNewArgs {
    fn exec(self) -> ::list::ListTreeWidgetItemMutPtr;
  }
  impl<'a> ListTreeWidgetItemMutPtrNewArgs for &'a ::list::ListTreeWidgetItemMutPtr {
    fn exec(self) -> ::list::ListTreeWidgetItemMutPtr {
      let l = self;
      {
        let mut object: ::list::ListTreeWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_constructor_l(l as *const ::list::ListTreeWidgetItemMutPtr,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl ListTreeWidgetItemMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListTreeWidgetItemMutPtr {

      {
        let mut object: ::list::ListTreeWidgetItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTreeWidgetItemMutPtr::swap](../struct.ListTreeWidgetItemMutPtr.html#method.swap) method.
  pub trait ListTreeWidgetItemMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTreeWidgetItemMutPtr) -> ();
  }
  impl<'largs> ListTreeWidgetItemMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListTreeWidgetItemMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_swap_i_j(original_self as *mut ::list::ListTreeWidgetItemMutPtr,
                                                               i,
                                                               j)
      }
    }
  }
  impl<'largs> ListTreeWidgetItemMutPtrSwapArgs<'largs> for &'largs mut ::list::ListTreeWidgetItemMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListTreeWidgetItemMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QTreeWidgetItem_ptr_swap_other(original_self as *mut ::list::ListTreeWidgetItemMutPtr, other as *mut ::list::ListTreeWidgetItemMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListUndoStackMutPtr::index_of](../struct.ListUndoStackMutPtr.html#method.index_of) method.
  pub trait ListUndoStackMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListUndoStackMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListUndoStackMutPtrIndexOfArgs<'largs> for &'largs *mut ::undo_stack::UndoStack {
    unsafe fn exec(self, original_self: &'largs ::list::ListUndoStackMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QUndoStack_ptr_indexOf_t(original_self as *const ::list::ListUndoStackMutPtr,
                                                         t as *const *mut ::undo_stack::UndoStack)
    }
  }
  impl<'largs> ListUndoStackMutPtrIndexOfArgs<'largs> for (&'largs *mut ::undo_stack::UndoStack, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListUndoStackMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QUndoStack_ptr_indexOf_t_from(original_self as *const ::list::ListUndoStackMutPtr,
                                                              t as *const *mut ::undo_stack::UndoStack,
                                                              from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListUndoStackMutPtr::last_index_of](../struct.ListUndoStackMutPtr.html#method.last_index_of) method.
  pub trait ListUndoStackMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListUndoStackMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListUndoStackMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::undo_stack::UndoStack {
    unsafe fn exec(self, original_self: &'largs ::list::ListUndoStackMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QUndoStack_ptr_lastIndexOf_t(original_self as *const ::list::ListUndoStackMutPtr,
                                                             t as *const *mut ::undo_stack::UndoStack)
    }
  }
  impl<'largs> ListUndoStackMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::undo_stack::UndoStack, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListUndoStackMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QUndoStack_ptr_lastIndexOf_t_from(original_self as *const ::list::ListUndoStackMutPtr,
                                                                  t as *const *mut ::undo_stack::UndoStack,
                                                                  from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListUndoStackMutPtr::mid](../struct.ListUndoStackMutPtr.html#method.mid) method.
  pub trait ListUndoStackMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListUndoStackMutPtr) -> ::list::ListUndoStackMutPtr;
  }
  impl<'largs> ListUndoStackMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListUndoStackMutPtr) -> ::list::ListUndoStackMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListUndoStackMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QUndoStack_ptr_mid_to_output_pos(original_self as *const ::list::ListUndoStackMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListUndoStackMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListUndoStackMutPtr) -> ::list::ListUndoStackMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListUndoStackMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QUndoStack_ptr_mid_to_output_pos_length(original_self as *const ::list::ListUndoStackMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListUndoStackMutPtr::new](../struct.ListUndoStackMutPtr.html#method.new) method.
  pub trait ListUndoStackMutPtrNewArgs {
    fn exec(self) -> ::list::ListUndoStackMutPtr;
  }
  impl<'a> ListUndoStackMutPtrNewArgs for &'a ::list::ListUndoStackMutPtr {
    fn exec(self) -> ::list::ListUndoStackMutPtr {
      let l = self;
      {
        let mut object: ::list::ListUndoStackMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QUndoStack_ptr_constructor_l(l as *const ::list::ListUndoStackMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListUndoStackMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListUndoStackMutPtr {

      {
        let mut object: ::list::ListUndoStackMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QUndoStack_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListUndoStackMutPtr::swap](../struct.ListUndoStackMutPtr.html#method.swap) method.
  pub trait ListUndoStackMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListUndoStackMutPtr) -> ();
  }
  impl<'largs> ListUndoStackMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListUndoStackMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QUndoStack_ptr_swap_i_j(original_self as *mut ::list::ListUndoStackMutPtr, i, j)
      }
    }
  }
  impl<'largs> ListUndoStackMutPtrSwapArgs<'largs> for &'largs mut ::list::ListUndoStackMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListUndoStackMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QUndoStack_ptr_swap_other(original_self as *mut ::list::ListUndoStackMutPtr,
                                                            other as *mut ::list::ListUndoStackMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWidgetMutPtr::index_of](../struct.ListWidgetMutPtr.html#method.index_of) method.
  pub trait ListWidgetMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListWidgetMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListWidgetMutPtrIndexOfArgs<'largs> for &'largs *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs ::list::ListWidgetMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QWidget_ptr_indexOf_t(original_self as *const ::list::ListWidgetMutPtr,
                                                      t as *const *mut ::widget::Widget)
    }
  }
  impl<'largs> ListWidgetMutPtrIndexOfArgs<'largs> for (&'largs *mut ::widget::Widget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListWidgetMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QWidget_ptr_indexOf_t_from(original_self as *const ::list::ListWidgetMutPtr,
                                                           t as *const *mut ::widget::Widget,
                                                           from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListWidgetMutPtr::last_index_of](../struct.ListWidgetMutPtr.html#method.last_index_of) method.
  pub trait ListWidgetMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListWidgetMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListWidgetMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::widget::Widget {
    unsafe fn exec(self, original_self: &'largs ::list::ListWidgetMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QList_QWidget_ptr_lastIndexOf_t(original_self as *const ::list::ListWidgetMutPtr,
                                                          t as *const *mut ::widget::Widget)
    }
  }
  impl<'largs> ListWidgetMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::widget::Widget, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListWidgetMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QList_QWidget_ptr_lastIndexOf_t_from(original_self as *const ::list::ListWidgetMutPtr,
                                                               t as *const *mut ::widget::Widget,
                                                               from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListWidgetMutPtr::mid](../struct.ListWidgetMutPtr.html#method.mid) method.
  pub trait ListWidgetMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListWidgetMutPtr) -> ::list::ListWidgetMutPtr;
  }
  impl<'largs> ListWidgetMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListWidgetMutPtr) -> ::list::ListWidgetMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QWidget_ptr_mid_to_output_pos(original_self as *const ::list::ListWidgetMutPtr,
                                                                  pos,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListWidgetMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListWidgetMutPtr) -> ::list::ListWidgetMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QWidget_ptr_mid_to_output_pos_length(original_self as *const ::list::ListWidgetMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWidgetMutPtr::new](../struct.ListWidgetMutPtr.html#method.new) method.
  pub trait ListWidgetMutPtrNewArgs {
    fn exec(self) -> ::list::ListWidgetMutPtr;
  }
  impl<'a> ListWidgetMutPtrNewArgs for &'a ::list::ListWidgetMutPtr {
    fn exec(self) -> ::list::ListWidgetMutPtr {
      let l = self;
      {
        let mut object: ::list::ListWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QWidget_ptr_constructor_l(l as *const ::list::ListWidgetMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListWidgetMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListWidgetMutPtr {

      {
        let mut object: ::list::ListWidgetMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QWidget_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWidgetMutPtr::swap](../struct.ListWidgetMutPtr.html#method.swap) method.
  pub trait ListWidgetMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListWidgetMutPtr) -> ();
  }
  impl<'largs> ListWidgetMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListWidgetMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QWidget_ptr_swap_i_j(original_self as *mut ::list::ListWidgetMutPtr, i, j) }
    }
  }
  impl<'largs> ListWidgetMutPtrSwapArgs<'largs> for &'largs mut ::list::ListWidgetMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListWidgetMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QWidget_ptr_swap_other(original_self as *mut ::list::ListWidgetMutPtr,
                                                         other as *mut ::list::ListWidgetMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWizardWizardButton::append](../struct.ListWizardWizardButton.html#method.append) method.
  pub trait ListWizardWizardButtonAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListWizardWizardButton) -> ();
  }
  impl<'largs> ListWizardWizardButtonAppendArgs<'largs> for &'largs ::list::ListWizardWizardButton {
    fn exec(self, original_self: &'largs mut ::list::ListWizardWizardButton) -> () {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_append_QList_QWizard_WizardButton(original_self as *mut ::list::ListWizardWizardButton, t as *const ::list::ListWizardWizardButton) }
    }
  }
  impl<'largs> ListWizardWizardButtonAppendArgs<'largs> for &'largs ::wizard::WizardButton {
    fn exec(self, original_self: &'largs mut ::list::ListWizardWizardButton) -> () {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_append_QWizard_WizardButton(original_self as *mut ::list::ListWizardWizardButton, t as *const ::wizard::WizardButton) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWizardWizardButton::count](../struct.ListWizardWizardButton.html#method.count) method.
  pub trait ListWizardWizardButtonCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::libc::c_int;
  }
  impl<'largs> ListWizardWizardButtonCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_count_no_args(original_self as *const ::list::ListWizardWizardButton) }
    }
  }
  impl<'largs> ListWizardWizardButtonCountArgs<'largs> for &'largs ::wizard::WizardButton {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_count_t(original_self as *const ::list::ListWizardWizardButton,
                                                               t as *const ::wizard::WizardButton)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWizardWizardButton::index_of](../struct.ListWizardWizardButton.html#method.index_of) method.
  pub trait ListWizardWizardButtonIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::libc::c_int;
  }
  impl<'largs> ListWizardWizardButtonIndexOfArgs<'largs> for &'largs ::wizard::WizardButton {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_indexOf_t(original_self as *const ::list::ListWizardWizardButton, t as *const ::wizard::WizardButton)
      }
    }
  }
  impl<'largs> ListWizardWizardButtonIndexOfArgs<'largs> for (&'largs ::wizard::WizardButton, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_indexOf_t_from(original_self as *const ::list::ListWizardWizardButton, t as *const ::wizard::WizardButton, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWizardWizardButton::last_index_of](../struct.ListWizardWizardButton.html#method.last_index_of) method.
  pub trait ListWizardWizardButtonLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::libc::c_int;
  }
  impl<'largs> ListWizardWizardButtonLastIndexOfArgs<'largs> for &'largs ::wizard::WizardButton {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_lastIndexOf_t(original_self as *const ::list::ListWizardWizardButton, t as *const ::wizard::WizardButton) }
    }
  }
  impl<'largs> ListWizardWizardButtonLastIndexOfArgs<'largs> for (&'largs ::wizard::WizardButton, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_lastIndexOf_t_from(original_self as *const ::list::ListWizardWizardButton, t as *const ::wizard::WizardButton, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWizardWizardButton::mid](../struct.ListWizardWizardButton.html#method.mid) method.
  pub trait ListWizardWizardButtonMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::list::ListWizardWizardButton;
  }
  impl<'largs> ListWizardWizardButtonMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::list::ListWizardWizardButton {
      let pos = self;
      {
        let mut object: ::list::ListWizardWizardButton =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QWizard_WizardButton_mid_to_output_pos(original_self as *const ::list::ListWizardWizardButton, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListWizardWizardButtonMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::list::ListWizardWizardButton {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListWizardWizardButton =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QWizard_WizardButton_mid_to_output_pos_length(original_self as *const ::list::ListWizardWizardButton, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWizardWizardButton::new](../struct.ListWizardWizardButton.html#method.new) method.
  pub trait ListWizardWizardButtonNewArgs {
    fn exec(self) -> ::list::ListWizardWizardButton;
  }
  impl<'a> ListWizardWizardButtonNewArgs for &'a ::list::ListWizardWizardButton {
    fn exec(self) -> ::list::ListWizardWizardButton {
      let l = self;
      {
        let mut object: ::list::ListWizardWizardButton =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QWizard_WizardButton_constructor_l(l as *const ::list::ListWizardWizardButton,
                                                                       &mut object);
        }
        object
      }
    }
  }
  impl ListWizardWizardButtonNewArgs for () {
    fn exec(self) -> ::list::ListWizardWizardButton {

      {
        let mut object: ::list::ListWizardWizardButton =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QList_QWizard_WizardButton_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWizardWizardButton::op_add_assign](../struct.ListWizardWizardButton.html#method.op_add_assign) method.
  pub trait ListWizardWizardButtonOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListWizardWizardButton)
            -> &'largs mut ::list::ListWizardWizardButton;
  }
  impl<'largs> ListWizardWizardButtonOpAddAssignArgs<'largs> for &'largs ::list::ListWizardWizardButton {
    fn exec(self,
            original_self: &'largs mut ::list::ListWizardWizardButton)
            -> &'largs mut ::list::ListWizardWizardButton {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_operator_add_assign_l(original_self as *mut ::list::ListWizardWizardButton, l as *const ::list::ListWizardWizardButton) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListWizardWizardButtonOpAddAssignArgs<'largs> for &'largs ::wizard::WizardButton {
    fn exec(self,
            original_self: &'largs mut ::list::ListWizardWizardButton)
            -> &'largs mut ::list::ListWizardWizardButton {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_operator_add_assign_t(original_self as *mut ::list::ListWizardWizardButton, t as *const ::wizard::WizardButton) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListWizardWizardButton::op_shl](../struct.ListWizardWizardButton.html#method.op_shl) method.
  pub trait ListWizardWizardButtonOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListWizardWizardButton)
            -> &'largs mut ::list::ListWizardWizardButton;
  }
  impl<'largs> ListWizardWizardButtonOpShlArgs<'largs> for &'largs ::list::ListWizardWizardButton {
    fn exec(self,
            original_self: &'largs mut ::list::ListWizardWizardButton)
            -> &'largs mut ::list::ListWizardWizardButton {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_operator_shl_l(original_self as *mut ::list::ListWizardWizardButton, l as *const ::list::ListWizardWizardButton) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListWizardWizardButtonOpShlArgs<'largs> for &'largs ::wizard::WizardButton {
    fn exec(self,
            original_self: &'largs mut ::list::ListWizardWizardButton)
            -> &'largs mut ::list::ListWizardWizardButton {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_operator_shl_t(original_self as *mut ::list::ListWizardWizardButton, t as *const ::wizard::WizardButton) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListWizardWizardButton::swap](../struct.ListWizardWizardButton.html#method.swap) method.
  pub trait ListWizardWizardButtonSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListWizardWizardButton) -> ();
  }
  impl<'largs> ListWizardWizardButtonSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListWizardWizardButton) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_swap_i_j(original_self as *mut ::list::ListWizardWizardButton,
                                                                i,
                                                                j)
      }
    }
  }
  impl<'largs> ListWizardWizardButtonSwapArgs<'largs> for &'largs mut ::list::ListWizardWizardButton {
    fn exec(self, original_self: &'largs mut ::list::ListWizardWizardButton) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_swap_other(original_self as *mut ::list::ListWizardWizardButton, other as *mut ::list::ListWizardWizardButton)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWizardWizardButton::value](../struct.ListWizardWizardButton.html#method.value) method.
  pub trait ListWizardWizardButtonValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::wizard::WizardButton;
  }
  impl<'largs> ListWizardWizardButtonValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::wizard::WizardButton {
      let i = self;
      unsafe {
        ::ffi::qt_widgets_c_QList_QWizard_WizardButton_value_i(original_self as *const ::list::ListWizardWizardButton,
                                                               i)
      }
    }
  }
  impl<'largs> ListWizardWizardButtonValueArgs<'largs> for (::libc::c_int, &'largs ::wizard::WizardButton) {
    fn exec(self, original_self: &'largs ::list::ListWizardWizardButton) -> ::wizard::WizardButton {
      let i = self.0;
      let default_value = self.1;
      unsafe { ::ffi::qt_widgets_c_QList_QWizard_WizardButton_value_i_defaultValue(original_self as *const ::list::ListWizardWizardButton, i, default_value as *const ::wizard::WizardButton) }
    }
  }
}
