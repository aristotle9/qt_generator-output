/// C++ type: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>```</span>
#[repr(C)]
pub struct VectorAbstractActionInputMutPtr([u8; ::type_sizes::QT_3D_INPUT_VECTOR_VECTOR_ABSTRACT_ACTION_INPUT_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorAbstractActionInputMutPtr {
  unsafe fn new_uninitialized() -> VectorAbstractActionInputMutPtr {
    VectorAbstractActionInputMutPtr(::std::mem::uninitialized())
  }
}

impl VectorAbstractActionInputMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::append(const QVector<Qt3DInput::QAbstractActionInput*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorAbstractActionInputMutPtr) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_append_l(self as *mut ::vector::VectorAbstractActionInputMutPtr, l as *const ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::append(Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::abstract_action_input::AbstractActionInput) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_append_t(self as *mut ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* const & QVector<Qt3DInput::QAbstractActionInput*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_at(self as *const ::vector::VectorAbstractActionInputMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* const & QVector<Qt3DInput::QAbstractActionInput*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_back_const(self as *const ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput*& QVector<Qt3DInput::QAbstractActionInput*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_back(self as *mut ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractActionInput*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_capacity(self as *const ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_clear(self as *mut ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* const * QVector<Qt3DInput::QAbstractActionInput*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::abstract_action_input::AbstractActionInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_constData(self as *const ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* const & QVector<Qt3DInput::QAbstractActionInput*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_constFirst(self as *const ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* const & QVector<Qt3DInput::QAbstractActionInput*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_constLast(self as *const ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractActionInput*>::contains(Qt3DInput::QAbstractActionInput* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::abstract_action_input::AbstractActionInput) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_contains(self as *const ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractActionInput*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_count_no_args(self as *const ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractActionInput*>::count(Qt3DInput::QAbstractActionInput* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::abstract_action_input::AbstractActionInput) -> ::libc::c_int {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_count_t(self as *const ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* const * QVector<Qt3DInput::QAbstractActionInput*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::abstract_action_input::AbstractActionInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_data_const(self as *const ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput** QVector<Qt3DInput::QAbstractActionInput*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::abstract_action_input::AbstractActionInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_data(self as *mut ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractActionInput*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_empty(self as *const ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractActionInput*>::endsWith(Qt3DInput::QAbstractActionInput* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::abstract_action_input::AbstractActionInput) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_endsWith(self as *const ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::abstract_action_input::AbstractActionInput) -> &'l0 mut ::vector::VectorAbstractActionInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>& QVector<Qt3DInput::QAbstractActionInput*>::fill(Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::abstract_action_input::AbstractActionInput, ::libc::c_int)) -> &'l0 mut ::vector::VectorAbstractActionInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>& QVector<Qt3DInput::QAbstractActionInput*>::fill(Qt3DInput::QAbstractActionInput* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self,
                                   args: Args)
                                   -> &'largs mut ::vector::VectorAbstractActionInputMutPtr
    where Args: overloading::VectorAbstractActionInputMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* const & QVector<Qt3DInput::QAbstractActionInput*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_first_const(self as *const ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput*& QVector<Qt3DInput::QAbstractActionInput*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_first(self as *mut ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* const & QVector<Qt3DInput::QAbstractActionInput*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_front_const(self as *const ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput*& QVector<Qt3DInput::QAbstractActionInput*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_front(self as *mut ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::abstract_action_input::AbstractActionInput) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractActionInput*>::indexOf(Qt3DInput::QAbstractActionInput* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::abstract_action_input::AbstractActionInput, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractActionInput*>::indexOf(Qt3DInput::QAbstractActionInput* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAbstractActionInputMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::abstract_action_input::AbstractActionInput)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::insert(int i, Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::abstract_action_input::AbstractActionInput)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::insert(int i, int n, Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAbstractActionInputMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractActionInput*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_isEmpty(self as *const ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* const & QVector<Qt3DInput::QAbstractActionInput*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_last_const(self as *const ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::abstract_action_input::AbstractActionInput) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractActionInput*>::lastIndexOf(Qt3DInput::QAbstractActionInput* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::abstract_action_input::AbstractActionInput, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractActionInput*>::lastIndexOf(Qt3DInput::QAbstractActionInput* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAbstractActionInputMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput*& QVector<Qt3DInput::QAbstractActionInput*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_last(self as *mut ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractActionInput*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_length(self as *const ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorAbstractActionInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*> QVector<Qt3DInput::QAbstractActionInput*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorAbstractActionInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*> QVector<Qt3DInput::QAbstractActionInput*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorAbstractActionInputMutPtr
    where Args: overloading::VectorAbstractActionInputMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_move(self as *mut ::vector::VectorAbstractActionInputMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorAbstractActionInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAbstractActionInput*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorAbstractActionInputMutPtr) -> ::vector::VectorAbstractActionInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAbstractActionInput*>::QVector(const QVector<Qt3DInput::QAbstractActionInput*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorAbstractActionInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAbstractActionInput*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorAbstractActionInputMutPtr
    where Args: overloading::VectorAbstractActionInputMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAbstractActionInput*>::QVector(int size, Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int,
                           t: &*mut ::abstract_action_input::AbstractActionInput)
                           -> ::vector::VectorAbstractActionInputMutPtr {
    {
      let mut object: ::vector::VectorAbstractActionInputMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_constructor_size_t(size, t as *const *mut ::abstract_action_input::AbstractActionInput, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*> QVector<Qt3DInput::QAbstractActionInput*>::operator+(const QVector<Qt3DInput::QAbstractActionInput*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorAbstractActionInputMutPtr) -> ::vector::VectorAbstractActionInputMutPtr {
    {
      let mut object: ::vector::VectorAbstractActionInputMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_operator_add_to_output(self as *const ::vector::VectorAbstractActionInputMutPtr, l as *const ::vector::VectorAbstractActionInputMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>& QVector<Qt3DInput::QAbstractActionInput*>::operator+=(const QVector<Qt3DInput::QAbstractActionInput*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorAbstractActionInputMutPtr)
                                 -> &'l0 mut ::vector::VectorAbstractActionInputMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_operator_add_assign_l(self as *mut ::vector::VectorAbstractActionInputMutPtr, l as *const ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>& QVector<Qt3DInput::QAbstractActionInput*>::operator+=(Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::abstract_action_input::AbstractActionInput)
                                               -> &'l0 mut ::vector::VectorAbstractActionInputMutPtr {
    let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_operator_add_assign_t(self as *mut ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>& QVector<Qt3DInput::QAbstractActionInput*>::operator=(const QVector<Qt3DInput::QAbstractActionInput*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorAbstractActionInputMutPtr)
                             -> &'l0 mut ::vector::VectorAbstractActionInputMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_operator_assign(self as *mut ::vector::VectorAbstractActionInputMutPtr, v as *const ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractActionInput*>::operator==(const QVector<Qt3DInput::QAbstractActionInput*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorAbstractActionInputMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_operator_eq(self as *const ::vector::VectorAbstractActionInputMutPtr, v as *const ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* const & QVector<Qt3DInput::QAbstractActionInput*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_operator_index_const(self as *const ::vector::VectorAbstractActionInputMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput*& QVector<Qt3DInput::QAbstractActionInput*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self,
                           i: ::libc::c_int)
                           -> &'l0 mut *mut ::abstract_action_input::AbstractActionInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_operator_index(self as *mut ::vector::VectorAbstractActionInputMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractActionInput*>::operator!=(const QVector<Qt3DInput::QAbstractActionInput*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorAbstractActionInputMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_operator_neq(self as *const ::vector::VectorAbstractActionInputMutPtr, v as *const ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>& QVector<Qt3DInput::QAbstractActionInput*>::operator<<(const QVector<Qt3DInput::QAbstractActionInput*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorAbstractActionInputMutPtr)
                          -> &'l0 mut ::vector::VectorAbstractActionInputMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_operator_shl_l(self as *mut ::vector::VectorAbstractActionInputMutPtr, l as *const ::vector::VectorAbstractActionInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>& QVector<Qt3DInput::QAbstractActionInput*>::operator<<(Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::abstract_action_input::AbstractActionInput)
                                        -> &'l0 mut ::vector::VectorAbstractActionInputMutPtr {
    let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_operator_shl_t(self as *mut ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_pop_back(self as *mut ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_pop_front(self as *mut ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::prepend(Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::abstract_action_input::AbstractActionInput) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_prepend(self as *mut ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::push_back(Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::abstract_action_input::AbstractActionInput) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_push_back(self as *mut ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::push_front(Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::abstract_action_input::AbstractActionInput) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_push_front(self as *mut ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractActionInput*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAbstractActionInputMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractActionInput*>::removeAll(Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::abstract_action_input::AbstractActionInput) -> ::libc::c_int {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_removeAll(self as *mut ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_removeAt(self as *mut ::vector::VectorAbstractActionInputMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_removeFirst(self as *mut ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_removeLast(self as *mut ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractActionInput*>::removeOne(Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::abstract_action_input::AbstractActionInput) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_removeOne(self as *mut ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::replace(int i, Qt3DInput::QAbstractActionInput* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::abstract_action_input::AbstractActionInput) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_replace(self as *mut ::vector::VectorAbstractActionInputMutPtr, i, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_reserve(self as *mut ::vector::VectorAbstractActionInputMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_resize(self as *mut ::vector::VectorAbstractActionInputMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractActionInput*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_size(self as *const ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_squeeze(self as *mut ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractActionInput*>::startsWith(Qt3DInput::QAbstractActionInput* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::abstract_action_input::AbstractActionInput) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_startsWith(self as *const ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractActionInput*>::swap(QVector<Qt3DInput::QAbstractActionInput*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorAbstractActionInputMutPtr) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_swap(self as *mut ::vector::VectorAbstractActionInputMutPtr, other as *mut ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* QVector<Qt3DInput::QAbstractActionInput*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::abstract_action_input::AbstractActionInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_takeAt(self as *mut ::vector::VectorAbstractActionInputMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* QVector<Qt3DInput::QAbstractActionInput*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::abstract_action_input::AbstractActionInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_takeFirst(self as *mut ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* QVector<Qt3DInput::QAbstractActionInput*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::abstract_action_input::AbstractActionInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_takeLast(self as *mut ::vector::VectorAbstractActionInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* QVector<Qt3DInput::QAbstractActionInput*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::abstract_action_input::AbstractActionInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_value_i(self as *const ::vector::VectorAbstractActionInputMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractActionInput* QVector<Qt3DInput::QAbstractActionInput*>::value(int i, Qt3DInput::QAbstractActionInput* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::abstract_action_input::AbstractActionInput)
                             -> *mut ::abstract_action_input::AbstractActionInput {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_value_i_defaultValue(self as *const ::vector::VectorAbstractActionInputMutPtr, i, default_value as *const *mut ::abstract_action_input::AbstractActionInput)
  }
}

impl Drop for ::vector::VectorAbstractActionInputMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DInput::QAbstractActionInput*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_destructor(self as *mut ::vector::VectorAbstractActionInputMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>```</span>
#[repr(C)]
pub struct VectorAbstractAxisInputMutPtr([u8; ::type_sizes::QT_3D_INPUT_VECTOR_VECTOR_ABSTRACT_AXIS_INPUT_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorAbstractAxisInputMutPtr {
  unsafe fn new_uninitialized() -> VectorAbstractAxisInputMutPtr {
    VectorAbstractAxisInputMutPtr(::std::mem::uninitialized())
  }
}

impl VectorAbstractAxisInputMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::append(const QVector<Qt3DInput::QAbstractAxisInput*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorAbstractAxisInputMutPtr) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_append_l(self as *mut ::vector::VectorAbstractAxisInputMutPtr, l as *const ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::append(Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::abstract_axis_input::AbstractAxisInput) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_append_t(self as *mut ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* const & QVector<Qt3DInput::QAbstractAxisInput*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_at(self as *const ::vector::VectorAbstractAxisInputMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* const & QVector<Qt3DInput::QAbstractAxisInput*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_back_const(self as *const ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput*& QVector<Qt3DInput::QAbstractAxisInput*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_back(self as *mut ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractAxisInput*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_capacity(self as *const ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_clear(self as *mut ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* const * QVector<Qt3DInput::QAbstractAxisInput*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::abstract_axis_input::AbstractAxisInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_constData(self as *const ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* const & QVector<Qt3DInput::QAbstractAxisInput*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_constFirst(self as *const ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* const & QVector<Qt3DInput::QAbstractAxisInput*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_constLast(self as *const ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractAxisInput*>::contains(Qt3DInput::QAbstractAxisInput* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::abstract_axis_input::AbstractAxisInput) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_contains(self as *const ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractAxisInput*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_count_no_args(self as *const ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractAxisInput*>::count(Qt3DInput::QAbstractAxisInput* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::abstract_axis_input::AbstractAxisInput) -> ::libc::c_int {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_count_t(self as *const ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* const * QVector<Qt3DInput::QAbstractAxisInput*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::abstract_axis_input::AbstractAxisInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_data_const(self as *const ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput** QVector<Qt3DInput::QAbstractAxisInput*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::abstract_axis_input::AbstractAxisInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_data(self as *mut ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractAxisInput*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_empty(self as *const ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractAxisInput*>::endsWith(Qt3DInput::QAbstractAxisInput* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::abstract_axis_input::AbstractAxisInput) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_endsWith(self as *const ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::abstract_axis_input::AbstractAxisInput) -> &'l0 mut ::vector::VectorAbstractAxisInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>& QVector<Qt3DInput::QAbstractAxisInput*>::fill(Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::abstract_axis_input::AbstractAxisInput, ::libc::c_int)) -> &'l0 mut ::vector::VectorAbstractAxisInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>& QVector<Qt3DInput::QAbstractAxisInput*>::fill(Qt3DInput::QAbstractAxisInput* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorAbstractAxisInputMutPtr
    where Args: overloading::VectorAbstractAxisInputMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* const & QVector<Qt3DInput::QAbstractAxisInput*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_first_const(self as *const ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput*& QVector<Qt3DInput::QAbstractAxisInput*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_first(self as *mut ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* const & QVector<Qt3DInput::QAbstractAxisInput*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_front_const(self as *const ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput*& QVector<Qt3DInput::QAbstractAxisInput*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_front(self as *mut ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::abstract_axis_input::AbstractAxisInput) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractAxisInput*>::indexOf(Qt3DInput::QAbstractAxisInput* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::abstract_axis_input::AbstractAxisInput, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractAxisInput*>::indexOf(Qt3DInput::QAbstractAxisInput* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAbstractAxisInputMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::abstract_axis_input::AbstractAxisInput)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::insert(int i, Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::abstract_axis_input::AbstractAxisInput)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::insert(int i, int n, Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAbstractAxisInputMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractAxisInput*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_isEmpty(self as *const ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* const & QVector<Qt3DInput::QAbstractAxisInput*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_last_const(self as *const ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::abstract_axis_input::AbstractAxisInput) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractAxisInput*>::lastIndexOf(Qt3DInput::QAbstractAxisInput* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::abstract_axis_input::AbstractAxisInput, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractAxisInput*>::lastIndexOf(Qt3DInput::QAbstractAxisInput* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAbstractAxisInputMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput*& QVector<Qt3DInput::QAbstractAxisInput*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_last(self as *mut ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractAxisInput*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_length(self as *const ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorAbstractAxisInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*> QVector<Qt3DInput::QAbstractAxisInput*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorAbstractAxisInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*> QVector<Qt3DInput::QAbstractAxisInput*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorAbstractAxisInputMutPtr
    where Args: overloading::VectorAbstractAxisInputMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_move(self as *mut ::vector::VectorAbstractAxisInputMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorAbstractAxisInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAbstractAxisInput*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorAbstractAxisInputMutPtr) -> ::vector::VectorAbstractAxisInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAbstractAxisInput*>::QVector(const QVector<Qt3DInput::QAbstractAxisInput*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorAbstractAxisInputMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAbstractAxisInput*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorAbstractAxisInputMutPtr
    where Args: overloading::VectorAbstractAxisInputMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAbstractAxisInput*>::QVector(int size, Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int,
                           t: &*mut ::abstract_axis_input::AbstractAxisInput)
                           -> ::vector::VectorAbstractAxisInputMutPtr {
    {
      let mut object: ::vector::VectorAbstractAxisInputMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_constructor_size_t(size, t as *const *mut ::abstract_axis_input::AbstractAxisInput, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*> QVector<Qt3DInput::QAbstractAxisInput*>::operator+(const QVector<Qt3DInput::QAbstractAxisInput*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorAbstractAxisInputMutPtr) -> ::vector::VectorAbstractAxisInputMutPtr {
    {
      let mut object: ::vector::VectorAbstractAxisInputMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_operator_add_to_output(self as *const ::vector::VectorAbstractAxisInputMutPtr, l as *const ::vector::VectorAbstractAxisInputMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>& QVector<Qt3DInput::QAbstractAxisInput*>::operator+=(const QVector<Qt3DInput::QAbstractAxisInput*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorAbstractAxisInputMutPtr)
                                 -> &'l0 mut ::vector::VectorAbstractAxisInputMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_operator_add_assign_l(self as *mut ::vector::VectorAbstractAxisInputMutPtr, l as *const ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>& QVector<Qt3DInput::QAbstractAxisInput*>::operator+=(Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::abstract_axis_input::AbstractAxisInput)
                                               -> &'l0 mut ::vector::VectorAbstractAxisInputMutPtr {
    let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_operator_add_assign_t(self as *mut ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>& QVector<Qt3DInput::QAbstractAxisInput*>::operator=(const QVector<Qt3DInput::QAbstractAxisInput*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorAbstractAxisInputMutPtr)
                             -> &'l0 mut ::vector::VectorAbstractAxisInputMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_operator_assign(self as *mut ::vector::VectorAbstractAxisInputMutPtr, v as *const ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractAxisInput*>::operator==(const QVector<Qt3DInput::QAbstractAxisInput*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorAbstractAxisInputMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_operator_eq(self as *const ::vector::VectorAbstractAxisInputMutPtr, v as *const ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* const & QVector<Qt3DInput::QAbstractAxisInput*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_operator_index_const(self as *const ::vector::VectorAbstractAxisInputMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput*& QVector<Qt3DInput::QAbstractAxisInput*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::abstract_axis_input::AbstractAxisInput {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_operator_index(self as *mut ::vector::VectorAbstractAxisInputMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractAxisInput*>::operator!=(const QVector<Qt3DInput::QAbstractAxisInput*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorAbstractAxisInputMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_operator_neq(self as *const ::vector::VectorAbstractAxisInputMutPtr, v as *const ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>& QVector<Qt3DInput::QAbstractAxisInput*>::operator<<(const QVector<Qt3DInput::QAbstractAxisInput*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorAbstractAxisInputMutPtr)
                          -> &'l0 mut ::vector::VectorAbstractAxisInputMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_operator_shl_l(self as *mut ::vector::VectorAbstractAxisInputMutPtr, l as *const ::vector::VectorAbstractAxisInputMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>& QVector<Qt3DInput::QAbstractAxisInput*>::operator<<(Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::abstract_axis_input::AbstractAxisInput)
                                        -> &'l0 mut ::vector::VectorAbstractAxisInputMutPtr {
    let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_operator_shl_t(self as *mut ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_pop_back(self as *mut ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_pop_front(self as *mut ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::prepend(Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::abstract_axis_input::AbstractAxisInput) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_prepend(self as *mut ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::push_back(Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::abstract_axis_input::AbstractAxisInput) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_push_back(self as *mut ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::push_front(Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::abstract_axis_input::AbstractAxisInput) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_push_front(self as *mut ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAbstractAxisInput*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAbstractAxisInputMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractAxisInput*>::removeAll(Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::abstract_axis_input::AbstractAxisInput) -> ::libc::c_int {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_removeAll(self as *mut ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_removeAt(self as *mut ::vector::VectorAbstractAxisInputMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_removeFirst(self as *mut ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_removeLast(self as *mut ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractAxisInput*>::removeOne(Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::abstract_axis_input::AbstractAxisInput) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_removeOne(self as *mut ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::replace(int i, Qt3DInput::QAbstractAxisInput* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::abstract_axis_input::AbstractAxisInput) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_replace(self as *mut ::vector::VectorAbstractAxisInputMutPtr, i, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_reserve(self as *mut ::vector::VectorAbstractAxisInputMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_resize(self as *mut ::vector::VectorAbstractAxisInputMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAbstractAxisInput*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_size(self as *const ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_squeeze(self as *mut ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAbstractAxisInput*>::startsWith(Qt3DInput::QAbstractAxisInput* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::abstract_axis_input::AbstractAxisInput) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_startsWith(self as *const ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAbstractAxisInput*>::swap(QVector<Qt3DInput::QAbstractAxisInput*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorAbstractAxisInputMutPtr) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_swap(self as *mut ::vector::VectorAbstractAxisInputMutPtr, other as *mut ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* QVector<Qt3DInput::QAbstractAxisInput*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::abstract_axis_input::AbstractAxisInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_takeAt(self as *mut ::vector::VectorAbstractAxisInputMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* QVector<Qt3DInput::QAbstractAxisInput*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::abstract_axis_input::AbstractAxisInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_takeFirst(self as *mut ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* QVector<Qt3DInput::QAbstractAxisInput*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::abstract_axis_input::AbstractAxisInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_takeLast(self as *mut ::vector::VectorAbstractAxisInputMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* QVector<Qt3DInput::QAbstractAxisInput*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::abstract_axis_input::AbstractAxisInput {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_value_i(self as *const ::vector::VectorAbstractAxisInputMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAbstractAxisInput* QVector<Qt3DInput::QAbstractAxisInput*>::value(int i, Qt3DInput::QAbstractAxisInput* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::abstract_axis_input::AbstractAxisInput)
                             -> *mut ::abstract_axis_input::AbstractAxisInput {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_value_i_defaultValue(self as *const ::vector::VectorAbstractAxisInputMutPtr, i, default_value as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }
}

impl Drop for ::vector::VectorAbstractAxisInputMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DInput::QAbstractAxisInput*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_destructor(self as *mut ::vector::VectorAbstractAxisInputMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DInput::QAction*>```</span>
#[repr(C)]
pub struct VectorActionMutPtr([u8; ::type_sizes::QT_3D_INPUT_VECTOR_VECTOR_ACTION_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorActionMutPtr {
  unsafe fn new_uninitialized() -> VectorActionMutPtr {
    VectorActionMutPtr(::std::mem::uninitialized())
  }
}

impl VectorActionMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::append(const QVector<Qt3DInput::QAction*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorActionMutPtr) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_append_l(self as *mut ::vector::VectorActionMutPtr,
                                                                  l as *const ::vector::VectorActionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::append(Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::action::Action) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_append_t(self as *mut ::vector::VectorActionMutPtr,
                                                                t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* const & QVector<Qt3DInput::QAction*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_at(self as *const ::vector::VectorActionMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* const & QVector<Qt3DInput::QAction*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_back_const(self as *const ::vector::VectorActionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction*& QVector<Qt3DInput::QAction*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_back(self as *mut ::vector::VectorActionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAction*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_capacity(self as *const ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_clear(self as *mut ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* const * QVector<Qt3DInput::QAction*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::action::Action {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_constData(self as *const ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* const & QVector<Qt3DInput::QAction*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_constFirst(self as *const ::vector::VectorActionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* const & QVector<Qt3DInput::QAction*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_constLast(self as *const ::vector::VectorActionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAction*>::contains(Qt3DInput::QAction* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::action::Action) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_contains(self as *const ::vector::VectorActionMutPtr,
                                                                t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAction*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_count_no_args(self as *const ::vector::VectorActionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAction*>::count(Qt3DInput::QAction* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::action::Action) -> ::libc::c_int {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_count_t(self as *const ::vector::VectorActionMutPtr,
                                                               t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* const * QVector<Qt3DInput::QAction*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::action::Action {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_data_const(self as *const ::vector::VectorActionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction** QVector<Qt3DInput::QAction*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::action::Action {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_data(self as *mut ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAction*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_empty(self as *const ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAction*>::endsWith(Qt3DInput::QAction* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::action::Action) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_endsWith(self as *const ::vector::VectorActionMutPtr,
                                                                t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::action::Action) -> &'l0 mut ::vector::VectorActionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>& QVector<Qt3DInput::QAction*>::fill(Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::action::Action, ::libc::c_int)) -> &'l0 mut ::vector::VectorActionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>& QVector<Qt3DInput::QAction*>::fill(Qt3DInput::QAction* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorActionMutPtr
    where Args: overloading::VectorActionMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* const & QVector<Qt3DInput::QAction*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_first_const(self as *const ::vector::VectorActionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction*& QVector<Qt3DInput::QAction*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_first(self as *mut ::vector::VectorActionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* const & QVector<Qt3DInput::QAction*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_front_const(self as *const ::vector::VectorActionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction*& QVector<Qt3DInput::QAction*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_front(self as *mut ::vector::VectorActionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::action::Action) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAction*>::indexOf(Qt3DInput::QAction* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::action::Action, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAction*>::indexOf(Qt3DInput::QAction* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorActionMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::action::Action)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::insert(int i, Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::action::Action)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::insert(int i, int n, Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorActionMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAction*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_isEmpty(self as *const ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* const & QVector<Qt3DInput::QAction*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::action::Action {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_last_const(self as *const ::vector::VectorActionMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::action::Action) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAction*>::lastIndexOf(Qt3DInput::QAction* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::action::Action, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAction*>::lastIndexOf(Qt3DInput::QAction* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorActionMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction*& QVector<Qt3DInput::QAction*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::action::Action {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_last(self as *mut ::vector::VectorActionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAction*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_length(self as *const ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorActionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*> QVector<Qt3DInput::QAction*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorActionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*> QVector<Qt3DInput::QAction*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorActionMutPtr
    where Args: overloading::VectorActionMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_move(self as *mut ::vector::VectorActionMutPtr, from, to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorActionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAction*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorActionMutPtr) -> ::vector::VectorActionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAction*>::QVector(const QVector<Qt3DInput::QAction*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorActionMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAction*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorActionMutPtr
    where Args: overloading::VectorActionMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAction*>::QVector(int size, Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int, t: &*mut ::action::Action) -> ::vector::VectorActionMutPtr {
    {
      let mut object: ::vector::VectorActionMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_constructor_size_t(size,
                                                                            t as *const *mut ::action::Action,
                                                                            &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*> QVector<Qt3DInput::QAction*>::operator+(const QVector<Qt3DInput::QAction*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorActionMutPtr) -> ::vector::VectorActionMutPtr {
    {
      let mut object: ::vector::VectorActionMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_operator_add_to_output(self as *const ::vector::VectorActionMutPtr, l as *const ::vector::VectorActionMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>& QVector<Qt3DInput::QAction*>::operator+=(const QVector<Qt3DInput::QAction*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorActionMutPtr)
                                 -> &'l0 mut ::vector::VectorActionMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_operator_add_assign_l(self as *mut ::vector::VectorActionMutPtr, l as *const ::vector::VectorActionMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>& QVector<Qt3DInput::QAction*>::operator+=(Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::action::Action)
                                               -> &'l0 mut ::vector::VectorActionMutPtr {
    let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_operator_add_assign_t(self as *mut ::vector::VectorActionMutPtr, t as *const *mut ::action::Action);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>& QVector<Qt3DInput::QAction*>::operator=(const QVector<Qt3DInput::QAction*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorActionMutPtr)
                             -> &'l0 mut ::vector::VectorActionMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_operator_assign(self as *mut ::vector::VectorActionMutPtr,
                                                                           v as *const ::vector::VectorActionMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAction*>::operator==(const QVector<Qt3DInput::QAction*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorActionMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_operator_eq(self as *const ::vector::VectorActionMutPtr,
                                                                     v as *const ::vector::VectorActionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* const & QVector<Qt3DInput::QAction*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::action::Action {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_operator_index_const(self as *const ::vector::VectorActionMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction*& QVector<Qt3DInput::QAction*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::action::Action {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_operator_index(self as *mut ::vector::VectorActionMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAction*>::operator!=(const QVector<Qt3DInput::QAction*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorActionMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_operator_neq(self as *const ::vector::VectorActionMutPtr,
                                                                      v as *const ::vector::VectorActionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>& QVector<Qt3DInput::QAction*>::operator<<(const QVector<Qt3DInput::QAction*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorActionMutPtr)
                          -> &'l0 mut ::vector::VectorActionMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_operator_shl_l(self as *mut ::vector::VectorActionMutPtr,
                                                                          l as *const ::vector::VectorActionMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>& QVector<Qt3DInput::QAction*>::operator<<(Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::action::Action)
                                        -> &'l0 mut ::vector::VectorActionMutPtr {
    let ffi_result =
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_operator_shl_t(self as *mut ::vector::VectorActionMutPtr,
                                                                        t as *const *mut ::action::Action);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_pop_back(self as *mut ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_pop_front(self as *mut ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::prepend(Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::action::Action) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_prepend(self as *mut ::vector::VectorActionMutPtr,
                                                               t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::push_back(Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::action::Action) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_push_back(self as *mut ::vector::VectorActionMutPtr,
                                                                 t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::push_front(Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::action::Action) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_push_front(self as *mut ::vector::VectorActionMutPtr,
                                                                  t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAction*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorActionMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAction*>::removeAll(Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::action::Action) -> ::libc::c_int {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_removeAll(self as *mut ::vector::VectorActionMutPtr,
                                                                 t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_removeAt(self as *mut ::vector::VectorActionMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_removeFirst(self as *mut ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_removeLast(self as *mut ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAction*>::removeOne(Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::action::Action) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_removeOne(self as *mut ::vector::VectorActionMutPtr,
                                                                 t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::replace(int i, Qt3DInput::QAction* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::action::Action) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_replace(self as *mut ::vector::VectorActionMutPtr,
                                                               i,
                                                               t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_reserve(self as *mut ::vector::VectorActionMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_resize(self as *mut ::vector::VectorActionMutPtr, size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAction*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_size(self as *const ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_squeeze(self as *mut ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAction*>::startsWith(Qt3DInput::QAction* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::action::Action) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_startsWith(self as *const ::vector::VectorActionMutPtr,
                                                                  t as *const *mut ::action::Action)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAction*>::swap(QVector<Qt3DInput::QAction*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorActionMutPtr) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_swap(self as *mut ::vector::VectorActionMutPtr,
                                                              other as *mut ::vector::VectorActionMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* QVector<Qt3DInput::QAction*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::action::Action {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_takeAt(self as *mut ::vector::VectorActionMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* QVector<Qt3DInput::QAction*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_takeFirst(self as *mut ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* QVector<Qt3DInput::QAction*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::action::Action {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_takeLast(self as *mut ::vector::VectorActionMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* QVector<Qt3DInput::QAction*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::action::Action {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_value_i(self as *const ::vector::VectorActionMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAction* QVector<Qt3DInput::QAction*>::value(int i, Qt3DInput::QAction* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self, i: ::libc::c_int, default_value: &*mut ::action::Action) -> *mut ::action::Action {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_value_i_defaultValue(self as *const ::vector::VectorActionMutPtr, i, default_value as *const *mut ::action::Action)
  }
}

impl Drop for ::vector::VectorActionMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DInput::QAction*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_destructor(self as *mut ::vector::VectorActionMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>```</span>
#[repr(C)]
pub struct VectorAxisMutPtr([u8; ::type_sizes::QT_3D_INPUT_VECTOR_VECTOR_AXIS_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorAxisMutPtr {
  unsafe fn new_uninitialized() -> VectorAxisMutPtr {
    VectorAxisMutPtr(::std::mem::uninitialized())
  }
}

impl VectorAxisMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::append(const QVector<Qt3DInput::QAxis*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorAxisMutPtr) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_append_l(self as *mut ::vector::VectorAxisMutPtr,
                                                                l as *const ::vector::VectorAxisMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::append(Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::axis::Axis) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_append_t(self as *mut ::vector::VectorAxisMutPtr,
                                                              t as *const *mut ::axis::Axis)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* const & QVector<Qt3DInput::QAxis*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::axis::Axis {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_at(self as *const ::vector::VectorAxisMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* const & QVector<Qt3DInput::QAxis*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::axis::Axis {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_back_const(self as *const ::vector::VectorAxisMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis*& QVector<Qt3DInput::QAxis*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::axis::Axis {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_back(self as *mut ::vector::VectorAxisMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxis*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_capacity(self as *const ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_clear(self as *mut ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* const * QVector<Qt3DInput::QAxis*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::axis::Axis {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_constData(self as *const ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* const & QVector<Qt3DInput::QAxis*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::axis::Axis {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_constFirst(self as *const ::vector::VectorAxisMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* const & QVector<Qt3DInput::QAxis*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::axis::Axis {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_constLast(self as *const ::vector::VectorAxisMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxis*>::contains(Qt3DInput::QAxis* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::axis::Axis) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_contains(self as *const ::vector::VectorAxisMutPtr,
                                                              t as *const *mut ::axis::Axis)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxis*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_count_no_args(self as *const ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxis*>::count(Qt3DInput::QAxis* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::axis::Axis) -> ::libc::c_int {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_count_t(self as *const ::vector::VectorAxisMutPtr,
                                                             t as *const *mut ::axis::Axis)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* const * QVector<Qt3DInput::QAxis*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::axis::Axis {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_data_const(self as *const ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis** QVector<Qt3DInput::QAxis*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::axis::Axis {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_data(self as *mut ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxis*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_empty(self as *const ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxis*>::endsWith(Qt3DInput::QAxis* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::axis::Axis) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_endsWith(self as *const ::vector::VectorAxisMutPtr,
                                                              t as *const *mut ::axis::Axis)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::axis::Axis) -> &'l0 mut ::vector::VectorAxisMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>& QVector<Qt3DInput::QAxis*>::fill(Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::axis::Axis, ::libc::c_int)) -> &'l0 mut ::vector::VectorAxisMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>& QVector<Qt3DInput::QAxis*>::fill(Qt3DInput::QAxis* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorAxisMutPtr
    where Args: overloading::VectorAxisMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* const & QVector<Qt3DInput::QAxis*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::axis::Axis {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_first_const(self as *const ::vector::VectorAxisMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis*& QVector<Qt3DInput::QAxis*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::axis::Axis {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_first(self as *mut ::vector::VectorAxisMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* const & QVector<Qt3DInput::QAxis*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::axis::Axis {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_front_const(self as *const ::vector::VectorAxisMutPtr)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis*& QVector<Qt3DInput::QAxis*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::axis::Axis {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_front(self as *mut ::vector::VectorAxisMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::axis::Axis) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxis*>::indexOf(Qt3DInput::QAxis* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::axis::Axis, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxis*>::indexOf(Qt3DInput::QAxis* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAxisMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::axis::Axis)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::insert(int i, Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::axis::Axis)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::insert(int i, int n, Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAxisMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxis*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_isEmpty(self as *const ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* const & QVector<Qt3DInput::QAxis*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::axis::Axis {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_last_const(self as *const ::vector::VectorAxisMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::axis::Axis) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxis*>::lastIndexOf(Qt3DInput::QAxis* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::axis::Axis, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxis*>::lastIndexOf(Qt3DInput::QAxis* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAxisMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis*& QVector<Qt3DInput::QAxis*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::axis::Axis {
    let ffi_result =
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_last(self as *mut ::vector::VectorAxisMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxis*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_length(self as *const ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorAxisMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*> QVector<Qt3DInput::QAxis*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorAxisMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*> QVector<Qt3DInput::QAxis*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorAxisMutPtr
    where Args: overloading::VectorAxisMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_move(self as *mut ::vector::VectorAxisMutPtr, from, to) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorAxisMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAxis*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorAxisMutPtr) -> ::vector::VectorAxisMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAxis*>::QVector(const QVector<Qt3DInput::QAxis*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorAxisMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAxis*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorAxisMutPtr
    where Args: overloading::VectorAxisMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAxis*>::QVector(int size, Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int, t: &*mut ::axis::Axis) -> ::vector::VectorAxisMutPtr {
    {
      let mut object: ::vector::VectorAxisMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_constructor_size_t(size,
                                                                          t as *const *mut ::axis::Axis,
                                                                          &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*> QVector<Qt3DInput::QAxis*>::operator+(const QVector<Qt3DInput::QAxis*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorAxisMutPtr) -> ::vector::VectorAxisMutPtr {
    {
      let mut object: ::vector::VectorAxisMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_operator_add_to_output(self as *const ::vector::VectorAxisMutPtr, l as *const ::vector::VectorAxisMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>& QVector<Qt3DInput::QAxis*>::operator+=(const QVector<Qt3DInput::QAxis*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorAxisMutPtr)
                                 -> &'l0 mut ::vector::VectorAxisMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_operator_add_assign_l(self as *mut ::vector::VectorAxisMutPtr, l as *const ::vector::VectorAxisMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>& QVector<Qt3DInput::QAxis*>::operator+=(Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::axis::Axis)
                                               -> &'l0 mut ::vector::VectorAxisMutPtr {
    let ffi_result =
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_operator_add_assign_t(self as *mut ::vector::VectorAxisMutPtr,
                                                                             t as *const *mut ::axis::Axis);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>& QVector<Qt3DInput::QAxis*>::operator=(const QVector<Qt3DInput::QAxis*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self, v: &'l1 ::vector::VectorAxisMutPtr) -> &'l0 mut ::vector::VectorAxisMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_operator_assign(self as *mut ::vector::VectorAxisMutPtr,
                                                                         v as *const ::vector::VectorAxisMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxis*>::operator==(const QVector<Qt3DInput::QAxis*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorAxisMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_operator_eq(self as *const ::vector::VectorAxisMutPtr,
                                                                   v as *const ::vector::VectorAxisMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* const & QVector<Qt3DInput::QAxis*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::axis::Axis {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_operator_index_const(self as *const ::vector::VectorAxisMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis*& QVector<Qt3DInput::QAxis*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::axis::Axis {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_operator_index(self as *mut ::vector::VectorAxisMutPtr, i)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxis*>::operator!=(const QVector<Qt3DInput::QAxis*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorAxisMutPtr) -> bool {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_operator_neq(self as *const ::vector::VectorAxisMutPtr,
                                                                    v as *const ::vector::VectorAxisMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>& QVector<Qt3DInput::QAxis*>::operator<<(const QVector<Qt3DInput::QAxis*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self, l: &'l1 ::vector::VectorAxisMutPtr) -> &'l0 mut ::vector::VectorAxisMutPtr {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_operator_shl_l(self as *mut ::vector::VectorAxisMutPtr,
                                                                        l as *const ::vector::VectorAxisMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>& QVector<Qt3DInput::QAxis*>::operator<<(Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::axis::Axis)
                                        -> &'l0 mut ::vector::VectorAxisMutPtr {
    let ffi_result =
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_operator_shl_t(self as *mut ::vector::VectorAxisMutPtr,
                                                                      t as *const *mut ::axis::Axis);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_pop_back(self as *mut ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_pop_front(self as *mut ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::prepend(Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::axis::Axis) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_prepend(self as *mut ::vector::VectorAxisMutPtr,
                                                             t as *const *mut ::axis::Axis)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::push_back(Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::axis::Axis) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_push_back(self as *mut ::vector::VectorAxisMutPtr,
                                                               t as *const *mut ::axis::Axis)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::push_front(Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::axis::Axis) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_push_front(self as *mut ::vector::VectorAxisMutPtr,
                                                                t as *const *mut ::axis::Axis)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxis*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAxisMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxis*>::removeAll(Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::axis::Axis) -> ::libc::c_int {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_removeAll(self as *mut ::vector::VectorAxisMutPtr,
                                                               t as *const *mut ::axis::Axis)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_removeAt(self as *mut ::vector::VectorAxisMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_removeFirst(self as *mut ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_removeLast(self as *mut ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxis*>::removeOne(Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::axis::Axis) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_removeOne(self as *mut ::vector::VectorAxisMutPtr,
                                                               t as *const *mut ::axis::Axis)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::replace(int i, Qt3DInput::QAxis* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::axis::Axis) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_replace(self as *mut ::vector::VectorAxisMutPtr,
                                                             i,
                                                             t as *const *mut ::axis::Axis)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_reserve(self as *mut ::vector::VectorAxisMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_resize(self as *mut ::vector::VectorAxisMutPtr, size) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxis*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_size(self as *const ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_squeeze(self as *mut ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxis*>::startsWith(Qt3DInput::QAxis* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::axis::Axis) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_startsWith(self as *const ::vector::VectorAxisMutPtr,
                                                                t as *const *mut ::axis::Axis)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxis*>::swap(QVector<Qt3DInput::QAxis*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorAxisMutPtr) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_swap(self as *mut ::vector::VectorAxisMutPtr,
                                                            other as *mut ::vector::VectorAxisMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* QVector<Qt3DInput::QAxis*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::axis::Axis {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_takeAt(self as *mut ::vector::VectorAxisMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* QVector<Qt3DInput::QAxis*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::axis::Axis {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_takeFirst(self as *mut ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* QVector<Qt3DInput::QAxis*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::axis::Axis {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_takeLast(self as *mut ::vector::VectorAxisMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* QVector<Qt3DInput::QAxis*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::axis::Axis {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_value_i(self as *const ::vector::VectorAxisMutPtr, i) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxis* QVector<Qt3DInput::QAxis*>::value(int i, Qt3DInput::QAxis* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self, i: ::libc::c_int, default_value: &*mut ::axis::Axis) -> *mut ::axis::Axis {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_value_i_defaultValue(self as *const ::vector::VectorAxisMutPtr,
                                                                          i,
                                                                          default_value as *const *mut ::axis::Axis)
  }
}

impl Drop for ::vector::VectorAxisMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DInput::QAxis*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_destructor(self as *mut ::vector::VectorAxisMutPtr) }
  }
}

/// C++ type: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>```</span>
#[repr(C)]
pub struct VectorAxisSettingMutPtr([u8; ::type_sizes::QT_3D_INPUT_VECTOR_VECTOR_AXIS_SETTING_MUT_PTR]);

impl ::cpp_utils::new_uninitialized::NewUninitialized for VectorAxisSettingMutPtr {
  unsafe fn new_uninitialized() -> VectorAxisSettingMutPtr {
    VectorAxisSettingMutPtr(::std::mem::uninitialized())
  }
}

impl VectorAxisSettingMutPtr {
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::append(const QVector<Qt3DInput::QAxisSetting*>& l)```</span>
  ///
  ///
  pub fn append(&mut self, l: &::vector::VectorAxisSettingMutPtr) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_append_l(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                       l as *const ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::append(Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  pub unsafe fn append_unsafe(&mut self, t: &*mut ::axis_setting::AxisSetting) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_append_t(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                     t as *const *mut ::axis_setting::AxisSetting)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* const & QVector<Qt3DInput::QAxisSetting*>::at(int i) const```</span>
  ///
  ///
  pub fn at<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::axis_setting::AxisSetting {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_at(self as *const ::vector::VectorAxisSettingMutPtr, i)
      };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* const & QVector<Qt3DInput::QAxisSetting*>::back() const```</span>
  ///
  ///
  pub fn back<'l0>(&'l0 self) -> &'l0 *mut ::axis_setting::AxisSetting {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_back_const(self as *const ::vector::VectorAxisSettingMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting*& QVector<Qt3DInput::QAxisSetting*>::back()```</span>
  ///
  ///
  pub fn back_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::axis_setting::AxisSetting {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_back(self as *mut ::vector::VectorAxisSettingMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxisSetting*>::capacity() const```</span>
  ///
  ///
  pub fn capacity(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_capacity(self as *const ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::clear()```</span>
  ///
  ///
  pub fn clear(&mut self) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_clear(self as *mut ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* const * QVector<Qt3DInput::QAxisSetting*>::constData() const```</span>
  ///
  ///
  pub fn const_data(&self) -> *const *mut ::axis_setting::AxisSetting {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_constData(self as *const ::vector::VectorAxisSettingMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* const & QVector<Qt3DInput::QAxisSetting*>::constFirst() const```</span>
  ///
  ///
  pub fn const_first<'l0>(&'l0 self) -> &'l0 *mut ::axis_setting::AxisSetting {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_constFirst(self as *const ::vector::VectorAxisSettingMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* const & QVector<Qt3DInput::QAxisSetting*>::constLast() const```</span>
  ///
  ///
  pub fn const_last<'l0>(&'l0 self) -> &'l0 *mut ::axis_setting::AxisSetting {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_constLast(self as *const ::vector::VectorAxisSettingMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxisSetting*>::contains(Qt3DInput::QAxisSetting* const & t) const```</span>
  ///
  ///
  pub unsafe fn contains(&self, t: &*mut ::axis_setting::AxisSetting) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_contains(self as *const ::vector::VectorAxisSettingMutPtr,
                                                                     t as *const *mut ::axis_setting::AxisSetting)
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxisSetting*>::count() const```</span>
  ///
  ///
  pub fn count(&self) -> ::libc::c_int {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_count_no_args(self as *const ::vector::VectorAxisSettingMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxisSetting*>::count(Qt3DInput::QAxisSetting* const & t) const```</span>
  ///
  ///
  pub unsafe fn count_unsafe(&self, t: &*mut ::axis_setting::AxisSetting) -> ::libc::c_int {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_count_t(self as *const ::vector::VectorAxisSettingMutPtr,
                                                                    t as *const *mut ::axis_setting::AxisSetting)
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* const * QVector<Qt3DInput::QAxisSetting*>::data() const```</span>
  ///
  ///
  pub fn data(&self) -> *const *mut ::axis_setting::AxisSetting {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_data_const(self as *const ::vector::VectorAxisSettingMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting** QVector<Qt3DInput::QAxisSetting*>::data()```</span>
  ///
  ///
  pub fn data_mut(&mut self) -> *mut *mut ::axis_setting::AxisSetting {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_data(self as *mut ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxisSetting*>::empty() const```</span>
  ///
  ///
  pub fn empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_empty(self as *const ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxisSetting*>::endsWith(Qt3DInput::QAxisSetting* const & t) const```</span>
  ///
  ///
  pub unsafe fn ends_with(&self, t: &*mut ::axis_setting::AxisSetting) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_endsWith(self as *const ::vector::VectorAxisSettingMutPtr,
                                                                     t as *const *mut ::axis_setting::AxisSetting)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>::fill```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn fill(&mut self, &'l1 *mut ::axis_setting::AxisSetting) -> &'l0 mut ::vector::VectorAxisSettingMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>& QVector<Qt3DInput::QAxisSetting*>::fill(Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn fill(&mut self, (&'l1 *mut ::axis_setting::AxisSetting, ::libc::c_int)) -> &'l0 mut ::vector::VectorAxisSettingMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>& QVector<Qt3DInput::QAxisSetting*>::fill(Qt3DInput::QAxisSetting* const & t, int size = ?)```</span>
  ///
  ///
  pub unsafe fn fill<'largs, Args>(&'largs mut self, args: Args) -> &'largs mut ::vector::VectorAxisSettingMutPtr
    where Args: overloading::VectorAxisSettingMutPtrFillArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* const & QVector<Qt3DInput::QAxisSetting*>::first() const```</span>
  ///
  ///
  pub fn first<'l0>(&'l0 self) -> &'l0 *mut ::axis_setting::AxisSetting {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_first_const(self as *const ::vector::VectorAxisSettingMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting*& QVector<Qt3DInput::QAxisSetting*>::first()```</span>
  ///
  ///
  pub fn first_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::axis_setting::AxisSetting {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_first(self as *mut ::vector::VectorAxisSettingMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* const & QVector<Qt3DInput::QAxisSetting*>::front() const```</span>
  ///
  ///
  pub fn front<'l0>(&'l0 self) -> &'l0 *mut ::axis_setting::AxisSetting {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_front_const(self as *const ::vector::VectorAxisSettingMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting*& QVector<Qt3DInput::QAxisSetting*>::front()```</span>
  ///
  ///
  pub fn front_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::axis_setting::AxisSetting {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_front(self as *mut ::vector::VectorAxisSettingMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>::indexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn index_of(&self, &*mut ::axis_setting::AxisSetting) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxisSetting*>::indexOf(Qt3DInput::QAxisSetting* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn index_of(&self, (&*mut ::axis_setting::AxisSetting, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxisSetting*>::indexOf(Qt3DInput::QAxisSetting* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAxisSettingMutPtrIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>::insert```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, &*mut ::axis_setting::AxisSetting)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::insert(int i, Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn insert(&mut self, (::libc::c_int, ::libc::c_int, &*mut ::axis_setting::AxisSetting)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::insert(int i, int n, Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  pub unsafe fn insert<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAxisSettingMutPtrInsertArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxisSetting*>::isEmpty() const```</span>
  ///
  ///
  pub fn is_empty(&self) -> bool {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_isEmpty(self as *const ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* const & QVector<Qt3DInput::QAxisSetting*>::last() const```</span>
  ///
  ///
  pub fn last<'l0>(&'l0 self) -> &'l0 *mut ::axis_setting::AxisSetting {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_last_const(self as *const ::vector::VectorAxisSettingMutPtr) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>::lastIndexOf```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn last_index_of(&self, &*mut ::axis_setting::AxisSetting) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxisSetting*>::lastIndexOf(Qt3DInput::QAxisSetting* const & t) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn last_index_of(&self, (&*mut ::axis_setting::AxisSetting, ::libc::c_int)) -> ::libc::c_int```<br>
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxisSetting*>::lastIndexOf(Qt3DInput::QAxisSetting* const & t, int from = ?) const```</span>
  ///
  ///
  pub unsafe fn last_index_of<'largs, Args>(&'largs self, args: Args) -> ::libc::c_int
    where Args: overloading::VectorAxisSettingMutPtrLastIndexOfArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting*& QVector<Qt3DInput::QAxisSetting*>::last()```</span>
  ///
  ///
  pub fn last_mut<'l0>(&'l0 mut self) -> &'l0 mut *mut ::axis_setting::AxisSetting {
    let ffi_result =
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_last(self as *mut ::vector::VectorAxisSettingMutPtr)
      };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxisSetting*>::length() const```</span>
  ///
  ///
  pub fn length(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_length(self as *const ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>::mid```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn mid(&self, ::libc::c_int) -> ::vector::VectorAxisSettingMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*> QVector<Qt3DInput::QAxisSetting*>::mid(int pos) const```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn mid(&self, (::libc::c_int, ::libc::c_int)) -> ::vector::VectorAxisSettingMutPtr```<br>
  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*> QVector<Qt3DInput::QAxisSetting*>::mid(int pos, int len = ?) const```</span>
  ///
  ///
  pub fn mid<'largs, Args>(&'largs self, args: Args) -> ::vector::VectorAxisSettingMutPtr
    where Args: overloading::VectorAxisSettingMutPtrMidArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::move(int from, int to)```</span>
  ///
  ///
  pub fn move_(&mut self, from: ::libc::c_int, to: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_move(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                   from,
                                                                   to)
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>::QVector```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn new(()) -> ::vector::VectorAxisSettingMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAxisSetting*>::QVector()```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn new(&::vector::VectorAxisSettingMutPtr) -> ::vector::VectorAxisSettingMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAxisSetting*>::QVector(const QVector<Qt3DInput::QAxisSetting*>& v)```</span>
  ///
  ///
  ///
  /// ## Variant 3
  ///
  /// Rust arguments: ```fn new(::libc::c_int) -> ::vector::VectorAxisSettingMutPtr```<br>
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAxisSetting*>::QVector(int size)```</span>
  ///
  ///
  pub fn new<Args>(args: Args) -> ::vector::VectorAxisSettingMutPtr
    where Args: overloading::VectorAxisSettingMutPtrNewArgs
  {
    args.exec()
  }
  /// C++ method: <span style='color: green;'>```[constructor] void QVector<Qt3DInput::QAxisSetting*>::QVector(int size, Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  pub unsafe fn new_unsafe(size: ::libc::c_int,
                           t: &*mut ::axis_setting::AxisSetting)
                           -> ::vector::VectorAxisSettingMutPtr {
    {
      let mut object: ::vector::VectorAxisSettingMutPtr =
        ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized();
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_constructor_size_t(size, t as *const *mut ::axis_setting::AxisSetting, &mut object);
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*> QVector<Qt3DInput::QAxisSetting*>::operator+(const QVector<Qt3DInput::QAxisSetting*>& l) const```</span>
  ///
  ///
  pub fn op_add(&self, l: &::vector::VectorAxisSettingMutPtr) -> ::vector::VectorAxisSettingMutPtr {
    {
      let mut object: ::vector::VectorAxisSettingMutPtr =
        unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_operator_add_to_output(self as *const ::vector::VectorAxisSettingMutPtr, l as *const ::vector::VectorAxisSettingMutPtr, &mut object);
      }
      object
    }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>& QVector<Qt3DInput::QAxisSetting*>::operator+=(const QVector<Qt3DInput::QAxisSetting*>& l)```</span>
  ///
  ///
  pub fn op_add_assign<'l0, 'l1>(&'l0 mut self,
                                 l: &'l1 ::vector::VectorAxisSettingMutPtr)
                                 -> &'l0 mut ::vector::VectorAxisSettingMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_operator_add_assign_l(self as *mut ::vector::VectorAxisSettingMutPtr, l as *const ::vector::VectorAxisSettingMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>& QVector<Qt3DInput::QAxisSetting*>::operator+=(Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  pub unsafe fn op_add_assign_unsafe<'l0, 'l1>(&'l0 mut self,
                                               t: &'l1 *mut ::axis_setting::AxisSetting)
                                               -> &'l0 mut ::vector::VectorAxisSettingMutPtr {
    let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_operator_add_assign_t(self as *mut ::vector::VectorAxisSettingMutPtr, t as *const *mut ::axis_setting::AxisSetting);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>& QVector<Qt3DInput::QAxisSetting*>::operator=(const QVector<Qt3DInput::QAxisSetting*>& v)```</span>
  ///
  ///
  pub fn op_assign<'l0, 'l1>(&'l0 mut self,
                             v: &'l1 ::vector::VectorAxisSettingMutPtr)
                             -> &'l0 mut ::vector::VectorAxisSettingMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_operator_assign(self as *mut ::vector::VectorAxisSettingMutPtr, v as *const ::vector::VectorAxisSettingMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxisSetting*>::operator==(const QVector<Qt3DInput::QAxisSetting*>& v) const```</span>
  ///
  ///
  pub fn op_eq(&self, v: &::vector::VectorAxisSettingMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_operator_eq(self as *const ::vector::VectorAxisSettingMutPtr, v as *const ::vector::VectorAxisSettingMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* const & QVector<Qt3DInput::QAxisSetting*>::operator[](int i) const```</span>
  ///
  ///
  pub fn op_index<'l0>(&'l0 self, i: ::libc::c_int) -> &'l0 *mut ::axis_setting::AxisSetting {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_operator_index_const(self as *const ::vector::VectorAxisSettingMutPtr, i) };
    unsafe { ffi_result.as_ref() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting*& QVector<Qt3DInput::QAxisSetting*>::operator[](int i)```</span>
  ///
  ///
  pub fn op_index_mut<'l0>(&'l0 mut self, i: ::libc::c_int) -> &'l0 mut *mut ::axis_setting::AxisSetting {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_operator_index(self as *mut ::vector::VectorAxisSettingMutPtr, i) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxisSetting*>::operator!=(const QVector<Qt3DInput::QAxisSetting*>& v) const```</span>
  ///
  ///
  pub fn op_neq(&self, v: &::vector::VectorAxisSettingMutPtr) -> bool {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_operator_neq(self as *const ::vector::VectorAxisSettingMutPtr, v as *const ::vector::VectorAxisSettingMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>& QVector<Qt3DInput::QAxisSetting*>::operator<<(const QVector<Qt3DInput::QAxisSetting*>& l)```</span>
  ///
  ///
  pub fn op_shl<'l0, 'l1>(&'l0 mut self,
                          l: &'l1 ::vector::VectorAxisSettingMutPtr)
                          -> &'l0 mut ::vector::VectorAxisSettingMutPtr {
    let ffi_result = unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_operator_shl_l(self as *mut ::vector::VectorAxisSettingMutPtr, l as *const ::vector::VectorAxisSettingMutPtr) };
    unsafe { ffi_result.as_mut() }.expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>& QVector<Qt3DInput::QAxisSetting*>::operator<<(Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  pub unsafe fn op_shl_unsafe<'l0, 'l1>(&'l0 mut self,
                                        t: &'l1 *mut ::axis_setting::AxisSetting)
                                        -> &'l0 mut ::vector::VectorAxisSettingMutPtr {
    let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_operator_shl_t(self as *mut ::vector::VectorAxisSettingMutPtr, t as *const *mut ::axis_setting::AxisSetting);
    ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::pop_back()```</span>
  ///
  ///
  pub fn pop_back(&mut self) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_pop_back(self as *mut ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::pop_front()```</span>
  ///
  ///
  pub fn pop_front(&mut self) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_pop_front(self as *mut ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::prepend(Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  pub unsafe fn prepend(&mut self, t: &*mut ::axis_setting::AxisSetting) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_prepend(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                    t as *const *mut ::axis_setting::AxisSetting)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::push_back(Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  pub unsafe fn push_back(&mut self, t: &*mut ::axis_setting::AxisSetting) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_push_back(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                      t as *const *mut ::axis_setting::AxisSetting)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::push_front(Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  pub unsafe fn push_front(&mut self, t: &*mut ::axis_setting::AxisSetting) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_push_front(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                       t as *const *mut ::axis_setting::AxisSetting)
  }

  /// C++ method: <span style='color: green;'>```QVector<Qt3DInput::QAxisSetting*>::remove```</span>
  ///
  /// This is an overloaded function. Available variants:
  ///
  ///
  ///
  /// ## Variant 1
  ///
  /// Rust arguments: ```fn remove(&mut self, ::libc::c_int) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::remove(int i)```</span>
  ///
  ///
  ///
  /// ## Variant 2
  ///
  /// Rust arguments: ```fn remove(&mut self, (::libc::c_int, ::libc::c_int)) -> ()```<br>
  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::remove(int i, int n)```</span>
  ///
  ///
  pub fn remove<'largs, Args>(&'largs mut self, args: Args) -> ()
    where Args: overloading::VectorAxisSettingMutPtrRemoveArgs<'largs>
  {
    args.exec(self)
  }
  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxisSetting*>::removeAll(Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_all(&mut self, t: &*mut ::axis_setting::AxisSetting) -> ::libc::c_int {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_removeAll(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                      t as *const *mut ::axis_setting::AxisSetting)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::removeAt(int i)```</span>
  ///
  ///
  pub fn remove_at(&mut self, i: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_removeAt(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                       i)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::removeFirst()```</span>
  ///
  ///
  pub fn remove_first(&mut self) {
    unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_removeFirst(self as *mut ::vector::VectorAxisSettingMutPtr) }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::removeLast()```</span>
  ///
  ///
  pub fn remove_last(&mut self) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_removeLast(self as *mut ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxisSetting*>::removeOne(Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  pub unsafe fn remove_one(&mut self, t: &*mut ::axis_setting::AxisSetting) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_removeOne(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                      t as *const *mut ::axis_setting::AxisSetting)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::replace(int i, Qt3DInput::QAxisSetting* const & t)```</span>
  ///
  ///
  pub unsafe fn replace(&mut self, i: ::libc::c_int, t: &*mut ::axis_setting::AxisSetting) {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_replace(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                    i,
                                                                    t as *const *mut ::axis_setting::AxisSetting)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::reserve(int size)```</span>
  ///
  ///
  pub fn reserve(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_reserve(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                      size)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::resize(int size)```</span>
  ///
  ///
  pub fn resize(&mut self, size: ::libc::c_int) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_resize(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                     size)
    }
  }

  /// C++ method: <span style='color: green;'>```int QVector<Qt3DInput::QAxisSetting*>::size() const```</span>
  ///
  ///
  pub fn size(&self) -> ::libc::c_int {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_size(self as *const ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::squeeze()```</span>
  ///
  ///
  pub fn squeeze(&mut self) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_squeeze(self as *mut ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```bool QVector<Qt3DInput::QAxisSetting*>::startsWith(Qt3DInput::QAxisSetting* const & t) const```</span>
  ///
  ///
  pub unsafe fn starts_with(&self, t: &*mut ::axis_setting::AxisSetting) -> bool {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_startsWith(self as *const ::vector::VectorAxisSettingMutPtr, t as *const *mut ::axis_setting::AxisSetting)
  }

  /// C++ method: <span style='color: green;'>```void QVector<Qt3DInput::QAxisSetting*>::swap(QVector<Qt3DInput::QAxisSetting*>& other)```</span>
  ///
  ///
  pub fn swap(&mut self, other: &mut ::vector::VectorAxisSettingMutPtr) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_swap(self as *mut ::vector::VectorAxisSettingMutPtr,
                                                                   other as *mut ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* QVector<Qt3DInput::QAxisSetting*>::takeAt(int i)```</span>
  ///
  ///
  pub fn take_at(&mut self, i: ::libc::c_int) -> *mut ::axis_setting::AxisSetting {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_takeAt(self as *mut ::vector::VectorAxisSettingMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* QVector<Qt3DInput::QAxisSetting*>::takeFirst()```</span>
  ///
  ///
  pub fn take_first(&mut self) -> *mut ::axis_setting::AxisSetting {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_takeFirst(self as *mut ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* QVector<Qt3DInput::QAxisSetting*>::takeLast()```</span>
  ///
  ///
  pub fn take_last(&mut self) -> *mut ::axis_setting::AxisSetting {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_takeLast(self as *mut ::vector::VectorAxisSettingMutPtr)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* QVector<Qt3DInput::QAxisSetting*>::value(int i) const```</span>
  ///
  ///
  pub fn value(&self, i: ::libc::c_int) -> *mut ::axis_setting::AxisSetting {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_value_i(self as *const ::vector::VectorAxisSettingMutPtr, i)
    }
  }

  /// C++ method: <span style='color: green;'>```Qt3DInput::QAxisSetting* QVector<Qt3DInput::QAxisSetting*>::value(int i, Qt3DInput::QAxisSetting* const & defaultValue) const```</span>
  ///
  ///
  pub unsafe fn value_unsafe(&self,
                             i: ::libc::c_int,
                             default_value: &*mut ::axis_setting::AxisSetting)
                             -> *mut ::axis_setting::AxisSetting {
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_value_i_defaultValue(self as *const ::vector::VectorAxisSettingMutPtr, i, default_value as *const *mut ::axis_setting::AxisSetting)
  }
}

impl Drop for ::vector::VectorAxisSettingMutPtr {
  /// C++ method: <span style='color: green;'>```[destructor] void QVector<Qt3DInput::QAxisSetting*>::~QVector()```</span>
  ///
  ///
  fn drop(&mut self) {
    unsafe {
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_destructor(self as *mut ::vector::VectorAxisSettingMutPtr)
    }
  }
}

/// Types for emulating overloading for overloaded functions in this module
pub mod overloading {
  /// This trait represents a set of arguments accepted by [VectorAbstractActionInputMutPtr::fill](../struct.VectorAbstractActionInputMutPtr.html#method.fill) method.
  pub trait VectorAbstractActionInputMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAbstractActionInputMutPtr)
                   -> &'largs mut ::vector::VectorAbstractActionInputMutPtr;
  }
  impl<'largs> VectorAbstractActionInputMutPtrFillArgs<'largs> for &'largs *mut ::abstract_action_input::AbstractActionInput {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractActionInputMutPtr) -> &'largs mut ::vector::VectorAbstractActionInputMutPtr {
    let t = self;
    let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_fill_t(original_self as *mut ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }
}
  impl<'largs> VectorAbstractActionInputMutPtrFillArgs<'largs> for (&'largs *mut ::abstract_action_input::AbstractActionInput,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractActionInputMutPtr) -> &'largs mut ::vector::VectorAbstractActionInputMutPtr {
    let t = self.0;
let size = self.1;
    let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_fill_t_size(original_self as *mut ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput, size);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractActionInputMutPtr::index_of](../struct.VectorAbstractActionInputMutPtr.html#method.index_of) method.
  pub trait VectorAbstractActionInputMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractActionInputMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAbstractActionInputMutPtrIndexOfArgs<'largs> for &'largs *mut ::abstract_action_input::AbstractActionInput {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractActionInputMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_indexOf_t(original_self as *const ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }
}
  impl<'largs> VectorAbstractActionInputMutPtrIndexOfArgs<'largs> for (&'largs *mut ::abstract_action_input::AbstractActionInput,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractActionInputMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_indexOf_t_from(original_self as *const ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput, from)
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractActionInputMutPtr::insert](../struct.VectorAbstractActionInputMutPtr.html#method.insert) method.
  pub trait VectorAbstractActionInputMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractActionInputMutPtr) -> ();
  }
  impl<'largs> VectorAbstractActionInputMutPtrInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs *mut ::abstract_action_input::AbstractActionInput) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractActionInputMutPtr) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_insert_i_n_t(original_self as *mut ::vector::VectorAbstractActionInputMutPtr, i, n, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }
}
  impl<'largs> VectorAbstractActionInputMutPtrInsertArgs<'largs> for (::libc::c_int,&'largs *mut ::abstract_action_input::AbstractActionInput) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractActionInputMutPtr) -> () {
    let i = self.0;
let t = self.1;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_insert_i_t(original_self as *mut ::vector::VectorAbstractActionInputMutPtr, i, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractActionInputMutPtr::last_index_of](../struct.VectorAbstractActionInputMutPtr.html#method.last_index_of) method.
  pub trait VectorAbstractActionInputMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractActionInputMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAbstractActionInputMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::abstract_action_input::AbstractActionInput {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractActionInputMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_lastIndexOf_t(original_self as *const ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput)
  }
}
  impl<'largs> VectorAbstractActionInputMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::abstract_action_input::AbstractActionInput,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractActionInputMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorAbstractActionInputMutPtr, t as *const *mut ::abstract_action_input::AbstractActionInput, from)
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractActionInputMutPtr::mid](../struct.VectorAbstractActionInputMutPtr.html#method.mid) method.
  pub trait VectorAbstractActionInputMutPtrMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractActionInputMutPtr)
            -> ::vector::VectorAbstractActionInputMutPtr;
  }
  impl<'largs> VectorAbstractActionInputMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractActionInputMutPtr)
            -> ::vector::VectorAbstractActionInputMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorAbstractActionInputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_mid_to_output_pos(original_self as *const ::vector::VectorAbstractActionInputMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorAbstractActionInputMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractActionInputMutPtr)
            -> ::vector::VectorAbstractActionInputMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorAbstractActionInputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorAbstractActionInputMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractActionInputMutPtr::new](../struct.VectorAbstractActionInputMutPtr.html#method.new) method.
  pub trait VectorAbstractActionInputMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorAbstractActionInputMutPtr;
  }
  impl VectorAbstractActionInputMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorAbstractActionInputMutPtr {

      {
        let mut object: ::vector::VectorAbstractActionInputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorAbstractActionInputMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorAbstractActionInputMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorAbstractActionInputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorAbstractActionInputMutPtrNewArgs for &'a ::vector::VectorAbstractActionInputMutPtr {
    fn exec(self) -> ::vector::VectorAbstractActionInputMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorAbstractActionInputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_constructor_v(v as *const ::vector::VectorAbstractActionInputMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractActionInputMutPtr::remove](../struct.VectorAbstractActionInputMutPtr.html#method.remove) method.
  pub trait VectorAbstractActionInputMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractActionInputMutPtr) -> ();
  }
  impl<'largs> VectorAbstractActionInputMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractActionInputMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_remove_i(original_self as *mut ::vector::VectorAbstractActionInputMutPtr, i) }
    }
  }
  impl<'largs> VectorAbstractActionInputMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractActionInputMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractActionInput_ptr_remove_i_n(original_self as *mut ::vector::VectorAbstractActionInputMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractAxisInputMutPtr::fill](../struct.VectorAbstractAxisInputMutPtr.html#method.fill) method.
  pub trait VectorAbstractAxisInputMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAbstractAxisInputMutPtr)
                   -> &'largs mut ::vector::VectorAbstractAxisInputMutPtr;
  }
  impl<'largs> VectorAbstractAxisInputMutPtrFillArgs<'largs> for &'largs *mut ::abstract_axis_input::AbstractAxisInput {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAbstractAxisInputMutPtr)
                   -> &'largs mut ::vector::VectorAbstractAxisInputMutPtr {
      let t = self;
      let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_fill_t(original_self as *mut ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorAbstractAxisInputMutPtrFillArgs<'largs> for (&'largs *mut ::abstract_axis_input::AbstractAxisInput,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAxisInputMutPtr) -> &'largs mut ::vector::VectorAbstractAxisInputMutPtr {
    let t = self.0;
let size = self.1;
    let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_fill_t_size(original_self as *mut ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput, size);
ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractAxisInputMutPtr::index_of](../struct.VectorAbstractAxisInputMutPtr.html#method.index_of) method.
  pub trait VectorAbstractAxisInputMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAxisInputMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAbstractAxisInputMutPtrIndexOfArgs<'largs> for &'largs *mut ::abstract_axis_input::AbstractAxisInput {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAxisInputMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_indexOf_t(original_self as *const ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }
}
  impl<'largs> VectorAbstractAxisInputMutPtrIndexOfArgs<'largs> for (&'largs *mut ::abstract_axis_input::AbstractAxisInput,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAxisInputMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_indexOf_t_from(original_self as *const ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput, from)
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractAxisInputMutPtr::insert](../struct.VectorAbstractAxisInputMutPtr.html#method.insert) method.
  pub trait VectorAbstractAxisInputMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAxisInputMutPtr) -> ();
  }
  impl<'largs> VectorAbstractAxisInputMutPtrInsertArgs<'largs> for (::libc::c_int,::libc::c_int,&'largs *mut ::abstract_axis_input::AbstractAxisInput) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAxisInputMutPtr) -> () {
    let i = self.0;
let n = self.1;
let t = self.2;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_insert_i_n_t(original_self as *mut ::vector::VectorAbstractAxisInputMutPtr, i, n, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }
}
  impl<'largs> VectorAbstractAxisInputMutPtrInsertArgs<'largs> for (::libc::c_int,&'largs *mut ::abstract_axis_input::AbstractAxisInput) {

  unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAxisInputMutPtr) -> () {
    let i = self.0;
let t = self.1;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_insert_i_t(original_self as *mut ::vector::VectorAbstractAxisInputMutPtr, i, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractAxisInputMutPtr::last_index_of](../struct.VectorAbstractAxisInputMutPtr.html#method.last_index_of) method.
  pub trait VectorAbstractAxisInputMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAxisInputMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAbstractAxisInputMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::abstract_axis_input::AbstractAxisInput {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAxisInputMutPtr) -> ::libc::c_int {
    let t = self;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_lastIndexOf_t(original_self as *const ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput)
  }
}
  impl<'largs> VectorAbstractAxisInputMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::abstract_axis_input::AbstractAxisInput,::libc::c_int) {

  unsafe fn exec(self, original_self: &'largs ::vector::VectorAbstractAxisInputMutPtr) -> ::libc::c_int {
    let t = self.0;
let from = self.1;
    ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorAbstractAxisInputMutPtr, t as *const *mut ::abstract_axis_input::AbstractAxisInput, from)
  }
}
  /// This trait represents a set of arguments accepted by [VectorAbstractAxisInputMutPtr::mid](../struct.VectorAbstractAxisInputMutPtr.html#method.mid) method.
  pub trait VectorAbstractAxisInputMutPtrMidArgs<'largs> {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractAxisInputMutPtr)
            -> ::vector::VectorAbstractAxisInputMutPtr;
  }
  impl<'largs> VectorAbstractAxisInputMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractAxisInputMutPtr)
            -> ::vector::VectorAbstractAxisInputMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorAbstractAxisInputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_mid_to_output_pos(original_self as *const ::vector::VectorAbstractAxisInputMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorAbstractAxisInputMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self,
            original_self: &'largs ::vector::VectorAbstractAxisInputMutPtr)
            -> ::vector::VectorAbstractAxisInputMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorAbstractAxisInputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorAbstractAxisInputMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractAxisInputMutPtr::new](../struct.VectorAbstractAxisInputMutPtr.html#method.new) method.
  pub trait VectorAbstractAxisInputMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorAbstractAxisInputMutPtr;
  }
  impl VectorAbstractAxisInputMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorAbstractAxisInputMutPtr {

      {
        let mut object: ::vector::VectorAbstractAxisInputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorAbstractAxisInputMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorAbstractAxisInputMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorAbstractAxisInputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorAbstractAxisInputMutPtrNewArgs for &'a ::vector::VectorAbstractAxisInputMutPtr {
    fn exec(self) -> ::vector::VectorAbstractAxisInputMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorAbstractAxisInputMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_constructor_v(v as *const ::vector::VectorAbstractAxisInputMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAbstractAxisInputMutPtr::remove](../struct.VectorAbstractAxisInputMutPtr.html#method.remove) method.
  pub trait VectorAbstractAxisInputMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAxisInputMutPtr) -> ();
  }
  impl<'largs> VectorAbstractAxisInputMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAxisInputMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_remove_i(original_self as *mut ::vector::VectorAbstractAxisInputMutPtr, i) }
    }
  }
  impl<'largs> VectorAbstractAxisInputMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorAbstractAxisInputMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAbstractAxisInput_ptr_remove_i_n(original_self as *mut ::vector::VectorAbstractAxisInputMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorActionMutPtr::fill](../struct.VectorActionMutPtr.html#method.fill) method.
  pub trait VectorActionMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorActionMutPtr)
                   -> &'largs mut ::vector::VectorActionMutPtr;
  }
  impl<'largs> VectorActionMutPtrFillArgs<'largs> for &'largs *mut ::action::Action {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorActionMutPtr)
                   -> &'largs mut ::vector::VectorActionMutPtr {
      let t = self;
      let ffi_result =
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_fill_t(original_self as *mut ::vector::VectorActionMutPtr,
                                                                  t as *const *mut ::action::Action);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorActionMutPtrFillArgs<'largs> for (&'largs *mut ::action::Action, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorActionMutPtr)
                   -> &'largs mut ::vector::VectorActionMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_fill_t_size(original_self as *mut ::vector::VectorActionMutPtr, t as *const *mut ::action::Action, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorActionMutPtr::index_of](../struct.VectorActionMutPtr.html#method.index_of) method.
  pub trait VectorActionMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorActionMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorActionMutPtrIndexOfArgs<'largs> for &'largs *mut ::action::Action {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorActionMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_indexOf_t(original_self as *const ::vector::VectorActionMutPtr, t as *const *mut ::action::Action)
    }
  }
  impl<'largs> VectorActionMutPtrIndexOfArgs<'largs> for (&'largs *mut ::action::Action, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorActionMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_indexOf_t_from(original_self as *const ::vector::VectorActionMutPtr, t as *const *mut ::action::Action, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorActionMutPtr::insert](../struct.VectorActionMutPtr.html#method.insert) method.
  pub trait VectorActionMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorActionMutPtr) -> ();
  }
  impl<'largs> VectorActionMutPtrInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs *mut ::action::Action) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorActionMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_insert_i_n_t(original_self as *mut ::vector::VectorActionMutPtr, i, n, t as *const *mut ::action::Action)
    }
  }
  impl<'largs> VectorActionMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::action::Action) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorActionMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_insert_i_t(original_self as *mut ::vector::VectorActionMutPtr, i, t as *const *mut ::action::Action)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorActionMutPtr::last_index_of](../struct.VectorActionMutPtr.html#method.last_index_of) method.
  pub trait VectorActionMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorActionMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorActionMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::action::Action {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorActionMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_lastIndexOf_t(original_self as *const ::vector::VectorActionMutPtr, t as *const *mut ::action::Action)
    }
  }
  impl<'largs> VectorActionMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::action::Action, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorActionMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorActionMutPtr, t as *const *mut ::action::Action, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorActionMutPtr::mid](../struct.VectorActionMutPtr.html#method.mid) method.
  pub trait VectorActionMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorActionMutPtr) -> ::vector::VectorActionMutPtr;
  }
  impl<'largs> VectorActionMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorActionMutPtr) -> ::vector::VectorActionMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorActionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_mid_to_output_pos(original_self as *const ::vector::VectorActionMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorActionMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorActionMutPtr) -> ::vector::VectorActionMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorActionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorActionMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorActionMutPtr::new](../struct.VectorActionMutPtr.html#method.new) method.
  pub trait VectorActionMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorActionMutPtr;
  }
  impl VectorActionMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorActionMutPtr {

      {
        let mut object: ::vector::VectorActionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorActionMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorActionMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorActionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorActionMutPtrNewArgs for &'a ::vector::VectorActionMutPtr {
    fn exec(self) -> ::vector::VectorActionMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorActionMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_constructor_v(v as *const ::vector::VectorActionMutPtr,
                                                                           &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorActionMutPtr::remove](../struct.VectorActionMutPtr.html#method.remove) method.
  pub trait VectorActionMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorActionMutPtr) -> ();
  }
  impl<'largs> VectorActionMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorActionMutPtr) -> () {
      let i = self;
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_remove_i(original_self as *mut ::vector::VectorActionMutPtr, i)
      }
    }
  }
  impl<'largs> VectorActionMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorActionMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAction_ptr_remove_i_n(original_self as *mut ::vector::VectorActionMutPtr, i, n) }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisMutPtr::fill](../struct.VectorAxisMutPtr.html#method.fill) method.
  pub trait VectorAxisMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAxisMutPtr)
                   -> &'largs mut ::vector::VectorAxisMutPtr;
  }
  impl<'largs> VectorAxisMutPtrFillArgs<'largs> for &'largs *mut ::axis::Axis {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAxisMutPtr)
                   -> &'largs mut ::vector::VectorAxisMutPtr {
      let t = self;
      let ffi_result =
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_fill_t(original_self as *mut ::vector::VectorAxisMutPtr,
                                                                t as *const *mut ::axis::Axis);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorAxisMutPtrFillArgs<'largs> for (&'largs *mut ::axis::Axis, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAxisMutPtr)
                   -> &'largs mut ::vector::VectorAxisMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_fill_t_size(original_self as *mut ::vector::VectorAxisMutPtr, t as *const *mut ::axis::Axis, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisMutPtr::index_of](../struct.VectorAxisMutPtr.html#method.index_of) method.
  pub trait VectorAxisMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAxisMutPtrIndexOfArgs<'largs> for &'largs *mut ::axis::Axis {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_indexOf_t(original_self as *const ::vector::VectorAxisMutPtr,
                                                                 t as *const *mut ::axis::Axis)
    }
  }
  impl<'largs> VectorAxisMutPtrIndexOfArgs<'largs> for (&'largs *mut ::axis::Axis, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_indexOf_t_from(original_self as *const ::vector::VectorAxisMutPtr, t as *const *mut ::axis::Axis, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisMutPtr::insert](../struct.VectorAxisMutPtr.html#method.insert) method.
  pub trait VectorAxisMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAxisMutPtr) -> ();
  }
  impl<'largs> VectorAxisMutPtrInsertArgs<'largs> for (::libc::c_int, ::libc::c_int, &'largs *mut ::axis::Axis) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAxisMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_insert_i_n_t(original_self as *mut ::vector::VectorAxisMutPtr,
                                                                    i,
                                                                    n,
                                                                    t as *const *mut ::axis::Axis)
    }
  }
  impl<'largs> VectorAxisMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::axis::Axis) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAxisMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_insert_i_t(original_self as *mut ::vector::VectorAxisMutPtr,
                                                                  i,
                                                                  t as *const *mut ::axis::Axis)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisMutPtr::last_index_of](../struct.VectorAxisMutPtr.html#method.last_index_of) method.
  pub trait VectorAxisMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAxisMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::axis::Axis {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_lastIndexOf_t(original_self as *const ::vector::VectorAxisMutPtr, t as *const *mut ::axis::Axis)
    }
  }
  impl<'largs> VectorAxisMutPtrLastIndexOfArgs<'largs> for (&'largs *mut ::axis::Axis, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorAxisMutPtr, t as *const *mut ::axis::Axis, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisMutPtr::mid](../struct.VectorAxisMutPtr.html#method.mid) method.
  pub trait VectorAxisMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorAxisMutPtr) -> ::vector::VectorAxisMutPtr;
  }
  impl<'largs> VectorAxisMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorAxisMutPtr) -> ::vector::VectorAxisMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorAxisMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_mid_to_output_pos(original_self as *const ::vector::VectorAxisMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorAxisMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorAxisMutPtr) -> ::vector::VectorAxisMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorAxisMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorAxisMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisMutPtr::new](../struct.VectorAxisMutPtr.html#method.new) method.
  pub trait VectorAxisMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorAxisMutPtr;
  }
  impl VectorAxisMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorAxisMutPtr {

      {
        let mut object: ::vector::VectorAxisMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorAxisMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorAxisMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorAxisMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorAxisMutPtrNewArgs for &'a ::vector::VectorAxisMutPtr {
    fn exec(self) -> ::vector::VectorAxisMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorAxisMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_constructor_v(v as *const ::vector::VectorAxisMutPtr,
                                                                         &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisMutPtr::remove](../struct.VectorAxisMutPtr.html#method.remove) method.
  pub trait VectorAxisMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorAxisMutPtr) -> ();
  }
  impl<'largs> VectorAxisMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorAxisMutPtr) -> () {
      let i = self;
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_remove_i(original_self as *mut ::vector::VectorAxisMutPtr, i)
      }
    }
  }
  impl<'largs> VectorAxisMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorAxisMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe {
        ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxis_ptr_remove_i_n(original_self as *mut ::vector::VectorAxisMutPtr,
                                                                    i,
                                                                    n)
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisSettingMutPtr::fill](../struct.VectorAxisSettingMutPtr.html#method.fill) method.
  pub trait VectorAxisSettingMutPtrFillArgs<'largs> {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAxisSettingMutPtr)
                   -> &'largs mut ::vector::VectorAxisSettingMutPtr;
  }
  impl<'largs> VectorAxisSettingMutPtrFillArgs<'largs> for &'largs *mut ::axis_setting::AxisSetting {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAxisSettingMutPtr)
                   -> &'largs mut ::vector::VectorAxisSettingMutPtr {
      let t = self;
      let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_fill_t(original_self as *mut ::vector::VectorAxisSettingMutPtr, t as *const *mut ::axis_setting::AxisSetting);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  impl<'largs> VectorAxisSettingMutPtrFillArgs<'largs> for (&'largs *mut ::axis_setting::AxisSetting, ::libc::c_int) {
    unsafe fn exec(self,
                   original_self: &'largs mut ::vector::VectorAxisSettingMutPtr)
                   -> &'largs mut ::vector::VectorAxisSettingMutPtr {
      let t = self.0;
      let size = self.1;
      let ffi_result = ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_fill_t_size(original_self as *mut ::vector::VectorAxisSettingMutPtr, t as *const *mut ::axis_setting::AxisSetting, size);
      ffi_result.as_mut().expect("Attempted to convert null pointer to reference")
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisSettingMutPtr::index_of](../struct.VectorAxisSettingMutPtr.html#method.index_of) method.
  pub trait VectorAxisSettingMutPtrIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisSettingMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAxisSettingMutPtrIndexOfArgs<'largs> for &'largs *mut ::axis_setting::AxisSetting {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisSettingMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_indexOf_t(original_self as *const ::vector::VectorAxisSettingMutPtr, t as *const *mut ::axis_setting::AxisSetting)
    }
  }
  impl<'largs> VectorAxisSettingMutPtrIndexOfArgs<'largs> for (&'largs *mut ::axis_setting::AxisSetting, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisSettingMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_indexOf_t_from(original_self as *const ::vector::VectorAxisSettingMutPtr, t as *const *mut ::axis_setting::AxisSetting, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisSettingMutPtr::insert](../struct.VectorAxisSettingMutPtr.html#method.insert) method.
  pub trait VectorAxisSettingMutPtrInsertArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAxisSettingMutPtr) -> ();
  }
  impl<'largs> VectorAxisSettingMutPtrInsertArgs<'largs>
    for (::libc::c_int, ::libc::c_int, &'largs *mut ::axis_setting::AxisSetting) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAxisSettingMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      let t = self.2;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_insert_i_n_t(original_self as *mut ::vector::VectorAxisSettingMutPtr, i, n, t as *const *mut ::axis_setting::AxisSetting)
    }
  }
  impl<'largs> VectorAxisSettingMutPtrInsertArgs<'largs> for (::libc::c_int, &'largs *mut ::axis_setting::AxisSetting) {
    unsafe fn exec(self, original_self: &'largs mut ::vector::VectorAxisSettingMutPtr) -> () {
      let i = self.0;
      let t = self.1;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_insert_i_t(original_self as *mut ::vector::VectorAxisSettingMutPtr, i, t as *const *mut ::axis_setting::AxisSetting)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisSettingMutPtr::last_index_of](../struct.VectorAxisSettingMutPtr.html#method.last_index_of) method.
  pub trait VectorAxisSettingMutPtrLastIndexOfArgs<'largs> {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisSettingMutPtr) -> ::libc::c_int;
  }
  impl<'largs> VectorAxisSettingMutPtrLastIndexOfArgs<'largs> for &'largs *mut ::axis_setting::AxisSetting {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisSettingMutPtr) -> ::libc::c_int {
      let t = self;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_lastIndexOf_t(original_self as *const ::vector::VectorAxisSettingMutPtr, t as *const *mut ::axis_setting::AxisSetting)
    }
  }
  impl<'largs> VectorAxisSettingMutPtrLastIndexOfArgs<'largs>
    for (&'largs *mut ::axis_setting::AxisSetting, ::libc::c_int) {
    unsafe fn exec(self, original_self: &'largs ::vector::VectorAxisSettingMutPtr) -> ::libc::c_int {
      let t = self.0;
      let from = self.1;
      ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_lastIndexOf_t_from(original_self as *const ::vector::VectorAxisSettingMutPtr, t as *const *mut ::axis_setting::AxisSetting, from)
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisSettingMutPtr::mid](../struct.VectorAxisSettingMutPtr.html#method.mid) method.
  pub trait VectorAxisSettingMutPtrMidArgs<'largs> {
    fn exec(self, original_self: &'largs ::vector::VectorAxisSettingMutPtr) -> ::vector::VectorAxisSettingMutPtr;
  }
  impl<'largs> VectorAxisSettingMutPtrMidArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs ::vector::VectorAxisSettingMutPtr) -> ::vector::VectorAxisSettingMutPtr {
      let pos = self;
      {
        let mut object: ::vector::VectorAxisSettingMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_mid_to_output_pos(original_self as *const ::vector::VectorAxisSettingMutPtr, pos, &mut object);
        }
        object
      }
    }
  }
  impl<'largs> VectorAxisSettingMutPtrMidArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs ::vector::VectorAxisSettingMutPtr) -> ::vector::VectorAxisSettingMutPtr {
      let pos = self.0;
      let len = self.1;
      {
        let mut object: ::vector::VectorAxisSettingMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_mid_to_output_pos_len(original_self as *const ::vector::VectorAxisSettingMutPtr, pos, len, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisSettingMutPtr::new](../struct.VectorAxisSettingMutPtr.html#method.new) method.
  pub trait VectorAxisSettingMutPtrNewArgs {
    fn exec(self) -> ::vector::VectorAxisSettingMutPtr;
  }
  impl VectorAxisSettingMutPtrNewArgs for () {
    fn exec(self) -> ::vector::VectorAxisSettingMutPtr {

      {
        let mut object: ::vector::VectorAxisSettingMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_constructor_no_args(&mut object);
        }
        object
      }
    }
  }
  impl VectorAxisSettingMutPtrNewArgs for ::libc::c_int {
    fn exec(self) -> ::vector::VectorAxisSettingMutPtr {
      let size = self;
      {
        let mut object: ::vector::VectorAxisSettingMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_constructor_size(size, &mut object);
        }
        object
      }
    }
  }
  impl<'a> VectorAxisSettingMutPtrNewArgs for &'a ::vector::VectorAxisSettingMutPtr {
    fn exec(self) -> ::vector::VectorAxisSettingMutPtr {
      let v = self;
      {
        let mut object: ::vector::VectorAxisSettingMutPtr =
          unsafe { ::cpp_utils::new_uninitialized::NewUninitialized::new_uninitialized() };
        unsafe {
          ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_constructor_v(v as *const ::vector::VectorAxisSettingMutPtr, &mut object);
        }
        object
      }
    }
  }
  /// This trait represents a set of arguments accepted by [VectorAxisSettingMutPtr::remove](../struct.VectorAxisSettingMutPtr.html#method.remove) method.
  pub trait VectorAxisSettingMutPtrRemoveArgs<'largs> {
    fn exec(self, original_self: &'largs mut ::vector::VectorAxisSettingMutPtr) -> ();
  }
  impl<'largs> VectorAxisSettingMutPtrRemoveArgs<'largs> for ::libc::c_int {
    fn exec(self, original_self: &'largs mut ::vector::VectorAxisSettingMutPtr) -> () {
      let i = self;
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_remove_i(original_self as *mut ::vector::VectorAxisSettingMutPtr, i) }
    }
  }
  impl<'largs> VectorAxisSettingMutPtrRemoveArgs<'largs> for (::libc::c_int, ::libc::c_int) {
    fn exec(self, original_self: &'largs mut ::vector::VectorAxisSettingMutPtr) -> () {
      let i = self.0;
      let n = self.1;
      unsafe { ::ffi::qt_3d_input_c_QVector_Qt3DInput_QAxisSetting_ptr_remove_i_n(original_self as *mut ::vector::VectorAxisSettingMutPtr, i, n) }
    }
  }
}
