/// C++ type: <span style='color: green;'>```QList<QAccessibleInterface*>```</span>
#[repr(C)]
pub struct ListAccessibleInterfaceMutPtr([u8; ::type_sizes::QT_GUI_LIST_LIST_ACCESSIBLE_INTERFACE_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListAccessibleInterfaceMutPtr {
  unsafe fn new_uninitialized() -> ListAccessibleInterfaceMutPtr {
    ListAccessibleInterfaceMutPtr(::std::mem::uninitialized())
  }
}

impl ListAccessibleInterfaceMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::append(const QList<QAccessibleInterface*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListAccessibleInterfaceMutPtr) {
    unsafe { ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_append_QList_QAccessibleInterface_ptr(self as *mut ::list::ListAccessibleInterfaceMutPtr, t as *const ::list::ListAccessibleInterfaceMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::append(QAccessibleInterface* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::accessible_interface::AccessibleInterface) {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_append_QAccessibleInterface(self as *mut ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* const & QList<QAccessibleInterface*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::accessible_interface::AccessibleInterface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_at(self as *const ::list::ListAccessibleInterfaceMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* const & QList<QAccessibleInterface*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::accessible_interface::AccessibleInterface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_back_const(self as *const ::list::ListAccessibleInterfaceMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface*& QList<QAccessibleInterface*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::accessible_interface::AccessibleInterface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_back(self as *mut ::list::ListAccessibleInterfaceMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_clear(self as *mut ::list::ListAccessibleInterfaceMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* const & QList<QAccessibleInterface*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::accessible_interface::AccessibleInterface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_constFirst(self as *const ::list::ListAccessibleInterfaceMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* const & QList<QAccessibleInterface*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::accessible_interface::AccessibleInterface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_constLast(self as *const ::list::ListAccessibleInterfaceMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAccessibleInterface*>::contains(QAccessibleInterface* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::accessible_interface::AccessibleInterface) -> bool {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_contains(self as *const ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAccessibleInterface*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_count_no_args(self as *const ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAccessibleInterface*>::count(QAccessibleInterface* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::accessible_interface::AccessibleInterface) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_count_t(self as *const ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAccessibleInterface*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_empty(self as *const ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAccessibleInterface*>::endsWith(QAccessibleInterface* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::accessible_interface::AccessibleInterface) -> bool {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_endsWith(self as *const ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* const & QList<QAccessibleInterface*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::accessible_interface::AccessibleInterface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_first_const(self as *const ::list::ListAccessibleInterfaceMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface*& QList<QAccessibleInterface*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::accessible_interface::AccessibleInterface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_first(self as *mut ::list::ListAccessibleInterfaceMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* const & QList<QAccessibleInterface*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::accessible_interface::AccessibleInterface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_front_const(self as *const ::list::ListAccessibleInterfaceMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface*& QList<QAccessibleInterface*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::accessible_interface::AccessibleInterface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_front(self as *mut ::list::ListAccessibleInterfaceMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::accessible_interface::AccessibleInterface) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAccessibleInterface*>::indexOf(QAccessibleInterface* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::accessible_interface::AccessibleInterface, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAccessibleInterface*>::indexOf(QAccessibleInterface* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListAccessibleInterfaceMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::insert(int i, QAccessibleInterface* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::accessible_interface::AccessibleInterface) {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_insert(self as *mut ::list::ListAccessibleInterfaceMutPtr, i, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAccessibleInterface*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_isEmpty(self as *const ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* const & QList<QAccessibleInterface*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::accessible_interface::AccessibleInterface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_last_const(self as *const ::list::ListAccessibleInterfaceMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::accessible_interface::AccessibleInterface) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAccessibleInterface*>::lastIndexOf(QAccessibleInterface* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::accessible_interface::AccessibleInterface, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QAccessibleInterface*>::lastIndexOf(QAccessibleInterface* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListAccessibleInterfaceMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAccessibleInterface*& QList<QAccessibleInterface*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::accessible_interface::AccessibleInterface {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_last(self as *mut ::list::ListAccessibleInterfaceMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QAccessibleInterface*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_length(self as *const ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListAccessibleInterfaceMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*> QList<QAccessibleInterface*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListAccessibleInterfaceMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*> QList<QAccessibleInterface*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListAccessibleInterfaceMutPtr
    where Args: overloading::ListAccessibleInterfaceMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_move(self as *mut ::list::ListAccessibleInterfaceMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListAccessibleInterfaceMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAccessibleInterface*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListAccessibleInterfaceMutPtr) -> ::list::ListAccessibleInterfaceMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QAccessibleInterface*>::QList(const QList<QAccessibleInterface*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListAccessibleInterfaceMutPtr
    where Args: overloading::ListAccessibleInterfaceMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*> QList<QAccessibleInterface*>::operator+(const QList<QAccessibleInterface*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListAccessibleInterfaceMutPtr) -> ::list::ListAccessibleInterfaceMutPtr {
    {
      let mut object: ::list::ListAccessibleInterfaceMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_operator_add_to_output(self as *const ::list::ListAccessibleInterfaceMutPtr, l as *const ::list::ListAccessibleInterfaceMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*>& QList<QAccessibleInterface*>::operator+=(const QList<QAccessibleInterface*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListAccessibleInterfaceMutPtr)
                                 -> &'l0 mut ::list::ListAccessibleInterfaceMutPtr {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_operator_add_assign_l(self as *mut ::list::ListAccessibleInterfaceMutPtr, l as *const ::list::ListAccessibleInterfaceMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*>& QList<QAccessibleInterface*>::operator+=(QAccessibleInterface* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::accessible_interface::AccessibleInterface)
                                               -> &'l0 mut ::list::ListAccessibleInterfaceMutPtr {
    let ffi_result = ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_operator_add_assign_t(self as *mut ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*>& QList<QAccessibleInterface*>::operator=(const QList<QAccessibleInterface*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListAccessibleInterfaceMutPtr)
                             -> &'l0 mut ::list::ListAccessibleInterfaceMutPtr {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_operator_assign(self as *mut ::list::ListAccessibleInterfaceMutPtr, l as *const ::list::ListAccessibleInterfaceMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAccessibleInterface*>::operator==(const QList<QAccessibleInterface*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListAccessibleInterfaceMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_operator_eq(self as *const ::list::ListAccessibleInterfaceMutPtr,
                                                                 l as *const ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* const & QList<QAccessibleInterface*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::accessible_interface::AccessibleInterface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_operator_index_const(self as *const ::list::ListAccessibleInterfaceMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface*& QList<QAccessibleInterface*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self,
                           i: ::libc::c_int)
                           -> &'l0 mut *mut ::accessible_interface::AccessibleInterface {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_operator_index(self as *mut ::list::ListAccessibleInterfaceMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAccessibleInterface*>::operator!=(const QList<QAccessibleInterface*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListAccessibleInterfaceMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_operator_neq(self as *const ::list::ListAccessibleInterfaceMutPtr, l as *const ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*>& QList<QAccessibleInterface*>::operator<<(const QList<QAccessibleInterface*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListAccessibleInterfaceMutPtr)
                          -> &'l0 mut ::list::ListAccessibleInterfaceMutPtr {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_operator_shl_l(self as *mut ::list::ListAccessibleInterfaceMutPtr, l as *const ::list::ListAccessibleInterfaceMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*>& QList<QAccessibleInterface*>::operator<<(QAccessibleInterface* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::accessible_interface::AccessibleInterface)
                                        -> &'l0 mut ::list::ListAccessibleInterfaceMutPtr {
    let ffi_result = ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_operator_shl_t(self as *mut ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_pop_back(self as *mut ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_pop_front(self as *mut ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::prepend(QAccessibleInterface* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::accessible_interface::AccessibleInterface) {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_prepend(self as *mut ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::push_back(QAccessibleInterface* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::accessible_interface::AccessibleInterface) {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_push_back(self as *mut ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::push_front(QAccessibleInterface* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::accessible_interface::AccessibleInterface) {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_push_front(self as *mut ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```int QList<QAccessibleInterface*>::removeAll(QAccessibleInterface* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::accessible_interface::AccessibleInterface) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_removeAll(self as *mut ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_removeAt(self as *mut ::list::ListAccessibleInterfaceMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_removeFirst(self as *mut ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_removeLast(self as *mut ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAccessibleInterface*>::removeOne(QAccessibleInterface* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::accessible_interface::AccessibleInterface) -> bool {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_removeOne(self as *mut ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::replace(int i, QAccessibleInterface* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::accessible_interface::AccessibleInterface) {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_replace(self as *mut ::list::ListAccessibleInterfaceMutPtr, i, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_reserve(self as *mut ::list::ListAccessibleInterfaceMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QAccessibleInterface*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_size(self as *const ::list::ListAccessibleInterfaceMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QAccessibleInterface*>::startsWith(QAccessibleInterface* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::accessible_interface::AccessibleInterface) -> bool {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_startsWith(self as *const ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }

  /// C++ method: <span style='color: green;'>```QList<QAccessibleInterface*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListAccessibleInterfaceMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::swap(QList<QAccessibleInterface*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QAccessibleInterface*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListAccessibleInterfaceMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QAccessibleInterface* QList<QAccessibleInterface*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_takeAt(self as *mut ::list::ListAccessibleInterfaceMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* QList<QAccessibleInterface*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_takeFirst(self as *mut ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* QList<QAccessibleInterface*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_takeLast(self as *mut ::list::ListAccessibleInterfaceMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* QList<QAccessibleInterface*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::accessible_interface::AccessibleInterface {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_value_i(self as *const ::list::ListAccessibleInterfaceMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```QAccessibleInterface* QList<QAccessibleInterface*>::value(int i, QAccessibleInterface* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::accessible_interface::AccessibleInterface)
                             -> *mut ::accessible_interface::AccessibleInterface {
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_value_i_defaultValue(self as *const ::list::ListAccessibleInterfaceMutPtr, i, default_value as *const *mut ::accessible_interface::AccessibleInterface)
  }
}

impl Drop for ::list::ListAccessibleInterfaceMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QAccessibleInterface*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_destructor(self as *mut ::list::ListAccessibleInterfaceMutPtr)
    }
  }
}

/// C++ type: <span style='color: green;'>```QList<double>```</span>
#[repr(C)]
pub struct ListCDouble([u8; ::type_sizes::QT_GUI_LIST_LIST_C_DOUBLE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListCDouble {
  unsafe fn new_uninitialized() -> ListCDouble {
    ListCDouble(::std::mem::uninitialized())
  }
}

impl ListCDouble {
  /// C++ method: <span style='color: green;'>```QList<double>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListCDouble) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<double>::append(const QList<double>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::libc::c_double) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<double>::append(const double& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListCDoubleAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const double& QList<double>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_at(self as *const ::list::ListCDouble, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const double& QList<double>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_back_const(self as *const ::list::ListCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QList<double>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_back(self as *mut ::list::ListCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<double>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_double_clear(self as *mut ::list::ListCDouble) }
  }

  /// C++ method: <span style='color: green;'>```const double& QList<double>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_constFirst(self as *const ::list::ListCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const double& QList<double>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_constLast(self as *const ::list::ListCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<double>::contains(const double& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::libc::c_double) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_double_contains(self as *const ::list::ListCDouble,
                                            t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<double>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<double>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::libc::c_double) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<double>::count(const double& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListCDoubleCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<double>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_double_empty(self as *const ::list::ListCDouble) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<double>::endsWith(const double& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::libc::c_double) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_double_endsWith(self as *const ::list::ListCDouble,
                                            t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```const double& QList<double>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_first_const(self as *const ::list::ListCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QList<double>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_first(self as *mut ::list::ListCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QList<double> QList<double>::fromVector(const QVector<double>& vector)```</span>
  ///
  ///
  pub fn from_vector(vector: &::vector::VectorCDouble) -> ::list::ListCDouble {
    {
      let mut object: ::list::ListCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_double_fromVector_to_output(vector as *const ::vector::VectorCDouble, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const double& QList<double>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_front_const(self as *const ::list::ListCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QList<double>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_front(self as *mut ::list::ListCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<double>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::libc::c_double) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<double>::indexOf(const double& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::libc::c_double, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<double>::indexOf(const double& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListCDoubleIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<double>::insert(int i, const double& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QList_double_insert(self as *mut ::list::ListCDouble,
                                          i,
                                          t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<double>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_double_isEmpty(self as *const ::list::ListCDouble) }
  }

  /// C++ method: <span style='color: green;'>```const double& QList<double>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_last_const(self as *const ::list::ListCDouble) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<double>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::libc::c_double) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<double>::lastIndexOf(const double& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::libc::c_double, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<double>::lastIndexOf(const double& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListCDoubleLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double& QList<double>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_last(self as *mut ::list::ListCDouble) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<double>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_double_length(self as *const ::list::ListCDouble) }
  }

  /// C++ method: <span style='color: green;'>```QList<double>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<double> QList<double>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<double> QList<double>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListCDouble
    where Args: overloading::ListCDoubleMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<double>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_double_move(self as *mut ::list::ListCDouble, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<double>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListCDouble```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<double>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListCDouble) -> ::list::ListCDouble```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<double>::QList(const QList<double>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListCDouble
    where Args: overloading::ListCDoubleNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<double> QList<double>::operator+(const QList<double>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListCDouble) -> ::list::ListCDouble {
    {
      let mut object: ::list::ListCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_double_operator_add_to_output(self as *const ::list::ListCDouble,
                                                            l as *const ::list::ListCDouble,
                                                            &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<double>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListCDouble) -> &'l0 mut ::list::ListCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<double>& QList<double>::operator+=(const QList<double>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::libc::c_double) -> &'l0 mut ::list::ListCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<double>& QList<double>::operator+=(const double& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListCDouble
    where Args: overloading::ListCDoubleOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<double>& QList<double>::operator=(const QList<double>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListCDouble) -> &'l0 mut ::list::ListCDouble {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_double_operator_assign(self as *mut ::list::ListCDouble,
                                                   l as *const ::list::ListCDouble)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<double>::operator==(const QList<double>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListCDouble) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_double_operator_eq(self as *const ::list::ListCDouble,
                                               l as *const ::list::ListCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```const double& QList<double>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::libc::c_double {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_double_operator_index_const(self as *const ::list::ListCDouble, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```double& QList<double>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::libc::c_double {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_double_operator_index(self as *mut ::list::ListCDouble, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<double>::operator!=(const QList<double>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListCDouble) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_double_operator_neq(self as *const ::list::ListCDouble,
                                                l as *const ::list::ListCDouble)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<double>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListCDouble) -> &'l0 mut ::list::ListCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<double>& QList<double>::operator<<(const QList<double>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::libc::c_double) -> &'l0 mut ::list::ListCDouble```<br>
  /// C++ method: <span style='color: green;'>```QList<double>& QList<double>::operator<<(const double& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListCDouble
    where Args: overloading::ListCDoubleOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<double>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_double_pop_back(self as *mut ::list::ListCDouble) }
  }

  /// C++ method: <span style='color: green;'>```void QList<double>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_double_pop_front(self as *mut ::list::ListCDouble) }
  }

  /// C++ method: <span style='color: green;'>```void QList<double>::prepend(const double& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QList_double_prepend(self as *mut ::list::ListCDouble,
                                           t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<double>::push_back(const double& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QList_double_push_back(self as *mut ::list::ListCDouble,
                                             t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<double>::push_front(const double& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QList_double_push_front(self as *mut ::list::ListCDouble,
                                              t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<double>::removeAll(const double& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::libc::c_double) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_double_removeAll(self as *mut ::list::ListCDouble,
                                             t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<double>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_double_removeAt(self as *mut ::list::ListCDouble, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<double>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_double_removeFirst(self as *mut ::list::ListCDouble) }
  }

  /// C++ method: <span style='color: green;'>```void QList<double>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_double_removeLast(self as *mut ::list::ListCDouble) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<double>::removeOne(const double& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::libc::c_double) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_double_removeOne(self as *mut ::list::ListCDouble,
                                             t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<double>::replace(int i, const double& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::libc::c_double) {
    unsafe {
      ::ffi::qt_gui_c_QList_double_replace(self as *mut ::list::ListCDouble,
                                           i,
                                           t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<double>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_double_reserve(self as *mut ::list::ListCDouble, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<double>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_double_size(self as *const ::list::ListCDouble) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<double>::startsWith(const double& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::libc::c_double) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_double_startsWith(self as *const ::list::ListCDouble,
                                              t as *const ::libc::c_double)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<double>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListCDouble) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<double>::swap(QList<double>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<double>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListCDoubleSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```double QList<double>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QList_double_takeAt(self as *mut ::list::ListCDouble, i) }
  }

  /// C++ method: <span style='color: green;'>```double QList<double>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QList_double_takeFirst(self as *mut ::list::ListCDouble) }
  }

  /// C++ method: <span style='color: green;'>```double QList<double>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::libc::c_double {
    unsafe { ::ffi::qt_gui_c_QList_double_takeLast(self as *mut ::list::ListCDouble) }
  }

  /// C++ method: <span style='color: green;'>```QVector<double> QList<double>::toVector() const```</span>
  ///
  ///
  pub fn to_vector(&self) -> ::vector::VectorCDouble {
    {
      let mut object: ::vector::VectorCDouble =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_double_toVector_to_output(self as *const ::list::ListCDouble, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<double>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QList<double>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::libc::c_double)) -> ::libc::c_double```<br>
  /// C++ method: <span style='color: green;'>```double QList<double>::value(int i, const double& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::libc::c_double
    where Args: overloading::ListCDoubleValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListCDouble {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<double>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_double_destructor(self as *mut ::list::ListCDouble) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>```</span>
#[repr(C)]
pub struct ListFontDatabaseWritingSystem([u8; ::type_sizes::QT_GUI_LIST_LIST_FONT_DATABASE_WRITING_SYSTEM]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListFontDatabaseWritingSystem {
  unsafe fn new_uninitialized() -> ListFontDatabaseWritingSystem {
    ListFontDatabaseWritingSystem(::std::mem::uninitialized())
  }
}

impl ListFontDatabaseWritingSystem {
  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::font_database::WritingSystem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::append(const QFontDatabase::WritingSystem& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListFontDatabaseWritingSystem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::append(const QList<QFontDatabase::WritingSystem>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListFontDatabaseWritingSystemAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::font_database::WritingSystem {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_at(self as *const ::list::ListFontDatabaseWritingSystem, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::font_database::WritingSystem {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_back_const(self as *const ::list::ListFontDatabaseWritingSystem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::font_database::WritingSystem {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_back(self as *mut ::list::ListFontDatabaseWritingSystem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_clear(self as *mut ::list::ListFontDatabaseWritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```const QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::font_database::WritingSystem {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_constFirst(self as *const ::list::ListFontDatabaseWritingSystem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::font_database::WritingSystem {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_constLast(self as *const ::list::ListFontDatabaseWritingSystem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFontDatabase::WritingSystem>::contains(const QFontDatabase::WritingSystem& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::font_database::WritingSystem) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_contains(self as *const ::list::ListFontDatabaseWritingSystem,
                                                                 t as *const ::font_database::WritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFontDatabase::WritingSystem>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::font_database::WritingSystem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFontDatabase::WritingSystem>::count(const QFontDatabase::WritingSystem& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListFontDatabaseWritingSystemCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QFontDatabase::WritingSystem>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_empty(self as *const ::list::ListFontDatabaseWritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFontDatabase::WritingSystem>::endsWith(const QFontDatabase::WritingSystem& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::font_database::WritingSystem) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_endsWith(self as *const ::list::ListFontDatabaseWritingSystem,
                                                                 t as *const ::font_database::WritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```const QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::font_database::WritingSystem {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_first_const(self as *const ::list::ListFontDatabaseWritingSystem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::font_database::WritingSystem {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_first(self as *mut ::list::ListFontDatabaseWritingSystem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::font_database::WritingSystem {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_front_const(self as *const ::list::ListFontDatabaseWritingSystem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::font_database::WritingSystem {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_front(self as *mut ::list::ListFontDatabaseWritingSystem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::font_database::WritingSystem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFontDatabase::WritingSystem>::indexOf(const QFontDatabase::WritingSystem& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::font_database::WritingSystem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFontDatabase::WritingSystem>::indexOf(const QFontDatabase::WritingSystem& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListFontDatabaseWritingSystemIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::insert(int i, const QFontDatabase::WritingSystem& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::font_database::WritingSystem) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_insert(self as *mut ::list::ListFontDatabaseWritingSystem,
                                                               i,
                                                               t as *const ::font_database::WritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFontDatabase::WritingSystem>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_isEmpty(self as *const ::list::ListFontDatabaseWritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```const QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::font_database::WritingSystem {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_last_const(self as *const ::list::ListFontDatabaseWritingSystem) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::font_database::WritingSystem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFontDatabase::WritingSystem>::lastIndexOf(const QFontDatabase::WritingSystem& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::font_database::WritingSystem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QFontDatabase::WritingSystem>::lastIndexOf(const QFontDatabase::WritingSystem& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListFontDatabaseWritingSystemLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::font_database::WritingSystem {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_last(self as *mut ::list::ListFontDatabaseWritingSystem)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QFontDatabase::WritingSystem>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_length(self as *const ::list::ListFontDatabaseWritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListFontDatabaseWritingSystem```<br>
  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem> QList<QFontDatabase::WritingSystem>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListFontDatabaseWritingSystem```<br>
  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem> QList<QFontDatabase::WritingSystem>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListFontDatabaseWritingSystem
    where Args: overloading::ListFontDatabaseWritingSystemMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_move(self as *mut ::list::ListFontDatabaseWritingSystem,
                                                             from,
                                                             to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListFontDatabaseWritingSystem```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QFontDatabase::WritingSystem>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListFontDatabaseWritingSystem) -> ::list::ListFontDatabaseWritingSystem```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QFontDatabase::WritingSystem>::QList(const QList<QFontDatabase::WritingSystem>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListFontDatabaseWritingSystem
    where Args: overloading::ListFontDatabaseWritingSystemNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem> QList<QFontDatabase::WritingSystem>::operator+(const QList<QFontDatabase::WritingSystem>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListFontDatabaseWritingSystem) -> ::list::ListFontDatabaseWritingSystem {
    {
      let mut object: ::list::ListFontDatabaseWritingSystem =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_operator_add_to_output(self as *const ::list::ListFontDatabaseWritingSystem, l as *const ::list::ListFontDatabaseWritingSystem, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::font_database::WritingSystem) -> &'l0 mut ::list::ListFontDatabaseWritingSystem```<br>
  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>& QList<QFontDatabase::WritingSystem>::operator+=(const QFontDatabase::WritingSystem& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListFontDatabaseWritingSystem) -> &'l0 mut ::list::ListFontDatabaseWritingSystem```<br>
  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>& QList<QFontDatabase::WritingSystem>::operator+=(const QList<QFontDatabase::WritingSystem>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListFontDatabaseWritingSystem
    where Args: overloading::ListFontDatabaseWritingSystemOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>& QList<QFontDatabase::WritingSystem>::operator=(const QList<QFontDatabase::WritingSystem>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListFontDatabaseWritingSystem)
                             -> &'l0 mut ::list::ListFontDatabaseWritingSystem {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_operator_assign(self as *mut ::list::ListFontDatabaseWritingSystem, l as *const ::list::ListFontDatabaseWritingSystem) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFontDatabase::WritingSystem>::operator==(const QList<QFontDatabase::WritingSystem>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListFontDatabaseWritingSystem) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_operator_eq(self as *const ::list::ListFontDatabaseWritingSystem, l as *const ::list::ListFontDatabaseWritingSystem) }
  }

  /// C++ method: <span style='color: green;'>```const QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::font_database::WritingSystem {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_operator_index_const(self as *const ::list::ListFontDatabaseWritingSystem, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::WritingSystem& QList<QFontDatabase::WritingSystem>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::font_database::WritingSystem {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_operator_index(self as *mut ::list::ListFontDatabaseWritingSystem, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFontDatabase::WritingSystem>::operator!=(const QList<QFontDatabase::WritingSystem>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListFontDatabaseWritingSystem) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_operator_neq(self as *const ::list::ListFontDatabaseWritingSystem, l as *const ::list::ListFontDatabaseWritingSystem) }
  }

  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::font_database::WritingSystem) -> &'l0 mut ::list::ListFontDatabaseWritingSystem```<br>
  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>& QList<QFontDatabase::WritingSystem>::operator<<(const QFontDatabase::WritingSystem& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListFontDatabaseWritingSystem) -> &'l0 mut ::list::ListFontDatabaseWritingSystem```<br>
  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>& QList<QFontDatabase::WritingSystem>::operator<<(const QList<QFontDatabase::WritingSystem>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListFontDatabaseWritingSystem
    where Args: overloading::ListFontDatabaseWritingSystemOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_pop_back(self as *mut ::list::ListFontDatabaseWritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_pop_front(self as *mut ::list::ListFontDatabaseWritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::prepend(const QFontDatabase::WritingSystem& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::font_database::WritingSystem) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_prepend(self as *mut ::list::ListFontDatabaseWritingSystem,
                                                                t as *const ::font_database::WritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::push_back(const QFontDatabase::WritingSystem& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::font_database::WritingSystem) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_push_back(self as *mut ::list::ListFontDatabaseWritingSystem,
                                                                  t as *const ::font_database::WritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::push_front(const QFontDatabase::WritingSystem& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::font_database::WritingSystem) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_push_front(self as *mut ::list::ListFontDatabaseWritingSystem,
                                                                   t as *const ::font_database::WritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QFontDatabase::WritingSystem>::removeAll(const QFontDatabase::WritingSystem& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::font_database::WritingSystem) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_removeAll(self as *mut ::list::ListFontDatabaseWritingSystem,
                                                                  t as *const ::font_database::WritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_removeAt(self as *mut ::list::ListFontDatabaseWritingSystem, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_removeFirst(self as *mut ::list::ListFontDatabaseWritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_removeLast(self as *mut ::list::ListFontDatabaseWritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFontDatabase::WritingSystem>::removeOne(const QFontDatabase::WritingSystem& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::font_database::WritingSystem) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_removeOne(self as *mut ::list::ListFontDatabaseWritingSystem,
                                                                  t as *const ::font_database::WritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::replace(int i, const QFontDatabase::WritingSystem& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::font_database::WritingSystem) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_replace(self as *mut ::list::ListFontDatabaseWritingSystem,
                                                                i,
                                                                t as *const ::font_database::WritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_reserve(self as *mut ::list::ListFontDatabaseWritingSystem,
                                                                size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QFontDatabase::WritingSystem>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_size(self as *const ::list::ListFontDatabaseWritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QFontDatabase::WritingSystem>::startsWith(const QFontDatabase::WritingSystem& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::font_database::WritingSystem) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_startsWith(self as *const ::list::ListFontDatabaseWritingSystem, t as *const ::font_database::WritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListFontDatabaseWritingSystem) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::swap(QList<QFontDatabase::WritingSystem>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QFontDatabase::WritingSystem>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListFontDatabaseWritingSystemSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QFontDatabase::WritingSystem QList<QFontDatabase::WritingSystem>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::font_database::WritingSystem {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_takeAt(self as *mut ::list::ListFontDatabaseWritingSystem, i)
    }
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::WritingSystem QList<QFontDatabase::WritingSystem>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::font_database::WritingSystem {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_takeFirst(self as *mut ::list::ListFontDatabaseWritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```QFontDatabase::WritingSystem QList<QFontDatabase::WritingSystem>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::font_database::WritingSystem {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_takeLast(self as *mut ::list::ListFontDatabaseWritingSystem)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QFontDatabase::WritingSystem>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::font_database::WritingSystem```<br>
  /// C++ method: <span style='color: green;'>```QFontDatabase::WritingSystem QList<QFontDatabase::WritingSystem>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::font_database::WritingSystem)) -> ::font_database::WritingSystem```<br>
  /// C++ method: <span style='color: green;'>```QFontDatabase::WritingSystem QList<QFontDatabase::WritingSystem>::value(int i, const QFontDatabase::WritingSystem& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::font_database::WritingSystem
    where Args: overloading::ListFontDatabaseWritingSystemValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListFontDatabaseWritingSystem {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QFontDatabase::WritingSystem>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_destructor(self as *mut ::list::ListFontDatabaseWritingSystem)
    }
  }
}

/// C++ type: <span style='color: green;'>```QList<QGlyphRun>```</span>
#[repr(C)]
pub struct ListGlyphRun([u8; ::type_sizes::QT_GUI_LIST_LIST_GLYPH_RUN]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListGlyphRun {
  unsafe fn new_uninitialized() -> ListGlyphRun {
    ListGlyphRun(::std::mem::uninitialized())
  }
}

impl ListGlyphRun {
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::glyph_run::GlyphRun) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::append(const QGlyphRun& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListGlyphRun) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::append(const QList<QGlyphRun>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListGlyphRunAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QGlyphRun& QList<QGlyphRun>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_at(self as *const ::list::ListGlyphRun, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QGlyphRun& QList<QGlyphRun>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_back_const(self as *const ::list::ListGlyphRun) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGlyphRun& QList<QGlyphRun>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_back(self as *mut ::list::ListGlyphRun) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_clear(self as *mut ::list::ListGlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```const QGlyphRun& QList<QGlyphRun>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_constFirst(self as *const ::list::ListGlyphRun) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QGlyphRun& QList<QGlyphRun>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_constLast(self as *const ::list::ListGlyphRun) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGlyphRun>::contains(const QGlyphRun& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::glyph_run::GlyphRun) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_contains(self as *const ::list::ListGlyphRun,
                                               t as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGlyphRun>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::glyph_run::GlyphRun) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGlyphRun>::count(const QGlyphRun& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGlyphRunCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QGlyphRun>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_empty(self as *const ::list::ListGlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGlyphRun>::endsWith(const QGlyphRun& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::glyph_run::GlyphRun) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_endsWith(self as *const ::list::ListGlyphRun,
                                               t as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```const QGlyphRun& QList<QGlyphRun>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_first_const(self as *const ::list::ListGlyphRun) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGlyphRun& QList<QGlyphRun>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_first(self as *mut ::list::ListGlyphRun) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QGlyphRun& QList<QGlyphRun>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_front_const(self as *const ::list::ListGlyphRun) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGlyphRun& QList<QGlyphRun>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_front(self as *mut ::list::ListGlyphRun) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::glyph_run::GlyphRun) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGlyphRun>::indexOf(const QGlyphRun& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::glyph_run::GlyphRun, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGlyphRun>::indexOf(const QGlyphRun& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGlyphRunIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::insert(int i, const QGlyphRun& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::glyph_run::GlyphRun) {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_insert(self as *mut ::list::ListGlyphRun,
                                             i,
                                             t as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGlyphRun>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_isEmpty(self as *const ::list::ListGlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```const QGlyphRun& QList<QGlyphRun>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_last_const(self as *const ::list::ListGlyphRun) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::glyph_run::GlyphRun) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGlyphRun>::lastIndexOf(const QGlyphRun& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::glyph_run::GlyphRun, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QGlyphRun>::lastIndexOf(const QGlyphRun& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListGlyphRunLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGlyphRun& QList<QGlyphRun>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_last(self as *mut ::list::ListGlyphRun) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QGlyphRun>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_length(self as *const ::list::ListGlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QList<QGlyphRun>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QList<QGlyphRun>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListGlyphRun
    where Args: overloading::ListGlyphRunMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_move(self as *mut ::list::ListGlyphRun, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGlyphRun>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListGlyphRun) -> ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QGlyphRun>::QList(const QList<QGlyphRun>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListGlyphRun
    where Args: overloading::ListGlyphRunNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun> QList<QGlyphRun>::operator+(const QList<QGlyphRun>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListGlyphRun) -> ::list::ListGlyphRun {
    {
      let mut object: ::list::ListGlyphRun =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_operator_add_to_output(self as *const ::list::ListGlyphRun,
                                                               l as *const ::list::ListGlyphRun,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::glyph_run::GlyphRun) -> &'l0 mut ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>& QList<QGlyphRun>::operator+=(const QGlyphRun& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListGlyphRun) -> &'l0 mut ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>& QList<QGlyphRun>::operator+=(const QList<QGlyphRun>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListGlyphRun
    where Args: overloading::ListGlyphRunOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>& QList<QGlyphRun>::operator=(const QList<QGlyphRun>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListGlyphRun) -> &'l0 mut ::list::ListGlyphRun {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_operator_assign(self as *mut ::list::ListGlyphRun,
                                                      l as *const ::list::ListGlyphRun)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGlyphRun>::operator==(const QList<QGlyphRun>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListGlyphRun) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_operator_eq(self as *const ::list::ListGlyphRun,
                                                  l as *const ::list::ListGlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```const QGlyphRun& QList<QGlyphRun>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::glyph_run::GlyphRun {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_operator_index_const(self as *const ::list::ListGlyphRun, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QGlyphRun& QList<QGlyphRun>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::glyph_run::GlyphRun {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_operator_index(self as *mut ::list::ListGlyphRun, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGlyphRun>::operator!=(const QList<QGlyphRun>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListGlyphRun) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_operator_neq(self as *const ::list::ListGlyphRun,
                                                   l as *const ::list::ListGlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::glyph_run::GlyphRun) -> &'l0 mut ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>& QList<QGlyphRun>::operator<<(const QGlyphRun& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListGlyphRun) -> &'l0 mut ::list::ListGlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>& QList<QGlyphRun>::operator<<(const QList<QGlyphRun>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListGlyphRun
    where Args: overloading::ListGlyphRunOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_pop_back(self as *mut ::list::ListGlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_pop_front(self as *mut ::list::ListGlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::prepend(const QGlyphRun& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::glyph_run::GlyphRun) {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_prepend(self as *mut ::list::ListGlyphRun,
                                              t as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::push_back(const QGlyphRun& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::glyph_run::GlyphRun) {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_push_back(self as *mut ::list::ListGlyphRun,
                                                t as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::push_front(const QGlyphRun& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::glyph_run::GlyphRun) {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_push_front(self as *mut ::list::ListGlyphRun,
                                                 t as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGlyphRun>::removeAll(const QGlyphRun& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::glyph_run::GlyphRun) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_removeAll(self as *mut ::list::ListGlyphRun,
                                                t as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_removeAt(self as *mut ::list::ListGlyphRun, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_removeFirst(self as *mut ::list::ListGlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_removeLast(self as *mut ::list::ListGlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGlyphRun>::removeOne(const QGlyphRun& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::glyph_run::GlyphRun) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_removeOne(self as *mut ::list::ListGlyphRun,
                                                t as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::replace(int i, const QGlyphRun& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::glyph_run::GlyphRun) {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_replace(self as *mut ::list::ListGlyphRun,
                                              i,
                                              t as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_reserve(self as *mut ::list::ListGlyphRun, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QGlyphRun>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_size(self as *const ::list::ListGlyphRun) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QGlyphRun>::startsWith(const QGlyphRun& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::glyph_run::GlyphRun) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QGlyphRun_startsWith(self as *const ::list::ListGlyphRun,
                                                 t as *const ::glyph_run::GlyphRun)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListGlyphRun) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::swap(QList<QGlyphRun>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QGlyphRun>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListGlyphRunSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QGlyphRun QList<QGlyphRun>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::glyph_run::GlyphRun {
    {
      let mut object: ::glyph_run::GlyphRun =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_takeAt_to_output(self as *mut ::list::ListGlyphRun, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGlyphRun QList<QGlyphRun>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::glyph_run::GlyphRun {
    {
      let mut object: ::glyph_run::GlyphRun =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_takeFirst_to_output(self as *mut ::list::ListGlyphRun, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QGlyphRun QList<QGlyphRun>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::glyph_run::GlyphRun {
    {
      let mut object: ::glyph_run::GlyphRun =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_takeLast_to_output(self as *mut ::list::ListGlyphRun, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QGlyphRun>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::glyph_run::GlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QGlyphRun QList<QGlyphRun>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::glyph_run::GlyphRun)) -> ::glyph_run::GlyphRun```<br>
  /// C++ method: <span style='color: green;'>```QGlyphRun QList<QGlyphRun>::value(int i, const QGlyphRun& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::glyph_run::GlyphRun
    where Args: overloading::ListGlyphRunValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListGlyphRun {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QGlyphRun>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_destructor(self as *mut ::list::ListGlyphRun) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>```</span>
#[repr(C)]
pub struct ListInputMethodEventAttribute([u8; ::type_sizes::QT_GUI_LIST_LIST_INPUT_METHOD_EVENT_ATTRIBUTE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListInputMethodEventAttribute {
  unsafe fn new_uninitialized() -> ListInputMethodEventAttribute {
    ListInputMethodEventAttribute(::std::mem::uninitialized())
  }
}

impl ListInputMethodEventAttribute {
  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::input_method_event::Attribute) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::append(const QInputMethodEvent::Attribute& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListInputMethodEventAttribute) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::append(const QList<QInputMethodEvent::Attribute>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListInputMethodEventAttributeAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::input_method_event::Attribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_at(self as *const ::list::ListInputMethodEventAttribute, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::input_method_event::Attribute {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_back_const(self as *const ::list::ListInputMethodEventAttribute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::input_method_event::Attribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_back(self as *mut ::list::ListInputMethodEventAttribute)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_clear(self as *mut ::list::ListInputMethodEventAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```const QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::input_method_event::Attribute {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_constFirst(self as *const ::list::ListInputMethodEventAttribute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::input_method_event::Attribute {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_constLast(self as *const ::list::ListInputMethodEventAttribute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QInputMethodEvent::Attribute>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_count(self as *const ::list::ListInputMethodEventAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QInputMethodEvent::Attribute>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_empty(self as *const ::list::ListInputMethodEventAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```const QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::input_method_event::Attribute {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_first_const(self as *const ::list::ListInputMethodEventAttribute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::input_method_event::Attribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_first(self as *mut ::list::ListInputMethodEventAttribute)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::input_method_event::Attribute {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_front_const(self as *const ::list::ListInputMethodEventAttribute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::input_method_event::Attribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_front(self as *mut ::list::ListInputMethodEventAttribute)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::insert(int i, const QInputMethodEvent::Attribute& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::input_method_event::Attribute) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_insert(self as *mut ::list::ListInputMethodEventAttribute,
                                                               i,
                                                               t as *const ::input_method_event::Attribute)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QInputMethodEvent::Attribute>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_isEmpty(self as *const ::list::ListInputMethodEventAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```const QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::input_method_event::Attribute {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_last_const(self as *const ::list::ListInputMethodEventAttribute) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::input_method_event::Attribute {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_last(self as *mut ::list::ListInputMethodEventAttribute)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QInputMethodEvent::Attribute>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_length(self as *const ::list::ListInputMethodEventAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListInputMethodEventAttribute```<br>
  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute> QList<QInputMethodEvent::Attribute>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListInputMethodEventAttribute```<br>
  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute> QList<QInputMethodEvent::Attribute>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListInputMethodEventAttribute
    where Args: overloading::ListInputMethodEventAttributeMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_move(self as *mut ::list::ListInputMethodEventAttribute,
                                                             from,
                                                             to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListInputMethodEventAttribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QInputMethodEvent::Attribute>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListInputMethodEventAttribute) -> ::list::ListInputMethodEventAttribute```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QInputMethodEvent::Attribute>::QList(const QList<QInputMethodEvent::Attribute>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListInputMethodEventAttribute
    where Args: overloading::ListInputMethodEventAttributeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute> QList<QInputMethodEvent::Attribute>::operator+(const QList<QInputMethodEvent::Attribute>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListInputMethodEventAttribute) -> ::list::ListInputMethodEventAttribute {
    {
      let mut object: ::list::ListInputMethodEventAttribute =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_operator_add_to_output(self as *const ::list::ListInputMethodEventAttribute, l as *const ::list::ListInputMethodEventAttribute, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::input_method_event::Attribute) -> &'l0 mut ::list::ListInputMethodEventAttribute```<br>
  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>& QList<QInputMethodEvent::Attribute>::operator+=(const QInputMethodEvent::Attribute& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListInputMethodEventAttribute) -> &'l0 mut ::list::ListInputMethodEventAttribute```<br>
  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>& QList<QInputMethodEvent::Attribute>::operator+=(const QList<QInputMethodEvent::Attribute>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListInputMethodEventAttribute
    where Args: overloading::ListInputMethodEventAttributeOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>& QList<QInputMethodEvent::Attribute>::operator=(const QList<QInputMethodEvent::Attribute>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListInputMethodEventAttribute)
                             -> &'l0 mut ::list::ListInputMethodEventAttribute {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_operator_assign(self as *mut ::list::ListInputMethodEventAttribute, l as *const ::list::ListInputMethodEventAttribute) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::input_method_event::Attribute {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_operator_index_const(self as *const ::list::ListInputMethodEventAttribute, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QInputMethodEvent::Attribute& QList<QInputMethodEvent::Attribute>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::input_method_event::Attribute {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_operator_index(self as *mut ::list::ListInputMethodEventAttribute, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::input_method_event::Attribute) -> &'l0 mut ::list::ListInputMethodEventAttribute```<br>
  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>& QList<QInputMethodEvent::Attribute>::operator<<(const QInputMethodEvent::Attribute& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListInputMethodEventAttribute) -> &'l0 mut ::list::ListInputMethodEventAttribute```<br>
  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>& QList<QInputMethodEvent::Attribute>::operator<<(const QList<QInputMethodEvent::Attribute>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListInputMethodEventAttribute
    where Args: overloading::ListInputMethodEventAttributeOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_pop_back(self as *mut ::list::ListInputMethodEventAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_pop_front(self as *mut ::list::ListInputMethodEventAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::prepend(const QInputMethodEvent::Attribute& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::input_method_event::Attribute) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_prepend(self as *mut ::list::ListInputMethodEventAttribute,
                                                                t as *const ::input_method_event::Attribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::push_back(const QInputMethodEvent::Attribute& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::input_method_event::Attribute) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_push_back(self as *mut ::list::ListInputMethodEventAttribute,
                                                                  t as *const ::input_method_event::Attribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::push_front(const QInputMethodEvent::Attribute& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::input_method_event::Attribute) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_push_front(self as *mut ::list::ListInputMethodEventAttribute,
                                                                   t as *const ::input_method_event::Attribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_removeAt(self as *mut ::list::ListInputMethodEventAttribute, i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_removeFirst(self as *mut ::list::ListInputMethodEventAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_removeLast(self as *mut ::list::ListInputMethodEventAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::replace(int i, const QInputMethodEvent::Attribute& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::input_method_event::Attribute) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_replace(self as *mut ::list::ListInputMethodEventAttribute,
                                                                i,
                                                                t as *const ::input_method_event::Attribute)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_reserve(self as *mut ::list::ListInputMethodEventAttribute,
                                                                size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QInputMethodEvent::Attribute>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_size(self as *const ::list::ListInputMethodEventAttribute)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QInputMethodEvent::Attribute>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListInputMethodEventAttribute) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::swap(QList<QInputMethodEvent::Attribute>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QInputMethodEvent::Attribute>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListInputMethodEventAttributeSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QInputMethodEvent::Attribute QList<QInputMethodEvent::Attribute>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::input_method_event::Attribute {
    {
      let mut object: ::input_method_event::Attribute =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_takeAt_to_output(self as *mut ::list::ListInputMethodEventAttribute, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QInputMethodEvent::Attribute QList<QInputMethodEvent::Attribute>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::input_method_event::Attribute {
    {
      let mut object: ::input_method_event::Attribute =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_takeFirst_to_output(self as *mut ::list::ListInputMethodEventAttribute, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QInputMethodEvent::Attribute QList<QInputMethodEvent::Attribute>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::input_method_event::Attribute {
    {
      let mut object: ::input_method_event::Attribute =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_takeLast_to_output(self as *mut ::list::ListInputMethodEventAttribute, &mut object);
      }
      object
    }
  }
}

impl Drop for ::list::ListInputMethodEventAttribute {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QInputMethodEvent::Attribute>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_destructor(self as *mut ::list::ListInputMethodEventAttribute)
    }
  }
}

/// C++ type: <span style='color: green;'>```QList<QKeySequence>```</span>
#[repr(C)]
pub struct ListKeySequence([u8; ::type_sizes::QT_GUI_LIST_LIST_KEY_SEQUENCE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListKeySequence {
  unsafe fn new_uninitialized() -> ListKeySequence {
    ListKeySequence(::std::mem::uninitialized())
  }
}

impl ListKeySequence {
  /// C++ method: <span style='color: green;'>```QList<QKeySequence>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::key_sequence::KeySequence) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::append(const QKeySequence& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListKeySequence) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::append(const QList<QKeySequence>& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListKeySequenceAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QKeySequence& QList<QKeySequence>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QKeySequence_at(self as *const ::list::ListKeySequence, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QKeySequence& QList<QKeySequence>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QKeySequence_back_const(self as *const ::list::ListKeySequence) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QKeySequence& QList<QKeySequence>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QKeySequence_back(self as *mut ::list::ListKeySequence) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_clear(self as *mut ::list::ListKeySequence) }
  }

  /// C++ method: <span style='color: green;'>```const QKeySequence& QList<QKeySequence>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QKeySequence_constFirst(self as *const ::list::ListKeySequence) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QKeySequence& QList<QKeySequence>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QKeySequence_constLast(self as *const ::list::ListKeySequence) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QKeySequence>::contains(const QKeySequence& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::key_sequence::KeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_contains(self as *const ::list::ListKeySequence,
                                                  t as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QKeySequence>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QKeySequence>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::key_sequence::KeySequence) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QKeySequence>::count(const QKeySequence& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListKeySequenceCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QKeySequence>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_empty(self as *const ::list::ListKeySequence) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QKeySequence>::endsWith(const QKeySequence& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::key_sequence::KeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_endsWith(self as *const ::list::ListKeySequence,
                                                  t as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```const QKeySequence& QList<QKeySequence>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QKeySequence_first_const(self as *const ::list::ListKeySequence) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QKeySequence& QList<QKeySequence>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QKeySequence_first(self as *mut ::list::ListKeySequence) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QKeySequence& QList<QKeySequence>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QKeySequence_front_const(self as *const ::list::ListKeySequence) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QKeySequence& QList<QKeySequence>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QKeySequence_front(self as *mut ::list::ListKeySequence) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QKeySequence>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::key_sequence::KeySequence) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QKeySequence>::indexOf(const QKeySequence& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::key_sequence::KeySequence, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QKeySequence>::indexOf(const QKeySequence& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListKeySequenceIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::insert(int i, const QKeySequence& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::key_sequence::KeySequence) {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_insert(self as *mut ::list::ListKeySequence,
                                                i,
                                                t as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QKeySequence>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_isEmpty(self as *const ::list::ListKeySequence) }
  }

  /// C++ method: <span style='color: green;'>```const QKeySequence& QList<QKeySequence>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QKeySequence_last_const(self as *const ::list::ListKeySequence) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QKeySequence>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::key_sequence::KeySequence) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QKeySequence>::lastIndexOf(const QKeySequence& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::key_sequence::KeySequence, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QKeySequence>::lastIndexOf(const QKeySequence& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListKeySequenceLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QKeySequence& QList<QKeySequence>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::key_sequence::KeySequence {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QKeySequence_last(self as *mut ::list::ListKeySequence) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QKeySequence>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_length(self as *const ::list::ListKeySequence) }
  }

  /// C++ method: <span style='color: green;'>```QList<QKeySequence>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListKeySequence```<br>
  /// C++ method: <span style='color: green;'>```QList<QKeySequence> QList<QKeySequence>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListKeySequence```<br>
  /// C++ method: <span style='color: green;'>```QList<QKeySequence> QList<QKeySequence>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListKeySequence
    where Args: overloading::ListKeySequenceMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_move(self as *mut ::list::ListKeySequence, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QKeySequence>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListKeySequence```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QKeySequence>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListKeySequence) -> ::list::ListKeySequence```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QKeySequence>::QList(const QList<QKeySequence>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListKeySequence
    where Args: overloading::ListKeySequenceNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QKeySequence> QList<QKeySequence>::operator+(const QList<QKeySequence>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListKeySequence) -> ::list::ListKeySequence {
    {
      let mut object: ::list::ListKeySequence =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_operator_add_to_output(self as *const ::list::ListKeySequence,
                                                                  l as *const ::list::ListKeySequence,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QKeySequence>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::key_sequence::KeySequence) -> &'l0 mut ::list::ListKeySequence```<br>
  /// C++ method: <span style='color: green;'>```QList<QKeySequence>& QList<QKeySequence>::operator+=(const QKeySequence& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListKeySequence) -> &'l0 mut ::list::ListKeySequence```<br>
  /// C++ method: <span style='color: green;'>```QList<QKeySequence>& QList<QKeySequence>::operator+=(const QList<QKeySequence>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListKeySequence
    where Args: overloading::ListKeySequenceOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QKeySequence>& QList<QKeySequence>::operator=(const QList<QKeySequence>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListKeySequence) -> &'l0 mut ::list::ListKeySequence {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_operator_assign(self as *mut ::list::ListKeySequence,
                                                         l as *const ::list::ListKeySequence)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QKeySequence>::operator==(const QList<QKeySequence>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListKeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_operator_eq(self as *const ::list::ListKeySequence,
                                                     l as *const ::list::ListKeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```const QKeySequence& QList<QKeySequence>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::key_sequence::KeySequence {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QKeySequence_operator_index_const(self as *const ::list::ListKeySequence, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QKeySequence& QList<QKeySequence>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::key_sequence::KeySequence {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QKeySequence_operator_index(self as *mut ::list::ListKeySequence, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QKeySequence>::operator!=(const QList<QKeySequence>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListKeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_operator_neq(self as *const ::list::ListKeySequence,
                                                      l as *const ::list::ListKeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QKeySequence>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::key_sequence::KeySequence) -> &'l0 mut ::list::ListKeySequence```<br>
  /// C++ method: <span style='color: green;'>```QList<QKeySequence>& QList<QKeySequence>::operator<<(const QKeySequence& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListKeySequence) -> &'l0 mut ::list::ListKeySequence```<br>
  /// C++ method: <span style='color: green;'>```QList<QKeySequence>& QList<QKeySequence>::operator<<(const QList<QKeySequence>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListKeySequence
    where Args: overloading::ListKeySequenceOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_pop_back(self as *mut ::list::ListKeySequence) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_pop_front(self as *mut ::list::ListKeySequence) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::prepend(const QKeySequence& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::key_sequence::KeySequence) {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_prepend(self as *mut ::list::ListKeySequence,
                                                 t as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::push_back(const QKeySequence& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::key_sequence::KeySequence) {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_push_back(self as *mut ::list::ListKeySequence,
                                                   t as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::push_front(const QKeySequence& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::key_sequence::KeySequence) {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_push_front(self as *mut ::list::ListKeySequence,
                                                    t as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QKeySequence>::removeAll(const QKeySequence& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::key_sequence::KeySequence) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_removeAll(self as *mut ::list::ListKeySequence,
                                                   t as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_removeAt(self as *mut ::list::ListKeySequence, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_removeFirst(self as *mut ::list::ListKeySequence) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_removeLast(self as *mut ::list::ListKeySequence) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QKeySequence>::removeOne(const QKeySequence& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::key_sequence::KeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_removeOne(self as *mut ::list::ListKeySequence,
                                                   t as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::replace(int i, const QKeySequence& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::key_sequence::KeySequence) {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_replace(self as *mut ::list::ListKeySequence,
                                                 i,
                                                 t as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_reserve(self as *mut ::list::ListKeySequence, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QKeySequence>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_size(self as *const ::list::ListKeySequence) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QKeySequence>::startsWith(const QKeySequence& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::key_sequence::KeySequence) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QKeySequence_startsWith(self as *const ::list::ListKeySequence,
                                                    t as *const ::key_sequence::KeySequence)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QKeySequence>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListKeySequence) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::swap(QList<QKeySequence>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QKeySequence>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListKeySequenceSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QKeySequence QList<QKeySequence>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::key_sequence::KeySequence {
    {
      let mut object: ::key_sequence::KeySequence =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_takeAt_to_output(self as *mut ::list::ListKeySequence, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QKeySequence QList<QKeySequence>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::key_sequence::KeySequence {
    {
      let mut object: ::key_sequence::KeySequence =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_takeFirst_to_output(self as *mut ::list::ListKeySequence, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QKeySequence QList<QKeySequence>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::key_sequence::KeySequence {
    {
      let mut object: ::key_sequence::KeySequence =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_takeLast_to_output(self as *mut ::list::ListKeySequence, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QKeySequence>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```QKeySequence QList<QKeySequence>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::key_sequence::KeySequence)) -> ::key_sequence::KeySequence```<br>
  /// C++ method: <span style='color: green;'>```QKeySequence QList<QKeySequence>::value(int i, const QKeySequence& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::key_sequence::KeySequence
    where Args: overloading::ListKeySequenceValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListKeySequence {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QKeySequence>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QKeySequence_destructor(self as *mut ::list::ListKeySequence) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QOpenGLContext*>```</span>
#[repr(C)]
pub struct ListOpenglContextMutPtr([u8; ::type_sizes::QT_GUI_LIST_LIST_OPENGL_CONTEXT_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListOpenglContextMutPtr {
  unsafe fn new_uninitialized() -> ListOpenglContextMutPtr {
    ListOpenglContextMutPtr(::std::mem::uninitialized())
  }
}

impl ListOpenglContextMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::append(const QList<QOpenGLContext*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListOpenglContextMutPtr) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_append_QList_QOpenGLContext_ptr(self as *mut ::list::ListOpenglContextMutPtr, t as *const ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::append(QOpenGLContext* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::opengl_context::OpenGLContext) {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_append_QOpenGLContext(self as *mut ::list::ListOpenglContextMutPtr,
                                                                   t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* const & QList<QOpenGLContext*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_at(self as *const ::list::ListOpenglContextMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* const & QList<QOpenGLContext*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_back_const(self as *const ::list::ListOpenglContextMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext*& QList<QOpenGLContext*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_back(self as *mut ::list::ListOpenglContextMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_clear(self as *mut ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* const & QList<QOpenGLContext*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_constFirst(self as *const ::list::ListOpenglContextMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* const & QList<QOpenGLContext*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_constLast(self as *const ::list::ListOpenglContextMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLContext*>::contains(QOpenGLContext* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::opengl_context::OpenGLContext) -> bool {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_contains(self as *const ::list::ListOpenglContextMutPtr,
                                                      t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLContext*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_count_no_args(self as *const ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLContext*>::count(QOpenGLContext* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::opengl_context::OpenGLContext) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_count_t(self as *const ::list::ListOpenglContextMutPtr,
                                                     t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLContext*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_empty(self as *const ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLContext*>::endsWith(QOpenGLContext* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::opengl_context::OpenGLContext) -> bool {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_endsWith(self as *const ::list::ListOpenglContextMutPtr,
                                                      t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* const & QList<QOpenGLContext*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_first_const(self as *const ::list::ListOpenglContextMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext*& QList<QOpenGLContext*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_first(self as *mut ::list::ListOpenglContextMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* const & QList<QOpenGLContext*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_front_const(self as *const ::list::ListOpenglContextMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext*& QList<QOpenGLContext*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_front(self as *mut ::list::ListOpenglContextMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::opengl_context::OpenGLContext) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLContext*>::indexOf(QOpenGLContext* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::opengl_context::OpenGLContext, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLContext*>::indexOf(QOpenGLContext* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListOpenglContextMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::insert(int i, QOpenGLContext* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::opengl_context::OpenGLContext) {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_insert(self as *mut ::list::ListOpenglContextMutPtr,
                                                    i,
                                                    t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLContext*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_isEmpty(self as *const ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* const & QList<QOpenGLContext*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_last_const(self as *const ::list::ListOpenglContextMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::opengl_context::OpenGLContext) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLContext*>::lastIndexOf(QOpenGLContext* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::opengl_context::OpenGLContext, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLContext*>::lastIndexOf(QOpenGLContext* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListOpenglContextMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLContext*& QList<QOpenGLContext*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_last(self as *mut ::list::ListOpenglContextMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLContext*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_length(self as *const ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListOpenglContextMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*> QList<QOpenGLContext*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListOpenglContextMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*> QList<QOpenGLContext*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListOpenglContextMutPtr
    where Args: overloading::ListOpenglContextMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_move(self as *mut ::list::ListOpenglContextMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListOpenglContextMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QOpenGLContext*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListOpenglContextMutPtr) -> ::list::ListOpenglContextMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QOpenGLContext*>::QList(const QList<QOpenGLContext*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListOpenglContextMutPtr
    where Args: overloading::ListOpenglContextMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*> QList<QOpenGLContext*>::operator+(const QList<QOpenGLContext*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListOpenglContextMutPtr) -> ::list::ListOpenglContextMutPtr {
    {
      let mut object: ::list::ListOpenglContextMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_operator_add_to_output(self as *const ::list::ListOpenglContextMutPtr, l as *const ::list::ListOpenglContextMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*>& QList<QOpenGLContext*>::operator+=(const QList<QOpenGLContext*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListOpenglContextMutPtr)
                                 -> &'l0 mut ::list::ListOpenglContextMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_operator_add_assign_l(self as *mut ::list::ListOpenglContextMutPtr,
                                                                       l as *const ::list::ListOpenglContextMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*>& QList<QOpenGLContext*>::operator+=(QOpenGLContext* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::opengl_context::OpenGLContext)
                                               -> &'l0 mut ::list::ListOpenglContextMutPtr {
    let ffi_result = ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_operator_add_assign_t(self as *mut ::list::ListOpenglContextMutPtr, t as *const *mut ::opengl_context::OpenGLContext);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*>& QList<QOpenGLContext*>::operator=(const QList<QOpenGLContext*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListOpenglContextMutPtr)
                             -> &'l0 mut ::list::ListOpenglContextMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_operator_assign(self as *mut ::list::ListOpenglContextMutPtr,
                                                                 l as *const ::list::ListOpenglContextMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLContext*>::operator==(const QList<QOpenGLContext*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListOpenglContextMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_operator_eq(self as *const ::list::ListOpenglContextMutPtr,
                                                           l as *const ::list::ListOpenglContextMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* const & QList<QOpenGLContext*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_operator_index_const(self as *const ::list::ListOpenglContextMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext*& QList<QOpenGLContext*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::opengl_context::OpenGLContext {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_operator_index(self as *mut ::list::ListOpenglContextMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLContext*>::operator!=(const QList<QOpenGLContext*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListOpenglContextMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_operator_neq(self as *const ::list::ListOpenglContextMutPtr,
                                                            l as *const ::list::ListOpenglContextMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*>& QList<QOpenGLContext*>::operator<<(const QList<QOpenGLContext*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListOpenglContextMutPtr)
                          -> &'l0 mut ::list::ListOpenglContextMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_operator_shl_l(self as *mut ::list::ListOpenglContextMutPtr,
                                                                l as *const ::list::ListOpenglContextMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*>& QList<QOpenGLContext*>::operator<<(QOpenGLContext* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::opengl_context::OpenGLContext)
                                        -> &'l0 mut ::list::ListOpenglContextMutPtr {
    let ffi_result =
      ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_operator_shl_t(self as *mut ::list::ListOpenglContextMutPtr,
                                                              t as *const *mut ::opengl_context::OpenGLContext);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_pop_back(self as *mut ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_pop_front(self as *mut ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::prepend(QOpenGLContext* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::opengl_context::OpenGLContext) {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_prepend(self as *mut ::list::ListOpenglContextMutPtr,
                                                     t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::push_back(QOpenGLContext* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::opengl_context::OpenGLContext) {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_push_back(self as *mut ::list::ListOpenglContextMutPtr,
                                                       t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::push_front(QOpenGLContext* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::opengl_context::OpenGLContext) {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_push_front(self as *mut ::list::ListOpenglContextMutPtr,
                                                        t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLContext*>::removeAll(QOpenGLContext* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::opengl_context::OpenGLContext) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_removeAll(self as *mut ::list::ListOpenglContextMutPtr,
                                                       t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_removeAt(self as *mut ::list::ListOpenglContextMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_removeFirst(self as *mut ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_removeLast(self as *mut ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLContext*>::removeOne(QOpenGLContext* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::opengl_context::OpenGLContext) -> bool {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_removeOne(self as *mut ::list::ListOpenglContextMutPtr,
                                                       t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::replace(int i, QOpenGLContext* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::opengl_context::OpenGLContext) {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_replace(self as *mut ::list::ListOpenglContextMutPtr,
                                                     i,
                                                     t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_reserve(self as *mut ::list::ListOpenglContextMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLContext*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_size(self as *const ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLContext*>::startsWith(QOpenGLContext* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::opengl_context::OpenGLContext) -> bool {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_startsWith(self as *const ::list::ListOpenglContextMutPtr,
                                                        t as *const *mut ::opengl_context::OpenGLContext)
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLContext*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListOpenglContextMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::swap(QList<QOpenGLContext*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLContext*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListOpenglContextMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLContext* QList<QOpenGLContext*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::opengl_context::OpenGLContext {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_takeAt(self as *mut ::list::ListOpenglContextMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* QList<QOpenGLContext*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::opengl_context::OpenGLContext {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_takeFirst(self as *mut ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* QList<QOpenGLContext*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::opengl_context::OpenGLContext {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_takeLast(self as *mut ::list::ListOpenglContextMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* QList<QOpenGLContext*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::opengl_context::OpenGLContext {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_value_i(self as *const ::list::ListOpenglContextMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLContext* QList<QOpenGLContext*>::value(int i, QOpenGLContext* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::opengl_context::OpenGLContext)
                             -> *mut ::opengl_context::OpenGLContext {
    ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_value_i_defaultValue(self as *const ::list::ListOpenglContextMutPtr, i, default_value as *const *mut ::opengl_context::OpenGLContext)
  }
}

impl Drop for ::list::ListOpenglContextMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QOpenGLContext*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_destructor(self as *mut ::list::ListOpenglContextMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QOpenGLDebugMessage>```</span>
#[repr(C)]
pub struct ListOpenglDebugMessage([u8; ::type_sizes::QT_GUI_LIST_LIST_OPENGL_DEBUG_MESSAGE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListOpenglDebugMessage {
  unsafe fn new_uninitialized() -> ListOpenglDebugMessage {
    ListOpenglDebugMessage(::std::mem::uninitialized())
  }
}

impl ListOpenglDebugMessage {
  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListOpenglDebugMessage) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::append(const QList<QOpenGLDebugMessage>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::opengl_debug_message::OpenGLDebugMessage) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::append(const QOpenGLDebugMessage& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListOpenglDebugMessageAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_at(self as *const ::list::ListOpenglDebugMessage, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_back_const(self as *const ::list::ListOpenglDebugMessage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_back(self as *mut ::list::ListOpenglDebugMessage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_clear(self as *mut ::list::ListOpenglDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```const QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_constFirst(self as *const ::list::ListOpenglDebugMessage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_constLast(self as *const ::list::ListOpenglDebugMessage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLDebugMessage>::contains(const QOpenGLDebugMessage& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::opengl_debug_message::OpenGLDebugMessage) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_contains(self as *const ::list::ListOpenglDebugMessage,
                                                         t as *const ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLDebugMessage>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::opengl_debug_message::OpenGLDebugMessage) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLDebugMessage>::count(const QOpenGLDebugMessage& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListOpenglDebugMessageCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLDebugMessage>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_empty(self as *const ::list::ListOpenglDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLDebugMessage>::endsWith(const QOpenGLDebugMessage& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::opengl_debug_message::OpenGLDebugMessage) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_endsWith(self as *const ::list::ListOpenglDebugMessage,
                                                         t as *const ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```const QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_first_const(self as *const ::list::ListOpenglDebugMessage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_first(self as *mut ::list::ListOpenglDebugMessage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_front_const(self as *const ::list::ListOpenglDebugMessage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_front(self as *mut ::list::ListOpenglDebugMessage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::opengl_debug_message::OpenGLDebugMessage) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLDebugMessage>::indexOf(const QOpenGLDebugMessage& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::opengl_debug_message::OpenGLDebugMessage, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLDebugMessage>::indexOf(const QOpenGLDebugMessage& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListOpenglDebugMessageIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::insert(int i, const QOpenGLDebugMessage& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::opengl_debug_message::OpenGLDebugMessage) {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_insert(self as *mut ::list::ListOpenglDebugMessage,
                                                       i,
                                                       t as *const ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLDebugMessage>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_isEmpty(self as *const ::list::ListOpenglDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```const QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_last_const(self as *const ::list::ListOpenglDebugMessage) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::opengl_debug_message::OpenGLDebugMessage) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLDebugMessage>::lastIndexOf(const QOpenGLDebugMessage& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::opengl_debug_message::OpenGLDebugMessage, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLDebugMessage>::lastIndexOf(const QOpenGLDebugMessage& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListOpenglDebugMessageLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_last(self as *mut ::list::ListOpenglDebugMessage) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLDebugMessage>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_length(self as *const ::list::ListOpenglDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListOpenglDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage> QList<QOpenGLDebugMessage>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListOpenglDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage> QList<QOpenGLDebugMessage>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListOpenglDebugMessage
    where Args: overloading::ListOpenglDebugMessageMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_move(self as *mut ::list::ListOpenglDebugMessage, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListOpenglDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QOpenGLDebugMessage>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListOpenglDebugMessage) -> ::list::ListOpenglDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QOpenGLDebugMessage>::QList(const QList<QOpenGLDebugMessage>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListOpenglDebugMessage
    where Args: overloading::ListOpenglDebugMessageNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage> QList<QOpenGLDebugMessage>::operator+(const QList<QOpenGLDebugMessage>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListOpenglDebugMessage) -> ::list::ListOpenglDebugMessage {
    {
      let mut object: ::list::ListOpenglDebugMessage =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_operator_add_to_output(self as *const ::list::ListOpenglDebugMessage, l as *const ::list::ListOpenglDebugMessage, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListOpenglDebugMessage) -> &'l0 mut ::list::ListOpenglDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>& QList<QOpenGLDebugMessage>::operator+=(const QList<QOpenGLDebugMessage>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::opengl_debug_message::OpenGLDebugMessage) -> &'l0 mut ::list::ListOpenglDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>& QList<QOpenGLDebugMessage>::operator+=(const QOpenGLDebugMessage& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListOpenglDebugMessage
    where Args: overloading::ListOpenglDebugMessageOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>& QList<QOpenGLDebugMessage>::operator=(const QList<QOpenGLDebugMessage>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListOpenglDebugMessage)
                             -> &'l0 mut ::list::ListOpenglDebugMessage {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_operator_assign(self as *mut ::list::ListOpenglDebugMessage,
                                                                  l as *const ::list::ListOpenglDebugMessage)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLDebugMessage>::operator==(const QList<QOpenGLDebugMessage>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListOpenglDebugMessage) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_operator_eq(self as *const ::list::ListOpenglDebugMessage,
                                                            l as *const ::list::ListOpenglDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```const QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_operator_index_const(self as *const ::list::ListOpenglDebugMessage, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage& QList<QOpenGLDebugMessage>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::opengl_debug_message::OpenGLDebugMessage {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_operator_index(self as *mut ::list::ListOpenglDebugMessage, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLDebugMessage>::operator!=(const QList<QOpenGLDebugMessage>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListOpenglDebugMessage) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_operator_neq(self as *const ::list::ListOpenglDebugMessage,
                                                             l as *const ::list::ListOpenglDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListOpenglDebugMessage) -> &'l0 mut ::list::ListOpenglDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>& QList<QOpenGLDebugMessage>::operator<<(const QList<QOpenGLDebugMessage>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::opengl_debug_message::OpenGLDebugMessage) -> &'l0 mut ::list::ListOpenglDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>& QList<QOpenGLDebugMessage>::operator<<(const QOpenGLDebugMessage& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListOpenglDebugMessage
    where Args: overloading::ListOpenglDebugMessageOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_pop_back(self as *mut ::list::ListOpenglDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_pop_front(self as *mut ::list::ListOpenglDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::prepend(const QOpenGLDebugMessage& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::opengl_debug_message::OpenGLDebugMessage) {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_prepend(self as *mut ::list::ListOpenglDebugMessage,
                                                        t as *const ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::push_back(const QOpenGLDebugMessage& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::opengl_debug_message::OpenGLDebugMessage) {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_push_back(self as *mut ::list::ListOpenglDebugMessage,
                                                          t as *const ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::push_front(const QOpenGLDebugMessage& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::opengl_debug_message::OpenGLDebugMessage) {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_push_front(self as *mut ::list::ListOpenglDebugMessage,
                                                           t as *const ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLDebugMessage>::removeAll(const QOpenGLDebugMessage& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::opengl_debug_message::OpenGLDebugMessage) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_removeAll(self as *mut ::list::ListOpenglDebugMessage,
                                                          t as *const ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_removeAt(self as *mut ::list::ListOpenglDebugMessage, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_removeFirst(self as *mut ::list::ListOpenglDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_removeLast(self as *mut ::list::ListOpenglDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLDebugMessage>::removeOne(const QOpenGLDebugMessage& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::opengl_debug_message::OpenGLDebugMessage) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_removeOne(self as *mut ::list::ListOpenglDebugMessage,
                                                          t as *const ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::replace(int i, const QOpenGLDebugMessage& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::opengl_debug_message::OpenGLDebugMessage) {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_replace(self as *mut ::list::ListOpenglDebugMessage,
                                                        i,
                                                        t as *const ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_reserve(self as *mut ::list::ListOpenglDebugMessage, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLDebugMessage>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_size(self as *const ::list::ListOpenglDebugMessage) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLDebugMessage>::startsWith(const QOpenGLDebugMessage& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::opengl_debug_message::OpenGLDebugMessage) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_startsWith(self as *const ::list::ListOpenglDebugMessage,
                                                           t as *const ::opengl_debug_message::OpenGLDebugMessage)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListOpenglDebugMessage) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::swap(QList<QOpenGLDebugMessage>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLDebugMessage>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListOpenglDebugMessageSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage QList<QOpenGLDebugMessage>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::opengl_debug_message::OpenGLDebugMessage {
    {
      let mut object: ::opengl_debug_message::OpenGLDebugMessage =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_takeAt_to_output(self as *mut ::list::ListOpenglDebugMessage,
                                                                   i,
                                                                   &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage QList<QOpenGLDebugMessage>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::opengl_debug_message::OpenGLDebugMessage {
    {
      let mut object: ::opengl_debug_message::OpenGLDebugMessage =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_takeFirst_to_output(self as *mut ::list::ListOpenglDebugMessage,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage QList<QOpenGLDebugMessage>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::opengl_debug_message::OpenGLDebugMessage {
    {
      let mut object: ::opengl_debug_message::OpenGLDebugMessage =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_takeLast_to_output(self as *mut ::list::ListOpenglDebugMessage,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLDebugMessage>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage QList<QOpenGLDebugMessage>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::opengl_debug_message::OpenGLDebugMessage)) -> ::opengl_debug_message::OpenGLDebugMessage```<br>
  /// C++ method: <span style='color: green;'>```QOpenGLDebugMessage QList<QOpenGLDebugMessage>::value(int i, const QOpenGLDebugMessage& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::opengl_debug_message::OpenGLDebugMessage
    where Args: overloading::ListOpenglDebugMessageValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListOpenglDebugMessage {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QOpenGLDebugMessage>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_destructor(self as *mut ::list::ListOpenglDebugMessage) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QOpenGLShader*>```</span>
#[repr(C)]
pub struct ListOpenglShaderMutPtr([u8; ::type_sizes::QT_GUI_LIST_LIST_OPENGL_SHADER_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListOpenglShaderMutPtr {
  unsafe fn new_uninitialized() -> ListOpenglShaderMutPtr {
    ListOpenglShaderMutPtr(::std::mem::uninitialized())
  }
}

impl ListOpenglShaderMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::append(const QList<QOpenGLShader*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListOpenglShaderMutPtr) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_append_QList_QOpenGLShader_ptr(self as *mut ::list::ListOpenglShaderMutPtr, t as *const ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::append(QOpenGLShader* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::opengl_shader::OpenGLShader) {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_append_QOpenGLShader(self as *mut ::list::ListOpenglShaderMutPtr,
                                                                 t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* const & QList<QOpenGLShader*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_at(self as *const ::list::ListOpenglShaderMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* const & QList<QOpenGLShader*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_back_const(self as *const ::list::ListOpenglShaderMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader*& QList<QOpenGLShader*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_back(self as *mut ::list::ListOpenglShaderMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_clear(self as *mut ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* const & QList<QOpenGLShader*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_constFirst(self as *const ::list::ListOpenglShaderMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* const & QList<QOpenGLShader*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_constLast(self as *const ::list::ListOpenglShaderMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLShader*>::contains(QOpenGLShader* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::opengl_shader::OpenGLShader) -> bool {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_contains(self as *const ::list::ListOpenglShaderMutPtr,
                                                     t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLShader*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_count_no_args(self as *const ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLShader*>::count(QOpenGLShader* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::opengl_shader::OpenGLShader) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_count_t(self as *const ::list::ListOpenglShaderMutPtr,
                                                    t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLShader*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_empty(self as *const ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLShader*>::endsWith(QOpenGLShader* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::opengl_shader::OpenGLShader) -> bool {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_endsWith(self as *const ::list::ListOpenglShaderMutPtr,
                                                     t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* const & QList<QOpenGLShader*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_first_const(self as *const ::list::ListOpenglShaderMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader*& QList<QOpenGLShader*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_first(self as *mut ::list::ListOpenglShaderMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* const & QList<QOpenGLShader*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_front_const(self as *const ::list::ListOpenglShaderMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader*& QList<QOpenGLShader*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_front(self as *mut ::list::ListOpenglShaderMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::opengl_shader::OpenGLShader) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLShader*>::indexOf(QOpenGLShader* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::opengl_shader::OpenGLShader, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLShader*>::indexOf(QOpenGLShader* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListOpenglShaderMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::insert(int i, QOpenGLShader* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::opengl_shader::OpenGLShader) {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_insert(self as *mut ::list::ListOpenglShaderMutPtr,
                                                   i,
                                                   t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLShader*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_isEmpty(self as *const ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* const & QList<QOpenGLShader*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_last_const(self as *const ::list::ListOpenglShaderMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::opengl_shader::OpenGLShader) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLShader*>::lastIndexOf(QOpenGLShader* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::opengl_shader::OpenGLShader, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QOpenGLShader*>::lastIndexOf(QOpenGLShader* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListOpenglShaderMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLShader*& QList<QOpenGLShader*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_last(self as *mut ::list::ListOpenglShaderMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLShader*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_length(self as *const ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListOpenglShaderMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*> QList<QOpenGLShader*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListOpenglShaderMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*> QList<QOpenGLShader*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListOpenglShaderMutPtr
    where Args: overloading::ListOpenglShaderMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_move(self as *mut ::list::ListOpenglShaderMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListOpenglShaderMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QOpenGLShader*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListOpenglShaderMutPtr) -> ::list::ListOpenglShaderMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QOpenGLShader*>::QList(const QList<QOpenGLShader*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListOpenglShaderMutPtr
    where Args: overloading::ListOpenglShaderMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*> QList<QOpenGLShader*>::operator+(const QList<QOpenGLShader*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListOpenglShaderMutPtr) -> ::list::ListOpenglShaderMutPtr {
    {
      let mut object: ::list::ListOpenglShaderMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_operator_add_to_output(self as *const ::list::ListOpenglShaderMutPtr,
                                                                       l as *const ::list::ListOpenglShaderMutPtr,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*>& QList<QOpenGLShader*>::operator+=(const QList<QOpenGLShader*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListOpenglShaderMutPtr)
                                 -> &'l0 mut ::list::ListOpenglShaderMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_operator_add_assign_l(self as *mut ::list::ListOpenglShaderMutPtr,
                                                                      l as *const ::list::ListOpenglShaderMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*>& QList<QOpenGLShader*>::operator+=(QOpenGLShader* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::opengl_shader::OpenGLShader)
                                               -> &'l0 mut ::list::ListOpenglShaderMutPtr {
    let ffi_result =
      ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_operator_add_assign_t(self as *mut ::list::ListOpenglShaderMutPtr,
                                                                    t as *const *mut ::opengl_shader::OpenGLShader);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*>& QList<QOpenGLShader*>::operator=(const QList<QOpenGLShader*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListOpenglShaderMutPtr)
                             -> &'l0 mut ::list::ListOpenglShaderMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_operator_assign(self as *mut ::list::ListOpenglShaderMutPtr,
                                                                l as *const ::list::ListOpenglShaderMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLShader*>::operator==(const QList<QOpenGLShader*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListOpenglShaderMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_operator_eq(self as *const ::list::ListOpenglShaderMutPtr,
                                                          l as *const ::list::ListOpenglShaderMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* const & QList<QOpenGLShader*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_operator_index_const(self as *const ::list::ListOpenglShaderMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader*& QList<QOpenGLShader*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::opengl_shader::OpenGLShader {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_operator_index(self as *mut ::list::ListOpenglShaderMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLShader*>::operator!=(const QList<QOpenGLShader*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListOpenglShaderMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_operator_neq(self as *const ::list::ListOpenglShaderMutPtr,
                                                           l as *const ::list::ListOpenglShaderMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*>& QList<QOpenGLShader*>::operator<<(const QList<QOpenGLShader*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListOpenglShaderMutPtr)
                          -> &'l0 mut ::list::ListOpenglShaderMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_operator_shl_l(self as *mut ::list::ListOpenglShaderMutPtr,
                                                               l as *const ::list::ListOpenglShaderMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*>& QList<QOpenGLShader*>::operator<<(QOpenGLShader* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::opengl_shader::OpenGLShader)
                                        -> &'l0 mut ::list::ListOpenglShaderMutPtr {
    let ffi_result =
      ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_operator_shl_t(self as *mut ::list::ListOpenglShaderMutPtr,
                                                             t as *const *mut ::opengl_shader::OpenGLShader);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_pop_back(self as *mut ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_pop_front(self as *mut ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::prepend(QOpenGLShader* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::opengl_shader::OpenGLShader) {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_prepend(self as *mut ::list::ListOpenglShaderMutPtr,
                                                    t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::push_back(QOpenGLShader* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::opengl_shader::OpenGLShader) {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_push_back(self as *mut ::list::ListOpenglShaderMutPtr,
                                                      t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::push_front(QOpenGLShader* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::opengl_shader::OpenGLShader) {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_push_front(self as *mut ::list::ListOpenglShaderMutPtr,
                                                       t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLShader*>::removeAll(QOpenGLShader* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::opengl_shader::OpenGLShader) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_removeAll(self as *mut ::list::ListOpenglShaderMutPtr,
                                                      t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_removeAt(self as *mut ::list::ListOpenglShaderMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_removeFirst(self as *mut ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_removeLast(self as *mut ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLShader*>::removeOne(QOpenGLShader* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::opengl_shader::OpenGLShader) -> bool {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_removeOne(self as *mut ::list::ListOpenglShaderMutPtr,
                                                      t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::replace(int i, QOpenGLShader* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::opengl_shader::OpenGLShader) {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_replace(self as *mut ::list::ListOpenglShaderMutPtr,
                                                    i,
                                                    t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_reserve(self as *mut ::list::ListOpenglShaderMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QOpenGLShader*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_size(self as *const ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QOpenGLShader*>::startsWith(QOpenGLShader* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::opengl_shader::OpenGLShader) -> bool {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_startsWith(self as *const ::list::ListOpenglShaderMutPtr,
                                                       t as *const *mut ::opengl_shader::OpenGLShader)
  }

  /// C++ method: <span style='color: green;'>```QList<QOpenGLShader*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListOpenglShaderMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::swap(QList<QOpenGLShader*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QOpenGLShader*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListOpenglShaderMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QOpenGLShader* QList<QOpenGLShader*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::opengl_shader::OpenGLShader {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_takeAt(self as *mut ::list::ListOpenglShaderMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* QList<QOpenGLShader*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::opengl_shader::OpenGLShader {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_takeFirst(self as *mut ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* QList<QOpenGLShader*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::opengl_shader::OpenGLShader {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_takeLast(self as *mut ::list::ListOpenglShaderMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* QList<QOpenGLShader*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::opengl_shader::OpenGLShader {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_value_i(self as *const ::list::ListOpenglShaderMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QOpenGLShader* QList<QOpenGLShader*>::value(int i, QOpenGLShader* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::opengl_shader::OpenGLShader)
                             -> *mut ::opengl_shader::OpenGLShader {
    ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_value_i_defaultValue(self as *const ::list::ListOpenglShaderMutPtr, i, default_value as *const *mut ::opengl_shader::OpenGLShader)
  }
}

impl Drop for ::list::ListOpenglShaderMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QOpenGLShader*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_destructor(self as *mut ::list::ListOpenglShaderMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QPolygonF>```</span>
#[repr(C)]
pub struct ListPolygonF([u8; ::type_sizes::QT_GUI_LIST_LIST_POLYGON_F]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListPolygonF {
  unsafe fn new_uninitialized() -> ListPolygonF {
    ListPolygonF(::std::mem::uninitialized())
  }
}

impl ListPolygonF {
  /// C++ method: <span style='color: green;'>```QList<QPolygonF>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListPolygonF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::append(const QList<QPolygonF>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::polygon_f::PolygonF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::append(const QPolygonF& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListPolygonFAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QPolygonF& QList<QPolygonF>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_at(self as *const ::list::ListPolygonF, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPolygonF& QList<QPolygonF>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_back_const(self as *const ::list::ListPolygonF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPolygonF& QList<QPolygonF>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_back(self as *mut ::list::ListPolygonF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_clear(self as *mut ::list::ListPolygonF) }
  }

  /// C++ method: <span style='color: green;'>```const QPolygonF& QList<QPolygonF>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_constFirst(self as *const ::list::ListPolygonF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPolygonF& QList<QPolygonF>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_constLast(self as *const ::list::ListPolygonF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPolygonF>::contains(const QPolygonF& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::polygon_f::PolygonF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_contains(self as *const ::list::ListPolygonF,
                                               t as *const ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPolygonF>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPolygonF>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::polygon_f::PolygonF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPolygonF>::count(const QPolygonF& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPolygonFCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QPolygonF>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_empty(self as *const ::list::ListPolygonF) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPolygonF>::endsWith(const QPolygonF& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::polygon_f::PolygonF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_endsWith(self as *const ::list::ListPolygonF,
                                               t as *const ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPolygonF& QList<QPolygonF>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_first_const(self as *const ::list::ListPolygonF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPolygonF& QList<QPolygonF>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_first(self as *mut ::list::ListPolygonF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QPolygonF& QList<QPolygonF>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_front_const(self as *const ::list::ListPolygonF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPolygonF& QList<QPolygonF>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_front(self as *mut ::list::ListPolygonF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QPolygonF>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::polygon_f::PolygonF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPolygonF>::indexOf(const QPolygonF& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::polygon_f::PolygonF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPolygonF>::indexOf(const QPolygonF& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPolygonFIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::insert(int i, const QPolygonF& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::polygon_f::PolygonF) {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_insert(self as *mut ::list::ListPolygonF,
                                             i,
                                             t as *const ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPolygonF>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_isEmpty(self as *const ::list::ListPolygonF) }
  }

  /// C++ method: <span style='color: green;'>```const QPolygonF& QList<QPolygonF>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_last_const(self as *const ::list::ListPolygonF) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QPolygonF>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::polygon_f::PolygonF) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPolygonF>::lastIndexOf(const QPolygonF& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::polygon_f::PolygonF, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QPolygonF>::lastIndexOf(const QPolygonF& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListPolygonFLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPolygonF& QList<QPolygonF>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_last(self as *mut ::list::ListPolygonF) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QPolygonF>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_length(self as *const ::list::ListPolygonF) }
  }

  /// C++ method: <span style='color: green;'>```QList<QPolygonF>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF> QList<QPolygonF>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF> QList<QPolygonF>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListPolygonF
    where Args: overloading::ListPolygonFMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_move(self as *mut ::list::ListPolygonF, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QPolygonF>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QPolygonF>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListPolygonF) -> ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QPolygonF>::QList(const QList<QPolygonF>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListPolygonF
    where Args: overloading::ListPolygonFNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QPolygonF> QList<QPolygonF>::operator+(const QList<QPolygonF>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListPolygonF) -> ::list::ListPolygonF {
    {
      let mut object: ::list::ListPolygonF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_operator_add_to_output(self as *const ::list::ListPolygonF,
                                                               l as *const ::list::ListPolygonF,
                                                               &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPolygonF>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListPolygonF) -> &'l0 mut ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF>& QList<QPolygonF>::operator+=(const QList<QPolygonF>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::polygon_f::PolygonF) -> &'l0 mut ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF>& QList<QPolygonF>::operator+=(const QPolygonF& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListPolygonF
    where Args: overloading::ListPolygonFOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QPolygonF>& QList<QPolygonF>::operator=(const QList<QPolygonF>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListPolygonF) -> &'l0 mut ::list::ListPolygonF {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_operator_assign(self as *mut ::list::ListPolygonF,
                                                      l as *const ::list::ListPolygonF)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPolygonF>::operator==(const QList<QPolygonF>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListPolygonF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_operator_eq(self as *const ::list::ListPolygonF,
                                                  l as *const ::list::ListPolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```const QPolygonF& QList<QPolygonF>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::polygon_f::PolygonF {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QPolygonF_operator_index_const(self as *const ::list::ListPolygonF, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QPolygonF& QList<QPolygonF>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::polygon_f::PolygonF {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QPolygonF_operator_index(self as *mut ::list::ListPolygonF, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPolygonF>::operator!=(const QList<QPolygonF>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListPolygonF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_operator_neq(self as *const ::list::ListPolygonF,
                                                   l as *const ::list::ListPolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPolygonF>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListPolygonF) -> &'l0 mut ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF>& QList<QPolygonF>::operator<<(const QList<QPolygonF>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::polygon_f::PolygonF) -> &'l0 mut ::list::ListPolygonF```<br>
  /// C++ method: <span style='color: green;'>```QList<QPolygonF>& QList<QPolygonF>::operator<<(const QPolygonF& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListPolygonF
    where Args: overloading::ListPolygonFOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_pop_back(self as *mut ::list::ListPolygonF) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_pop_front(self as *mut ::list::ListPolygonF) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::prepend(const QPolygonF& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::polygon_f::PolygonF) {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_prepend(self as *mut ::list::ListPolygonF,
                                              t as *const ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::push_back(const QPolygonF& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::polygon_f::PolygonF) {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_push_back(self as *mut ::list::ListPolygonF,
                                                t as *const ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::push_front(const QPolygonF& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::polygon_f::PolygonF) {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_push_front(self as *mut ::list::ListPolygonF,
                                                 t as *const ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QPolygonF>::removeAll(const QPolygonF& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::polygon_f::PolygonF) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_removeAll(self as *mut ::list::ListPolygonF,
                                                t as *const ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_removeAt(self as *mut ::list::ListPolygonF, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_removeFirst(self as *mut ::list::ListPolygonF) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_removeLast(self as *mut ::list::ListPolygonF) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPolygonF>::removeOne(const QPolygonF& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::polygon_f::PolygonF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_removeOne(self as *mut ::list::ListPolygonF,
                                                t as *const ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::replace(int i, const QPolygonF& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::polygon_f::PolygonF) {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_replace(self as *mut ::list::ListPolygonF,
                                              i,
                                              t as *const ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_reserve(self as *mut ::list::ListPolygonF, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QPolygonF>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_size(self as *const ::list::ListPolygonF) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QPolygonF>::startsWith(const QPolygonF& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::polygon_f::PolygonF) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QPolygonF_startsWith(self as *const ::list::ListPolygonF,
                                                 t as *const ::polygon_f::PolygonF)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPolygonF>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListPolygonF) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::swap(QList<QPolygonF>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QPolygonF>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListPolygonFSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QPolygonF QList<QPolygonF>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::polygon_f::PolygonF {
    {
      let mut object: ::polygon_f::PolygonF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_takeAt_to_output(self as *mut ::list::ListPolygonF, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPolygonF QList<QPolygonF>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::polygon_f::PolygonF {
    {
      let mut object: ::polygon_f::PolygonF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_takeFirst_to_output(self as *mut ::list::ListPolygonF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QPolygonF QList<QPolygonF>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::polygon_f::PolygonF {
    {
      let mut object: ::polygon_f::PolygonF =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_takeLast_to_output(self as *mut ::list::ListPolygonF, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QPolygonF>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QList<QPolygonF>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::polygon_f::PolygonF)) -> ::polygon_f::PolygonF```<br>
  /// C++ method: <span style='color: green;'>```QPolygonF QList<QPolygonF>::value(int i, const QPolygonF& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::polygon_f::PolygonF
    where Args: overloading::ListPolygonFValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListPolygonF {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QPolygonF>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QPolygonF_destructor(self as *mut ::list::ListPolygonF) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QSize>```</span>
#[repr(C)]
pub struct ListQtCoreSize([u8; ::type_sizes::QT_GUI_LIST_LIST_QT_CORE_SIZE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListQtCoreSize {
  unsafe fn new_uninitialized() -> ListQtCoreSize {
    ListQtCoreSize(::std::mem::uninitialized())
  }
}

impl ListQtCoreSize {
  /// C++ method: <span style='color: green;'>```QList<QSize>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListQtCoreSize) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QSize>::append(const QList<QSize>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::qt_core::size::Size) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QSize>::append(const QSize& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListQtCoreSizeAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QSize& QList<QSize>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_at(self as *const ::list::ListQtCoreSize, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSize& QList<QSize>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_back_const(self as *const ::list::ListQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QList<QSize>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_back(self as *mut ::list::ListQtCoreSize) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QSize>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QSize_clear(self as *mut ::list::ListQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QList<QSize>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_constFirst(self as *const ::list::ListQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QSize& QList<QSize>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_constLast(self as *const ::list::ListQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QSize>::contains(const QSize& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::qt_core::size::Size) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_contains(self as *const ::list::ListQtCoreSize,
                                           t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QSize>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QSize>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_core::size::Size) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QSize>::count(const QSize& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListQtCoreSizeCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QSize>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QSize_empty(self as *const ::list::ListQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QSize>::endsWith(const QSize& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::qt_core::size::Size) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_endsWith(self as *const ::list::ListQtCoreSize,
                                           t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QList<QSize>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_first_const(self as *const ::list::ListQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QList<QSize>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_first(self as *mut ::list::ListQtCoreSize) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QList<QSize> QList<QSize>::fromVector(const QVector<QSize>& vector)```</span>
  ///
  ///
  pub fn from_vector(vector: &::vector::VectorQtCoreSize) -> ::list::ListQtCoreSize {
    {
      let mut object: ::list::ListQtCoreSize =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_fromVector_to_output(vector as *const ::vector::VectorQtCoreSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QList<QSize>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_front_const(self as *const ::list::ListQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QList<QSize>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_front(self as *mut ::list::ListQtCoreSize) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QSize>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::qt_core::size::Size) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QSize>::indexOf(const QSize& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::qt_core::size::Size, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QSize>::indexOf(const QSize& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListQtCoreSizeIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QSize>::insert(int i, const QSize& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_insert(self as *mut ::list::ListQtCoreSize,
                                         i,
                                         t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QSize>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QSize_isEmpty(self as *const ::list::ListQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QList<QSize>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_last_const(self as *const ::list::ListQtCoreSize) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QSize>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::qt_core::size::Size) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QSize>::lastIndexOf(const QSize& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::qt_core::size::Size, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QSize>::lastIndexOf(const QSize& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListQtCoreSizeLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSize& QList<QSize>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_last(self as *mut ::list::ListQtCoreSize) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QSize>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QSize_length(self as *const ::list::ListQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```QList<QSize>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QList<QSize> QList<QSize>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QList<QSize> QList<QSize>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListQtCoreSize
    where Args: overloading::ListQtCoreSizeMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QSize>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QSize_move(self as *mut ::list::ListQtCoreSize, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QSize>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QSize>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListQtCoreSize) -> ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QSize>::QList(const QList<QSize>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListQtCoreSize
    where Args: overloading::ListQtCoreSizeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QSize> QList<QSize>::operator+(const QList<QSize>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListQtCoreSize) -> ::list::ListQtCoreSize {
    {
      let mut object: ::list::ListQtCoreSize =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_operator_add_to_output(self as *const ::list::ListQtCoreSize,
                                                           l as *const ::list::ListQtCoreSize,
                                                           &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QSize>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListQtCoreSize) -> &'l0 mut ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QList<QSize>& QList<QSize>::operator+=(const QList<QSize>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt_core::size::Size) -> &'l0 mut ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QList<QSize>& QList<QSize>::operator+=(const QSize& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListQtCoreSize
    where Args: overloading::ListQtCoreSizeOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QSize>& QList<QSize>::operator=(const QList<QSize>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListQtCoreSize) -> &'l0 mut ::list::ListQtCoreSize {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QSize_operator_assign(self as *mut ::list::ListQtCoreSize,
                                                  l as *const ::list::ListQtCoreSize)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QSize>::operator==(const QList<QSize>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListQtCoreSize) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_operator_eq(self as *const ::list::ListQtCoreSize,
                                              l as *const ::list::ListQtCoreSize)
    }
  }

  /// C++ method: <span style='color: green;'>```const QSize& QList<QSize>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_core::size::Size {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QSize_operator_index_const(self as *const ::list::ListQtCoreSize, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QSize& QList<QSize>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::qt_core::size::Size {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QSize_operator_index(self as *mut ::list::ListQtCoreSize, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QSize>::operator!=(const QList<QSize>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListQtCoreSize) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_operator_neq(self as *const ::list::ListQtCoreSize,
                                               l as *const ::list::ListQtCoreSize)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QSize>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListQtCoreSize) -> &'l0 mut ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QList<QSize>& QList<QSize>::operator<<(const QList<QSize>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::qt_core::size::Size) -> &'l0 mut ::list::ListQtCoreSize```<br>
  /// C++ method: <span style='color: green;'>```QList<QSize>& QList<QSize>::operator<<(const QSize& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListQtCoreSize
    where Args: overloading::ListQtCoreSizeOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QSize>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QSize_pop_back(self as *mut ::list::ListQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QSize>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QSize_pop_front(self as *mut ::list::ListQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QSize>::prepend(const QSize& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_prepend(self as *mut ::list::ListQtCoreSize,
                                          t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QSize>::push_back(const QSize& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_push_back(self as *mut ::list::ListQtCoreSize,
                                            t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QSize>::push_front(const QSize& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_push_front(self as *mut ::list::ListQtCoreSize,
                                             t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QSize>::removeAll(const QSize& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::qt_core::size::Size) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_removeAll(self as *mut ::list::ListQtCoreSize,
                                            t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QSize>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QSize_removeAt(self as *mut ::list::ListQtCoreSize, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QSize>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QSize_removeFirst(self as *mut ::list::ListQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QSize>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QSize_removeLast(self as *mut ::list::ListQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QSize>::removeOne(const QSize& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::qt_core::size::Size) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_removeOne(self as *mut ::list::ListQtCoreSize,
                                            t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QSize>::replace(int i, const QSize& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::qt_core::size::Size) {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_replace(self as *mut ::list::ListQtCoreSize,
                                          i,
                                          t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QSize>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QSize_reserve(self as *mut ::list::ListQtCoreSize, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QSize>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QSize_size(self as *const ::list::ListQtCoreSize) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QSize>::startsWith(const QSize& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::qt_core::size::Size) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QSize_startsWith(self as *const ::list::ListQtCoreSize,
                                             t as *const ::qt_core::size::Size)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QSize>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListQtCoreSize) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QSize>::swap(QList<QSize>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QSize>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListQtCoreSizeSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QSize QList<QSize>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_takeAt_to_output(self as *mut ::list::ListQtCoreSize, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QList<QSize>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_takeFirst_to_output(self as *mut ::list::ListQtCoreSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QSize QList<QSize>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::qt_core::size::Size {
    {
      let mut object: ::qt_core::size::Size =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_takeLast_to_output(self as *mut ::list::ListQtCoreSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QSize> QList<QSize>::toVector() const```</span>
  ///
  ///
  pub fn to_vector(&self) -> ::vector::VectorQtCoreSize {
    {
      let mut object: ::vector::VectorQtCoreSize =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_toVector_to_output(self as *const ::list::ListQtCoreSize, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QSize>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QList<QSize>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::qt_core::size::Size)) -> ::qt_core::size::Size```<br>
  /// C++ method: <span style='color: green;'>```QSize QList<QSize>::value(int i, const QSize& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::qt_core::size::Size
    where Args: overloading::ListQtCoreSizeValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListQtCoreSize {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QSize>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QSize_destructor(self as *mut ::list::ListQtCoreSize) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QScreen*>```</span>
#[repr(C)]
pub struct ListScreenMutPtr([u8; ::type_sizes::QT_GUI_LIST_LIST_SCREEN_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListScreenMutPtr {
  unsafe fn new_uninitialized() -> ListScreenMutPtr {
    ListScreenMutPtr(::std::mem::uninitialized())
  }
}

impl ListScreenMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::append(const QList<QScreen*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListScreenMutPtr) {
    unsafe {
      ::ffi::qt_gui_c_QList_QScreen_ptr_append_QList_QScreen_ptr(self as *mut ::list::ListScreenMutPtr,
                                                                 t as *const ::list::ListScreenMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::append(QScreen* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::screen::Screen) {
    ::ffi::qt_gui_c_QList_QScreen_ptr_append_QScreen(self as *mut ::list::ListScreenMutPtr,
                                                     t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```QScreen* const & QList<QScreen*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::screen::Screen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_at(self as *const ::list::ListScreenMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScreen* const & QList<QScreen*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::screen::Screen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_back_const(self as *const ::list::ListScreenMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScreen*& QList<QScreen*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::screen::Screen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_back(self as *mut ::list::ListScreenMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_clear(self as *mut ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QScreen* const & QList<QScreen*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::screen::Screen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_constFirst(self as *const ::list::ListScreenMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScreen* const & QList<QScreen*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::screen::Screen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_constLast(self as *const ::list::ListScreenMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScreen*>::contains(QScreen* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::screen::Screen) -> bool {
    ::ffi::qt_gui_c_QList_QScreen_ptr_contains(self as *const ::list::ListScreenMutPtr,
                                               t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```int QList<QScreen*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_count_no_args(self as *const ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QScreen*>::count(QScreen* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::screen::Screen) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QScreen_ptr_count_t(self as *const ::list::ListScreenMutPtr,
                                              t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScreen*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_empty(self as *const ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScreen*>::endsWith(QScreen* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::screen::Screen) -> bool {
    ::ffi::qt_gui_c_QList_QScreen_ptr_endsWith(self as *const ::list::ListScreenMutPtr,
                                               t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```QScreen* const & QList<QScreen*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::screen::Screen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_first_const(self as *const ::list::ListScreenMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScreen*& QList<QScreen*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::screen::Screen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_first(self as *mut ::list::ListScreenMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScreen* const & QList<QScreen*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::screen::Screen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_front_const(self as *const ::list::ListScreenMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScreen*& QList<QScreen*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::screen::Screen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_front(self as *mut ::list::ListScreenMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QScreen*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::screen::Screen) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QScreen*>::indexOf(QScreen* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::screen::Screen, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QScreen*>::indexOf(QScreen* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListScreenMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::insert(int i, QScreen* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::screen::Screen) {
    ::ffi::qt_gui_c_QList_QScreen_ptr_insert(self as *mut ::list::ListScreenMutPtr,
                                             i,
                                             t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScreen*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_isEmpty(self as *const ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QScreen* const & QList<QScreen*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::screen::Screen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_last_const(self as *const ::list::ListScreenMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QScreen*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::screen::Screen) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QScreen*>::lastIndexOf(QScreen* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::screen::Screen, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QScreen*>::lastIndexOf(QScreen* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListScreenMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QScreen*& QList<QScreen*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::screen::Screen {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_last(self as *mut ::list::ListScreenMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QScreen*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_length(self as *const ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QScreen*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListScreenMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QScreen*> QList<QScreen*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListScreenMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QScreen*> QList<QScreen*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListScreenMutPtr
    where Args: overloading::ListScreenMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_move(self as *mut ::list::ListScreenMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QScreen*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListScreenMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QScreen*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListScreenMutPtr) -> ::list::ListScreenMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QScreen*>::QList(const QList<QScreen*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListScreenMutPtr
    where Args: overloading::ListScreenMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QScreen*> QList<QScreen*>::operator+(const QList<QScreen*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListScreenMutPtr) -> ::list::ListScreenMutPtr {
    {
      let mut object: ::list::ListScreenMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QScreen_ptr_operator_add_to_output(self as *const ::list::ListScreenMutPtr,
                                                                 l as *const ::list::ListScreenMutPtr,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QScreen*>& QList<QScreen*>::operator+=(const QList<QScreen*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListScreenMutPtr) -> &'l0 mut ::list::ListScreenMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QScreen_ptr_operator_add_assign_l(self as *mut ::list::ListScreenMutPtr,
                                                              l as *const ::list::ListScreenMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QScreen*>& QList<QScreen*>::operator+=(QScreen* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::screen::Screen)
                                               -> &'l0 mut ::list::ListScreenMutPtr {
    let ffi_result = ::ffi::qt_gui_c_QList_QScreen_ptr_operator_add_assign_t(self as *mut ::list::ListScreenMutPtr,
                                                                             t as *const *mut ::screen::Screen);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QScreen*>& QList<QScreen*>::operator=(const QList<QScreen*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListScreenMutPtr) -> &'l0 mut ::list::ListScreenMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QScreen_ptr_operator_assign(self as *mut ::list::ListScreenMutPtr,
                                                        l as *const ::list::ListScreenMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScreen*>::operator==(const QList<QScreen*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListScreenMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QScreen_ptr_operator_eq(self as *const ::list::ListScreenMutPtr,
                                                    l as *const ::list::ListScreenMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QScreen* const & QList<QScreen*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::screen::Screen {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_operator_index_const(self as *const ::list::ListScreenMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QScreen*& QList<QScreen*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::screen::Screen {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_operator_index(self as *mut ::list::ListScreenMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScreen*>::operator!=(const QList<QScreen*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListScreenMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QScreen_ptr_operator_neq(self as *const ::list::ListScreenMutPtr,
                                                     l as *const ::list::ListScreenMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QScreen*>& QList<QScreen*>::operator<<(const QList<QScreen*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListScreenMutPtr) -> &'l0 mut ::list::ListScreenMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QScreen_ptr_operator_shl_l(self as *mut ::list::ListScreenMutPtr,
                                                       l as *const ::list::ListScreenMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QScreen*>& QList<QScreen*>::operator<<(QScreen* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::screen::Screen)
                                        -> &'l0 mut ::list::ListScreenMutPtr {
    let ffi_result = ::ffi::qt_gui_c_QList_QScreen_ptr_operator_shl_t(self as *mut ::list::ListScreenMutPtr,
                                                                      t as *const *mut ::screen::Screen);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_pop_back(self as *mut ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_pop_front(self as *mut ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::prepend(QScreen* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::screen::Screen) {
    ::ffi::qt_gui_c_QList_QScreen_ptr_prepend(self as *mut ::list::ListScreenMutPtr,
                                              t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::push_back(QScreen* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::screen::Screen) {
    ::ffi::qt_gui_c_QList_QScreen_ptr_push_back(self as *mut ::list::ListScreenMutPtr,
                                                t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::push_front(QScreen* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::screen::Screen) {
    ::ffi::qt_gui_c_QList_QScreen_ptr_push_front(self as *mut ::list::ListScreenMutPtr,
                                                 t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```int QList<QScreen*>::removeAll(QScreen* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::screen::Screen) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QScreen_ptr_removeAll(self as *mut ::list::ListScreenMutPtr,
                                                t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_removeAt(self as *mut ::list::ListScreenMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_removeFirst(self as *mut ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_removeLast(self as *mut ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScreen*>::removeOne(QScreen* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::screen::Screen) -> bool {
    ::ffi::qt_gui_c_QList_QScreen_ptr_removeOne(self as *mut ::list::ListScreenMutPtr,
                                                t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::replace(int i, QScreen* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::screen::Screen) {
    ::ffi::qt_gui_c_QList_QScreen_ptr_replace(self as *mut ::list::ListScreenMutPtr,
                                              i,
                                              t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_reserve(self as *mut ::list::ListScreenMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QScreen*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_size(self as *const ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QScreen*>::startsWith(QScreen* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::screen::Screen) -> bool {
    ::ffi::qt_gui_c_QList_QScreen_ptr_startsWith(self as *const ::list::ListScreenMutPtr,
                                                 t as *const *mut ::screen::Screen)
  }

  /// C++ method: <span style='color: green;'>```QList<QScreen*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListScreenMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::swap(QList<QScreen*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QScreen*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListScreenMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QScreen* QList<QScreen*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::screen::Screen {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_takeAt(self as *mut ::list::ListScreenMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QScreen* QList<QScreen*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::screen::Screen {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_takeFirst(self as *mut ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QScreen* QList<QScreen*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::screen::Screen {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_takeLast(self as *mut ::list::ListScreenMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QScreen* QList<QScreen*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::screen::Screen {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_value_i(self as *const ::list::ListScreenMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QScreen* QList<QScreen*>::value(int i, QScreen* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self, i: ::libc::c_int, default_value: &*mut ::screen::Screen) -> *mut ::screen::Screen {
    ::ffi::qt_gui_c_QList_QScreen_ptr_value_i_defaultValue(self as *const ::list::ListScreenMutPtr,
                                                           i,
                                                           default_value as *const *mut ::screen::Screen)
  }
}

impl Drop for ::list::ListScreenMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QScreen*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_destructor(self as *mut ::list::ListScreenMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QStandardItem*>```</span>
#[repr(C)]
pub struct ListStandardItemMutPtr([u8; ::type_sizes::QT_GUI_LIST_LIST_STANDARD_ITEM_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListStandardItemMutPtr {
  unsafe fn new_uninitialized() -> ListStandardItemMutPtr {
    ListStandardItemMutPtr(::std::mem::uninitialized())
  }
}

impl ListStandardItemMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::append(const QList<QStandardItem*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListStandardItemMutPtr) {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_append_QList_QStandardItem_ptr(self as *mut ::list::ListStandardItemMutPtr, t as *const ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::append(QStandardItem* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_append_QStandardItem(self as *mut ::list::ListStandardItemMutPtr,
                                                                 t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* const & QList<QStandardItem*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_at(self as *const ::list::ListStandardItemMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* const & QList<QStandardItem*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_back_const(self as *const ::list::ListStandardItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStandardItem*& QList<QStandardItem*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_back(self as *mut ::list::ListStandardItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_clear(self as *mut ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* const & QList<QStandardItem*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_constFirst(self as *const ::list::ListStandardItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* const & QList<QStandardItem*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_constLast(self as *const ::list::ListStandardItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStandardItem*>::contains(QStandardItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::standard_item::StandardItem) -> bool {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_contains(self as *const ::list::ListStandardItemMutPtr,
                                                     t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```int QList<QStandardItem*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_count_no_args(self as *const ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QStandardItem*>::count(QStandardItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::standard_item::StandardItem) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_count_t(self as *const ::list::ListStandardItemMutPtr,
                                                    t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStandardItem*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_empty(self as *const ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStandardItem*>::endsWith(QStandardItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::standard_item::StandardItem) -> bool {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_endsWith(self as *const ::list::ListStandardItemMutPtr,
                                                     t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* const & QList<QStandardItem*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_first_const(self as *const ::list::ListStandardItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStandardItem*& QList<QStandardItem*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_first(self as *mut ::list::ListStandardItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* const & QList<QStandardItem*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_front_const(self as *const ::list::ListStandardItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStandardItem*& QList<QStandardItem*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_front(self as *mut ::list::ListStandardItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::standard_item::StandardItem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QStandardItem*>::indexOf(QStandardItem* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::standard_item::StandardItem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QStandardItem*>::indexOf(QStandardItem* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListStandardItemMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::insert(int i, QStandardItem* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_insert(self as *mut ::list::ListStandardItemMutPtr,
                                                   i,
                                                   t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStandardItem*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_isEmpty(self as *const ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* const & QList<QStandardItem*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_last_const(self as *const ::list::ListStandardItemMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::standard_item::StandardItem) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QStandardItem*>::lastIndexOf(QStandardItem* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::standard_item::StandardItem, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QStandardItem*>::lastIndexOf(QStandardItem* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListStandardItemMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStandardItem*& QList<QStandardItem*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_last(self as *mut ::list::ListStandardItemMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QStandardItem*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_length(self as *const ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListStandardItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QStandardItem*> QList<QStandardItem*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListStandardItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QStandardItem*> QList<QStandardItem*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListStandardItemMutPtr
    where Args: overloading::ListStandardItemMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_move(self as *mut ::list::ListStandardItemMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListStandardItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QStandardItem*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListStandardItemMutPtr) -> ::list::ListStandardItemMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QStandardItem*>::QList(const QList<QStandardItem*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListStandardItemMutPtr
    where Args: overloading::ListStandardItemMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QStandardItem*> QList<QStandardItem*>::operator+(const QList<QStandardItem*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListStandardItemMutPtr) -> ::list::ListStandardItemMutPtr {
    {
      let mut object: ::list::ListStandardItemMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QStandardItem_ptr_operator_add_to_output(self as *const ::list::ListStandardItemMutPtr,
                                                                       l as *const ::list::ListStandardItemMutPtr,
                                                                       &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*>& QList<QStandardItem*>::operator+=(const QList<QStandardItem*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListStandardItemMutPtr)
                                 -> &'l0 mut ::list::ListStandardItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QStandardItem_ptr_operator_add_assign_l(self as *mut ::list::ListStandardItemMutPtr,
                                                                      l as *const ::list::ListStandardItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*>& QList<QStandardItem*>::operator+=(QStandardItem* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::standard_item::StandardItem)
                                               -> &'l0 mut ::list::ListStandardItemMutPtr {
    let ffi_result =
      ::ffi::qt_gui_c_QList_QStandardItem_ptr_operator_add_assign_t(self as *mut ::list::ListStandardItemMutPtr,
                                                                    t as *const *mut ::standard_item::StandardItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*>& QList<QStandardItem*>::operator=(const QList<QStandardItem*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListStandardItemMutPtr)
                             -> &'l0 mut ::list::ListStandardItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QStandardItem_ptr_operator_assign(self as *mut ::list::ListStandardItemMutPtr,
                                                                l as *const ::list::ListStandardItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStandardItem*>::operator==(const QList<QStandardItem*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListStandardItemMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QStandardItem_ptr_operator_eq(self as *const ::list::ListStandardItemMutPtr,
                                                          l as *const ::list::ListStandardItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* const & QList<QStandardItem*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QStandardItem_ptr_operator_index_const(self as *const ::list::ListStandardItemMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QStandardItem*& QList<QStandardItem*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::standard_item::StandardItem {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_operator_index(self as *mut ::list::ListStandardItemMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStandardItem*>::operator!=(const QList<QStandardItem*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListStandardItemMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QStandardItem_ptr_operator_neq(self as *const ::list::ListStandardItemMutPtr,
                                                           l as *const ::list::ListStandardItemMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*>& QList<QStandardItem*>::operator<<(const QList<QStandardItem*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::list::ListStandardItemMutPtr)
                          -> &'l0 mut ::list::ListStandardItemMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QStandardItem_ptr_operator_shl_l(self as *mut ::list::ListStandardItemMutPtr,
                                                               l as *const ::list::ListStandardItemMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*>& QList<QStandardItem*>::operator<<(QStandardItem* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::standard_item::StandardItem)
                                        -> &'l0 mut ::list::ListStandardItemMutPtr {
    let ffi_result =
      ::ffi::qt_gui_c_QList_QStandardItem_ptr_operator_shl_t(self as *mut ::list::ListStandardItemMutPtr,
                                                             t as *const *mut ::standard_item::StandardItem);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_pop_back(self as *mut ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_pop_front(self as *mut ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::prepend(QStandardItem* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_prepend(self as *mut ::list::ListStandardItemMutPtr,
                                                    t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::push_back(QStandardItem* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_push_back(self as *mut ::list::ListStandardItemMutPtr,
                                                      t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::push_front(QStandardItem* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_push_front(self as *mut ::list::ListStandardItemMutPtr,
                                                       t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```int QList<QStandardItem*>::removeAll(QStandardItem* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::standard_item::StandardItem) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_removeAll(self as *mut ::list::ListStandardItemMutPtr,
                                                      t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_removeAt(self as *mut ::list::ListStandardItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_removeFirst(self as *mut ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_removeLast(self as *mut ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStandardItem*>::removeOne(QStandardItem* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::standard_item::StandardItem) -> bool {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_removeOne(self as *mut ::list::ListStandardItemMutPtr,
                                                      t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::replace(int i, QStandardItem* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::standard_item::StandardItem) {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_replace(self as *mut ::list::ListStandardItemMutPtr,
                                                    i,
                                                    t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_reserve(self as *mut ::list::ListStandardItemMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QStandardItem*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_size(self as *const ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QStandardItem*>::startsWith(QStandardItem* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::standard_item::StandardItem) -> bool {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_startsWith(self as *const ::list::ListStandardItemMutPtr,
                                                       t as *const *mut ::standard_item::StandardItem)
  }

  /// C++ method: <span style='color: green;'>```QList<QStandardItem*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListStandardItemMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::swap(QList<QStandardItem*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QStandardItem*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListStandardItemMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QStandardItem* QList<QStandardItem*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::standard_item::StandardItem {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_takeAt(self as *mut ::list::ListStandardItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* QList<QStandardItem*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::standard_item::StandardItem {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_takeFirst(self as *mut ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* QList<QStandardItem*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::standard_item::StandardItem {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_takeLast(self as *mut ::list::ListStandardItemMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* QList<QStandardItem*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::standard_item::StandardItem {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_value_i(self as *const ::list::ListStandardItemMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QStandardItem* QList<QStandardItem*>::value(int i, QStandardItem* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::standard_item::StandardItem)
                             -> *mut ::standard_item::StandardItem {
    ::ffi::qt_gui_c_QList_QStandardItem_ptr_value_i_defaultValue(self as *const ::list::ListStandardItemMutPtr, i, default_value as *const *mut ::standard_item::StandardItem)
  }
}

impl Drop for ::list::ListStandardItemMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QStandardItem*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QStandardItem_ptr_destructor(self as *mut ::list::ListStandardItemMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QTextBlock>```</span>
#[repr(C)]
pub struct ListTextBlock([u8; ::type_sizes::QT_GUI_LIST_LIST_TEXT_BLOCK]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListTextBlock {
  unsafe fn new_uninitialized() -> ListTextBlock {
    ListTextBlock(::std::mem::uninitialized())
  }
}

impl ListTextBlock {
  /// C++ method: <span style='color: green;'>```QList<QTextBlock>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListTextBlock) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::append(const QList<QTextBlock>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::text_block::TextBlock) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::append(const QTextBlock& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTextBlockAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextBlock& QList<QTextBlock>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_at(self as *const ::list::ListTextBlock, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextBlock& QList<QTextBlock>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_back_const(self as *const ::list::ListTextBlock) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextBlock& QList<QTextBlock>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_back(self as *mut ::list::ListTextBlock) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_clear(self as *mut ::list::ListTextBlock) }
  }

  /// C++ method: <span style='color: green;'>```const QTextBlock& QList<QTextBlock>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_constFirst(self as *const ::list::ListTextBlock) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextBlock& QList<QTextBlock>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_constLast(self as *const ::list::ListTextBlock) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextBlock>::contains(const QTextBlock& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::text_block::TextBlock) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_contains(self as *const ::list::ListTextBlock,
                                                t as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextBlock>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextBlock>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::text_block::TextBlock) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextBlock>::count(const QTextBlock& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTextBlockCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QTextBlock>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_empty(self as *const ::list::ListTextBlock) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextBlock>::endsWith(const QTextBlock& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::text_block::TextBlock) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_endsWith(self as *const ::list::ListTextBlock,
                                                t as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextBlock& QList<QTextBlock>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_first_const(self as *const ::list::ListTextBlock) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextBlock& QList<QTextBlock>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_first(self as *mut ::list::ListTextBlock) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextBlock& QList<QTextBlock>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_front_const(self as *const ::list::ListTextBlock) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextBlock& QList<QTextBlock>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_front(self as *mut ::list::ListTextBlock) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTextBlock>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::text_block::TextBlock) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextBlock>::indexOf(const QTextBlock& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::text_block::TextBlock, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextBlock>::indexOf(const QTextBlock& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTextBlockIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::insert(int i, const QTextBlock& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::text_block::TextBlock) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_insert(self as *mut ::list::ListTextBlock,
                                              i,
                                              t as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextBlock>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_isEmpty(self as *const ::list::ListTextBlock) }
  }

  /// C++ method: <span style='color: green;'>```const QTextBlock& QList<QTextBlock>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_last_const(self as *const ::list::ListTextBlock) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTextBlock>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::text_block::TextBlock) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextBlock>::lastIndexOf(const QTextBlock& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::text_block::TextBlock, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextBlock>::lastIndexOf(const QTextBlock& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTextBlockLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextBlock& QList<QTextBlock>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_last(self as *mut ::list::ListTextBlock) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextBlock>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_length(self as *const ::list::ListTextBlock) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextBlock>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListTextBlock```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextBlock> QList<QTextBlock>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListTextBlock```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextBlock> QList<QTextBlock>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListTextBlock
    where Args: overloading::ListTextBlockMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_move(self as *mut ::list::ListTextBlock, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextBlock>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListTextBlock```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTextBlock>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListTextBlock) -> ::list::ListTextBlock```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTextBlock>::QList(const QList<QTextBlock>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListTextBlock
    where Args: overloading::ListTextBlockNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QTextBlock> QList<QTextBlock>::operator+(const QList<QTextBlock>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListTextBlock) -> ::list::ListTextBlock {
    {
      let mut object: ::list::ListTextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_operator_add_to_output(self as *const ::list::ListTextBlock,
                                                                l as *const ::list::ListTextBlock,
                                                                &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextBlock>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListTextBlock) -> &'l0 mut ::list::ListTextBlock```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextBlock>& QList<QTextBlock>::operator+=(const QList<QTextBlock>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::text_block::TextBlock) -> &'l0 mut ::list::ListTextBlock```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextBlock>& QList<QTextBlock>::operator+=(const QTextBlock& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTextBlock
    where Args: overloading::ListTextBlockOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QTextBlock>& QList<QTextBlock>::operator=(const QList<QTextBlock>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListTextBlock) -> &'l0 mut ::list::ListTextBlock {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_operator_assign(self as *mut ::list::ListTextBlock,
                                                       l as *const ::list::ListTextBlock)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextBlock>::operator==(const QList<QTextBlock>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListTextBlock) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_operator_eq(self as *const ::list::ListTextBlock,
                                                   l as *const ::list::ListTextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextBlock& QList<QTextBlock>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_block::TextBlock {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextBlock_operator_index_const(self as *const ::list::ListTextBlock, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextBlock& QList<QTextBlock>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::text_block::TextBlock {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextBlock_operator_index(self as *mut ::list::ListTextBlock, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextBlock>::operator!=(const QList<QTextBlock>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListTextBlock) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_operator_neq(self as *const ::list::ListTextBlock,
                                                    l as *const ::list::ListTextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextBlock>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListTextBlock) -> &'l0 mut ::list::ListTextBlock```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextBlock>& QList<QTextBlock>::operator<<(const QList<QTextBlock>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::text_block::TextBlock) -> &'l0 mut ::list::ListTextBlock```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextBlock>& QList<QTextBlock>::operator<<(const QTextBlock& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTextBlock
    where Args: overloading::ListTextBlockOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_pop_back(self as *mut ::list::ListTextBlock) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_pop_front(self as *mut ::list::ListTextBlock) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::prepend(const QTextBlock& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::text_block::TextBlock) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_prepend(self as *mut ::list::ListTextBlock,
                                               t as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::push_back(const QTextBlock& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::text_block::TextBlock) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_push_back(self as *mut ::list::ListTextBlock,
                                                 t as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::push_front(const QTextBlock& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::text_block::TextBlock) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_push_front(self as *mut ::list::ListTextBlock,
                                                  t as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextBlock>::removeAll(const QTextBlock& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::text_block::TextBlock) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_removeAll(self as *mut ::list::ListTextBlock,
                                                 t as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_removeAt(self as *mut ::list::ListTextBlock, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_removeFirst(self as *mut ::list::ListTextBlock) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_removeLast(self as *mut ::list::ListTextBlock) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextBlock>::removeOne(const QTextBlock& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::text_block::TextBlock) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_removeOne(self as *mut ::list::ListTextBlock,
                                                 t as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::replace(int i, const QTextBlock& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::text_block::TextBlock) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_replace(self as *mut ::list::ListTextBlock,
                                               i,
                                               t as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_reserve(self as *mut ::list::ListTextBlock, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextBlock>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_size(self as *const ::list::ListTextBlock) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextBlock>::startsWith(const QTextBlock& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::text_block::TextBlock) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextBlock_startsWith(self as *const ::list::ListTextBlock,
                                                  t as *const ::text_block::TextBlock)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextBlock>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListTextBlock) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::swap(QList<QTextBlock>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextBlock>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTextBlockSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextBlock QList<QTextBlock>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_takeAt_to_output(self as *mut ::list::ListTextBlock, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock QList<QTextBlock>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_takeFirst_to_output(self as *mut ::list::ListTextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextBlock QList<QTextBlock>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::text_block::TextBlock {
    {
      let mut object: ::text_block::TextBlock =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_takeLast_to_output(self as *mut ::list::ListTextBlock, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextBlock>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::text_block::TextBlock```<br>
  /// C++ method: <span style='color: green;'>```QTextBlock QList<QTextBlock>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::text_block::TextBlock)) -> ::text_block::TextBlock```<br>
  /// C++ method: <span style='color: green;'>```QTextBlock QList<QTextBlock>::value(int i, const QTextBlock& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::text_block::TextBlock
    where Args: overloading::ListTextBlockValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListTextBlock {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QTextBlock>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextBlock_destructor(self as *mut ::list::ListTextBlock) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QTextFrame*>```</span>
#[repr(C)]
pub struct ListTextFrameMutPtr([u8; ::type_sizes::QT_GUI_LIST_LIST_TEXT_FRAME_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListTextFrameMutPtr {
  unsafe fn new_uninitialized() -> ListTextFrameMutPtr {
    ListTextFrameMutPtr(::std::mem::uninitialized())
  }
}

impl ListTextFrameMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::append(const QList<QTextFrame*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListTextFrameMutPtr) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextFrame_ptr_append_QList_QTextFrame_ptr(self as *mut ::list::ListTextFrameMutPtr,
                                                                       t as *const ::list::ListTextFrameMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::append(QTextFrame* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::text_frame::TextFrame) {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_append_QTextFrame(self as *mut ::list::ListTextFrameMutPtr,
                                                           t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* const & QList<QTextFrame*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::text_frame::TextFrame {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_at(self as *const ::list::ListTextFrameMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* const & QList<QTextFrame*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::text_frame::TextFrame {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_back_const(self as *const ::list::ListTextFrameMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFrame*& QList<QTextFrame*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::text_frame::TextFrame {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_back(self as *mut ::list::ListTextFrameMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_clear(self as *mut ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* const & QList<QTextFrame*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::text_frame::TextFrame {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_constFirst(self as *const ::list::ListTextFrameMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* const & QList<QTextFrame*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::text_frame::TextFrame {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_constLast(self as *const ::list::ListTextFrameMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextFrame*>::contains(QTextFrame* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::text_frame::TextFrame) -> bool {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_contains(self as *const ::list::ListTextFrameMutPtr,
                                                  t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextFrame*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_count_no_args(self as *const ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextFrame*>::count(QTextFrame* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::text_frame::TextFrame) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_count_t(self as *const ::list::ListTextFrameMutPtr,
                                                 t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextFrame*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_empty(self as *const ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextFrame*>::endsWith(QTextFrame* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::text_frame::TextFrame) -> bool {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_endsWith(self as *const ::list::ListTextFrameMutPtr,
                                                  t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* const & QList<QTextFrame*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::text_frame::TextFrame {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_first_const(self as *const ::list::ListTextFrameMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFrame*& QList<QTextFrame*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::text_frame::TextFrame {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_first(self as *mut ::list::ListTextFrameMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* const & QList<QTextFrame*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::text_frame::TextFrame {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_front_const(self as *const ::list::ListTextFrameMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFrame*& QList<QTextFrame*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::text_frame::TextFrame {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_front(self as *mut ::list::ListTextFrameMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTextFrame*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::text_frame::TextFrame) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextFrame*>::indexOf(QTextFrame* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::text_frame::TextFrame, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextFrame*>::indexOf(QTextFrame* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTextFrameMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::insert(int i, QTextFrame* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::text_frame::TextFrame) {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_insert(self as *mut ::list::ListTextFrameMutPtr,
                                                i,
                                                t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextFrame*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_isEmpty(self as *const ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* const & QList<QTextFrame*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::text_frame::TextFrame {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_last_const(self as *const ::list::ListTextFrameMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTextFrame*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::text_frame::TextFrame) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextFrame*>::lastIndexOf(QTextFrame* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::text_frame::TextFrame, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextFrame*>::lastIndexOf(QTextFrame* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTextFrameMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextFrame*& QList<QTextFrame*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::text_frame::TextFrame {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_last(self as *mut ::list::ListTextFrameMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextFrame*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_length(self as *const ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextFrame*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListTextFrameMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextFrame*> QList<QTextFrame*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListTextFrameMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextFrame*> QList<QTextFrame*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListTextFrameMutPtr
    where Args: overloading::ListTextFrameMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_move(self as *mut ::list::ListTextFrameMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextFrame*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListTextFrameMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTextFrame*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListTextFrameMutPtr) -> ::list::ListTextFrameMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTextFrame*>::QList(const QList<QTextFrame*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListTextFrameMutPtr
    where Args: overloading::ListTextFrameMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QTextFrame*> QList<QTextFrame*>::operator+(const QList<QTextFrame*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListTextFrameMutPtr) -> ::list::ListTextFrameMutPtr {
    {
      let mut object: ::list::ListTextFrameMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTextFrame_ptr_operator_add_to_output(self as *const ::list::ListTextFrameMutPtr,
                                                                    l as *const ::list::ListTextFrameMutPtr,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextFrame*>& QList<QTextFrame*>::operator+=(const QList<QTextFrame*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListTextFrameMutPtr)
                                 -> &'l0 mut ::list::ListTextFrameMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextFrame_ptr_operator_add_assign_l(self as *mut ::list::ListTextFrameMutPtr,
                                                                   l as *const ::list::ListTextFrameMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTextFrame*>& QList<QTextFrame*>::operator+=(QTextFrame* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::text_frame::TextFrame)
                                               -> &'l0 mut ::list::ListTextFrameMutPtr {
    let ffi_result =
      ::ffi::qt_gui_c_QList_QTextFrame_ptr_operator_add_assign_t(self as *mut ::list::ListTextFrameMutPtr,
                                                                 t as *const *mut ::text_frame::TextFrame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTextFrame*>& QList<QTextFrame*>::operator=(const QList<QTextFrame*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListTextFrameMutPtr)
                             -> &'l0 mut ::list::ListTextFrameMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QTextFrame_ptr_operator_assign(self as *mut ::list::ListTextFrameMutPtr,
                                                           l as *const ::list::ListTextFrameMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextFrame*>::operator==(const QList<QTextFrame*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListTextFrameMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextFrame_ptr_operator_eq(self as *const ::list::ListTextFrameMutPtr,
                                                       l as *const ::list::ListTextFrameMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* const & QList<QTextFrame*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::text_frame::TextFrame {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextFrame_ptr_operator_index_const(self as *const ::list::ListTextFrameMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextFrame*& QList<QTextFrame*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::text_frame::TextFrame {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_operator_index(self as *mut ::list::ListTextFrameMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextFrame*>::operator!=(const QList<QTextFrame*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListTextFrameMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextFrame_ptr_operator_neq(self as *const ::list::ListTextFrameMutPtr,
                                                        l as *const ::list::ListTextFrameMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextFrame*>& QList<QTextFrame*>::operator<<(const QList<QTextFrame*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListTextFrameMutPtr) -> &'l0 mut ::list::ListTextFrameMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QTextFrame_ptr_operator_shl_l(self as *mut ::list::ListTextFrameMutPtr,
                                                          l as *const ::list::ListTextFrameMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTextFrame*>& QList<QTextFrame*>::operator<<(QTextFrame* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::text_frame::TextFrame)
                                        -> &'l0 mut ::list::ListTextFrameMutPtr {
    let ffi_result = ::ffi::qt_gui_c_QList_QTextFrame_ptr_operator_shl_t(self as *mut ::list::ListTextFrameMutPtr,
                                                                         t as *const *mut ::text_frame::TextFrame);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_pop_back(self as *mut ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_pop_front(self as *mut ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::prepend(QTextFrame* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::text_frame::TextFrame) {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_prepend(self as *mut ::list::ListTextFrameMutPtr,
                                                 t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::push_back(QTextFrame* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::text_frame::TextFrame) {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_push_back(self as *mut ::list::ListTextFrameMutPtr,
                                                   t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::push_front(QTextFrame* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::text_frame::TextFrame) {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_push_front(self as *mut ::list::ListTextFrameMutPtr,
                                                    t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextFrame*>::removeAll(QTextFrame* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::text_frame::TextFrame) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_removeAll(self as *mut ::list::ListTextFrameMutPtr,
                                                   t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_removeAt(self as *mut ::list::ListTextFrameMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_removeFirst(self as *mut ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_removeLast(self as *mut ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextFrame*>::removeOne(QTextFrame* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::text_frame::TextFrame) -> bool {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_removeOne(self as *mut ::list::ListTextFrameMutPtr,
                                                   t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::replace(int i, QTextFrame* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::text_frame::TextFrame) {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_replace(self as *mut ::list::ListTextFrameMutPtr,
                                                 i,
                                                 t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_reserve(self as *mut ::list::ListTextFrameMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextFrame*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_size(self as *const ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextFrame*>::startsWith(QTextFrame* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::text_frame::TextFrame) -> bool {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_startsWith(self as *const ::list::ListTextFrameMutPtr,
                                                    t as *const *mut ::text_frame::TextFrame)
  }

  /// C++ method: <span style='color: green;'>```QList<QTextFrame*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListTextFrameMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::swap(QList<QTextFrame*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextFrame*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTextFrameMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextFrame* QList<QTextFrame*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::text_frame::TextFrame {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_takeAt(self as *mut ::list::ListTextFrameMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* QList<QTextFrame*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::text_frame::TextFrame {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_takeFirst(self as *mut ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* QList<QTextFrame*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::text_frame::TextFrame {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_takeLast(self as *mut ::list::ListTextFrameMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* QList<QTextFrame*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::text_frame::TextFrame {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_value_i(self as *const ::list::ListTextFrameMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QTextFrame* QList<QTextFrame*>::value(int i, QTextFrame* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::text_frame::TextFrame)
                             -> *mut ::text_frame::TextFrame {
    ::ffi::qt_gui_c_QList_QTextFrame_ptr_value_i_defaultValue(self as *const ::list::ListTextFrameMutPtr,
                                                              i,
                                                              default_value as *const *mut ::text_frame::TextFrame)
  }
}

impl Drop for ::list::ListTextFrameMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QTextFrame*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_destructor(self as *mut ::list::ListTextFrameMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QTextLayout::FormatRange>```</span>
#[repr(C)]
pub struct ListTextLayoutFormatRange([u8; ::type_sizes::QT_GUI_LIST_LIST_TEXT_LAYOUT_FORMAT_RANGE]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListTextLayoutFormatRange {
  unsafe fn new_uninitialized() -> ListTextLayoutFormatRange {
    ListTextLayoutFormatRange(::std::mem::uninitialized())
  }
}

impl ListTextLayoutFormatRange {
  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListTextLayoutFormatRange) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::append(const QList<QTextLayout::FormatRange>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::text_layout::FormatRange) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::append(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTextLayoutFormatRangeAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_at(self as *const ::list::ListTextLayoutFormatRange, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_back_const(self as *const ::list::ListTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_layout::FormatRange {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_back(self as *mut ::list::ListTextLayoutFormatRange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_clear(self as *mut ::list::ListTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_constFirst(self as *const ::list::ListTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_constLast(self as *const ::list::ListTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextLayout::FormatRange>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_count(self as *const ::list::ListTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextLayout::FormatRange>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_empty(self as *const ::list::ListTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_first_const(self as *const ::list::ListTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_layout::FormatRange {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_first(self as *mut ::list::ListTextLayoutFormatRange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```static QList<QTextLayout::FormatRange> QList<QTextLayout::FormatRange>::fromVector(const QVector<QTextLayout::FormatRange>& vector)```</span>
  ///
  ///
  pub fn from_vector(vector: &::vector::VectorTextLayoutFormatRange) -> ::list::ListTextLayoutFormatRange {
    {
      let mut object: ::list::ListTextLayoutFormatRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_fromVector_to_output(vector as *const ::vector::VectorTextLayoutFormatRange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_front_const(self as *const ::list::ListTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_layout::FormatRange {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_front(self as *mut ::list::ListTextLayoutFormatRange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::insert(int i, const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::text_layout::FormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_insert(self as *mut ::list::ListTextLayoutFormatRange,
                                                           i,
                                                           t as *const ::text_layout::FormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextLayout::FormatRange>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_isEmpty(self as *const ::list::ListTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_last_const(self as *const ::list::ListTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_layout::FormatRange {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_last(self as *mut ::list::ListTextLayoutFormatRange) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextLayout::FormatRange>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_length(self as *const ::list::ListTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange> QList<QTextLayout::FormatRange>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange> QList<QTextLayout::FormatRange>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListTextLayoutFormatRange
    where Args: overloading::ListTextLayoutFormatRangeMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_move(self as *mut ::list::ListTextLayoutFormatRange, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTextLayout::FormatRange>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListTextLayoutFormatRange) -> ::list::ListTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTextLayout::FormatRange>::QList(const QList<QTextLayout::FormatRange>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListTextLayoutFormatRange
    where Args: overloading::ListTextLayoutFormatRangeNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange> QList<QTextLayout::FormatRange>::operator+(const QList<QTextLayout::FormatRange>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListTextLayoutFormatRange) -> ::list::ListTextLayoutFormatRange {
    {
      let mut object: ::list::ListTextLayoutFormatRange =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_operator_add_to_output(self as *const ::list::ListTextLayoutFormatRange, l as *const ::list::ListTextLayoutFormatRange, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListTextLayoutFormatRange) -> &'l0 mut ::list::ListTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange>& QList<QTextLayout::FormatRange>::operator+=(const QList<QTextLayout::FormatRange>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::text_layout::FormatRange) -> &'l0 mut ::list::ListTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange>& QList<QTextLayout::FormatRange>::operator+=(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTextLayoutFormatRange
    where Args: overloading::ListTextLayoutFormatRangeOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange>& QList<QTextLayout::FormatRange>::operator=(const QList<QTextLayout::FormatRange>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListTextLayoutFormatRange)
                             -> &'l0 mut ::list::ListTextLayoutFormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_operator_assign(self as *mut ::list::ListTextLayoutFormatRange,
                                                                      l as *const ::list::ListTextLayoutFormatRange)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_layout::FormatRange {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_operator_index_const(self as *const ::list::ListTextLayoutFormatRange, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange& QList<QTextLayout::FormatRange>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::text_layout::FormatRange {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_operator_index(self as *mut ::list::ListTextLayoutFormatRange, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListTextLayoutFormatRange) -> &'l0 mut ::list::ListTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange>& QList<QTextLayout::FormatRange>::operator<<(const QList<QTextLayout::FormatRange>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::text_layout::FormatRange) -> &'l0 mut ::list::ListTextLayoutFormatRange```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange>& QList<QTextLayout::FormatRange>::operator<<(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTextLayoutFormatRange
    where Args: overloading::ListTextLayoutFormatRangeOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_pop_back(self as *mut ::list::ListTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_pop_front(self as *mut ::list::ListTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::prepend(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::text_layout::FormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_prepend(self as *mut ::list::ListTextLayoutFormatRange,
                                                            t as *const ::text_layout::FormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::push_back(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::text_layout::FormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_push_back(self as *mut ::list::ListTextLayoutFormatRange,
                                                              t as *const ::text_layout::FormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::push_front(const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::text_layout::FormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_push_front(self as *mut ::list::ListTextLayoutFormatRange,
                                                               t as *const ::text_layout::FormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_removeAt(self as *mut ::list::ListTextLayoutFormatRange, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_removeFirst(self as *mut ::list::ListTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_removeLast(self as *mut ::list::ListTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::replace(int i, const QTextLayout::FormatRange& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::text_layout::FormatRange) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_replace(self as *mut ::list::ListTextLayoutFormatRange,
                                                            i,
                                                            t as *const ::text_layout::FormatRange)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_reserve(self as *mut ::list::ListTextLayoutFormatRange, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextLayout::FormatRange>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_size(self as *const ::list::ListTextLayoutFormatRange) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextLayout::FormatRange>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListTextLayoutFormatRange) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::swap(QList<QTextLayout::FormatRange>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextLayout::FormatRange>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTextLayoutFormatRangeSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange QList<QTextLayout::FormatRange>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::cpp_utils::CppBox<::text_layout::FormatRange> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_takeAt_as_ptr(self as *mut ::list::ListTextLayoutFormatRange, i)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange QList<QTextLayout::FormatRange>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::cpp_utils::CppBox<::text_layout::FormatRange> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_takeFirst_as_ptr(self as *mut ::list::ListTextLayoutFormatRange)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }

  /// C++ method: <span style='color: green;'>```QTextLayout::FormatRange QList<QTextLayout::FormatRange>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::cpp_utils::CppBox<::text_layout::FormatRange> {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_takeLast_as_ptr(self as *mut ::list::ListTextLayoutFormatRange)
      };
    unsafe { ::cpp_utils::CppBox::new(ffi_result) }
  }
}

impl Drop for ::list::ListTextLayoutFormatRange {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QTextLayout::FormatRange>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_destructor(self as *mut ::list::ListTextLayoutFormatRange) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QTextOption::Tab>```</span>
#[repr(C)]
pub struct ListTextOptionTab([u8; ::type_sizes::QT_GUI_LIST_LIST_TEXT_OPTION_TAB]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListTextOptionTab {
  unsafe fn new_uninitialized() -> ListTextOptionTab {
    ListTextOptionTab(::std::mem::uninitialized())
  }
}

impl ListTextOptionTab {
  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListTextOptionTab) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::append(const QList<QTextOption::Tab>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::text_option::Tab) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::append(const QTextOption::Tab& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTextOptionTabAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTextOption::Tab& QList<QTextOption::Tab>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_option::Tab {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_at(self as *const ::list::ListTextOptionTab, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextOption::Tab& QList<QTextOption::Tab>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::text_option::Tab {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_back_const(self as *const ::list::ListTextOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextOption::Tab& QList<QTextOption::Tab>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_option::Tab {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_back(self as *mut ::list::ListTextOptionTab) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_clear(self as *mut ::list::ListTextOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```const QTextOption::Tab& QList<QTextOption::Tab>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::text_option::Tab {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_constFirst(self as *const ::list::ListTextOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextOption::Tab& QList<QTextOption::Tab>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::text_option::Tab {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_constLast(self as *const ::list::ListTextOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextOption::Tab>::contains(const QTextOption::Tab& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::text_option::Tab) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_contains(self as *const ::list::ListTextOptionTab,
                                                     t as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextOption::Tab>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::text_option::Tab) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextOption::Tab>::count(const QTextOption::Tab& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTextOptionTabCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QList<QTextOption::Tab>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_empty(self as *const ::list::ListTextOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextOption::Tab>::endsWith(const QTextOption::Tab& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::text_option::Tab) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_endsWith(self as *const ::list::ListTextOptionTab,
                                                     t as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextOption::Tab& QList<QTextOption::Tab>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::text_option::Tab {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_first_const(self as *const ::list::ListTextOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextOption::Tab& QList<QTextOption::Tab>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_option::Tab {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_first(self as *mut ::list::ListTextOptionTab) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTextOption::Tab& QList<QTextOption::Tab>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::text_option::Tab {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_front_const(self as *const ::list::ListTextOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextOption::Tab& QList<QTextOption::Tab>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_option::Tab {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_front(self as *mut ::list::ListTextOptionTab) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::text_option::Tab) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextOption::Tab>::indexOf(const QTextOption::Tab& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::text_option::Tab, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextOption::Tab>::indexOf(const QTextOption::Tab& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTextOptionTabIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::insert(int i, const QTextOption::Tab& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::text_option::Tab) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_insert(self as *mut ::list::ListTextOptionTab,
                                                   i,
                                                   t as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextOption::Tab>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_isEmpty(self as *const ::list::ListTextOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```const QTextOption::Tab& QList<QTextOption::Tab>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::text_option::Tab {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_last_const(self as *const ::list::ListTextOptionTab) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::text_option::Tab) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextOption::Tab>::lastIndexOf(const QTextOption::Tab& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::text_option::Tab, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QTextOption::Tab>::lastIndexOf(const QTextOption::Tab& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTextOptionTabLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextOption::Tab& QList<QTextOption::Tab>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::text_option::Tab {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_last(self as *mut ::list::ListTextOptionTab) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextOption::Tab>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_length(self as *const ::list::ListTextOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListTextOptionTab```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab> QList<QTextOption::Tab>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListTextOptionTab```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab> QList<QTextOption::Tab>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListTextOptionTab
    where Args: overloading::ListTextOptionTabMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_move(self as *mut ::list::ListTextOptionTab, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListTextOptionTab```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTextOption::Tab>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListTextOptionTab) -> ::list::ListTextOptionTab```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTextOption::Tab>::QList(const QList<QTextOption::Tab>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListTextOptionTab
    where Args: overloading::ListTextOptionTabNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab> QList<QTextOption::Tab>::operator+(const QList<QTextOption::Tab>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListTextOptionTab) -> ::list::ListTextOptionTab {
    {
      let mut object: ::list::ListTextOptionTab =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_operator_add_to_output(self as *const ::list::ListTextOptionTab,
                                                                     l as *const ::list::ListTextOptionTab,
                                                                     &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListTextOptionTab) -> &'l0 mut ::list::ListTextOptionTab```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>& QList<QTextOption::Tab>::operator+=(const QList<QTextOption::Tab>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::text_option::Tab) -> &'l0 mut ::list::ListTextOptionTab```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>& QList<QTextOption::Tab>::operator+=(const QTextOption::Tab& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTextOptionTab
    where Args: overloading::ListTextOptionTabOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>& QList<QTextOption::Tab>::operator=(const QList<QTextOption::Tab>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListTextOptionTab) -> &'l0 mut ::list::ListTextOptionTab {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_operator_assign(self as *mut ::list::ListTextOptionTab,
                                                            l as *const ::list::ListTextOptionTab)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextOption::Tab>::operator==(const QList<QTextOption::Tab>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListTextOptionTab) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_operator_eq(self as *const ::list::ListTextOptionTab,
                                                        l as *const ::list::ListTextOptionTab)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTextOption::Tab& QList<QTextOption::Tab>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::text_option::Tab {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_operator_index_const(self as *const ::list::ListTextOptionTab, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTextOption::Tab& QList<QTextOption::Tab>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::text_option::Tab {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_operator_index(self as *mut ::list::ListTextOptionTab, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextOption::Tab>::operator!=(const QList<QTextOption::Tab>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListTextOptionTab) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_operator_neq(self as *const ::list::ListTextOptionTab,
                                                         l as *const ::list::ListTextOptionTab)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListTextOptionTab) -> &'l0 mut ::list::ListTextOptionTab```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>& QList<QTextOption::Tab>::operator<<(const QList<QTextOption::Tab>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::text_option::Tab) -> &'l0 mut ::list::ListTextOptionTab```<br>
  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>& QList<QTextOption::Tab>::operator<<(const QTextOption::Tab& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTextOptionTab
    where Args: overloading::ListTextOptionTabOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_pop_back(self as *mut ::list::ListTextOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_pop_front(self as *mut ::list::ListTextOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::prepend(const QTextOption::Tab& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::text_option::Tab) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_prepend(self as *mut ::list::ListTextOptionTab,
                                                    t as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::push_back(const QTextOption::Tab& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::text_option::Tab) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_push_back(self as *mut ::list::ListTextOptionTab,
                                                      t as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::push_front(const QTextOption::Tab& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::text_option::Tab) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_push_front(self as *mut ::list::ListTextOptionTab,
                                                       t as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextOption::Tab>::removeAll(const QTextOption::Tab& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::text_option::Tab) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_removeAll(self as *mut ::list::ListTextOptionTab,
                                                      t as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_removeAt(self as *mut ::list::ListTextOptionTab, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_removeFirst(self as *mut ::list::ListTextOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_removeLast(self as *mut ::list::ListTextOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextOption::Tab>::removeOne(const QTextOption::Tab& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::text_option::Tab) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_removeOne(self as *mut ::list::ListTextOptionTab,
                                                      t as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::replace(int i, const QTextOption::Tab& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::text_option::Tab) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_replace(self as *mut ::list::ListTextOptionTab,
                                                    i,
                                                    t as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_reserve(self as *mut ::list::ListTextOptionTab, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTextOption::Tab>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_size(self as *const ::list::ListTextOptionTab) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTextOption::Tab>::startsWith(const QTextOption::Tab& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::text_option::Tab) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QTextOption_Tab_startsWith(self as *const ::list::ListTextOptionTab,
                                                       t as *const ::text_option::Tab)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListTextOptionTab) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::swap(QList<QTextOption::Tab>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTextOption::Tab>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTextOptionTabSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTextOption::Tab QList<QTextOption::Tab>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::text_option::Tab {
    {
      let mut object: ::text_option::Tab =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_takeAt_to_output(self as *mut ::list::ListTextOptionTab, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextOption::Tab QList<QTextOption::Tab>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::text_option::Tab {
    {
      let mut object: ::text_option::Tab =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_takeFirst_to_output(self as *mut ::list::ListTextOptionTab, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTextOption::Tab QList<QTextOption::Tab>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::text_option::Tab {
    {
      let mut object: ::text_option::Tab =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_takeLast_to_output(self as *mut ::list::ListTextOptionTab, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTextOption::Tab>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::text_option::Tab```<br>
  /// C++ method: <span style='color: green;'>```QTextOption::Tab QList<QTextOption::Tab>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::text_option::Tab)) -> ::text_option::Tab```<br>
  /// C++ method: <span style='color: green;'>```QTextOption::Tab QList<QTextOption::Tab>::value(int i, const QTextOption::Tab& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::text_option::Tab
    where Args: overloading::ListTextOptionTabValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::list::ListTextOptionTab {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QTextOption::Tab>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_destructor(self as *mut ::list::ListTextOptionTab) }
  }
}

/// C++ type: <span style='color: green;'>```QList<const QTouchDevice*>```</span>
#[repr(C)]
pub struct ListTouchDevicePtr([u8; ::type_sizes::QT_GUI_LIST_LIST_TOUCH_DEVICE_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListTouchDevicePtr {
  unsafe fn new_uninitialized() -> ListTouchDevicePtr {
    ListTouchDevicePtr(::std::mem::uninitialized())
  }
}

impl ListTouchDevicePtr {
  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::append(const QList<const QTouchDevice*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListTouchDevicePtr) {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_append_QList_const_QTouchDevice_ptr(self as *mut ::list::ListTouchDevicePtr, t as *const ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::append(const QTouchDevice* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*const ::touch_device::TouchDevice) {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_append_QTouchDevice(self as *mut ::list::ListTouchDevicePtr,
                                                                     t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* const & QList<const QTouchDevice*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_at(self as *const ::list::ListTouchDevicePtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* const & QList<const QTouchDevice*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_back_const(self as *const ::list::ListTouchDevicePtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice*& QList<const QTouchDevice*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_back(self as *mut ::list::ListTouchDevicePtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_clear(self as *mut ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* const & QList<const QTouchDevice*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_constFirst(self as *const ::list::ListTouchDevicePtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* const & QList<const QTouchDevice*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_constLast(self as *const ::list::ListTouchDevicePtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<const QTouchDevice*>::contains(const QTouchDevice* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*const ::touch_device::TouchDevice) -> bool {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_contains(self as *const ::list::ListTouchDevicePtr,
                                                          t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```int QList<const QTouchDevice*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_count_no_args(self as *const ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<const QTouchDevice*>::count(const QTouchDevice* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*const ::touch_device::TouchDevice) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_count_t(self as *const ::list::ListTouchDevicePtr,
                                                         t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```bool QList<const QTouchDevice*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_empty(self as *const ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<const QTouchDevice*>::endsWith(const QTouchDevice* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*const ::touch_device::TouchDevice) -> bool {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_endsWith(self as *const ::list::ListTouchDevicePtr,
                                                          t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* const & QList<const QTouchDevice*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_first_const(self as *const ::list::ListTouchDevicePtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice*& QList<const QTouchDevice*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_first(self as *mut ::list::ListTouchDevicePtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* const & QList<const QTouchDevice*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_front_const(self as *const ::list::ListTouchDevicePtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice*& QList<const QTouchDevice*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_front(self as *mut ::list::ListTouchDevicePtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*const ::touch_device::TouchDevice) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<const QTouchDevice*>::indexOf(const QTouchDevice* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*const ::touch_device::TouchDevice, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<const QTouchDevice*>::indexOf(const QTouchDevice* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTouchDevicePtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::insert(int i, const QTouchDevice* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*const ::touch_device::TouchDevice) {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_insert(self as *mut ::list::ListTouchDevicePtr,
                                                        i,
                                                        t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```bool QList<const QTouchDevice*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_isEmpty(self as *const ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* const & QList<const QTouchDevice*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_last_const(self as *const ::list::ListTouchDevicePtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*const ::touch_device::TouchDevice) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<const QTouchDevice*>::lastIndexOf(const QTouchDevice* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*const ::touch_device::TouchDevice, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<const QTouchDevice*>::lastIndexOf(const QTouchDevice* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListTouchDevicePtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTouchDevice*& QList<const QTouchDevice*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_last(self as *mut ::list::ListTouchDevicePtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<const QTouchDevice*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_length(self as *const ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListTouchDevicePtr```<br>
  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*> QList<const QTouchDevice*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListTouchDevicePtr```<br>
  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*> QList<const QTouchDevice*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListTouchDevicePtr
    where Args: overloading::ListTouchDevicePtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_move(self as *mut ::list::ListTouchDevicePtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListTouchDevicePtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<const QTouchDevice*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListTouchDevicePtr) -> ::list::ListTouchDevicePtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<const QTouchDevice*>::QList(const QList<const QTouchDevice*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListTouchDevicePtr
    where Args: overloading::ListTouchDevicePtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*> QList<const QTouchDevice*>::operator+(const QList<const QTouchDevice*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListTouchDevicePtr) -> ::list::ListTouchDevicePtr {
    {
      let mut object: ::list::ListTouchDevicePtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_operator_add_to_output(self as *const ::list::ListTouchDevicePtr, l as *const ::list::ListTouchDevicePtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*>& QList<const QTouchDevice*>::operator+=(const QList<const QTouchDevice*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::list::ListTouchDevicePtr)
                                 -> &'l0 mut ::list::ListTouchDevicePtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_operator_add_assign_l(self as *mut ::list::ListTouchDevicePtr,
                                                                           l as *const ::list::ListTouchDevicePtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*>& QList<const QTouchDevice*>::operator+=(const QTouchDevice* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *const ::touch_device::TouchDevice)
                                               -> &'l0 mut ::list::ListTouchDevicePtr {
    let ffi_result = ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_operator_add_assign_t(self as *mut ::list::ListTouchDevicePtr, t as *const *const ::touch_device::TouchDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*>& QList<const QTouchDevice*>::operator=(const QList<const QTouchDevice*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListTouchDevicePtr) -> &'l0 mut ::list::ListTouchDevicePtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_operator_assign(self as *mut ::list::ListTouchDevicePtr,
                                                                     l as *const ::list::ListTouchDevicePtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<const QTouchDevice*>::operator==(const QList<const QTouchDevice*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListTouchDevicePtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_operator_eq(self as *const ::list::ListTouchDevicePtr,
                                                               l as *const ::list::ListTouchDevicePtr)
    }
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* const & QList<const QTouchDevice*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_operator_index_const(self as *const ::list::ListTouchDevicePtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice*& QList<const QTouchDevice*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *const ::touch_device::TouchDevice {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_operator_index(self as *mut ::list::ListTouchDevicePtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<const QTouchDevice*>::operator!=(const QList<const QTouchDevice*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListTouchDevicePtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_operator_neq(self as *const ::list::ListTouchDevicePtr,
                                                                l as *const ::list::ListTouchDevicePtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*>& QList<const QTouchDevice*>::operator<<(const QList<const QTouchDevice*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListTouchDevicePtr) -> &'l0 mut ::list::ListTouchDevicePtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_operator_shl_l(self as *mut ::list::ListTouchDevicePtr,
                                                                    l as *const ::list::ListTouchDevicePtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*>& QList<const QTouchDevice*>::operator<<(const QTouchDevice* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *const ::touch_device::TouchDevice)
                                        -> &'l0 mut ::list::ListTouchDevicePtr {
    let ffi_result =
      ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_operator_shl_t(self as *mut ::list::ListTouchDevicePtr,
                                                                  t as *const *const ::touch_device::TouchDevice);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_pop_back(self as *mut ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_pop_front(self as *mut ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::prepend(const QTouchDevice* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*const ::touch_device::TouchDevice) {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_prepend(self as *mut ::list::ListTouchDevicePtr,
                                                         t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::push_back(const QTouchDevice* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*const ::touch_device::TouchDevice) {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_push_back(self as *mut ::list::ListTouchDevicePtr,
                                                           t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::push_front(const QTouchDevice* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*const ::touch_device::TouchDevice) {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_push_front(self as *mut ::list::ListTouchDevicePtr,
                                                            t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```int QList<const QTouchDevice*>::removeAll(const QTouchDevice* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*const ::touch_device::TouchDevice) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_removeAll(self as *mut ::list::ListTouchDevicePtr,
                                                           t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_removeAt(self as *mut ::list::ListTouchDevicePtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_removeFirst(self as *mut ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_removeLast(self as *mut ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<const QTouchDevice*>::removeOne(const QTouchDevice* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*const ::touch_device::TouchDevice) -> bool {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_removeOne(self as *mut ::list::ListTouchDevicePtr,
                                                           t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::replace(int i, const QTouchDevice* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*const ::touch_device::TouchDevice) {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_replace(self as *mut ::list::ListTouchDevicePtr,
                                                         i,
                                                         t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_reserve(self as *mut ::list::ListTouchDevicePtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<const QTouchDevice*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_size(self as *const ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<const QTouchDevice*>::startsWith(const QTouchDevice* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*const ::touch_device::TouchDevice) -> bool {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_startsWith(self as *const ::list::ListTouchDevicePtr,
                                                            t as *const *const ::touch_device::TouchDevice)
  }

  /// C++ method: <span style='color: green;'>```QList<const QTouchDevice*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListTouchDevicePtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::swap(QList<const QTouchDevice*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<const QTouchDevice*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTouchDevicePtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTouchDevice* QList<const QTouchDevice*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *const ::touch_device::TouchDevice {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_takeAt(self as *mut ::list::ListTouchDevicePtr, i) }
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* QList<const QTouchDevice*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *const ::touch_device::TouchDevice {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_takeFirst(self as *mut ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* QList<const QTouchDevice*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *const ::touch_device::TouchDevice {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_takeLast(self as *mut ::list::ListTouchDevicePtr) }
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* QList<const QTouchDevice*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *const ::touch_device::TouchDevice {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_value_i(self as *const ::list::ListTouchDevicePtr, i) }
  }

  /// C++ method: <span style='color: green;'>```const QTouchDevice* QList<const QTouchDevice*>::value(int i, const QTouchDevice* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*const ::touch_device::TouchDevice)
                             -> *const ::touch_device::TouchDevice {
    ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_value_i_defaultValue(self as *const ::list::ListTouchDevicePtr, i, default_value as *const *const ::touch_device::TouchDevice)
  }
}

impl Drop for ::list::ListTouchDevicePtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<const QTouchDevice*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_destructor(self as *mut ::list::ListTouchDevicePtr) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>```</span>
#[repr(C)]
pub struct ListTouchEventTouchPoint([u8; ::type_sizes::QT_GUI_LIST_LIST_TOUCH_EVENT_TOUCH_POINT]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListTouchEventTouchPoint {
  unsafe fn new_uninitialized() -> ListTouchEventTouchPoint {
    ListTouchEventTouchPoint(::std::mem::uninitialized())
  }
}

impl ListTouchEventTouchPoint {
  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::list::ListTouchEventTouchPoint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::append(const QList<QTouchEvent::TouchPoint>& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::touch_event::TouchPoint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::append(const QTouchEvent::TouchPoint& t)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTouchEventTouchPointAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::touch_event::TouchPoint {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_at(self as *const ::list::ListTouchEventTouchPoint, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::touch_event::TouchPoint {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_back_const(self as *const ::list::ListTouchEventTouchPoint)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::touch_event::TouchPoint {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_back(self as *mut ::list::ListTouchEventTouchPoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_clear(self as *mut ::list::ListTouchEventTouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```const QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::touch_event::TouchPoint {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_constFirst(self as *const ::list::ListTouchEventTouchPoint)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::touch_event::TouchPoint {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_constLast(self as *const ::list::ListTouchEventTouchPoint)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTouchEvent::TouchPoint>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_count(self as *const ::list::ListTouchEventTouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTouchEvent::TouchPoint>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_empty(self as *const ::list::ListTouchEventTouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```const QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::touch_event::TouchPoint {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_first_const(self as *const ::list::ListTouchEventTouchPoint)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::touch_event::TouchPoint {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_first(self as *mut ::list::ListTouchEventTouchPoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::touch_event::TouchPoint {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_front_const(self as *const ::list::ListTouchEventTouchPoint)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::touch_event::TouchPoint {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_front(self as *mut ::list::ListTouchEventTouchPoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::insert(int i, const QTouchEvent::TouchPoint& t)```</span>
  ///
  ///
  pub fn insert(&mut self, i: ::libc::c_int, t: &::touch_event::TouchPoint) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_insert(self as *mut ::list::ListTouchEventTouchPoint,
                                                          i,
                                                          t as *const ::touch_event::TouchPoint)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QTouchEvent::TouchPoint>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_isEmpty(self as *const ::list::ListTouchEventTouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```const QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::touch_event::TouchPoint {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_last_const(self as *const ::list::ListTouchEventTouchPoint)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::touch_event::TouchPoint {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_last(self as *mut ::list::ListTouchEventTouchPoint) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QTouchEvent::TouchPoint>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_length(self as *const ::list::ListTouchEventTouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListTouchEventTouchPoint```<br>
  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint> QList<QTouchEvent::TouchPoint>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListTouchEventTouchPoint```<br>
  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint> QList<QTouchEvent::TouchPoint>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListTouchEventTouchPoint
    where Args: overloading::ListTouchEventTouchPointMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_move(self as *mut ::list::ListTouchEventTouchPoint, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListTouchEventTouchPoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTouchEvent::TouchPoint>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListTouchEventTouchPoint) -> ::list::ListTouchEventTouchPoint```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QTouchEvent::TouchPoint>::QList(const QList<QTouchEvent::TouchPoint>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListTouchEventTouchPoint
    where Args: overloading::ListTouchEventTouchPointNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint> QList<QTouchEvent::TouchPoint>::operator+(const QList<QTouchEvent::TouchPoint>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListTouchEventTouchPoint) -> ::list::ListTouchEventTouchPoint {
    {
      let mut object: ::list::ListTouchEventTouchPoint =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_operator_add_to_output(self as *const ::list::ListTouchEventTouchPoint, l as *const ::list::ListTouchEventTouchPoint, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::list::ListTouchEventTouchPoint) -> &'l0 mut ::list::ListTouchEventTouchPoint```<br>
  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>& QList<QTouchEvent::TouchPoint>::operator+=(const QList<QTouchEvent::TouchPoint>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::touch_event::TouchPoint) -> &'l0 mut ::list::ListTouchEventTouchPoint```<br>
  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>& QList<QTouchEvent::TouchPoint>::operator+=(const QTouchEvent::TouchPoint& t)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTouchEventTouchPoint
    where Args: overloading::ListTouchEventTouchPointOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>& QList<QTouchEvent::TouchPoint>::operator=(const QList<QTouchEvent::TouchPoint>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             l: &'l1 ::list::ListTouchEventTouchPoint)
                             -> &'l0 mut ::list::ListTouchEventTouchPoint {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_operator_assign(self as *mut ::list::ListTouchEventTouchPoint,
                                                                     l as *const ::list::ListTouchEventTouchPoint)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::touch_event::TouchPoint {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_operator_index_const(self as *const ::list::ListTouchEventTouchPoint, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QTouchEvent::TouchPoint& QList<QTouchEvent::TouchPoint>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::touch_event::TouchPoint {
    let ffi_result =
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_operator_index(self as *mut ::list::ListTouchEventTouchPoint, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::list::ListTouchEventTouchPoint) -> &'l0 mut ::list::ListTouchEventTouchPoint```<br>
  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>& QList<QTouchEvent::TouchPoint>::operator<<(const QList<QTouchEvent::TouchPoint>& l)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::touch_event::TouchPoint) -> &'l0 mut ::list::ListTouchEventTouchPoint```<br>
  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>& QList<QTouchEvent::TouchPoint>::operator<<(const QTouchEvent::TouchPoint& t)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::list::ListTouchEventTouchPoint
    where Args: overloading::ListTouchEventTouchPointOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_pop_back(self as *mut ::list::ListTouchEventTouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_pop_front(self as *mut ::list::ListTouchEventTouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::prepend(const QTouchEvent::TouchPoint& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::touch_event::TouchPoint) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_prepend(self as *mut ::list::ListTouchEventTouchPoint,
                                                           t as *const ::touch_event::TouchPoint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::push_back(const QTouchEvent::TouchPoint& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::touch_event::TouchPoint) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_push_back(self as *mut ::list::ListTouchEventTouchPoint,
                                                             t as *const ::touch_event::TouchPoint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::push_front(const QTouchEvent::TouchPoint& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::touch_event::TouchPoint) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_push_front(self as *mut ::list::ListTouchEventTouchPoint,
                                                              t as *const ::touch_event::TouchPoint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_removeAt(self as *mut ::list::ListTouchEventTouchPoint, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_removeFirst(self as *mut ::list::ListTouchEventTouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_removeLast(self as *mut ::list::ListTouchEventTouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::replace(int i, const QTouchEvent::TouchPoint& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::touch_event::TouchPoint) {
    unsafe {
      ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_replace(self as *mut ::list::ListTouchEventTouchPoint,
                                                           i,
                                                           t as *const ::touch_event::TouchPoint)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_reserve(self as *mut ::list::ListTouchEventTouchPoint, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QTouchEvent::TouchPoint>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_size(self as *const ::list::ListTouchEventTouchPoint) }
  }

  /// C++ method: <span style='color: green;'>```QList<QTouchEvent::TouchPoint>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListTouchEventTouchPoint) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::swap(QList<QTouchEvent::TouchPoint>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QTouchEvent::TouchPoint>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListTouchEventTouchPointSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QTouchEvent::TouchPoint QList<QTouchEvent::TouchPoint>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::touch_event::TouchPoint {
    {
      let mut object: ::touch_event::TouchPoint =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_takeAt_to_output(self as *mut ::list::ListTouchEventTouchPoint,
                                                                      i,
                                                                      &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTouchEvent::TouchPoint QList<QTouchEvent::TouchPoint>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::touch_event::TouchPoint {
    {
      let mut object: ::touch_event::TouchPoint =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_takeFirst_to_output(self as *mut ::list::ListTouchEventTouchPoint, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QTouchEvent::TouchPoint QList<QTouchEvent::TouchPoint>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::touch_event::TouchPoint {
    {
      let mut object: ::touch_event::TouchPoint =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_takeLast_to_output(self as *mut ::list::ListTouchEventTouchPoint, &mut object);
      }
      object
    }
  }
}

impl Drop for ::list::ListTouchEventTouchPoint {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QTouchEvent::TouchPoint>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_destructor(self as *mut ::list::ListTouchEventTouchPoint) }
  }
}

/// C++ type: <span style='color: green;'>```QList<QWindow*>```</span>
#[repr(C)]
pub struct ListWindowMutPtr([u8; ::type_sizes::QT_GUI_LIST_LIST_WINDOW_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for ListWindowMutPtr {
  unsafe fn new_uninitialized() -> ListWindowMutPtr {
    ListWindowMutPtr(::std::mem::uninitialized())
  }
}

impl ListWindowMutPtr {
  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::append(const QList<QWindow*>& t)```</span>
  ///
  ///
  pub fn append(&mut self, t: &::list::ListWindowMutPtr) {
    unsafe {
      ::ffi::qt_gui_c_QList_QWindow_ptr_append_QList_QWindow_ptr(self as *mut ::list::ListWindowMutPtr,
                                                                 t as *const ::list::ListWindowMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::append(QWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::window::Window) {
    ::ffi::qt_gui_c_QList_QWindow_ptr_append_QWindow(self as *mut ::list::ListWindowMutPtr,
                                                     t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```QWindow* const & QList<QWindow*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_at(self as *const ::list::ListWindowMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWindow* const & QList<QWindow*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_back_const(self as *const ::list::ListWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWindow*& QList<QWindow*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_back(self as *mut ::list::ListWindowMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_clear(self as *mut ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QWindow* const & QList<QWindow*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_constFirst(self as *const ::list::ListWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWindow* const & QList<QWindow*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_constLast(self as *const ::list::ListWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWindow*>::contains(QWindow* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::window::Window) -> bool {
    ::ffi::qt_gui_c_QList_QWindow_ptr_contains(self as *const ::list::ListWindowMutPtr,
                                               t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```int QList<QWindow*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_count_no_args(self as *const ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QWindow*>::count(QWindow* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::window::Window) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QWindow_ptr_count_t(self as *const ::list::ListWindowMutPtr,
                                              t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWindow*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_empty(self as *const ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWindow*>::endsWith(QWindow* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::window::Window) -> bool {
    ::ffi::qt_gui_c_QList_QWindow_ptr_endsWith(self as *const ::list::ListWindowMutPtr,
                                               t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```QWindow* const & QList<QWindow*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_first_const(self as *const ::list::ListWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWindow*& QList<QWindow*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_first(self as *mut ::list::ListWindowMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWindow* const & QList<QWindow*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_front_const(self as *const ::list::ListWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWindow*& QList<QWindow*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_front(self as *mut ::list::ListWindowMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWindow*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::window::Window) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWindow*>::indexOf(QWindow* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::window::Window, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWindow*>::indexOf(QWindow* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListWindowMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::insert(int i, QWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn insert(&mut self, i: ::libc::c_int, t: &*mut ::window::Window) {
    ::ffi::qt_gui_c_QList_QWindow_ptr_insert(self as *mut ::list::ListWindowMutPtr,
                                             i,
                                             t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWindow*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_isEmpty(self as *const ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QWindow* const & QList<QWindow*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_last_const(self as *const ::list::ListWindowMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWindow*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::window::Window) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWindow*>::lastIndexOf(QWindow* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::window::Window, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QList<QWindow*>::lastIndexOf(QWindow* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::ListWindowMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWindow*& QList<QWindow*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::window::Window {
    let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_last(self as *mut ::list::ListWindowMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QList<QWindow*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_length(self as *const ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QList<QWindow*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::list::ListWindowMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QWindow*> QList<QWindow*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::list::ListWindowMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QList<QWindow*> QList<QWindow*>::mid(int pos, int length = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::list::ListWindowMutPtr
    where Args: overloading::ListWindowMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_move(self as *mut ::list::ListWindowMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QList<QWindow*>::QList```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::list::ListWindowMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QWindow*>::QList()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::list::ListWindowMutPtr) -> ::list::ListWindowMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QList<QWindow*>::QList(const QList<QWindow*>& l)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::list::ListWindowMutPtr
    where Args: overloading::ListWindowMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QList<QWindow*> QList<QWindow*>::operator+(const QList<QWindow*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::list::ListWindowMutPtr) -> ::list::ListWindowMutPtr {
    {
      let mut object: ::list::ListWindowMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_gui_c_QList_QWindow_ptr_operator_add_to_output(self as *const ::list::ListWindowMutPtr,
                                                                 l as *const ::list::ListWindowMutPtr,
                                                                 &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QWindow*>& QList<QWindow*>::operator+=(const QList<QWindow*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListWindowMutPtr) -> &'l0 mut ::list::ListWindowMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QWindow_ptr_operator_add_assign_l(self as *mut ::list::ListWindowMutPtr,
                                                              l as *const ::list::ListWindowMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWindow*>& QList<QWindow*>::operator+=(QWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::window::Window)
                                               -> &'l0 mut ::list::ListWindowMutPtr {
    let ffi_result = ::ffi::qt_gui_c_QList_QWindow_ptr_operator_add_assign_t(self as *mut ::list::ListWindowMutPtr,
                                                                             t as *const *mut ::window::Window);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWindow*>& QList<QWindow*>::operator=(const QList<QWindow*>& l)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListWindowMutPtr) -> &'l0 mut ::list::ListWindowMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QWindow_ptr_operator_assign(self as *mut ::list::ListWindowMutPtr,
                                                        l as *const ::list::ListWindowMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWindow*>::operator==(const QList<QWindow*>& l) const```</span>
  ///
  ///
  pub fn op_eq(&self, l: &::list::ListWindowMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QWindow_ptr_operator_eq(self as *const ::list::ListWindowMutPtr,
                                                    l as *const ::list::ListWindowMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QWindow* const & QList<QWindow*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::window::Window {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_operator_index_const(self as *const ::list::ListWindowMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QWindow*& QList<QWindow*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::window::Window {
    let ffi_result =
      unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_operator_index(self as *mut ::list::ListWindowMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWindow*>::operator!=(const QList<QWindow*>& l) const```</span>
  ///
  ///
  pub fn op_neq(&self, l: &::list::ListWindowMutPtr) -> bool {
    unsafe {
      ::ffi::qt_gui_c_QList_QWindow_ptr_operator_neq(self as *const ::list::ListWindowMutPtr,
                                                     l as *const ::list::ListWindowMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QList<QWindow*>& QList<QWindow*>::operator<<(const QList<QWindow*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::list::ListWindowMutPtr) -> &'l0 mut ::list::ListWindowMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_gui_c_QList_QWindow_ptr_operator_shl_l(self as *mut ::list::ListWindowMutPtr,
                                                       l as *const ::list::ListWindowMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QList<QWindow*>& QList<QWindow*>::operator<<(QWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::window::Window)
                                        -> &'l0 mut ::list::ListWindowMutPtr {
    let ffi_result = ::ffi::qt_gui_c_QList_QWindow_ptr_operator_shl_t(self as *mut ::list::ListWindowMutPtr,
                                                                      t as *const *mut ::window::Window);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_pop_back(self as *mut ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_pop_front(self as *mut ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::prepend(QWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::window::Window) {
    ::ffi::qt_gui_c_QList_QWindow_ptr_prepend(self as *mut ::list::ListWindowMutPtr,
                                              t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::push_back(QWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::window::Window) {
    ::ffi::qt_gui_c_QList_QWindow_ptr_push_back(self as *mut ::list::ListWindowMutPtr,
                                                t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::push_front(QWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::window::Window) {
    ::ffi::qt_gui_c_QList_QWindow_ptr_push_front(self as *mut ::list::ListWindowMutPtr,
                                                 t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```int QList<QWindow*>::removeAll(QWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::window::Window) -> ::libc::c_int {
    ::ffi::qt_gui_c_QList_QWindow_ptr_removeAll(self as *mut ::list::ListWindowMutPtr,
                                                t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_removeAt(self as *mut ::list::ListWindowMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_removeFirst(self as *mut ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_removeLast(self as *mut ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWindow*>::removeOne(QWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::window::Window) -> bool {
    ::ffi::qt_gui_c_QList_QWindow_ptr_removeOne(self as *mut ::list::ListWindowMutPtr,
                                                t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::replace(int i, QWindow* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::window::Window) {
    ::ffi::qt_gui_c_QList_QWindow_ptr_replace(self as *mut ::list::ListWindowMutPtr,
                                              i,
                                              t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_reserve(self as *mut ::list::ListWindowMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QList<QWindow*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_size(self as *const ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QList<QWindow*>::startsWith(QWindow* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::window::Window) -> bool {
    ::ffi::qt_gui_c_QList_QWindow_ptr_startsWith(self as *const ::list::ListWindowMutPtr,
                                                 t as *const *mut ::window::Window)
  }

  /// C++ method: <span style='color: green;'>```QList<QWindow*>::swap```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn swap(&mut self, &mut ::list::ListWindowMutPtr) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::swap(QList<QWindow*>& other)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn swap(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QList<QWindow*>::swap(int i, int j)```</span>
  ///
  ///
  pub fn swap<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::ListWindowMutPtrSwapArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QWindow* QList<QWindow*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_takeAt(self as *mut ::list::ListWindowMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QWindow* QList<QWindow*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_takeFirst(self as *mut ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QWindow* QList<QWindow*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_takeLast(self as *mut ::list::ListWindowMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QWindow* QList<QWindow*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::window::Window {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_value_i(self as *const ::list::ListWindowMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```QWindow* QList<QWindow*>::value(int i, QWindow* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self, i: ::libc::c_int, default_value: &*mut ::window::Window) -> *mut ::window::Window {
    ::ffi::qt_gui_c_QList_QWindow_ptr_value_i_defaultValue(self as *const ::list::ListWindowMutPtr,
                                                           i,
                                                           default_value as *const *mut ::window::Window)
  }
}

impl Drop for ::list::ListWindowMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QList<QWindow*>::~QList()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_destructor(self as *mut ::list::ListWindowMutPtr) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [ListAccessibleInterfaceMutPtr::index_of](../struct.ListAccessibleInterfaceMutPtr.html#method.index_of) method.
  pub trait ListAccessibleInterfaceMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListAccessibleInterfaceMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListAccessibleInterfaceMutPtrIndexOfArgs<'largs> for &'largs *mut ::accessible_interface::AccessibleInterface {

  unsafe fn exec(self, original_self: &'largs ::list::ListAccessibleInterfaceMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_indexOf_t(original_self as *const ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }
}
  impl<'largs> ListAccessibleInterfaceMutPtrIndexOfArgs<'largs> for (&'largs *mut ::accessible_interface::AccessibleInterface,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::list::ListAccessibleInterfaceMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_indexOf_t_from(original_self as *const ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface, from)
  }
}
  /// This trait represents a set of arguments accepted by [ListAccessibleInterfaceMutPtr::last_index_of](../struct.ListAccessibleInterfaceMutPtr.html#method.last_index_of) method.
  pub trait ListAccessibleInterfaceMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListAccessibleInterfaceMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListAccessibleInterfaceMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::accessible_interface::AccessibleInterface {

  unsafe fn exec(self, original_self: &'largs ::list::ListAccessibleInterfaceMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_lastIndexOf_t(original_self as *const ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface)
  }
}
  impl<'largs> ListAccessibleInterfaceMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::accessible_interface::AccessibleInterface,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::list::ListAccessibleInterfaceMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_lastIndexOf_t_from(original_self as *const ::list::ListAccessibleInterfaceMutPtr, t as *const *mut ::accessible_interface::AccessibleInterface, from)
  }
}
  /// This trait represents a set of arguments accepted by [ListAccessibleInterfaceMutPtr::mid](../struct.ListAccessibleInterfaceMutPtr.html#method.mid) method.
  pub trait ListAccessibleInterfaceMutPtrMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::list::ListAccessibleInterfaceMutPtr)
            -> ::list::ListAccessibleInterfaceMutPtr;
  }
  impl<'largs> ListAccessibleInterfaceMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::list::ListAccessibleInterfaceMutPtr)
            -> ::list::ListAccessibleInterfaceMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListAccessibleInterfaceMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_mid_to_output_pos(original_self as *const ::list::ListAccessibleInterfaceMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListAccessibleInterfaceMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::list::ListAccessibleInterfaceMutPtr)
            -> ::list::ListAccessibleInterfaceMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListAccessibleInterfaceMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_mid_to_output_pos_length(original_self as *const ::list::ListAccessibleInterfaceMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAccessibleInterfaceMutPtr::new](../struct.ListAccessibleInterfaceMutPtr.html#method.new) method.
  pub trait ListAccessibleInterfaceMutPtrNewArgs {
    fn exec(self) -> ::list::ListAccessibleInterfaceMutPtr;
  }
  impl<'a> ListAccessibleInterfaceMutPtrNewArgs for &'a ::list::ListAccessibleInterfaceMutPtr {
    fn exec(self) -> ::list::ListAccessibleInterfaceMutPtr {
      let l = self;
      {
        let mut object: ::list::ListAccessibleInterfaceMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_constructor_l(l as *const ::list::ListAccessibleInterfaceMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListAccessibleInterfaceMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListAccessibleInterfaceMutPtr {

      {
        let mut object: ::list::ListAccessibleInterfaceMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListAccessibleInterfaceMutPtr::swap](../struct.ListAccessibleInterfaceMutPtr.html#method.swap) method.
  pub trait ListAccessibleInterfaceMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListAccessibleInterfaceMutPtr) -> ();
  }
  impl<'largs> ListAccessibleInterfaceMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListAccessibleInterfaceMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_swap_i_j(original_self as *mut ::list::ListAccessibleInterfaceMutPtr, i, j) }
    }
  }
  impl<'largs> ListAccessibleInterfaceMutPtrSwapArgs<'largs> for &'largs mut ::list::ListAccessibleInterfaceMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListAccessibleInterfaceMutPtr) -> () {
      let other = self;
      unsafe { ::ffi::qt_gui_c_QList_QAccessibleInterface_ptr_swap_other(original_self as *mut ::list::ListAccessibleInterfaceMutPtr, other as *mut ::list::ListAccessibleInterfaceMutPtr) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCDouble::append](../struct.ListCDouble.html#method.append) method.
  pub trait ListCDoubleAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> ();
  }
  impl<'largs> ListCDoubleAppendArgs<'largs> for &'largs ::list::ListCDouble {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_double_append_QList_double(original_self as *mut ::list::ListCDouble,
                                                         t as *const ::list::ListCDouble)
      }
    }
  }
  impl<'largs> ListCDoubleAppendArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_double_append_double(original_self as *mut ::list::ListCDouble,
                                                   t as *const ::libc::c_double)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCDouble::count](../struct.ListCDouble.html#method.count) method.
  pub trait ListCDoubleCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_int;
  }
  impl<'largs> ListCDoubleCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QList_double_count_no_args(original_self as *const ::list::ListCDouble) }
    }
  }
  impl<'largs> ListCDoubleCountArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_double_count_t(original_self as *const ::list::ListCDouble,
                                             t as *const ::libc::c_double)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCDouble::index_of](../struct.ListCDouble.html#method.index_of) method.
  pub trait ListCDoubleIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_int;
  }
  impl<'largs> ListCDoubleIndexOfArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_double_indexOf_t(original_self as *const ::list::ListCDouble,
                                               t as *const ::libc::c_double)
      }
    }
  }
  impl<'largs> ListCDoubleIndexOfArgs<'largs> for (&'largs ::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_double_indexOf_t_from(original_self as *const ::list::ListCDouble,
                                                    t as *const ::libc::c_double,
                                                    from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCDouble::last_index_of](../struct.ListCDouble.html#method.last_index_of) method.
  pub trait ListCDoubleLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_int;
  }
  impl<'largs> ListCDoubleLastIndexOfArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_double_lastIndexOf_t(original_self as *const ::list::ListCDouble,
                                                   t as *const ::libc::c_double)
      }
    }
  }
  impl<'largs> ListCDoubleLastIndexOfArgs<'largs> for (&'largs ::libc::c_double, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_double_lastIndexOf_t_from(original_self as *const ::list::ListCDouble,
                                                        t as *const ::libc::c_double,
                                                        from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCDouble::mid](../struct.ListCDouble.html#method.mid) method.
  pub trait ListCDoubleMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::list::ListCDouble;
  }
  impl<'largs> ListCDoubleMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::list::ListCDouble {
      let pos = self;
      {
        let mut object: ::list::ListCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_double_mid_to_output_pos(original_self as *const ::list::ListCDouble,
                                                         pos,
                                                         &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListCDoubleMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::list::ListCDouble {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_double_mid_to_output_pos_length(original_self as *const ::list::ListCDouble,
                                                                pos,
                                                                length,
                                                                &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCDouble::new](../struct.ListCDouble.html#method.new) method.
  pub trait ListCDoubleNewArgs {
    fn exec(self) -> ::list::ListCDouble;
  }
  impl<'a> ListCDoubleNewArgs for &'a ::list::ListCDouble {
    fn exec(self) -> ::list::ListCDouble {
      let l = self;
      {
        let mut object: ::list::ListCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_double_constructor_l(l as *const ::list::ListCDouble, &mut object);
        }
        object
      }
    }
  }
  impl ListCDoubleNewArgs for () {
    fn exec(self) -> ::list::ListCDouble {

      {
        let mut object: ::list::ListCDouble =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_double_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCDouble::op_add_assign](../struct.ListCDouble.html#method.op_add_assign) method.
  pub trait ListCDoubleOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> &'largs mut ::list::ListCDouble;
  }
  impl<'largs> ListCDoubleOpAddAssignArgs<'largs> for &'largs ::list::ListCDouble {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> &'largs mut ::list::ListCDouble {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_double_operator_add_assign_l(original_self as *mut ::list::ListCDouble,
                                                           l as *const ::list::ListCDouble)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListCDoubleOpAddAssignArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> &'largs mut ::list::ListCDouble {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_double_operator_add_assign_t(original_self as *mut ::list::ListCDouble,
                                                           t as *const ::libc::c_double)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListCDouble::op_shl](../struct.ListCDouble.html#method.op_shl) method.
  pub trait ListCDoubleOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> &'largs mut ::list::ListCDouble;
  }
  impl<'largs> ListCDoubleOpShlArgs<'largs> for &'largs ::list::ListCDouble {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> &'largs mut ::list::ListCDouble {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_double_operator_shl_l(original_self as *mut ::list::ListCDouble,
                                                    l as *const ::list::ListCDouble)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListCDoubleOpShlArgs<'largs> for &'largs ::libc::c_double {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> &'largs mut ::list::ListCDouble {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_double_operator_shl_t(original_self as *mut ::list::ListCDouble,
                                                    t as *const ::libc::c_double)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListCDouble::swap](../struct.ListCDouble.html#method.swap) method.
  pub trait ListCDoubleSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> ();
  }
  impl<'largs> ListCDoubleSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_double_swap_i_j(original_self as *mut ::list::ListCDouble, i, j) }
    }
  }
  impl<'largs> ListCDoubleSwapArgs<'largs> for &'largs mut ::list::ListCDouble {
    fn exec(self, original_self: &'largs mut ::list::ListCDouble) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_double_swap_other(original_self as *mut ::list::ListCDouble,
                                                other as *mut ::list::ListCDouble)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListCDouble::value](../struct.ListCDouble.html#method.value) method.
  pub trait ListCDoubleValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_double;
  }
  impl<'largs> ListCDoubleValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_double {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QList_double_value_i(original_self as *const ::list::ListCDouble, i) }
    }
  }
  impl<'largs> ListCDoubleValueArgs<'largs> for (::libc::c_int, &'largs ::libc::c_double) {
    fn exec(self, original_self: &'largs ::list::ListCDouble) -> ::libc::c_double {
      let i = self.0;
      let default_value = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_double_value_i_defaultValue(original_self as *const ::list::ListCDouble,
                                                          i,
                                                          default_value as *const ::libc::c_double)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFontDatabaseWritingSystem::append](../struct.ListFontDatabaseWritingSystem.html#method.append) method.
  pub trait ListFontDatabaseWritingSystemAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListFontDatabaseWritingSystem) -> ();
  }
  impl<'largs> ListFontDatabaseWritingSystemAppendArgs<'largs> for &'largs ::font_database::WritingSystem {
    fn exec(self, original_self: &'largs mut ::list::ListFontDatabaseWritingSystem) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_append_QFontDatabase_WritingSystem(original_self as *mut ::list::ListFontDatabaseWritingSystem, t as *const ::font_database::WritingSystem) }
    }
  }
  impl<'largs> ListFontDatabaseWritingSystemAppendArgs<'largs> for &'largs ::list::ListFontDatabaseWritingSystem {
    fn exec(self, original_self: &'largs mut ::list::ListFontDatabaseWritingSystem) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_append_QList_QFontDatabase_WritingSystem(original_self as *mut ::list::ListFontDatabaseWritingSystem, t as *const ::list::ListFontDatabaseWritingSystem) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFontDatabaseWritingSystem::count](../struct.ListFontDatabaseWritingSystem.html#method.count) method.
  pub trait ListFontDatabaseWritingSystemCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::libc::c_int;
  }
  impl<'largs> ListFontDatabaseWritingSystemCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_count_no_args(original_self as *const ::list::ListFontDatabaseWritingSystem) }
    }
  }
  impl<'largs> ListFontDatabaseWritingSystemCountArgs<'largs> for &'largs ::font_database::WritingSystem {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_count_t(original_self as *const ::list::ListFontDatabaseWritingSystem, t as *const ::font_database::WritingSystem) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFontDatabaseWritingSystem::index_of](../struct.ListFontDatabaseWritingSystem.html#method.index_of) method.
  pub trait ListFontDatabaseWritingSystemIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::libc::c_int;
  }
  impl<'largs> ListFontDatabaseWritingSystemIndexOfArgs<'largs> for &'largs ::font_database::WritingSystem {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_indexOf_t(original_self as *const ::list::ListFontDatabaseWritingSystem, t as *const ::font_database::WritingSystem) }
    }
  }
  impl<'largs> ListFontDatabaseWritingSystemIndexOfArgs<'largs>
    for (&'largs ::font_database::WritingSystem, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_indexOf_t_from(original_self as *const ::list::ListFontDatabaseWritingSystem, t as *const ::font_database::WritingSystem, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFontDatabaseWritingSystem::last_index_of](../struct.ListFontDatabaseWritingSystem.html#method.last_index_of) method.
  pub trait ListFontDatabaseWritingSystemLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::libc::c_int;
  }
  impl<'largs> ListFontDatabaseWritingSystemLastIndexOfArgs<'largs> for &'largs ::font_database::WritingSystem {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::libc::c_int {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_lastIndexOf_t(original_self as *const ::list::ListFontDatabaseWritingSystem, t as *const ::font_database::WritingSystem) }
    }
  }
  impl<'largs> ListFontDatabaseWritingSystemLastIndexOfArgs<'largs>
    for (&'largs ::font_database::WritingSystem, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_lastIndexOf_t_from(original_self as *const ::list::ListFontDatabaseWritingSystem, t as *const ::font_database::WritingSystem, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFontDatabaseWritingSystem::mid](../struct.ListFontDatabaseWritingSystem.html#method.mid) method.
  pub trait ListFontDatabaseWritingSystemMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::list::ListFontDatabaseWritingSystem)
            -> ::list::ListFontDatabaseWritingSystem;
  }
  impl<'largs> ListFontDatabaseWritingSystemMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::list::ListFontDatabaseWritingSystem)
            -> ::list::ListFontDatabaseWritingSystem {
      let pos = self;
      {
        let mut object: ::list::ListFontDatabaseWritingSystem =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_mid_to_output_pos(original_self as *const ::list::ListFontDatabaseWritingSystem, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListFontDatabaseWritingSystemMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::list::ListFontDatabaseWritingSystem)
            -> ::list::ListFontDatabaseWritingSystem {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListFontDatabaseWritingSystem =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_mid_to_output_pos_length(original_self as *const ::list::ListFontDatabaseWritingSystem, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFontDatabaseWritingSystem::new](../struct.ListFontDatabaseWritingSystem.html#method.new) method.
  pub trait ListFontDatabaseWritingSystemNewArgs {
    fn exec(self) -> ::list::ListFontDatabaseWritingSystem;
  }
  impl<'a> ListFontDatabaseWritingSystemNewArgs for &'a ::list::ListFontDatabaseWritingSystem {
    fn exec(self) -> ::list::ListFontDatabaseWritingSystem {
      let l = self;
      {
        let mut object: ::list::ListFontDatabaseWritingSystem =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_constructor_l(l as *const ::list::ListFontDatabaseWritingSystem, &mut object);
        }
        object
      }
    }
  }
  impl ListFontDatabaseWritingSystemNewArgs for () {
    fn exec(self) -> ::list::ListFontDatabaseWritingSystem {

      {
        let mut object: ::list::ListFontDatabaseWritingSystem =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFontDatabaseWritingSystem::op_add_assign](../struct.ListFontDatabaseWritingSystem.html#method.op_add_assign) method.
  pub trait ListFontDatabaseWritingSystemOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListFontDatabaseWritingSystem)
            -> &'largs mut ::list::ListFontDatabaseWritingSystem;
  }
  impl<'largs> ListFontDatabaseWritingSystemOpAddAssignArgs<'largs> for &'largs ::list::ListFontDatabaseWritingSystem {
    fn exec(self,
            original_self: &'largs mut ::list::ListFontDatabaseWritingSystem)
            -> &'largs mut ::list::ListFontDatabaseWritingSystem {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_operator_add_assign_l(original_self as *mut ::list::ListFontDatabaseWritingSystem, l as *const ::list::ListFontDatabaseWritingSystem) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListFontDatabaseWritingSystemOpAddAssignArgs<'largs> for &'largs ::font_database::WritingSystem {
    fn exec(self,
            original_self: &'largs mut ::list::ListFontDatabaseWritingSystem)
            -> &'largs mut ::list::ListFontDatabaseWritingSystem {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_operator_add_assign_t(original_self as *mut ::list::ListFontDatabaseWritingSystem, t as *const ::font_database::WritingSystem) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListFontDatabaseWritingSystem::op_shl](../struct.ListFontDatabaseWritingSystem.html#method.op_shl) method.
  pub trait ListFontDatabaseWritingSystemOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListFontDatabaseWritingSystem)
            -> &'largs mut ::list::ListFontDatabaseWritingSystem;
  }
  impl<'largs> ListFontDatabaseWritingSystemOpShlArgs<'largs> for &'largs ::list::ListFontDatabaseWritingSystem {
    fn exec(self,
            original_self: &'largs mut ::list::ListFontDatabaseWritingSystem)
            -> &'largs mut ::list::ListFontDatabaseWritingSystem {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_operator_shl_l(original_self as *mut ::list::ListFontDatabaseWritingSystem, l as *const ::list::ListFontDatabaseWritingSystem) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListFontDatabaseWritingSystemOpShlArgs<'largs> for &'largs ::font_database::WritingSystem {
    fn exec(self,
            original_self: &'largs mut ::list::ListFontDatabaseWritingSystem)
            -> &'largs mut ::list::ListFontDatabaseWritingSystem {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_operator_shl_t(original_self as *mut ::list::ListFontDatabaseWritingSystem, t as *const ::font_database::WritingSystem) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListFontDatabaseWritingSystem::swap](../struct.ListFontDatabaseWritingSystem.html#method.swap) method.
  pub trait ListFontDatabaseWritingSystemSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListFontDatabaseWritingSystem) -> ();
  }
  impl<'largs> ListFontDatabaseWritingSystemSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListFontDatabaseWritingSystem) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_swap_i_j(original_self as *mut ::list::ListFontDatabaseWritingSystem, i, j) }
    }
  }
  impl<'largs> ListFontDatabaseWritingSystemSwapArgs<'largs> for &'largs mut ::list::ListFontDatabaseWritingSystem {
    fn exec(self, original_self: &'largs mut ::list::ListFontDatabaseWritingSystem) -> () {
      let other = self;
      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_swap_other(original_self as *mut ::list::ListFontDatabaseWritingSystem, other as *mut ::list::ListFontDatabaseWritingSystem) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListFontDatabaseWritingSystem::value](../struct.ListFontDatabaseWritingSystem.html#method.value) method.
  pub trait ListFontDatabaseWritingSystemValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::font_database::WritingSystem;
  }
  impl<'largs> ListFontDatabaseWritingSystemValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::font_database::WritingSystem {
      let i = self;
      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_value_i(original_self as *const ::list::ListFontDatabaseWritingSystem, i) }
    }
  }
  impl<'largs> ListFontDatabaseWritingSystemValueArgs<'largs>
    for (::libc::c_int, &'largs ::font_database::WritingSystem) {
    fn exec(self, original_self: &'largs ::list::ListFontDatabaseWritingSystem) -> ::font_database::WritingSystem {
      let i = self.0;
      let default_value = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QFontDatabase_WritingSystem_value_i_defaultValue(original_self as *const ::list::ListFontDatabaseWritingSystem, i, default_value as *const ::font_database::WritingSystem) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGlyphRun::append](../struct.ListGlyphRun.html#method.append) method.
  pub trait ListGlyphRunAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> ();
  }
  impl<'largs> ListGlyphRunAppendArgs<'largs> for &'largs ::glyph_run::GlyphRun {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_append_QGlyphRun(original_self as *mut ::list::ListGlyphRun,
                                                         t as *const ::glyph_run::GlyphRun)
      }
    }
  }
  impl<'largs> ListGlyphRunAppendArgs<'largs> for &'largs ::list::ListGlyphRun {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_append_QList_QGlyphRun(original_self as *mut ::list::ListGlyphRun,
                                                               t as *const ::list::ListGlyphRun)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGlyphRun::count](../struct.ListGlyphRun.html#method.count) method.
  pub trait ListGlyphRunCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::libc::c_int;
  }
  impl<'largs> ListGlyphRunCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_count_no_args(original_self as *const ::list::ListGlyphRun) }
    }
  }
  impl<'largs> ListGlyphRunCountArgs<'largs> for &'largs ::glyph_run::GlyphRun {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_count_t(original_self as *const ::list::ListGlyphRun,
                                                t as *const ::glyph_run::GlyphRun)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGlyphRun::index_of](../struct.ListGlyphRun.html#method.index_of) method.
  pub trait ListGlyphRunIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::libc::c_int;
  }
  impl<'largs> ListGlyphRunIndexOfArgs<'largs> for &'largs ::glyph_run::GlyphRun {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_indexOf_t(original_self as *const ::list::ListGlyphRun,
                                                  t as *const ::glyph_run::GlyphRun)
      }
    }
  }
  impl<'largs> ListGlyphRunIndexOfArgs<'largs> for (&'largs ::glyph_run::GlyphRun, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_indexOf_t_from(original_self as *const ::list::ListGlyphRun,
                                                       t as *const ::glyph_run::GlyphRun,
                                                       from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGlyphRun::last_index_of](../struct.ListGlyphRun.html#method.last_index_of) method.
  pub trait ListGlyphRunLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::libc::c_int;
  }
  impl<'largs> ListGlyphRunLastIndexOfArgs<'largs> for &'largs ::glyph_run::GlyphRun {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_lastIndexOf_t(original_self as *const ::list::ListGlyphRun,
                                                      t as *const ::glyph_run::GlyphRun)
      }
    }
  }
  impl<'largs> ListGlyphRunLastIndexOfArgs<'largs> for (&'largs ::glyph_run::GlyphRun, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_lastIndexOf_t_from(original_self as *const ::list::ListGlyphRun,
                                                           t as *const ::glyph_run::GlyphRun,
                                                           from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGlyphRun::mid](../struct.ListGlyphRun.html#method.mid) method.
  pub trait ListGlyphRunMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::list::ListGlyphRun;
  }
  impl<'largs> ListGlyphRunMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::list::ListGlyphRun {
      let pos = self;
      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QGlyphRun_mid_to_output_pos(original_self as *const ::list::ListGlyphRun,
                                                            pos,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListGlyphRunMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::list::ListGlyphRun {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QGlyphRun_mid_to_output_pos_length(original_self as *const ::list::ListGlyphRun,
                                                                   pos,
                                                                   length,
                                                                   &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGlyphRun::new](../struct.ListGlyphRun.html#method.new) method.
  pub trait ListGlyphRunNewArgs {
    fn exec(self) -> ::list::ListGlyphRun;
  }
  impl<'a> ListGlyphRunNewArgs for &'a ::list::ListGlyphRun {
    fn exec(self) -> ::list::ListGlyphRun {
      let l = self;
      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QGlyphRun_constructor_l(l as *const ::list::ListGlyphRun, &mut object);
        }
        object
      }
    }
  }
  impl ListGlyphRunNewArgs for () {
    fn exec(self) -> ::list::ListGlyphRun {

      {
        let mut object: ::list::ListGlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QGlyphRun_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGlyphRun::op_add_assign](../struct.ListGlyphRun.html#method.op_add_assign) method.
  pub trait ListGlyphRunOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> &'largs mut ::list::ListGlyphRun;
  }
  impl<'largs> ListGlyphRunOpAddAssignArgs<'largs> for &'largs ::list::ListGlyphRun {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> &'largs mut ::list::ListGlyphRun {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QGlyphRun_operator_add_assign_l(original_self as *mut ::list::ListGlyphRun,
                                                                l as *const ::list::ListGlyphRun)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListGlyphRunOpAddAssignArgs<'largs> for &'largs ::glyph_run::GlyphRun {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> &'largs mut ::list::ListGlyphRun {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QGlyphRun_operator_add_assign_t(original_self as *mut ::list::ListGlyphRun,
                                                                t as *const ::glyph_run::GlyphRun)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListGlyphRun::op_shl](../struct.ListGlyphRun.html#method.op_shl) method.
  pub trait ListGlyphRunOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> &'largs mut ::list::ListGlyphRun;
  }
  impl<'largs> ListGlyphRunOpShlArgs<'largs> for &'largs ::list::ListGlyphRun {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> &'largs mut ::list::ListGlyphRun {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_operator_shl_l(original_self as *mut ::list::ListGlyphRun,
                                                       l as *const ::list::ListGlyphRun)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListGlyphRunOpShlArgs<'largs> for &'largs ::glyph_run::GlyphRun {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> &'largs mut ::list::ListGlyphRun {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_operator_shl_t(original_self as *mut ::list::ListGlyphRun,
                                                       t as *const ::glyph_run::GlyphRun)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListGlyphRun::swap](../struct.ListGlyphRun.html#method.swap) method.
  pub trait ListGlyphRunSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> ();
  }
  impl<'largs> ListGlyphRunSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QGlyphRun_swap_i_j(original_self as *mut ::list::ListGlyphRun, i, j) }
    }
  }
  impl<'largs> ListGlyphRunSwapArgs<'largs> for &'largs mut ::list::ListGlyphRun {
    fn exec(self, original_self: &'largs mut ::list::ListGlyphRun) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QGlyphRun_swap_other(original_self as *mut ::list::ListGlyphRun,
                                                   other as *mut ::list::ListGlyphRun)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListGlyphRun::value](../struct.ListGlyphRun.html#method.value) method.
  pub trait ListGlyphRunValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::glyph_run::GlyphRun;
  }
  impl<'largs> ListGlyphRunValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::glyph_run::GlyphRun {
      let i = self;
      {
        let mut object: ::glyph_run::GlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QGlyphRun_value_to_output_i(original_self as *const ::list::ListGlyphRun,
                                                            i,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListGlyphRunValueArgs<'largs> for (::libc::c_int, &'largs ::glyph_run::GlyphRun) {
    fn exec(self, original_self: &'largs ::list::ListGlyphRun) -> ::glyph_run::GlyphRun {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::glyph_run::GlyphRun =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QGlyphRun_value_to_output_i_defaultValue(original_self as *const ::list::ListGlyphRun, i, default_value as *const ::glyph_run::GlyphRun, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListInputMethodEventAttribute::append](../struct.ListInputMethodEventAttribute.html#method.append) method.
  pub trait ListInputMethodEventAttributeAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListInputMethodEventAttribute) -> ();
  }
  impl<'largs> ListInputMethodEventAttributeAppendArgs<'largs> for &'largs ::input_method_event::Attribute {
    fn exec(self, original_self: &'largs mut ::list::ListInputMethodEventAttribute) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_append_QInputMethodEvent_Attribute(original_self as *mut ::list::ListInputMethodEventAttribute, t as *const ::input_method_event::Attribute) }
    }
  }
  impl<'largs> ListInputMethodEventAttributeAppendArgs<'largs> for &'largs ::list::ListInputMethodEventAttribute {
    fn exec(self, original_self: &'largs mut ::list::ListInputMethodEventAttribute) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_append_QList_QInputMethodEvent_Attribute(original_self as *mut ::list::ListInputMethodEventAttribute, t as *const ::list::ListInputMethodEventAttribute) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListInputMethodEventAttribute::mid](../struct.ListInputMethodEventAttribute.html#method.mid) method.
  pub trait ListInputMethodEventAttributeMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::list::ListInputMethodEventAttribute)
            -> ::list::ListInputMethodEventAttribute;
  }
  impl<'largs> ListInputMethodEventAttributeMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::list::ListInputMethodEventAttribute)
            -> ::list::ListInputMethodEventAttribute {
      let pos = self;
      {
        let mut object: ::list::ListInputMethodEventAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_mid_to_output_pos(original_self as *const ::list::ListInputMethodEventAttribute, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListInputMethodEventAttributeMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::list::ListInputMethodEventAttribute)
            -> ::list::ListInputMethodEventAttribute {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListInputMethodEventAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_mid_to_output_pos_length(original_self as *const ::list::ListInputMethodEventAttribute, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListInputMethodEventAttribute::new](../struct.ListInputMethodEventAttribute.html#method.new) method.
  pub trait ListInputMethodEventAttributeNewArgs {
    fn exec(self) -> ::list::ListInputMethodEventAttribute;
  }
  impl<'a> ListInputMethodEventAttributeNewArgs for &'a ::list::ListInputMethodEventAttribute {
    fn exec(self) -> ::list::ListInputMethodEventAttribute {
      let l = self;
      {
        let mut object: ::list::ListInputMethodEventAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_constructor_l(l as *const ::list::ListInputMethodEventAttribute, &mut object);
        }
        object
      }
    }
  }
  impl ListInputMethodEventAttributeNewArgs for () {
    fn exec(self) -> ::list::ListInputMethodEventAttribute {

      {
        let mut object: ::list::ListInputMethodEventAttribute =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListInputMethodEventAttribute::op_add_assign](../struct.ListInputMethodEventAttribute.html#method.op_add_assign) method.
  pub trait ListInputMethodEventAttributeOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListInputMethodEventAttribute)
            -> &'largs mut ::list::ListInputMethodEventAttribute;
  }
  impl<'largs> ListInputMethodEventAttributeOpAddAssignArgs<'largs> for &'largs ::list::ListInputMethodEventAttribute {
    fn exec(self,
            original_self: &'largs mut ::list::ListInputMethodEventAttribute)
            -> &'largs mut ::list::ListInputMethodEventAttribute {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_operator_add_assign_l(original_self as *mut ::list::ListInputMethodEventAttribute, l as *const ::list::ListInputMethodEventAttribute) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListInputMethodEventAttributeOpAddAssignArgs<'largs> for &'largs ::input_method_event::Attribute {
    fn exec(self,
            original_self: &'largs mut ::list::ListInputMethodEventAttribute)
            -> &'largs mut ::list::ListInputMethodEventAttribute {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_operator_add_assign_t(original_self as *mut ::list::ListInputMethodEventAttribute, t as *const ::input_method_event::Attribute) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListInputMethodEventAttribute::op_shl](../struct.ListInputMethodEventAttribute.html#method.op_shl) method.
  pub trait ListInputMethodEventAttributeOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListInputMethodEventAttribute)
            -> &'largs mut ::list::ListInputMethodEventAttribute;
  }
  impl<'largs> ListInputMethodEventAttributeOpShlArgs<'largs> for &'largs ::list::ListInputMethodEventAttribute {
    fn exec(self,
            original_self: &'largs mut ::list::ListInputMethodEventAttribute)
            -> &'largs mut ::list::ListInputMethodEventAttribute {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_operator_shl_l(original_self as *mut ::list::ListInputMethodEventAttribute, l as *const ::list::ListInputMethodEventAttribute) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListInputMethodEventAttributeOpShlArgs<'largs> for &'largs ::input_method_event::Attribute {
    fn exec(self,
            original_self: &'largs mut ::list::ListInputMethodEventAttribute)
            -> &'largs mut ::list::ListInputMethodEventAttribute {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_operator_shl_t(original_self as *mut ::list::ListInputMethodEventAttribute, t as *const ::input_method_event::Attribute) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListInputMethodEventAttribute::swap](../struct.ListInputMethodEventAttribute.html#method.swap) method.
  pub trait ListInputMethodEventAttributeSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListInputMethodEventAttribute) -> ();
  }
  impl<'largs> ListInputMethodEventAttributeSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListInputMethodEventAttribute) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_swap_i_j(original_self as *mut ::list::ListInputMethodEventAttribute, i, j) }
    }
  }
  impl<'largs> ListInputMethodEventAttributeSwapArgs<'largs> for &'largs mut ::list::ListInputMethodEventAttribute {
    fn exec(self, original_self: &'largs mut ::list::ListInputMethodEventAttribute) -> () {
      let other = self;
      unsafe { ::ffi::qt_gui_c_QList_QInputMethodEvent_Attribute_swap_other(original_self as *mut ::list::ListInputMethodEventAttribute, other as *mut ::list::ListInputMethodEventAttribute) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListKeySequence::append](../struct.ListKeySequence.html#method.append) method.
  pub trait ListKeySequenceAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> ();
  }
  impl<'largs> ListKeySequenceAppendArgs<'largs> for &'largs ::key_sequence::KeySequence {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_append_QKeySequence(original_self as *mut ::list::ListKeySequence,
                                                               t as *const ::key_sequence::KeySequence)
      }
    }
  }
  impl<'largs> ListKeySequenceAppendArgs<'largs> for &'largs ::list::ListKeySequence {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_append_QList_QKeySequence(original_self as *mut ::list::ListKeySequence,
                                                                     t as *const ::list::ListKeySequence)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListKeySequence::count](../struct.ListKeySequence.html#method.count) method.
  pub trait ListKeySequenceCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::libc::c_int;
  }
  impl<'largs> ListKeySequenceCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QList_QKeySequence_count_no_args(original_self as *const ::list::ListKeySequence) }
    }
  }
  impl<'largs> ListKeySequenceCountArgs<'largs> for &'largs ::key_sequence::KeySequence {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_count_t(original_self as *const ::list::ListKeySequence,
                                                   t as *const ::key_sequence::KeySequence)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListKeySequence::index_of](../struct.ListKeySequence.html#method.index_of) method.
  pub trait ListKeySequenceIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::libc::c_int;
  }
  impl<'largs> ListKeySequenceIndexOfArgs<'largs> for &'largs ::key_sequence::KeySequence {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_indexOf_t(original_self as *const ::list::ListKeySequence,
                                                     t as *const ::key_sequence::KeySequence)
      }
    }
  }
  impl<'largs> ListKeySequenceIndexOfArgs<'largs> for (&'largs ::key_sequence::KeySequence, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_indexOf_t_from(original_self as *const ::list::ListKeySequence,
                                                          t as *const ::key_sequence::KeySequence,
                                                          from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListKeySequence::last_index_of](../struct.ListKeySequence.html#method.last_index_of) method.
  pub trait ListKeySequenceLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::libc::c_int;
  }
  impl<'largs> ListKeySequenceLastIndexOfArgs<'largs> for &'largs ::key_sequence::KeySequence {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_lastIndexOf_t(original_self as *const ::list::ListKeySequence,
                                                         t as *const ::key_sequence::KeySequence)
      }
    }
  }
  impl<'largs> ListKeySequenceLastIndexOfArgs<'largs> for (&'largs ::key_sequence::KeySequence, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_lastIndexOf_t_from(original_self as *const ::list::ListKeySequence,
                                                              t as *const ::key_sequence::KeySequence,
                                                              from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListKeySequence::mid](../struct.ListKeySequence.html#method.mid) method.
  pub trait ListKeySequenceMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::list::ListKeySequence;
  }
  impl<'largs> ListKeySequenceMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::list::ListKeySequence {
      let pos = self;
      {
        let mut object: ::list::ListKeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QKeySequence_mid_to_output_pos(original_self as *const ::list::ListKeySequence,
                                                               pos,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListKeySequenceMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::list::ListKeySequence {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListKeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QKeySequence_mid_to_output_pos_length(original_self as *const ::list::ListKeySequence, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListKeySequence::new](../struct.ListKeySequence.html#method.new) method.
  pub trait ListKeySequenceNewArgs {
    fn exec(self) -> ::list::ListKeySequence;
  }
  impl<'a> ListKeySequenceNewArgs for &'a ::list::ListKeySequence {
    fn exec(self) -> ::list::ListKeySequence {
      let l = self;
      {
        let mut object: ::list::ListKeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QKeySequence_constructor_l(l as *const ::list::ListKeySequence, &mut object);
        }
        object
      }
    }
  }
  impl ListKeySequenceNewArgs for () {
    fn exec(self) -> ::list::ListKeySequence {

      {
        let mut object: ::list::ListKeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QKeySequence_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListKeySequence::op_add_assign](../struct.ListKeySequence.html#method.op_add_assign) method.
  pub trait ListKeySequenceOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> &'largs mut ::list::ListKeySequence;
  }
  impl<'largs> ListKeySequenceOpAddAssignArgs<'largs> for &'largs ::list::ListKeySequence {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> &'largs mut ::list::ListKeySequence {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QKeySequence_operator_add_assign_l(original_self as *mut ::list::ListKeySequence,
                                                                   l as *const ::list::ListKeySequence)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListKeySequenceOpAddAssignArgs<'largs> for &'largs ::key_sequence::KeySequence {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> &'largs mut ::list::ListKeySequence {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QKeySequence_operator_add_assign_t(original_self as *mut ::list::ListKeySequence,
                                                                   t as *const ::key_sequence::KeySequence)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListKeySequence::op_shl](../struct.ListKeySequence.html#method.op_shl) method.
  pub trait ListKeySequenceOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> &'largs mut ::list::ListKeySequence;
  }
  impl<'largs> ListKeySequenceOpShlArgs<'largs> for &'largs ::list::ListKeySequence {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> &'largs mut ::list::ListKeySequence {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QKeySequence_operator_shl_l(original_self as *mut ::list::ListKeySequence,
                                                            l as *const ::list::ListKeySequence)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListKeySequenceOpShlArgs<'largs> for &'largs ::key_sequence::KeySequence {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> &'largs mut ::list::ListKeySequence {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QKeySequence_operator_shl_t(original_self as *mut ::list::ListKeySequence,
                                                            t as *const ::key_sequence::KeySequence)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListKeySequence::swap](../struct.ListKeySequence.html#method.swap) method.
  pub trait ListKeySequenceSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> ();
  }
  impl<'largs> ListKeySequenceSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QKeySequence_swap_i_j(original_self as *mut ::list::ListKeySequence, i, j) }
    }
  }
  impl<'largs> ListKeySequenceSwapArgs<'largs> for &'largs mut ::list::ListKeySequence {
    fn exec(self, original_self: &'largs mut ::list::ListKeySequence) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QKeySequence_swap_other(original_self as *mut ::list::ListKeySequence,
                                                      other as *mut ::list::ListKeySequence)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListKeySequence::value](../struct.ListKeySequence.html#method.value) method.
  pub trait ListKeySequenceValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::key_sequence::KeySequence;
  }
  impl<'largs> ListKeySequenceValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::key_sequence::KeySequence {
      let i = self;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QKeySequence_value_to_output_i(original_self as *const ::list::ListKeySequence,
                                                               i,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListKeySequenceValueArgs<'largs> for (::libc::c_int, &'largs ::key_sequence::KeySequence) {
    fn exec(self, original_self: &'largs ::list::ListKeySequence) -> ::key_sequence::KeySequence {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::key_sequence::KeySequence =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QKeySequence_value_to_output_i_defaultValue(original_self as *const ::list::ListKeySequence, i, default_value as *const ::key_sequence::KeySequence, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglContextMutPtr::index_of](../struct.ListOpenglContextMutPtr.html#method.index_of) method.
  pub trait ListOpenglContextMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglContextMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListOpenglContextMutPtrIndexOfArgs<'largs> for &'largs *mut ::opengl_context::OpenGLContext {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglContextMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_indexOf_t(original_self as *const ::list::ListOpenglContextMutPtr,
                                                         t as *const *mut ::opengl_context::OpenGLContext)
    }
  }
  impl<'largs> ListOpenglContextMutPtrIndexOfArgs<'largs>
    for (&'largs *mut ::opengl_context::OpenGLContext, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglContextMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_indexOf_t_from(original_self as *const ::list::ListOpenglContextMutPtr,
                                                              t as *const *mut ::opengl_context::OpenGLContext,
                                                              from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglContextMutPtr::last_index_of](../struct.ListOpenglContextMutPtr.html#method.last_index_of) method.
  pub trait ListOpenglContextMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglContextMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListOpenglContextMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::opengl_context::OpenGLContext {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglContextMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_lastIndexOf_t(original_self as *const ::list::ListOpenglContextMutPtr,
                                                             t as *const *mut ::opengl_context::OpenGLContext)
    }
  }
  impl<'largs> ListOpenglContextMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::opengl_context::OpenGLContext, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglContextMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_lastIndexOf_t_from(original_self as *const ::list::ListOpenglContextMutPtr, t as *const *mut ::opengl_context::OpenGLContext, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglContextMutPtr::mid](../struct.ListOpenglContextMutPtr.html#method.mid) method.
  pub trait ListOpenglContextMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListOpenglContextMutPtr) -> ::list::ListOpenglContextMutPtr;
  }
  impl<'largs> ListOpenglContextMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListOpenglContextMutPtr) -> ::list::ListOpenglContextMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListOpenglContextMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_mid_to_output_pos(original_self as *const ::list::ListOpenglContextMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListOpenglContextMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListOpenglContextMutPtr) -> ::list::ListOpenglContextMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListOpenglContextMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_mid_to_output_pos_length(original_self as *const ::list::ListOpenglContextMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglContextMutPtr::new](../struct.ListOpenglContextMutPtr.html#method.new) method.
  pub trait ListOpenglContextMutPtrNewArgs {
    fn exec(self) -> ::list::ListOpenglContextMutPtr;
  }
  impl<'a> ListOpenglContextMutPtrNewArgs for &'a ::list::ListOpenglContextMutPtr {
    fn exec(self) -> ::list::ListOpenglContextMutPtr {
      let l = self;
      {
        let mut object: ::list::ListOpenglContextMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_constructor_l(l as *const ::list::ListOpenglContextMutPtr,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl ListOpenglContextMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListOpenglContextMutPtr {

      {
        let mut object: ::list::ListOpenglContextMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglContextMutPtr::swap](../struct.ListOpenglContextMutPtr.html#method.swap) method.
  pub trait ListOpenglContextMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglContextMutPtr) -> ();
  }
  impl<'largs> ListOpenglContextMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglContextMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_swap_i_j(original_self as *mut ::list::ListOpenglContextMutPtr, i, j)
      }
    }
  }
  impl<'largs> ListOpenglContextMutPtrSwapArgs<'largs> for &'largs mut ::list::ListOpenglContextMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglContextMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLContext_ptr_swap_other(original_self as *mut ::list::ListOpenglContextMutPtr,
                                                            other as *mut ::list::ListOpenglContextMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglDebugMessage::append](../struct.ListOpenglDebugMessage.html#method.append) method.
  pub trait ListOpenglDebugMessageAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglDebugMessage) -> ();
  }
  impl<'largs> ListOpenglDebugMessageAppendArgs<'largs> for &'largs ::list::ListOpenglDebugMessage {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglDebugMessage) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_append_QList_QOpenGLDebugMessage(original_self as *mut ::list::ListOpenglDebugMessage, t as *const ::list::ListOpenglDebugMessage) }
    }
  }
  impl<'largs> ListOpenglDebugMessageAppendArgs<'largs> for &'largs ::opengl_debug_message::OpenGLDebugMessage {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglDebugMessage) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_append_QOpenGLDebugMessage(original_self as *mut ::list::ListOpenglDebugMessage, t as *const ::opengl_debug_message::OpenGLDebugMessage) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglDebugMessage::count](../struct.ListOpenglDebugMessage.html#method.count) method.
  pub trait ListOpenglDebugMessageCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::libc::c_int;
  }
  impl<'largs> ListOpenglDebugMessageCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::libc::c_int {

      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_count_no_args(original_self as *const ::list::ListOpenglDebugMessage)
      }
    }
  }
  impl<'largs> ListOpenglDebugMessageCountArgs<'largs> for &'largs ::opengl_debug_message::OpenGLDebugMessage {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_count_t(original_self as *const ::list::ListOpenglDebugMessage,
                                                          t as *const ::opengl_debug_message::OpenGLDebugMessage)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglDebugMessage::index_of](../struct.ListOpenglDebugMessage.html#method.index_of) method.
  pub trait ListOpenglDebugMessageIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::libc::c_int;
  }
  impl<'largs> ListOpenglDebugMessageIndexOfArgs<'largs> for &'largs ::opengl_debug_message::OpenGLDebugMessage {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_indexOf_t(original_self as *const ::list::ListOpenglDebugMessage,
                                                            t as *const ::opengl_debug_message::OpenGLDebugMessage)
      }
    }
  }
  impl<'largs> ListOpenglDebugMessageIndexOfArgs<'largs>
    for (&'largs ::opengl_debug_message::OpenGLDebugMessage, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_indexOf_t_from(original_self as *const ::list::ListOpenglDebugMessage, t as *const ::opengl_debug_message::OpenGLDebugMessage, from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglDebugMessage::last_index_of](../struct.ListOpenglDebugMessage.html#method.last_index_of) method.
  pub trait ListOpenglDebugMessageLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::libc::c_int;
  }
  impl<'largs> ListOpenglDebugMessageLastIndexOfArgs<'largs> for &'largs ::opengl_debug_message::OpenGLDebugMessage {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_lastIndexOf_t(original_self as *const ::list::ListOpenglDebugMessage, t as *const ::opengl_debug_message::OpenGLDebugMessage)
      }
    }
  }
  impl<'largs> ListOpenglDebugMessageLastIndexOfArgs<'largs>
    for (&'largs ::opengl_debug_message::OpenGLDebugMessage, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_lastIndexOf_t_from(original_self as *const ::list::ListOpenglDebugMessage, t as *const ::opengl_debug_message::OpenGLDebugMessage, from) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglDebugMessage::mid](../struct.ListOpenglDebugMessage.html#method.mid) method.
  pub trait ListOpenglDebugMessageMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::list::ListOpenglDebugMessage;
  }
  impl<'largs> ListOpenglDebugMessageMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::list::ListOpenglDebugMessage {
      let pos = self;
      {
        let mut object: ::list::ListOpenglDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_mid_to_output_pos(original_self as *const ::list::ListOpenglDebugMessage, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListOpenglDebugMessageMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::list::ListOpenglDebugMessage {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListOpenglDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_mid_to_output_pos_length(original_self as *const ::list::ListOpenglDebugMessage, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglDebugMessage::new](../struct.ListOpenglDebugMessage.html#method.new) method.
  pub trait ListOpenglDebugMessageNewArgs {
    fn exec(self) -> ::list::ListOpenglDebugMessage;
  }
  impl<'a> ListOpenglDebugMessageNewArgs for &'a ::list::ListOpenglDebugMessage {
    fn exec(self) -> ::list::ListOpenglDebugMessage {
      let l = self;
      {
        let mut object: ::list::ListOpenglDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_constructor_l(l as *const ::list::ListOpenglDebugMessage,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl ListOpenglDebugMessageNewArgs for () {
    fn exec(self) -> ::list::ListOpenglDebugMessage {

      {
        let mut object: ::list::ListOpenglDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglDebugMessage::op_add_assign](../struct.ListOpenglDebugMessage.html#method.op_add_assign) method.
  pub trait ListOpenglDebugMessageOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListOpenglDebugMessage)
            -> &'largs mut ::list::ListOpenglDebugMessage;
  }
  impl<'largs> ListOpenglDebugMessageOpAddAssignArgs<'largs> for &'largs ::list::ListOpenglDebugMessage {
    fn exec(self,
            original_self: &'largs mut ::list::ListOpenglDebugMessage)
            -> &'largs mut ::list::ListOpenglDebugMessage {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_operator_add_assign_l(original_self as *mut ::list::ListOpenglDebugMessage, l as *const ::list::ListOpenglDebugMessage) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListOpenglDebugMessageOpAddAssignArgs<'largs> for &'largs ::opengl_debug_message::OpenGLDebugMessage {
    fn exec(self,
            original_self: &'largs mut ::list::ListOpenglDebugMessage)
            -> &'largs mut ::list::ListOpenglDebugMessage {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_operator_add_assign_t(original_self as *mut ::list::ListOpenglDebugMessage, t as *const ::opengl_debug_message::OpenGLDebugMessage) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglDebugMessage::op_shl](../struct.ListOpenglDebugMessage.html#method.op_shl) method.
  pub trait ListOpenglDebugMessageOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListOpenglDebugMessage)
            -> &'largs mut ::list::ListOpenglDebugMessage;
  }
  impl<'largs> ListOpenglDebugMessageOpShlArgs<'largs> for &'largs ::list::ListOpenglDebugMessage {
    fn exec(self,
            original_self: &'largs mut ::list::ListOpenglDebugMessage)
            -> &'largs mut ::list::ListOpenglDebugMessage {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_operator_shl_l(original_self as *mut ::list::ListOpenglDebugMessage, l as *const ::list::ListOpenglDebugMessage)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListOpenglDebugMessageOpShlArgs<'largs> for &'largs ::opengl_debug_message::OpenGLDebugMessage {
    fn exec(self,
            original_self: &'largs mut ::list::ListOpenglDebugMessage)
            -> &'largs mut ::list::ListOpenglDebugMessage {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_operator_shl_t(original_self as *mut ::list::ListOpenglDebugMessage, t as *const ::opengl_debug_message::OpenGLDebugMessage) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglDebugMessage::swap](../struct.ListOpenglDebugMessage.html#method.swap) method.
  pub trait ListOpenglDebugMessageSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglDebugMessage) -> ();
  }
  impl<'largs> ListOpenglDebugMessageSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglDebugMessage) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_swap_i_j(original_self as *mut ::list::ListOpenglDebugMessage, i, j)
      }
    }
  }
  impl<'largs> ListOpenglDebugMessageSwapArgs<'largs> for &'largs mut ::list::ListOpenglDebugMessage {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglDebugMessage) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_swap_other(original_self as *mut ::list::ListOpenglDebugMessage,
                                                             other as *mut ::list::ListOpenglDebugMessage)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglDebugMessage::value](../struct.ListOpenglDebugMessage.html#method.value) method.
  pub trait ListOpenglDebugMessageValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::opengl_debug_message::OpenGLDebugMessage;
  }
  impl<'largs> ListOpenglDebugMessageValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::opengl_debug_message::OpenGLDebugMessage {
      let i = self;
      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_value_to_output_i(original_self as *const ::list::ListOpenglDebugMessage, i, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListOpenglDebugMessageValueArgs<'largs>
    for (::libc::c_int, &'largs ::opengl_debug_message::OpenGLDebugMessage) {
    fn exec(self, original_self: &'largs ::list::ListOpenglDebugMessage) -> ::opengl_debug_message::OpenGLDebugMessage {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::opengl_debug_message::OpenGLDebugMessage =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLDebugMessage_value_to_output_i_defaultValue(original_self as *const ::list::ListOpenglDebugMessage, i, default_value as *const ::opengl_debug_message::OpenGLDebugMessage, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglShaderMutPtr::index_of](../struct.ListOpenglShaderMutPtr.html#method.index_of) method.
  pub trait ListOpenglShaderMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglShaderMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListOpenglShaderMutPtrIndexOfArgs<'largs> for &'largs *mut ::opengl_shader::OpenGLShader {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglShaderMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_indexOf_t(original_self as *const ::list::ListOpenglShaderMutPtr,
                                                        t as *const *mut ::opengl_shader::OpenGLShader)
    }
  }
  impl<'largs> ListOpenglShaderMutPtrIndexOfArgs<'largs> for (&'largs *mut ::opengl_shader::OpenGLShader, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglShaderMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_indexOf_t_from(original_self as *const ::list::ListOpenglShaderMutPtr,
                                                             t as *const *mut ::opengl_shader::OpenGLShader,
                                                             from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglShaderMutPtr::last_index_of](../struct.ListOpenglShaderMutPtr.html#method.last_index_of) method.
  pub trait ListOpenglShaderMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglShaderMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListOpenglShaderMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::opengl_shader::OpenGLShader {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglShaderMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_lastIndexOf_t(original_self as *const ::list::ListOpenglShaderMutPtr,
                                                            t as *const *mut ::opengl_shader::OpenGLShader)
    }
  }
  impl<'largs> ListOpenglShaderMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::opengl_shader::OpenGLShader, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListOpenglShaderMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_lastIndexOf_t_from(original_self as *const ::list::ListOpenglShaderMutPtr, t as *const *mut ::opengl_shader::OpenGLShader, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglShaderMutPtr::mid](../struct.ListOpenglShaderMutPtr.html#method.mid) method.
  pub trait ListOpenglShaderMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListOpenglShaderMutPtr) -> ::list::ListOpenglShaderMutPtr;
  }
  impl<'largs> ListOpenglShaderMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListOpenglShaderMutPtr) -> ::list::ListOpenglShaderMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListOpenglShaderMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_mid_to_output_pos(original_self as *const ::list::ListOpenglShaderMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListOpenglShaderMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListOpenglShaderMutPtr) -> ::list::ListOpenglShaderMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListOpenglShaderMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_mid_to_output_pos_length(original_self as *const ::list::ListOpenglShaderMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglShaderMutPtr::new](../struct.ListOpenglShaderMutPtr.html#method.new) method.
  pub trait ListOpenglShaderMutPtrNewArgs {
    fn exec(self) -> ::list::ListOpenglShaderMutPtr;
  }
  impl<'a> ListOpenglShaderMutPtrNewArgs for &'a ::list::ListOpenglShaderMutPtr {
    fn exec(self) -> ::list::ListOpenglShaderMutPtr {
      let l = self;
      {
        let mut object: ::list::ListOpenglShaderMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_constructor_l(l as *const ::list::ListOpenglShaderMutPtr,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl ListOpenglShaderMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListOpenglShaderMutPtr {

      {
        let mut object: ::list::ListOpenglShaderMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListOpenglShaderMutPtr::swap](../struct.ListOpenglShaderMutPtr.html#method.swap) method.
  pub trait ListOpenglShaderMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglShaderMutPtr) -> ();
  }
  impl<'largs> ListOpenglShaderMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglShaderMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_swap_i_j(original_self as *mut ::list::ListOpenglShaderMutPtr, i, j)
      }
    }
  }
  impl<'largs> ListOpenglShaderMutPtrSwapArgs<'largs> for &'largs mut ::list::ListOpenglShaderMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListOpenglShaderMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QOpenGLShader_ptr_swap_other(original_self as *mut ::list::ListOpenglShaderMutPtr,
                                                           other as *mut ::list::ListOpenglShaderMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPolygonF::append](../struct.ListPolygonF.html#method.append) method.
  pub trait ListPolygonFAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> ();
  }
  impl<'largs> ListPolygonFAppendArgs<'largs> for &'largs ::list::ListPolygonF {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_append_QList_QPolygonF(original_self as *mut ::list::ListPolygonF,
                                                               t as *const ::list::ListPolygonF)
      }
    }
  }
  impl<'largs> ListPolygonFAppendArgs<'largs> for &'largs ::polygon_f::PolygonF {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_append_QPolygonF(original_self as *mut ::list::ListPolygonF,
                                                         t as *const ::polygon_f::PolygonF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPolygonF::count](../struct.ListPolygonF.html#method.count) method.
  pub trait ListPolygonFCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::libc::c_int;
  }
  impl<'largs> ListPolygonFCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QList_QPolygonF_count_no_args(original_self as *const ::list::ListPolygonF) }
    }
  }
  impl<'largs> ListPolygonFCountArgs<'largs> for &'largs ::polygon_f::PolygonF {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_count_t(original_self as *const ::list::ListPolygonF,
                                                t as *const ::polygon_f::PolygonF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPolygonF::index_of](../struct.ListPolygonF.html#method.index_of) method.
  pub trait ListPolygonFIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::libc::c_int;
  }
  impl<'largs> ListPolygonFIndexOfArgs<'largs> for &'largs ::polygon_f::PolygonF {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_indexOf_t(original_self as *const ::list::ListPolygonF,
                                                  t as *const ::polygon_f::PolygonF)
      }
    }
  }
  impl<'largs> ListPolygonFIndexOfArgs<'largs> for (&'largs ::polygon_f::PolygonF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_indexOf_t_from(original_self as *const ::list::ListPolygonF,
                                                       t as *const ::polygon_f::PolygonF,
                                                       from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPolygonF::last_index_of](../struct.ListPolygonF.html#method.last_index_of) method.
  pub trait ListPolygonFLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::libc::c_int;
  }
  impl<'largs> ListPolygonFLastIndexOfArgs<'largs> for &'largs ::polygon_f::PolygonF {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_lastIndexOf_t(original_self as *const ::list::ListPolygonF,
                                                      t as *const ::polygon_f::PolygonF)
      }
    }
  }
  impl<'largs> ListPolygonFLastIndexOfArgs<'largs> for (&'largs ::polygon_f::PolygonF, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_lastIndexOf_t_from(original_self as *const ::list::ListPolygonF,
                                                           t as *const ::polygon_f::PolygonF,
                                                           from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPolygonF::mid](../struct.ListPolygonF.html#method.mid) method.
  pub trait ListPolygonFMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::list::ListPolygonF;
  }
  impl<'largs> ListPolygonFMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::list::ListPolygonF {
      let pos = self;
      {
        let mut object: ::list::ListPolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QPolygonF_mid_to_output_pos(original_self as *const ::list::ListPolygonF,
                                                            pos,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListPolygonFMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::list::ListPolygonF {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListPolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QPolygonF_mid_to_output_pos_length(original_self as *const ::list::ListPolygonF,
                                                                   pos,
                                                                   length,
                                                                   &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPolygonF::new](../struct.ListPolygonF.html#method.new) method.
  pub trait ListPolygonFNewArgs {
    fn exec(self) -> ::list::ListPolygonF;
  }
  impl<'a> ListPolygonFNewArgs for &'a ::list::ListPolygonF {
    fn exec(self) -> ::list::ListPolygonF {
      let l = self;
      {
        let mut object: ::list::ListPolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QPolygonF_constructor_l(l as *const ::list::ListPolygonF, &mut object);
        }
        object
      }
    }
  }
  impl ListPolygonFNewArgs for () {
    fn exec(self) -> ::list::ListPolygonF {

      {
        let mut object: ::list::ListPolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QPolygonF_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPolygonF::op_add_assign](../struct.ListPolygonF.html#method.op_add_assign) method.
  pub trait ListPolygonFOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> &'largs mut ::list::ListPolygonF;
  }
  impl<'largs> ListPolygonFOpAddAssignArgs<'largs> for &'largs ::list::ListPolygonF {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> &'largs mut ::list::ListPolygonF {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QPolygonF_operator_add_assign_l(original_self as *mut ::list::ListPolygonF,
                                                                l as *const ::list::ListPolygonF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListPolygonFOpAddAssignArgs<'largs> for &'largs ::polygon_f::PolygonF {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> &'largs mut ::list::ListPolygonF {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QPolygonF_operator_add_assign_t(original_self as *mut ::list::ListPolygonF,
                                                                t as *const ::polygon_f::PolygonF)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListPolygonF::op_shl](../struct.ListPolygonF.html#method.op_shl) method.
  pub trait ListPolygonFOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> &'largs mut ::list::ListPolygonF;
  }
  impl<'largs> ListPolygonFOpShlArgs<'largs> for &'largs ::list::ListPolygonF {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> &'largs mut ::list::ListPolygonF {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_operator_shl_l(original_self as *mut ::list::ListPolygonF,
                                                       l as *const ::list::ListPolygonF)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListPolygonFOpShlArgs<'largs> for &'largs ::polygon_f::PolygonF {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> &'largs mut ::list::ListPolygonF {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_operator_shl_t(original_self as *mut ::list::ListPolygonF,
                                                       t as *const ::polygon_f::PolygonF)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListPolygonF::swap](../struct.ListPolygonF.html#method.swap) method.
  pub trait ListPolygonFSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> ();
  }
  impl<'largs> ListPolygonFSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QPolygonF_swap_i_j(original_self as *mut ::list::ListPolygonF, i, j) }
    }
  }
  impl<'largs> ListPolygonFSwapArgs<'largs> for &'largs mut ::list::ListPolygonF {
    fn exec(self, original_self: &'largs mut ::list::ListPolygonF) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QPolygonF_swap_other(original_self as *mut ::list::ListPolygonF,
                                                   other as *mut ::list::ListPolygonF)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListPolygonF::value](../struct.ListPolygonF.html#method.value) method.
  pub trait ListPolygonFValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::polygon_f::PolygonF;
  }
  impl<'largs> ListPolygonFValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::polygon_f::PolygonF {
      let i = self;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QPolygonF_value_to_output_i(original_self as *const ::list::ListPolygonF,
                                                            i,
                                                            &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListPolygonFValueArgs<'largs> for (::libc::c_int, &'largs ::polygon_f::PolygonF) {
    fn exec(self, original_self: &'largs ::list::ListPolygonF) -> ::polygon_f::PolygonF {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::polygon_f::PolygonF =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QPolygonF_value_to_output_i_defaultValue(original_self as *const ::list::ListPolygonF, i, default_value as *const ::polygon_f::PolygonF, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreSize::append](../struct.ListQtCoreSize.html#method.append) method.
  pub trait ListQtCoreSizeAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> ();
  }
  impl<'largs> ListQtCoreSizeAppendArgs<'largs> for &'largs ::list::ListQtCoreSize {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_append_QList_QSize(original_self as *mut ::list::ListQtCoreSize,
                                                       t as *const ::list::ListQtCoreSize)
      }
    }
  }
  impl<'largs> ListQtCoreSizeAppendArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_append_QSize(original_self as *mut ::list::ListQtCoreSize,
                                                 t as *const ::qt_core::size::Size)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreSize::count](../struct.ListQtCoreSize.html#method.count) method.
  pub trait ListQtCoreSizeCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::libc::c_int;
  }
  impl<'largs> ListQtCoreSizeCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QList_QSize_count_no_args(original_self as *const ::list::ListQtCoreSize) }
    }
  }
  impl<'largs> ListQtCoreSizeCountArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_count_t(original_self as *const ::list::ListQtCoreSize,
                                            t as *const ::qt_core::size::Size)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreSize::index_of](../struct.ListQtCoreSize.html#method.index_of) method.
  pub trait ListQtCoreSizeIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::libc::c_int;
  }
  impl<'largs> ListQtCoreSizeIndexOfArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_indexOf_t(original_self as *const ::list::ListQtCoreSize,
                                              t as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> ListQtCoreSizeIndexOfArgs<'largs> for (&'largs ::qt_core::size::Size, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_indexOf_t_from(original_self as *const ::list::ListQtCoreSize,
                                                   t as *const ::qt_core::size::Size,
                                                   from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreSize::last_index_of](../struct.ListQtCoreSize.html#method.last_index_of) method.
  pub trait ListQtCoreSizeLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::libc::c_int;
  }
  impl<'largs> ListQtCoreSizeLastIndexOfArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_lastIndexOf_t(original_self as *const ::list::ListQtCoreSize,
                                                  t as *const ::qt_core::size::Size)
      }
    }
  }
  impl<'largs> ListQtCoreSizeLastIndexOfArgs<'largs> for (&'largs ::qt_core::size::Size, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_lastIndexOf_t_from(original_self as *const ::list::ListQtCoreSize,
                                                       t as *const ::qt_core::size::Size,
                                                       from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreSize::mid](../struct.ListQtCoreSize.html#method.mid) method.
  pub trait ListQtCoreSizeMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::list::ListQtCoreSize;
  }
  impl<'largs> ListQtCoreSizeMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::list::ListQtCoreSize {
      let pos = self;
      {
        let mut object: ::list::ListQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QSize_mid_to_output_pos(original_self as *const ::list::ListQtCoreSize,
                                                        pos,
                                                        &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListQtCoreSizeMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::list::ListQtCoreSize {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QSize_mid_to_output_pos_length(original_self as *const ::list::ListQtCoreSize,
                                                               pos,
                                                               length,
                                                               &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreSize::new](../struct.ListQtCoreSize.html#method.new) method.
  pub trait ListQtCoreSizeNewArgs {
    fn exec(self) -> ::list::ListQtCoreSize;
  }
  impl<'a> ListQtCoreSizeNewArgs for &'a ::list::ListQtCoreSize {
    fn exec(self) -> ::list::ListQtCoreSize {
      let l = self;
      {
        let mut object: ::list::ListQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QSize_constructor_l(l as *const ::list::ListQtCoreSize, &mut object);
        }
        object
      }
    }
  }
  impl ListQtCoreSizeNewArgs for () {
    fn exec(self) -> ::list::ListQtCoreSize {

      {
        let mut object: ::list::ListQtCoreSize =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QSize_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreSize::op_add_assign](../struct.ListQtCoreSize.html#method.op_add_assign) method.
  pub trait ListQtCoreSizeOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> &'largs mut ::list::ListQtCoreSize;
  }
  impl<'largs> ListQtCoreSizeOpAddAssignArgs<'largs> for &'largs ::list::ListQtCoreSize {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> &'largs mut ::list::ListQtCoreSize {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_QSize_operator_add_assign_l(original_self as *mut ::list::ListQtCoreSize,
                                                          l as *const ::list::ListQtCoreSize)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListQtCoreSizeOpAddAssignArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> &'largs mut ::list::ListQtCoreSize {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_QSize_operator_add_assign_t(original_self as *mut ::list::ListQtCoreSize,
                                                          t as *const ::qt_core::size::Size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreSize::op_shl](../struct.ListQtCoreSize.html#method.op_shl) method.
  pub trait ListQtCoreSizeOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> &'largs mut ::list::ListQtCoreSize;
  }
  impl<'largs> ListQtCoreSizeOpShlArgs<'largs> for &'largs ::list::ListQtCoreSize {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> &'largs mut ::list::ListQtCoreSize {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_QSize_operator_shl_l(original_self as *mut ::list::ListQtCoreSize,
                                                   l as *const ::list::ListQtCoreSize)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListQtCoreSizeOpShlArgs<'largs> for &'largs ::qt_core::size::Size {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> &'largs mut ::list::ListQtCoreSize {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_QSize_operator_shl_t(original_self as *mut ::list::ListQtCoreSize,
                                                   t as *const ::qt_core::size::Size)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreSize::swap](../struct.ListQtCoreSize.html#method.swap) method.
  pub trait ListQtCoreSizeSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> ();
  }
  impl<'largs> ListQtCoreSizeSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QSize_swap_i_j(original_self as *mut ::list::ListQtCoreSize, i, j) }
    }
  }
  impl<'largs> ListQtCoreSizeSwapArgs<'largs> for &'largs mut ::list::ListQtCoreSize {
    fn exec(self, original_self: &'largs mut ::list::ListQtCoreSize) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QSize_swap_other(original_self as *mut ::list::ListQtCoreSize,
                                               other as *mut ::list::ListQtCoreSize)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListQtCoreSize::value](../struct.ListQtCoreSize.html#method.value) method.
  pub trait ListQtCoreSizeValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::qt_core::size::Size;
  }
  impl<'largs> ListQtCoreSizeValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::qt_core::size::Size {
      let i = self;
      {
        let mut object: ::qt_core::size::Size =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QSize_value_to_output_i(original_self as *const ::list::ListQtCoreSize,
                                                        i,
                                                        &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListQtCoreSizeValueArgs<'largs> for (::libc::c_int, &'largs ::qt_core::size::Size) {
    fn exec(self, original_self: &'largs ::list::ListQtCoreSize) -> ::qt_core::size::Size {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::qt_core::size::Size =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QSize_value_to_output_i_defaultValue(original_self as *const ::list::ListQtCoreSize,
                                                                     i,
                                                                     default_value as *const ::qt_core::size::Size,
                                                                     &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListScreenMutPtr::index_of](../struct.ListScreenMutPtr.html#method.index_of) method.
  pub trait ListScreenMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListScreenMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListScreenMutPtrIndexOfArgs<'largs> for &'largs *mut ::screen::Screen {
    unsafe fn exec(self, original_self: &'largs ::list::ListScreenMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QScreen_ptr_indexOf_t(original_self as *const ::list::ListScreenMutPtr,
                                                  t as *const *mut ::screen::Screen)
    }
  }
  impl<'largs> ListScreenMutPtrIndexOfArgs<'largs> for (&'largs *mut ::screen::Screen, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListScreenMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QScreen_ptr_indexOf_t_from(original_self as *const ::list::ListScreenMutPtr,
                                                       t as *const *mut ::screen::Screen,
                                                       from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListScreenMutPtr::last_index_of](../struct.ListScreenMutPtr.html#method.last_index_of) method.
  pub trait ListScreenMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListScreenMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListScreenMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::screen::Screen {
    unsafe fn exec(self, original_self: &'largs ::list::ListScreenMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QScreen_ptr_lastIndexOf_t(original_self as *const ::list::ListScreenMutPtr,
                                                      t as *const *mut ::screen::Screen)
    }
  }
  impl<'largs> ListScreenMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::screen::Screen, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListScreenMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QScreen_ptr_lastIndexOf_t_from(original_self as *const ::list::ListScreenMutPtr,
                                                           t as *const *mut ::screen::Screen,
                                                           from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListScreenMutPtr::mid](../struct.ListScreenMutPtr.html#method.mid) method.
  pub trait ListScreenMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListScreenMutPtr) -> ::list::ListScreenMutPtr;
  }
  impl<'largs> ListScreenMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListScreenMutPtr) -> ::list::ListScreenMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListScreenMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QScreen_ptr_mid_to_output_pos(original_self as *const ::list::ListScreenMutPtr,
                                                              pos,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListScreenMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListScreenMutPtr) -> ::list::ListScreenMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListScreenMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QScreen_ptr_mid_to_output_pos_length(original_self as *const ::list::ListScreenMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListScreenMutPtr::new](../struct.ListScreenMutPtr.html#method.new) method.
  pub trait ListScreenMutPtrNewArgs {
    fn exec(self) -> ::list::ListScreenMutPtr;
  }
  impl<'a> ListScreenMutPtrNewArgs for &'a ::list::ListScreenMutPtr {
    fn exec(self) -> ::list::ListScreenMutPtr {
      let l = self;
      {
        let mut object: ::list::ListScreenMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QScreen_ptr_constructor_l(l as *const ::list::ListScreenMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListScreenMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListScreenMutPtr {

      {
        let mut object: ::list::ListScreenMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QScreen_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListScreenMutPtr::swap](../struct.ListScreenMutPtr.html#method.swap) method.
  pub trait ListScreenMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListScreenMutPtr) -> ();
  }
  impl<'largs> ListScreenMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListScreenMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QScreen_ptr_swap_i_j(original_self as *mut ::list::ListScreenMutPtr, i, j) }
    }
  }
  impl<'largs> ListScreenMutPtrSwapArgs<'largs> for &'largs mut ::list::ListScreenMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListScreenMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QScreen_ptr_swap_other(original_self as *mut ::list::ListScreenMutPtr,
                                                     other as *mut ::list::ListScreenMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListStandardItemMutPtr::index_of](../struct.ListStandardItemMutPtr.html#method.index_of) method.
  pub trait ListStandardItemMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListStandardItemMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListStandardItemMutPtrIndexOfArgs<'largs> for &'largs *mut ::standard_item::StandardItem {
    unsafe fn exec(self, original_self: &'largs ::list::ListStandardItemMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QStandardItem_ptr_indexOf_t(original_self as *const ::list::ListStandardItemMutPtr,
                                                        t as *const *mut ::standard_item::StandardItem)
    }
  }
  impl<'largs> ListStandardItemMutPtrIndexOfArgs<'largs> for (&'largs *mut ::standard_item::StandardItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListStandardItemMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QStandardItem_ptr_indexOf_t_from(original_self as *const ::list::ListStandardItemMutPtr,
                                                             t as *const *mut ::standard_item::StandardItem,
                                                             from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListStandardItemMutPtr::last_index_of](../struct.ListStandardItemMutPtr.html#method.last_index_of) method.
  pub trait ListStandardItemMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListStandardItemMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListStandardItemMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::standard_item::StandardItem {
    unsafe fn exec(self, original_self: &'largs ::list::ListStandardItemMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QStandardItem_ptr_lastIndexOf_t(original_self as *const ::list::ListStandardItemMutPtr,
                                                            t as *const *mut ::standard_item::StandardItem)
    }
  }
  impl<'largs> ListStandardItemMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::standard_item::StandardItem, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListStandardItemMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QStandardItem_ptr_lastIndexOf_t_from(original_self as *const ::list::ListStandardItemMutPtr, t as *const *mut ::standard_item::StandardItem, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListStandardItemMutPtr::mid](../struct.ListStandardItemMutPtr.html#method.mid) method.
  pub trait ListStandardItemMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListStandardItemMutPtr) -> ::list::ListStandardItemMutPtr;
  }
  impl<'largs> ListStandardItemMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListStandardItemMutPtr) -> ::list::ListStandardItemMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListStandardItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QStandardItem_ptr_mid_to_output_pos(original_self as *const ::list::ListStandardItemMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListStandardItemMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListStandardItemMutPtr) -> ::list::ListStandardItemMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListStandardItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QStandardItem_ptr_mid_to_output_pos_length(original_self as *const ::list::ListStandardItemMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListStandardItemMutPtr::new](../struct.ListStandardItemMutPtr.html#method.new) method.
  pub trait ListStandardItemMutPtrNewArgs {
    fn exec(self) -> ::list::ListStandardItemMutPtr;
  }
  impl<'a> ListStandardItemMutPtrNewArgs for &'a ::list::ListStandardItemMutPtr {
    fn exec(self) -> ::list::ListStandardItemMutPtr {
      let l = self;
      {
        let mut object: ::list::ListStandardItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QStandardItem_ptr_constructor_l(l as *const ::list::ListStandardItemMutPtr,
                                                                &mut object);
        }
        object
      }
    }
  }
  impl ListStandardItemMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListStandardItemMutPtr {

      {
        let mut object: ::list::ListStandardItemMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QStandardItem_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListStandardItemMutPtr::swap](../struct.ListStandardItemMutPtr.html#method.swap) method.
  pub trait ListStandardItemMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListStandardItemMutPtr) -> ();
  }
  impl<'largs> ListStandardItemMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListStandardItemMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QStandardItem_ptr_swap_i_j(original_self as *mut ::list::ListStandardItemMutPtr, i, j)
      }
    }
  }
  impl<'largs> ListStandardItemMutPtrSwapArgs<'largs> for &'largs mut ::list::ListStandardItemMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListStandardItemMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QStandardItem_ptr_swap_other(original_self as *mut ::list::ListStandardItemMutPtr,
                                                           other as *mut ::list::ListStandardItemMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextBlock::append](../struct.ListTextBlock.html#method.append) method.
  pub trait ListTextBlockAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> ();
  }
  impl<'largs> ListTextBlockAppendArgs<'largs> for &'largs ::list::ListTextBlock {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_append_QList_QTextBlock(original_self as *mut ::list::ListTextBlock,
                                                                 t as *const ::list::ListTextBlock)
      }
    }
  }
  impl<'largs> ListTextBlockAppendArgs<'largs> for &'largs ::text_block::TextBlock {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_append_QTextBlock(original_self as *mut ::list::ListTextBlock,
                                                           t as *const ::text_block::TextBlock)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextBlock::count](../struct.ListTextBlock.html#method.count) method.
  pub trait ListTextBlockCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::libc::c_int;
  }
  impl<'largs> ListTextBlockCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QList_QTextBlock_count_no_args(original_self as *const ::list::ListTextBlock) }
    }
  }
  impl<'largs> ListTextBlockCountArgs<'largs> for &'largs ::text_block::TextBlock {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_count_t(original_self as *const ::list::ListTextBlock,
                                                 t as *const ::text_block::TextBlock)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextBlock::index_of](../struct.ListTextBlock.html#method.index_of) method.
  pub trait ListTextBlockIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::libc::c_int;
  }
  impl<'largs> ListTextBlockIndexOfArgs<'largs> for &'largs ::text_block::TextBlock {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_indexOf_t(original_self as *const ::list::ListTextBlock,
                                                   t as *const ::text_block::TextBlock)
      }
    }
  }
  impl<'largs> ListTextBlockIndexOfArgs<'largs> for (&'largs ::text_block::TextBlock, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_indexOf_t_from(original_self as *const ::list::ListTextBlock,
                                                        t as *const ::text_block::TextBlock,
                                                        from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextBlock::last_index_of](../struct.ListTextBlock.html#method.last_index_of) method.
  pub trait ListTextBlockLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::libc::c_int;
  }
  impl<'largs> ListTextBlockLastIndexOfArgs<'largs> for &'largs ::text_block::TextBlock {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_lastIndexOf_t(original_self as *const ::list::ListTextBlock,
                                                       t as *const ::text_block::TextBlock)
      }
    }
  }
  impl<'largs> ListTextBlockLastIndexOfArgs<'largs> for (&'largs ::text_block::TextBlock, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_lastIndexOf_t_from(original_self as *const ::list::ListTextBlock,
                                                            t as *const ::text_block::TextBlock,
                                                            from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextBlock::mid](../struct.ListTextBlock.html#method.mid) method.
  pub trait ListTextBlockMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::list::ListTextBlock;
  }
  impl<'largs> ListTextBlockMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::list::ListTextBlock {
      let pos = self;
      {
        let mut object: ::list::ListTextBlock =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextBlock_mid_to_output_pos(original_self as *const ::list::ListTextBlock,
                                                             pos,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTextBlockMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::list::ListTextBlock {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListTextBlock =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextBlock_mid_to_output_pos_length(original_self as *const ::list::ListTextBlock,
                                                                    pos,
                                                                    length,
                                                                    &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextBlock::new](../struct.ListTextBlock.html#method.new) method.
  pub trait ListTextBlockNewArgs {
    fn exec(self) -> ::list::ListTextBlock;
  }
  impl<'a> ListTextBlockNewArgs for &'a ::list::ListTextBlock {
    fn exec(self) -> ::list::ListTextBlock {
      let l = self;
      {
        let mut object: ::list::ListTextBlock =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextBlock_constructor_l(l as *const ::list::ListTextBlock, &mut object);
        }
        object
      }
    }
  }
  impl ListTextBlockNewArgs for () {
    fn exec(self) -> ::list::ListTextBlock {

      {
        let mut object: ::list::ListTextBlock =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextBlock_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextBlock::op_add_assign](../struct.ListTextBlock.html#method.op_add_assign) method.
  pub trait ListTextBlockOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> &'largs mut ::list::ListTextBlock;
  }
  impl<'largs> ListTextBlockOpAddAssignArgs<'largs> for &'largs ::list::ListTextBlock {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> &'largs mut ::list::ListTextBlock {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QTextBlock_operator_add_assign_l(original_self as *mut ::list::ListTextBlock,
                                                                 l as *const ::list::ListTextBlock)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTextBlockOpAddAssignArgs<'largs> for &'largs ::text_block::TextBlock {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> &'largs mut ::list::ListTextBlock {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QTextBlock_operator_add_assign_t(original_self as *mut ::list::ListTextBlock,
                                                                 t as *const ::text_block::TextBlock)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextBlock::op_shl](../struct.ListTextBlock.html#method.op_shl) method.
  pub trait ListTextBlockOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> &'largs mut ::list::ListTextBlock;
  }
  impl<'largs> ListTextBlockOpShlArgs<'largs> for &'largs ::list::ListTextBlock {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> &'largs mut ::list::ListTextBlock {
      let l = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_operator_shl_l(original_self as *mut ::list::ListTextBlock,
                                                        l as *const ::list::ListTextBlock)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTextBlockOpShlArgs<'largs> for &'largs ::text_block::TextBlock {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> &'largs mut ::list::ListTextBlock {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_operator_shl_t(original_self as *mut ::list::ListTextBlock,
                                                        t as *const ::text_block::TextBlock)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextBlock::swap](../struct.ListTextBlock.html#method.swap) method.
  pub trait ListTextBlockSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> ();
  }
  impl<'largs> ListTextBlockSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QTextBlock_swap_i_j(original_self as *mut ::list::ListTextBlock, i, j) }
    }
  }
  impl<'largs> ListTextBlockSwapArgs<'largs> for &'largs mut ::list::ListTextBlock {
    fn exec(self, original_self: &'largs mut ::list::ListTextBlock) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextBlock_swap_other(original_self as *mut ::list::ListTextBlock,
                                                    other as *mut ::list::ListTextBlock)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextBlock::value](../struct.ListTextBlock.html#method.value) method.
  pub trait ListTextBlockValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::text_block::TextBlock;
  }
  impl<'largs> ListTextBlockValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::text_block::TextBlock {
      let i = self;
      {
        let mut object: ::text_block::TextBlock =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextBlock_value_to_output_i(original_self as *const ::list::ListTextBlock,
                                                             i,
                                                             &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTextBlockValueArgs<'largs> for (::libc::c_int, &'largs ::text_block::TextBlock) {
    fn exec(self, original_self: &'largs ::list::ListTextBlock) -> ::text_block::TextBlock {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::text_block::TextBlock =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextBlock_value_to_output_i_defaultValue(original_self as *const ::list::ListTextBlock, i, default_value as *const ::text_block::TextBlock, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextFrameMutPtr::index_of](../struct.ListTextFrameMutPtr.html#method.index_of) method.
  pub trait ListTextFrameMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListTextFrameMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListTextFrameMutPtrIndexOfArgs<'largs> for &'largs *mut ::text_frame::TextFrame {
    unsafe fn exec(self, original_self: &'largs ::list::ListTextFrameMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QTextFrame_ptr_indexOf_t(original_self as *const ::list::ListTextFrameMutPtr,
                                                     t as *const *mut ::text_frame::TextFrame)
    }
  }
  impl<'largs> ListTextFrameMutPtrIndexOfArgs<'largs> for (&'largs *mut ::text_frame::TextFrame, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListTextFrameMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QTextFrame_ptr_indexOf_t_from(original_self as *const ::list::ListTextFrameMutPtr,
                                                          t as *const *mut ::text_frame::TextFrame,
                                                          from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextFrameMutPtr::last_index_of](../struct.ListTextFrameMutPtr.html#method.last_index_of) method.
  pub trait ListTextFrameMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListTextFrameMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListTextFrameMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::text_frame::TextFrame {
    unsafe fn exec(self, original_self: &'largs ::list::ListTextFrameMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QTextFrame_ptr_lastIndexOf_t(original_self as *const ::list::ListTextFrameMutPtr,
                                                         t as *const *mut ::text_frame::TextFrame)
    }
  }
  impl<'largs> ListTextFrameMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::text_frame::TextFrame, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListTextFrameMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QTextFrame_ptr_lastIndexOf_t_from(original_self as *const ::list::ListTextFrameMutPtr,
                                                              t as *const *mut ::text_frame::TextFrame,
                                                              from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextFrameMutPtr::mid](../struct.ListTextFrameMutPtr.html#method.mid) method.
  pub trait ListTextFrameMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextFrameMutPtr) -> ::list::ListTextFrameMutPtr;
  }
  impl<'largs> ListTextFrameMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListTextFrameMutPtr) -> ::list::ListTextFrameMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListTextFrameMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextFrame_ptr_mid_to_output_pos(original_self as *const ::list::ListTextFrameMutPtr,
                                                                 pos,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTextFrameMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTextFrameMutPtr) -> ::list::ListTextFrameMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListTextFrameMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextFrame_ptr_mid_to_output_pos_length(original_self as *const ::list::ListTextFrameMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextFrameMutPtr::new](../struct.ListTextFrameMutPtr.html#method.new) method.
  pub trait ListTextFrameMutPtrNewArgs {
    fn exec(self) -> ::list::ListTextFrameMutPtr;
  }
  impl<'a> ListTextFrameMutPtrNewArgs for &'a ::list::ListTextFrameMutPtr {
    fn exec(self) -> ::list::ListTextFrameMutPtr {
      let l = self;
      {
        let mut object: ::list::ListTextFrameMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextFrame_ptr_constructor_l(l as *const ::list::ListTextFrameMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListTextFrameMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListTextFrameMutPtr {

      {
        let mut object: ::list::ListTextFrameMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextFrame_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextFrameMutPtr::swap](../struct.ListTextFrameMutPtr.html#method.swap) method.
  pub trait ListTextFrameMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextFrameMutPtr) -> ();
  }
  impl<'largs> ListTextFrameMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListTextFrameMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QTextFrame_ptr_swap_i_j(original_self as *mut ::list::ListTextFrameMutPtr, i, j) }
    }
  }
  impl<'largs> ListTextFrameMutPtrSwapArgs<'largs> for &'largs mut ::list::ListTextFrameMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListTextFrameMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextFrame_ptr_swap_other(original_self as *mut ::list::ListTextFrameMutPtr,
                                                        other as *mut ::list::ListTextFrameMutPtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextLayoutFormatRange::append](../struct.ListTextLayoutFormatRange.html#method.append) method.
  pub trait ListTextLayoutFormatRangeAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextLayoutFormatRange) -> ();
  }
  impl<'largs> ListTextLayoutFormatRangeAppendArgs<'largs> for &'largs ::list::ListTextLayoutFormatRange {
    fn exec(self, original_self: &'largs mut ::list::ListTextLayoutFormatRange) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_append_QList_QTextLayout_FormatRange(original_self as *mut ::list::ListTextLayoutFormatRange, t as *const ::list::ListTextLayoutFormatRange) }
    }
  }
  impl<'largs> ListTextLayoutFormatRangeAppendArgs<'largs> for &'largs ::text_layout::FormatRange {
    fn exec(self, original_self: &'largs mut ::list::ListTextLayoutFormatRange) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_append_QTextLayout_FormatRange(original_self as *mut ::list::ListTextLayoutFormatRange, t as *const ::text_layout::FormatRange) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextLayoutFormatRange::mid](../struct.ListTextLayoutFormatRange.html#method.mid) method.
  pub trait ListTextLayoutFormatRangeMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextLayoutFormatRange) -> ::list::ListTextLayoutFormatRange;
  }
  impl<'largs> ListTextLayoutFormatRangeMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListTextLayoutFormatRange) -> ::list::ListTextLayoutFormatRange {
      let pos = self;
      {
        let mut object: ::list::ListTextLayoutFormatRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_mid_to_output_pos(original_self as *const ::list::ListTextLayoutFormatRange, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTextLayoutFormatRangeMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTextLayoutFormatRange) -> ::list::ListTextLayoutFormatRange {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListTextLayoutFormatRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_mid_to_output_pos_length(original_self as *const ::list::ListTextLayoutFormatRange, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextLayoutFormatRange::new](../struct.ListTextLayoutFormatRange.html#method.new) method.
  pub trait ListTextLayoutFormatRangeNewArgs {
    fn exec(self) -> ::list::ListTextLayoutFormatRange;
  }
  impl<'a> ListTextLayoutFormatRangeNewArgs for &'a ::list::ListTextLayoutFormatRange {
    fn exec(self) -> ::list::ListTextLayoutFormatRange {
      let l = self;
      {
        let mut object: ::list::ListTextLayoutFormatRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_constructor_l(l as *const ::list::ListTextLayoutFormatRange,
                                                                      &mut object);
        }
        object
      }
    }
  }
  impl ListTextLayoutFormatRangeNewArgs for () {
    fn exec(self) -> ::list::ListTextLayoutFormatRange {

      {
        let mut object: ::list::ListTextLayoutFormatRange =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextLayoutFormatRange::op_add_assign](../struct.ListTextLayoutFormatRange.html#method.op_add_assign) method.
  pub trait ListTextLayoutFormatRangeOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextLayoutFormatRange)
            -> &'largs mut ::list::ListTextLayoutFormatRange;
  }
  impl<'largs> ListTextLayoutFormatRangeOpAddAssignArgs<'largs> for &'largs ::list::ListTextLayoutFormatRange {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextLayoutFormatRange)
            -> &'largs mut ::list::ListTextLayoutFormatRange {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_operator_add_assign_l(original_self as *mut ::list::ListTextLayoutFormatRange, l as *const ::list::ListTextLayoutFormatRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTextLayoutFormatRangeOpAddAssignArgs<'largs> for &'largs ::text_layout::FormatRange {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextLayoutFormatRange)
            -> &'largs mut ::list::ListTextLayoutFormatRange {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_operator_add_assign_t(original_self as *mut ::list::ListTextLayoutFormatRange, t as *const ::text_layout::FormatRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextLayoutFormatRange::op_shl](../struct.ListTextLayoutFormatRange.html#method.op_shl) method.
  pub trait ListTextLayoutFormatRangeOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextLayoutFormatRange)
            -> &'largs mut ::list::ListTextLayoutFormatRange;
  }
  impl<'largs> ListTextLayoutFormatRangeOpShlArgs<'largs> for &'largs ::list::ListTextLayoutFormatRange {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextLayoutFormatRange)
            -> &'largs mut ::list::ListTextLayoutFormatRange {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_operator_shl_l(original_self as *mut ::list::ListTextLayoutFormatRange, l as *const ::list::ListTextLayoutFormatRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTextLayoutFormatRangeOpShlArgs<'largs> for &'largs ::text_layout::FormatRange {
    fn exec(self,
            original_self: &'largs mut ::list::ListTextLayoutFormatRange)
            -> &'largs mut ::list::ListTextLayoutFormatRange {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_operator_shl_t(original_self as *mut ::list::ListTextLayoutFormatRange, t as *const ::text_layout::FormatRange) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextLayoutFormatRange::swap](../struct.ListTextLayoutFormatRange.html#method.swap) method.
  pub trait ListTextLayoutFormatRangeSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextLayoutFormatRange) -> ();
  }
  impl<'largs> ListTextLayoutFormatRangeSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListTextLayoutFormatRange) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_swap_i_j(original_self as *mut ::list::ListTextLayoutFormatRange, i, j)
      }
    }
  }
  impl<'largs> ListTextLayoutFormatRangeSwapArgs<'largs> for &'largs mut ::list::ListTextLayoutFormatRange {
    fn exec(self, original_self: &'largs mut ::list::ListTextLayoutFormatRange) -> () {
      let other = self;
      unsafe { ::ffi::qt_gui_c_QList_QTextLayout_FormatRange_swap_other(original_self as *mut ::list::ListTextLayoutFormatRange, other as *mut ::list::ListTextLayoutFormatRange) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextOptionTab::append](../struct.ListTextOptionTab.html#method.append) method.
  pub trait ListTextOptionTabAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> ();
  }
  impl<'largs> ListTextOptionTabAppendArgs<'largs> for &'largs ::list::ListTextOptionTab {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_append_QList_QTextOption_Tab(original_self as *mut ::list::ListTextOptionTab, t as *const ::list::ListTextOptionTab) }
    }
  }
  impl<'largs> ListTextOptionTabAppendArgs<'largs> for &'largs ::text_option::Tab {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_append_QTextOption_Tab(original_self as *mut ::list::ListTextOptionTab,
                                                                     t as *const ::text_option::Tab)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextOptionTab::count](../struct.ListTextOptionTab.html#method.count) method.
  pub trait ListTextOptionTabCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::libc::c_int;
  }
  impl<'largs> ListTextOptionTabCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::libc::c_int {

      unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_count_no_args(original_self as *const ::list::ListTextOptionTab) }
    }
  }
  impl<'largs> ListTextOptionTabCountArgs<'largs> for &'largs ::text_option::Tab {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_count_t(original_self as *const ::list::ListTextOptionTab,
                                                      t as *const ::text_option::Tab)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextOptionTab::index_of](../struct.ListTextOptionTab.html#method.index_of) method.
  pub trait ListTextOptionTabIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::libc::c_int;
  }
  impl<'largs> ListTextOptionTabIndexOfArgs<'largs> for &'largs ::text_option::Tab {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_indexOf_t(original_self as *const ::list::ListTextOptionTab,
                                                        t as *const ::text_option::Tab)
      }
    }
  }
  impl<'largs> ListTextOptionTabIndexOfArgs<'largs> for (&'largs ::text_option::Tab, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_indexOf_t_from(original_self as *const ::list::ListTextOptionTab,
                                                             t as *const ::text_option::Tab,
                                                             from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextOptionTab::last_index_of](../struct.ListTextOptionTab.html#method.last_index_of) method.
  pub trait ListTextOptionTabLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::libc::c_int;
  }
  impl<'largs> ListTextOptionTabLastIndexOfArgs<'largs> for &'largs ::text_option::Tab {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_lastIndexOf_t(original_self as *const ::list::ListTextOptionTab,
                                                            t as *const ::text_option::Tab)
      }
    }
  }
  impl<'largs> ListTextOptionTabLastIndexOfArgs<'largs> for (&'largs ::text_option::Tab, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_lastIndexOf_t_from(original_self as *const ::list::ListTextOptionTab,
                                                                 t as *const ::text_option::Tab,
                                                                 from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextOptionTab::mid](../struct.ListTextOptionTab.html#method.mid) method.
  pub trait ListTextOptionTabMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::list::ListTextOptionTab;
  }
  impl<'largs> ListTextOptionTabMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::list::ListTextOptionTab {
      let pos = self;
      {
        let mut object: ::list::ListTextOptionTab =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextOption_Tab_mid_to_output_pos(original_self as *const ::list::ListTextOptionTab,
                                                                  pos,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTextOptionTabMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::list::ListTextOptionTab {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListTextOptionTab =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextOption_Tab_mid_to_output_pos_length(original_self as *const ::list::ListTextOptionTab, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextOptionTab::new](../struct.ListTextOptionTab.html#method.new) method.
  pub trait ListTextOptionTabNewArgs {
    fn exec(self) -> ::list::ListTextOptionTab;
  }
  impl<'a> ListTextOptionTabNewArgs for &'a ::list::ListTextOptionTab {
    fn exec(self) -> ::list::ListTextOptionTab {
      let l = self;
      {
        let mut object: ::list::ListTextOptionTab =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextOption_Tab_constructor_l(l as *const ::list::ListTextOptionTab, &mut object);
        }
        object
      }
    }
  }
  impl ListTextOptionTabNewArgs for () {
    fn exec(self) -> ::list::ListTextOptionTab {

      {
        let mut object: ::list::ListTextOptionTab =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextOption_Tab_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextOptionTab::op_add_assign](../struct.ListTextOptionTab.html#method.op_add_assign) method.
  pub trait ListTextOptionTabOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> &'largs mut ::list::ListTextOptionTab;
  }
  impl<'largs> ListTextOptionTabOpAddAssignArgs<'largs> for &'largs ::list::ListTextOptionTab {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> &'largs mut ::list::ListTextOptionTab {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QTextOption_Tab_operator_add_assign_l(original_self as *mut ::list::ListTextOptionTab,
                                                                      l as *const ::list::ListTextOptionTab)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTextOptionTabOpAddAssignArgs<'largs> for &'largs ::text_option::Tab {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> &'largs mut ::list::ListTextOptionTab {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QTextOption_Tab_operator_add_assign_t(original_self as *mut ::list::ListTextOptionTab,
                                                                      t as *const ::text_option::Tab)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextOptionTab::op_shl](../struct.ListTextOptionTab.html#method.op_shl) method.
  pub trait ListTextOptionTabOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> &'largs mut ::list::ListTextOptionTab;
  }
  impl<'largs> ListTextOptionTabOpShlArgs<'largs> for &'largs ::list::ListTextOptionTab {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> &'largs mut ::list::ListTextOptionTab {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QTextOption_Tab_operator_shl_l(original_self as *mut ::list::ListTextOptionTab,
                                                               l as *const ::list::ListTextOptionTab)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTextOptionTabOpShlArgs<'largs> for &'largs ::text_option::Tab {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> &'largs mut ::list::ListTextOptionTab {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_gui_c_QList_QTextOption_Tab_operator_shl_t(original_self as *mut ::list::ListTextOptionTab,
                                                               t as *const ::text_option::Tab)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextOptionTab::swap](../struct.ListTextOptionTab.html#method.swap) method.
  pub trait ListTextOptionTabSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> ();
  }
  impl<'largs> ListTextOptionTabSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QTextOption_Tab_swap_i_j(original_self as *mut ::list::ListTextOptionTab, i, j) }
    }
  }
  impl<'largs> ListTextOptionTabSwapArgs<'largs> for &'largs mut ::list::ListTextOptionTab {
    fn exec(self, original_self: &'largs mut ::list::ListTextOptionTab) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTextOption_Tab_swap_other(original_self as *mut ::list::ListTextOptionTab,
                                                         other as *mut ::list::ListTextOptionTab)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTextOptionTab::value](../struct.ListTextOptionTab.html#method.value) method.
  pub trait ListTextOptionTabValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::text_option::Tab;
  }
  impl<'largs> ListTextOptionTabValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::text_option::Tab {
      let i = self;
      {
        let mut object: ::text_option::Tab =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextOption_Tab_value_to_output_i(original_self as *const ::list::ListTextOptionTab,
                                                                  i,
                                                                  &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTextOptionTabValueArgs<'largs> for (::libc::c_int, &'largs ::text_option::Tab) {
    fn exec(self, original_self: &'largs ::list::ListTextOptionTab) -> ::text_option::Tab {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::text_option::Tab =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTextOption_Tab_value_to_output_i_defaultValue(original_self as *const ::list::ListTextOptionTab, i, default_value as *const ::text_option::Tab, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTouchDevicePtr::index_of](../struct.ListTouchDevicePtr.html#method.index_of) method.
  pub trait ListTouchDevicePtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListTouchDevicePtr) -> ::libc::c_int;
  }
  impl<'largs> ListTouchDevicePtrIndexOfArgs<'largs> for &'largs *const ::touch_device::TouchDevice {
    unsafe fn exec(self, original_self: &'largs ::list::ListTouchDevicePtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_indexOf_t(original_self as *const ::list::ListTouchDevicePtr,
                                                             t as *const *const ::touch_device::TouchDevice)
    }
  }
  impl<'largs> ListTouchDevicePtrIndexOfArgs<'largs> for (&'largs *const ::touch_device::TouchDevice, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListTouchDevicePtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_indexOf_t_from(original_self as *const ::list::ListTouchDevicePtr,
                                                                  t as *const *const ::touch_device::TouchDevice,
                                                                  from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListTouchDevicePtr::last_index_of](../struct.ListTouchDevicePtr.html#method.last_index_of) method.
  pub trait ListTouchDevicePtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListTouchDevicePtr) -> ::libc::c_int;
  }
  impl<'largs> ListTouchDevicePtrLastIndexOfArgs<'largs> for &'largs *const ::touch_device::TouchDevice {
    unsafe fn exec(self, original_self: &'largs ::list::ListTouchDevicePtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_lastIndexOf_t(original_self as *const ::list::ListTouchDevicePtr,
                                                                 t as *const *const ::touch_device::TouchDevice)
    }
  }
  impl<'largs> ListTouchDevicePtrLastIndexOfArgs<'largs> for (&'largs *const ::touch_device::TouchDevice, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListTouchDevicePtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_lastIndexOf_t_from(original_self as *const ::list::ListTouchDevicePtr, t as *const *const ::touch_device::TouchDevice, from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListTouchDevicePtr::mid](../struct.ListTouchDevicePtr.html#method.mid) method.
  pub trait ListTouchDevicePtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTouchDevicePtr) -> ::list::ListTouchDevicePtr;
  }
  impl<'largs> ListTouchDevicePtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListTouchDevicePtr) -> ::list::ListTouchDevicePtr {
      let pos = self;
      {
        let mut object: ::list::ListTouchDevicePtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_mid_to_output_pos(original_self as *const ::list::ListTouchDevicePtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTouchDevicePtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTouchDevicePtr) -> ::list::ListTouchDevicePtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListTouchDevicePtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_mid_to_output_pos_length(original_self as *const ::list::ListTouchDevicePtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTouchDevicePtr::new](../struct.ListTouchDevicePtr.html#method.new) method.
  pub trait ListTouchDevicePtrNewArgs {
    fn exec(self) -> ::list::ListTouchDevicePtr;
  }
  impl<'a> ListTouchDevicePtrNewArgs for &'a ::list::ListTouchDevicePtr {
    fn exec(self) -> ::list::ListTouchDevicePtr {
      let l = self;
      {
        let mut object: ::list::ListTouchDevicePtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_constructor_l(l as *const ::list::ListTouchDevicePtr,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl ListTouchDevicePtrNewArgs for () {
    fn exec(self) -> ::list::ListTouchDevicePtr {

      {
        let mut object: ::list::ListTouchDevicePtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTouchDevicePtr::swap](../struct.ListTouchDevicePtr.html#method.swap) method.
  pub trait ListTouchDevicePtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTouchDevicePtr) -> ();
  }
  impl<'largs> ListTouchDevicePtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListTouchDevicePtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_swap_i_j(original_self as *mut ::list::ListTouchDevicePtr, i, j)
      }
    }
  }
  impl<'largs> ListTouchDevicePtrSwapArgs<'largs> for &'largs mut ::list::ListTouchDevicePtr {
    fn exec(self, original_self: &'largs mut ::list::ListTouchDevicePtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_const_QTouchDevice_ptr_swap_other(original_self as *mut ::list::ListTouchDevicePtr,
                                                                other as *mut ::list::ListTouchDevicePtr)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTouchEventTouchPoint::append](../struct.ListTouchEventTouchPoint.html#method.append) method.
  pub trait ListTouchEventTouchPointAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTouchEventTouchPoint) -> ();
  }
  impl<'largs> ListTouchEventTouchPointAppendArgs<'largs> for &'largs ::list::ListTouchEventTouchPoint {
    fn exec(self, original_self: &'largs mut ::list::ListTouchEventTouchPoint) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_append_QList_QTouchEvent_TouchPoint(original_self as *mut ::list::ListTouchEventTouchPoint, t as *const ::list::ListTouchEventTouchPoint) }
    }
  }
  impl<'largs> ListTouchEventTouchPointAppendArgs<'largs> for &'largs ::touch_event::TouchPoint {
    fn exec(self, original_self: &'largs mut ::list::ListTouchEventTouchPoint) -> () {
      let t = self;
      unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_append_QTouchEvent_TouchPoint(original_self as *mut ::list::ListTouchEventTouchPoint, t as *const ::touch_event::TouchPoint) }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTouchEventTouchPoint::mid](../struct.ListTouchEventTouchPoint.html#method.mid) method.
  pub trait ListTouchEventTouchPointMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListTouchEventTouchPoint) -> ::list::ListTouchEventTouchPoint;
  }
  impl<'largs> ListTouchEventTouchPointMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListTouchEventTouchPoint) -> ::list::ListTouchEventTouchPoint {
      let pos = self;
      {
        let mut object: ::list::ListTouchEventTouchPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_mid_to_output_pos(original_self as *const ::list::ListTouchEventTouchPoint, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListTouchEventTouchPointMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListTouchEventTouchPoint) -> ::list::ListTouchEventTouchPoint {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListTouchEventTouchPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_mid_to_output_pos_length(original_self as *const ::list::ListTouchEventTouchPoint, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTouchEventTouchPoint::new](../struct.ListTouchEventTouchPoint.html#method.new) method.
  pub trait ListTouchEventTouchPointNewArgs {
    fn exec(self) -> ::list::ListTouchEventTouchPoint;
  }
  impl<'a> ListTouchEventTouchPointNewArgs for &'a ::list::ListTouchEventTouchPoint {
    fn exec(self) -> ::list::ListTouchEventTouchPoint {
      let l = self;
      {
        let mut object: ::list::ListTouchEventTouchPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_constructor_l(l as *const ::list::ListTouchEventTouchPoint,
                                                                     &mut object);
        }
        object
      }
    }
  }
  impl ListTouchEventTouchPointNewArgs for () {
    fn exec(self) -> ::list::ListTouchEventTouchPoint {

      {
        let mut object: ::list::ListTouchEventTouchPoint =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListTouchEventTouchPoint::op_add_assign](../struct.ListTouchEventTouchPoint.html#method.op_add_assign) method.
  pub trait ListTouchEventTouchPointOpAddAssignArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListTouchEventTouchPoint)
            -> &'largs mut ::list::ListTouchEventTouchPoint;
  }
  impl<'largs> ListTouchEventTouchPointOpAddAssignArgs<'largs> for &'largs ::list::ListTouchEventTouchPoint {
    fn exec(self,
            original_self: &'largs mut ::list::ListTouchEventTouchPoint)
            -> &'largs mut ::list::ListTouchEventTouchPoint {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_operator_add_assign_l(original_self as *mut ::list::ListTouchEventTouchPoint, l as *const ::list::ListTouchEventTouchPoint) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTouchEventTouchPointOpAddAssignArgs<'largs> for &'largs ::touch_event::TouchPoint {
    fn exec(self,
            original_self: &'largs mut ::list::ListTouchEventTouchPoint)
            -> &'largs mut ::list::ListTouchEventTouchPoint {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_operator_add_assign_t(original_self as *mut ::list::ListTouchEventTouchPoint, t as *const ::touch_event::TouchPoint) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListTouchEventTouchPoint::op_shl](../struct.ListTouchEventTouchPoint.html#method.op_shl) method.
  pub trait ListTouchEventTouchPointOpShlArgs<'largs> {
    fn exec(self,
            original_self: &'largs mut ::list::ListTouchEventTouchPoint)
            -> &'largs mut ::list::ListTouchEventTouchPoint;
  }
  impl<'largs> ListTouchEventTouchPointOpShlArgs<'largs> for &'largs ::list::ListTouchEventTouchPoint {
    fn exec(self,
            original_self: &'largs mut ::list::ListTouchEventTouchPoint)
            -> &'largs mut ::list::ListTouchEventTouchPoint {
      let l = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_operator_shl_l(original_self as *mut ::list::ListTouchEventTouchPoint, l as *const ::list::ListTouchEventTouchPoint) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> ListTouchEventTouchPointOpShlArgs<'largs> for &'largs ::touch_event::TouchPoint {
    fn exec(self,
            original_self: &'largs mut ::list::ListTouchEventTouchPoint)
            -> &'largs mut ::list::ListTouchEventTouchPoint {
      let t = self;
      let ffi_result = unsafe { ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_operator_shl_t(original_self as *mut ::list::ListTouchEventTouchPoint, t as *const ::touch_event::TouchPoint) };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [ListTouchEventTouchPoint::swap](../struct.ListTouchEventTouchPoint.html#method.swap) method.
  pub trait ListTouchEventTouchPointSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListTouchEventTouchPoint) -> ();
  }
  impl<'largs> ListTouchEventTouchPointSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListTouchEventTouchPoint) -> () {
      let i = self.0;
      let j = self.1;
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_swap_i_j(original_self as *mut ::list::ListTouchEventTouchPoint,
                                                              i,
                                                              j)
      }
    }
  }
  impl<'largs> ListTouchEventTouchPointSwapArgs<'largs> for &'largs mut ::list::ListTouchEventTouchPoint {
    fn exec(self, original_self: &'largs mut ::list::ListTouchEventTouchPoint) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QTouchEvent_TouchPoint_swap_other(original_self as *mut ::list::ListTouchEventTouchPoint, other as *mut ::list::ListTouchEventTouchPoint)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWindowMutPtr::index_of](../struct.ListWindowMutPtr.html#method.index_of) method.
  pub trait ListWindowMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListWindowMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListWindowMutPtrIndexOfArgs<'largs> for &'largs *mut ::window::Window {
    unsafe fn exec(self, original_self: &'largs ::list::ListWindowMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QWindow_ptr_indexOf_t(original_self as *const ::list::ListWindowMutPtr,
                                                  t as *const *mut ::window::Window)
    }
  }
  impl<'largs> ListWindowMutPtrIndexOfArgs<'largs> for (&'largs *mut ::window::Window, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListWindowMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QWindow_ptr_indexOf_t_from(original_self as *const ::list::ListWindowMutPtr,
                                                       t as *const *mut ::window::Window,
                                                       from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListWindowMutPtr::last_index_of](../struct.ListWindowMutPtr.html#method.last_index_of) method.
  pub trait ListWindowMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::list::ListWindowMutPtr) -> ::libc::c_int;
  }
  impl<'largs> ListWindowMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::window::Window {
    unsafe fn exec(self, original_self: &'largs ::list::ListWindowMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_gui_c_QList_QWindow_ptr_lastIndexOf_t(original_self as *const ::list::ListWindowMutPtr,
                                                      t as *const *mut ::window::Window)
    }
  }
  impl<'largs> ListWindowMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::window::Window, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::list::ListWindowMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_gui_c_QList_QWindow_ptr_lastIndexOf_t_from(original_self as *const ::list::ListWindowMutPtr,
                                                           t as *const *mut ::window::Window,
                                                           from)
    }
  }
  /// This trait represents a set of arguments accepted by [ListWindowMutPtr::mid](../struct.ListWindowMutPtr.html#method.mid) method.
  pub trait ListWindowMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::list::ListWindowMutPtr) -> ::list::ListWindowMutPtr;
  }
  impl<'largs> ListWindowMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::list::ListWindowMutPtr) -> ::list::ListWindowMutPtr {
      let pos = self;
      {
        let mut object: ::list::ListWindowMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QWindow_ptr_mid_to_output_pos(original_self as *const ::list::ListWindowMutPtr,
                                                              pos,
                                                              &mut object);
        }
        object
      }
    }
  }
  impl<'largs> ListWindowMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::list::ListWindowMutPtr) -> ::list::ListWindowMutPtr {
      let pos = self.0;
      let length = self.1;
      {
        let mut object: ::list::ListWindowMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QWindow_ptr_mid_to_output_pos_length(original_self as *const ::list::ListWindowMutPtr, pos, length, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWindowMutPtr::new](../struct.ListWindowMutPtr.html#method.new) method.
  pub trait ListWindowMutPtrNewArgs {
    fn exec(self) -> ::list::ListWindowMutPtr;
  }
  impl<'a> ListWindowMutPtrNewArgs for &'a ::list::ListWindowMutPtr {
    fn exec(self) -> ::list::ListWindowMutPtr {
      let l = self;
      {
        let mut object: ::list::ListWindowMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QWindow_ptr_constructor_l(l as *const ::list::ListWindowMutPtr, &mut object);
        }
        object
      }
    }
  }
  impl ListWindowMutPtrNewArgs for () {
    fn exec(self) -> ::list::ListWindowMutPtr {

      {
        let mut object: ::list::ListWindowMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_gui_c_QList_QWindow_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [ListWindowMutPtr::swap](../struct.ListWindowMutPtr.html#method.swap) method.
  pub trait ListWindowMutPtrSwapArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::list::ListWindowMutPtr) -> ();
  }
  impl<'largs> ListWindowMutPtrSwapArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::list::ListWindowMutPtr) -> () {
      let i = self.0;
      let j = self.1;
      unsafe { ::ffi::qt_gui_c_QList_QWindow_ptr_swap_i_j(original_self as *mut ::list::ListWindowMutPtr, i, j) }
    }
  }
  impl<'largs> ListWindowMutPtrSwapArgs<'largs> for &'largs mut ::list::ListWindowMutPtr {
    fn exec(self, original_self: &'largs mut ::list::ListWindowMutPtr) -> () {
      let other = self;
      unsafe {
        ::ffi::qt_gui_c_QList_QWindow_ptr_swap_other(original_self as *mut ::list::ListWindowMutPtr,
                                                     other as *mut ::list::ListWindowMutPtr)
      }
    }
  }
}
