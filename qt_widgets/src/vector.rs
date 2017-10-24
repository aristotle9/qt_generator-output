/// C++ type: <span style='color: green;'>```QVector<void*>```</span>
#[repr(C)]
pub struct VectorCVoidMutPtr([u8; ::type_sizes::QT_WIDGETS_VECTOR_VECTOR_C_VOID_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorCVoidMutPtr {
  unsafe fn new_uninitialized() -> VectorCVoidMutPtr {
    VectorCVoidMutPtr(::std::mem::uninitialized())
  }
}

impl VectorCVoidMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<void*>::append(const QVector<void*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorCVoidMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QVector_void_ptr_append_l(self as *mut ::vector::VectorCVoidMutPtr,
                                                    l as *const ::vector::VectorCVoidMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::append(void* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::libc::c_void) {
    ::ffi::qt_widgets_c_QVector_void_ptr_append_t(self as *mut ::vector::VectorCVoidMutPtr,
                                                  t as *const *mut ::libc::c_void)
  }

  /// C++ method: <span style='color: green;'>```void* const & QVector<void*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::libc::c_void {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_at(self as *const ::vector::VectorCVoidMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void* const & QVector<void*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::libc::c_void {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_back_const(self as *const ::vector::VectorCVoidMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void*& QVector<void*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::libc::c_void {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_back(self as *mut ::vector::VectorCVoidMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<void*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_capacity(self as *const ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_clear(self as *mut ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void* const * QVector<void*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::libc::c_void {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_constData(self as *const ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void* const & QVector<void*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::libc::c_void {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_constFirst(self as *const ::vector::VectorCVoidMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void* const & QVector<void*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::libc::c_void {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_constLast(self as *const ::vector::VectorCVoidMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<void*>::contains(void* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::libc::c_void) -> bool {
    ::ffi::qt_widgets_c_QVector_void_ptr_contains(self as *const ::vector::VectorCVoidMutPtr,
                                                  t as *const *mut ::libc::c_void)
  }

  /// C++ method: <span style='color: green;'>```int QVector<void*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_count_no_args(self as *const ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<void*>::count(void* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::libc::c_void) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QVector_void_ptr_count_t(self as *const ::vector::VectorCVoidMutPtr,
                                                 t as *const *mut ::libc::c_void)
  }

  /// C++ method: <span style='color: green;'>```void* const * QVector<void*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::libc::c_void {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_data_const(self as *const ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void** QVector<void*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::libc::c_void {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_data(self as *mut ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<void*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_empty(self as *const ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<void*>::endsWith(void* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::libc::c_void) -> bool {
    ::ffi::qt_widgets_c_QVector_void_ptr_endsWith(self as *const ::vector::VectorCVoidMutPtr,
                                                  t as *const *mut ::libc::c_void)
  }

  /// C++ method: <span style='color: green;'>```QVector<void*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::libc::c_void) -> &'l0 mut ::vector::VectorCVoidMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<void*>& QVector<void*>::fill(void* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::libc::c_void, ::libc::c_int)) -> &'l0 mut ::vector::VectorCVoidMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<void*>& QVector<void*>::fill(void* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorCVoidMutPtr
    where Args: overloading::VectorCVoidMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void* const & QVector<void*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::libc::c_void {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_first_const(self as *const ::vector::VectorCVoidMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void*& QVector<void*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::libc::c_void {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_first(self as *mut ::vector::VectorCVoidMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void* const & QVector<void*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::libc::c_void {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_front_const(self as *const ::vector::VectorCVoidMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void*& QVector<void*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::libc::c_void {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_front(self as *mut ::vector::VectorCVoidMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<void*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::libc::c_void) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<void*>::indexOf(void* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::libc::c_void, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<void*>::indexOf(void* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCVoidMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<void*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<void*>::insert(int i, int n, void* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::libc::c_void)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<void*>::insert(int i, void* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCVoidMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<void*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_isEmpty(self as *const ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void* const & QVector<void*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::libc::c_void {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_last_const(self as *const ::vector::VectorCVoidMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<void*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::libc::c_void) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<void*>::lastIndexOf(void* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::libc::c_void, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<void*>::lastIndexOf(void* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorCVoidMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void*& QVector<void*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::libc::c_void {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_last(self as *mut ::vector::VectorCVoidMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<void*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_length(self as *const ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<void*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorCVoidMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<void*> QVector<void*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorCVoidMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<void*> QVector<void*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorCVoidMutPtr
    where Args: overloading::VectorCVoidMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<void*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_move(self as *mut ::vector::VectorCVoidMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<void*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorCVoidMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<void*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorCVoidMutPtr) -> ::vector::VectorCVoidMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<void*>::QVector(const QVector<void*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorCVoidMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<void*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorCVoidMutPtr
    where Args: overloading::VectorCVoidMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<void*>::QVector(int size, void* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int, t: &*mut ::libc::c_void) -> ::vector::VectorCVoidMutPtr {
    {
      let mut object: ::vector::VectorCVoidMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_widgets_c_QVector_void_ptr_constructor_size_t(size, t as *const *mut ::libc::c_void, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<void*> QVector<void*>::operator+(const QVector<void*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorCVoidMutPtr) -> ::vector::VectorCVoidMutPtr {
    {
      let mut object: ::vector::VectorCVoidMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QVector_void_ptr_operator_add_to_output(self as *const ::vector::VectorCVoidMutPtr,
                                                                    l as *const ::vector::VectorCVoidMutPtr,
                                                                    &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<void*>& QVector<void*>::operator+=(const QVector<void*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorCVoidMutPtr)
                                 -> &'l0 mut ::vector::VectorCVoidMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QVector_void_ptr_operator_add_assign_l(self as *mut ::vector::VectorCVoidMutPtr,
                                                                   l as *const ::vector::VectorCVoidMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<void*>& QVector<void*>::operator+=(void* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::libc::c_void)
                                               -> &'l0 mut ::vector::VectorCVoidMutPtr {
    let ffi_result =
      ::ffi::qt_widgets_c_QVector_void_ptr_operator_add_assign_t(self as *mut ::vector::VectorCVoidMutPtr,
                                                                 t as *const *mut ::libc::c_void);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<void*>& QVector<void*>::operator=(const QVector<void*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorCVoidMutPtr)
                             -> &'l0 mut ::vector::VectorCVoidMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QVector_void_ptr_operator_assign(self as *mut ::vector::VectorCVoidMutPtr,
                                                           v as *const ::vector::VectorCVoidMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<void*>::operator==(const QVector<void*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorCVoidMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QVector_void_ptr_operator_eq(self as *const ::vector::VectorCVoidMutPtr,
                                                       v as *const ::vector::VectorCVoidMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void* const & QVector<void*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::libc::c_void {
    let ffi_result =
      unsafe {
        ::ffi::qt_widgets_c_QVector_void_ptr_operator_index_const(self as *const ::vector::VectorCVoidMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void*& QVector<void*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::libc::c_void {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_operator_index(self as *mut ::vector::VectorCVoidMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<void*>::operator!=(const QVector<void*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorCVoidMutPtr) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QVector_void_ptr_operator_neq(self as *const ::vector::VectorCVoidMutPtr,
                                                        v as *const ::vector::VectorCVoidMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<void*>& QVector<void*>::operator<<(const QVector<void*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::vector::VectorCVoidMutPtr) -> &'l0 mut ::vector::VectorCVoidMutPtr {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QVector_void_ptr_operator_shl_l(self as *mut ::vector::VectorCVoidMutPtr,
                                                          l as *const ::vector::VectorCVoidMutPtr)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<void*>& QVector<void*>::operator<<(void* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::libc::c_void)
                                        -> &'l0 mut ::vector::VectorCVoidMutPtr {
    let ffi_result = ::ffi::qt_widgets_c_QVector_void_ptr_operator_shl_t(self as *mut ::vector::VectorCVoidMutPtr,
                                                                         t as *const *mut ::libc::c_void);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_pop_back(self as *mut ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_pop_front(self as *mut ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::prepend(void* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::libc::c_void) {
    ::ffi::qt_widgets_c_QVector_void_ptr_prepend(self as *mut ::vector::VectorCVoidMutPtr,
                                                 t as *const *mut ::libc::c_void)
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::push_back(void* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::libc::c_void) {
    ::ffi::qt_widgets_c_QVector_void_ptr_push_back(self as *mut ::vector::VectorCVoidMutPtr,
                                                   t as *const *mut ::libc::c_void)
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::push_front(void* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::libc::c_void) {
    ::ffi::qt_widgets_c_QVector_void_ptr_push_front(self as *mut ::vector::VectorCVoidMutPtr,
                                                    t as *const *mut ::libc::c_void)
  }

  /// C++ method: <span style='color: green;'>```QVector<void*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<void*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<void*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorCVoidMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<void*>::removeAll(void* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::libc::c_void) -> ::libc::c_int {
    ::ffi::qt_widgets_c_QVector_void_ptr_removeAll(self as *mut ::vector::VectorCVoidMutPtr,
                                                   t as *const *mut ::libc::c_void)
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_removeAt(self as *mut ::vector::VectorCVoidMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_removeFirst(self as *mut ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_removeLast(self as *mut ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<void*>::removeOne(void* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::libc::c_void) -> bool {
    ::ffi::qt_widgets_c_QVector_void_ptr_removeOne(self as *mut ::vector::VectorCVoidMutPtr,
                                                   t as *const *mut ::libc::c_void)
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::replace(int i, void* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::libc::c_void) {
    ::ffi::qt_widgets_c_QVector_void_ptr_replace(self as *mut ::vector::VectorCVoidMutPtr,
                                                 i,
                                                 t as *const *mut ::libc::c_void)
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_reserve(self as *mut ::vector::VectorCVoidMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_resize(self as *mut ::vector::VectorCVoidMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<void*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_size(self as *const ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_squeeze(self as *mut ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<void*>::startsWith(void* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::libc::c_void) -> bool {
    ::ffi::qt_widgets_c_QVector_void_ptr_startsWith(self as *const ::vector::VectorCVoidMutPtr,
                                                    t as *const *mut ::libc::c_void)
  }

  /// C++ method: <span style='color: green;'>```void QVector<void*>::swap(QVector<void*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorCVoidMutPtr) {
    unsafe {
      ::ffi::qt_widgets_c_QVector_void_ptr_swap(self as *mut ::vector::VectorCVoidMutPtr,
                                                other as *mut ::vector::VectorCVoidMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void* QVector<void*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_takeAt(self as *mut ::vector::VectorCVoidMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void* QVector<void*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_takeFirst(self as *mut ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void* QVector<void*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_takeLast(self as *mut ::vector::VectorCVoidMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void* QVector<void*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::libc::c_void {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_value_i(self as *const ::vector::VectorCVoidMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void* QVector<void*>::value(int i, void* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self, i: ::libc::c_int, default_value: &*mut ::libc::c_void) -> *mut ::libc::c_void {
    ::ffi::qt_widgets_c_QVector_void_ptr_value_i_defaultValue(self as *const ::vector::VectorCVoidMutPtr,
                                                              i,
                                                              default_value as *const *mut ::libc::c_void)
  }
}

impl Drop for ::vector::VectorCVoidMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<void*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_destructor(self as *mut ::vector::VectorCVoidMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<QColor>```</span>
#[repr(C)]
pub struct VectorQtGuiColor([u8; ::type_sizes::QT_WIDGETS_VECTOR_VECTOR_QT_GUI_COLOR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorQtGuiColor {
  unsafe fn new_uninitialized() -> VectorQtGuiColor {
    VectorQtGuiColor(::std::mem::uninitialized())
  }
}

impl VectorQtGuiColor {
  /// C++ method: <span style='color: green;'>```QVector<QColor>::append```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn append(&mut self, &::qt_gui::color::Color) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QColor>::append(const QColor& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn append(&mut self, &::vector::VectorQtGuiColor) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QColor>::append(const QVector<QColor>& l)```</span>
  ///
  ///
  pub fn append<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtGuiColorAppendArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QColor& QVector<QColor>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_gui::color::Color {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVector_QColor_at(self as *const ::vector::VectorQtGuiColor, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QColor& QVector<QColor>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 ::qt_gui::color::Color {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_QColor_back_const(self as *const ::vector::VectorQtGuiColor) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QColor& QVector<QColor>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::color::Color {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVector_QColor_back(self as *mut ::vector::VectorQtGuiColor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QColor>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_capacity(self as *const ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_clear(self as *mut ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```const QColor* QVector<QColor>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const ::qt_gui::color::Color {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_constData(self as *const ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```const QColor& QVector<QColor>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 ::qt_gui::color::Color {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_QColor_constFirst(self as *const ::vector::VectorQtGuiColor) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QColor& QVector<QColor>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 ::qt_gui::color::Color {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVector_QColor_constLast(self as *const ::vector::VectorQtGuiColor) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QColor>::contains(const QColor& t) const```</span>
  ///
  ///
  pub fn contains(&self, t: &::qt_gui::color::Color) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_contains(self as *const ::vector::VectorQtGuiColor,
                                                  t as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QColor>::count```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn count(&self, ()) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QColor>::count() const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn count(&self, &::qt_gui::color::Color) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QColor>::count(const QColor& t) const```</span>
  ///
  ///
  pub fn count<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtGuiColorCountArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QColor* QVector<QColor>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const ::qt_gui::color::Color {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_data_const(self as *const ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```QColor* QVector<QColor>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut ::qt_gui::color::Color {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_data(self as *mut ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QColor>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_empty(self as *const ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QColor>::endsWith(const QColor& t) const```</span>
  ///
  ///
  pub fn ends_with(&self, t: &::qt_gui::color::Color) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_endsWith(self as *const ::vector::VectorQtGuiColor,
                                                  t as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QColor>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 ::qt_gui::color::Color) -> &'l0 mut ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QColor>& QVector<QColor>::fill(const QColor& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 ::qt_gui::color::Color, ::libc::c_int)) -> &'l0 mut ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QColor>& QVector<QColor>::fill(const QColor& t, int size = ?)```</span>
  ///
  ///
  pub fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtGuiColor
    where Args: overloading::VectorQtGuiColorFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```const QColor& QVector<QColor>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 ::qt_gui::color::Color {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_QColor_first_const(self as *const ::vector::VectorQtGuiColor) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QColor& QVector<QColor>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::color::Color {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVector_QColor_first(self as *mut ::vector::VectorQtGuiColor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```const QColor& QVector<QColor>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 ::qt_gui::color::Color {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_QColor_front_const(self as *const ::vector::VectorQtGuiColor) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QColor& QVector<QColor>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::color::Color {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVector_QColor_front(self as *mut ::vector::VectorQtGuiColor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QColor>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &::qt_gui::color::Color) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QColor>::indexOf(const QColor& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&::qt_gui::color::Color, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QColor>::indexOf(const QColor& t, int from = ?) const```</span>
  ///
  ///
  pub fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtGuiColorIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QColor>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &::qt_gui::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QColor>::insert(int i, const QColor& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &::qt_gui::color::Color)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QColor>::insert(int i, int n, const QColor& t)```</span>
  ///
  ///
  pub fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtGuiColorInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<QColor>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_isEmpty(self as *const ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```const QColor& QVector<QColor>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 ::qt_gui::color::Color {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_QColor_last_const(self as *const ::vector::VectorQtGuiColor) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<QColor>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &::qt_gui::color::Color) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QColor>::lastIndexOf(const QColor& t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&::qt_gui::color::Color, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<QColor>::lastIndexOf(const QColor& t, int from = ?) const```</span>
  ///
  ///
  pub fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorQtGuiColorLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QColor& QVector<QColor>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut ::qt_gui::color::Color {
    let ffi_result = unsafe { ::ffi::qt_widgets_c_QVector_QColor_last(self as *mut ::vector::VectorQtGuiColor) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<QColor>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_length(self as *const ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QColor>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QColor> QVector<QColor>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QColor> QVector<QColor>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorQtGuiColor
    where Args: overloading::VectorQtGuiColorMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QColor>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_move(self as *mut ::vector::VectorQtGuiColor, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<QColor>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QColor>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorQtGuiColor) -> ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QColor>::QVector(const QVector<QColor>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QColor>::QVector(int size)```</span>
  ///
  ///
  ///
  /// ## Variant 4
  ///
  /// Rust arguments: ```fn new((::libc::c_int, &::qt_gui::color::Color)) -> ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<QColor>::QVector(int size, const QColor& t)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorQtGuiColor
    where Args: overloading::VectorQtGuiColorNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```QVector<QColor> QVector<QColor>::operator+(const QVector<QColor>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorQtGuiColor) -> ::vector::VectorQtGuiColor {
    {
      let mut object: ::vector::VectorQtGuiColor =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_operator_add_to_output(self as *const ::vector::VectorQtGuiColor,
                                                                  l as *const ::vector::VectorQtGuiColor,
                                                                  &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QColor>::operator+=```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::qt_gui::color::Color) -> &'l0 mut ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QColor>& QVector<QColor>::operator+=(const QColor& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_add_assign(&mut self, &'l1 ::vector::VectorQtGuiColor) -> &'l0 mut ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QColor>& QVector<QColor>::operator+=(const QVector<QColor>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtGuiColor
    where Args: overloading::VectorQtGuiColorOpAddAssignArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<QColor>& QVector<QColor>::operator=(const QVector<QColor>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorQtGuiColor) -> &'l0 mut ::vector::VectorQtGuiColor {
    let ffi_result = unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_operator_assign(self as *mut ::vector::VectorQtGuiColor,
                                                         v as *const ::vector::VectorQtGuiColor)
    };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QColor>::operator==(const QVector<QColor>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorQtGuiColor) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_operator_eq(self as *const ::vector::VectorQtGuiColor,
                                                     v as *const ::vector::VectorQtGuiColor)
    }
  }

  /// C++ method: <span style='color: green;'>```const QColor& QVector<QColor>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 ::qt_gui::color::Color {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_QColor_operator_index_const(self as *const ::vector::VectorQtGuiColor, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QColor& QVector<QColor>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut ::qt_gui::color::Color {
    let ffi_result =
      unsafe { ::ffi::qt_widgets_c_QVector_QColor_operator_index(self as *mut ::vector::VectorQtGuiColor, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QColor>::operator!=(const QVector<QColor>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorQtGuiColor) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_operator_neq(self as *const ::vector::VectorQtGuiColor,
                                                      v as *const ::vector::VectorQtGuiColor)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QColor>::operator<<```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::qt_gui::color::Color) -> &'l0 mut ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QColor>& QVector<QColor>::operator<<(const QColor& t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn op_shl(&mut self, &'l1 ::vector::VectorQtGuiColor) -> &'l0 mut ::vector::VectorQtGuiColor```<br>
  /// C++ method: <span style='color: green;'>```QVector<QColor>& QVector<QColor>::operator<<(const QVector<QColor>& l)```</span>
  ///
  ///
  pub fn op_shl<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorQtGuiColor
    where Args: overloading::VectorQtGuiColorOpShlArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<QColor>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_pop_back(self as *mut ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_pop_front(self as *mut ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::prepend(const QColor& t)```</span>
  ///
  ///
  pub fn prepend(&mut self, t: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_prepend(self as *mut ::vector::VectorQtGuiColor,
                                                 t as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::push_back(const QColor& t)```</span>
  ///
  ///
  pub fn push_back(&mut self, t: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_push_back(self as *mut ::vector::VectorQtGuiColor,
                                                   t as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::push_front(const QColor& t)```</span>
  ///
  ///
  pub fn push_front(&mut self, t: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_push_front(self as *mut ::vector::VectorQtGuiColor,
                                                    t as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QColor>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QColor>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<QColor>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorQtGuiColorRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<QColor>::removeAll(const QColor& t)```</span>
  ///
  ///
  pub fn remove_all(&mut self, t: &::qt_gui::color::Color) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_removeAll(self as *mut ::vector::VectorQtGuiColor,
                                                   t as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_removeAt(self as *mut ::vector::VectorQtGuiColor, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_removeFirst(self as *mut ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_removeLast(self as *mut ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QColor>::removeOne(const QColor& t)```</span>
  ///
  ///
  pub fn remove_one(&mut self, t: &::qt_gui::color::Color) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_removeOne(self as *mut ::vector::VectorQtGuiColor,
                                                   t as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::replace(int i, const QColor& t)```</span>
  ///
  ///
  pub fn replace(&mut self, i: ::libc::c_int, t: &::qt_gui::color::Color) {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_replace(self as *mut ::vector::VectorQtGuiColor,
                                                 i,
                                                 t as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_reserve(self as *mut ::vector::VectorQtGuiColor, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_resize(self as *mut ::vector::VectorQtGuiColor, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<QColor>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_size(self as *const ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_squeeze(self as *mut ::vector::VectorQtGuiColor) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<QColor>::startsWith(const QColor& t) const```</span>
  ///
  ///
  pub fn starts_with(&self, t: &::qt_gui::color::Color) -> bool {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_startsWith(self as *const ::vector::VectorQtGuiColor,
                                                    t as *const ::qt_gui::color::Color)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<QColor>::swap(QVector<QColor>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorQtGuiColor) {
    unsafe {
      ::ffi::qt_widgets_c_QVector_QColor_swap(self as *mut ::vector::VectorQtGuiColor,
                                              other as *mut ::vector::VectorQtGuiColor)
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QVector<QColor>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_takeAt_to_output(self as *mut ::vector::VectorQtGuiColor, i, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QVector<QColor>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_takeFirst_to_output(self as *mut ::vector::VectorQtGuiColor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QColor QVector<QColor>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> ::qt_gui::color::Color {
    {
      let mut object: ::qt_gui::color::Color =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_takeLast_to_output(self as *mut ::vector::VectorQtGuiColor, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<QColor>::value```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn value(&self, ::libc::c_int) -> ::qt_gui::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QVector<QColor>::value(int i) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn value(&self, (::libc::c_int, &::qt_gui::color::Color)) -> ::qt_gui::color::Color```<br>
  /// C++ method: <span style='color: green;'>```QColor QVector<QColor>::value(int i, const QColor& defaultValue) const```</span>
  ///
  ///
  pub fn value<'largs, Args>(&'largs self, args: Args) -> ::qt_gui::color::Color
    where Args: overloading::VectorQtGuiColorValueArgs<'largs>
  {
    args.exec(self)
  }
}

impl Drop for ::vector::VectorQtGuiColor {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<QColor>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_widgets_c_QVector_QColor_destructor(self as *mut ::vector::VectorQtGuiColor) }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [VectorCVoidMutPtr::fill](../struct.VectorCVoidMutPtr.html#method.fill) method.
  pub trait VectorCVoidMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorCVoidMutPtr)
                   -> &'largs mut ::vector::VectorCVoidMutPtr;
  }
  impl<'largs> VectorCVoidMutPtrFillArgs<'largs> for &'largs *mut ::libc::c_void {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorCVoidMutPtr)
                   -> &'largs mut ::vector::VectorCVoidMutPtr {
      let t = self;
      let ffi_result = ::ffi::qt_widgets_c_QVector_void_ptr_fill_t(original_self as *mut ::vector::VectorCVoidMutPtr,
                                                                   t as *const *mut ::libc::c_void);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorCVoidMutPtrFillArgs<'largs> for (&'largs *mut ::libc::c_void, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorCVoidMutPtr)
                   -> &'largs mut ::vector::VectorCVoidMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result =
        ::ffi::qt_widgets_c_QVector_void_ptr_fill_t_size(original_self as *mut ::vector::VectorCVoidMutPtr,
                                                         t as *const *mut ::libc::c_void,
                                                         size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCVoidMutPtr::index_of](../struct.VectorCVoidMutPtr.html#method.index_of) method.
  pub trait VectorCVoidMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorCVoidMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorCVoidMutPtrIndexOfArgs<'largs> for &'largs *mut ::libc::c_void {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorCVoidMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QVector_void_ptr_indexOf_t(original_self as *const ::vector::VectorCVoidMutPtr,
                                                     t as *const *mut ::libc::c_void)
    }
  }
  impl<'largs> VectorCVoidMutPtrIndexOfArgs<'largs> for (&'largs *mut ::libc::c_void, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorCVoidMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QVector_void_ptr_indexOf_t_from(original_self as *const ::vector::VectorCVoidMutPtr,
                                                          t as *const *mut ::libc::c_void,
                                                          from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCVoidMutPtr::insert](../struct.VectorCVoidMutPtr.html#method.insert) method.
  pub trait VectorCVoidMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorCVoidMutPtr) -> ();
  }
  impl<'largs> VectorCVoidMutPtrInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs *mut ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorCVoidMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_widgets_c_QVector_void_ptr_insert_i_n_t(original_self as *mut ::vector::VectorCVoidMutPtr,
                                                        i,
                                                        n,
                                                        t as *const *mut ::libc::c_void)
    }
  }
  impl<'largs> VectorCVoidMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::libc::c_void) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorCVoidMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_widgets_c_QVector_void_ptr_insert_i_t(original_self as *mut ::vector::VectorCVoidMutPtr,
                                                      i,
                                                      t as *const *mut ::libc::c_void)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCVoidMutPtr::last_index_of](../struct.VectorCVoidMutPtr.html#method.last_index_of) method.
  pub trait VectorCVoidMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorCVoidMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorCVoidMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::libc::c_void {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorCVoidMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_widgets_c_QVector_void_ptr_lastIndexOf_t(original_self as *const ::vector::VectorCVoidMutPtr,
                                                         t as *const *mut ::libc::c_void)
    }
  }
  impl<'largs> VectorCVoidMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::libc::c_void, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorCVoidMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_widgets_c_QVector_void_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorCVoidMutPtr,
                                                              t as *const *mut ::libc::c_void,
                                                              from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCVoidMutPtr::mid](../struct.VectorCVoidMutPtr.html#method.mid) method.
  pub trait VectorCVoidMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorCVoidMutPtr) -> ::vector::VectorCVoidMutPtr;
  }
  impl<'largs> VectorCVoidMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorCVoidMutPtr) -> ::vector::VectorCVoidMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorCVoidMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_void_ptr_mid_to_output_pos(original_self as *const ::vector::VectorCVoidMutPtr,
                                                                 pos,
                                                                 &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorCVoidMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorCVoidMutPtr) -> ::vector::VectorCVoidMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorCVoidMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_void_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorCVoidMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCVoidMutPtr::new](../struct.VectorCVoidMutPtr.html#method.new) method.
  pub trait VectorCVoidMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorCVoidMutPtr;
  }
  impl VectorCVoidMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorCVoidMutPtr {

      {
        let mut object: ::vector::VectorCVoidMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_void_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorCVoidMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorCVoidMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorCVoidMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_void_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorCVoidMutPtrNewArgs for &'a ::vector::VectorCVoidMutPtr {
    fn exec(self) -> ::vector::VectorCVoidMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorCVoidMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_void_ptr_constructor_v(v as *const ::vector::VectorCVoidMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorCVoidMutPtr::remove](../struct.VectorCVoidMutPtr.html#method.remove) method.
  pub trait VectorCVoidMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorCVoidMutPtr) -> ();
  }
  impl<'largs> VectorCVoidMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorCVoidMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_widgets_c_QVector_void_ptr_remove_i(original_self as *mut ::vector::VectorCVoidMutPtr, i) }
    }
  }
  impl<'largs> VectorCVoidMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorCVoidMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QVector_void_ptr_remove_i_n(original_self as *mut ::vector::VectorCVoidMutPtr, i, n)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::append](../struct.VectorQtGuiColor.html#method.append) method.
  pub trait VectorQtGuiColorAppendArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> ();
  }
  impl<'largs> VectorQtGuiColorAppendArgs<'largs> for &'largs ::vector::VectorQtGuiColor {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> () {
      let l = self;
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_append_l(original_self as *mut ::vector::VectorQtGuiColor,
                                                    l as *const ::vector::VectorQtGuiColor)
      }
    }
  }
  impl<'largs> VectorQtGuiColorAppendArgs<'largs> for &'largs ::qt_gui::color::Color {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> () {
      let t = self;
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_append_t(original_self as *mut ::vector::VectorQtGuiColor,
                                                    t as *const ::qt_gui::color::Color)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::count](../struct.VectorQtGuiColor.html#method.count) method.
  pub trait VectorQtGuiColorCountArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::libc::c_int;
  }
  impl<'largs> VectorQtGuiColorCountArgs<'largs> for () {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::libc::c_int {

      unsafe { ::ffi::qt_widgets_c_QVector_QColor_count_no_args(original_self as *const ::vector::VectorQtGuiColor) }
    }
  }
  impl<'largs> VectorQtGuiColorCountArgs<'largs> for &'largs ::qt_gui::color::Color {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_count_t(original_self as *const ::vector::VectorQtGuiColor,
                                                   t as *const ::qt_gui::color::Color)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::fill](../struct.VectorQtGuiColor.html#method.fill) method.
  pub trait VectorQtGuiColorFillArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> &'largs mut ::vector::VectorQtGuiColor;
  }
  impl<'largs> VectorQtGuiColorFillArgs<'largs> for &'largs ::qt_gui::color::Color {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> &'largs mut ::vector::VectorQtGuiColor {
      let t = self;
      let ffi_result = unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_fill_t(original_self as *mut ::vector::VectorQtGuiColor,
                                                  t as *const ::qt_gui::color::Color)
      };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtGuiColorFillArgs<'largs> for (&'largs ::qt_gui::color::Color, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> &'largs mut ::vector::VectorQtGuiColor {
      let t = self.0;
      let size = self.1;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_fill_t_size(original_self as *mut ::vector::VectorQtGuiColor,
                                                         t as *const ::qt_gui::color::Color,
                                                         size)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::index_of](../struct.VectorQtGuiColor.html#method.index_of) method.
  pub trait VectorQtGuiColorIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::libc::c_int;
  }
  impl<'largs> VectorQtGuiColorIndexOfArgs<'largs> for &'largs ::qt_gui::color::Color {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_indexOf_t(original_self as *const ::vector::VectorQtGuiColor,
                                                     t as *const ::qt_gui::color::Color)
      }
    }
  }
  impl<'largs> VectorQtGuiColorIndexOfArgs<'largs> for (&'largs ::qt_gui::color::Color, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_indexOf_t_from(original_self as *const ::vector::VectorQtGuiColor,
                                                          t as *const ::qt_gui::color::Color,
                                                          from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::insert](../struct.VectorQtGuiColor.html#method.insert) method.
  pub trait VectorQtGuiColorInsertArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> ();
  }
  impl<'largs> VectorQtGuiColorInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs ::qt_gui::color::Color) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_insert_i_n_t(original_self as *mut ::vector::VectorQtGuiColor,
                                                        i,
                                                        n,
                                                        t as *const ::qt_gui::color::Color)
      }
    }
  }
  impl<'largs> VectorQtGuiColorInsertArgs<'largs> for (::libc::c_int, &'largs ::qt_gui::color::Color) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> () {
      let i = self.0;
      let t = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_insert_i_t(original_self as *mut ::vector::VectorQtGuiColor,
                                                      i,
                                                      t as *const ::qt_gui::color::Color)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::last_index_of](../struct.VectorQtGuiColor.html#method.last_index_of) method.
  pub trait VectorQtGuiColorLastIndexOfArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::libc::c_int;
  }
  impl<'largs> VectorQtGuiColorLastIndexOfArgs<'largs> for &'largs ::qt_gui::color::Color {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::libc::c_int {
      let t = self;
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_lastIndexOf_t(original_self as *const ::vector::VectorQtGuiColor,
                                                         t as *const ::qt_gui::color::Color)
      }
    }
  }
  impl<'largs> VectorQtGuiColorLastIndexOfArgs<'largs> for (&'largs ::qt_gui::color::Color, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      unsafe {
        ::ffi::qt_widgets_c_QVector_QColor_lastIndexOf_t_from(original_self as *const ::vector::VectorQtGuiColor,
                                                              t as *const ::qt_gui::color::Color,
                                                              from)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::mid](../struct.VectorQtGuiColor.html#method.mid) method.
  pub trait VectorQtGuiColorMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::vector::VectorQtGuiColor;
  }
  impl<'largs> VectorQtGuiColorMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::vector::VectorQtGuiColor {
      let pos = self;
      {
        let mut object: ::vector::VectorQtGuiColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_mid_to_output_pos(original_self as *const ::vector::VectorQtGuiColor,
                                                               pos,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtGuiColorMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::vector::VectorQtGuiColor {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorQtGuiColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_mid_to_output_pos_len(original_self as *const ::vector::VectorQtGuiColor, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::new](../struct.VectorQtGuiColor.html#method.new) method.
  pub trait VectorQtGuiColorNewArgs {
    fn exec(self) -> ::vector::VectorQtGuiColor;
  }
  impl VectorQtGuiColorNewArgs for () {
    fn exec(self) -> ::vector::VectorQtGuiColor {

      {
        let mut object: ::vector::VectorQtGuiColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorQtGuiColorNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorQtGuiColor {
      let size = self;
      {
        let mut object: ::vector::VectorQtGuiColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtGuiColorNewArgs for (::libc::c_int, &'a ::qt_gui::color::Color) {
    fn exec(self) -> ::vector::VectorQtGuiColor {
      let size = self.0;
      let t = self.1;
      {
        let mut object: ::vector::VectorQtGuiColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_constructor_size_t(size, t as *const ::qt_gui::color::Color, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorQtGuiColorNewArgs for &'a ::vector::VectorQtGuiColor {
    fn exec(self) -> ::vector::VectorQtGuiColor {
      let v = self;
      {
        let mut object: ::vector::VectorQtGuiColor =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_constructor_v(v as *const ::vector::VectorQtGuiColor, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::op_add_assign](../struct.VectorQtGuiColor.html#method.op_add_assign) method.
  pub trait VectorQtGuiColorOpAddAssignArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> &'largs mut ::vector::VectorQtGuiColor;
  }
  impl<'largs> VectorQtGuiColorOpAddAssignArgs<'largs> for &'largs ::vector::VectorQtGuiColor {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> &'largs mut ::vector::VectorQtGuiColor {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_operator_add_assign_l(original_self as *mut ::vector::VectorQtGuiColor,
                                                                   l as *const ::vector::VectorQtGuiColor)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtGuiColorOpAddAssignArgs<'largs> for &'largs ::qt_gui::color::Color {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> &'largs mut ::vector::VectorQtGuiColor {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_operator_add_assign_t(original_self as *mut ::vector::VectorQtGuiColor,
                                                                   t as *const ::qt_gui::color::Color)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::op_shl](../struct.VectorQtGuiColor.html#method.op_shl) method.
  pub trait VectorQtGuiColorOpShlArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> &'largs mut ::vector::VectorQtGuiColor;
  }
  impl<'largs> VectorQtGuiColorOpShlArgs<'largs> for &'largs ::vector::VectorQtGuiColor {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> &'largs mut ::vector::VectorQtGuiColor {
      let l = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_operator_shl_l(original_self as *mut ::vector::VectorQtGuiColor,
                                                            l as *const ::vector::VectorQtGuiColor)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorQtGuiColorOpShlArgs<'largs> for &'largs ::qt_gui::color::Color {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> &'largs mut ::vector::VectorQtGuiColor {
      let t = self;
      let ffi_result =
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_operator_shl_t(original_self as *mut ::vector::VectorQtGuiColor,
                                                            t as *const ::qt_gui::color::Color)
        };
      unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::remove](../struct.VectorQtGuiColor.html#method.remove) method.
  pub trait VectorQtGuiColorRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> ();
  }
  impl<'largs> VectorQtGuiColorRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> () {
      let i = self;
      unsafe { ::ffi::qt_widgets_c_QVector_QColor_remove_i(original_self as *mut ::vector::VectorQtGuiColor, i) }
    }
  }
  impl<'largs> VectorQtGuiColorRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorQtGuiColor) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_widgets_c_QVector_QColor_remove_i_n(original_self as *mut ::vector::VectorQtGuiColor, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorQtGuiColor::value](../struct.VectorQtGuiColor.html#method.value) method.
  pub trait VectorQtGuiColorValueArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::qt_gui::color::Color;
  }
  impl<'largs> VectorQtGuiColorValueArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::qt_gui::color::Color {
      let i = self;
      {
        let mut object: ::qt_gui::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_value_to_output_i(original_self as *const ::vector::VectorQtGuiColor,
                                                               i,
                                                               &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorQtGuiColorValueArgs<'largs> for (::libc::c_int, &'largs ::qt_gui::color::Color) {
    fn exec(self, original_self: &'largs ::vector::VectorQtGuiColor) -> ::qt_gui::color::Color {
      let i = self.0;
      let default_value = self.1;
      {
        let mut object: ::qt_gui::color::Color =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_widgets_c_QVector_QColor_value_to_output_i_defaultValue(original_self as *const ::vector::VectorQtGuiColor, i, default_value as *const ::qt_gui::color::Color, &mut object);
        }
        object
      }
    }
  }
}
